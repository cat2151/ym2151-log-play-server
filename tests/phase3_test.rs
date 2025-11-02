// Integration tests for Phase 3: Event Processing Engine
//
// These tests validate the event processing, timing, and scheduling functionality.

use ym2151_log_player_rust::events::{EventLog, RegisterEvent};
use ym2151_log_player_rust::player::Player;

#[test]
fn test_phase3_pass1_to_pass2_conversion() {
    // Create a simple event log
    let log = EventLog {
        event_count: 3,
        events: vec![
            RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 100,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
            RegisterEvent {
                time: 200,
                addr: 0x28,
                data: 0x3E,
                is_data: None,
            },
        ],
    };

    let player = Player::new(log);
    
    // Each pass1 event should become 2 pass2 events
    assert_eq!(player.total_events(), 6);
    assert_eq!(player.events_processed(), 0);
}

#[test]
fn test_phase3_event_execution_timing() {
    // Create events with specific timing
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
                time: 500,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
        ],
    };

    let mut player = Player::new(log);
    
    // Generate 100 samples (should execute first 2 events at times 0 and 2)
    let mut buffer = vec![0i16; 200]; // 100 stereo samples
    player.generate_samples(&mut buffer);
    
    assert_eq!(player.current_sample(), 100);
    assert_eq!(player.events_processed(), 2, "Should have processed events at times 0 and 2");
    
    // Generate another 400 samples to reach time 500 (events at time 500 not yet processed)
    let mut buffer = vec![0i16; 800]; // 400 stereo samples
    player.generate_samples(&mut buffer);
    
    assert_eq!(player.current_sample(), 500);
    assert_eq!(player.events_processed(), 2, "Events at time 500 not yet processed (boundary)");
    
    // Generate a few more samples to process events at times 500 and 502
    let mut buffer = vec![0i16; 20]; // 10 stereo samples
    player.generate_samples(&mut buffer);
    
    assert_eq!(player.events_processed(), 4, "Should have processed all events");
    assert!(player.is_complete());
}

#[test]
fn test_phase3_delay_samples() {
    // Create a single event
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 10,
            addr: 0x08,
            data: 0xFF,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);
    
    // The event at time 10 should create:
    // - Address write at time 10
    // - Data write at time 12 (10 + DELAY_SAMPLES=2)
    
    // Generate 11 samples
    let mut buffer = vec![0i16; 22]; // 11 stereo samples
    player.generate_samples(&mut buffer);
    
    assert_eq!(player.events_processed(), 1, "Should have processed address write at time 10");
    
    // Generate 2 more samples to reach time 13
    let mut buffer = vec![0i16; 4]; // 2 stereo samples
    player.generate_samples(&mut buffer);
    
    assert_eq!(player.events_processed(), 2, "Should have processed data write at time 12");
    assert!(player.is_complete());
}

#[test]
fn test_phase3_sample_events_json() {
    // Load the actual sample_events.json file
    let log = EventLog::from_file("sample_events.json")
        .expect("Failed to load sample_events.json");
    
    let mut player = Player::new(log);
    
    // Should have converted 100 pass1 events to 200 pass2 events
    assert_eq!(player.total_events(), 200);
    
    // Process some samples
    let mut buffer = vec![0i16; 1024];
    let mut total_processed = 0;
    
    for _ in 0..10 {
        if player.is_complete() {
            break;
        }
        player.generate_samples(&mut buffer);
        total_processed += buffer.len() / 2;
    }
    
    // Should have processed some events
    assert!(player.events_processed() > 0);
    assert!(total_processed > 0);
}

#[test]
fn test_phase3_complete_playback() {
    // Create a short event sequence
    let log = EventLog {
        event_count: 3,
        events: vec![
            RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 10,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
            RegisterEvent {
                time: 20,
                addr: 0x28,
                data: 0x3E,
                is_data: None,
            },
        ],
    };

    let mut player = Player::new(log);
    let mut buffer = vec![0i16; 100]; // 50 stereo samples
    
    // Process until complete
    let mut iterations = 0;
    while !player.is_complete() && iterations < 1000 {
        player.generate_samples(&mut buffer);
        iterations += 1;
    }
    
    // Should have completed
    assert!(player.is_complete());
    assert_eq!(player.events_processed(), player.total_events());
}

#[test]
fn test_phase3_empty_log() {
    let log = EventLog {
        event_count: 0,
        events: vec![],
    };

    let player = Player::new(log);
    
    assert_eq!(player.total_events(), 0);
    assert_eq!(player.total_samples(), 0);
    assert!(player.is_complete());
}

#[test]
fn test_phase3_event_order_preservation() {
    // Create events that test ordering
    let log = EventLog {
        event_count: 3,
        events: vec![
            RegisterEvent {
                time: 0,
                addr: 0x01,
                data: 0x11,
                is_data: None,
            },
            RegisterEvent {
                time: 1,
                addr: 0x02,
                data: 0x22,
                is_data: None,
            },
            RegisterEvent {
                time: 2,
                addr: 0x03,
                data: 0x33,
                is_data: None,
            },
        ],
    };

    let mut player = Player::new(log);
    
    // Generate enough samples to process all events
    let mut buffer = vec![0i16; 100]; // 50 stereo samples
    player.generate_samples(&mut buffer);
    
    // All events should be processed
    assert!(player.events_processed() >= 6);
}

#[test]
fn test_phase3_buffer_boundaries() {
    // Test that events at buffer boundaries are handled correctly
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 512, // Exactly one common buffer size
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);
    
    // Generate exactly 512 samples
    let mut buffer = vec![0i16; 1024]; // 512 stereo samples
    player.generate_samples(&mut buffer);
    
    assert_eq!(player.current_sample(), 512);
    // Event at time 512 should NOT be processed yet (it's at the boundary, excluded)
    assert!(player.events_processed() < 2);
    
    // Generate one more sample to trigger the event
    let mut buffer = vec![0i16; 2]; // 1 stereo sample
    player.generate_samples(&mut buffer);
    
    // Now the address write at time 512 should be processed
    assert!(player.events_processed() >= 1);
}

#[test]
fn test_phase3_sample_rate() {
    assert_eq!(Player::sample_rate(), 55930);
}

#[test]
fn test_phase3_total_samples_calculation() {
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
    
    // Last event (data write) is at time 1002
    // Total samples should include 1 second (55930 samples) of decay time
    let expected = 1002 + 55930;
    assert_eq!(player.total_samples(), expected);
}
