use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::player::Player;
use crate::resampler::{AudioResampler, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};

const GENERATION_BUFFER_SIZE: usize = 2048;

enum AudioCommand {
    Stop,
}

pub struct AudioPlayer {
    #[allow(dead_code)]
    stream: cpal::Stream,
    command_tx: Sender<AudioCommand>,
    generator_handle: Option<std::thread::JoinHandle<()>>,

    wav_buffer: Arc<Mutex<Vec<i16>>>,
}

impl AudioPlayer {
    pub fn new(player: Player) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

        println!(
            "Using audio device: {}",
            device.name().unwrap_or_else(|_| "Unknown".to_string())
        );

        let config = cpal::StreamConfig {
            channels: 2,
            sample_rate: cpal::SampleRate(OUTPUT_SAMPLE_RATE),
            buffer_size: cpal::BufferSize::Default,
        };

        let (sample_tx, sample_rx): (SyncSender<Vec<f32>>, Receiver<Vec<f32>>) =
            mpsc::sync_channel(8);
        let (command_tx, command_rx) = mpsc::channel();

        let wav_buffer = Arc::new(Mutex::new(Vec::new()));
        let wav_buffer_clone = wav_buffer.clone();

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
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )
            .context("Failed to build output stream")?;

        stream.play().context("Failed to start audio stream")?;

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

        let mut tail_reported = false;

        loop {
            if let Ok(AudioCommand::Stop) = command_rx.try_recv() {
                println!("Stopping audio playback...");
                break;
            }


            if !player.should_continue_tail() {
                let elapsed = playback_start_time.elapsed();
                println!("■  Playback complete");
                println!("  Wall-clock time: {:.2} seconds", elapsed.as_secs_f64());

                if let Some((tail_samples, _)) = player.tail_info() {
                    let tail_ms = tail_samples as f64 / OPM_SAMPLE_RATE as f64 * 1000.0;
                    println!("  演奏データの余韻{}ms 波形生成 OK", tail_ms as u32);
                }
                break;
            }


            if !tail_reported && player.is_complete() {
                println!("  演奏データ終了、余韻を生成中...");
                tail_reported = true;
            }

            player.generate_samples(&mut generation_buffer);

            if let Ok(mut buffer) = wav_buffer.lock() {
                buffer.extend_from_slice(&generation_buffer);
            }

            let resampled = resampler
                .resample(&generation_buffer)
                .context("Failed to resample audio")?;

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

    pub fn get_wav_buffer(&self) -> Vec<i16> {
        self.wav_buffer
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
