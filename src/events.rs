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
    pub time: f64,

    #[serde(deserialize_with = "parse_hex_string")]
    pub addr: u8,

    #[serde(deserialize_with = "parse_hex_string")]
    pub data: u8,

    #[serde(default, skip_deserializing)]
    pub is_data: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventLog {
    pub events: Vec<RegisterEvent>,
}

impl EventLog {
    /// Load event log from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let log: EventLog = serde_json::from_str(&content)?;
        Ok(log)
    }

    /// Parse event log directly from a JSON string
    ///
    /// This is useful when receiving JSON data via IPC (e.g., named pipes)
    /// without writing to an intermediate file first.
    ///
    /// # Example
    /// ```
    /// # use ym2151_log_play_server::events::EventLog;
    /// let json_str = r#"{
    ///     "events": [
    ///         {"time": 0.0, "addr": "0x08", "data": "0x00"},
    ///         {"time": 0.0001, "addr": "0x20", "data": "0xC7"}
    ///     ]
    /// }"#;
    ///
    /// let log = EventLog::from_json_str(json_str).unwrap();
    /// assert!(log.validate());
    /// ```
    pub fn from_json_str(json_str: &str) -> anyhow::Result<Self> {
        let log: EventLog = serde_json::from_str(json_str)?;
        Ok(log)
    }

    pub fn validate(&self) -> bool {
        // Check if events are sorted by time
        for i in 1..self.events.len() {
            if self.events[i].time < self.events[i - 1].time {
                return false;
            }
        }

        true
    }
}
