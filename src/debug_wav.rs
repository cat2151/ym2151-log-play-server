//! Debug WAV output functionality for audio glitch diagnosis.
//!
//! This module provides functionality to generate 4 different WAV files
//! for debugging audio glitches, particularly with high MUL values.
//!
//! # Output Files
//!
//! When enabled via the `YM2151_DEBUG_WAV` environment variable, the following
//! WAV files are generated:
//!
//! - `realtime_55k.wav` - 55930Hz buffer captured during real-time playback
//! - `realtime_48k.wav` - 48000Hz resampled buffer from real-time playback
//! - `post_55k.wav` - 55930Hz buffer from non-real-time rendering
//! - `post_48k.wav` - 48000Hz resampled buffer from non-real-time rendering
//!
//! # Usage
//!
//! Set the `YM2151_DEBUG_WAV` environment variable before running:
//!
//! ```bash
//! # Windows (PowerShell)
//! $env:YM2151_DEBUG_WAV="1"
//! .\ym2151-log-play-server.exe sample_events.json
//!
//! # Windows (cmd)
//! set YM2151_DEBUG_WAV=1
//! .\ym2151-log-play-server.exe sample_events.json
//! ```
//!
//! # Purpose
//!
//! The 4 different WAV files allow comparison to diagnose where audio glitches occur:
//! - Compare realtime vs post-playback to identify real-time processing issues
//! - Compare 55930Hz vs 48000Hz to identify resampling issues
//!
//! This is particularly useful for debugging audio glitches that occur with
//! high MUL (frequency multiplier) values in tone parameters.

use crate::events::EventLog;
use crate::player::Player;
use crate::resampler::{AudioResampler, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};
use crate::wav_writer;
use anyhow::{Context, Result};
use std::env;

const GENERATION_BUFFER_SIZE: usize = 2048;

/// Check if debug WAV output is enabled via environment variable.
///
/// Returns true if the `YM2151_DEBUG_WAV` environment variable is set to any value.
pub fn is_debug_wav_enabled() -> bool {
    env::var("YM2151_DEBUG_WAV").is_ok()
}

/// Generate post-playback WAV buffers at both 55930Hz and 48000Hz.
///
/// This performs non-real-time rendering of the event log, which can be compared
/// with real-time playback buffers to diagnose timing-related audio issues.
///
/// # Arguments
///
/// * `log` - The event log to render
///
/// # Returns
///
/// A tuple containing:
/// - Vector of i16 samples at 55930Hz (stereo)
/// - Vector of i16 samples at 48000Hz (stereo, resampled)
///
/// # Example
///
/// ```no_run
/// use ym2151_log_play_server::debug_wav;
/// use ym2151_log_play_server::events::EventLog;
///
/// let log = EventLog::from_file("sample_events.json").unwrap();
/// let (buffer_55k, buffer_48k) = debug_wav::generate_post_playback_buffers(&log).unwrap();
/// ```
pub fn generate_post_playback_buffers(log: &EventLog) -> Result<(Vec<i16>, Vec<i16>)> {
    let mut player = Player::new(log.clone());
    let mut resampler = AudioResampler::new().context("Failed to initialize resampler")?;

    let mut buffer_55k = Vec::new();
    let mut buffer_48k = Vec::new();

    let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];

    loop {
        if !player.should_continue_tail() {
            break;
        }

        player.generate_samples(&mut generation_buffer);
        buffer_55k.extend_from_slice(&generation_buffer);

        let resampled = resampler
            .resample(&generation_buffer)
            .context("Failed to resample audio in post-playback generation")?;
        buffer_48k.extend_from_slice(&resampled);
    }

    Ok((buffer_55k, buffer_48k))
}

