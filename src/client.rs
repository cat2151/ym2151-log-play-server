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
//! ## Interactive Mode with JSON Data
//!
//! Use [`play_json_interactive`] to send ym2151log format JSON data to interactive mode.
//! This convenience function handles JSON parsing and register writes without managing
//! the interactive mode lifecycle, allowing continuous playback without audio gaps:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Start interactive mode once
//! client::start_interactive()?;
//!
//! // Send multiple JSONs without stopping - no audio gaps!
//! let json1 = r#"{"event_count": 2, "events": [
//!     {"time": 0, "addr": "0x08", "data": "0x00"},
//!     {"time": 100, "addr": "0x20", "data": "0xC7"}
//! ]}"#;
//! client::play_json_interactive(json1)?;
//!
//! let json2 = r#"{"event_count": 1, "events": [
//!     {"time": 200, "addr": "0x28", "data": "0x3E"}
//! ]}"#;
//! client::play_json_interactive(json2)?;
//!
//! // Stop when done
//! client::stop_interactive()?;
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
pub fn log_client(message: &str) {
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
    log_client("🎮 [インタラクティブモード] 開始要求を送信中...");
    log_client(&format!(
        "🔌 [デバッグ] パイプパス: {}",
        crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
    ));
    let result = send_command(Command::StartInteractive);
    if result.is_ok() {
        log_client("✅ [インタラクティブモード] 正常に開始しました");
    } else {
        log_client("❌ [インタラクティブモード] 開始に失敗しました");
    }
    result
}

/// Write a register value in interactive mode
///
/// Schedules a YM2151 register write at the specified time offset.
/// The server applies a 50ms latency buffer for jitter compensation.
///
/// # Arguments
/// * `time_offset_sec` - Time offset in seconds (f64) from now, providing sample-accurate precision
/// * `addr` - YM2151 register address (0x00-0xFF)
/// * `data` - Data value to write (0x00-0xFF)
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// // Write to register 0x08 immediately
/// client::write_register(0.0, 0x08, 0x78)?;
///
/// // Write to register 0x28 after 100ms (0.1 seconds)
/// client::write_register(0.1, 0x28, 0x3E)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn write_register(time_offset_sec: f64, addr: u8, data: u8) -> Result<()> {
    log_client(&format!(
        "📝 [レジスタ書き込み] offset={:.6}s, addr=0x{:02X}, data=0x{:02X}",
        time_offset_sec, addr, data
    ));
    send_command(Command::WriteRegister {
        time_offset_sec,
        addr,
        data,
    })
}

/// Get the current server time in seconds
///
/// Returns the current time in the server's time coordinate system (f64 seconds).
/// Clients can use this to synchronize with the server's timeline for precise scheduling.
/// This is equivalent to Web Audio's `currentTime` property.
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// let server_time = client::get_server_time()?;
/// println!("Server time: {:.6} seconds", server_time);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn get_server_time() -> Result<f64> {
    let mut writer = NamedPipe::connect_default()
        .context("Failed to connect to server. Is the server running?")?;

    let command = Command::GetServerTime;
    let binary_data = command
        .to_binary()
        .map_err(|e| anyhow::anyhow!("Failed to serialize command: {}", e))?;

    log_client("⏳ サーバー時刻を取得中...");

    writer
        .write_binary(&binary_data)
        .context("Failed to send command to server")?;

    let response_data = writer
        .read_binary_response()
        .context("Failed to read response from server")?;

    let response = Response::from_binary(&response_data)
        .map_err(|e| anyhow::anyhow!("Failed to parse server response: {}", e))?;

    match response {
        Response::ServerTime { time_sec } => {
            log_client(&format!("✅ サーバー時刻: {:.6} 秒", time_sec));
            Ok(time_sec)
        }
        Response::Error { message } => {
            log_client(&format!("❌ サーバーエラー: {}", message));
            Err(anyhow::anyhow!("Server returned error: {}", message))
        }
        _ => Err(anyhow::anyhow!(
            "Unexpected response type for GetServerTime"
        )),
    }
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
    log_client("⏹️  [インタラクティブモード] 停止要求を送信中...");
    let result = send_command(Command::StopInteractive);
    if result.is_ok() {
        log_client("✅ [インタラクティブモード] 正常に停止しました");
    }
    result
}

/// Clear all scheduled events in interactive mode
///
/// Removes all pending register write events from the server's schedule queue.
/// This allows seamless phrase transitions without audio gaps - you can cancel
/// phrase 1's scheduled events and immediately start phrase 2.
///
/// Note: Events that have already been processed (played) cannot be cleared.
/// Only future scheduled events are removed.
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// // Start interactive mode
/// client::start_interactive()?;
///
/// // Schedule some events for phrase 1
/// client::write_register(0.1, 0x08, 0x78)?;
/// client::write_register(0.2, 0x20, 0xC7)?;
///
/// // Cancel phrase 1 and switch to phrase 2 without audio gap
/// client::clear_schedule()?;
/// client::write_register(0.1, 0x28, 0x3E)?;
///
/// // Stop interactive mode when done
/// client::stop_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn clear_schedule() -> Result<()> {
    send_command(Command::ClearSchedule)
}

