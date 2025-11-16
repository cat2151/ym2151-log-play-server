//! Integration tests for the client module
//!
//! These tests verify that the client can send commands to a mock server using the binary protocol.

mod test_utils;

mod client_integration_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_play_server::ipc::pipe_windows::NamedPipe;
    use ym2151_log_play_server::ipc::protocol::{Command, Response};

    // Import test utilities for sequential server tests
    use super::test_utils::server_test_lock;

    /// Helper to clean up pipe before test
    fn cleanup_pipe() {
        // On Windows, pipes are automatically cleaned up when all handles are closed
        thread::sleep(Duration::from_millis(50));
    }

    #[test]
    fn test_client_play_file() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Start a mock server in a separate thread
        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a PlayFile command with the correct path
            match cmd {
                Command::PlayFile { ref path } => {
                    assert_eq!(path, "output_ym2151.json");
                }
                _ => panic!("Expected PlayFile command"),
            }

            // Send OK response in binary format
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        // Give server time to start and create the pipe
        thread::sleep(Duration::from_millis(200));

        // Send PlayFile command from client
        let result = ym2151_log_play_server::client::play_file("output_ym2151.json");
        assert!(result.is_ok());

        // Wait for server to finish
        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_stop_playback() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a Stop command
            assert!(matches!(cmd, Command::Stop));

            // Send OK response in binary format
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::stop_playback();
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_shutdown_server() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        let server_handle = thread::spawn(|| {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a Shutdown command
            assert!(matches!(cmd, Command::Shutdown));

            // Send OK response in binary format
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::shutdown_server();
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_no_server() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Try to send a command when no server is running
        // This should fail with a connection error
        let result = ym2151_log_play_server::client::stop_playback();
        assert!(result.is_err());

        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Failed to connect to server")
                || err_msg.contains("The system cannot find the file specified")
                || err_msg.contains("No such file or directory")
        );
    }
}
