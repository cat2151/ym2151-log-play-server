//! Integration tests for the client module direct JSON functionality
//!
//! These tests verify that the client can send JSON string data directly via named pipe.

mod test_utils;

mod client_json_integration_tests {
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
    fn test_client_send_json_direct() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        let json_data = r#"{"event_count": 2, "events": [{"time": 0, "addr": "0x08", "data": "0x00"}, {"time": 2, "addr": "0x20", "data": "0xC7"}]}"#;

        // Start a mock server in a separate thread
        let server_handle = thread::spawn(move || {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the PLAY command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a PLAY command with JSON string data
            match cmd {
                Command::Play(ref data) => {
                    assert!(Command::is_json_string(data));
                    assert!(data.contains("event_count"));
                    assert!(data.contains("events"));
                }
                _ => panic!("Expected PLAY command"),
            }

            // Send OK response
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            writer.write_str(&response.serialize()).unwrap();
        });

        // Give server time to start and create the pipe
        thread::sleep(Duration::from_millis(200));

        // Send PLAY command with JSON data directly via pipe
        let result = ym2151_log_play_server::client::send_json_direct(json_data);
        assert!(result.is_ok());

        // Wait for server to finish
        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_send_json_direct_empty() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        let json_data = r#"{"event_count": 0, "events": []}"#;

        let server_handle = thread::spawn(move || {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the PLAY command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a PLAY command with empty JSON
            match cmd {
                Command::Play(ref data) => {
                    assert!(Command::is_json_string(data));
                    assert!(data.contains("\"event_count\": 0"));
                }
                _ => panic!("Expected PLAY command"),
            }

            // Send OK response
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            writer.write_str(&response.serialize()).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::send_json_direct(json_data);
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }
}
