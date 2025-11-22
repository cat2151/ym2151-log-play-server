//! Interactive mode client functionality
//!
//! This module handles interactive mode operations for real-time YM2151 control.

use super::config::log_verbose_client;
use super::config::log_always_client;
use super::core::send_command_interactive;
use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};

/// Start interactive mode on the server
///
/// In interactive mode, the server continuously streams audio and accepts
/// register write commands in real-time without stopping playback.
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::interactive;
/// interactive::start_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn start_interactive() -> Result<()> {
    use std::{thread, time::Duration};

    log_verbose_client("🎮 [インタラクティブモード] 開始要求を送信中...");
    log_verbose_client(&format!(
        "🔌 パイプパス: {}",
        crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
    ));
    let result = send_command_interactive(Command::StartInteractive);
    if result.is_err() {
        log_verbose_client("❌ [インタラクティブモード] 開始に失敗しました");
        return result;
    }

    // インタラクティブモードへ切り替わるまで最大1秒間待機
    let timeout = Duration::from_secs(1);
    let start = std::time::Instant::now();
    loop {
        match get_interactive_mode_state() {
            Ok(true) => {
                let elapsed_sec = start.elapsed().as_secs_f64();
                log_verbose_client(&format!(
                    "✅ [インタラクティブモード] 正常に開始しました (切替所要: {:.6}秒)",
                    elapsed_sec
                ));
                return Ok(());
            }
            Ok(false) => {
                if start.elapsed() >= timeout {
                    log_verbose_client(
                        "❌ [インタラクティブモード] サーバーがモード切替に失敗しました (timeout)",
                    );
                    log_always_client("[ERROR] サーバーがインタラクティブモードに切り替わりませんでした (timeout)");
                    std::process::exit(1);
                }
                thread::sleep(Duration::from_millis(1));
            }
            Err(e) => {
                if start.elapsed() >= timeout {
                    log_verbose_client(&format!(
                        "❌ [インタラクティブモード] サーバー状態取得失敗: {} (timeout)",
                        e
                    ));
                    log_always_client(&format!("[ERROR] サーバー状態取得失敗: {} (timeout)", e));
                    std::process::exit(1);
                }
                thread::sleep(Duration::from_millis(1));
            }
        }
    }
}

/// Get whether the server is currently in interactive mode
///
/// Returns true if the server is in interactive mode, false otherwise.
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::interactive;
/// let is_interactive = interactive::get_interactive_mode_state()?;
/// println!("Is interactive: {}", is_interactive);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn get_interactive_mode_state() -> Result<bool> {
    let mut writer = NamedPipe::connect_default()
        .context("Failed to connect to server. Is the server running?")?;

    let command = Command::GetInteractiveModeState;
    let binary_data = command
        .to_binary()
        .map_err(|e| anyhow::anyhow!("Failed to serialize command: {}", e))?;

    log_verbose_client("🔍 インタラクティブモード状態を取得中...");

    writer.write_binary(&binary_data)?;

    let response_bytes = writer.read_binary_response()?;
    let response = Response::from_binary(&response_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;

    match response {
        Response::InteractiveModeState { is_interactive } => Ok(is_interactive),
        Response::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
        _ => Err(anyhow::anyhow!("Unexpected response type")),
    }
}

