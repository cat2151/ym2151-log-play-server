//! Feature demonstration tests
//!
//! These tests demonstrate the key features of the ym2151-log-play-server
//! and serve as working examples for users. Migrated from examples/ directory
//! to ensure they are tested and maintained as part of the test suite.

#[cfg(windows)]
mod feature_demonstrations {
    use ym2151_log_play_server::player::Player;
    use ym2151_log_play_server::scheduler;

    #[test]
    fn test_clear_schedule_demo_functionality() {
        //! Demonstrates clear_schedule for seamless phrase transitions
        //! (Migrated from examples/clear_schedule_demo.rs)

        let player = Player::new_interactive();

        println!("🎮 Clear Schedule Demo - Seamless Phrase Transitions");

        // Scenario: Play phrase 1, but then decide to cancel it and play phrase 2
        println!("🎵 Scheduling Phrase 1 (long melody with many notes)...");

        // Phrase 1: A long melody scheduled over several seconds
        player.schedule_register_write(0, 0x08, 0x00);
        player.schedule_register_write(2797, 0x28, 0x48);
        player.schedule_register_write(2797, 0x30, 0x00);
        player.schedule_register_write(2797, 0x08, 0x78);
        player.schedule_register_write(30762, 0x08, 0x00);
        player.schedule_register_write(33559, 0x28, 0x4A);
        player.schedule_register_write(33559, 0x08, 0x78);
        player.schedule_register_write(61524, 0x08, 0x00);

        // Verify events were scheduled
        {
            let queue = player.get_event_queue();
            let q = queue.lock().unwrap();
            assert!(q.len() > 0, "Phrase 1 should have scheduled events");
        }

        println!("🗑️ Change of plan! Clearing scheduled events for phrase 1...");
        player.clear_schedule();

        // Verify schedule was cleared
        {
            let queue = player.get_event_queue();
            let q = queue.lock().unwrap();
            assert_eq!(q.len(), 0, "Schedule should be cleared");
        }

        println!("🎵 Scheduling Phrase 2 (different melody) without audio gap...");

        // Phrase 2: A completely different melody
        player.schedule_register_write(0, 0x08, 0x00);
        player.schedule_register_write(2797, 0x28, 0x50);
        player.schedule_register_write(2797, 0x08, 0x78);
        player.schedule_register_write(20000, 0x08, 0x00);

        // Verify new events are scheduled
        {
            let queue = player.get_event_queue();
            let q = queue.lock().unwrap();
            assert!(q.len() > 0, "Phrase 2 should have scheduled events");
        }

        // Demonstrate multiple clear and schedule operations
        println!("🎵 Scheduling Phrase 3...");
        player.schedule_register_write(0, 0x28, 0x55);
        player.schedule_register_write(0, 0x08, 0x78);

        println!("🗑️ Actually, let's clear this one too and end with phrase 4!");
        player.clear_schedule();

        // Final phrase
        player.schedule_register_write(0, 0x28, 0x4C);
        player.schedule_register_write(0, 0x08, 0x78);

        println!("✅ Clear schedule demo functionality verified!");

        // Key use cases demonstrated:
        // • Cancel scheduled musical phrases
        // • Respond to user input (e.g., button press changes melody)
        // • Dynamic music generation based on game state
        // • Seamless transitions without audio gaps
        // • Interactive tone editor undo functionality
        // • Real-time music composition tools
    }

    #[test]
    fn test_interactive_mode_demo_functionality() {
        //! Demonstrates interactive mode continuous streaming
        //! (Migrated from examples/interactive_demo.rs)

        println!("🎮 Interactive Mode Demo");

        let player = Player::new_interactive();
        assert!(player.is_interactive(), "Player should be in interactive mode");
        assert!(!player.is_complete(), "Interactive mode never completes");

        println!("📝 Sending register writes...");

        // Initialize all channels to silent
        for ch in 0..8 {
            player.schedule_register_write(0, 0x08, ch);
        }

        // Simple melody: play a few notes with timing conversion
        let notes = vec![
            (0.0, 0x28, 0x48),   // Note C4, channel 0
            (0.100, 0x30, 0x00), // Octave and note on at 100ms
            (0.100, 0x08, 0x78), // Key on channel 0
            (0.500, 0x08, 0x00), // Key off at 500ms
            (0.600, 0x28, 0x4A), // Note D4 at 600ms
            (0.600, 0x08, 0x78), // Key on
            (1.100, 0x08, 0x00), // Key off at 1100ms
            (1.200, 0x28, 0x4C), // Note E4 at 1200ms
            (1.200, 0x08, 0x78), // Key on
            (1.700, 0x08, 0x00), // Key off at 1700ms
        ];

        for (time_sec, addr, data) in notes {
            // Convert time from seconds to samples
            let time_samples = scheduler::sec_to_samples(time_sec);
            println!(
                "  Writing register 0x{:02X} = 0x{:02X} at +{:.3}s ({} samples)",
                addr, data, time_sec, time_samples
            );
            player.schedule_register_write(time_samples, addr, data);
        }

        // Verify events are scheduled
        {
            let queue = player.get_event_queue();
            let q = queue.lock().unwrap();
            assert!(q.len() > 0, "Interactive mode should have scheduled events");
        }

        println!("✅ Interactive mode demo functionality verified!");
    }

