//! Server Integration Tests
//!
//! These tests validate the server functionality including playback control
//! and command processing via named pipes.

mod test_util_server_mutex;

/// Server integration tests for playback control
#[cfg(windows)]
mod server_playback_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_play_server::ipc::pipe_windows::NamedPipe;
    use ym2151_log_play_server::ipc::protocol::Command;
    use ym2151_log_play_server::server::Server;

    // Import test utilities from the parent module
    use super::test_util_server_mutex::server_test_lock;

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

        // Send PLAY command to play JSON data using binary protocol
        match NamedPipe::connect_default() {
            Ok(mut writer) => {
                eprintln!("Connected to server, sending PlayJson command...");

                // Read and send JSON data
                let json_content = match std::fs::read_to_string("output_ym2151.json") {
                    Ok(content) => content,
                    Err(e) => {
                        eprintln!("Failed to read JSON file: {}", e);
                        return;
                    }
                };

                let json_data: serde_json::Value = match serde_json::from_str(&json_content) {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        return;
                    }
                };

                let cmd = Command::PlayJson { data: json_data };
                match cmd.to_binary() {
                    Ok(binary_data) => {
                        if let Err(e) = writer.write_binary(&binary_data) {
                            eprintln!("Failed to send PlayJson: {}", e);
                        } else {
                            eprintln!("PlayJson command sent successfully");
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

        // Wait for server thread and verify it shuts down cleanly
        match server_handle.join() {
            Ok(_) => eprintln!("Server thread finished successfully"),
            Err(e) => {
                eprintln!("Server thread panicked: {:?}", e);
                panic!("Server thread should not panic");
            }
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

        // Wait for server thread and verify it shuts down cleanly
        match server_handle.join() {
            Ok(_) => eprintln!("Server thread finished successfully"),
            Err(e) => {
                eprintln!("Server thread panicked: {:?}", e);
                panic!("Server thread should not panic");
            }
        }

        eprintln!("Test complete");
    }
}
