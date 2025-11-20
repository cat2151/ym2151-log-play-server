//! Integration test for clear_schedule functionality
//!
//! This test verifies that the ClearSchedule command properly clears
//! all scheduled events in interactive mode, allowing seamless phrase transitions.

use ym2151_log_play_server::player::Player;

#[test]
fn test_clear_schedule_removes_all_events() {
    let player = Player::new_interactive();

    // Schedule phrase 1 events
    player.schedule_register_write(100, 0x08, 0x78);
    player.schedule_register_write(200, 0x20, 0xC7);
    player.schedule_register_write(300, 0x28, 0x3E);

    // Verify events were scheduled
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 3); // 3 addr-data pair events
    }

    // Clear the schedule to cancel phrase 1
    player.clear_schedule();

    // Verify all events were removed
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 0);
    }

    // Now schedule phrase 2 events
    player.schedule_register_write(100, 0x30, 0xAA);
    player.schedule_register_write(200, 0x38, 0xBB);

    // Verify only phrase 2 events are in the queue
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 2); // 2 addr-data pair events

        // Verify first event is from phrase 2
        assert_eq!(q[0].addr, 0x30); // First address from phrase 2
    }
}

#[test]
fn test_clear_schedule_after_partial_playback() {
    let mut player = Player::new_interactive();

    // Schedule events at different times
    player.schedule_register_write(100, 0x08, 0x78);
    player.schedule_register_write(200, 0x20, 0xC7);
    player.schedule_register_write(300, 0x28, 0x3E);

    // Simulate some playback by generating samples
    let mut buffer = vec![0i16; 400]; // Generate 200 samples worth of audio
    player.generate_samples(&mut buffer);

    // At this point, some events may have been processed
    // But future events should still be in the queue

    // Clear all remaining scheduled events
    player.clear_schedule();

    // Verify queue is empty
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 0);
    }

    // Schedule new events and verify they can be added after clearing
    player.schedule_register_write(400, 0x40, 0xFF);

    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 1); // 1 addr-data pair event
    }
}

#[test]
fn test_clear_schedule_on_empty_queue() {
    let player = Player::new_interactive();

    // Clear schedule when queue is already empty (should not panic)
    player.clear_schedule();

    // Verify queue is still empty
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 0);
    }

    // Can still schedule events after clearing empty queue
    player.schedule_register_write(100, 0x08, 0x78);

    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 1); // 1 addr-data pair event
    }
}

#[test]
fn test_clear_schedule_multiple_times() {
    let player = Player::new_interactive();

    // Schedule some events
    player.schedule_register_write(100, 0x08, 0x78);
    player.schedule_register_write(200, 0x20, 0xC7);

    // Clear first time
    player.clear_schedule();

    // Schedule new events
    player.schedule_register_write(300, 0x28, 0x3E);

    // Clear second time
    player.clear_schedule();

    // Verify queue is empty
    {
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 0);
    }
}
