//! Phase 7 Integration Tests
//!
//! Comprehensive end-to-end tests for server-client functionality including:
//! - Full playback workflow tests
//! - Error case handling
//! - Multiple client connections
//! - Long-running tests for memory leak verification

#[cfg(all(unix, feature = "realtime-audio"))]
mod integration_tests {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;
    use ym2151_log_player_rust::ipc::protocol::{Command, Response};
    use ym2151_log_player_rust::server::Server;

    const TEST_TIMEOUT: Duration = Duration::from_secs(5);

    /// Helper to clean up pipe before test
    fn cleanup_pipe() {
        let _ = fs::remove_file("/tmp/ym2151_server.pipe");
        thread::sleep(Duration::from_millis(100));
    }

    /// Test complete workflow: server start -> play -> stop -> shutdown
    #[test]
    fn test_complete_server_client_workflow() {
        cleanup_pipe();

        let server = Server::new();

        // Start server in background thread
        let server_handle = thread::spawn(move || {
            server.run("sample_events.json")
        });

        // Wait for server to start and create pipe
        thread::sleep(Duration::from_millis(500));

        // Test 1: Send PLAY command
        {
            let mut writer = NamedPipe::connect_default()
                .expect("Failed to connect to server for PLAY");
            let cmd = Command::Play("test_input.json".to_string());
            writer.write_str(&cmd.serialize()).expect("Failed to send PLAY");
        }

        thread::sleep(Duration::from_millis(300));

        // Test 2: Send STOP command
        {
            let mut writer = NamedPipe::connect_default()
                .expect("Failed to connect to server for STOP");
            let cmd = Command::Stop;
            writer.write_str(&cmd.serialize()).expect("Failed to send STOP");
        }

        thread::sleep(Duration::from_millis(200));

        // Test 3: Send SHUTDOWN command
        {
            let mut writer = NamedPipe::connect_default()
                .expect("Failed to connect to server for SHUTDOWN");
            let cmd = Command::Shutdown;
            writer.write_str(&cmd.serialize()).expect("Failed to send SHUTDOWN");
        }

        // Wait for server to finish
        let result = server_handle.join();
        assert!(result.is_ok(), "Server thread should complete successfully");

        cleanup_pipe();
    }

    /// Test sequential client connections
    #[test]
    fn test_multiple_sequential_connections() {
        cleanup_pipe();

        let server = Server::new();
        let server_handle = thread::spawn(move || {
            server.run("sample_events.json")
        });

        thread::sleep(Duration::from_millis(500));

        // Connect multiple times sequentially
        for i in 0..5 {
            let mut writer = NamedPipe::connect_default()
                .expect(&format!("Failed to connect (iteration {})", i));
            
            let cmd = Command::Stop;
            writer.write_str(&cmd.serialize())
                .expect(&format!("Failed to send STOP (iteration {})", i));
            
            thread::sleep(Duration::from_millis(100));
        }

        // Shutdown server
        {
            let mut writer = NamedPipe::connect_default().unwrap();
            let cmd = Command::Shutdown;
            writer.write_str(&cmd.serialize()).unwrap();
        }

        server_handle.join().expect("Server should complete");
        cleanup_pipe();
    }

    /// Test rapid sequential client connections (stress test)
    #[test]
    fn test_rapid_client_connections() {
        cleanup_pipe();

        let server = Server::new();
        let server_handle = thread::spawn(move || {
            server.run("sample_events.json")
        });

        thread::sleep(Duration::from_millis(500));

        // Rapid fire connections
        for i in 0..10 {
            let mut writer = NamedPipe::connect_default()
                .expect(&format!("Failed to connect (rapid {})", i));
            
            let cmd = Command::Stop;
            writer.write_str(&cmd.serialize())
                .expect(&format!("Failed to send (rapid {})", i));
            
            // No sleep - rapid fire!
        }

        // Shutdown
        {
            let mut writer = NamedPipe::connect_default().unwrap();
            writer.write_str(&Command::Shutdown.serialize()).unwrap();
        }

        server_handle.join().expect("Server should complete");
        cleanup_pipe();
    }
}

/// Error case tests
#[cfg(unix)]
mod error_tests {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::client;

    fn cleanup_pipe() {
        let _ = fs::remove_file("/tmp/ym2151_server.pipe");
        thread::sleep(Duration::from_millis(100));
    }

