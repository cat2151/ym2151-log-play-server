//! AudioPlayer - Main audio playback controller with dual-thread architecture
//!
//! This module implements the AudioPlayer struct which coordinates audio playback
//! through a dual-thread architecture with priority optimization for minimal dropouts.

use anyhow::{Context, Result};
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};
use std::time::Instant;

use crate::audio::buffers::WavBuffers;
use crate::audio::commands::AudioCommand;
use crate::audio::generator;
use crate::audio::scheduler::AudioScheduler;
use crate::audio::stream::AudioStream;
use crate::audio_config::buffer::SYNC_CHANNEL_CAPACITY;
use crate::events::EventLog;
use crate::player::Player;

/// Main audio player with dual-thread architecture
///
/// Implements a sophisticated audio system with the following features:
/// - Generator thread: OPM emulation with MMCSS Pro Audio priority
/// - CPAL callback thread: Hardware audio output with automatic priority
/// - Interactive scheduling: Real-time register writes for live performance
/// - WAV recording: Debugging and analysis support
pub struct AudioPlayer {
    /// Audio output stream (must be kept alive)
    #[allow(dead_code)]
    stream: AudioStream,
    /// Command channel for controlling the generator thread
    command_tx: Sender<AudioCommand>,
    /// Generator thread handle
    generator_handle: Option<std::thread::JoinHandle<()>>,
    /// WAV buffer manager for debugging
    wav_buffers: WavBuffers,
    /// Event log for WAV file generation
    #[allow(dead_code)]
    event_log: Option<EventLog>,
    /// Interactive scheduler for real-time register writes
    scheduler: Option<AudioScheduler>,
}

impl AudioPlayer {
    /// Create a new AudioPlayer with default settings
    ///
    /// # Arguments
    /// * `player` - The player instance that generates OPM samples
    pub fn new(player: Player) -> Result<Self> {
        Self::new_with_quality(player, None, crate::resampler::ResamplingQuality::Linear)
    }

    /// Create a new AudioPlayer with event logging
    ///
    /// # Arguments
    /// * `player` - The player instance
    /// * `event_log` - Optional event log for WAV file generation
    pub fn new_with_log(player: Player, event_log: Option<EventLog>) -> Result<Self> {
        Self::new_with_quality(
            player,
            event_log,
            crate::resampler::ResamplingQuality::Linear,
        )
    }

    /// Create a new AudioPlayer with custom resampling quality
    ///
    /// # Arguments
    /// * `player` - The player instance
    /// * `event_log` - Optional event log for WAV file generation
    /// * `resampling_quality` - Quality setting for the resampler
    pub fn new_with_quality(
        player: Player,
        event_log: Option<EventLog>,
        resampling_quality: crate::resampler::ResamplingQuality,
    ) -> Result<Self> {
        // Set up inter-thread communication
        let (sample_tx, sample_rx): (SyncSender<Vec<f32>>, Receiver<Vec<f32>>) =
            mpsc::sync_channel(SYNC_CHANNEL_CAPACITY);
        let (command_tx, command_rx) = mpsc::channel();

        // Create WAV buffers for debugging
        let wav_buffers = WavBuffers::new();
        let (wav_buffer_55k, wav_buffer_48k) = wav_buffers.get_handles();

        // Create audio output stream
        let stream = AudioStream::new(sample_rx).context("Failed to create audio stream")?;

        // Set up interactive scheduler if needed
        let scheduler = if player.is_interactive() {
            Some(AudioScheduler::new(
                player.get_event_queue(),
                Instant::now(),
            ))
        } else {
            None
        };

        // Clone data for the generator thread
        let event_log_for_thread = event_log.clone();

        // Spawn the generator thread
        let generator_handle = std::thread::spawn(move || {
            if let Err(e) = generator::run_generator_thread(
                player,
                sample_tx,
                command_rx,
                wav_buffer_55k,
                wav_buffer_48k,
                event_log_for_thread,
                resampling_quality,
            ) {
                // Sample generation errors should always be logged
                crate::logging::log_always_server(&format!("Sample generation error: {}", e));
            }
        });

        Ok(Self {
            stream,
            command_tx,
            generator_handle: Some(generator_handle),
            wav_buffers,
            event_log,
            scheduler,
        })
    }

