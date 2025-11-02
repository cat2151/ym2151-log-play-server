// WAV file output functionality
//
// This module provides functionality to write audio samples to WAV files
// using the hound crate. It handles buffering and progress tracking for
// long-running audio generation tasks.

use anyhow::{Context, Result};
use hound::{SampleFormat, WavSpec, WavWriter};

use crate::player::Player;

/// Default output WAV filename
pub const DEFAULT_OUTPUT_FILENAME: &str = "output.wav";

/// Buffer size for audio generation (in stereo samples)
const GENERATION_BUFFER_SIZE: usize = 2048;

/// Write audio samples to a WAV file.
///
/// This is a simple helper function that writes pre-generated samples
/// to a WAV file with the specified sample rate.
///
/// # Parameters
/// - `path`: Output file path
/// - `samples`: Interleaved stereo i16 samples
/// - `sample_rate`: Sample rate in Hz
///
/// # Returns
/// Ok(()) on success
///
/// # Errors
/// Returns error if file cannot be created or written
///
/// # Examples
/// ```no_run
/// use ym2151_log_player_rust::wav_writer::write_wav;
///
/// let samples = vec![0i16; 48000 * 2]; // 1 second of silence at 48kHz
/// write_wav("output.wav", &samples, 48000).unwrap();
/// ```
pub fn write_wav(path: &str, samples: &[i16], sample_rate: u32) -> Result<()> {
    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)
        .with_context(|| format!("Failed to create WAV file: {}", path))?;

    for &sample in samples {
        writer
            .write_sample(sample)
            .context("Failed to write sample to WAV file")?;
    }

    writer.finalize().context("Failed to finalize WAV file")?;
    Ok(())
}

/// Generate and write audio from a Player to a WAV file with resampling.
///
/// This function generates all audio samples from the player, resamples them
/// from OPM's native rate (55930 Hz) to the standard output rate (48000 Hz),
/// and writes them to a WAV file. Progress is reported to the console.
///
/// # Parameters
/// - `player`: The player to generate audio from
/// - `output_path`: Output WAV file path
///
/// # Returns
/// Ok(()) on success
///
/// # Errors
/// Returns error if generation, resampling, or writing fails
///
/// # Examples
/// ```no_run
/// use ym2151_log_player_rust::events::EventLog;
/// use ym2151_log_player_rust::player::Player;
/// use ym2151_log_player_rust::wav_writer::generate_wav;
///
/// let log = EventLog::from_file("sample_events.json").unwrap();
/// let player = Player::new(log);
/// generate_wav(player, "output.wav").unwrap();
/// ```
pub fn generate_wav(mut player: Player, output_path: &str) -> Result<()> {
    println!("Generating WAV file: {}", output_path);

    let total_samples = player.total_samples();
    let total_duration = total_samples as f64 / Player::sample_rate() as f64;
    println!("  Total duration: {:.2} seconds", total_duration);
    println!(
        "  Sample rate: {} Hz (native OPM rate, no resampling)",
        Player::sample_rate()
    );

    // Pre-allocate output buffer for audio at native OPM sample rate
    // To match the C implementation, we output WAV at 55930 Hz without resampling
    let mut output_samples = Vec::with_capacity((total_samples as usize) * 2);

    // Generate audio in chunks
    let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];
    let mut processed_samples = 0;
    let mut last_progress = 0;

    println!("  Progress: 0%");

    loop {
        // Generate samples from player
        player.generate_samples(&mut generation_buffer);
        processed_samples += GENERATION_BUFFER_SIZE;

        // Append to output buffer (no resampling)
        output_samples.extend_from_slice(&generation_buffer);

        // Report progress every 10%
        let progress = (processed_samples * 100 / total_samples as usize).min(100);
        if progress >= last_progress + 10 {
            println!("  Progress: {}%", progress);
            last_progress = progress;
        }

        // Check if we've generated enough samples
        // Continue even after all events are processed to allow audio to decay naturally
        if processed_samples >= total_samples as usize {
            break;
        }
    }

    println!("  Progress: 100%");
    println!(
        "  Generated {} samples ({:.2}s at {} Hz)",
        output_samples.len() / 2,
        output_samples.len() as f64 / 2.0 / Player::sample_rate() as f64,
        Player::sample_rate()
    );

    // Write to WAV file at native OPM sample rate (55930 Hz)
    println!("  Writing to file...");
    write_wav(output_path, &output_samples, Player::sample_rate())
        .with_context(|| format!("Failed to write WAV file: {}", output_path))?;

    println!("✅ WAV file created successfully!");

    Ok(())
}