/// Send ym2151log format JSON data to interactive mode
///
/// This is a convenience function that accepts ym2151log format JSON data
/// and sends the register writes to the server in interactive mode. It handles:
/// - Parsing the JSON data
/// - Converting event timestamps to time offsets
/// - Sending register writes with proper timing
///
/// This function does NOT start or stop interactive mode - the client must
/// manage the interactive mode lifecycle using `start_interactive()` and
/// `stop_interactive()`. This allows sending multiple JSONs continuously
/// without audio gaps.
///
/// This eliminates the need for client applications to implement similar
/// processing logic repeatedly.
///
/// # Arguments
/// * `json_data` - JSON string in ym2151log format with events
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client;
/// // Start interactive mode once
/// client::start_interactive()?;
///
/// // Send multiple JSONs without stopping - no audio gaps!
/// let json1 = r#"{"event_count": 2, "events": [
///     {"time": 0, "addr": "0x08", "data": "0x00"},
///     {"time": 2797, "addr": "0x20", "data": "0xC7"}
/// ]}"#;
/// client::play_json_interactive(json1)?;
///
/// let json2 = r#"{"event_count": 1, "events": [
///     {"time": 5594, "addr": "0x28", "data": "0x3E"}
/// ]}"#;
/// client::play_json_interactive(json2)?;
///
/// // Stop interactive mode when done
/// client::stop_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// # Notes
/// - Events are scheduled with their original timing preserved
/// - Time values in the JSON are in YM2151 sample units (55930 Hz)
/// - Interactive mode must be started before calling this function
/// - Interactive mode must be stopped manually when done
pub fn play_json_interactive(json_data: &str) -> Result<()> {
    use crate::events::EventLog;
    use crate::resampler::OPM_SAMPLE_RATE;

    // Parse the JSON data
    let event_log = EventLog::from_json_str(json_data).context("Failed to parse JSON data")?;

    // Validate the event log
    if !event_log.validate() {
        return Err(anyhow::anyhow!("Invalid event log: validation failed"));
    }

    log_client(&format!(
        "📝 JSONから{}個のイベントを読み込みました",
        event_log.event_count
    ));

    // Convert events to time offsets and send them
    for event in &event_log.events {
        // Convert sample time to seconds (f64)
        // Event time is in samples at 55930 Hz
        let time_offset_sec = event.time as f64 / OPM_SAMPLE_RATE as f64;

        // Send register write
        write_register(time_offset_sec, event.addr, event.data).with_context(|| {
            format!(
                "Failed to write register 0x{:02X} = 0x{:02X} at {:.6}s",
                event.addr, event.data, time_offset_sec
            )
        })?;
    }

    log_client(&format!(
        "✅ {}個のレジスタ書き込みを送信しました",
        event_log.event_count
    ));

    Ok(())
}

/// Ensure the server is running and ready to accept commands
///
/// This function ensures that the YM2151 server is running and ready to accept
/// commands. It provides a seamless developer experience by automatically:
/// 1. Checking if the server is already running
/// 2. Locating the server executable (test binary, PATH, or install via cargo)
/// 3. Starting the server if not running
/// 4. Waiting until the server is ready to accept commands
///
/// # Test Context Support (Windows only)
/// When running in a test context (e.g., during `cargo test`), this function
/// automatically detects and uses the test-built binary from `target/debug` or
/// `target/debug/deps` instead of requiring the binary to be in PATH. This
/// enables seamless integration testing without manual setup.
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

    // Determine the server path based on context
    #[cfg(windows)]
    let server_path = {
        // First, try to find the binary in test context
        if let Some(test_binary) = get_test_binary_path(server_app_name) {
            log_client(&format!("🧪 テストコンテキストを検出: {:?}", test_binary));
            test_binary.to_string_lossy().to_string()
        } else if is_app_in_path(server_app_name) {
            // Use the app from PATH
            server_app_name.to_string()
        } else {
            // Not in test context and not in PATH, install it
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
            server_app_name.to_string()
        }
    };

    #[cfg(not(windows))]
    let server_path = {
        // On non-Windows platforms, use the original logic
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
        server_app_name.to_string()
    };

    // Start the server in background mode
    log_client("🚀 サーバーを起動中...");
    start_server(&server_path)
        .with_context(|| format!("Failed to start server: {}", server_app_name))?;

    // Poll the server until it's ready (max 10 seconds)
    log_client("⏳ サーバーの起動完了を待機中...");
    wait_for_server_ready(Duration::from_secs(10))
        .context("Server failed to become ready within timeout")?;

    log_client("✅ サーバーが起動し、コマンド受付可能になりました");
    Ok(())
}

