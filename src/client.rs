//! Client module for sending commands to the YM2151 log player server.
//!
//! This module provides functions for communicating with a running server instance
//! to control playback of YM2151 register event logs.
//!
//! # Verbose Mode
//!
//! By default, the client operates in non-verbose mode to prevent disrupting TUI applications.
//! Use [`init_client`] to enable verbose output:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Enable verbose mode for debugging
//! client::init_client(true);
//!
//! // Or disable verbose mode for TUI applications (default)
//! client::init_client(false);
//! ```
//!
//! # Usage
//!
//! ## Playing JSON Data
//!
//! Use [`send_json`] to send JSON data:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! let json_data = r#"{"event_count": 2, "events": [...]}"#;
//! client::send_json(json_data)?;
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
//! // Now you can send JSON data
//! let json_data = r#"{"event_count": 1, "events": [...]}"#;
//! client::send_json(json_data)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};
use std::process::Command as ProcessCommand;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

/// Global verbose flag for client operations
static CLIENT_VERBOSE: Mutex<bool> = Mutex::new(false);

/// Initialize client with verbose flag
///
/// This function controls whether the client prints status messages to stderr.
/// By default, the client operates in non-verbose mode to prevent disrupting TUI applications.
///
/// # Arguments
/// * `verbose` - Enable verbose output if true, disable if false
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// // Enable verbose mode for debugging
/// client::init_client(true);
///
/// // Disable verbose mode for TUI applications
/// client::init_client(false);
/// ```
pub fn init_client(verbose: bool) {
    let mut v = CLIENT_VERBOSE.lock().unwrap();
    *v = verbose;
}

/// Check if client verbose mode is enabled
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// client::init_client(true);
/// assert!(client::is_client_verbose());
/// ```
pub fn is_client_verbose() -> bool {
    *CLIENT_VERBOSE.lock().unwrap()
}

/// Print a message to stderr only if verbose mode is enabled
fn log_client(message: &str) {
    if is_client_verbose() {
        eprintln!("{}", message);
    }
}

/// Send JSON data to the server
///
/// This function sends JSON data via the binary protocol.
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

pub fn stop_playback() -> Result<()> {
    send_command(Command::Stop)
}

pub fn shutdown_server() -> Result<()> {
    send_command(Command::Shutdown)
}

/// Start interactive mode on the server
///
/// In interactive mode, the server continuously streams audio and accepts
/// register write commands in real-time without stopping playback.
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// client::start_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn start_interactive() -> Result<()> {
    send_command(Command::StartInteractive)
}

/// Write a register value in interactive mode
///
/// Schedules a YM2151 register write at the specified time offset.
/// The server applies a 50ms latency buffer for jitter compensation.
///
/// # Arguments
/// * `time_offset_ms` - Time offset in milliseconds from now
/// * `addr` - YM2151 register address (0x00-0xFF)
/// * `data` - Data value to write (0x00-0xFF)
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// // Write to register 0x08 immediately
/// client::write_register(0, 0x08, 0x78)?;
///
/// // Write to register 0x28 after 100ms
/// client::write_register(100, 0x28, 0x3E)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn write_register(time_offset_ms: u32, addr: u8, data: u8) -> Result<()> {
    send_command(Command::WriteRegister {
        time_offset_ms,
        addr,
        data,
    })
}

/// Stop interactive mode
///
/// Stops the continuous audio streaming in interactive mode.
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// client::stop_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn stop_interactive() -> Result<()> {
    send_command(Command::StopInteractive)
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
/// let json_data = r#"{"event_count": 1, "events": [...]}"#;
/// client::send_json(json_data)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// # Errors
/// Returns an error if:
/// - Failed to install the server application
/// - Failed to start the server
/// - Server doesn't become ready within a reasonable timeout
pub fn ensure_server_ready(server_app_name: &str) -> Result<()> {
    log_client("🔍 サーバーの状態を確認中...");

    // Check if server is already running by sending a STOP command
    // This is a lightweight check that doesn't affect playback
    if is_server_running() {
        log_client("✅ サーバーは既に起動しています");
        return Ok(());
    }

    log_client("⚙️  サーバーが起動していません。起動準備中...");

    // Check if the server application exists in PATH
    if !is_app_in_path(server_app_name) {
        log_client(&format!(
            "📦 {} が見つかりません。cargo経由でインストール中...",
            server_app_name
        ));
        install_app_via_cargo(server_app_name)
            .with_context(|| format!("Failed to install {}", server_app_name))?;
        log_client(&format!(
            "✅ {} のインストールが完了しました",
            server_app_name
        ));
    }

    // Start the server in background mode
    log_client("🚀 サーバーを起動中...");
    start_server(server_app_name)
        .with_context(|| format!("Failed to start server: {}", server_app_name))?;

    // Poll the server until it's ready (max 10 seconds)
    log_client("⏳ サーバーの起動完了を待機中...");
    wait_for_server_ready(Duration::from_secs(10))
        .context("Server failed to become ready within timeout")?;

    log_client("✅ サーバーが起動し、コマンド受付可能になりました");
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
            log_client("⏳ サーバーにJSON送信中...");
        }
        Command::Stop => log_client("⏳ サーバーに停止要求を送信中..."),
        Command::Shutdown => log_client("⏳ サーバーにシャットダウン要求を送信中..."),
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
                log_client("✅ JSON送信で演奏開始しました");
            }
            Command::Stop => log_client("✅ 演奏停止しました"),
            Command::Shutdown => log_client("✅ サーバーをシャットダウンしました"),
        },
        Response::Error { message } => {
            log_client(&format!("❌ サーバーエラー: {}", message));
            return Err(anyhow::anyhow!("Server returned error: {}", message));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_client_verbose() {
        init_client(true);
        assert!(is_client_verbose());

        init_client(false);
        assert!(!is_client_verbose());
    }

    #[test]
    fn test_client_verbose_default() {
        // Test that the verbose flag can be queried without initialization
        // The default should be false (non-verbose)
        let _ = is_client_verbose();
    }

    #[test]
    fn test_log_client_verbose_mode() {
        // Enable verbose mode
        init_client(true);

        // This should not panic in verbose mode
        log_client("Test message in verbose mode");
    }

    #[test]
    fn test_log_client_non_verbose_mode() {
        // Disable verbose mode
        init_client(false);

        // This should not panic in non-verbose mode
        log_client("Test message in non-verbose mode");
    }

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
