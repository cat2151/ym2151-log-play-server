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

    #[test]
    fn test_client_send_json_small() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Small JSON (< 4KB) should be sent directly
        let json_data = r#"{"event_count": 2, "events": [{"time": 0, "addr": "0x08", "data": "0x00"}, {"time": 2, "addr": "0x20", "data": "0xC7"}]}"#;
        assert!(json_data.len() < 4096);

        let server_handle = thread::spawn(move || {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the PLAY command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a PLAY command with JSON string data (sent directly)
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

        thread::sleep(Duration::from_millis(200));

        // Use the new send_json function which should auto-select direct mode
        let result = ym2151_log_play_server::client::send_json(json_data);
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_send_json_large() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Create large JSON (> 4KB) that should be sent via file
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
        assert!(large_events.len() > 4096);

        let server_handle = thread::spawn(move || {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the PLAY command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a PLAY command with a file path (not JSON string)
            match cmd {
                Command::Play(ref data) => {
                    // Should be a file path, not JSON string
                    assert!(!Command::is_json_string(data));
                    assert!(data.contains("ym2151_temp.json"));
                }
                _ => panic!("Expected PLAY command"),
            }

            // Send OK response
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            writer.write_str(&response.serialize()).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        // Use the new send_json function which should auto-select file mode
        let result = ym2151_log_play_server::client::send_json(&large_events);
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }

    #[test]
    fn test_client_send_json_boundary() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();

        cleanup_pipe();

        // Create JSON exactly at 4096 bytes boundary
        let base_json =
            r#"{"event_count": 1, "events": [{"time": 0, "addr": "0x08", "data": "0x00"}]}"#;
        let padding_size = 4096 - base_json.len();
        let boundary_json = format!(
            r#"{{"event_count": 1, "events": [{{"time": 0, "addr": "0x08", "data": "{}"}}]}}"#,
            "0".repeat(padding_size)
        );
        assert_eq!(boundary_json.len(), 4096);

        let server_handle = thread::spawn(move || {
            let pipe = NamedPipe::create().unwrap();
            let mut reader = pipe.open_read().unwrap();

            // Read the PLAY command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // At exactly 4096 bytes, should still be sent directly
            match cmd {
                Command::Play(ref data) => {
                    assert!(Command::is_json_string(data));
                }
                _ => panic!("Expected PLAY command"),
            }

            // Send OK response
            let mut writer = pipe.open_write().unwrap();
            let response = Response::Ok;
            writer.write_str(&response.serialize()).unwrap();
        });

        thread::sleep(Duration::from_millis(200));

        let result = ym2151_log_play_server::client::send_json(&boundary_json);
        assert!(result.is_ok());

        server_handle.join().unwrap();
    }
}
