//! Client module for sending commands to the YM2151 log player server.
//!
//! This module provides functions for communicating with a running server instance
//! to control playback of YM2151 register event logs.
//!
//! # Usage
//!
//! ## Playing JSON Data
//!
//! Use [`send_json`] to send JSON data. The function automatically chooses
//! the optimal transmission method based on data size:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Automatically handles small and large JSON
//! let json_data = r#"{"event_count": 2, "events": [...]}"#;
//! client::send_json(json_data)?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Playing from File
//!
//! Use [`play_file`] to play a JSON file:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! client::play_file("path/to/music.json")?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Controlling Playback
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Stop playback
//! client::stop_playback()?;
//!
//! // Shutdown server
//! client::shutdown_server()?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Ensuring Server is Ready
//!
//! Use [`ensure_server_ready`] to automatically ensure the server is running and ready:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Ensure server is ready (installs and starts if needed)
//! client::ensure_server_ready("cat-play-mml")?;
//!
//! // Now you can play files
//! client::play_file("music.json")?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::process::Command as ProcessCommand;
use std::thread;
use std::time::Duration;

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
        let result = play_file(
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
/// Internal function used by send_json for small JSON data
fn send_json_direct(json_data: &str) -> Result<()> {
    send_command(Command::Play(json_data.to_string()))
}

/// Play a JSON file by sending its file path to the server
///
/// The server will read and play the JSON file at the specified path.
/// This is useful when you already have a JSON file on disk.
///
/// # Arguments
/// * `file_path` - Path to the JSON file to play
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// client::play_file("output_ym2151.json")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn play_file(file_path: &str) -> Result<()> {
    send_command(Command::Play(file_path.to_string()))
}

pub fn stop_playback() -> Result<()> {
    send_command(Command::Stop)
}

pub fn shutdown_server() -> Result<()> {
    send_command(Command::Shutdown)
}

/// Ensure the server is running and ready to accept commands
///
/// This function ensures that the YM2151 server is running and ready to accept
/// commands. It provides a seamless developer experience by automatically:
/// 1. Checking if the server is already running
/// 2. Installing the server application if not found in PATH
/// 3. Starting the server if not running
/// 4. Waiting until the server is ready to accept commands
///
/// # Arguments
/// * `server_app_name` - Name of the server application (e.g., "cat-play-mml")
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// // Ensure server is ready before playing music
/// client::ensure_server_ready("cat-play-mml")?;
///
/// // Now the server is guaranteed to be running and ready
/// client::play_file("music.json")?;
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// # Errors
/// Returns an error if:
/// - Failed to install the server application
/// - Failed to start the server
/// - Server doesn't become ready within a reasonable timeout
pub fn ensure_server_ready(server_app_name: &str) -> Result<()> {
    eprintln!("🔍 サーバーの状態を確認中...");

    // Check if server is already running by sending a STOP command
    // This is a lightweight check that doesn't affect playback
    if is_server_running() {
        eprintln!("✅ サーバーは既に起動しています");
        return Ok(());
    }

    eprintln!("⚙️  サーバーが起動していません。起動準備中...");

    // Check if the server application exists in PATH
    if !is_app_in_path(server_app_name) {
        eprintln!(
            "📦 {} が見つかりません。cargo経由でインストール中...",
            server_app_name
        );
        install_app_via_cargo(server_app_name)
            .with_context(|| format!("Failed to install {}", server_app_name))?;
        eprintln!("✅ {} のインストールが完了しました", server_app_name);
    }

    // Start the server in background mode
    eprintln!("🚀 サーバーを起動中...");
    start_server(server_app_name)
        .with_context(|| format!("Failed to start server: {}", server_app_name))?;

    // Poll the server until it's ready (max 10 seconds)
    eprintln!("⏳ サーバーの起動完了を待機中...");
    wait_for_server_ready(Duration::from_secs(10))
        .context("Server failed to become ready within timeout")?;

    eprintln!("✅ サーバーが起動し、コマンド受付可能になりました");
    Ok(())
}

/// Check if the server is currently running
fn is_server_running() -> bool {
    // Try to connect to the server
    // If successful, the server is running
    match NamedPipe::connect_default() {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Check if an application is available in PATH
fn is_app_in_path(app_name: &str) -> bool {
    which::which(app_name).is_ok()
}

/// Install an application via cargo
fn install_app_via_cargo(app_name: &str) -> Result<()> {
    let output = ProcessCommand::new("cargo")
        .args([
            "install",
            "--git",
            &format!("https://github.com/cat2151/{}", app_name),
        ])
        .output()
        .context("Failed to execute cargo install")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("cargo install failed: {}", stderr));
    }

    Ok(())
}

/// Start the server application in background mode
fn start_server(server_app_name: &str) -> Result<()> {
    ProcessCommand::new(server_app_name)
        .arg("--server")
        .spawn()
        .context("Failed to spawn server process")?;

    Ok(())
}

/// Wait for the server to become ready by polling with STOP commands
fn wait_for_server_ready(timeout: Duration) -> Result<()> {
    let start_time = std::time::Instant::now();
    let poll_interval = Duration::from_millis(100);

    loop {
        if start_time.elapsed() > timeout {
            return Err(anyhow::anyhow!(
                "Timeout waiting for server to become ready"
            ));
        }

        // Try to send a STOP command
        // If successful, the server is ready
        if is_server_running() {
            // Give the server a moment to fully initialize
            thread::sleep(Duration::from_millis(50));
            return Ok(());
        }

        thread::sleep(poll_interval);
    }
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

    #[test]
    fn test_is_server_running_when_not_running() {
        // When server is not running, should return false
        let result = is_server_running();
        // On Linux this will be false since we can't test Windows named pipes
        // On Windows without server, this should also be false
        assert!(!result || cfg!(windows));
    }

    #[test]
    fn test_is_app_in_path() {
        // Test with a command that should always exist
        assert!(is_app_in_path("cargo"));

        // Test with a command that likely doesn't exist
        assert!(!is_app_in_path("this-command-should-not-exist-xyz123"));
    }
}
