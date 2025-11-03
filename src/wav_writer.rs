use anyhow::{Context, Result};
use hound::{SampleFormat, WavSpec, WavWriter};

use crate::player::Player;

pub const DEFAULT_OUTPUT_FILENAME: &str = "output.wav";

const GENERATION_BUFFER_SIZE: usize = 2048;

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

pub fn generate_wav(mut player: Player, output_path: &str) -> Result<()> {
    println!("Generating WAV file: {}", output_path);

    let total_samples = player.total_samples();
    let total_duration = total_samples as f64 / Player::sample_rate() as f64;
    println!("  Total duration: {:.2} seconds", total_duration);
    println!(
        "  Sample rate: {} Hz (native OPM rate, no resampling)",
        Player::sample_rate()
    );

    let mut output_samples = Vec::with_capacity((total_samples as usize) * 2);

    let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];
    let mut processed_samples = 0;
    let mut last_progress = 0;

    println!("  Progress: 0%");

    loop {
        player.generate_samples(&mut generation_buffer);
        processed_samples += GENERATION_BUFFER_SIZE;

        output_samples.extend_from_slice(&generation_buffer);

        let progress = (processed_samples * 100 / total_samples as usize).min(100);
        if progress >= last_progress + 10 {
            println!("  Progress: {}%", progress);
            last_progress = progress;
        }

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

    println!("  Writing to file...");
    write_wav(output_path, &output_samples, Player::sample_rate())
        .with_context(|| format!("Failed to write WAV file: {}", output_path))?;

    println!("✅ WAV file created successfully!");

    Ok(())
}

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

        let samples = vec![0i16; 48000 * 2];
        let result = write_wav(temp_path_str, &samples, 48000);

        assert!(result.is_ok());
        assert!(Path::new(temp_path_str).exists());

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

        let _ = std::fs::remove_file(temp_path_str);
    }

    #[test]
    fn test_write_wav_non_zero() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_write_wav_nonzero.wav");
        let temp_path_str = temp_path.to_str().unwrap();

        let mut samples = Vec::with_capacity(1000);
        for i in 0..500 {
            samples.push((i * 100) as i16);
            samples.push((i * 100) as i16);
        }

        let result = write_wav(temp_path_str, &samples, 48000);

        assert!(result.is_ok());
        assert!(Path::new(temp_path_str).exists());

        let metadata = std::fs::metadata(temp_path_str).unwrap();
        assert!(metadata.len() > 100);

        let _ = std::fs::remove_file(temp_path_str);
    }

    #[test]
    fn test_generate_wav_with_player() {
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join("test_generate_wav.wav");
        let temp_path_str = temp_path.to_str().unwrap();

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

        let metadata = std::fs::metadata(temp_path_str).unwrap();
        assert!(metadata.len() > 1000);

        let _ = std::fs::remove_file(temp_path_str);
    }

    #[test]
    fn test_default_output_filename() {
        assert_eq!(DEFAULT_OUTPUT_FILENAME, "output.wav");
    }
}
