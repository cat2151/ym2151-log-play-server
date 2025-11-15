#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Play(String),

    Stop,

    Shutdown,
}

impl Command {
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
                    Ok(Command::Play(parts[1].to_string()))
                }
            }
            "STOP" => Ok(Command::Stop),
            "SHUTDOWN" => Ok(Command::Shutdown),
            _ => Err(format!("Unknown command: {}", parts[0])),
        }
    }

    pub fn serialize(&self) -> String {
        match self {
            Command::Play(path) => format!("PLAY {}\n", path),
            Command::Stop => "STOP\n".to_string(),
            Command::Shutdown => "SHUTDOWN\n".to_string(),
        }
    }

    /// Check if a string appears to be JSON data (starts with '{', ends with '}')
    pub fn is_json_string(s: &str) -> bool {
        let trimmed = s.trim();
        trimmed.starts_with('{') && trimmed.ends_with('}')
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    Ok,

    Error(String),
}

impl Response {
    pub fn serialize(&self) -> String {
        match self {
            Response::Ok => "OK\n".to_string(),
            Response::Error(msg) => format!("ERROR {}\n", msg),
        }
    }

    pub fn parse(line: &str) -> Result<Self, String> {
        let line = line.trim();

        if line.is_empty() {
            return Err("Empty response".to_string());
        }

        if line == "OK" {
            return Ok(Response::Ok);
        }

        if let Some(msg) = line.strip_prefix("ERROR ") {
            return Ok(Response::Error(msg.to_string()));
        }

        Err(format!("Unknown response: {}", line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_play_command() {
        let cmd = Command::parse("PLAY /path/to/file.json").unwrap();
        assert_eq!(cmd, Command::Play("/path/to/file.json".to_string()));
    }

    #[test]
    fn test_parse_play_command_with_spaces() {
        let cmd = Command::parse("PLAY /path/with spaces/file.json").unwrap();
        assert_eq!(
            cmd,
            Command::Play("/path/with spaces/file.json".to_string())
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
        let cmd = Command::Play("/path/to/file.json".to_string());
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
        let resp = Response::Error("File not found".to_string());
        assert_eq!(resp.serialize(), "ERROR File not found\n");
    }

    #[test]
    fn test_serialize_error_response_with_special_chars() {
        let resp = Response::Error("Path: /invalid/path".to_string());
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
        assert_eq!(resp, Response::Error("File not found".to_string()));
    }

    #[test]
    fn test_parse_error_response_with_multiple_words() {
        let resp = Response::parse("ERROR Could not read file: /path/to/file.json").unwrap();
        assert_eq!(
            resp,
            Response::Error("Could not read file: /path/to/file.json".to_string())
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
        let original = Command::Play("/test/path.json".to_string());
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
        let original = Response::Error("Test error message".to_string());
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

    #[test]
    fn test_parse_play_command_with_json_string() {
        let json = r#"{"event_count": 1, "events": []}"#;
        let cmd = Command::parse(&format!("PLAY {}", json)).unwrap();
        if let Command::Play(data) = &cmd {
            assert!(Command::is_json_string(data));
        } else {
            panic!("Expected Play command");
        }
    }
}
