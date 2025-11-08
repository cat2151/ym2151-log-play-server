//! Integration tests for IPC named pipe functionality
//!
//! These tests verify that the named pipe implementation works correctly
//! for server-client communication.

#[cfg(unix)]
mod pipe_integration_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::ipc::pipe::NamedPipe;
    use ym2151_log_player_rust::ipc::protocol::{Command, Response};

    #[test]
    fn test_command_response_roundtrip() {
        let test_path = "/tmp/test_ym2151_cmd_resp.pipe";

        // Create the pipe
        let pipe = NamedPipe::create_at(test_path).unwrap();

        // Spawn server thread
        let server_handle = thread::spawn(move || {
            let mut reader = pipe.open_read().unwrap();

            // Read command
            let line = reader.read_line().unwrap();
            let cmd = Command::parse(&line).unwrap();

            // Verify it's a PLAY command
            match cmd {
                Command::Play(ref path) => {
                    assert_eq!(path, "/test/music.json");

                    // Send OK response (in real implementation, this would be on a separate pipe)
                    // For this test, we just verify the command was received correctly
                }
                _ => panic!("Expected PLAY command"),
            }
        });

        // Spawn client thread
        let client_path = test_path.to_string();
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100)); // Ensure server is ready
            let mut writer = NamedPipe::connect(&client_path).unwrap();

            // Send PLAY command
            let cmd = Command::Play("/test/music.json".to_string());
            writer.write_str(&cmd.serialize()).unwrap();
        });

        // Wait for both threads to complete
        server_handle.join().unwrap();
        client_handle.join().unwrap();
    }

    #[test]
    fn test_multiple_commands() {
        let test_path = "/tmp/test_ym2151_multi_cmd.pipe";

        let pipe = NamedPipe::create_at(test_path).unwrap();

        let server_handle = thread::spawn(move || {
            let mut reader = pipe.open_read().unwrap();

            // Read and verify first command (PLAY)
            let line1 = reader.read_line().unwrap();
            let cmd1 = Command::parse(&line1).unwrap();
            assert!(matches!(cmd1, Command::Play(_)));

            // Read and verify second command (STOP)
            let line2 = reader.read_line().unwrap();
            let cmd2 = Command::parse(&line2).unwrap();
            assert!(matches!(cmd2, Command::Stop));

            // Read and verify third command (SHUTDOWN)
            let line3 = reader.read_line().unwrap();
            let cmd3 = Command::parse(&line3).unwrap();
            assert!(matches!(cmd3, Command::Shutdown));
        });

        let client_path = test_path.to_string();
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let mut writer = NamedPipe::connect(&client_path).unwrap();

            // Send multiple commands
            writer
                .write_str(&Command::Play("/music1.json".to_string()).serialize())
                .unwrap();
            writer.write_str(&Command::Stop.serialize()).unwrap();
            writer.write_str(&Command::Shutdown.serialize()).unwrap();
        });

        server_handle.join().unwrap();
        client_handle.join().unwrap();
    }

    #[test]
    fn test_protocol_serialization_over_pipe() {
        let test_path = "/tmp/test_ym2151_protocol.pipe";

        let pipe = NamedPipe::create_at(test_path).unwrap();

        let commands = vec![
            Command::Play("/path/to/file1.json".to_string()),
            Command::Stop,
            Command::Shutdown,
        ];

        let expected = commands.clone();

        let server_handle = thread::spawn(move || {
            let mut reader = pipe.open_read().unwrap();

            for expected_cmd in expected {
                let line = reader.read_line().unwrap();
                let received_cmd = Command::parse(&line).unwrap();
                assert_eq!(received_cmd, expected_cmd);
            }
        });

        let client_path = test_path.to_string();
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let mut writer = NamedPipe::connect(&client_path).unwrap();

            for cmd in commands {
                writer.write_str(&cmd.serialize()).unwrap();
            }
        });

        server_handle.join().unwrap();
        client_handle.join().unwrap();
    }

    #[test]
    fn test_error_handling_invalid_command() {
        let test_path = "/tmp/test_ym2151_invalid.pipe";

        let pipe = NamedPipe::create_at(test_path).unwrap();

        let server_handle = thread::spawn(move || {
            let mut reader = pipe.open_read().unwrap();

            // Read invalid command
            let line = reader.read_line().unwrap();
            let result = Command::parse(&line);

            // Should fail to parse
            assert!(result.is_err());

            // The error should indicate an unknown command
            let err = result.unwrap_err();
            assert!(err.contains("Unknown command") || err.contains("Empty command"));
        });

        let client_path = test_path.to_string();
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            let mut writer = NamedPipe::connect(&client_path).unwrap();

            // Send invalid command
            writer.write_str("INVALID_COMMAND\n").unwrap();
        });

        server_handle.join().unwrap();
        client_handle.join().unwrap();
    }

    #[test]
    fn test_response_parsing() {
        // Test that responses can be correctly parsed
        let ok_resp = Response::parse("OK").unwrap();
        assert_eq!(ok_resp, Response::Ok);

        let err_resp = Response::parse("ERROR File not found").unwrap();
        assert!(matches!(err_resp, Response::Error(_)));

        // Test serialization
        assert_eq!(Response::Ok.serialize(), "OK\n");
        assert_eq!(
            Response::Error("Test error".to_string()).serialize(),
            "ERROR Test error\n"
        );
    }
}

#[cfg(windows)]
mod windows_stub_tests {
    use ym2151_log_player_rust::ipc::pipe::NamedPipe;

    #[test]
    fn test_windows_not_implemented() {
        // Windows implementation should return an error
        let result = NamedPipe::create();
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Unsupported);
    }
}
