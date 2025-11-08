//! Integration tests for the client module
//!
//! These tests verify that the client can send commands to a mock server.

#[cfg(unix)]
mod client_integration_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::ipc::pipe_unix::{NamedPipe, DEFAULT_PIPE_PATH};
    use ym2151_log_player_rust::ipc::protocol::Command;

    /// Helper to clean up pipe before test
    fn cleanup_pipe() {
        let _ = std::fs::remove_file(DEFAULT_PIPE_PATH);
        thread::sleep(Duration::from_millis(50));
    }

    #[test]
    fn test_client_play_file() {
        cleanup_pipe();

        // Start a mock server in a separate thread
        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the PLAY command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a PLAY command with the correct path
            match cmd {
                Command::Play(ref path) => {
                    assert_eq!(path, "test_file.json");
                }
                _ => panic!("Expected PLAY command"),
            }
        });

        // Give server time to start and create the pipe
        thread::sleep(Duration::from_millis(200));

        // Send PLAY command from client
        let result = ym2151_log_player_rust::client::play_file("test_file.json");
        assert!(result.is_ok());

        // Wait for server to finish
        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_stop_playback() {
        cleanup_pipe();

        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the STOP command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a STOP command
            assert!(matches!(cmd, Command::Stop));
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_player_rust::client::stop_playback();
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_shutdown_server() {
        cleanup_pipe();

        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the SHUTDOWN command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a SHUTDOWN command
            assert!(matches!(cmd, Command::Shutdown));
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_player_rust::client::shutdown_server();
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_no_server() {
        cleanup_pipe();

        // Try to send a command when no server is running
        // This should fail with a connection error
        let result = ym2151_log_player_rust::client::stop_playback();
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Failed to connect to server")
                || err_msg.contains("No such file or directory")
        );
    }
}

#[cfg(windows)]
mod windows_client_tests {
    use ym2151_log_player_rust::client;

    #[test]
    fn test_windows_client_not_implemented() {
        // All client functions should return an error on Windows
        let result = client::play_file("test.json");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not yet implemented"));

        let result = client::stop_playback();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not yet implemented"));

        let result = client::shutdown_server();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("not yet implemented"));
    }
}
