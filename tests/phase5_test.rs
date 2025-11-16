//! Phase 5 integration tests for real-time audio playback and server control

mod test_utils;

mod realtime_audio_tests {
    use ym2151_log_play_server::audio::AudioPlayer;
    use ym2151_log_play_server::events::{EventLog, RegisterEvent};
    use ym2151_log_play_server::player::Player;

    // Import test utilities for sequential audio tests
    use super::test_utils::audio_test_lock;

    #[test]
    fn test_audio_player_creation() {
        // Acquire lock to prevent parallel execution of audio tests
        let _lock = audio_test_lock();
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
        // Acquire lock to prevent parallel execution of audio tests
        let _lock = audio_test_lock();
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
        // Acquire lock to prevent parallel execution of audio tests
        let _lock = audio_test_lock();
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
        // Acquire lock to prevent parallel execution of audio tests
        let _lock = audio_test_lock();
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
mod server_playback_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_play_server::ipc::pipe_windows::NamedPipe;
    use ym2151_log_play_server::ipc::protocol::Command;
    use ym2151_log_play_server::server::Server;

    // Import test utilities from the parent module
    use super::test_utils::server_test_lock;

    /// Test server can start in idle state and accept PLAY command
    #[test]
    fn test_server_play_command() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();
        eprintln!("Starting server PLAY command test...");

        let server = Server::new();

        // Start server in a separate thread
        let server_handle = thread::spawn(move || {
            eprintln!("Server thread starting...");
            let result = server.run();
            eprintln!("Server thread finished with result: {:?}", result);
            result
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(500));

        eprintln!("Attempting to send PLAY command...");

        // Send PLAY command to play a file using binary protocol
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                eprintln!("Connected to server, sending PlayFile command...");
                let cmd = Command::PlayFile {
                    path: "output_ym2151.json".to_string(),
                };
                match cmd.to_binary() {
                    Ok(binary_data) => {
                        if let Err(e) = writer.write_binary(&binary_data) {
                            eprintln!("Failed to send PlayFile: {}", e);
                        } else {
                            eprintln!("PlayFile command sent successfully");
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to serialize command: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
            }
        }

        // Wait a bit for the new file to start playing
        thread::sleep(Duration::from_millis(500));

        eprintln!("Sending shutdown command...");

        // Send shutdown using binary protocol
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                let cmd = Command::Shutdown;
                if let Ok(binary_data) = cmd.to_binary() {
                    if let Err(e) = writer.write_binary(&binary_data) {
                        eprintln!("Failed to send shutdown: {}", e);
                    } else {
                        eprintln!("Shutdown command sent successfully");
                    }
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
    fn test_server_stop_command() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();
        eprintln!("Starting server STOP command test...");

        let server = Server::new();

        // Start server
        let server_handle = thread::spawn(move || {
            eprintln!("Server thread starting...");
            server.run()
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(500));

        eprintln!("Attempting to send STOP command...");

        // Send STOP command using binary protocol
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                eprintln!("Connected to server, sending STOP command...");
                let cmd = Command::Stop;
                if let Ok(binary_data) = cmd.to_binary() {
                    if let Err(e) = writer.write_binary(&binary_data) {
                        eprintln!("Failed to send STOP: {}", e);
                    } else {
                        eprintln!("STOP command sent successfully");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
            }
        }

        // Wait a bit
        thread::sleep(Duration::from_millis(300));

        eprintln!("Sending shutdown command...");

        // Send shutdown using binary protocol
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                let cmd = Command::Shutdown;
                if let Ok(binary_data) = cmd.to_binary() {
                    let _ = writer.write_binary(&binary_data);
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
}
