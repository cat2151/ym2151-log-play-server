//! Interactive audio scheduling for real-time register writes
//!
//! This module handles the scheduling of OPM register writes in interactive mode,
//! allowing real-time manipulation of the audio stream. It provides time-based
//! scheduling with sample-accurate timing.

use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Interactive audio scheduler for real-time register writes
pub struct AudioScheduler {
    /// Shared reference to player's event queue
    player_event_queue: Arc<Mutex<std::collections::VecDeque<crate::player::ProcessedEvent>>>,
    /// Audio stream start time for continuous time-based scheduling
    audio_start_time: Instant,
}

impl AudioScheduler {
    /// Create a new audio scheduler
    ///
    /// # Arguments
    /// * `player_event_queue` - Shared event queue from the player
    /// * `audio_start_time` - Time when the audio stream started
    pub fn new(
        player_event_queue: Arc<Mutex<std::collections::VecDeque<crate::player::ProcessedEvent>>>,
        audio_start_time: Instant,
    ) -> Self {
        Self {
            player_event_queue,
            audio_start_time,
        }
    }

    /// Schedule a register write in interactive mode
    ///
    /// # Arguments
    /// * `scheduled_samples` - Target sample time for the write
    /// * `addr` - Register address
    /// * `data` - Register data
    pub fn schedule_register_write(&self, scheduled_samples: u32, addr: u8, data: u8) {
        // Store addr-data pair directly in a single event
        // The 2-sample delay between address and data writes will be applied
        // at the final stage in generate_samples()
        let mut q = self.player_event_queue.lock().unwrap();

        q.push_back(crate::player::ProcessedEvent {
            time: scheduled_samples,
            addr,
            data,
        });
    }

    /// Get elapsed time since audio stream started
    pub fn get_audio_elapsed_sec(&self) -> f64 {
        self.audio_start_time.elapsed().as_secs_f64()
    }

    /// Schedule register write using audio-relative time
    ///
    /// This method uses the audio stream start time as reference
    ///
    /// # Arguments
    /// * `event_time_sec` - Time relative to current audio position
    /// * `addr` - Register address
    /// * `data` - Register data
    pub fn schedule_register_write_audio_time(
        &self,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<()> {
        let elapsed_sec = self.audio_start_time.elapsed().as_secs_f64();
        let absolute_time_sec = elapsed_sec + event_time_sec;
        let scheduled_samples = crate::scheduler::sec_to_samples(absolute_time_sec);
        self.schedule_register_write(scheduled_samples, addr, data);
        Ok(())
    }

    /// Schedule register write and return actual scheduled times
    ///
    /// Returns (address_time, data_time) tuple
    ///
    /// # Arguments
    /// * `scheduled_samples` - Target sample time
    /// * `addr` - Register address
    /// * `data` - Register data
    pub fn schedule_register_write_with_times(
        &self,
        scheduled_samples: u32,
        addr: u8,
        data: u8,
    ) -> (u32, u32) {
        // Store addr-data pair directly in a single event
        // The 2-sample delay between address and data writes will be applied
        // at the final stage in generate_samples()
        let mut q = self.player_event_queue.lock().unwrap();

        q.push_back(crate::player::ProcessedEvent {
            time: scheduled_samples,
            addr,
            data,
        });

        // Both are scheduled at the same time, delay will be applied at final stage
        (scheduled_samples, scheduled_samples)
    }

    /// Schedule using audio-relative time and return actual scheduled times
    ///
    /// # Arguments
    /// * `event_time_sec` - Time relative to current audio position
    /// * `addr` - Register address
    /// * `data` - Register data
    pub fn schedule_register_write_audio_time_with_times(
        &self,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<(u32, u32)> {
        let elapsed_sec = self.audio_start_time.elapsed().as_secs_f64();
        let absolute_time_sec = elapsed_sec + event_time_sec;
        let scheduled_samples = crate::scheduler::sec_to_samples(absolute_time_sec);

        let times = self.schedule_register_write_with_times(scheduled_samples, addr, data);
        Ok(times)
    }

    /// Schedule using fixed base time and return actual scheduled times
    ///
    /// This prevents time drift during batch scheduling
    ///
    /// # Arguments
    /// * `base_audio_elapsed` - Fixed base time in seconds
    /// * `event_time_sec` - Event time offset in seconds
    /// * `addr` - Register address
    /// * `data` - Register data
    pub fn schedule_register_write_fixed_time_with_times(
        &self,
        base_audio_elapsed: f64,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<(u32, u32)> {
        let absolute_time_sec = base_audio_elapsed + event_time_sec;
        let scheduled_samples = crate::scheduler::sec_to_samples(absolute_time_sec);

        let times = self.schedule_register_write_with_times(scheduled_samples, addr, data);
        Ok(times)
    }

    /// Schedule using fixed base time with future offset and return actual scheduled times
    ///
    /// This prevents time drift during batch scheduling and adds safety buffer
    ///
    /// # Arguments
    /// * `audio_stream_elapsed_sec` - Current audio stream elapsed time
    /// * `future_offset_sec` - Safety buffer for future scheduling
    /// * `event_time_sec` - Event time offset
    /// * `addr` - Register address
    /// * `data` - Register data
    pub fn schedule_register_write_fixed_time_with_future_offset(
        &self,
        audio_stream_elapsed_sec: f64,
        future_offset_sec: f64,
        event_time_sec: f64,
        addr: u8,
        data: u8,
    ) -> Result<(u32, u32)> {
        let absolute_time_sec = audio_stream_elapsed_sec + future_offset_sec + event_time_sec;
        let scheduled_samples = crate::scheduler::sec_to_samples(absolute_time_sec);

        let times = self.schedule_register_write_with_times(scheduled_samples, addr, data);
        Ok(times)
    }

    /// Clear all scheduled events in interactive mode
    ///
    /// This allows seamless phrase transitions without audio gaps
    pub fn clear_schedule(&self) {
        let mut q = self.player_event_queue.lock().unwrap();
        q.clear();
    }

    /// Get current schedule queue size (number of scheduled events)
    pub fn get_scheduled_event_count(&self) -> usize {
        let q = self.player_event_queue.lock().unwrap();
        q.len()
    }
}
