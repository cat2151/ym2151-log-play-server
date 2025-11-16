//! Integration tests for the client module direct JSON functionality
//!
//! These tests verify that the client can send JSON string data directly via named pipe
//! using the new binary protocol.

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

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a PlayJson command
            match cmd {
                Command::PlayJson { data } => {
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

        // Send JSON command
        let result = ym2151_log_play_server::client::send_json(json_data);
        assert!(result.is_ok());

        // Wait for server to finish
        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_send_json_empty() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        let json_data = r#"{"event_count": 0, "events": []}"#;

        let server_handle = thread::spawn(move || {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a PlayJson command with empty events
            match cmd {
                Command::PlayJson { data } => {
                    assert_eq!(data.get("event_count").and_then(|v| v.as_u64()), Some(0));
                }
                _ => panic!("Expected PlayJson command"),
            }

            // Send OK response
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::send_json(json_data);
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_send_json_large() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Create large JSON to test binary protocol can handle it
        let mut large_events = String::from(r#"{"event_count": 500, "events": ["#);
        for i in 0..500 {
            if i > 0 {
                large_events.push_str(", ");
            }
            large_events.push_str(&format!(
                r#"{{"time": {}, "addr": "0x08", "data": "0x00"}}"#,
                i
            ));
        }
        large_events.push_str("]}");

        let server_handle = thread::spawn(move || {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the binary command
            let binary_data = reader.read_binary().unwrap();
            let cmd = Command::from_binary(&binary_data).unwrap();

            // Verify it's a PlayJson command with large data
            match cmd {
                Command::PlayJson { data } => {
                    assert_eq!(data.get("event_count").and_then(|v| v.as_u64()), Some(500));
                }
                _ => panic!("Expected PlayJson command"),
            }

            // Send OK response
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            let response_binary = response.to_binary().unwrap();
            writer.write_binary(&response_binary).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::send_json(&large_events);
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }
}
