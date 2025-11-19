use std::time::Instant;

use crate::resampler::OPM_SAMPLE_RATE;

/// Latency buffer in seconds (Web Audio-style scheduling)
/// This provides jitter compensation for interactive playback
const LATENCY_BUFFER_SEC: f64 = 0.050; // 50ms

/// Converts physical time offset to YM2151 sample time
///
/// # Arguments
/// * `time_offset_sec` - Time offset in seconds (f64) from current moment
///
/// # Returns
/// Sample time in YM2151 internal sample units (55930 Hz)
pub fn sec_to_samples(time_offset_sec: f64) -> u32 {
    (time_offset_sec * OPM_SAMPLE_RATE as f64).round() as u32
}

/// Schedules an event with latency compensation
///
/// # Arguments
/// * `current_sample_time` - Current playback position in samples
/// * `time_offset_sec` - Requested time offset in seconds (f64)
///
/// # Returns
/// Scheduled sample time with latency buffer applied
pub fn schedule_event(current_sample_time: u32, time_offset_sec: f64) -> u32 {
    let latency_samples = sec_to_samples(LATENCY_BUFFER_SEC);
    let offset_samples = sec_to_samples(time_offset_sec);
    current_sample_time + latency_samples + offset_samples
}

/// Physical time tracker for interactive mode
pub struct TimeTracker {
    start_time: Instant,
}

impl TimeTracker {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// Get elapsed time in seconds (f64) since creation
    pub fn elapsed_sec(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    /// Convert elapsed time to sample time
    pub fn elapsed_samples(&self) -> u32 {
        sec_to_samples(self.elapsed_sec())
    }

    /// Reset the time tracker
    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

impl Default for TimeTracker {
    fn default() -> Self {
        Self::new()
    }
}


