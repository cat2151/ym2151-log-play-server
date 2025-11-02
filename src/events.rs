// Event structures and JSON deserialization for YM2151 register events
//
// This module provides structures and functions for loading and parsing
// JSON event log files containing YM2151 register operations.

use serde::{Deserialize, Deserializer};
use std::fs;
use std::path::Path;

/// Parse a hexadecimal string (e.g., "0x08", "0xFF") into a u8 value.
///
/// # Errors
/// Returns an error if the string cannot be parsed as a hexadecimal number.
fn parse_hex_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let without_prefix = s.trim_start_matches("0x").trim_start_matches("0X");
    u8::from_str_radix(without_prefix, 16).map_err(serde::de::Error::custom)
}

/// Represents a single YM2151 register write event.
///
/// Each event specifies when (in samples) to write what data to which register address.
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterEvent {
    /// Sample time at which this event should occur (absolute time, not delta).
    pub time: u32,

    /// YM2151 register address to write to (parsed from hex string like "0x08").
    #[serde(deserialize_with = "parse_hex_string")]
    pub addr: u8,

    /// Data value to write to the register (parsed from hex string like "0xC7").
    #[serde(deserialize_with = "parse_hex_string")]
    pub data: u8,

    /// Optional field indicating if this is a data write (0 or 1).
    /// This field is ignored during deserialization as the player automatically
    /// handles the two-step register write process.
    #[serde(default, skip_deserializing)]
    pub is_data: Option<u8>,
}

/// Represents a complete event log loaded from JSON.
///
/// The log contains metadata about the number of events and the list of events themselves.
#[derive(Debug, Deserialize)]
pub struct EventLog {
    /// Total number of events in the log.
    pub event_count: usize,

    /// List of register write events, ordered by time.
    pub events: Vec<RegisterEvent>,
}

impl EventLog {
    /// Load an event log from a JSON file.
    ///
    /// # Parameters
    /// - `path`: Path to the JSON file to load
    ///
    /// # Errors
    /// Returns an error if:
    /// - The file cannot be read
    /// - The JSON is malformed
    /// - Required fields are missing
    /// - Hex strings cannot be parsed
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::events::EventLog;
    ///
    /// let log = EventLog::from_file("sample_events.json").unwrap();
    /// println!("Loaded {} events", log.event_count);
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let log: EventLog = serde_json::from_str(&content)?;
        Ok(log)
    }

    /// Validate that the event log is well-formed.
    ///
    /// Checks that:
    /// - The event_count matches the actual number of events
    /// - Events are sorted by time (non-strictly, duplicates allowed)
    ///
    /// # Returns
    /// `true` if the log is valid, `false` otherwise.
    pub fn validate(&self) -> bool {
        // Check event count matches
        if self.event_count != self.events.len() {
            return false;
        }

        // Check events are sorted by time
        for i in 1..self.events.len() {
            if self.events[i].time < self.events[i - 1].time {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // is_data should be ignored (None)
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
}
