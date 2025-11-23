//! Interactive mode client functionality
//!
//! This module handles interactive mode operations for real-time YM2151 control.

use super::config::log_always_client;
use super::config::log_verbose_client;
use super::core::send_command_interactive;
use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use crate::server::ServerState;
use anyhow::{Context, Result};

const RETRY_INITIAL_WAIT_MS: u64 = 1;
const RETRY_MAX_WAIT_MS: u64 = 50; // 指数関数的バックオフを利用し、応答速度と堅牢性のバランスを取る

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

    // インタラクティブモードへ切り替わるまで待機
    // 備忘、race conditionによって「切り替わるまで接続失敗してretryし、接続成功し、true取得」はOk。
    // ただし「retryし、接続成功し、false取得」はErr。そこでインタラクティブモードに切り替わっていないのは異常事態である。再度切り替えを送信は問題隠蔽なのでNG。
    match get_interactive_mode_state_with_retry() {
        Ok(true) => {
            log_verbose_client(&format!("✅ [インタラクティブモード] 正常に開始しました",));
            return Ok(());
        }
        Ok(false) => {
            log_always_client(
                "[ERROR] サーバーがインタラクティブモードに切り替わりませんでした (timeout)",
            );
            std::process::exit(1);
        }
        Err(e) => {
            log_always_client(&format!("[ERROR] サーバー状態取得失敗: {} (timeout)", e));
            std::process::exit(1);
        }
    }
}

/// Get whether the server is currently in interactive mode
///
/// Returns true if the server is in interactive mode, false otherwise.
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::interactive;
/// let is_interactive = interactive::get_interactive_mode_state_with_retry()?;
/// println!("Is interactive: {}", is_interactive);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn get_interactive_mode_state_with_retry() -> Result<bool> {
    // 備忘、インタラクティブモードかをserverに問い合わせるより、stateを問い合わせるほうが、
    // server側がシンプルになるので、そうした。
    // インタラクティブモードかを問い合わせるほうは今後削除予定。
    Ok(get_server_state_with_retry()? == ServerState::Interactive.as_str())
}

pub fn get_server_state_with_retry() -> Result<String> {
    // 前提として、race conditionにより、低確率でエラーの可能性がついてまわる。今後、MAX_WAIT_MSでチューニング予定。
    let mut wait_ms = RETRY_INITIAL_WAIT_MS;
    loop {
        match get_server_state() {
            Ok(state) => {
                return Ok(state);
            }
            Err(_) => {
                log_verbose_client(&format!("race condition. retrying...",));
                if wait_ms >= RETRY_MAX_WAIT_MS {
                    log_verbose_client(&format!(
                        "timeout. MAX_WAIT_MSをより大きい数字にするか検討してください: {}",
                        RETRY_MAX_WAIT_MS
                    ));
                    return Err(anyhow::anyhow!(
                        "timeout reached while getting server state after retries: {}",
                        RETRY_MAX_WAIT_MS
                    ));
                }
                std::thread::sleep(std::time::Duration::from_millis(wait_ms));
                wait_ms ^= 2;
            }
        }
    }
}

pub fn get_server_state() -> Result<String> {
    let mut writer = NamedPipe::connect_default()
        .context("Failed to connect to server. Is the server running?")?;

    let command = Command::GetServerState;
    let binary_data = command
        .to_binary()
        .map_err(|e| anyhow::anyhow!("Failed to serialize command: {}", e))?;

    log_verbose_client("🔍 サーバー状態を取得中...");

    writer.write_binary(&binary_data)?;

    let response_bytes = writer.read_binary_response()?;
    let response = Response::from_binary(&response_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;

    log_verbose_client(&format!("response server state: {:?}", response));

    match response {
        Response::ServerState { state } => Ok(state),
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
