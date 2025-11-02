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
use std::sync::{Arc, Mutex};
#[cfg(feature = "realtime-audio")]
use std::sync::mpsc::{self, Receiver, Sender};

#[cfg(feature = "realtime-audio")]
use crate::player::Player;
#[cfg(feature = "realtime-audio")]
use crate::resampler::{AudioResampler, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};

/// Buffer size for internal audio generation (in stereo samples)
#[cfg(feature = "realtime-audio")]
const GENERATION_BUFFER_SIZE: usize = 2048;

/// Size of the sample queue (in stereo samples)
#[cfg(feature = "realtime-audio")]
const SAMPLE_QUEUE_SIZE: usize = 16384;

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
#[cfg(feature = "realtime-audio")]
pub struct AudioPlayer {
    stream: cpal::Stream,
    command_tx: Sender<AudioCommand>,
    generator_handle: Option<std::thread::JoinHandle<()>>,
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

        println!("Using audio device: {}", device.name().unwrap_or_else(|_| "Unknown".to_string()));

        // Configure audio stream
        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(OUTPUT_SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        // Create channels for sample passing
        let (sample_tx, sample_rx): (Sender<Vec<f32>>, Receiver<Vec<f32>>) = mpsc::sync_channel(8);
        let (command_tx, command_rx) = mpsc::channel();

        // Shared state for tracking playback position
        let position = Arc::new(Mutex::new(0usize));
        let position_clone = position.clone();

        // Build output stream
        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Audio callback - runs in real-time audio thread
                    if let Ok(samples) = sample_rx.try_recv() {
                        let len = data.len().min(samples.len());
                        data[..len].copy_from_slice(&samples[..len]);
                        
                        // Fill remainder with silence if samples are exhausted
                        if len < data.len() {
                            data[len..].fill(0.0);
                        }

                        // Update position
                        if let Ok(mut pos) = position_clone.lock() {
                            *pos += len / 2; // Convert to frame count (stereo)
                        }
                    } else {
                        // No samples available - fill with silence to prevent underrun
                        data.fill(0.0);
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
            if let Err(e) = Self::generate_samples_thread(player, sample_tx, command_rx, position) {
                eprintln!("Sample generation error: {}", e);
            }
        });

        Ok(Self {
            stream,
            command_tx,
            generator_handle: Some(generator_handle),
        })
    }

    /// Sample generation thread function.
    ///
    /// This runs in a background thread and continuously generates samples
    /// from the player, resamples them, and sends them to the audio callback.
    fn generate_samples_thread(
        mut player: Player,
        sample_tx: Sender<Vec<f32>>,
        command_rx: Receiver<AudioCommand>,
        position: Arc<Mutex<usize>>,
    ) -> Result<()> {
        let mut resampler = AudioResampler::new()
            .context("Failed to initialize resampler")?;

        let mut generation_buffer = vec![0i16; GENERATION_BUFFER_SIZE * 2];
        let total_samples = player.total_samples();

        println!("Starting audio playback...");
        println!("  Duration: {:.2} seconds", total_samples as f64 / OPM_SAMPLE_RATE as f64);
        println!("  Sample rate: {} Hz → {} Hz", OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE);

        loop {
            // Check for stop command
            if let Ok(AudioCommand::Stop) = command_rx.try_recv() {
                println!("Stopping audio playback...");
                break;
            }

            // Check if playback is complete
            if player.is_complete() && player.current_sample() >= total_samples {
                println!("Playback complete!");
                break;
            }

            // Generate samples
            let _has_more = player.generate_samples(&mut generation_buffer);

            // Resample
            let resampled = resampler
                .resample(&generation_buffer)
                .context("Failed to resample audio")?;

            // Convert to f32 and send to audio callback
            let mut f32_samples = Vec::with_capacity(resampled.len());
            for &sample in &resampled {
                f32_samples.push(sample as f32 / 32768.0);
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
