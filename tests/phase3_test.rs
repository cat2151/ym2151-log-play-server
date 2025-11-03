use ym2151_log_player_rust::events::{EventLog, RegisterEvent};
use ym2151_log_player_rust::player::Player;

#[test]
fn test_phase3_pass1_to_pass2_conversion() {
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

    assert_eq!(player.total_events(), 6);
    assert_eq!(player.events_processed(), 0);
}

#[test]
fn test_phase3_event_execution_timing() {
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

    let mut buffer = vec![0i16; 200];
    player.generate_samples(&mut buffer);

    assert_eq!(player.current_sample(), 100);
    assert_eq!(
        player.events_processed(),
        2,
        "Should have processed events at times 0 and 2"
    );

    let mut buffer = vec![0i16; 800];
    player.generate_samples(&mut buffer);

    assert_eq!(player.current_sample(), 500);
    assert_eq!(
        player.events_processed(),
        2,
        "Events at time 500 not yet processed (boundary)"
    );

    let mut buffer = vec![0i16; 20];
    player.generate_samples(&mut buffer);

    assert_eq!(
        player.events_processed(),
        4,
        "Should have processed all events"
    );
    assert!(player.is_complete());
}

#[test]
fn test_phase3_delay_samples() {
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

    let mut buffer = vec![0i16; 22];
    player.generate_samples(&mut buffer);

    assert_eq!(
        player.events_processed(),
        1,
        "Should have processed address write at time 10"
    );

    let mut buffer = vec![0i16; 4];
    player.generate_samples(&mut buffer);

    assert_eq!(
        player.events_processed(),
        2,
        "Should have processed data write at time 12"
    );
    assert!(player.is_complete());
}

#[test]
fn test_phase3_sample_events_json() {
    let log = EventLog::from_file("sample_events.json").expect("Failed to load sample_events.json");

    let mut player = Player::new(log);

    assert_eq!(player.total_events(), 200);

    let mut buffer = vec![0i16; 1024];
    let mut total_processed = 0;

    for _ in 0..10 {
        if player.is_complete() {
            break;
        }
        player.generate_samples(&mut buffer);
        total_processed += buffer.len() / 2;
    }

    assert!(player.events_processed() > 0);
    assert!(total_processed > 0);
}

#[test]
fn test_phase3_complete_playback() {
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
    let mut buffer = vec![0i16; 100];

    let mut iterations = 0;
    while !player.is_complete() && iterations < 1000 {
        player.generate_samples(&mut buffer);
        iterations += 1;
    }

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

    let mut buffer = vec![0i16; 100];
    player.generate_samples(&mut buffer);

    assert!(player.events_processed() >= 6);
}

#[test]
fn test_phase3_buffer_boundaries() {
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 512,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let mut player = Player::new(log);

    let mut buffer = vec![0i16; 1024];
    player.generate_samples(&mut buffer);

    assert_eq!(player.current_sample(), 512);

    assert!(player.events_processed() < 2);

    let mut buffer = vec![0i16; 2];
    player.generate_samples(&mut buffer);

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

    let expected = 1002;
    assert_eq!(player.total_samples(), expected);
}
