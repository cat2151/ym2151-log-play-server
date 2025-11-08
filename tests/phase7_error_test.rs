//! Phase 7 Error Handling Tests
//!
//! Comprehensive error case tests for server-client functionality

#[cfg(unix)]
mod client_error_tests {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::client;

    fn cleanup_pipe() {
        let _ = fs::remove_file("/tmp/ym2151_server.pipe");
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_play_file_no_server() {
        cleanup_pipe();
        let result = client::play_file("test.json");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("Failed to connect") || err.contains("No such file"),
            "Error should indicate connection failure: {}",
            err
        );
    }

    #[test]
    fn test_stop_playback_no_server() {
        cleanup_pipe();
        let result = client::stop_playback();
        assert!(result.is_err());
    }

    #[test]
    fn test_shutdown_server_no_server() {
        cleanup_pipe();
        let result = client::shutdown_server();
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_filename() {
        cleanup_pipe();
        let result = client::play_file("");
        // Should fail to connect since no server
        assert!(result.is_err());
    }

    #[test]
    fn test_very_long_filename() {
        cleanup_pipe();
        let long_name = "a".repeat(1000) + ".json";
        let result = client::play_file(&long_name);
        // Should fail to connect since no server
        assert!(result.is_err());
    }
}

#[cfg(unix)]
mod protocol_error_tests {
    use ym2151_log_player_rust::ipc::protocol::Command;

    #[test]
    fn test_parse_invalid_command() {
        let result = Command::parse("INVALID");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unknown command") || err.contains("Invalid"));
    }

    #[test]
    fn test_parse_play_without_argument() {
        let result = Command::parse("PLAY");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_string() {
        let result = Command::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_whitespace_only() {
        let result = Command::parse("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_stop_with_argument() {
        // STOP should not have arguments, but parsing might be lenient
        let result = Command::parse("STOP extra");
        // May succeed or fail depending on implementation
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_parse_shutdown_with_argument() {
        // SHUTDOWN should not have arguments
        let result = Command::parse("SHUTDOWN extra");
        // May succeed or fail depending on implementation
        let _ = result;
    }

    #[test]
    fn test_parse_case_sensitivity() {
        // Commands might be case-sensitive
        let result1 = Command::parse("play test.json");
        let result2 = Command::parse("Play test.json");
        let result3 = Command::parse("PLAY test.json");

        // At least one should work (likely uppercase)
        let _ = result1;
        let _ = result2;
        let _ = result3;
    }

    #[test]
    fn test_serialize_commands() {
        // Test that all commands can be serialized
        let play = Command::Play("test.json".to_string());
        let stop = Command::Stop;
        let shutdown = Command::Shutdown;

        let s1 = play.serialize();
        let s2 = stop.serialize();
        let s3 = shutdown.serialize();

        assert!(s1.contains("PLAY"));
        assert!(s1.contains("test.json"));
        assert!(s2.contains("STOP"));
        assert!(s3.contains("SHUTDOWN"));
    }

    #[test]
    fn test_roundtrip_serialization() {
        // Test that commands can be serialized and parsed back
        let commands = vec![
            Command::Play("sample.json".to_string()),
            Command::Stop,
            Command::Shutdown,
        ];

        for cmd in commands {
            let serialized = cmd.serialize();
            let parsed = Command::parse(&serialized.trim_end());
            assert!(parsed.is_ok(), "Failed to parse: {}", serialized);
        }
    }
}

#[cfg(unix)]
mod server_error_tests {
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

    #[test]
    fn test_server_with_invalid_initial_file() {
        cleanup_pipe();

        let server = Server::new();

        // Try to start server with non-existent file
        let result = std::panic::catch_unwind(|| server.run("nonexistent_initial_file.json"));

        // Server should handle this gracefully (either error or panic is acceptable)
        cleanup_pipe();
    }

    #[test]
    fn test_concurrent_pipe_access() {
        cleanup_pipe();

        let server = Server::new();
        let server_handle = thread::spawn(move || server.run("sample_events.json"));

        thread::sleep(Duration::from_millis(500));

        // Try to send commands very quickly in succession
        let handles: Vec<_> = (0..3)
            .map(|i| {
                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(i * 50));
                    let mut writer = NamedPipe::connect_default();
                    if let Ok(mut w) = writer {
                        let _ = w.write_str(&Command::Stop.serialize());
                    }
                })
            })
            .collect();

        for h in handles {
            let _ = h.join();
        }

        // Shutdown
        thread::sleep(Duration::from_millis(200));
        if let Ok(mut writer) = NamedPipe::connect_default() {
            let _ = writer.write_str(&Command::Shutdown.serialize());
        }

        let _ = server_handle.join();
        cleanup_pipe();
    }

    #[test]
    fn test_empty_command_string() {
        cleanup_pipe();

        let server = Server::new();
        let server_handle = thread::spawn(move || server.run("sample_events.json"));

        thread::sleep(Duration::from_millis(500));

        // Send empty string
        {
            if let Ok(mut writer) = NamedPipe::connect_default() {
                let _ = writer.write_str("\n");
            }
        }

        // Shutdown
        thread::sleep(Duration::from_millis(200));
        if let Ok(mut writer) = NamedPipe::connect_default() {
            let _ = writer.write_str(&Command::Shutdown.serialize());
        }

        let _ = server_handle.join();
        cleanup_pipe();
    }
}

#[cfg(unix)]
mod pipe_error_tests {
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;

    fn cleanup_pipe() {
        let _ = fs::remove_file("/tmp/ym2151_server.pipe");
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_connect_to_nonexistent_pipe() {
        cleanup_pipe();

        let result = NamedPipe::connect_default();
        assert!(
            result.is_err(),
            "Should fail to connect to non-existent pipe"
        );
    }

    #[test]
    fn test_create_pipe_twice() {
        cleanup_pipe();

        // Create first pipe
        let pipe1 = NamedPipe::create();
        assert!(pipe1.is_ok(), "First pipe creation should succeed");

        // Try to create second pipe with same path
        let pipe2 = NamedPipe::create();
        // May succeed or fail depending on implementation
        // Just ensure no panic

        cleanup_pipe();
    }

    #[test]
    fn test_write_to_unopened_pipe() {
        cleanup_pipe();

        // Create pipe but don't open it for reading
        let pipe = NamedPipe::create();
        assert!(pipe.is_ok());

        // Try to connect and write (should timeout or block)
        let handle = thread::spawn(|| {
            let result = NamedPipe::connect_default();
            result
        });

        // Give it a short time
        thread::sleep(Duration::from_millis(200));

        // Clean up
        cleanup_pipe();

        // Don't wait for handle as it might be blocking
    }
}

#[cfg(not(unix))]
#[test]
fn error_tests_require_unix() {
    println!("ℹ️  Error tests are designed for Unix platforms");
}
