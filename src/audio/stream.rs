//! CPAL audio stream management and hardware abstraction
//!
//! This module handles audio device selection, stream configuration, and the audio callback
//! implementation. It abstracts the complexities of CPAL and provides a clean interface
//! for audio output.

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use crate::audio_config::buffer::*;
use crate::logging;
use crate::resampler::OUTPUT_SAMPLE_RATE;

/// Audio stream manager that handles CPAL stream creation and management
pub struct AudioStream {
    #[allow(dead_code)] // Stream must be kept alive for audio playback until dropped
    stream: cpal::Stream,
}

impl AudioStream {
    /// Create and start a new audio stream
    ///
    /// # Arguments
    /// * `sample_rx` - Receiver for f32 audio samples from the generator thread
    ///
    /// # Returns
    /// * `Result<Self>` - The audio stream manager or an error
    pub fn new(sample_rx: Receiver<Vec<f32>>) -> Result<Self> {
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
            buffer_size: CPAL_BUFFER_SIZE,
        };

        // Log the actual buffer size configuration
        logging::log_verbose(&format!(
            "Audio buffer size configured: {:?}",
            CPAL_BUFFER_SIZE
        ));

        let leftover_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
        let leftover_buffer_clone = leftover_buffer.clone();

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    Self::audio_callback(data, &sample_rx, &leftover_buffer_clone);
                },
                |err| {
                    // Audio stream errors should always be logged
                    logging::log_always(&format!("Audio stream error: {}", err));
                },
                None,
            )
            .context("Failed to build output stream")?;

        stream.play().context("Failed to start audio stream")?;

        Ok(Self { stream })
    }

    /// Audio callback function that fills the output buffer with samples
    ///
    /// This function handles buffering and sample management to ensure smooth playback
    /// without dropouts or artifacts.
    fn audio_callback(
        data: &mut [f32],
        sample_rx: &Receiver<Vec<f32>>,
        leftover_buffer: &Arc<Mutex<Vec<f32>>>,
    ) {
        // Log buffer size on first callback (for debugging)
        static FIRST_CALLBACK: std::sync::Once = std::sync::Once::new();
        FIRST_CALLBACK.call_once(|| {
            logging::log_verbose(&format!(
                "Actual audio callback buffer size: {} samples",
                data.len()
            ));
        });

        let mut offset = 0;

        // First, use any leftover samples from the previous callback
        if let Ok(mut leftover) = leftover_buffer.lock() {
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

        // Fill remaining buffer with new samples from the receiver
        while offset < data.len() {
            if let Ok(samples) = sample_rx.try_recv() {
                let remaining = data.len() - offset;
                let to_copy = remaining.min(samples.len());
                data[offset..offset + to_copy].copy_from_slice(&samples[..to_copy]);

                offset += to_copy;

                // Store any excess samples for the next callback
                if to_copy < samples.len() {
                    if let Ok(mut leftover) = leftover_buffer.lock() {
                        *leftover = samples[to_copy..].to_vec();
                    }
                    break;
                }
            } else {
                // No more samples available, fill with silence
                data[offset..].fill(0.0);
                break;
            }
        }
    }
}
