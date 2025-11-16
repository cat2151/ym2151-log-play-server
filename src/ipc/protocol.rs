use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    PlayFile { path: String },
    PlayJson { data: serde_json::Value },
    Stop,
    Shutdown,
}

impl Command {
    /// Parse command from legacy text format for backward compatibility
    pub fn parse(line: &str) -> Result<Self, String> {
        let line = line.trim();

        if line.is_empty() {
            return Err("Empty command".to_string());
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();

        match parts[0] {
            "PLAY" => {
                if parts.len() < 2 {
                    Err("PLAY command requires a path argument".to_string())
                } else {
                    Ok(Command::PlayFile {
                        path: parts[1].to_string(),
                    })
                }
            }
            "STOP" => Ok(Command::Stop),
            "SHUTDOWN" => Ok(Command::Shutdown),
            _ => Err(format!("Unknown command: {}", parts[0])),
        }
    }

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
        let json_str = std::str::from_utf8(json_bytes)
            .map_err(|e| format!("Invalid UTF-8 in JSON: {}", e))?;

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

    /// Serialize command to legacy text format for backward compatibility
    pub fn serialize(&self) -> String {
        match self {
            Command::PlayFile { path } => format!("PLAY {}\n", path),
            Command::PlayJson { data } => {
                format!("PLAY {}\n", serde_json::to_string(data).unwrap_or_default())
            }
            Command::Stop => "STOP\n".to_string(),
            Command::Shutdown => "SHUTDOWN\n".to_string(),
        }
    }

    /// Check if a string appears to be JSON data (starts with '{', ends with '}')
    ///
    /// This is used to distinguish between file paths and JSON string data
    /// when processing PLAY commands.
    ///
    /// # Example
    /// ```
    /// # use ym2151_log_play_server::ipc::protocol::Command;
    /// assert!(Command::is_json_string(r#"{"key": "value"}"#));
    /// assert!(!Command::is_json_string("/path/to/file.json"));
    /// ```
    pub fn is_json_string(s: &str) -> bool {
        let trimmed = s.trim();
        trimmed.starts_with('{') && trimmed.ends_with('}')
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Response {
    Ok,
    Error { message: String },
}

impl Response {
    /// Serialize response to legacy text format for backward compatibility
    pub fn serialize(&self) -> String {
        match self {
            Response::Ok => "OK\n".to_string(),
            Response::Error { message } => format!("ERROR {}\n", message),
        }
    }

    /// Parse response from legacy text format
    pub fn parse(line: &str) -> Result<Self, String> {
        let line = line.trim();

        if line.is_empty() {
            return Err("Empty response".to_string());
        }

        if line == "OK" {
            return Ok(Response::Ok);
        }

        if let Some(msg) = line.strip_prefix("ERROR ") {
            return Ok(Response::Error {
                message: msg.to_string(),
            });
        }

        Err(format!("Unknown response: {}", line))
    }

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
        let json_str = std::str::from_utf8(json_bytes)
            .map_err(|e| format!("Invalid UTF-8 in JSON: {}", e))?;

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

    #[test]
    fn test_parse_play_command() {
        let cmd = Command::parse("PLAY /path/to/file.json").unwrap();
        assert_eq!(
            cmd,
            Command::PlayFile {
                path: "/path/to/file.json".to_string()
            }
        );
    }

    #[test]
    fn test_parse_play_command_with_spaces() {
        let cmd = Command::parse("PLAY /path/with spaces/file.json").unwrap();
        assert_eq!(
            cmd,
            Command::PlayFile {
                path: "/path/with spaces/file.json".to_string()
            }
        );
    }

    #[test]
    fn test_parse_stop_command() {
        let cmd = Command::parse("STOP").unwrap();
        assert_eq!(cmd, Command::Stop);
    }

    #[test]
    fn test_parse_shutdown_command() {
        let cmd = Command::parse("SHUTDOWN").unwrap();
        assert_eq!(cmd, Command::Shutdown);
    }

    #[test]
    fn test_parse_play_without_path() {
        let result = Command::parse("PLAY");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires a path"));
    }

    #[test]
    fn test_parse_empty_command() {
        let result = Command::parse("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty command");
    }

    #[test]
    fn test_parse_unknown_command() {
        let result = Command::parse("UNKNOWN");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown command"));
    }

    #[test]
    fn test_parse_command_with_whitespace() {
        let cmd = Command::parse("  STOP  ").unwrap();
        assert_eq!(cmd, Command::Stop);
    }

    #[test]
    fn test_serialize_play_command() {
        let cmd = Command::PlayFile {
            path: "/path/to/file.json".to_string(),
        };
        assert_eq!(cmd.serialize(), "PLAY /path/to/file.json\n");
    }

    #[test]
    fn test_serialize_stop_command() {
        let cmd = Command::Stop;
        assert_eq!(cmd.serialize(), "STOP\n");
    }

    #[test]
    fn test_serialize_shutdown_command() {
        let cmd = Command::Shutdown;
        assert_eq!(cmd.serialize(), "SHUTDOWN\n");
    }

    #[test]
    fn test_serialize_ok_response() {
        let resp = Response::Ok;
        assert_eq!(resp.serialize(), "OK\n");
    }

    #[test]
    fn test_serialize_error_response() {
        let resp = Response::Error {
            message: "File not found".to_string(),
        };
        assert_eq!(resp.serialize(), "ERROR File not found\n");
    }

    #[test]
    fn test_serialize_error_response_with_special_chars() {
        let resp = Response::Error {
            message: "Path: /invalid/path".to_string(),
        };
        assert_eq!(resp.serialize(), "ERROR Path: /invalid/path\n");
    }

    #[test]
    fn test_parse_ok_response() {
        let resp = Response::parse("OK").unwrap();
        assert_eq!(resp, Response::Ok);
    }

    #[test]
    fn test_parse_error_response() {
        let resp = Response::parse("ERROR File not found").unwrap();
        assert_eq!(
            resp,
            Response::Error {
                message: "File not found".to_string()
            }
        );
    }

    #[test]
    fn test_parse_error_response_with_multiple_words() {
        let resp = Response::parse("ERROR Could not read file: /path/to/file.json").unwrap();
        assert_eq!(
            resp,
            Response::Error {
                message: "Could not read file: /path/to/file.json".to_string()
            }
        );
    }

    #[test]
    fn test_parse_empty_response() {
        let result = Response::parse("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty response");
    }

    #[test]
    fn test_parse_unknown_response() {
        let result = Response::parse("UNKNOWN");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown response"));
    }

    #[test]
    fn test_parse_response_with_whitespace() {
        let resp = Response::parse("  OK  ").unwrap();
        assert_eq!(resp, Response::Ok);
    }

    #[test]
    fn test_command_roundtrip_play() {
        let original = Command::PlayFile {
            path: "/test/path.json".to_string(),
        };
        let serialized = original.serialize();
        let parsed = Command::parse(serialized.trim()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_command_roundtrip_stop() {
        let original = Command::Stop;
        let serialized = original.serialize();
        let parsed = Command::parse(serialized.trim()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_command_roundtrip_shutdown() {
        let original = Command::Shutdown;
        let serialized = original.serialize();
        let parsed = Command::parse(serialized.trim()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_response_roundtrip_ok() {
        let original = Response::Ok;
        let serialized = original.serialize();
        let parsed = Response::parse(serialized.trim()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_response_roundtrip_error() {
        let original = Response::Error {
            message: "Test error message".to_string(),
        };
        let serialized = original.serialize();
        let parsed = Response::parse(serialized.trim()).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_is_json_string_valid() {
        assert!(Command::is_json_string(r#"{"key": "value"}"#));
        assert!(Command::is_json_string(r#"{ "key": "value" }"#));
        assert!(Command::is_json_string(
            r#"{"event_count": 1, "events": []}"#
        ));
    }

    #[test]
    fn test_is_json_string_with_whitespace() {
        assert!(Command::is_json_string(r#"  {"key": "value"}  "#));
    }

    #[test]
    fn test_is_json_string_invalid() {
        assert!(!Command::is_json_string("/path/to/file.json"));
        assert!(!Command::is_json_string("file.json"));
        assert!(!Command::is_json_string("{incomplete"));
        assert!(!Command::is_json_string("incomplete}"));
        assert!(!Command::is_json_string("[]"));
        assert!(!Command::is_json_string(""));
    }

    // Binary protocol tests
    #[test]
    fn test_binary_play_file_roundtrip() {
        let original = Command::PlayFile {
            path: "/test/path.json".to_string(),
        };
        let binary = original.to_binary().unwrap();
        let parsed = Command::from_binary(&binary).unwrap();
        assert_eq!(original, parsed);
    }

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
}
