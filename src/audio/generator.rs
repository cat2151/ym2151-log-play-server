//! Audio sample generation thread with real-time priority optimization
//!
//! This module implements the core audio generation loop that runs in a separate thread
//! with Windows MMCSS Pro Audio priority. It handles OPM emulation, resampling,
//! and WAV file generation for debugging.

use anyhow::{Context, Result};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::audio::commands::AudioCommand;
use crate::audio_config::buffer::GENERATION_BUFFER_SIZE;
use crate::debug_wav;
use crate::events::EventLog;
use crate::logging;
use crate::player::Player;
use crate::resampler::{AudioResampler, OPM_SAMPLE_RATE};

/// Run the audio sample generation thread
///
/// This function implements the core audio generation loop with the following features:
/// - Windows MMCSS "Pro Audio" priority for minimal latency
/// - OPM emulation at 55930 Hz native rate
/// - Real-time resampling to 48000 Hz output rate
/// - WAV buffer recording for debugging
/// - Graceful shutdown on Stop command
///
/// # Arguments
/// * `player` - The player instance that generates OPM samples
/// * `sample_tx` - Channel sender for resampled f32 audio samples
/// * `command_rx` - Channel receiver for control commands
/// * `wav_buffer_55k` - Shared buffer for 55kHz WAV samples
/// * `wav_buffer_48k` - Shared buffer for 48kHz WAV samples
/// * `event_log` - Optional event log for WAV file generation
/// * `resampling_quality` - Quality setting for the resampler
///
/// # Returns
/// * `Result<()>` - Success or error result
pub fn run_generator_thread(
    mut player: Player,
    sample_tx: SyncSender<Vec<f32>>,
    command_rx: Receiver<AudioCommand>,
    wav_buffer_55k: Arc<Mutex<Vec<i16>>>,
    wav_buffer_48k: Arc<Mutex<Vec<i16>>>,
    event_log: Option<EventLog>,
    resampling_quality: crate::resampler::ResamplingQuality,
) -> Result<()> {
    // Set MMCSS Pro Audio priority for this thread on Windows
    // This handle will automatically revert priority when dropped
    let _mmcss_handle = crate::mmcss::MmcssHandle::set_pro_audio_priority();

    let mut resampler = AudioResampler::with_quality(resampling_quality)
        .context("Failed to initialize resampler")?;
    let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];
    let total_samples = player.total_samples();

    let playback_start_time = Instant::now();

    logging::log_verbose("▶  Playing sequence...");
    logging::log_verbose(&format!(
        "  Duration: {:.2} seconds",
        total_samples as f64 / OPM_SAMPLE_RATE as f64
    ));

    let mut tail_reported = false;

    loop {
        // Check for stop command
        if let Ok(AudioCommand::Stop) = command_rx.try_recv() {
            logging::log_verbose("Stopping audio playback...");
            break;
        }

        // Check if playback should continue
        if !player.should_continue_tail() {
            let elapsed = playback_start_time.elapsed();
            logging::log_verbose("■  Playback complete");
            logging::log_verbose(&format!(
                "  Wall-clock time: {:.2} seconds",
                elapsed.as_secs_f64()
            ));

            if let Some((tail_samples, _)) = player.tail_info() {
                let tail_ms = tail_samples as f64 / OPM_SAMPLE_RATE as f64 * 1000.0;
                logging::log_verbose(&format!(
                    "  演奏データの余韻{}ms 波形生成 OK",
                    tail_ms as u32
                ));
            }

            // Save 4 WAV files if verbose mode and event_log is available
            if logging::is_verbose() {
                if let Some(log) = event_log {
                    save_debug_wav_files(
                        &wav_buffer_55k,
                        &wav_buffer_48k,
                        &log,
                        resampling_quality,
                    );
                }
            }

            break;
        }

        // Report when entering tail generation
        if !tail_reported && player.is_complete() {
            logging::log_verbose("  演奏データ終了、余韻を生成中...");
            tail_reported = true;
        }

        // Generate samples from the OPM emulation
        player.generate_samples(&mut generation_buffer);

        // Store samples in 55kHz WAV buffer
        if let Ok(mut buffer) = wav_buffer_55k.lock() {
            buffer.extend_from_slice(&generation_buffer);
        }

        // Resample to 48kHz output rate
        let resampled = resampler
            .resample(&generation_buffer)
            .context("Failed to resample audio")?;

        // Store resampled samples in 48kHz WAV buffer
        if let Ok(mut buffer) = wav_buffer_48k.lock() {
            buffer.extend_from_slice(&resampled);
        }

        // Convert to f32 format for CPAL output
        let f32_samples: Vec<f32> = resampled
            .iter()
            .map(|&sample| sample as f32 / 32768.0)
            .collect();

        // Send samples to audio output thread
        if sample_tx.send(f32_samples).is_err() {
            break;
        }

        // Yield to prevent hogging CPU
        std::thread::yield_now();
    }

    Ok(())
}

/// Save debug WAV files if verbose logging is enabled
fn save_debug_wav_files(
    wav_buffer_55k: &Arc<Mutex<Vec<i16>>>,
    wav_buffer_48k: &Arc<Mutex<Vec<i16>>>,
    event_log: &EventLog,
    resampling_quality: crate::resampler::ResamplingQuality,
) {
    logging::log_verbose("\n4つのWAVファイルを保存中...");

    // Get realtime buffers
    let realtime_55k = wav_buffer_55k.lock().unwrap().clone();
    let realtime_48k = wav_buffer_48k.lock().unwrap().clone();

    // Generate post-playback buffers
    match debug_wav::generate_post_playback_buffers(event_log, resampling_quality) {
        Ok((post_55k, post_48k)) => {
            // Save all 4 WAV files
            if let Err(e) =
                debug_wav::save_debug_wav_files(&realtime_55k, &realtime_48k, &post_55k, &post_48k)
            {
                logging::log_always(&format!("⚠️  警告: WAVファイルの保存に失敗しました: {}", e));
            } else {
                logging::log_verbose("✅ 4つのWAVファイルの保存が完了しました");
            }
        }
        Err(e) => {
            logging::log_always(&format!(
                "⚠️  警告: post-playbackバッファの生成に失敗しました: {}",
                e
            ));
        }
    }
}