    /// Schedule a register write in interactive mode
    ///
    /// # Arguments
    /// * `scheduled_samples` - Target sample time for the write
    /// * `addr` - Register address
    /// * `data` - Register data
    pub fn schedule_register_write(&self, scheduled_samples: u32, addr: u8, data: u8) {
        if let Some(ref sched) = self.scheduler {
            sched.schedule_register_write(scheduled_samples, addr, data);
        }
    }

    /// Get current playback position in samples for interactive mode
    /// Returns None if not in interactive mode
    pub fn get_current_samples_played(&self) -> Option<u32> {
        if self.scheduler.is_some() {
            // TODO: Implement proper current position tracking
            None
        } else {
            None
        }
    }

    /// Get elapsed time since audio stream started (for interactive mode)
    /// Returns None if not in interactive mode
    pub fn get_audio_elapsed_sec(&self) -> Option<f64> {
        self.scheduler.as_ref().map(|s| s.get_audio_elapsed_sec())
    }

    /// Schedule register write using audio-relative time
    pub fn schedule_register_write_audio_time(
        &self,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<()> {
        if let Some(ref sched) = self.scheduler {
            sched.schedule_register_write_audio_time(event_time_sec, addr, data)
        } else {
            Err(anyhow::anyhow!(
                "Audio-relative scheduling not available in non-interactive mode"
            ))
        }
    }

    /// Schedule register write and return actual scheduled times
    pub fn schedule_register_write_with_times(
        &self,
        scheduled_samples: u32,
        addr: u8,
        data: u8,
    ) -> Option<(u32, u32)> {
        self.scheduler
            .as_ref()
            .map(|s| s.schedule_register_write_with_times(scheduled_samples, addr, data))
    }

    /// Schedule using audio-relative time and return actual scheduled times
    pub fn schedule_register_write_audio_time_with_times(
        &self,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<(u32, u32)> {
        if let Some(ref sched) = self.scheduler {
            sched.schedule_register_write_audio_time_with_times(event_time_sec, addr, data)
        } else {
            Err(anyhow::anyhow!(
                "Audio-relative scheduling not available in non-interactive mode"
            ))
        }
    }

    /// Schedule using fixed base time and return actual scheduled times
    pub fn schedule_register_write_fixed_time_with_times(
        &self,
        base_audio_elapsed: f64,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<(u32, u32)> {
        if let Some(ref sched) = self.scheduler {
            sched.schedule_register_write_fixed_time_with_times(
                base_audio_elapsed,
                event_time_sec,
                addr,
                data,
            )
        } else {
            Err(anyhow::anyhow!(
                "Fixed time scheduling not available in non-interactive mode"
            ))
        }
    }

    /// Schedule using fixed base time with future offset and return actual scheduled times
    pub fn schedule_register_write_fixed_time_with_future_offset(
        &self,
        audio_stream_elapsed_sec: f64,
        future_offset_sec: f64,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<(u32, u32)> {
        if let Some(ref sched) = self.scheduler {
            sched.schedule_register_write_fixed_time_with_future_offset(
                audio_stream_elapsed_sec,
                future_offset_sec,
                event_time_sec,
                addr,
                data,
            )
        } else {
            Err(anyhow::anyhow!(
                "Fixed time with offset scheduling not available in non-interactive mode"
            ))
        }
    }

    /// Clear all scheduled events in interactive mode
    pub fn clear_schedule(&self) {
        if let Some(ref sched) = self.scheduler {
            sched.clear_schedule();
        }
    }

    /// Get current schedule queue size (number of scheduled events)
    pub fn get_scheduled_event_count(&self) -> Option<usize> {
        self.scheduler
            .as_ref()
            .map(|s| s.get_scheduled_event_count())
    }

    /// Wait for playback to complete
    pub fn wait(&mut self) {
        if let Some(handle) = self.generator_handle.take() {
            let _ = handle.join();
        }
    }

    /// Stop playback immediately
    pub fn stop(&mut self) {
        let _ = self.command_tx.send(AudioCommand::Stop);
        self.wait();
    }

    /// Get a copy of the 55kHz WAV buffer contents
    pub fn get_wav_buffer_55k(&self) -> Vec<i16> {
        self.wav_buffers.get_buffer_55k()
    }

    /// Get a copy of the 48kHz WAV buffer contents
    pub fn get_wav_buffer_48k(&self) -> Vec<i16> {
        self.wav_buffers.get_buffer_48k()
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