/// Generate WAV file with default filename.
///
/// Convenience function that generates audio to "output.wav".
///
/// # Parameters
/// - `player`: The player to generate audio from
///
/// # Returns
/// Ok(()) on success
///
/// # Errors
/// Returns error if generation or writing fails
pub fn generate_wav_default(player: Player) -> Result<()> {
    generate_wav(player, DEFAULT_OUTPUT_FILENAME)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventLog, RegisterEvent};
    use std::path::Path;

    #[test]
    fn test_write_wav_basic() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_write_wav.wav");
        let temp_path_str = temp_path.to_str().unwrap();

        // Create 1 second of silence
        let samples = vec![0i16; 48000 * 2];
        let result = write_wav(temp_path_str, &samples, 48000);

        assert!(result.is_ok());
        assert!(Path::new(temp_path_str).exists());

        // Clean up
        let _ = std::fs::remove_file(temp_path_str);
    }

    #[test]
    fn test_write_wav_empty() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_write_wav_empty.wav");
        let temp_path_str = temp_path.to_str().unwrap();

        let samples = vec![];
        let result = write_wav(temp_path_str, &samples, 48000);

        assert!(result.is_ok());
        assert!(Path::new(temp_path_str).exists());

        // Clean up
        let _ = std::fs::remove_file(temp_path_str);
    }

    #[test]
    fn test_write_wav_non_zero() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_write_wav_nonzero.wav");
        let temp_path_str = temp_path.to_str().unwrap();

        // Create a simple pattern
        let mut samples = Vec::with_capacity(1000);
        for i in 0..500 {
            samples.push((i * 100) as i16);
            samples.push((i * 100) as i16);
        }

        let result = write_wav(temp_path_str, &samples, 48000);

        assert!(result.is_ok());
        assert!(Path::new(temp_path_str).exists());

        // Verify file size is reasonable
        let metadata = std::fs::metadata(temp_path_str).unwrap();
        assert!(metadata.len() > 100); // Should have WAV header + data

        // Clean up
        let _ = std::fs::remove_file(temp_path_str);
    }

    #[test]
    fn test_generate_wav_with_player() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_generate_wav.wav");
        let temp_path_str = temp_path.to_str().unwrap();

        // Create a minimal event log
        let log = EventLog {
            event_count: 2,
            events: vec![
                RegisterEvent {
                    time: 0,
                    addr: 0x08,
                    data: 0x00,
                    is_data: None,
                },
                RegisterEvent {
                    time: 100,
                    addr: 0x20,
                    data: 0xC7,
                    is_data: None,
                },
            ],
        };

        let player = Player::new(log);
        let result = generate_wav(player, temp_path_str);

        assert!(result.is_ok(), "Failed to generate WAV: {:?}", result.err());
        assert!(Path::new(temp_path_str).exists());

        // Verify the file has reasonable size
        let metadata = std::fs::metadata(temp_path_str).unwrap();
        assert!(metadata.len() > 1000); // Should have significant data

        // Clean up
        let _ = std::fs::remove_file(temp_path_str);
    }

    #[test]
    fn test_default_output_filename() {
        assert_eq!(DEFAULT_OUTPUT_FILENAME, "output.wav");
    }
}
