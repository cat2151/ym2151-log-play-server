use std::time::Instant;

use crate::resampler::OPM_SAMPLE_RATE;

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

/// Converts YM2151 sample time to physical time offset
///
/// # Arguments
/// * `sample_time` - Sample time in YM2151 internal sample units (55930 Hz)
///
/// # Returns
/// Time offset in seconds (f64)
pub fn samples_to_sec(sample_time: u32) -> f64 {
    sample_time as f64 / OPM_SAMPLE_RATE as f64
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