/// Save all 4 debug WAV files.
///
/// Writes the following files to the current directory:
/// - `realtime_55k.wav` - Real-time playback at 55930Hz
/// - `realtime_48k.wav` - Real-time playback at 48000Hz  
/// - `post_55k.wav` - Non-real-time rendering at 55930Hz
/// - `post_48k.wav` - Non-real-time rendering at 48000Hz
///
/// # Arguments
///
/// * `realtime_55k` - Buffer from real-time playback at 55930Hz
/// * `realtime_48k` - Buffer from real-time playback at 48000Hz
/// * `post_55k` - Buffer from post-playback rendering at 55930Hz
/// * `post_48k` - Buffer from post-playback rendering at 48000Hz
///
/// # Example
///
/// ```no_run
/// use ym2151_log_play_server::debug_wav;
///
/// let realtime_55k = vec![0i16; 55930 * 2]; // 1 second stereo
/// let realtime_48k = vec![0i16; 48000 * 2]; // 1 second stereo
/// let post_55k = vec![0i16; 55930 * 2];
/// let post_48k = vec![0i16; 48000 * 2];
///
/// debug_wav::save_debug_wav_files(&realtime_55k, &realtime_48k, &post_55k, &post_48k).unwrap();
/// ```
pub fn save_debug_wav_files(
    realtime_55k: &[i16],
    realtime_48k: &[i16],
    post_55k: &[i16],
    post_48k: &[i16],
) -> Result<()> {
    println!("\n保存中: デバッグ用WAVファイル...");

    wav_writer::write_wav("realtime_55k.wav", realtime_55k, OPM_SAMPLE_RATE)
        .context("Failed to write realtime_55k.wav")?;
    println!("✅ realtime_55k.wav を作成しました");

    wav_writer::write_wav("realtime_48k.wav", realtime_48k, OUTPUT_SAMPLE_RATE)
        .context("Failed to write realtime_48k.wav")?;
    println!("✅ realtime_48k.wav を作成しました");

    wav_writer::write_wav("post_55k.wav", post_55k, OPM_SAMPLE_RATE)
        .context("Failed to write post_55k.wav")?;
    println!("✅ post_55k.wav を作成しました");

    wav_writer::write_wav("post_48k.wav", post_48k, OUTPUT_SAMPLE_RATE)
        .context("Failed to write post_48k.wav")?;
    println!("✅ post_48k.wav を作成しました");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::RegisterEvent;

    #[test]
    fn test_is_debug_wav_enabled_false() {
        env::remove_var("YM2151_DEBUG_WAV");
        assert!(!is_debug_wav_enabled());
    }

    #[test]
    fn test_is_debug_wav_enabled_true() {
        env::set_var("YM2151_DEBUG_WAV", "1");
        assert!(is_debug_wav_enabled());
        env::remove_var("YM2151_DEBUG_WAV");
    }

    #[test]
    fn test_generate_post_playback_buffers() {
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

        let result = generate_post_playback_buffers(&log);
        assert!(result.is_ok());

        let (buffer_55k, buffer_48k) = result.unwrap();
        assert!(!buffer_55k.is_empty());
        assert!(!buffer_48k.is_empty());
        assert_eq!(buffer_55k.len() % 2, 0); // Stereo
        assert_eq!(buffer_48k.len() % 2, 0); // Stereo
    }

    #[test]
    fn test_save_debug_wav_files() {
        let temp_dir = std::env::temp_dir();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();

        let samples_55k = vec![0i16; 55930 * 2]; // 1 second
        let samples_48k = vec![0i16; 48000 * 2]; // 1 second

        let result = save_debug_wav_files(&samples_55k, &samples_48k, &samples_55k, &samples_48k);
        assert!(result.is_ok());

        // Clean up
        let _ = std::fs::remove_file(temp_dir.join("realtime_55k.wav"));
        let _ = std::fs::remove_file(temp_dir.join("realtime_48k.wav"));
        let _ = std::fs::remove_file(temp_dir.join("post_55k.wav"));
        let _ = std::fs::remove_file(temp_dir.join("post_48k.wav"));

        env::set_current_dir(original_dir).unwrap();
    }
}
