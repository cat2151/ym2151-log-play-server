use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::debug_wav;
use crate::events::EventLog;
use crate::logging;
use crate::player::Player;
use crate::resampler::{AudioResampler, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};

const GENERATION_BUFFER_SIZE: usize = 2048;

enum AudioCommand {
    Stop,
}

pub struct AudioPlayer {
    #[allow(dead_code)] // Stream must be kept alive for audio playback until dropped
    stream: cpal::Stream,
    command_tx: Sender<AudioCommand>,
    generator_handle: Option<std::thread::JoinHandle<()>>,

    wav_buffer_55k: Arc<Mutex<Vec<i16>>>,
    wav_buffer_48k: Arc<Mutex<Vec<i16>>>,
    #[allow(dead_code)] // Event log stored for potential future use
    event_log: Option<EventLog>,

    // For interactive mode: shared reference to player's event queue
    player_event_queue:
        Option<Arc<Mutex<std::collections::VecDeque<crate::player::ProcessedEvent>>>>,
}

impl AudioPlayer {
    pub fn new(player: Player) -> Result<Self> {
        Self::new_with_quality(player, None, crate::resampler::ResamplingQuality::Linear)
    }

    pub fn new_with_log(player: Player, event_log: Option<EventLog>) -> Result<Self> {
        Self::new_with_quality(
            player,
            event_log,
            crate::resampler::ResamplingQuality::Linear,
        )
    }

    pub fn new_with_quality(
        player: Player,
        event_log: Option<EventLog>,
        resampling_quality: crate::resampler::ResamplingQuality,
    ) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

