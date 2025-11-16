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
use std::process::Command as ProcessCommand;
use std::thread;
use std::time::Duration;

/// Send JSON data directly to the server
///
/// This function sends JSON data directly via the binary protocol.
/// The protocol uses length-prefixed JSON for robust transmission.
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
    // Parse the JSON to validate it
    let json_value: serde_json::Value =
        serde_json::from_str(json_data).context("Failed to parse JSON data")?;

    let command = Command::PlayJson { data: json_value };
    send_command(command)
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
    send_command(Command::PlayFile {
        path: file_path.to_string(),
    })
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

    // Serialize command to binary format
    let binary_data = command
        .to_binary()
        .map_err(|e| anyhow::anyhow!("Failed to serialize command: {}", e))?;

    // Display command info
    match &command {
        Command::PlayJson { .. } => {
            eprintln!("⏳ サーバーにJSON直接送信中...");
        }
        Command::PlayFile { path } => {
            eprintln!("⏳ サーバーにJSONファイル経由送信中: {}", path);
        }
        Command::Stop => eprintln!("⏳ サーバーに停止要求を送信中..."),
        Command::Shutdown => eprintln!("⏳ サーバーにシャットダウン要求を送信中..."),
    }

    // Send command via binary protocol
    writer
        .write_binary(&binary_data)
        .context("Failed to send command to server")?;

    // Read binary response from server
    let response_data = writer
        .read_binary_response()
        .context("Failed to read response from server")?;

    // Parse binary response
    let response = Response::from_binary(&response_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse server response: {}", e))?;

    match response {
        Response::Ok => match &command {
            Command::PlayJson { .. } => {
                eprintln!("✅ JSON直接送信で演奏開始しました");
            }
            Command::PlayFile { path } => {
                eprintln!("✅ JSONファイル経由で演奏開始: {}", path);
            }
            Command::Stop => eprintln!("✅ 演奏停止しました"),
            Command::Shutdown => eprintln!("✅ サーバーをシャットダウンしました"),
        },
        Response::Error { message } => {
            eprintln!("❌ サーバーエラー: {}", message);
            return Err(anyhow::anyhow!("Server returned error: {}", message));
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
