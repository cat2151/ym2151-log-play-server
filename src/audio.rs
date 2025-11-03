// Real-time audio playback functionality
//
// This module provides real-time audio playback using cpal for cross-platform
// audio output. It handles sample generation, resampling, and audio stream
// management with proper synchronization.

#[cfg(feature = "realtime-audio")]
use anyhow::{Context, Result};
#[cfg(feature = "realtime-audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "realtime-audio")]
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
#[cfg(feature = "realtime-audio")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "realtime-audio")]
use std::time::Instant;

#[cfg(feature = "realtime-audio")]
use crate::perf_monitor::{PerfMonitor, ScopedTimer};
#[cfg(feature = "realtime-audio")]
use crate::player::Player;
#[cfg(feature = "realtime-audio")]
use crate::resampler::{AudioResampler, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};

/// Buffer size for internal audio generation (in stereo samples)
#[cfg(feature = "realtime-audio")]
const GENERATION_BUFFER_SIZE: usize = 2048;

/// Size of the sample queue (in stereo samples)
#[cfg(feature = "realtime-audio")]
#[allow(dead_code)]
const SAMPLE_QUEUE_SIZE: usize = 16384;

/// Maximum attempts to detect audio buffer size (100 attempts * 10ms = 1 second timeout)
#[cfg(feature = "realtime-audio")]
const BUFFER_SIZE_DETECTION_MAX_ATTEMPTS: usize = 100;

/// Sleep duration between buffer size detection attempts
#[cfg(feature = "realtime-audio")]
const BUFFER_SIZE_DETECTION_SLEEP_MS: u64 = 10;

/// Commands that can be sent to the audio thread
#[cfg(feature = "realtime-audio")]
enum AudioCommand {
    Stop,
}

/// Real-time audio player for YM2151 playback.
///
/// This struct manages the audio stream and coordinates sample generation
/// with real-time audio output. It runs a background thread that generates
/// samples and feeds them to the audio callback.
///
/// Samples are captured to a WAV buffer during playback for later file output,
/// matching the behavior of the C implementation.
#[cfg(feature = "realtime-audio")]
pub struct AudioPlayer {
    #[allow(dead_code)]
    stream: cpal::Stream,
    command_tx: Sender<AudioCommand>,
    generator_handle: Option<std::thread::JoinHandle<()>>,
    /// WAV buffer captured during playback (native OPM rate, i16 stereo samples)
    wav_buffer: Arc<Mutex<Vec<i16>>>,
}

