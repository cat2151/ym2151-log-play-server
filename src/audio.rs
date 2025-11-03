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
use crate::player::Player;
#[cfg(feature = "realtime-audio")]
use crate::resampler::{AudioResampler, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};

/// Buffer size for internal audio generation (in stereo samples)
#[cfg(feature = "realtime-audio")]
const GENERATION_BUFFER_SIZE: usize = 2048;

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

        // WAV buffer for capturing samples during playback
        let wav_buffer = Arc::new(Mutex::new(Vec::new()));
        let wav_buffer_clone = wav_buffer.clone();

        // Buffer to hold leftover samples from previous callback
        let leftover_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
        let leftover_buffer_clone = leftover_buffer.clone();

        // Build output stream
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Audio callback - runs in real-time audio thread
                    let mut offset = 0;

                    // First, use any leftover samples from the previous callback
                    if let Ok(mut leftover) = leftover_buffer_clone.lock() {
                        if !leftover.is_empty() {
                            let available = leftover.len();
                            let to_copy = available.min(data.len());
                            data[..to_copy].copy_from_slice(&leftover[..to_copy]);

                            offset += to_copy;

                            // Remove used samples from leftover buffer
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
                            let remaining = data.len() - offset;
                            let to_copy = remaining.min(samples.len());
                            data[offset..offset + to_copy].copy_from_slice(&samples[..to_copy]);

                            offset += to_copy;

                            // If we didn't use all samples, store the remainder for next callback
                            if to_copy < samples.len() {
                                if let Ok(mut leftover) = leftover_buffer_clone.lock() {
                                    *leftover = samples[to_copy..].to_vec();
                                }
                                break; // Device buffer is full
                            }
                        } else {
                            // No more samples available - fill remainder with silence
                            data[offset..].fill(0.0);
                            break;
                        }
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
            if let Err(e) =
                Self::generate_samples_thread(player, sample_tx, command_rx, wav_buffer_clone)
            {
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
    fn generate_samples_thread(
        mut player: Player,
        sample_tx: SyncSender<Vec<f32>>,
        command_rx: Receiver<AudioCommand>,
        wav_buffer: Arc<Mutex<Vec<i16>>>,
    ) -> Result<()> {
        let mut resampler = AudioResampler::new().context("Failed to initialize resampler")?;
        let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];
        let total_samples = player.total_samples();

        let playback_start_time = Instant::now();

        println!("▶  Playing sequence...");
        println!(
            "  Duration: {:.2} seconds",
            total_samples as f64 / OPM_SAMPLE_RATE as f64
        );

        loop {
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

            // Generate samples
            player.generate_samples(&mut generation_buffer);

            // Capture samples to WAV buffer (at native OPM rate, matching C implementation)
            if let Ok(mut buffer) = wav_buffer.lock() {
                buffer.extend_from_slice(&generation_buffer);
            }

            // Resample for audio output
            let resampled = resampler
                .resample(&generation_buffer)
                .context("Failed to resample audio")?;

            // Convert to f32 and send to audio callback
            let f32_samples: Vec<f32> = resampled
                .iter()
                .map(|&sample| sample as f32 / 32768.0)
                .collect();

            // Send samples to audio callback
            // If the queue is full, this will block until space is available
            if sample_tx.send(f32_samples).is_err() {
                // Audio callback has been dropped - playback stopped
                break;
            }

            // Yield to prevent tight spinning
            std::thread::yield_now();
        }

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