/// Check if the server is currently running
pub fn is_server_running() -> bool {
    // Try to connect to the server
    // If successful, the server is running
    match NamedPipe::connect_default() {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Get the binary path when running in test context
///
/// During testing, the binary is located in target/debug or target/debug/deps,
/// not in PATH. This function locates the binary in the test build directory.
///
/// # Arguments
/// * `binary_name` - Name of the binary to find (e.g., "ym2151-log-play-server")
///
/// # Returns
/// * `Some(PathBuf)` - Path to the binary if found in test context
/// * `None` - Not in test context or binary not found
#[cfg(windows)]
fn get_test_binary_path(binary_name: &str) -> Option<std::path::PathBuf> {
    use std::path::PathBuf;

    // Try to get current exe path (works in test context)
    let current_exe = std::env::current_exe().ok()?;

    // Get the directory containing the test executable
    let mut path = current_exe.parent()?.to_path_buf();

    // In debug/test mode, we might be in deps directory
    if path.ends_with("deps") {
        path = path.parent()?.to_path_buf();
    }

    // Try with .exe extension (Windows)
    let exe_name = format!("{}.exe", binary_name);
    path.push(&exe_name);

    // Check if the binary exists
    if path.exists() {
        log_client(&format!("🔍 テストバイナリを検出: {:?}", path));
        return Some(path);
    }

    // Try without .exe extension
    path.pop();
    path.push(binary_name);
    if path.exists() {
        log_client(&format!("🔍 テストバイナリを検出: {:?}", path));
        return Some(path);
    }

    log_client(&format!(
        "⚠️  テストバイナリが見つかりません: {} (検索場所: {:?})",
        binary_name, path
    ));
    None
}

/// Check if an application is available in PATH
pub fn is_app_in_path(app_name: &str) -> bool {
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
///
/// # Arguments
/// * `server_path` - Path to the server executable (can be name in PATH or full path)
fn start_server(server_path: &str) -> Result<()> {
    ProcessCommand::new(server_path)
        .arg("server")
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

pub fn send_command(command: Command) -> Result<()> {
    log_client(&format!(
        "🔌 [デバッグ] パイプ接続を試行中: {}",
        crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
    ));

    let mut writer = NamedPipe::connect_default().context(
        r"Failed to connect to server. Is the server running? \
         サーバーが起動していることを確認してください。\
         \n💡 ヒント: 以下を確認してください:\
         \n  1. サーバーが起動しているか (ym2151-log-play-server server)\
         \n  2. パイプパスが正しいか (\\.\pipe\ym2151-log-play-server)\
         \n  3. 他のプロセスがパイプを使用していないか",
    )?;

    log_client("✅ [デバッグ] パイプ接続成功");

    // Serialize command to binary format
    let binary_data = command
        .to_binary()
        .map_err(|e| anyhow::anyhow!("Failed to serialize command: {}", e))?;

    log_client(&format!(
        "📤 [デバッグ] コマンドをバイナリ化しました ({}バイト)",
        binary_data.len()
    ));

    // Display command info
    match &command {
        Command::PlayJson { .. } => {
            log_client("⏳ サーバーにJSON送信中...");
        }
        Command::Stop => log_client("⏳ サーバーに停止要求を送信中..."),
        Command::Shutdown => log_client("⏳ サーバーにシャットダウン要求を送信中..."),
        Command::ClearSchedule => log_client("⏳ スケジュールクリア要求を送信中..."),
        Command::StartInteractive => log_client("⏳ インタラクティブモード開始要求を送信中..."),
        Command::StopInteractive => log_client("⏳ インタラクティブモード停止要求を送信中..."),
        Command::WriteRegister {
            time_offset_sec,
            addr,
            data,
        } => log_client(&format!(
            "⏳ レジスタ書き込み要求を送信中: offset={:.6}s, addr=0x{:02X}, data=0x{:02X}",
            time_offset_sec, addr, data
        )),
        _ => {}
    }

    // Send command via binary protocol
    writer
        .write_binary(&binary_data)
        .context("Failed to send command to server")?;

    log_client("✅ [デバッグ] コマンド送信完了");
    log_client("⏳ [デバッグ] サーバーからのレスポンス待機中...");

    // Read binary response from server
    let response_data = writer
        .read_binary_response()
        .context("Failed to read response from server")?;

    log_client(&format!(
        "✅ [デバッグ] レスポンス受信完了 ({}バイト)",
        response_data.len()
    ));

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
            Command::ClearSchedule => log_client("✅ スケジュールをクリアしました"),
            _ => {} // Other commands don't have custom success logging
        },
        Response::Error { message } => {
            log_client(&format!("❌ サーバーエラー: {}", message));
            return Err(anyhow::anyhow!("Server returned error: {}", message));
        }
        _ => {} // Handle other response types (like ServerTime) without error
    }

    Ok(())
}

// Test-only helper functions
#[cfg(all(test, windows))]
pub mod test_helpers {
    use super::*;

    /// Expose get_test_binary_path for testing
    pub fn get_test_binary_path_helper(binary_name: &str) -> Option<std::path::PathBuf> {
        super::get_test_binary_path(binary_name)
    }
}
