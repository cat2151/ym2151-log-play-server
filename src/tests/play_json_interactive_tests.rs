//! Tests for play_json_interactive function - f64 second timing
//!
//! These tests verify that:
//! 1. JSON timing uses f64 seconds
//! 2. JSON is sent to the server properly
//! 3. PlayJsonInInteractive command handles f64 timing correctly

use crate::ipc::protocol::Command;
use anyhow::Result;

#[test]
fn test_json_f64_parsing() {
    // Test parsing f64 JSON directly
    use crate::events::EventLog;

    let input_json = r#"{
        "events": [
            {"time": 0.0, "addr": "0x08", "data": "0x00"},
            {"time": 0.05, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let result = EventLog::from_json_str(input_json);
    assert!(result.is_ok());

    let log = result.unwrap();
    assert!(log.validate());
    assert_eq!(log.events[0].time, 0.0);
    assert_eq!(log.events[1].time, 0.05);
}

#[test]
fn test_play_json_interactive_sends_json() {
    // Test that play_json_interactive sends correct command
    let test_json = r#"{
        "events": [
            {"time": 0.0, "addr": "0x08", "data": "0x00"},
            {"time": 0.5, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let parsed: serde_json::Value = serde_json::from_str(test_json).unwrap();

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
fn test_event_log_validation() {
    use crate::events::EventLog;

    // Valid JSON should pass validation
    let valid_json = r#"{
        "events": [
            {"time": 0.0, "addr": "0x28", "data": "0x3E"},
            {"time": 0.1, "addr": "0x08", "data": "0x00"}
        ]
    }"#;

    let parsed_result = EventLog::from_json_str(valid_json);
    assert!(parsed_result.is_ok());

    let log = parsed_result.unwrap();
    assert!(log.validate());

    // Valid single-event JSON should pass validation
    let single_event_json = r#"{
        "events": [
            {"time": 0.0, "addr": "0x28", "data": "0x3E"}
        ]
    }"#;

    let single_log = EventLog::from_json_str(single_event_json).unwrap();
    assert!(single_log.validate());
}

/// Integration test that verifies the full play_json_interactive workflow
/// Note: This test requires a running server and may be skipped in CI
#[test]
fn test_play_json_interactive_integration() -> Result<()> {
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

    // Test JSON data with f64 second timing
    let test_json = r#"{
        "events": [
            {"time": 0.0, "addr": "0x08", "data": "0x00"},
            {"time": 0.5, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    // Send JSON to interactive mode
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
