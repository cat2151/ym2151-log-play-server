//! Test for interactive server mode
//!
//! This test verifies the new interactive mode functionality where
//! register writes can be streamed continuously to the server.

use ym2151_log_play_server::player::Player;
use ym2151_log_play_server::resampler::OPM_SAMPLE_RATE;
use ym2151_log_play_server::scheduler;

#[test]
fn test_interactive_player_creation() {
    let player = Player::new_interactive();
    assert!(player.is_interactive());
    assert!(!player.is_complete()); // Interactive mode never completes
}

#[test]
fn test_schedule_register_write() {
    let player = Player::new_interactive();

    // Schedule a register write at 100 samples converted to seconds
    let time_samples = 100;
    player.schedule_register_write(time_samples, 0x08, 0x78);

    // Verify the event was added to the queue
    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();

    // Should have 1 addr-data pair event
    assert_eq!(q.len(), 1);

    // Check the addr-data pair
    assert_eq!(q[0].time, 100);
    assert_eq!(q[0].addr, 0x08);
    assert_eq!(q[0].data, 0x78);
}

#[test]
fn test_multiple_register_writes() {
    let player = Player::new_interactive();

    // Schedule multiple writes
    player.schedule_register_write(100, 0x08, 0x00);
    player.schedule_register_write(200, 0x20, 0xC7);
    player.schedule_register_write(300, 0x28, 0x3E);

    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();

    // Should have 3 addr-data pair events
    assert_eq!(q.len(), 3);

    // Verify timing order is maintained
    for i in 0..q.len() - 1 {
        assert!(q[i].time <= q[i + 1].time, "Events should be in time order");
    }
}

#[test]
fn test_interactive_generate_samples() {
    let mut player = Player::new_interactive();

    // Schedule a write at sample 0
    player.schedule_register_write(0, 0x08, 0x00);

    // Generate some samples
    let mut buffer = vec![0i16; 1024]; // 512 stereo samples
    let has_more = player.generate_samples(&mut buffer);

    // Interactive mode always returns true (continuous streaming)
    assert!(has_more);

    // Event should have been processed
    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();
    assert!(
        q.is_empty(),
        "Event should be processed and removed from queue"
    );
}

#[test]
fn test_scheduler_sec_to_samples() {
    // At OPM_SAMPLE_RATE Hz, 1.0 sec = OPM_SAMPLE_RATE samples
    assert_eq!(scheduler::sec_to_samples(1.0), OPM_SAMPLE_RATE);

    // 0.05 sec (50ms) = 2796.5 ≈ 2797 samples
    assert_eq!(scheduler::sec_to_samples(0.050), 2797);

    // Sample-accurate precision test
    let one_sample_sec = 1.0 / OPM_SAMPLE_RATE as f64;
    assert_eq!(scheduler::sec_to_samples(one_sample_sec), 1);
}

// Note: test_scheduler_schedule_event was removed because schedule_event function
// was deleted as it was unused in production code. Production code uses direct
// sec_to_samples conversion for timing calculations.

#[cfg(windows)]
#[test]
fn test_protocol_interactive_commands() {
    use ym2151_log_play_server::ipc::protocol::Command;

    // Test StartInteractive command serialization
    let cmd = Command::StartInteractive;
    let binary = cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(cmd, parsed);

    // Test PlayJsonInInteractive command serialization
    let json_value = serde_json::json!({
        "events": [
            {"time": 50.0 / OPM_SAMPLE_RATE as f64, "addr": "0x08", "data": "0x78"}
        ]
    });
    let cmd = Command::PlayJsonInInteractive { data: json_value };
    let binary = cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(cmd, parsed);

    // Test StopInteractive command serialization
    let cmd = Command::StopInteractive;
    let binary = cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(cmd, parsed);

    // Test GetServerTime command serialization
    let cmd = Command::GetServerTime;
    let binary = cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(cmd, parsed);
}

#[test]
fn test_non_interactive_mode_unaffected() {
    use ym2151_log_play_server::events::{EventLog, RegisterEvent};

    // Create a normal player with static events
    let log = EventLog {
        events: vec![RegisterEvent {
            time: 0.0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let player = Player::new(log);

    // Should not be in interactive mode
    assert!(!player.is_interactive());

    // Can complete
    assert!(!player.is_complete()); // Not complete until events are processed
}
