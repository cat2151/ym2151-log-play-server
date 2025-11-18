//! Test for interactive server mode
//!
//! This test verifies the new interactive mode functionality where
//! register writes can be streamed continuously to the server.

use ym2151_log_play_server::player::Player;
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

    // Schedule a register write
    player.schedule_register_write(100, 0x08, 0x78);

    // Verify the event was added to the queue
    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();

    // Should have 2 events (address write + data write)
    assert_eq!(q.len(), 2);

    // Check first event (address write)
    assert_eq!(q[0].port, 0);
    assert_eq!(q[0].value, 0x08);

    // Check second event (data write)
    assert_eq!(q[1].port, 1);
    assert_eq!(q[1].value, 0x78);
    assert_eq!(q[1].time, q[0].time + 2); // DELAY_SAMPLES = 2
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

    // Should have 6 events (3 writes × 2 events each)
    assert_eq!(q.len(), 6);

    // Verify timing order is maintained
    for i in 0..q.len() - 1 {
        assert!(q[i].time <= q[i + 1].time, "Events should be in time order");
    }
}

#[test]
fn test_interactive_generate_samples() {
    let mut player = Player::new_interactive();

    // Schedule a write
    player.schedule_register_write(0, 0x08, 0x00);

    // Generate some samples
    let mut buffer = vec![0i16; 1024]; // 512 stereo samples
    let has_more = player.generate_samples(&mut buffer);

    // Interactive mode always returns true (continuous streaming)
    assert!(has_more);

    // Events should have been processed
    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();
    assert!(
        q.is_empty() || q.len() < 2,
        "Some events should be processed"
    );
}

#[test]
fn test_scheduler_ms_to_samples() {
    // At 55930 Hz, 1000ms = 55930 samples
    assert_eq!(scheduler::ms_to_samples(1000), 55930);

    // 50ms latency buffer = 2796.5 ≈ 2796 samples
    assert_eq!(scheduler::ms_to_samples(50), 2796);
}

#[test]
fn test_scheduler_schedule_event() {
    // Current time at 1000 samples, offset 0ms
    // Should add 50ms latency buffer
    let scheduled = scheduler::schedule_event(1000, 0);
    assert_eq!(scheduled, 1000 + 2796);

    // With 100ms offset
    let scheduled = scheduler::schedule_event(1000, 100);
    let expected = 1000 + 2796 + 5593; // 50ms latency + 100ms offset
    assert_eq!(scheduled, expected);
}

#[cfg(windows)]
#[test]
fn test_protocol_interactive_commands() {
    use ym2151_log_play_server::ipc::protocol::Command;

    // Test StartInteractive command serialization
    let cmd = Command::StartInteractive;
    let binary = cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(cmd, parsed);

    // Test WriteRegister command serialization
    let cmd = Command::WriteRegister {
        time_offset_ms: 50,
        addr: 0x08,
        data: 0x78,
    };
    let binary = cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(cmd, parsed);

    // Test StopInteractive command serialization
    let cmd = Command::StopInteractive;
    let binary = cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(cmd, parsed);
}

#[test]
fn test_non_interactive_mode_unaffected() {
    use ym2151_log_play_server::events::{EventLog, RegisterEvent};

    // Create a normal player with static events
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 0,
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
