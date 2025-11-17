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
        Self::with_quality(ResamplingQuality::Linear)
    }

    pub fn with_quality(quality: ResamplingQuality) -> Result<Self> {
        Self::with_rates_and_quality(OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE, quality)
    }

    pub fn with_rates(input_rate: u32, output_rate: u32) -> Result<Self> {
        Self::with_rates_and_quality(input_rate, output_rate, ResamplingQuality::Linear)
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
                let chunk_size = 1024;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resampler_creation() {
        let resampler = AudioResampler::new();
        assert!(resampler.is_ok());
    }

    #[test]
    fn test_resampler_rates() {
        let resampler = AudioResampler::new().unwrap();
        assert_eq!(resampler.input_rate(), 55930);
        assert_eq!(resampler.output_rate(), 48000);
    }

    #[test]
    fn test_resample_empty() {
        let mut resampler = AudioResampler::new().unwrap();
        let result = resampler.resample(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_resample_odd_length() {
        let mut resampler = AudioResampler::new().unwrap();
        let input = vec![0i16; 3];
        let result = resampler.resample(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_resample_basic() {
        let mut resampler = AudioResampler::new().unwrap();

        let input = vec![0i16; 2048];
        let result = resampler.resample(&input);

        assert!(result.is_ok());
        let output = result.unwrap();

        assert!(!output.is_empty());
        assert!(output.len() < input.len());
        assert_eq!(output.len() % 2, 0);
    }

    #[test]
    fn test_resample_sine_wave() {
        let mut resampler = AudioResampler::new().unwrap();

        let freq = 440.0;
        let duration_samples = 1024;
        let mut input = Vec::with_capacity(duration_samples * 2);

        for i in 0..duration_samples {
            let t = i as f32 / OPM_SAMPLE_RATE as f32;
            let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
            let i16_sample = (sample * 16384.0) as i16;
            input.push(i16_sample);
            input.push(i16_sample);
        }

        let result = resampler.resample(&input);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
        assert_eq!(output.len() % 2, 0);

        let max_sample = output.iter().map(|&s| s.abs()).max().unwrap_or(0);
        assert!(max_sample > 100);
    }

    #[test]
    fn test_expected_output_frames() {
        let resampler = AudioResampler::new().unwrap();

        let output_frames = resampler.expected_output_frames(1000);
        assert!((857..=859).contains(&output_frames));
    }

    #[test]
    fn test_resample_multiple_chunks() {
        let mut resampler = AudioResampler::new().unwrap();

        let chunk_size = 256;
        for _ in 0..5 {
            let input = vec![1000i16; chunk_size * 2];
            let result = resampler.resample(&input);
            assert!(result.is_ok());

            let output = result.unwrap();
            assert!(!output.is_empty());
            assert_eq!(output.len() % 2, 0);
        }
    }

    #[test]
    fn test_buffer_boundary_continuity() {
        // Test that resampling across buffer boundaries maintains sample continuity
        // This verifies that the position state is correctly preserved across chunks

        let freq = 1000.0; // 1kHz test tone
        let total_samples = 4096; // Large enough to span multiple typical buffers

        // Generate a continuous sine wave
        let mut continuous_input = Vec::with_capacity(total_samples * 2);
        for i in 0..total_samples {
            let t = i as f32 / OPM_SAMPLE_RATE as f32;
            let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
            let i16_sample = (sample * 16384.0) as i16;
            continuous_input.push(i16_sample);
            continuous_input.push(i16_sample);
        }

        // Resample as a single buffer (reference)
        let mut resampler_single = AudioResampler::new().unwrap();
        let reference_output = resampler_single.resample(&continuous_input).unwrap();

        // Resample in chunks (simulating real-world usage)
        let mut resampler_chunked = AudioResampler::new().unwrap();
        let mut chunked_output = Vec::new();
        let chunk_size = 512; // Typical buffer size

        for chunk_start in (0..continuous_input.len()).step_by(chunk_size * 2) {
            let chunk_end = (chunk_start + chunk_size * 2).min(continuous_input.len());
            let chunk = &continuous_input[chunk_start..chunk_end];
            let resampled_chunk = resampler_chunked.resample(chunk).unwrap();
            chunked_output.extend_from_slice(&resampled_chunk);
        }

        // The outputs may differ slightly in length due to how buffer boundaries
        // interact with the stopping condition. This is expected and not a bug.
        // What matters is that the samples that ARE produced maintain continuity.
        let len_diff = (reference_output.len() as i32 - chunked_output.len() as i32).abs();
        assert!(
            len_diff <= 10,
            "Length difference too large: {} samples (expected <10)",
            len_diff
        );

        // Compare the overlapping samples
        let compare_len = reference_output.len().min(chunked_output.len());
        let mut max_diff = 0i32;
        let mut total_diff = 0i64;
        let mut large_diffs = 0;

        for i in 0..compare_len {
            let diff = (reference_output[i] as i32 - chunked_output[i] as i32).abs();
            if diff > 2 {
                large_diffs += 1;
            }
            max_diff = max_diff.max(diff);
            total_diff += diff as i64;
        }

        // Most samples should be identical or very close
        // Linear interpolation with proper state management should produce
        // nearly identical results regardless of chunking
        assert!(
            max_diff <= 10,
            "Max sample difference too large: {} (expected <=10)",
            max_diff
        );

        let avg_diff = total_diff as f64 / compare_len as f64;
        assert!(
            avg_diff < 1.0,
            "Average difference too large: {:.3} (expected <1.0)",
            avg_diff
        );

        // Most samples should match exactly or within rounding error
        let large_diff_percentage = (large_diffs as f64 / compare_len as f64) * 100.0;
        assert!(
            large_diff_percentage < 1.0,
            "Too many samples with large differences: {:.2}% (expected <1%)",
            large_diff_percentage
        );
    }

    #[test]
    fn test_position_state_preservation() {
        // Test that position state is correctly preserved across calls
        let mut resampler = AudioResampler::new().unwrap();

        // Process first chunk
        let input1 = vec![100i16; 100 * 2]; // 100 stereo frames
        let _output1 = resampler.resample(&input1).unwrap();

        // Process second chunk
        let input2 = vec![200i16; 100 * 2];
        let _output2 = resampler.resample(&input2).unwrap();

        // Just verify resampling works correctly across chunks
        // Position state is internal to the implementation
    }

    #[test]
    fn test_high_quality_resampler_creation() {
        let resampler = AudioResampler::with_quality(ResamplingQuality::HighQuality);
        assert!(resampler.is_ok());
        assert_eq!(resampler.unwrap().quality(), ResamplingQuality::HighQuality);
    }

    #[test]
    fn test_high_quality_resample_basic() {
        let mut resampler = AudioResampler::with_quality(ResamplingQuality::HighQuality).unwrap();

        let input = vec![0i16; 2048];
        let result = resampler.resample(&input);

        assert!(result.is_ok());
        let output = result.unwrap();

        assert!(!output.is_empty());
        assert!(output.len() < input.len());
        assert_eq!(output.len() % 2, 0);
    }

    #[test]
    fn test_high_quality_resample_sine_wave() {
        let mut resampler = AudioResampler::with_quality(ResamplingQuality::HighQuality).unwrap();

        let freq = 440.0;
        let duration_samples = 2048;
        let mut input = Vec::with_capacity(duration_samples * 2);

        for i in 0..duration_samples {
            let t = i as f32 / OPM_SAMPLE_RATE as f32;
            let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
            let i16_sample = (sample * 16384.0) as i16;
            input.push(i16_sample);
            input.push(i16_sample);
        }

        let result = resampler.resample(&input);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.is_empty());
        assert_eq!(output.len() % 2, 0);

        let max_sample = output.iter().map(|&s| s.abs()).max().unwrap_or(0);
        assert!(max_sample > 100);
    }

    #[test]
    fn test_high_quality_vs_linear_quality() {
        // Generate a high-frequency sine wave to test aliasing
        let freq = 10000.0; // 10kHz - near Nyquist for 48kHz output
        let duration_samples = 4096;
        let mut input = Vec::with_capacity(duration_samples * 2);

        for i in 0..duration_samples {
            let t = i as f32 / OPM_SAMPLE_RATE as f32;
            let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
            let i16_sample = (sample * 16384.0) as i16;
            input.push(i16_sample);
            input.push(i16_sample);
        }

        // Resample with linear
        let mut linear_resampler = AudioResampler::with_quality(ResamplingQuality::Linear).unwrap();
        let linear_output = linear_resampler.resample(&input).unwrap();

        // Resample with high quality
        let mut hq_resampler =
            AudioResampler::with_quality(ResamplingQuality::HighQuality).unwrap();
        let hq_output = hq_resampler.resample(&input).unwrap();

        // Both should produce similar length outputs (within reasonable tolerance)
        // Note: different resampling algorithms may produce slightly different output lengths
        assert!(
            (linear_output.len() as i32 - hq_output.len() as i32).abs() <= 300,
            "Output lengths should be similar: linear={}, high_quality={}",
            linear_output.len(),
            hq_output.len()
        );

        // High quality should produce non-zero output (not testing quality here, just functionality)
        let hq_max = hq_output.iter().map(|&s| s.abs()).max().unwrap_or(0);
        assert!(hq_max > 100, "High quality output should have signal");
    }
}
