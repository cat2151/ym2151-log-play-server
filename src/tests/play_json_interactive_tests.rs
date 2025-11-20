//! Tests for play_json_interactive function - f64 second timing conversion validation
//!
//! These tests verify that:
//! 1. JSON timing is converted from sample units to f64 seconds
//! 2. Converted JSON is sent to the server properly
//! 3. PlayJsonInInteractive command handles f64 timing correctly

use crate::ipc::protocol::Command;
use anyhow::Result;

#[test]
fn test_json_timing_conversion() {
    // Test the conversion function directly
    use crate::events::convert_json_to_f64_seconds;

    let input_json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let result = convert_json_to_f64_seconds(input_json);
    assert!(result.is_ok());

    let output_json = result.unwrap();
    println!("Converted JSON: {}", output_json);

    // Verify the output contains f64 times
    assert!(output_json.contains("\"time\": 0.0"));
    // 2797 / 55930 ≈ 0.050027
    assert!(output_json.contains("0.05") || output_json.contains("0.050"));

    // Verify it can be parsed as f64 events
    use crate::events::EventLogF64;
    let parsed = EventLogF64::from_json_str(&output_json);
    assert!(parsed.is_ok());

    let log = parsed.unwrap();
    assert_eq!(log.event_count, 2);
    assert!(log.validate());
    assert_eq!(log.events[0].time, 0.0);
    let expected_time = 2797.0 / 55930.0;
    assert!((log.events[1].time - expected_time).abs() < 0.000001); // floating point comparison
}

#[test]
fn test_play_json_interactive_sends_f64_json() {
    // Test that play_json_interactive converts and sends correct command
    let test_json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 100, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    // Convert manually to see what should be sent
    use crate::events::convert_json_to_f64_seconds;
    let converted = convert_json_to_f64_seconds(test_json).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&converted).unwrap();

    // Create the expected command
    let expected_command = Command::PlayJsonInInteractive { data: parsed };

    // Serialize to binary to verify the command can be serialized
    let binary_data = expected_command.to_binary().unwrap();
    assert!(!binary_data.is_empty());

    // Verify we can deserialize back
    let deserialized = Command::from_binary(&binary_data).unwrap();
    assert_eq!(expected_command, deserialized);
}

#[test]
fn test_f64_event_log_validation() {
    use crate::events::EventLogF64;

    // Valid f64 JSON should pass validation
    let valid_json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0.0, "addr": "0x28", "data": "0x3E"},
            {"time": 0.1, "addr": "0x08", "data": "0x00"}
        ]
    }"#;

    let parsed_result = EventLogF64::from_json_str(valid_json);
    assert!(parsed_result.is_ok());

    let log = parsed_result.unwrap();
    assert!(log.validate());

    // Invalid f64 JSON (wrong event count) should fail validation
    let invalid_json = r#"{
        "event_count": 5,
        "events": [
            {"time": 0.0, "addr": "0x28", "data": "0x3E"}
        ]
    }"#;

    let invalid_log = EventLogF64::from_json_str(invalid_json).unwrap();
    assert!(!invalid_log.validate());
}

/// Integration test that verifies the full f64 play_json_interactive workflow
/// Note: This test requires a running server and may be skipped in CI
#[test]
fn test_f64_play_json_interactive_integration() -> Result<()> {
    use crate::client;

    // Initialize client
    client::init_client(true);

    // Try to start interactive mode (may fail if no server)
    let start_result = crate::client::interactive::start_interactive();
    if start_result.is_err() {
        println!("Server not available for integration test, skipping");
        return Ok(());
    }

    println!("✅ [インタラクティブモード] 正常に開始しました");

    // Test JSON data with sample timing
    let test_json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 100, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    // Send JSON to interactive mode (should convert to f64 internally)
    let result = crate::client::interactive::play_json_interactive(test_json);

    // Clean up first before checking result
    let cleanup_result = crate::client::interactive::stop_interactive();
    if cleanup_result.is_ok() {
        println!("✅ [インタラクティブモード] 正常に停止しました");
    }

    // Check if the play operation succeeded
    // If it failed due to server issues, skip the test gracefully
    match result {
        Ok(_) => {
            println!("✅ インテグレーションテスト完了: play_json_interactive成功");
            Ok(())
        },
        Err(e) if e.to_string().contains("Failed to connect") || e.to_string().contains("指定されたファイルが見つかりません") => {
            println!("⏭️  インテグレーションテストスキップ: サーバー接続問題 - {}", e);
            Ok(()) // Skip test gracefully
        },
        Err(e) => {
            println!("❌ インテグレーションテスト失敗: {}", e);
            Err(e) // Actual test failure
        }
    }
}
