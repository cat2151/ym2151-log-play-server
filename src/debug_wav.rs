//! Debug WAV output functionality for audio glitch diagnosis.
//!
//! This module provides functionality to generate 4 different WAV files
//! for debugging audio glitches, particularly with high MUL values.
//!
//! # Output Files
//!
//! When verbose mode is enabled, the following WAV files are generated:
//!
//! - `realtime_55k.wav` - 55930Hz buffer captured during real-time playback
//! - `realtime_48k.wav` - 48000Hz resampled buffer from real-time playback
//! - `post_55k.wav` - 55930Hz buffer from non-real-time rendering
//! - `post_48k.wav` - 48000Hz resampled buffer from non-real-time rendering
//!
//! # Usage
//!
//! Enable verbose mode when running:
//!
//! ```bash
//! # Server mode (Windows)
//! .\ym2151-log-play-server.exe --server --verbose
//!
//! # Client mode (Windows)
//! .\ym2151-log-play-server.exe --client test.json --verbose
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
use crate::logging;
use crate::player::Player;
use crate::resampler::{AudioResampler, ResamplingQuality, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};
use crate::wav_writer;
use anyhow::{Context, Result};

const GENERATION_BUFFER_SIZE: usize = 2048;

/// Check if debug WAV output is enabled via verbose mode.
///
/// Returns true if verbose logging is enabled.
pub fn is_debug_wav_enabled() -> bool {
    logging::is_verbose()
}

/// Generate post-playback WAV buffers at both 55930Hz and 48000Hz.
///
/// This performs non-real-time rendering of the event log, which can be compared
/// with real-time playback buffers to diagnose timing-related audio issues.
///
/// # Arguments
///
/// * `log` - The event log to render
/// * `resampling_quality` - Quality setting for resampling (Linear or HighQuality)
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
/// use ym2151_log_play_server::resampler::ResamplingQuality;
///
/// let log = EventLog::from_file("sample_events.json").unwrap();
/// let (buffer_55k, buffer_48k) = debug_wav::generate_post_playback_buffers(&log, ResamplingQuality::Linear).unwrap();
/// ```
pub fn generate_post_playback_buffers(
    log: &EventLog,
    resampling_quality: ResamplingQuality,
) -> Result<(Vec<i16>, Vec<i16>)> {
    let mut player = Player::new(log.clone());
    let mut resampler = AudioResampler::with_quality(resampling_quality)
        .context("Failed to initialize resampler")?;

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
