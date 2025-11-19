use ym2151_log_play_server::resampler::OPM_SAMPLE_RATE;
use ym2151_log_play_server::scheduler::*;

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
