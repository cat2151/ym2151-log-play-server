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


