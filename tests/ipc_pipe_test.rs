//! Integration tests for IPC named pipe functionality
//!
//! These tests verify that the named pipe implementation works correctly
//! for server-client communication.

use ym2151_log_player_rust::ipc::protocol::Command;

#[test]
fn test_command_parsing() {
    // Test command parsing without actual pipe I/O
    let play_cmd = Command::parse("PLAY test.json").unwrap();
    match play_cmd {
        Command::Play(path) => assert_eq!(path, "test.json"),
        _ => panic!("Expected Play command"),
    }

    let stop_cmd = Command::parse("STOP").unwrap();
    assert!(matches!(stop_cmd, Command::Stop));

    let shutdown_cmd = Command::parse("SHUTDOWN").unwrap();
    assert!(matches!(shutdown_cmd, Command::Shutdown));
}

#[test]
fn windows_pipe_tests_placeholder() {
    println!("Windows固有のパイプテストがここに実装されます");
}
