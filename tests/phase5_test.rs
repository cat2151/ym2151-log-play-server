#[cfg(feature = "realtime-audio")]
mod realtime_audio_tests {
    use ym2151_log_player_rust::audio::AudioPlayer;
    use ym2151_log_player_rust::events::{EventLog, RegisterEvent};
    use ym2151_log_player_rust::player::Player;

    #[test]
    fn test_audio_player_creation() {
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

        let result = AudioPlayer::new(player);

        match result {
            Ok(mut audio_player) => {
                audio_player.stop();
                println!("✅ Audio player created successfully");
            }
            Err(e) => {
                println!("ℹ️  Audio player creation failed (expected in CI): {}", e);
            }
        }
    }

    #[test]
    fn test_audio_player_with_events() {
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
    println!("ℹ️  Real-time audio tests are disabled (feature not enabled)");
}
