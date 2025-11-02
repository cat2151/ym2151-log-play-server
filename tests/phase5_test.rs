// Phase 5 Integration Tests
//
// These tests verify the real-time audio playback functionality.
// Note: These tests are only compiled when the realtime-audio feature is enabled.

#[cfg(feature = "realtime-audio")]
mod realtime_audio_tests {
    use ym2151_log_player_rust::audio::AudioPlayer;
    use ym2151_log_player_rust::events::{EventLog, RegisterEvent};
    use ym2151_log_player_rust::player::Player;

    #[test]
    fn test_audio_player_creation() {
        // Create a minimal event log
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

        // Try to create audio player - may fail in CI without audio device
        let result = AudioPlayer::new(player);

        // This test passes regardless of whether an audio device is available
        // We just verify that the code compiles and doesn't panic
        match result {
            Ok(mut audio_player) => {
                // Successfully created - stop immediately
                audio_player.stop();
                println!("✅ Audio player created successfully");
            }
            Err(e) => {
                // Expected in CI environments without audio
                println!("ℹ️  Audio player creation failed (expected in CI): {}", e);
            }
        }
    }

    #[test]
    fn test_audio_player_with_events() {
        // Create a simple event log with a few events
        let log = EventLog {
            event_count: 5,
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
                RegisterEvent {
                    time: 300,
                    addr: 0x38,
                    data: 0x01,
                    is_data: None,
                },
                RegisterEvent {
                    time: 400,
                    addr: 0x08,
                    data: 0x00,
                    is_data: None,
                },
            ],
        };

        let player = Player::new(log);

        match AudioPlayer::new(player) {
            Ok(mut audio_player) => {
                // Let it play for a very short time
                std::thread::sleep(std::time::Duration::from_millis(50));
                audio_player.stop();
                println!("✅ Audio playback test completed");
            }
            Err(e) => {
                println!("ℹ️  Audio player creation failed (expected in CI): {}", e);
            }
        }
    }

    #[test]
    fn test_audio_player_early_stop() {
        // Create a longer event sequence
        let mut events = Vec::new();
        for i in 0..20 {
            events.push(RegisterEvent {
                time: i * 1000,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            });
        }

        let log = EventLog {
            event_count: events.len(),
            events,
        };

        let player = Player::new(log);

        match AudioPlayer::new(player) {
            Ok(mut audio_player) => {
                // Start playback then immediately stop
                audio_player.stop();
                println!("✅ Early stop test completed");
            }
            Err(e) => {
                println!("ℹ️  Audio player creation failed (expected in CI): {}", e);
            }
        }
    }

    #[test]
    fn test_audio_player_drop() {
        // Test that dropping the player stops playback gracefully
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

        match AudioPlayer::new(player) {
            Ok(audio_player) => {
                // Let drop handle cleanup
                drop(audio_player);
                println!("✅ Drop test completed");
            }
            Err(e) => {
                println!("ℹ️  Audio player creation failed (expected in CI): {}", e);
            }
        }
    }
}

#[cfg(not(feature = "realtime-audio"))]
#[test]
fn test_realtime_audio_not_enabled() {
    // This test just verifies that the test file compiles without the feature
    println!("ℹ️  Real-time audio tests are disabled (feature not enabled)");
}