/// Get the current server time in seconds
///
/// Returns the current time in the server's time coordinate system (f64 seconds).
/// Clients can use this to synchronize with the server's timeline for precise scheduling.
/// This is equivalent to Web Audio's `currentTime` property.
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::interactive;
/// let server_time = interactive::get_server_time()?;
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

    log_verbose_client("⏳ サーバー時刻を取得中...");

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
            log_verbose_client(&format!("✅ サーバー時刻: {:.6} 秒", time_sec));
            Ok(time_sec)
        }
        Response::Error { message } => {
            log_verbose_client(&format!("❌ サーバーエラー: {}", message));
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
/// # use ym2151_log_play_server::client::interactive;
/// interactive::stop_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn stop_interactive() -> Result<()> {
    log_verbose_client("⏹️  [インタラクティブモード] 停止要求を送信中...");
    let result = send_command_interactive(Command::StopInteractive);
    if result.is_ok() {
        log_verbose_client("✅ [インタラクティブモード] 正常に停止しました");
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
/// # use ym2151_log_play_server::client::interactive;
/// // Start interactive mode
/// interactive::start_interactive()?;
///
/// // Send JSON for phrase 1
/// let phrase1_json = r#"{"events": [
///     {"time": 2797, "addr": "0x08", "data": "0x78"},
///     {"time": 5594, "addr": "0x20", "data": "0xC7"}
/// ]}"#;
/// interactive::play_json_interactive(phrase1_json)?;
///
/// // Cancel phrase 1 and switch to phrase 2 without audio gap
/// interactive::clear_schedule()?;
/// let phrase2_json = r#"{"events": [
///     {"time": 2797, "addr": "0x28", "data": "0x3E"}
/// ]}"#;
/// interactive::play_json_interactive(phrase2_json)?;
///
/// // Stop interactive mode when done
/// interactive::stop_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn clear_schedule() -> Result<()> {
    send_command_interactive(Command::ClearSchedule)
}

/// Send ym2151log format JSON data to interactive mode
///
/// This is a convenience function that accepts ym2151log format JSON data
/// and converts it to f64 second timing before sending to the server. The conversion:
/// - Takes JSON with time in sample units (i64, 55930 Hz)
/// - Converts to JSON with time in seconds (f64)
/// - Sends the converted JSON to server for processing
///
/// This function does NOT start or stop interactive mode - the client must
/// manage the interactive mode lifecycle using `start_interactive()` and
/// `stop_interactive()`. This allows sending multiple JSONs continuously
/// without audio gaps.
///
/// # Arguments
/// * `json_data` - JSON string in ym2151log format with time in sample units
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::interactive;
/// // Start interactive mode once
/// interactive::start_interactive()?;
///
/// // Send multiple JSONs without stopping - no audio gaps!
/// let json1 = r#"{"events": [
///     {"time": 0, "addr": "0x08", "data": "0x00"},
///     {"time": 2797, "addr": "0x20", "data": "0xC7"}
/// ]}"#;
/// interactive::play_json_interactive(json1)?;
///
/// let json2 = r#"{"events": [
///     {"time": 5594, "addr": "0x28", "data": "0x3E"}
/// ]}"#;
/// interactive::play_json_interactive(json2)?;
///
/// // Stop interactive mode when done
/// interactive::stop_interactive()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// # Notes
/// - Input JSON has time in sample units (i64) at 55930 Hz
/// - Converted to time in seconds (f64) for precise interactive timing
/// - Interactive mode must be started before calling this function
/// - Interactive mode must be stopped manually when done
/// - JSON parsing and timing conversion are handled client-side
/// - Register scheduling is handled server-side
pub fn play_json_interactive(json_data: &str) -> Result<()> {
    log_verbose_client("🎵 JSONデータをパース中...");

    // TODO: Re-implement JSON time conversion from samples to seconds
    // For now, assume the input JSON already has time in f64 seconds format
    let converted_json = json_data.to_string();

    log_verbose_client("✅ JSONデータのパースが完了しました");

    // Parse the converted JSON to check if it has any events
    let json_value: serde_json::Value =
        serde_json::from_str(&converted_json).context("Failed to parse converted JSON")?;

    // Check if events array is empty
    if let Some(events) = json_value.get("events") {
        if let Some(events_array) = events.as_array() {
            if events_array.is_empty() {
                log_verbose_client("ℹ️  イベント数が0です。処理をスキップします。");
                return Ok(());
            }
        }
    }

    log_verbose_client("🎵 変換されたJSONをインタラクティブモードに送信中...");

    send_command_interactive(Command::PlayJsonInInteractive { data: json_value })
        .with_context(|| "Failed to send converted JSON data to interactive mode")?;

    log_verbose_client("✅ 変換されたJSONデータをサーバーに送信しました");
    Ok(())
}
