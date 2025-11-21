use anyhow::Result;
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

pub const YM2151_CLOCK: u32 = 3_579_545;

pub const OPM_SAMPLE_RATE: u32 = YM2151_CLOCK / 64;

pub const OUTPUT_SAMPLE_RATE: u32 = 48000;

/// Resampling quality setting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResamplingQuality {
    /// Linear interpolation - Fast but may have aliasing artifacts
    Linear,
    /// High-quality sinc-based resampling using Rubato library
    HighQuality,
}

enum ResamplerImpl {
    Linear {
        ratio: f64,
        position: f64,
        last_frame: Option<(i16, i16)>,
    },
    HighQuality {
        rubato: SincFixedIn<f32>,
        leftover_input_left: Vec<f32>,
        leftover_input_right: Vec<f32>,
    },
}

pub struct AudioResampler {
    input_rate: u32,
    output_rate: u32,
    quality: ResamplingQuality,
    inner: ResamplerImpl,
}

impl AudioResampler {
    pub fn new() -> Result<Self> {
        Self::with_quality(ResamplingQuality::HighQuality)
    }

    pub fn with_quality(quality: ResamplingQuality) -> Result<Self> {
        Self::with_rates_and_quality(OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE, quality)
    }

    pub fn with_rates(input_rate: u32, output_rate: u32) -> Result<Self> {
        Self::with_rates_and_quality(input_rate, output_rate, ResamplingQuality::HighQuality)
    }

    pub fn with_rates_and_quality(
        input_rate: u32,
        output_rate: u32,
        quality: ResamplingQuality,
    ) -> Result<Self> {
        let inner = match quality {
            ResamplingQuality::Linear => {
                let ratio = input_rate as f64 / output_rate as f64;
                ResamplerImpl::Linear {
                    ratio,
                    position: 0.0,
                    last_frame: None,
                }
            }
            ResamplingQuality::HighQuality => {
                // Configure high-quality sinc interpolation
                let params = SincInterpolationParameters {
                    sinc_len: 256,
                    f_cutoff: 0.95,
                    interpolation: SincInterpolationType::Linear,
                    oversampling_factor: 256,
                    window: WindowFunction::BlackmanHarris2,
                };

                // Choose a reasonable chunk size for processing
                let chunk_size = crate::audio_config::buffer::RESAMPLING_CHUNK_SIZE;
                let resample_ratio = output_rate as f64 / input_rate as f64;

                let rubato = SincFixedIn::<f32>::new(
                    resample_ratio,
                    2.0, // max_resample_ratio_relative
                    params,
                    chunk_size,
                    2, // stereo
                )?;

                ResamplerImpl::HighQuality {
                    rubato,
                    leftover_input_left: Vec::new(),
                    leftover_input_right: Vec::new(),
                }
            }
        };

        Ok(Self {
            input_rate,
            output_rate,
            quality,
            inner,
        })
    }

    pub fn resample(&mut self, input: &[i16]) -> Result<Vec<i16>> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        if !input.len().is_multiple_of(2) {
            anyhow::bail!("Input buffer must have even length (stereo samples)");
        }

