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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sec_to_samples() {
        // At 55930 Hz, 1.0 sec = 55930 samples
        assert_eq!(sec_to_samples(1.0), 55930);

        // 0.05 sec (50ms) = 2796.5 samples, rounded to 2797
        assert_eq!(sec_to_samples(0.050), 2797);

        // 0 sec = 0 samples
        assert_eq!(sec_to_samples(0.0), 0);

        // 0.001 sec (1ms) ≈ 55.93 samples, rounded to 56
        assert_eq!(sec_to_samples(0.001), 56);

        // Sample-accurate precision: 1 sample = 1/55930 sec ≈ 0.0000179 sec
        let one_sample_sec = 1.0 / OPM_SAMPLE_RATE as f64;
        assert_eq!(sec_to_samples(one_sample_sec), 1);
    }

    #[test]
    fn test_schedule_event() {
        // Current time at 1000 samples, offset 0 sec
        // Should add 50ms (0.05 sec) latency buffer = 2797 samples
        let scheduled = schedule_event(1000, 0.0);
        assert_eq!(scheduled, 1000 + 2797);

        // Current time at 1000 samples, offset 0.1 sec (100ms)
        // Should add 50ms latency (2797) + 100ms offset (5593) = 8390 samples
        let scheduled = schedule_event(1000, 0.1);
        assert_eq!(scheduled, 1000 + 2797 + 5593);
    }

    #[test]
    fn test_time_tracker_creation() {
        let tracker = TimeTracker::new();
        // Elapsed should be very small (< 0.01 sec) immediately after creation
        let elapsed = tracker.elapsed_sec();
        assert!(elapsed < 0.01);
    }

    #[test]
    fn test_time_tracker_reset() {
        let mut tracker = TimeTracker::new();
        std::thread::sleep(std::time::Duration::from_millis(10));

        let elapsed_before = tracker.elapsed_sec();
        assert!(elapsed_before >= 0.010);

        tracker.reset();
        let elapsed_after = tracker.elapsed_sec();
        assert!(elapsed_after < 0.005); // Should be very small after reset
    }
}
