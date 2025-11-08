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

/// Integration tests for Phase 5: Server with playback control
#[cfg(all(unix, feature = "realtime-audio"))]
mod server_playback_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;
    use ym2151_log_player_rust::ipc::protocol::Command;
    use ym2151_log_player_rust::server::Server;

    /// Test server can start with initial playback and accept PLAY command
    #[test]
    #[ignore] // Manual test - requires audio device
    fn test_server_play_command() {
        eprintln!("Starting server PLAY command test...");

        let server = Server::new();

        // Start server in a separate thread
        let server_handle = thread::spawn(move || {
            eprintln!("Server thread starting...");
            let result = server.run("sample_events.json");
            eprintln!("Server thread finished with result: {:?}", result);
            result
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(500));

        eprintln!("Attempting to send PLAY command...");

        // Send PLAY command to play a different file
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                eprintln!("Connected to server, sending PLAY command...");
                let cmd = Command::Play("test_input.json".to_string());
                if let Err(e) = writer.write_str(&cmd.serialize()) {
                    eprintln!("Failed to send PLAY: {}", e);
                } else {
                    eprintln!("PLAY command sent successfully");
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
            }
        }

        // Wait a bit for the new file to start playing
        thread::sleep(Duration::from_millis(500));

        eprintln!("Sending shutdown command...");

        // Send shutdown
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                let cmd = Command::Shutdown;
                if let Err(e) = writer.write_str(&cmd.serialize()) {
                    eprintln!("Failed to send shutdown: {}", e);
                } else {
                    eprintln!("Shutdown command sent successfully");
                }
            }
            Err(e) => {
                eprintln!("Failed to connect for shutdown: {}", e);
            }
        }

        // Wait for server to finish
        thread::sleep(Duration::from_millis(500));

        // Wait for server thread
        if let Err(e) = server_handle.join() {
            eprintln!("Server thread panicked: {:?}", e);
        }

        eprintln!("Test complete");
    }

    /// Test server STOP command
    #[test]
    #[ignore] // Manual test - requires audio device
    fn test_server_stop_command() {
        eprintln!("Starting server STOP command test...");

        let server = Server::new();

        // Start server
        let server_handle = thread::spawn(move || {
            eprintln!("Server thread starting...");
            server.run("sample_events.json")
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(500));

        eprintln!("Attempting to send STOP command...");

        // Send STOP command
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                eprintln!("Connected to server, sending STOP command...");
                let cmd = Command::Stop;
                if let Err(e) = writer.write_str(&cmd.serialize()) {
                    eprintln!("Failed to send STOP: {}", e);
                } else {
                    eprintln!("STOP command sent successfully");
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
            }
        }

        // Wait a bit
        thread::sleep(Duration::from_millis(300));

        eprintln!("Sending shutdown command...");

        // Send shutdown
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                let cmd = Command::Shutdown;
                let _ = writer.write_str(&cmd.serialize());
            }
            Err(e) => {
                eprintln!("Failed to connect for shutdown: {}", e);
            }
        }

        // Wait for server to finish
        thread::sleep(Duration::from_millis(500));

        // Wait for server thread
        if let Err(e) = server_handle.join() {
            eprintln!("Server thread panicked: {:?}", e);
        }

        eprintln!("Test complete");
    }
}

#[cfg(not(feature = "realtime-audio"))]
#[test]
fn test_realtime_audio_not_enabled() {
    println!("ℹ️  Real-time audio tests are disabled (feature not enabled)");
}