        match &mut self.inner {
            ResamplerImpl::Linear { .. } => self.resample_linear(input),
            ResamplerImpl::HighQuality { .. } => self.resample_high_quality(input),
        }
    }

    fn resample_linear(&mut self, input: &[i16]) -> Result<Vec<i16>> {
        let (ratio, position, last_frame) = match &mut self.inner {
            ResamplerImpl::Linear {
                ratio,
                position,
                last_frame,
            } => (ratio, position, last_frame),
            _ => unreachable!(),
        };

        let input_frames = input.len() / 2;
        let output_frames = ((input_frames as f64) / *ratio).ceil() as usize;
        let mut output = Vec::with_capacity(output_frames * 2);

        let mut pos = *position;

        while pos < input_frames as f64 {
            let frame_idx = pos.floor() as isize;
            let frac = pos - frame_idx as f64;

            // Get the current and next frames for interpolation
            let (left0, right0, left1, right1) = if frame_idx < 0 && last_frame.is_some() {
                // Negative position means we need the last frame from the previous chunk
                let (last_left, last_right) = last_frame.unwrap();
                let curr_left = input[0] as f64;
                let curr_right = input[1] as f64;
                (last_left as f64, last_right as f64, curr_left, curr_right)
            } else if frame_idx >= 0 && (frame_idx as usize) + 1 < input_frames {
                // Normal case: interpolate between two frames in the current chunk
                let idx = frame_idx as usize;
                let left0 = input[idx * 2] as f64;
                let right0 = input[idx * 2 + 1] as f64;
                let left1 = input[(idx + 1) * 2] as f64;
                let right1 = input[(idx + 1) * 2 + 1] as f64;
                (left0, right0, left1, right1)
            } else {
                // We've reached the end of the chunk
                break;
            };

            // Linear interpolation
            let left_out = left0 + (left1 - left0) * frac;
            let right_out = right0 + (right1 - right0) * frac;

            output.push(left_out.clamp(-32768.0, 32767.0) as i16);
            output.push(right_out.clamp(-32768.0, 32767.0) as i16);

            pos += *ratio;
        }

        // Save the last frame from this chunk for the next call
        if input_frames > 0 {
            let last_idx = (input_frames - 1) * 2;
            *last_frame = Some((input[last_idx], input[last_idx + 1]));
        }

        // Update position for next chunk
        *position = pos - input_frames as f64;

        Ok(output)
    }

    fn resample_high_quality(&mut self, input: &[i16]) -> Result<Vec<i16>> {
        let (rubato, leftover_input_left, leftover_input_right) = match &mut self.inner {
            ResamplerImpl::HighQuality {
                rubato,
                leftover_input_left,
                leftover_input_right,
            } => (rubato, leftover_input_left, leftover_input_right),
            _ => unreachable!(),
        };

        let input_frames = input.len() / 2;

        // Deinterleave and convert i16 to f32, combining with leftovers
        let mut left_channel = Vec::with_capacity(leftover_input_left.len() + input_frames);
        let mut right_channel = Vec::with_capacity(leftover_input_right.len() + input_frames);

        left_channel.extend_from_slice(leftover_input_left);
        right_channel.extend_from_slice(leftover_input_right);
        leftover_input_left.clear();
        leftover_input_right.clear();

        for i in 0..input_frames {
            left_channel.push(input[i * 2] as f32 / 32768.0);
            right_channel.push(input[i * 2 + 1] as f32 / 32768.0);
        }

        let total_frames = left_channel.len();
        let chunk_size = rubato.input_frames_next();

        // Process in chunks
        let mut final_output: Vec<i16> = Vec::new();
        let mut processed_frames = 0;

        while processed_frames + chunk_size <= total_frames {
            let chunk_left = &left_channel[processed_frames..processed_frames + chunk_size];
            let chunk_right = &right_channel[processed_frames..processed_frames + chunk_size];

            // Process the chunk
            let input_buffer = vec![chunk_left.to_vec(), chunk_right.to_vec()];
            let output = rubato.process(&input_buffer, None)?;

            // Interleave and convert back to i16
            let output_frames = output[0].len();
            #[allow(clippy::needless_range_loop)]
            for i in 0..output_frames {
                let left_sample = (output[0][i] * 32768.0).clamp(-32768.0, 32767.0) as i16;
                let right_sample = (output[1][i] * 32768.0).clamp(-32768.0, 32767.0) as i16;
                final_output.push(left_sample);
                final_output.push(right_sample);
            }

            processed_frames += chunk_size;
        }

        // Store leftover input for next call
        if processed_frames < total_frames {
            leftover_input_left.extend_from_slice(&left_channel[processed_frames..]);
            leftover_input_right.extend_from_slice(&right_channel[processed_frames..]);
        }

        Ok(final_output)
    }

    pub fn output_rate(&self) -> u32 {
        self.output_rate
    }

    pub fn input_rate(&self) -> u32 {
        self.input_rate
    }

    pub fn quality(&self) -> ResamplingQuality {
        self.quality
    }

    pub fn expected_output_frames(&self, input_frames: usize) -> usize {
        match &self.inner {
            ResamplerImpl::Linear { ratio, .. } => ((input_frames as f64) / *ratio).ceil() as usize,
            ResamplerImpl::HighQuality { .. } => {
                // For Rubato, we need to account for chunk processing
                let ratio = self.input_rate as f64 / self.output_rate as f64;
                ((input_frames as f64) / ratio).ceil() as usize
            }
        }
    }
}
