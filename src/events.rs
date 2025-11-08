use serde::{Deserialize, Deserializer};
use std::fs;
use std::path::Path;

fn parse_hex_string<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let without_prefix = s.trim_start_matches("0x").trim_start_matches("0X");
    u8::from_str_radix(without_prefix, 16).map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterEvent {
    pub time: u32,

    #[serde(deserialize_with = "parse_hex_string")]
    pub addr: u8,

    #[serde(deserialize_with = "parse_hex_string")]
    pub data: u8,

    #[serde(default, skip_deserializing)]
    pub is_data: Option<u8>,
}

#[derive(Debug, Deserialize)]
pub struct EventLog {
    pub event_count: usize,

    pub events: Vec<RegisterEvent>,
}

impl EventLog {
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let log: EventLog = serde_json::from_str(&content)?;
        Ok(log)
    }

    pub fn validate(&self) -> bool {
        if self.event_count != self.events.len() {
            return false;
        }

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
