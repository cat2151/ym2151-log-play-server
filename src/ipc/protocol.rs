use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    PlayJson {
        data: serde_json::Value,
    },
    Stop,
    Shutdown,
    StartInteractive,
    WriteRegister {
        /// Time offset in seconds (f64) from now, for Web Audio-style scheduling
        /// Provides sample-accurate precision at 55930 Hz (1/55930 = ~0.0000179 sec per sample)
        time_offset_sec: f64,
        addr: u8,
        data: u8,
    },
    StopInteractive,
    /// Get the current server time in the server time coordinate system (f64 seconds)
    /// This allows clients to synchronize with the server's timeline for precise scheduling
    GetServerTime,
    /// Clear all scheduled events in interactive mode
    /// This allows seamless phrase transitions without audio gaps
    ClearSchedule,
}

impl Command {
    /// Parse command from binary (length-prefixed JSON) format
    pub fn from_binary(data: &[u8]) -> Result<Self, String> {
        if data.len() < 4 {
            return Err("Invalid binary data: too short".to_string());
        }

        let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;

        if data.len() < 4 + len {
            return Err(format!(
                "Invalid binary data: expected {} bytes, got {}",
                4 + len,
                data.len()
            ));
        }

        let json_bytes = &data[4..4 + len];
        let json_str =
            std::str::from_utf8(json_bytes).map_err(|e| format!("Invalid UTF-8 in JSON: {}", e))?;

        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Serialize command to binary (length-prefixed JSON) format
    pub fn to_binary(&self) -> Result<Vec<u8>, String> {
        let json_str =
            serde_json::to_string(self).map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        let json_bytes = json_str.as_bytes();
        let len = json_bytes.len() as u32;

        let mut result = Vec::with_capacity(4 + json_bytes.len());
        result.extend_from_slice(&len.to_le_bytes());
        result.extend_from_slice(json_bytes);

        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Response {
    Ok,
    Error {
        message: String,
    },
    /// Server time response containing current time in seconds (f64)
    ServerTime {
        time_sec: f64,
    },
}

impl Response {
    /// Parse response from binary (length-prefixed JSON) format
    pub fn from_binary(data: &[u8]) -> Result<Self, String> {
        if data.len() < 4 {
            return Err("Invalid binary data: too short".to_string());
        }

        let len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;

        if data.len() < 4 + len {
            return Err(format!(
                "Invalid binary data: expected {} bytes, got {}",
                4 + len,
                data.len()
            ));
        }

        let json_bytes = &data[4..4 + len];
        let json_str =
            std::str::from_utf8(json_bytes).map_err(|e| format!("Invalid UTF-8 in JSON: {}", e))?;

        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    /// Serialize response to binary (length-prefixed JSON) format
    pub fn to_binary(&self) -> Result<Vec<u8>, String> {
        let json_str =
            serde_json::to_string(self).map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        let json_bytes = json_str.as_bytes();
        let len = json_bytes.len() as u32;

        let mut result = Vec::with_capacity(4 + json_bytes.len());
        result.extend_from_slice(&len.to_le_bytes());
        result.extend_from_slice(json_bytes);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let json_str =
            r#"{"command":"play_json","data":{"event_count":0,"events":[]},"silent":true}"#;
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
}
