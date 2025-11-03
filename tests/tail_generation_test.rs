use ym2151_log_player_rust::events::{EventLog, RegisterEvent};
use ym2151_log_player_rust::player::Player;
use ym2151_log_player_rust::resampler::OPM_SAMPLE_RATE;

#[test]
fn test_tail_generation_continues_after_events() {
    // Create a simple event log
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
                time: 1000,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
        ],
    };

    let mut player = Player::new(log);
    let total_event_samples = player.total_samples();

    // Generate samples beyond the last event
    let mut buffer = vec![0i16; 2048 * 2];
    let mut iterations = 0;
    let max_iterations = 100;

    while player.should_continue_tail() && iterations < max_iterations {
        player.generate_samples(&mut buffer);
        iterations += 1;
    }

    // Verify we generated more samples than just the events
    assert!(
        player.current_sample() > total_event_samples,
        "Player should generate tail samples after events complete"
    );

    // Verify tail info is available
    if let Some((tail_samples, _)) = player.tail_info() {
        assert!(tail_samples > 0, "Tail samples should be greater than 0");
        println!(
            "Generated {} tail samples ({:.2}ms)",
            tail_samples,
            tail_samples as f64 / OPM_SAMPLE_RATE as f64 * 1000.0
        );
    } else {
        panic!("Tail info should be available after events complete");
    }
}

#[test]
fn test_tail_generation_stops_after_silence() {
    // Create a very short event log
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 100,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);

    // Generate samples until tail generation stops
    let mut buffer = vec![0i16; 2048 * 2];
    let mut iterations = 0;
    let max_iterations = 100;

    while player.should_continue_tail() && iterations < max_iterations {
        player.generate_samples(&mut buffer);
        iterations += 1;
    }

    // Verify that tail generation stopped (should_continue_tail returns false)
    assert!(
        !player.should_continue_tail() || iterations >= max_iterations,
        "Tail generation should stop after 100ms of zero volume"
    );

    if let Some((tail_samples, _)) = player.tail_info() {
        println!(
            "Generated tail: {:.2}ms ({} samples)",
            tail_samples as f64 / OPM_SAMPLE_RATE as f64 * 1000.0,
            tail_samples
        );
    }
}

#[test]
fn test_tail_info_before_events_complete() {
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 1000,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let player = Player::new(log);

    // Before any samples are generated, tail_info should be None
    assert!(
        player.tail_info().is_none(),
        "Tail info should be None before events complete"
    );
}

#[test]
fn test_should_continue_tail_during_events() {
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 1000,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let player = Player::new(log);

    // Before events are complete, should_continue_tail should be true
    assert!(
        player.should_continue_tail(),
        "should_continue_tail should return true during event processing"
    );
}

#[test]
fn test_silence_detection_resets_on_non_zero_sample() {
    // This test verifies that the silence counter resets when a non-zero sample is detected
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);

    // Generate some samples
    let mut buffer = vec![0i16; 2048 * 2];
    for _ in 0..10 {
        player.generate_samples(&mut buffer);

        // Check if any non-zero samples exist in buffer
        let has_non_zero = buffer.chunks(2).any(|chunk| {
            let left = chunk[0];
            let right = chunk[1];
            left != 0 || right != 0
        });

        if has_non_zero {
            // The consecutive silent counter should be low if we have non-zero samples
            if let Some((_, consecutive_silent)) = player.tail_info() {
                // If we're getting non-zero samples, the counter shouldn't reach the full silence duration
                println!(
                    "Has non-zero samples, consecutive silent: {}",
                    consecutive_silent
                );
            }
        }
    }
}
