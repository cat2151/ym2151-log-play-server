//! Tests for the play_json_interactive convenience function
//!
//! These tests verify the JSON parsing, validation, and error handling
//! without requiring a running server.

#[cfg(windows)]
mod windows_tests {
    use ym2151_log_play_server::client;

    #[test]
    fn test_play_json_interactive_parses_simple_json() {
        let json_data = r#"{
            "event_count": 2,
            "events": [
                {"time": 0, "addr": "0x08", "data": "0x00"},
                {"time": 2797, "addr": "0x20", "data": "0xC7"}
            ]
        }"#;

        // This will fail to connect to server, but should successfully parse JSON first
        let result = client::play_json_interactive(json_data);

        // Should fail because server is not running, but not because of JSON parsing
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // Error should be about server connection, not JSON parsing
        assert!(
            error_msg.contains("Failed to start interactive mode")
                || error_msg.contains("Failed to connect"),
            "Unexpected error: {}",
            error_msg
        );
    }

    #[test]
    fn test_play_json_interactive_rejects_malformed_json() {
        let invalid_json = r#"{"event_count": 1, "events": [}"#;

        let result = client::play_json_interactive(invalid_json);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Failed to parse JSON"),
            "Expected JSON parse error, got: {}",
            error_msg
        );
    }

    #[test]
    fn test_play_json_interactive_validates_event_count() {
        // Event count doesn't match actual events
        let json_data = r#"{
            "event_count": 5,
            "events": [
                {"time": 0, "addr": "0x08", "data": "0x00"}
            ]
        }"#;

        let result = client::play_json_interactive(json_data);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("validation failed"),
            "Expected validation error, got: {}",
            error_msg
        );
    }

    #[test]
    fn test_play_json_interactive_validates_time_ordering() {
        // Events not in time order
        let json_data = r#"{
            "event_count": 2,
            "events": [
                {"time": 100, "addr": "0x08", "data": "0x00"},
                {"time": 50, "addr": "0x20", "data": "0xC7"}
            ]
        }"#;

        let result = client::play_json_interactive(json_data);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("validation failed"),
            "Expected validation error, got: {}",
            error_msg
        );
    }

    #[test]
    fn test_play_json_interactive_accepts_empty_events() {
        let json_data = r#"{
            "event_count": 0,
            "events": []
        }"#;

        // Empty events should be valid but will fail at server connection
        let result = client::play_json_interactive(json_data);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // Should fail on server connection, not validation
        assert!(
            error_msg.contains("Failed to start interactive mode")
                || error_msg.contains("Failed to connect"),
            "Expected server connection error, got: {}",
            error_msg
        );
    }

    #[test]
    fn test_play_json_interactive_handles_hex_formats() {
        // Test both uppercase and lowercase hex
        let json_data = r#"{
            "event_count": 2,
            "events": [
                {"time": 0, "addr": "0x08", "data": "0xFF"},
                {"time": 100, "addr": "0XAB", "data": "0xcd"}
            ]
        }"#;

        let result = client::play_json_interactive(json_data);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // Should parse hex correctly and fail on server connection
        assert!(
            error_msg.contains("Failed to start interactive mode")
                || error_msg.contains("Failed to connect"),
            "Expected server connection error (hex parsing should succeed), got: {}",
            error_msg
        );
    }

    #[test]
    fn test_play_json_interactive_rejects_invalid_hex() {
        let json_data = r#"{
            "event_count": 1,
            "events": [
                {"time": 0, "addr": "0xZZ", "data": "0x00"}
            ]
        }"#;

        let result = client::play_json_interactive(json_data);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Failed to parse JSON"),
            "Expected JSON parse error for invalid hex, got: {}",
            error_msg
        );
    }

    #[test]
    fn test_play_json_interactive_handles_large_time_values() {
        // Test with large sample counts (e.g., several seconds of playback)
        let json_data = r#"{
            "event_count": 3,
            "events": [
                {"time": 0, "addr": "0x08", "data": "0x00"},
                {"time": 55930, "addr": "0x20", "data": "0xC7"},
                {"time": 111860, "addr": "0x28", "data": "0x3E"}
            ]
        }"#;

        let result = client::play_json_interactive(json_data);
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        // Should handle large values and fail on server connection
        assert!(
            error_msg.contains("Failed to start interactive mode")
                || error_msg.contains("Failed to connect"),
            "Expected server connection error (large values should be valid), got: {}",
            error_msg
        );
    }
}

#[cfg(not(windows))]
mod non_windows_tests {
    #[test]
    fn test_play_json_interactive_not_available_on_non_windows() {
        // On non-Windows platforms, the client module is not available
        // This test just documents that behavior
        assert!(true);
    }
}

// Cross-platform tests for time conversion logic
#[test]
fn test_time_conversion_accuracy() {
    use ym2151_log_play_server::resampler::OPM_SAMPLE_RATE;

    // Verify OPM_SAMPLE_RATE constant value
    assert_eq!(OPM_SAMPLE_RATE, 55930);

    // Test conversions that would be used by play_json_interactive

    // 0 samples = 0.0 seconds
    let time_sec = 0_f64 / OPM_SAMPLE_RATE as f64;
    assert_eq!(time_sec, 0.0);

    // 55930 samples = 1.0 second
    let time_sec = 55930_f64 / OPM_SAMPLE_RATE as f64;
    assert!((time_sec - 1.0).abs() < 0.000001);

    // 2797 samples ≈ 0.050 seconds (50ms)
    let time_sec = 2797_f64 / OPM_SAMPLE_RATE as f64;
    assert!((time_sec - 0.050).abs() < 0.0001);

    // 5593 samples ≈ 0.100 seconds (100ms)
    let time_sec = 5593_f64 / OPM_SAMPLE_RATE as f64;
    assert!((time_sec - 0.100).abs() < 0.0001);
}

#[test]
fn test_event_log_parsing_for_interactive_mode() {
    use ym2151_log_play_server::events::EventLog;

    // Test the EventLog parser that play_json_interactive uses
    let json_data = r#"{
        "event_count": 3,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x20", "data": "0xC7"},
            {"time": 5594, "addr": "0x28", "data": "0x3E"}
        ]
    }"#;

    let event_log = EventLog::from_json_str(json_data).unwrap();
    assert_eq!(event_log.event_count, 3);
    assert!(event_log.validate());

    // Verify event values
    assert_eq!(event_log.events[0].time, 0);
    assert_eq!(event_log.events[0].addr, 0x08);
    assert_eq!(event_log.events[0].data, 0x00);

    assert_eq!(event_log.events[1].time, 2797);
    assert_eq!(event_log.events[1].addr, 0x20);
    assert_eq!(event_log.events[1].data, 0xC7);

    assert_eq!(event_log.events[2].time, 5594);
    assert_eq!(event_log.events[2].addr, 0x28);
    assert_eq!(event_log.events[2].data, 0x3E);
}
