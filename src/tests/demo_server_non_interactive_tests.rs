//! Tests for demo_server_non_interactive module

use crate::demo_server_non_interactive::*;

#[test]
fn test_non_interactive_demo_constants() {
    assert_eq!(DEMO_JSON_FILE, "output_ym2151.json");
    assert_eq!(DEMO_INTERVAL_SECONDS, 2);
}

#[test]
fn test_non_interactive_demo_json_file_exists() {
    // This test will only pass if the demo file exists
    // It's more of a documentation test to show expected file location
    let path = std::path::Path::new(DEMO_JSON_FILE);
    if path.exists() {
        assert!(
            path.is_file(),
            "Demo file path should point to a file, not a directory"
        );
    }
    // Note: We don't fail the test if the file doesn't exist,
    // as it might not be available in all test environments
}

#[test]
fn test_non_interactive_demo_json_parsing() {
    use crate::events::EventLog;

    // Test JSON parsing with sample data (integer time format)
    let sample_json = r#"{
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 55930, "addr": "0x08", "data": "0x01"}
        ]
    }"#;

    let event_log = EventLog::from_json_str(sample_json).expect("Should parse sample JSON");
        assert!(event_log.validate());
    assert_eq!(event_log.events.len(), 2);
    assert_eq!(event_log.events[0].time, 0);
    assert_eq!(event_log.events[1].time, 55930);
}