#[cfg(feature = "realtime-audio")]
impl AudioPlayer {
    /// Create a new audio player and start playback.
    ///
    /// This initializes the audio device, starts the sample generation thread,
    /// and begins playing audio immediately.
    ///
    /// # Parameters
    /// - `player`: The player to generate audio from
    ///
    /// # Returns
    /// A new AudioPlayer instance
    ///
    /// # Errors
    /// Returns error if:
    /// - No audio output device is available
    /// - Audio stream cannot be created
    /// - Resampler initialization fails
    ///
    /// # Examples
    /// ```no_run
    /// # #[cfg(feature = "realtime-audio")]
    /// # {
    /// use ym2151_log_player_rust::events::EventLog;
    /// use ym2151_log_player_rust::player::Player;
    /// use ym2151_log_player_rust::audio::AudioPlayer;
    ///
    /// let log = EventLog::from_file("sample_events.json").unwrap();
    /// let player = Player::new(log);
    /// let audio_player = AudioPlayer::new(player).unwrap();
    ///
    /// // Audio plays in background
    /// std::thread::sleep(std::time::Duration::from_secs(5));
    /// # }
    /// ```
    pub fn new(player: Player) -> Result<Self> {
        // Initialize audio host and device
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

        println!(
            "Using audio device: {}",
            device.name().unwrap_or_else(|_| "Unknown".to_string())
        );

        // Configure audio stream
        // Use platform default buffer size for optimal performance
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(OUTPUT_SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        // Create channels for sample passing
        let (sample_tx, sample_rx): (SyncSender<Vec<f32>>, Receiver<Vec<f32>>) =
            mpsc::sync_channel(8);
        let (command_tx, command_rx) = mpsc::channel();

        // Shared state for tracking playback position
        let position = Arc::new(Mutex::new(0usize));
        let position_clone = position.clone();

        // WAV buffer for capturing samples during playback
        let wav_buffer = Arc::new(Mutex::new(Vec::new()));
        let wav_buffer_clone = wav_buffer.clone();

        // Shared state for audio buffer size (captured from first callback)
        let audio_buffer_size = Arc::new(Mutex::new(None::<usize>));
        let audio_buffer_size_clone = audio_buffer_size.clone();

        // Diagnostic counters for investigating playback speed issue
        let callback_count = Arc::new(Mutex::new(0u64));
        let callback_count_clone = callback_count.clone();
        let samples_received_total = Arc::new(Mutex::new(0u64));
        let samples_received_clone = samples_received_total.clone();
        let samples_used_total = Arc::new(Mutex::new(0u64));
        let samples_used_clone = samples_used_total.clone();
        let samples_silenced_total = Arc::new(Mutex::new(0u64));
        let samples_silenced_clone = samples_silenced_total.clone();

        // Buffer to hold leftover samples from previous callback
        let leftover_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
        let leftover_buffer_clone = leftover_buffer.clone();

        // Build output stream
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Audio callback - runs in real-time audio thread

                    // Capture buffer size on first call
                    if let Ok(mut size) = audio_buffer_size_clone.lock() {
                        if size.is_none() {
                            *size = Some(data.len());
                        }
                    }

                    // Increment callback counter
                    if let Ok(mut count) = callback_count_clone.lock() {
                        *count += 1;
                    }

                    let mut offset = 0;

                    // First, use any leftover samples from the previous callback
                    if let Ok(mut leftover) = leftover_buffer_clone.lock() {
                        if !leftover.is_empty() {
                            let available = leftover.len();
                            let to_copy = available.min(data.len());
                            data[..to_copy].copy_from_slice(&leftover[..to_copy]);

                            // Track usage
                            if let Ok(mut total) = samples_used_clone.lock() {
                                *total += to_copy as u64;
                            }

                            offset += to_copy;

                            // Remove used samples from leftover buffer
                            // Use split_off for better performance: extract the remaining part
                            // and replace the buffer with it
                            if to_copy < leftover.len() {
                                *leftover = leftover.split_off(to_copy);
                            } else {
                                leftover.clear();
                            }
                        }
                    }

                    // Fill the rest of the device buffer with new samples from the channel
                    while offset < data.len() {
                        if let Ok(samples) = sample_rx.try_recv() {
                            // Track that we received a buffer
                            if let Ok(mut total) = samples_received_clone.lock() {
                                *total += samples.len() as u64;
                            }

                            let remaining = data.len() - offset;
                            let to_copy = remaining.min(samples.len());
                            data[offset..offset + to_copy].copy_from_slice(&samples[..to_copy]);

                            // Track usage
                            if let Ok(mut total) = samples_used_clone.lock() {
                                *total += to_copy as u64;
                            }

                            offset += to_copy;

                            // If we didn't use all samples, store the remainder for next callback
                            if to_copy < samples.len() {
                                if let Ok(mut leftover) = leftover_buffer_clone.lock() {
                                    // More efficient: replace buffer contents instead of clear+extend
                                    *leftover = samples[to_copy..].to_vec();
                                }
                                break; // Device buffer is full
                            }
                        } else {
                            // No more samples available - fill remainder with silence
                            let silenced = data.len() - offset;
                            if silenced > 0 {
                                data[offset..].fill(0.0);
                                if let Ok(mut total) = samples_silenced_clone.lock() {
                                    *total += silenced as u64;
                                }
                            }
                            break;
                        }
                    }

                    // Update playback position
                    if let Ok(mut pos) = position_clone.lock() {
                        *pos += offset / 2; // Convert to frame count (stereo)
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
            .context("Failed to build output stream")?;

        // Start the stream
        stream.play().context("Failed to start audio stream")?;

        // Spawn sample generation thread
        let generator_handle = std::thread::spawn(move || {
            if let Err(e) = Self::generate_samples_thread(
                player,
                sample_tx,
                command_rx,
                position,
                wav_buffer_clone,
                audio_buffer_size,
                callback_count,
                samples_received_total,
                samples_used_total,
                samples_silenced_total,
            ) {
                eprintln!("Sample generation error: {}", e);
            }
        });

        Ok(Self {
            stream,
            command_tx,
            generator_handle: Some(generator_handle),
            wav_buffer,
        })
    }

    /// Sample generation thread function.
    ///
    /// This runs in a background thread and continuously generates samples
    /// from the player, resamples them, and sends them to the audio callback.
    /// It also captures the original samples to a WAV buffer for later file output,
    /// matching the behavior of the C implementation.
    ///
    /// Performance monitoring is enabled via PERF_MONITOR environment variable.
    #[allow(clippy::too_many_arguments)]
    fn generate_samples_thread(
        mut player: Player,
        sample_tx: SyncSender<Vec<f32>>,
        command_rx: Receiver<AudioCommand>,
        _position: Arc<Mutex<usize>>,
        wav_buffer: Arc<Mutex<Vec<i16>>>,
        audio_buffer_size: Arc<Mutex<Option<usize>>>,
        callback_count: Arc<Mutex<u64>>,
        samples_received_total: Arc<Mutex<u64>>,
        samples_used_total: Arc<Mutex<u64>>,
        samples_silenced_total: Arc<Mutex<u64>>,
    ) -> Result<()> {
        let mut resampler = AudioResampler::new().context("Failed to initialize resampler")?;

        let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];
        let total_samples = player.total_samples();

        // Wait for audio buffer size to be captured (with timeout)
        let mut actual_audio_buffer_size = None;
        println!("⏳ Waiting for audio device buffer size...");
        for _ in 0..BUFFER_SIZE_DETECTION_MAX_ATTEMPTS {
            if let Ok(size) = audio_buffer_size.lock() {
                if let Some(buffer_size) = *size {
                    actual_audio_buffer_size = Some(buffer_size);
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(
                BUFFER_SIZE_DETECTION_SLEEP_MS,
            ));
        }

        // Print buffer size information
        if let Some(buffer_size) = actual_audio_buffer_size {
            // Buffer size is in samples (stereo interleaved), convert to frames
            let buffer_frames = buffer_size / 2;
            let buffer_duration_ms = (buffer_frames as f64 / OUTPUT_SAMPLE_RATE as f64) * 1000.0;
            println!("✅ Audio device buffer detected:");
            println!(
                "   Buffer size: {} samples ({} stereo frames)",
                buffer_size, buffer_frames
            );
            println!(
                "   Buffer duration: {:.2}ms at {} Hz",
                buffer_duration_ms, OUTPUT_SAMPLE_RATE
            );
        } else {
            println!(
                "⚠️  Could not detect audio buffer size, using generation buffer size as fallback"
            );
        }

        println!(
            "   Generation buffer: {} samples ({} stereo frames) at {} Hz",
            GENERATION_BUFFER_SIZE * 2,
            GENERATION_BUFFER_SIZE,
            OPM_SAMPLE_RATE
        );
        let gen_buffer_duration_ms =
            (GENERATION_BUFFER_SIZE as f64 / OPM_SAMPLE_RATE as f64) * 1000.0;
        println!("   Generation duration: {:.2}ms", gen_buffer_duration_ms);

        // Performance monitoring (enabled via environment variable)
        let enable_perf_monitor = std::env::var("PERF_MONITOR").is_ok();
        let mut perf_monitor = if enable_perf_monitor {
            println!("\n📊 Performance monitoring enabled (PERF_MONITOR=1)");

            // Calculate threshold based on ACTUAL audio buffer size (not generation buffer)
            // This is the real deadline we need to meet to avoid stuttering
            let threshold_ms = if let Some(buffer_size) = actual_audio_buffer_size {
                let buffer_frames = buffer_size / 2;
                (buffer_frames as f64 / OUTPUT_SAMPLE_RATE as f64) * 1000.0
            } else {
                // Fallback to generation buffer duration
                gen_buffer_duration_ms
            };

            println!(
                "   Performance threshold: {:.2}ms (based on audio device buffer)",
                threshold_ms
            );
            println!("   This is the time we have to generate audio before underrun occurs");

            Some(PerfMonitor::new(
                threshold_ms as u64,
                actual_audio_buffer_size,
                GENERATION_BUFFER_SIZE,
            ))
        } else {
            None
        };

        let playback_start_time = Instant::now();

        println!("▶  Playing sequence...");
        println!(
            "  Duration: {:.2} seconds",
            total_samples as f64 / OPM_SAMPLE_RATE as f64
        );
        println!(
            "  Sample rate: {} Hz → {} Hz (resampled for playback)",
            OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE
        );

        loop {
            // Start timing total iteration
            let iteration_start = if perf_monitor.is_some() {
                Some(Instant::now())
            } else {
                None
            };

            // Check for stop command
            if let Ok(AudioCommand::Stop) = command_rx.try_recv() {
                println!("Stopping audio playback...");
                break;
            }

            // Check if playback is complete
            if player.is_complete() && player.current_sample() >= total_samples {
                let elapsed = playback_start_time.elapsed();
                println!("■  Playback complete");
                println!("  Wall-clock time: {:.2} seconds", elapsed.as_secs_f64());
                break;
            }

            // Macro to conditionally time an operation
            // This avoids code duplication while maintaining zero-cost abstraction
            // when performance monitoring is disabled
            macro_rules! timed {
                ($monitor:expr, $stats:ident, $block:expr) => {{
                    if let Some(ref mut monitor) = $monitor {
                        let _timer = ScopedTimer::new(&mut monitor.$stats, monitor.threshold);
                        $block
                    } else {
                        $block
                    }
                }};
            }

            // Generate samples
            timed!(perf_monitor, opm_generation, {
                player.generate_samples(&mut generation_buffer)
            });

            // Capture samples to WAV buffer (at native OPM rate, matching C implementation)
            timed!(perf_monitor, wav_capture, {
                if let Ok(mut buffer) = wav_buffer.lock() {
                    buffer.extend_from_slice(&generation_buffer);
                }
            });

            // Resample for audio output
            let resampled = timed!(perf_monitor, resampling, {
                resampler
                    .resample(&generation_buffer)
                    .context("Failed to resample audio")?
            });

            // Convert to f32 and send to audio callback
            let f32_samples = timed!(perf_monitor, format_conversion, {
                resampled
                    .iter()
                    .map(|&sample| sample as f32 / 32768.0)
                    .collect::<Vec<f32>>()
            });

            // Record total iteration time
            if let (Some(ref mut monitor), Some(start)) = (&mut perf_monitor, iteration_start) {
                let duration = start.elapsed();
                monitor.total_iteration.record(duration, monitor.threshold);
            }

            // Send samples to audio callback
            // If the queue is full, this will block until space is available
            if sample_tx.send(f32_samples).is_err() {
                // Audio callback has been dropped - playback stopped
                break;
            }

            // Yield to prevent tight spinning
            std::thread::yield_now();
        }

        // Print performance report if monitoring was enabled
        if let Some(monitor) = perf_monitor {
            monitor.report();
        }

        // Print diagnostic information to help identify playback speed issue
        println!("\n=== Audio Playback Diagnostics ===");
        if let Ok(count) = callback_count.lock() {
            println!("Total audio callbacks: {}", *count);
        }
        if let Ok(size) = audio_buffer_size.lock() {
            if let Some(buffer_size) = *size {
                let frames = buffer_size / 2;
                let duration_ms = (frames as f64 / OUTPUT_SAMPLE_RATE as f64) * 1000.0;
                println!(
                    "Audio callback buffer size: {} samples ({} stereo frames)",
                    buffer_size, frames
                );
                println!("Audio callback buffer duration: {:.2} ms", duration_ms);
            }
        }
        if let (Ok(received), Ok(used), Ok(silenced)) = (
            samples_received_total.lock(),
            samples_used_total.lock(),
            samples_silenced_total.lock(),
        ) {
            let received_val = *received;
            let used_val = *used;
            let silenced_val = *silenced;
            println!("\nSample statistics:");
            println!("  Samples received from generation: {}", received_val);
            println!("  Samples actually used: {}", used_val);
            println!("  Samples filled with silence: {}", silenced_val);

            if received_val > 0 {
                let usage_pct = (used_val as f64 / received_val as f64) * 100.0;
                println!("  Usage percentage: {:.1}%", usage_pct);
            }

            if used_val > 0 {
                let duration_used_sec = (used_val / 2) as f64 / OUTPUT_SAMPLE_RATE as f64;
                let duration_total_sec =
                    ((used_val + silenced_val) / 2) as f64 / OUTPUT_SAMPLE_RATE as f64;
                println!("\nTiming analysis:");
                println!("  Audio content played: {:.2} seconds", duration_used_sec);
                println!("  Total callback time: {:.2} seconds", duration_total_sec);
                if duration_total_sec > 0.0 {
                    let speedup = duration_total_sec / duration_used_sec;
                    println!("  *** Speed-up factor: {:.2}x ***", speedup);
                    if speedup > 1.1 {
                        println!("\n⚠️  WARNING: Audio is playing FASTER than intended!");
                        println!("  This is caused by the audio callback receiving fewer samples");
                        println!(
                            "  than the device buffer size, causing gaps filled with silence."
                        );
                    }
                }
            }
        }
        println!("==================================\n");

        Ok(())
    }

    /// Wait for playback to complete.
    ///
    /// This blocks until the audio playback finishes naturally.
    pub fn wait(&mut self) {
        if let Some(handle) = self.generator_handle.take() {
            let _ = handle.join();
        }
    }

    /// Stop playback immediately.
    ///
    /// This sends a stop command to the generation thread and waits for it to finish.
    pub fn stop(&mut self) {
        let _ = self.command_tx.send(AudioCommand::Stop);
        self.wait();
    }

    /// Get the captured WAV buffer.
    ///
    /// This returns a copy of all samples captured during playback at the native
    /// OPM sample rate (55930 Hz). Should be called after playback completes.
    ///
    /// # Returns
    /// Vector of interleaved stereo i16 samples
    pub fn get_wav_buffer(&self) -> Vec<i16> {
        self.wav_buffer
            .lock()
            .expect("Failed to lock WAV buffer - mutex poisoned")
            .clone()
    }
}

#[cfg(feature = "realtime-audio")]
impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
#[cfg(feature = "realtime-audio")]
mod tests {
    use super::*;
    use crate::events::{EventLog, RegisterEvent};

    #[test]
    fn test_audio_player_creation() {
        // Create a minimal event log
        let log = EventLog {
            event_count: 1,
            events: vec![RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            }],
        };

        let player = Player::new(log);

        // Try to create audio player - may fail in CI without audio device
        let result = AudioPlayer::new(player);

        // Don't fail the test if no audio device is available
        match result {
            Ok(mut audio_player) => {
                // Successfully created - wait a tiny bit then stop
                std::thread::sleep(std::time::Duration::from_millis(100));
                audio_player.stop();
            }
            Err(e) => {
                println!("Note: Audio player creation failed (expected in CI): {}", e);
            }
        }
    }

    #[test]
    fn test_audio_player_short_playback() {
        // Create a very short event log
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

        match AudioPlayer::new(player) {
            Ok(mut audio_player) => {
                // Let it play for a short time
                std::thread::sleep(std::time::Duration::from_millis(200));
                audio_player.stop();
            }
            Err(e) => {
                println!("Note: Audio player creation failed (expected in CI): {}", e);
            }
        }
    }
}
