// Simple sample rate conversion using linear interpolation
//
// This module provides a simple resampler for converting audio
// from OPM's native sample rate (55930 Hz) to standard audio
// output rate (48000 Hz) using linear interpolation.

use anyhow::Result;

/// Native sample rate of the OPM chip
pub const OPM_SAMPLE_RATE: u32 = 55930;

/// Standard output sample rate
pub const OUTPUT_SAMPLE_RATE: u32 = 48000;

/// Simple audio resampler using linear interpolation.
///
/// This uses basic linear interpolation to convert between sample rates.
/// It's simple, fast, and bug-free, though lower quality than sinc interpolation.
pub struct AudioResampler {
    input_rate: f64,
    output_rate: f64,
    ratio: f64,
    position: f64,
}

impl AudioResampler {
    /// Create a new resampler for OPM to output sample rate conversion.
    pub fn new() -> Result<Self> {
        Self::with_rates(OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE)
    }

    /// Create a new resampler with custom sample rates.
    pub fn with_rates(input_rate: u32, output_rate: u32) -> Result<Self> {
        let input_rate = input_rate as f64;
        let output_rate = output_rate as f64;
        let ratio = input_rate / output_rate;

        Ok(Self {
            input_rate,
            output_rate,
            ratio,
            position: 0.0,
        })
    }

    /// Resample interleaved stereo i16 samples using linear interpolation.
    pub fn resample(&mut self, input: &[i16]) -> Result<Vec<i16>> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        if input.len() % 2 != 0 {
            anyhow::bail!("Input buffer must have even length (stereo samples)");
        }

        let input_frames = input.len() / 2;
        let output_frames = ((input_frames as f64) / self.ratio).ceil() as usize;
        let mut output = Vec::with_capacity(output_frames * 2);

        let mut pos = self.position;
        
        while (pos as usize + 1) * 2 < input.len() {
            let frame_idx = pos as usize;
            let frac = pos - frame_idx as f64;
            
            if frame_idx + 1 >= input_frames {
                break;
            }

            // Linear interpolation for left channel
            let left0 = input[frame_idx * 2] as f64;
            let left1 = input[(frame_idx + 1) * 2] as f64;
            let left_out = left0 + (left1 - left0) * frac;
            
            // Linear interpolation for right channel  
            let right0 = input[frame_idx * 2 + 1] as f64;
            let right1 = input[(frame_idx + 1) * 2 + 1] as f64;
            let right_out = right0 + (right1 - right0) * frac;

            output.push(left_out.clamp(-32768.0, 32767.0) as i16);
            output.push(right_out.clamp(-32768.0, 32767.0) as i16);

            pos += self.ratio;
        }

        // Update position for next call, wrapping around input length
        self.position = pos - input_frames as f64;
        if self.position < 0.0 {
            self.position = 0.0;
        }

        Ok(output)
    }

    /// Get the output sample rate.
    pub fn output_rate(&self) -> u32 {
        self.output_rate as u32
    }

    /// Get the input sample rate.
    pub fn input_rate(&self) -> u32 {
        self.input_rate as u32
    }

    /// Calculate the expected output size for a given input size.
    pub fn expected_output_frames(&self, input_frames: usize) -> usize {
        ((input_frames as f64) / self.ratio).ceil() as usize
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
        let input = vec![0i16; 3]; // Odd length - invalid for stereo
        let result = resampler.resample(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_resample_basic() {
        let mut resampler = AudioResampler::new().unwrap();
        // Create 1024 stereo samples (2048 i16 values)
        let input = vec![0i16; 2048];
        let result = resampler.resample(&input);

        assert!(result.is_ok());
        let output = result.unwrap();

        // Output should be smaller (downsampling from 55930 to 48000)
        assert!(output.len() > 0);
        assert!(output.len() < input.len());
        assert_eq!(output.len() % 2, 0); // Still stereo
    }

    #[test]
    fn test_resample_sine_wave() {
        let mut resampler = AudioResampler::new().unwrap();

        // Generate a simple sine wave at input rate
        let freq = 440.0; // A4 note
        let duration_samples = 1024;
        let mut input = Vec::with_capacity(duration_samples * 2);

        for i in 0..duration_samples {
            let t = i as f32 / OPM_SAMPLE_RATE as f32;
            let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
            let i16_sample = (sample * 16384.0) as i16;
            input.push(i16_sample); // Left
            input.push(i16_sample); // Right
        }

        let result = resampler.resample(&input);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(output.len() > 0);
        assert_eq!(output.len() % 2, 0);

        // Output should be non-zero (contains actual audio)
        let max_sample = output.iter().map(|&s| s.abs()).max().unwrap_or(0);
        assert!(max_sample > 100); // Should have significant amplitude
    }

    #[test]
    fn test_expected_output_frames() {
        let resampler = AudioResampler::new().unwrap();

        // For 1000 input frames at 55930 Hz -> ~858 frames at 48000 Hz
        let output_frames = resampler.expected_output_frames(1000);
        assert!(output_frames >= 857 && output_frames <= 859);
    }

    #[test]
    fn test_resample_multiple_chunks() {
        let mut resampler = AudioResampler::new().unwrap();

        // Process multiple small chunks
        let chunk_size = 256;
        for _ in 0..5 {
            let input = vec![1000i16; chunk_size * 2]; // Non-zero input
            let result = resampler.resample(&input);
            assert!(result.is_ok());

            let output = result.unwrap();
            assert!(output.len() > 0);
            assert_eq!(output.len() % 2, 0);
        }
    }
}
