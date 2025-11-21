use crate::events::{EventLog, RegisterEvent};
use crate::player::Player;

#[test]
fn test_convert_events_empty() {
    let events = vec![];
    let processed = Player::convert_events(&events);
    assert_eq!(processed.len(), 0);
}

#[test]
fn test_convert_events_single() {
    let events = vec![RegisterEvent {
        time: 0.5,
        addr: 0x08,
        data: 0x00,
        is_data: None,
    }];

    let processed = Player::convert_events(&events);

    assert_eq!(processed.len(), 1);

    // Single event with addr-data pair, time converted from seconds to samples
    // 0.5 seconds * 55930 Hz = 27965 samples
    assert_eq!(processed[0].time, 27965);
    assert_eq!(processed[0].addr, 0x08);
    assert_eq!(processed[0].data, 0x00);
}

#[test]
fn test_convert_events_multiple() {
    let events = vec![
        RegisterEvent {
            time: 0.0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        },
        RegisterEvent {
            time: 0.0001,
            addr: 0x20,
            data: 0xC7,
            is_data: None,
        },
        RegisterEvent {
            time: 0.0002,
            addr: 0x28,
            data: 0x3E,
            is_data: None,
        },
    ];

    let processed = Player::convert_events(&events);

    assert_eq!(processed.len(), 3);

    // All events converted from seconds to samples
    assert_eq!(processed[0].time, 0);
    assert_eq!(processed[0].addr, 0x08);
    assert_eq!(processed[0].data, 0x00);

    // 0.0001 * 55930 ≈ 6 samples
    assert_eq!(processed[1].time, 6);
    assert_eq!(processed[1].addr, 0x20);
    assert_eq!(processed[1].data, 0xC7);

    // 0.0002 * 55930 ≈ 11 samples
    assert_eq!(processed[2].time, 11);
    assert_eq!(processed[2].addr, 0x28);
    assert_eq!(processed[2].data, 0x3E);
}

#[test]
fn test_convert_events_delay() {
    let events = vec![RegisterEvent {
        time: 0.0,
        addr: 0xFF,
        data: 0xAA,
        is_data: None,
    }];

    let processed = Player::convert_events(&events);

    // Single event with addr-data pair (delay applied in generate_samples)
    assert_eq!(processed.len(), 1);
    assert_eq!(processed[0].time, 0);
}

#[test]
fn test_convert_events_same_time_accumulation() {
    let events = vec![
        RegisterEvent {
            time: 0.0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        },
        RegisterEvent {
            time: 0.0,
            addr: 0x20,
            data: 0xC7,
            is_data: None,
        },
        RegisterEvent {
            time: 0.0,
            addr: 0x28,
            data: 0x3E,
            is_data: None,
        },
    ];

    let processed = Player::convert_events(&events);

    assert_eq!(processed.len(), 3);

    // All events at time 0 as addr-data pairs (delay applied in generate_samples)
    assert_eq!(processed[0].time, 0);
    assert_eq!(processed[0].addr, 0x08);
    assert_eq!(processed[0].data, 0x00);

    assert_eq!(processed[1].time, 0);
    assert_eq!(processed[1].addr, 0x20);
    assert_eq!(processed[1].data, 0xC7);

    assert_eq!(processed[2].time, 0);
    assert_eq!(processed[2].addr, 0x28);
    assert_eq!(processed[2].data, 0x3E);
}

