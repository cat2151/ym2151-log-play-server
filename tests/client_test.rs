//! Integration tests for the client module
//!
//! These tests verify that the client can send commands to a mock server using the binary protocol.

#![cfg(windows)]

mod test_util_server_mutex;

mod client_integration_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_play_server::ipc::pipe_windows::NamedPipe;
    use ym2151_log_play_server::ipc::protocol::{Command, Response};

    // Import test utilities for sequential server tests
    use super::test_util_server_mutex::server_test_lock;

    /// Helper to clean up pipe before test
    fn cleanup_pipe() {
        // On Windows, pipes are automatically cleaned up when all handles are closed
        thread::sleep(Duration::from_millis(50));
    }

    #[test]
    fn test_client_send_json() {
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

            // Verify it's a PlayJson command with JSON data
            match cmd {
                Command::PlayJson { data } => {
                    // Verify the JSON structure
                    assert!(data.get("event_count").is_some());
                    assert!(data.get("events").is_some());
                }
                _ => panic!("Expected PlayJson command"),
            }

            // Send OK response in binary format
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        // Give server time to start and create the pipe
        thread::sleep(Duration::from_millis(200));

        // Send JSON data from client
        let json_data =
            r#"{"event_count": 1, "events": [{"time": 0, "addr": "0x08", "data": "0x00"}]}"#;
        let result = ym2151_log_play_server::client::send_json(json_data);
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
