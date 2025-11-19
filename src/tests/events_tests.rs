use crate::events::EventLog;

#[test]
fn test_parse_simple_json() {
    let json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert_eq!(log.event_count, 2);
    assert_eq!(log.events.len(), 2);

    assert_eq!(log.events[0].time, 0);
    assert_eq!(log.events[0].addr, 0x08);
    assert_eq!(log.events[0].data, 0x00);

    assert_eq!(log.events[1].time, 2);
    assert_eq!(log.events[1].addr, 0x20);
    assert_eq!(log.events[1].data, 0xC7);
}

#[test]
fn test_parse_with_is_data_field() {
    let json = r#"{
        "event_count": 1,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00", "is_data": 1}
        ]
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert_eq!(log.events.len(), 1);

    assert!(log.events[0].is_data.is_none());
}

#[test]
fn test_uppercase_hex_strings() {
    let json = r#"{
        "event_count": 1,
        "events": [
            {"time": 100, "addr": "0XFF", "data": "0XAB"}
        ]
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert_eq!(log.events[0].addr, 0xFF);
    assert_eq!(log.events[0].data, 0xAB);
}

#[test]
fn test_empty_events_list() {
    let json = r#"{
        "event_count": 0,
        "events": []
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert_eq!(log.event_count, 0);
    assert_eq!(log.events.len(), 0);
}

#[test]
fn test_invalid_hex_string() {
    let json = r#"{
        "event_count": 1,
        "events": [
            {"time": 0, "addr": "0xZZ", "data": "0x00"}
        ]
    }"#;

    let result: Result<EventLog, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_missing_required_field() {
    let json = r#"{
        "event_count": 1,
        "events": [
            {"time": 0, "addr": "0x08"}
        ]
    }"#;

    let result: Result<EventLog, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_malformed_json() {
    let json = r#"{
        "event_count": 1,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"
    }"#;

    let result: Result<EventLog, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_validate_correct_log() {
    let json = r#"{
        "event_count": 3,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2, "addr": "0x20", "data": "0xC7"},
            {"time": 100, "addr": "0x28", "data": "0x3E"}
        ]
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert!(log.validate());
}

#[test]
fn test_validate_wrong_count() {
    let json = r#"{
        "event_count": 5,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert!(!log.validate());
}

#[test]
fn test_validate_unsorted_events() {
    let json = r#"{
        "event_count": 2,
        "events": [
            {"time": 100, "addr": "0x08", "data": "0x00"},
            {"time": 2, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert!(!log.validate());
}

#[test]
fn test_large_time_values() {
    let json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 111862, "addr": "0x08", "data": "0x00"}
        ]
    }"#;

    let log: EventLog = serde_json::from_str(json).unwrap();
    assert_eq!(log.events[1].time, 111862);
}

#[test]
fn test_from_json_str() {
    let json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let log = EventLog::from_json_str(json).unwrap();
    assert_eq!(log.event_count, 2);
    assert_eq!(log.events.len(), 2);
    assert_eq!(log.events[0].time, 0);
    assert_eq!(log.events[1].addr, 0x20);
}

#[test]
fn test_from_json_str_validates() {
    let json = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let log = EventLog::from_json_str(json).unwrap();
    assert!(log.validate());
}

#[test]
fn test_from_json_str_invalid_json() {
    let json = r#"{"event_count": 1, "events": [}"#;
    let result = EventLog::from_json_str(json);
    assert!(result.is_err());
}

#[test]
fn test_json_string_vs_file_workflow() {
    // This test demonstrates the difference between file-based and string-based loading

    // Scenario 1: JSON string (new feature)
    let json_string = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 100, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let log_from_string = EventLog::from_json_str(json_string).unwrap();
    assert_eq!(log_from_string.event_count, 2);
    assert!(log_from_string.validate());

    // Scenario 2: Both methods should produce identical results
    // (We can't test file loading in this test without creating temp files,
    // but the from_file method internally uses the same serde_json::from_str)

    // Verify the data is correctly parsed
    assert_eq!(log_from_string.events[0].addr, 0x08);
    assert_eq!(log_from_string.events[1].addr, 0x20);
    assert_eq!(log_from_string.events[1].data, 0xC7);
}