#[test]
fn test_player_creation() {
    let log = EventLog {
        events: vec![RegisterEvent {
            time: 0.0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let player = Player::new(log);

    assert_eq!(player.total_events(), 1);
    assert_eq!(player.events_processed(), 0);
    assert!(!player.is_complete());
}

#[test]
fn test_generate_samples_basic() {
    let log = EventLog {
        events: vec![RegisterEvent {
            time: 0.0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);
    let mut buffer = vec![0i16; 1024];

    let _has_more = player.generate_samples(&mut buffer);

    assert!(player.events_processed() > 0);
}

#[test]
fn test_generate_samples_timing() {
    let log = EventLog {
        events: vec![
            RegisterEvent {
                time: 0.0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 0.017881603406326504, // ~1000 samples at 55930 Hz
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
        ],
    };

    let mut player = Player::new(log);

    assert_eq!(player.total_events(), 2);

    let mut buffer = vec![0i16; 200];
    player.generate_samples(&mut buffer);

    assert_eq!(player.events_processed(), 1);
    assert_eq!(player.current_sample(), 100);

    let mut buffer = vec![0i16; 2000];
    player.generate_samples(&mut buffer);

    assert_eq!(player.events_processed(), 2);
    assert!(player.is_complete());
}

#[test]
fn test_total_samples() {
    let log = EventLog {
        events: vec![RegisterEvent {
            time: 0.017881603406326504, // ~1000 samples at 55930 Hz
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let player = Player::new(log);

    // Events are converted from seconds to samples
    let expected = 1000;
    assert_eq!(player.total_samples(), expected);
}

#[test]
fn test_empty_event_log() {
    let log = EventLog { events: vec![] };

    let player = Player::new(log);

    assert_eq!(player.total_events(), 0);
    assert_eq!(player.total_samples(), 0);
    assert!(player.is_complete());
}

#[test]
fn test_playback_completion() {
    let log = EventLog {
        events: vec![RegisterEvent {
            time: 0.00017881603406326504, // ~10 samples at 55930 Hz
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);
    let mut buffer = vec![0i16; 1024];

    let mut iterations = 0;
    while !player.is_complete() && iterations < 100 {
        player.generate_samples(&mut buffer);
        iterations += 1;
    }

    assert!(player.is_complete());
    assert_eq!(player.events_processed(), player.total_events());
}

#[test]
fn test_sample_rate() {
    assert_eq!(Player::sample_rate(), 55930);
}

#[test]
fn test_interactive_mode_creation() {
    let player = Player::new_interactive();
    assert!(player.is_interactive());
    assert_eq!(player.total_events(), 0);
    assert!(!player.is_complete()); // Interactive mode never completes
}

#[test]
fn test_schedule_register_write() {
    let player = Player::new_interactive();

    // Schedule a register write
    player.schedule_register_write(100, 0x08, 0x78);

    // Check that event was added to the queue
    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();
    assert_eq!(q.len(), 1); // One addr-data pair event

    // Check addr-data pair
    assert_eq!(q[0].time, 100);
    assert_eq!(q[0].addr, 0x08);
    assert_eq!(q[0].data, 0x78);
}

#[test]
fn test_clear_schedule() {
    let player = Player::new_interactive();

    // Schedule some events
    player.schedule_register_write(100, 0x08, 0x78);
    player.schedule_register_write(200, 0x20, 0xC7);

    // Verify events were added
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 2); // 2 register writes = 2 addr-data pair events
    }

    // Clear the schedule
    player.clear_schedule();

    // Verify queue is empty
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 0);
    }
}

#[test]
fn test_clear_schedule_non_interactive_mode() {
    let log = EventLog { events: vec![] };
    let player = Player::new(log);

    // clear_schedule should do nothing in non-interactive mode
    player.clear_schedule(); // Should not panic
    assert!(!player.is_interactive());
}

#[test]
fn test_schedule_events_are_sorted() {
    let player = Player::new_interactive();

    // Schedule events out of order
    player.schedule_register_write(200, 0x20, 0xC7);
    player.schedule_register_write(100, 0x08, 0x78);
    player.schedule_register_write(150, 0x28, 0x3E);

    // Check that events are sorted by time
    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();

    // Should have 3 events (3 register writes as addr-data pairs)
    assert_eq!(q.len(), 3);

    // Verify they are in time order
    for i in 1..q.len() {
        assert!(q[i].time >= q[i - 1].time);
    }
}