        // Device info respects verbose flag to avoid TUI disruption
        logging::log_verbose(&format!(
            "Using audio device: {}",
            device.name().unwrap_or_else(|_| "Unknown".to_string())
        ));

        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(OUTPUT_SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        let (sample_tx, sample_rx): (SyncSender<Vec<f32>>, Receiver<Vec<f32>>) =
            mpsc::sync_channel(8);
        let (command_tx, command_rx) = mpsc::channel();

        let wav_buffer_55k = Arc::new(Mutex::new(Vec::new()));
        let wav_buffer_55k_clone = wav_buffer_55k.clone();

        let wav_buffer_48k = Arc::new(Mutex::new(Vec::new()));
        let wav_buffer_48k_clone = wav_buffer_48k.clone();

        let leftover_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
        let leftover_buffer_clone = leftover_buffer.clone();

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut offset = 0;

                    if let Ok(mut leftover) = leftover_buffer_clone.lock() {
                        if !leftover.is_empty() {
                            let available = leftover.len();
                            let to_copy = available.min(data.len());
                            data[..to_copy].copy_from_slice(&leftover[..to_copy]);

                            offset += to_copy;

                            if to_copy < leftover.len() {
                                *leftover = leftover.split_off(to_copy);
                            } else {
                                leftover.clear();
                            }
                        }
                    }

                    while offset < data.len() {
                        if let Ok(samples) = sample_rx.try_recv() {
                            let remaining = data.len() - offset;
                            let to_copy = remaining.min(samples.len());
                            data[offset..offset + to_copy].copy_from_slice(&samples[..to_copy]);

                            offset += to_copy;

                            if to_copy < samples.len() {
                                if let Ok(mut leftover) = leftover_buffer_clone.lock() {
                                    *leftover = samples[to_copy..].to_vec();
                                }
                                break;
                            }
                        } else {
                            data[offset..].fill(0.0);
                            break;
                        }
                    }
                },
                |err| {
                    // Audio stream errors should always be logged
                    logging::log_always(&format!("Audio stream error: {}", err));
                },
                None,
            )
            .context("Failed to build output stream")?;

        stream.play().context("Failed to start audio stream")?;

        let player_event_queue = if player.is_interactive() {
            Some(player.get_event_queue())
        } else {
            None
        };

        let event_log_for_thread = event_log.clone();
        let generator_handle = std::thread::spawn(move || {
            if let Err(e) = Self::generate_samples_thread(
                player,
                sample_tx,
                command_rx,
                wav_buffer_55k_clone,
                wav_buffer_48k_clone,
                event_log_for_thread,
                resampling_quality,
            ) {
                // Sample generation errors should always be logged
                logging::log_always(&format!("Sample generation error: {}", e));
            }
        });

        Ok(Self {
            stream,
            command_tx,
            generator_handle: Some(generator_handle),
            wav_buffer_55k,
            wav_buffer_48k,
            event_log,
            player_event_queue,
        })
    }

    /// Schedule a register write in interactive mode
    pub fn schedule_register_write(&self, scheduled_samples: u32, addr: u8, data: u8) {
        if let Some(ref queue) = self.player_event_queue {
            // Lock the queue and add events
            let mut q = queue.lock().unwrap();
            q.push_back(crate::player::ProcessedEvent {
                time: scheduled_samples,
                port: 0, // OPM_ADDRESS_REGISTER
                value: addr,
            });
            q.push_back(crate::player::ProcessedEvent {
                time: scheduled_samples + 2, // DELAY_SAMPLES
                port: 1,                     // OPM_DATA_REGISTER
                value: data,
            });
        }
    }

    /// Clear all scheduled events in interactive mode
    /// This allows seamless phrase transitions without audio gaps
    pub fn clear_schedule(&self) {
        if let Some(ref queue) = self.player_event_queue {
            let mut q = queue.lock().unwrap();
            q.clear();
        }
    }

    fn generate_samples_thread(
        mut player: Player,
        sample_tx: SyncSender<Vec<f32>>,
        command_rx: Receiver<AudioCommand>,
        wav_buffer_55k: Arc<Mutex<Vec<i16>>>,
        wav_buffer_48k: Arc<Mutex<Vec<i16>>>,
        event_log: Option<EventLog>,
        resampling_quality: crate::resampler::ResamplingQuality,
    ) -> Result<()> {
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
            if let Ok(AudioCommand::Stop) = command_rx.try_recv() {
                logging::log_verbose("Stopping audio playback...");
                break;
            }

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
                        logging::log_verbose("\n4つのWAVファイルを保存中...");

                        // Get realtime buffers
                        let realtime_55k = wav_buffer_55k.lock().unwrap().clone();
                        let realtime_48k = wav_buffer_48k.lock().unwrap().clone();

                        // Generate post-playback buffers
                        match debug_wav::generate_post_playback_buffers(&log, resampling_quality) {
                            Ok((post_55k, post_48k)) => {
                                // Save all 4 WAV files
                                if let Err(e) = debug_wav::save_debug_wav_files(
                                    &realtime_55k,
                                    &realtime_48k,
                                    &post_55k,
                                    &post_48k,
                                ) {
                                    logging::log_always(&format!(
                                        "⚠️  警告: WAVファイルの保存に失敗しました: {}",
                                        e
                                    ));
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
                }

                break;
            }

            if !tail_reported && player.is_complete() {
                logging::log_verbose("  演奏データ終了、余韻を生成中...");
                tail_reported = true;
            }

            player.generate_samples(&mut generation_buffer);

            if let Ok(mut buffer) = wav_buffer_55k.lock() {
                buffer.extend_from_slice(&generation_buffer);
            }

            let resampled = resampler
                .resample(&generation_buffer)
                .context("Failed to resample audio")?;

            if let Ok(mut buffer) = wav_buffer_48k.lock() {
                buffer.extend_from_slice(&resampled);
            }

            let f32_samples: Vec<f32> = resampled
                .iter()
                .map(|&sample| sample as f32 / 32768.0)
                .collect();

            if sample_tx.send(f32_samples).is_err() {
                break;
            }

            std::thread::yield_now();
        }

        Ok(())
    }

    pub fn wait(&mut self) {
        if let Some(handle) = self.generator_handle.take() {
            let _ = handle.join();
        }
    }

    pub fn stop(&mut self) {
        let _ = self.command_tx.send(AudioCommand::Stop);
        self.wait();
    }

    pub fn get_wav_buffer_55k(&self) -> Vec<i16> {
        self.wav_buffer_55k
            .lock()
            .expect("Failed to lock WAV buffer - mutex poisoned")
            .clone()
    }

    pub fn get_wav_buffer_48k(&self) -> Vec<i16> {
        self.wav_buffer_48k
            .lock()
            .expect("Failed to lock WAV buffer - mutex poisoned")
            .clone()
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{EventLog, RegisterEvent};

    #[test]
    fn test_audio_player_creation() {
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

        let result = AudioPlayer::new(player);

        match result {
            Ok(mut audio_player) => {
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
                std::thread::sleep(std::time::Duration::from_millis(200));
                audio_player.stop();
            }
            Err(e) => {
                println!("Note: Audio player creation failed (expected in CI): {}", e);
            }
        }
    }
}
