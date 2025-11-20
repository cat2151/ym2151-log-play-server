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

#[derive(Debug, Clone, Deserialize)]
pub struct EventLog {
    pub event_count: u32,
    pub events: Vec<RegisterEvent>,
}

// F64版の構造体（時間表現がf64秒）
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterEventF64 {
    pub time: f64,

    #[serde(deserialize_with = "parse_hex_string")]
    pub addr: u8,

    #[serde(deserialize_with = "parse_hex_string")]
    pub data: u8,

    #[serde(default, skip_deserializing)]
    pub is_data: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventLogF64 {
    pub event_count: u32,
    pub events: Vec<RegisterEventF64>,
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
    ///     "event_count": 2,
    ///     "events": [
    ///         {"time": 0, "addr": "0x08", "data": "0x00"},
    ///         {"time": 2, "addr": "0x20", "data": "0xC7"}
    ///     ]
    /// }"#;
    ///
    /// let log = EventLog::from_json_str(json_str).unwrap();
    /// assert_eq!(log.event_count, 2);
    /// assert!(log.validate());
    /// ```
    pub fn from_json_str(json_str: &str) -> anyhow::Result<Self> {
        let log: EventLog = serde_json::from_str(json_str)?;
        Ok(log)
    }

    pub fn validate(&self) -> bool {
        // Check if event_count matches actual number of events
        if self.event_count != self.events.len() as u32 {
            return false;
        }

        // Check if events are sorted by time
        for i in 1..self.events.len() {
            if self.events[i].time < self.events[i - 1].time {
                return false;
            }
        }

        true
    }
}

impl EventLogF64 {
    /// Parse event log directly from a JSON string (for f64 time format)
    pub fn from_json_str(json_str: &str) -> anyhow::Result<Self> {
        let log: EventLogF64 = serde_json::from_str(json_str)?;
        Ok(log)
    }

    pub fn validate(&self) -> bool {
        // Check if event_count matches actual number of events
        if self.event_count != self.events.len() as u32 {
            return false;
        }

        // Check if events are sorted by time
        for i in 1..self.events.len() {
            if self.events[i].time < self.events[i - 1].time {
                return false;
            }
        }

        true
    }
}

/// Convert JSON from integer time (samples) to f64 time (seconds)
///
/// This function converts event timing from OPM samples (55930 Hz) to seconds.
/// This is useful for interactive playback and timing analysis.
///
/// The function also validates the input JSON to ensure:
/// - Event count matches the actual number of events
/// - Events are in chronological order
pub fn convert_json_to_f64_seconds(json_str: &str) -> anyhow::Result<String> {
    const OPM_SAMPLE_RATE: f64 = 55930.0;

    // Parse the original JSON
    let log: EventLog = serde_json::from_str(json_str)?;

    // Validate the event log
    if !log.validate() {
        return Err(anyhow::anyhow!("Invalid input event log"));
    }

    // Convert to f64 format
    let mut f64_events = Vec::new();
    for event in log.events {
        f64_events.push(serde_json::json!({
            "time": event.time as f64 / OPM_SAMPLE_RATE,
            "addr": format!("0x{:02x}", event.addr),
            "data": format!("0x{:02x}", event.data)
        }));
    }

    let f64_log = serde_json::json!({
        "event_count": log.event_count,
        "events": f64_events
    });

    Ok(serde_json::to_string_pretty(&f64_log)?)
}
