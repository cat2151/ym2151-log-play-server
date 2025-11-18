use std::time::Instant;

use crate::resampler::OPM_SAMPLE_RATE;

/// Latency buffer in milliseconds (Web Audio-style scheduling)
/// This provides jitter compensation for interactive playback
const LATENCY_BUFFER_MS: u32 = 50;

/// Converts physical time offset to YM2151 sample time
///
/// # Arguments
/// * `time_offset_ms` - Time offset in milliseconds from current moment
///
/// # Returns
/// Sample time in YM2151 internal sample units (55930 Hz)
pub fn ms_to_samples(time_offset_ms: u32) -> u32 {
    (time_offset_ms as u64 * OPM_SAMPLE_RATE as u64 / 1000) as u32
}

/// Schedules an event with latency compensation
///
/// # Arguments
/// * `current_sample_time` - Current playback position in samples
/// * `time_offset_ms` - Requested time offset in milliseconds
///
/// # Returns
/// Scheduled sample time with latency buffer applied
pub fn schedule_event(current_sample_time: u32, time_offset_ms: u32) -> u32 {
    let latency_samples = ms_to_samples(LATENCY_BUFFER_MS);
    let offset_samples = ms_to_samples(time_offset_ms);
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

    /// Get elapsed time in milliseconds since creation
    pub fn elapsed_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }

    /// Convert elapsed time to sample time
    pub fn elapsed_samples(&self) -> u32 {
        ms_to_samples(self.elapsed_ms() as u32)
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
    fn test_ms_to_samples() {
        // At 55930 Hz, 1000ms = 55930 samples
        assert_eq!(ms_to_samples(1000), 55930);

        // 50ms = 2796.5 samples, rounded to 2796
        assert_eq!(ms_to_samples(50), 2796);

        // 0ms = 0 samples
        assert_eq!(ms_to_samples(0), 0);

        // 1ms ≈ 55.93 samples, rounded to 55
        assert_eq!(ms_to_samples(1), 55);
    }

    #[test]
    fn test_schedule_event() {
        // Current time at 1000 samples, offset 0ms
        // Should add 50ms latency buffer = 2796 samples
        let scheduled = schedule_event(1000, 0);
        assert_eq!(scheduled, 1000 + 2796);

        // Current time at 1000 samples, offset 100ms
        // Should add 50ms latency (2796) + 100ms offset (5593) = 8389 samples
        let scheduled = schedule_event(1000, 100);
        assert_eq!(scheduled, 1000 + 2796 + 5593);
    }

    #[test]
    fn test_time_tracker_creation() {
        let tracker = TimeTracker::new();
        // Elapsed should be very small (< 10ms) immediately after creation
        let elapsed = tracker.elapsed_ms();
        assert!(elapsed < 10);
    }

    #[test]
    fn test_time_tracker_reset() {
        let mut tracker = TimeTracker::new();
        std::thread::sleep(std::time::Duration::from_millis(10));

        let elapsed_before = tracker.elapsed_ms();
        assert!(elapsed_before >= 10);

        tracker.reset();
        let elapsed_after = tracker.elapsed_ms();
        assert!(elapsed_after < 5); // Should be very small after reset
    }
}