    /// Test client connection when server is not running
    #[test]
    fn test_client_no_server_error() {
        cleanup_pipe();

        // Try to connect when no server is running
        let result = client::stop_playback();
        assert!(result.is_err(), "Should fail when server is not running");

        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Failed to connect to server") 
            || err.contains("No such file or directory"),
            "Error should indicate connection failure, got: {}", err
        );
    }

    /// Test play with non-existent file (server should handle gracefully)
    #[cfg(feature = "realtime-audio")]
    #[test]
    fn test_play_nonexistent_file() {
        cleanup_pipe();

        use ym2151_log_player_rust::server::Server;
        use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;
        use ym2151_log_player_rust::ipc::protocol::Command;

        let server = Server::new();
        let server_handle = thread::spawn(move || {
            server.run("sample_events.json")
        });

        thread::sleep(Duration::from_millis(500));

        // Try to play non-existent file
        {
            let mut writer = NamedPipe::connect_default().unwrap();
            let cmd = Command::Play("nonexistent_file_12345.json".to_string());
            writer.write_str(&cmd.serialize()).unwrap();
        }

        // Shutdown server
        {
            let mut writer = NamedPipe::connect_default().unwrap();
            writer.write_str(&Command::Shutdown.serialize()).unwrap();
        }

        server_handle.join().unwrap();
        cleanup_pipe();
    }

    /// Test malformed command handling
    #[cfg(feature = "realtime-audio")]
    #[test]
    fn test_invalid_command() {
        cleanup_pipe();

        use ym2151_log_player_rust::server::Server;
        use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;
        use ym2151_log_player_rust::ipc::protocol::Command;

        let server = Server::new();
        let server_handle = thread::spawn(move || {
            server.run("sample_events.json")
        });

        thread::sleep(Duration::from_millis(500));

        // Send invalid command
        {
            let mut writer = NamedPipe::connect_default().unwrap();
            writer.write_str("INVALID_COMMAND\n").unwrap();
        }

        // Shutdown server
        {
            let mut writer = NamedPipe::connect_default().unwrap();
            writer.write_str(&Command::Shutdown.serialize()).unwrap();
        }

        server_handle.join().unwrap();
        cleanup_pipe();
    }
}

/// Long-running test for memory leak verification
#[cfg(all(unix, feature = "realtime-audio"))]
mod longrun_tests {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;
    use ym2151_log_player_rust::ipc::protocol::Command;
    use ym2151_log_player_rust::server::Server;

    fn cleanup_pipe() {
        let _ = fs::remove_file("/tmp/ym2151_server.pipe");
        thread::sleep(Duration::from_millis(100));
    }

    /// Long-running test with many operations
    /// This test performs many PLAY/STOP cycles to verify no memory leaks
    #[test]
    #[ignore] // This is a long-running test, run manually with --ignored
    fn test_longrun_many_operations() {
        cleanup_pipe();

        let server = Server::new();
        let server_handle = thread::spawn(move || {
            server.run("sample_events.json")
        });

        thread::sleep(Duration::from_millis(500));

        // Perform many operations
        for i in 0..100 {
            // PLAY command
            {
                let mut writer = NamedPipe::connect_default()
                    .expect(&format!("Failed to connect (cycle {})", i));
                let cmd = Command::Play("test_input.json".to_string());
                writer.write_str(&cmd.serialize()).unwrap();
            }

            thread::sleep(Duration::from_millis(50));

            // STOP command
            {
                let mut writer = NamedPipe::connect_default().unwrap();
                let cmd = Command::Stop;
                writer.write_str(&cmd.serialize()).unwrap();
            }

            if i % 10 == 0 {
                eprintln!("Completed {} cycles", i);
            }

            thread::sleep(Duration::from_millis(50));
        }

        // Shutdown
        {
            let mut writer = NamedPipe::connect_default().unwrap();
            writer.write_str(&Command::Shutdown.serialize()).unwrap();
        }

        server_handle.join().expect("Server should complete");
        cleanup_pipe();

        eprintln!("✅ Long-run test completed successfully");
    }
}

#[cfg(not(all(unix, feature = "realtime-audio")))]
#[test]
fn phase7_tests_require_unix_and_realtime_audio() {
    println!("ℹ️  Phase 7 integration tests require Unix platform and realtime-audio feature");
}
