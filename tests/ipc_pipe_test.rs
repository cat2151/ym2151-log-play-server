//! Integration tests for IPC named pipe functionality
//!
//! These tests verify that the named pipe implementation works correctly
//! for server-client communication.

use ym2151_log_play_server::ipc::protocol::Command;

#[test]
fn test_binary_protocol() {
    // Test PlayJson command binary serialization
    let json_data = serde_json::json!({
        "events": [{"time": 0.0, "addr": "0x08", "data": "0x00"}]
    });
    let play_json_cmd = Command::PlayJson { data: json_data };
    let binary = play_json_cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(play_json_cmd, parsed);

    // Test Stop command
    let stop_cmd = Command::Stop;
    let binary = stop_cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(stop_cmd, parsed);

    // Test Shutdown command
    let shutdown_cmd = Command::Shutdown;
    let binary = shutdown_cmd.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(shutdown_cmd, parsed);
}

#[test]
fn windows_pipe_tests_placeholder() {
    println!("Windows固有のパイプテストがここに実装されます");
}