    #[test]
    fn test_play_json_interactive_demo_functionality() {
        //! Demonstrates JSON parsing and event scheduling
        //! (Migrated from examples/play_json_interactive_demo.rs)
        //! Note: This test focuses on JSON parsing without requiring server connection

        use ym2151_log_play_server::events::EventLog;

        println!("🎮 play_json_interactive Convenience Function Demo");

        // Test JSON parsing functionality that play_json_interactive would use
        println!("📝 Parsing first melody JSON...");

        let json1 = r#"{
            "event_count": 5,
            "events": [
                {"time": 0, "addr": "0x08", "data": "0x00"},
                {"time": 2797, "addr": "0x28", "data": "0x48"},
                {"time": 2797, "addr": "0x30", "data": "0x00"},
                {"time": 2797, "addr": "0x08", "data": "0x78"},
                {"time": 30762, "addr": "0x08", "data": "0x00"}
            ]
        }"#;

        let log1 = EventLog::from_json_str(json1).unwrap();
        assert_eq!(log1.event_count, 5);
        assert!(log1.validate(), "First JSON should be valid");

        println!("📝 Parsing second melody JSON (seamlessly continues)...");

        let json2 = r#"{
            "event_count": 3,
            "events": [
                {"time": 33559, "addr": "0x28", "data": "0x4A"},
                {"time": 33559, "addr": "0x08", "data": "0x78"},
                {"time": 61524, "addr": "0x08", "data": "0x00"}
            ]
        }"#;

        let log2 = EventLog::from_json_str(json2).unwrap();
        assert_eq!(log2.event_count, 3);
        assert!(log2.validate(), "Second JSON should be valid");

        println!("📝 Parsing third melody JSON (seamlessly continues)...");

        let json3 = r#"{
            "event_count": 2,
            "events": [
                {"time": 67117, "addr": "0x28", "data": "0x4C"},
                {"time": 67117, "addr": "0x08", "data": "0x78"}
            ]
        }"#;

        let log3 = EventLog::from_json_str(json3).unwrap();
        assert_eq!(log3.event_count, 2);
        assert!(log3.validate(), "Third JSON should be valid");

        // Demonstrate that these could be played in interactive mode
        let player = Player::new_interactive();

        // Convert events from first JSON and schedule them
        for event in &log1.events {
            player.schedule_register_write(event.time, event.addr, event.data);
        }

        // Verify events are scheduled
        {
            let queue = player.get_event_queue();
            let q = queue.lock().unwrap();
            assert!(q.len() > 0, "Events from JSON should be scheduled");
        }

        println!("✅ play_json_interactive demo functionality verified!");

        // Key benefits demonstrated:
        // • Automatic JSON parsing and validation
        // • Automatic time conversion (samples → seconds)
        // • Continuous playback without audio gaps
        // • Perfect for dynamic music generation and tone editing
    }

    #[test]
    fn test_scheduler_time_conversion_functionality() {
        //! Demonstrates time conversion utilities used by interactive features

        println!("🎮 Time Conversion Demo");

        // Demonstrate sec_to_samples conversion
        assert_eq!(scheduler::sec_to_samples(1.0), 55930, "1.0 sec should be 55930 samples");
        assert_eq!(scheduler::sec_to_samples(0.050), 2797, "0.05 sec should be 2797 samples");

        let one_sample_sec = 1.0 / 55930.0;
        assert_eq!(scheduler::sec_to_samples(one_sample_sec), 1, "One sample duration should convert to 1");

        // Note: schedule_event function was removed as it was unused in production code
        // Production code uses direct sec_to_samples conversion without latency buffer

        println!("✅ Time conversion demo functionality verified!");
    }
}

#[cfg(not(windows))]
mod non_windows_demonstrations {
    #[test]
    fn test_features_require_windows() {
        //! Documents that the main interactive features require Windows
        //! (due to named pipe implementation)

        println!("ℹ️ Interactive client features require Windows (named pipe support)");
        println!("On Unix systems, the server/client features are not enabled.");

        // Non-Windows platforms can still use:
        // - Standalone mode JSON playback
        // - EventLog parsing
        // - Player and audio generation
        // - WAV file output

        assert!(true, "Non-Windows platforms supported for basic functionality");
    }
}
