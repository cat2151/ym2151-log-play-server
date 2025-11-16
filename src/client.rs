//! Client module for sending commands to the YM2151 log player server.
//!
//! This module provides functions for communicating with a running server instance
//! to control playback of YM2151 register event logs.
//!
//! # Usage
//!
//! The recommended way to send JSON data is using the [`send_json`] function,
//! which automatically chooses the optimal transmission method based on data size:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Small JSON (< 4KB) - automatically sent directly via named pipe
//! let small_json = r#"{"event_count": 2, "events": [...]}"#;
//! client::send_json(small_json)?;
//!
//! // Large JSON (> 4KB) - automatically saved to temp file and sent via file path
//! let large_json = /* ... large JSON data ... */;
//! client::send_json(&large_json)?;
//!
//! // Control playback
//! client::stop_playback()?;
//! client::shutdown_server()?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! # Advanced Usage
//!
//! For explicit control over transmission method:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Force direct transmission (for small JSON < 4KB)
//! client::send_json_direct(r#"{"event_count": 1, "events": []}"#)?;
//!
//! // Force file-based transmission (for any size)
//! client::send_json_via_file("path/to/file.json")?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;

/// Maximum size for direct JSON transmission via named pipe (in bytes)
/// This corresponds to the Windows named pipe buffer size
const MAX_DIRECT_JSON_SIZE: usize = 4096;

/// Send JSON data automatically choosing the best method based on size
///
/// This function automatically determines whether to send JSON data directly
/// via named pipe or through a temporary file based on the data size.
///
/// - For JSON strings ≤ 4KB: sends directly via named pipe
/// - For JSON strings > 4KB: writes to a temporary file and sends the file path
///
/// # Arguments
/// * `json_data` - JSON string data to send
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// let json = r#"{"event_count": 1, "events": []}"#;
/// client::send_json(json).unwrap();
/// ```
pub fn send_json(json_data: &str) -> Result<()> {
    let json_bytes = json_data.as_bytes();

    if json_bytes.len() <= MAX_DIRECT_JSON_SIZE {
        // Small JSON: send directly via named pipe
        send_json_direct(json_data)
    } else {
        // Large JSON: write to temporary file and send file path
        let temp_path = std::env::temp_dir().join("ym2151_temp.json");

        // Write JSON data to temporary file
        let mut file =
            fs::File::create(&temp_path).context("Failed to create temporary JSON file")?;
        file.write_all(json_bytes)
            .context("Failed to write JSON data to temporary file")?;
        file.flush()
            .context("Failed to flush temporary JSON file")?;

        // Send the file path
        let result = send_json_via_file(
            temp_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid temporary file path"))?,
        );

        // Clean up temporary file
        let _ = fs::remove_file(&temp_path);

        result
    }
}

/// Send JSON data directly via named pipe (max ~4KB)
/// Use this for small JSON data that fits in pipe buffer
///
/// Note: Consider using `send_json()` instead, which automatically
/// chooses the best method based on data size.
pub fn send_json_direct(json_data: &str) -> Result<()> {
    send_command(Command::Play(json_data.to_string()))
}

/// Send JSON data via file path (unlimited size)
/// Use this for large JSON files that exceed pipe buffer
///
/// Note: Consider using `send_json()` instead, which automatically
/// chooses the best method based on data size.
pub fn send_json_via_file(json_path: &str) -> Result<()> {
    send_command(Command::Play(json_path.to_string()))
}

pub fn stop_playback() -> Result<()> {
    send_command(Command::Stop)
}

pub fn shutdown_server() -> Result<()> {
    send_command(Command::Shutdown)
}

fn send_command(command: Command) -> Result<()> {
    let mut writer = NamedPipe::connect_default()
        .context("Failed to connect to server. Is the server running?")?;

    let message = command.serialize();

    // コマンドの内容を表示
    match &command {
        Command::Play(data) => {
            if Command::is_json_string(data) {
                eprintln!("⏳ サーバーにJSON直接送信中...");
            } else {
                eprintln!("⏳ サーバーにJSONファイル経由送信中: {}", data);
            }
        }
        Command::Stop => eprintln!("⏳ サーバーに停止要求を送信中..."),
        Command::Shutdown => eprintln!("⏳ サーバーにシャットダウン要求を送信中..."),
    }

    writer
        .write_str(&message)
        .context("Failed to send command to server")?;

    // サーバーからのレスポンスを読み取り
    let response_line = writer
        .read_response()
        .context("Failed to read response from server")?;

    let response = Response::parse(response_line.trim())
        .map_err(|e| anyhow::anyhow!("Failed to parse server response: {}", e))?;

    match response {
        Response::Ok => match &command {
            Command::Play(data) => {
                if Command::is_json_string(data) {
                    eprintln!("✅ JSON直接送信で演奏開始しました");
                } else {
                    eprintln!("✅ JSONファイル経由で演奏開始: {}", data);
                }
            }
            Command::Stop => eprintln!("✅ 演奏停止しました"),
            Command::Shutdown => eprintln!("✅ サーバーをシャットダウンしました"),
        },
        Response::Error(msg) => {
            eprintln!("❌ サーバーエラー: {}", msg);
            return Err(anyhow::anyhow!("Server returned error: {}", msg));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_command_without_server() {
        let result = send_command(Command::Stop);
        assert!(result.is_err());
    }

    #[test]
    fn test_max_direct_json_size_constant() {
        // Verify the constant is set correctly
        assert_eq!(MAX_DIRECT_JSON_SIZE, 4096);
    }

    #[test]
    fn test_small_json_size_check() {
        // Small JSON should be under the threshold
        let small_json = r#"{"event_count": 1, "events": []}"#;
        assert!(small_json.as_bytes().len() <= MAX_DIRECT_JSON_SIZE);
    }

    #[test]
    fn test_large_json_size_check() {
        // Generate a large JSON that exceeds the threshold
        let mut large_json = String::from(r#"{"event_count": 500, "events": ["#);
        for i in 0..500 {
            if i > 0 {
                large_json.push_str(", ");
            }
            large_json.push_str(&format!(
                r#"{{"time": {}, "addr": "0x08", "data": "0x00"}}"#,
                i
            ));
        }
        large_json.push_str("]}");
        assert!(large_json.as_bytes().len() > MAX_DIRECT_JSON_SIZE);
    }
}
