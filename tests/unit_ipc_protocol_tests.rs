use ym2151_log_play_server::ipc::protocol::{Command, Response};

// Binary protocol tests

#[test]
fn test_binary_play_json_roundtrip() {
    let json_data = serde_json::json!({
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2, "addr": "0x20", "data": "0xC7"}
        ]
    });
    let original = Command::PlayJson { data: json_data };
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_stop_roundtrip() {
    let original = Command::Stop;
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_shutdown_roundtrip() {
    let original = Command::Shutdown;
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_response_ok_roundtrip() {
    let original = Response::Ok;
    let binary = original.to_binary().unwrap();
    let parsed = Response::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_response_error_roundtrip() {
    let original = Response::Error {
        message: "Test error".to_string(),
    };
    let binary = original.to_binary().unwrap();
    let parsed = Response::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_invalid_too_short() {
    let data = vec![1, 2]; // Only 2 bytes, need at least 4
    let result = Command::from_binary(&data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("too short"));
}

#[test]
fn test_binary_invalid_length_mismatch() {
    let mut data = vec![10, 0, 0, 0]; // Says 10 bytes of JSON
    data.extend_from_slice(b"short"); // But only 5 bytes
    let result = Command::from_binary(&data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("expected"));
}

#[test]
fn test_binary_invalid_utf8() {
    let mut data = vec![3, 0, 0, 0]; // 3 bytes of "JSON"
    data.extend_from_slice(&[0xFF, 0xFE, 0xFD]); // Invalid UTF-8
    let result = Command::from_binary(&data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("UTF-8"));
}

#[test]
fn test_binary_invalid_json() {
    let mut data = vec![8, 0, 0, 0]; // 8 bytes
    data.extend_from_slice(b"not json"); // Valid UTF-8 but not JSON
    let result = Command::from_binary(&data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("parse JSON"));
}

#[test]
fn test_binary_play_json_with_silent_removed() {
    // Test that PlayJson works without silent field
    let json_data = serde_json::json!({
        "event_count": 1,
        "events": [{"time": 0, "addr": "0x08", "data": "0x00"}]
    });
    let original = Command::PlayJson { data: json_data };
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_play_json_backward_compatibility() {
    // Test that old JSON with silent field still deserializes (field is ignored)
    let json_str = r#"{"command":"play_json","data":{"event_count":0,"events":[]},"silent":true}"#;
    let json_bytes = json_str.as_bytes();

    let mut binary = Vec::with_capacity(4 + json_bytes.len());
    binary.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    binary.extend_from_slice(json_bytes);

    let parsed = Command::from_binary(&binary).unwrap();
    match parsed {
        Command::PlayJson { data: _ } => {
            // Successfully parsed, silent field is ignored
        }
        _ => panic!("Expected PlayJson command"),
    }
}

#[test]
fn test_binary_length_prefix_format() {
    let cmd = Command::Stop;
    let binary = cmd.to_binary().unwrap();

    // First 4 bytes are the length in little-endian
    let len = u32::from_le_bytes([binary[0], binary[1], binary[2], binary[3]]) as usize;

    // The JSON part should match the length
    assert_eq!(binary.len(), 4 + len);

    // The JSON part should be valid UTF-8
    let json_str = std::str::from_utf8(&binary[4..]).unwrap();
    assert!(json_str.contains("stop"));
}

#[test]
fn test_binary_start_interactive_roundtrip() {
    let original = Command::StartInteractive;
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_write_register_roundtrip() {
    let original = Command::WriteRegister {
        time_offset_sec: 0.050,
        addr: 0x08,
        data: 0x78,
    };
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_stop_interactive_roundtrip() {
    let original = Command::StopInteractive;
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_get_server_time_roundtrip() {
    let original = Command::GetServerTime;
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_server_time_response_roundtrip() {
    let original = Response::ServerTime { time_sec: 1.234567 };
    let binary = original.to_binary().unwrap();
    let parsed = Response::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}

#[test]
fn test_binary_clear_schedule_roundtrip() {
    let original = Command::ClearSchedule;
    let binary = original.to_binary().unwrap();
    let parsed = Command::from_binary(&binary).unwrap();
    assert_eq!(original, parsed);
}
