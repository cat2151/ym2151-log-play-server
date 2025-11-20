//! Test to verify that playback generates the correct number of samples
//! This test validates the fix for the early termination issue

use ym2151_log_play_server::events::{EventLog, RegisterEvent};
use ym2151_log_play_server::player::Player;
use ym2151_log_play_server::resampler::OPM_SAMPLE_RATE;

#[test]
fn test_player_generates_all_samples() {
    // Create a log with the last event at 1500ms
    // At OPM_SAMPLE_RATE, this is 83895 samples
    let target_samples = ((1500.0 / 1000.0) * OPM_SAMPLE_RATE as f64) as u32;

    let log = EventLog {
        events: vec![
            RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: target_samples,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
        ],
    };

    let mut player = Player::new(log);

    // total_samples() returns the time of the LAST processed event
    // For an event at time T, it creates two processed events:
    // - Address register write at T
    // - Data register write at T + DELAY_SAMPLES
    // The player adds DELAY_SAMPLES between address and data writes
    // So for the last event, total_samples() will be slightly larger than the event time
    let actual_total = player.total_samples();
    assert!(
        actual_total >= target_samples && actual_total <= target_samples + 10,
        "Expected total_samples to be close to {} but got {}",
        target_samples,
        actual_total
    );

    // Generate samples until we reach the actual total
    let mut buffer = vec![0i16; 2048 * 2]; // GENERATION_BUFFER_SIZE * 2
    let mut generated_count = 0u32;

    while player.current_sample() < actual_total {
        player.generate_samples(&mut buffer);
        generated_count += 1;
    }

    // Verify we generated enough samples
    let final_sample_count = player.current_sample();
    assert!(
        final_sample_count >= actual_total,
        "Player stopped at {} samples, expected at least {}",
        final_sample_count,
        actual_total
    );

    // Verify the duration is approximately correct (within one buffer size)
    let expected_duration_ms = (target_samples as f64 / OPM_SAMPLE_RATE as f64) * 1000.0;
    let actual_duration_ms = (final_sample_count as f64 / OPM_SAMPLE_RATE as f64) * 1000.0;

    let duration_diff = (actual_duration_ms - expected_duration_ms).abs();
    let max_acceptable_diff = (2048.0 / OPM_SAMPLE_RATE as f64) * 1000.0; // One buffer duration

    assert!(
        duration_diff <= max_acceptable_diff,
        "Duration difference too large: {:.2}ms (max acceptable: {:.2}ms)",
        duration_diff,
        max_acceptable_diff
    );

    println!(
        "✓ Generated {} iterations, {} samples total ({:.2}ms)",
        generated_count, final_sample_count, actual_duration_ms
    );
}

#[test]
fn test_player_generates_samples_after_last_event() {
    // This test ensures that the player continues generating samples
    // even after all events are processed (is_complete() == true)

    let target_samples = 10000u32;

    let log = EventLog {
        events: vec![RegisterEvent {
            time: 0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);
    let mut buffer = vec![0i16; 2048 * 2];

    // Generate samples until we exceed the target
    while player.current_sample() < target_samples {
        player.generate_samples(&mut buffer);
    }

    // The player should be "complete" (all events processed)
    // but still able to generate samples
    assert!(player.is_complete(), "All events should be processed");
    assert!(
        player.current_sample() >= target_samples,
        "Should have generated at least {} samples",
        target_samples
    );
}
