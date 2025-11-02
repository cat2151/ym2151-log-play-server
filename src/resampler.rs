// Sample rate conversion using rubato
//
// This module provides a safe wrapper around the rubato resampler
// for converting audio from OPM's native sample rate (55930 Hz)
// to standard audio output rate (48000 Hz).

use anyhow::{Context, Result};
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

/// Native sample rate of the OPM chip
pub const OPM_SAMPLE_RATE: u32 = 55930;

/// Standard output sample rate
pub const OUTPUT_SAMPLE_RATE: u32 = 48000;

/// Audio resampler for converting OPM output to standard sample rates.
///
/// This wraps the rubato SincFixedIn resampler with a simple interface
/// for converting interleaved stereo i16 samples.
pub struct AudioResampler {
    resampler: SincFixedIn<f32>,
    channels: usize,
    input_buffer: Vec<Vec<f32>>,
}

impl AudioResampler {
    /// Create a new resampler for OPM to output sample rate conversion.
    ///
    /// # Returns
    /// A new AudioResampler configured for stereo OPM output
    ///
    /// # Errors
    /// Returns error if the resampler cannot be initialized
    pub fn new() -> Result<Self> {
        Self::with_rates(OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE)
    }

    /// Create a new resampler with custom sample rates.
    ///
    /// # Parameters
    /// - `input_rate`: Input sample rate in Hz
    /// - `output_rate`: Output sample rate in Hz
    ///
    /// # Returns
    /// A new AudioResampler
    ///
    /// # Errors
    /// Returns error if the resampler cannot be initialized
    pub fn with_rates(input_rate: u32, output_rate: u32) -> Result<Self> {
        let channels = 2; // Stereo
        let chunk_size = 1024; // Process 1024 input samples at a time

        // Configure high-quality sinc interpolation
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };

        let resampler = SincFixedIn::<f32>::new(
            output_rate as f64 / input_rate as f64,
            2.0, // Maximum relative buffer size ratio
            params,
            chunk_size,
            channels,
        )
        .context("Failed to create resampler")?;

        // Pre-allocate buffers for channel-wise processing
        let input_buffer = vec![vec![0.0f32; chunk_size]; channels];

        Ok(Self {
            resampler,
            channels,
            input_buffer,
        })
    }

    /// Resample interleaved stereo i16 samples.
    ///
    /// # Parameters
    /// - `input`: Interleaved stereo i16 samples (must have even length)
    ///
    /// # Returns
    /// Vector of resampled interleaved stereo i16 samples
    ///
    /// # Errors
    /// Returns error if resampling fails
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::resampler::AudioResampler;
    ///
    /// let mut resampler = AudioResampler::new().unwrap();
    /// let input = vec![0i16; 2048]; // 1024 stereo samples at 55930 Hz
    /// let output = resampler.resample(&input).unwrap();
    /// // output is now at 48000 Hz
    /// ```
    pub fn resample(&mut self, input: &[i16]) -> Result<Vec<i16>> {
        if input.is_empty() {
            return Ok(Vec::new());
        }

        if input.len() % 2 != 0 {
            anyhow::bail!("Input buffer must have even length (stereo samples)");
        }

        let num_frames = input.len() / self.channels;
        let chunk_size = self.resampler.input_frames_next();

        // If input doesn't match expected chunk size, pad with zeros
        let frames_needed = chunk_size;
        
        // Clear and prepare input buffer with exact chunk size
        for ch in 0..self.channels {
            self.input_buffer[ch].clear();
            self.input_buffer[ch].resize(frames_needed, 0.0);
        }

        // De-interleave and convert to f32, only up to available input
        for (frame_idx, stereo_frame) in input.chunks_exact(self.channels).enumerate().take(frames_needed) {
            for (ch, &sample) in stereo_frame.iter().enumerate() {
                // Convert i16 to f32 in range [-1.0, 1.0]
                let normalized = sample as f32 / 32768.0;
                self.input_buffer[ch][frame_idx] = normalized;
            }
        }

        // Process through resampler
        let output_frames = self
            .resampler
            .process(&self.input_buffer, None)
            .context("Resampling failed")?;

        // Calculate how many frames we actually used from input
        let frames_consumed = num_frames.min(frames_needed);
        let output_frames_count = (frames_consumed as f64 * OUTPUT_SAMPLE_RATE as f64 / OPM_SAMPLE_RATE as f64).ceil() as usize;

        // Interleave and convert back to i16, only up to the proportional output
        let mut result = Vec::with_capacity(output_frames_count * self.channels);
        for i in 0..output_frames_count.min(output_frames[0].len()) {
            for ch in 0..self.channels {
                let sample = output_frames[ch][i];
                // Clamp and convert to i16
                let clamped = sample.clamp(-1.0, 1.0);
                let i16_sample = (clamped * 32767.0) as i16;
                result.push(i16_sample);
            }
        }

        Ok(result)
    }

    /// Get the output sample rate.
    pub fn output_rate(&self) -> u32 {
        OUTPUT_SAMPLE_RATE
    }

    /// Get the input sample rate.
    pub fn input_rate(&self) -> u32 {
        OPM_SAMPLE_RATE
    }

    /// Calculate the expected output size for a given input size.
    ///
    /// # Parameters
    /// - `input_frames`: Number of input stereo frames
    ///
    /// # Returns
    /// Approximate number of output stereo frames
    pub fn expected_output_frames(&self, input_frames: usize) -> usize {
        let ratio = OUTPUT_SAMPLE_RATE as f64 / OPM_SAMPLE_RATE as f64;
        (input_frames as f64 * ratio).ceil() as usize
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
        
        // Output should be roughly (2048 / 2) * (48000 / 55930) * 2
        // = 1024 * 0.858 * 2 ≈ 1758 samples
        assert!(output.len() > 0);
        assert!(output.len() < input.len()); // Downsampling
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
        let max_sample = output.iter().map(|&s| s.abs()).max().unwrap();
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
            let input = vec![0i16; chunk_size * 2]; // Stereo
            let result = resampler.resample(&input);
            assert!(result.is_ok());
            
            let output = result.unwrap();
            assert!(output.len() > 0);
            assert_eq!(output.len() % 2, 0);
        }
    }
}
