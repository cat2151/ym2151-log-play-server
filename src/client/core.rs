//! Core client communication module
//!
//! This module provides basic client-server communication functionality.

use super::config::log_verbose_client;
use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};
use std::thread;
use std::time::Duration;

/// Initial delay for exponential backoff (ms)
const RETRY_INITIAL_DELAY_MS: u64 = 1;
/// Maximum delay for exponential backoff (ms)
const RETRY_MAX_DELAY_MS: u64 = 50;

/// Send a standard command to the server
pub fn send_command(command: Command) -> Result<()> {
    send_command_internal(command, false)
}

/// Send command specifically for interactive mode (includes [インタラクティブ] tag in debug messages)
pub fn send_command_interactive(command: Command) -> Result<()> {
    send_command_internal(command, true)
}

fn send_command_internal(command: Command, is_interactive: bool) -> Result<()> {
    let debug_tag = if is_interactive {
        "[インタラクティブ]"
    } else {
        ""
    };

    // Retry loop for connection (exponential backoff)
    let mut last_error = None;
    let mut delay = RETRY_INITIAL_DELAY_MS;
    loop {
        if delay != RETRY_INITIAL_DELAY_MS {
            log_verbose_client(&format!("🔄 {} 再試行...", debug_tag));
            log_verbose_client(&format!("⏳ {} バックオフ待機: {}ms", debug_tag, delay));
            thread::sleep(Duration::from_millis(delay));
            delay ^= 2;
        }
        if delay > RETRY_MAX_DELAY_MS {
            log_verbose_client(&format!(
                "⚠️  {} 最大バックオフ時間に到達しました",
                debug_tag
            ));
            break;
        }

        log_verbose_client(&format!(
            "🔌 {} パイプ接続を試行中: {}",
            debug_tag,
            crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
        ));

        let mut writer = match NamedPipe::connect_default() {
            Ok(w) => {
                log_verbose_client(&format!("✅ {} パイプ接続成功", debug_tag));
                w
            }
            Err(e) => {
                log_verbose_client(&format!("⚠️  {} パイプ接続失敗: {}", debug_tag, e));
                last_error = Some(e);
                continue; // Retry
            }
        };

        // Connection successful, proceed with command
        // Serialize command to binary format
        let binary_data = command
            .to_binary()
            .map_err(|e| anyhow::anyhow!("Failed to serialize command: {}", e))?;

        log_verbose_client(&format!(
            "📤 {} コマンドをバイナリ化しました ({}バイト)",
            debug_tag,
            binary_data.len()
        ));

        // Display command info
        match &command {
            Command::PlayJson { .. } => {
                log_verbose_client("⏳ サーバーにJSON送信中...");
            }
            Command::PlayJsonInInteractive { .. } => {
                log_verbose_client("⏳ インタラクティブモードにJSON送信中...");
            }
            Command::Stop => log_verbose_client("⏳ サーバーに停止要求を送信中..."),
            Command::Shutdown => log_verbose_client("⏳ サーバーにシャットダウン要求を送信中..."),
            Command::ClearSchedule => log_verbose_client("⏳ スケジュールクリア要求を送信中..."),
            Command::StartInteractive => {
                log_verbose_client("⏳ インタラクティブモード開始要求を送信中...")
            }
            Command::StopInteractive => {
                log_verbose_client("⏳ インタラクティブモード停止要求を送信中...")
            }
            _ => {}
        }

        // Send command via binary protocol
        if let Err(e) = writer.write_binary(&binary_data) {
            log_verbose_client(&format!("⚠️  {} コマンド送信失敗: {}", debug_tag, e));
            last_error = Some(e);
            continue; // Retry
        }

        log_verbose_client(&format!("✅ {} コマンド送信完了", debug_tag));
        log_verbose_client(&format!(
            "⏳ {} サーバーからのレスポンス待機中...",
            debug_tag
        ));

        // Read binary response from server
        let response_data = match writer.read_binary_response() {
            Ok(data) => data,
            Err(e) => {
                log_verbose_client(&format!("⚠️  {} レスポンス読み取り失敗: {}", debug_tag, e));
                last_error = Some(e);
                continue; // Retry
            }
        };

        log_verbose_client(&format!(
            "✅ {} レスポンス受信完了 ({}バイト)",
            debug_tag,
            response_data.len()
        ));

        // Parse binary response
        let response = Response::from_binary(&response_data)
            .map_err(|e| anyhow::anyhow!("Failed to parse server response: {}", e))?;

        match response {
            Response::Ok => match &command {
                Command::PlayJson { .. } => {
                    log_verbose_client("✅ JSON送信で演奏開始しました");
                }
                Command::PlayJsonInInteractive { .. } => {
                    log_verbose_client("✅ インタラクティブモードでJSON処理完了");
                }
                Command::Stop => log_verbose_client("✅ 演奏停止しました"),
                Command::Shutdown => log_verbose_client("✅ サーバーをシャットダウンしました"),
                Command::ClearSchedule => log_verbose_client("✅ スケジュールをクリアしました"),
                _ => {} // Other commands don't have custom success logging
            },
            Response::Error { message } => {
                log_verbose_client(&format!("❌ サーバーエラー: {}", message));
                return Err(anyhow::anyhow!("Server returned error: {}", message));
            }
            _ => {} // Handle other response types (like ServerTime) without error
        }

        return Ok(()); // Success
    }

    // All retries failed
    Err(last_error.unwrap_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to connect to server after all retries",
        )
    }))
    .context(
        r"Failed to connect to server. Is the server running? \
         サーバーが起動していることを確認してください。\
         \n💡 ヒント: 以下を確認してください:\
         \n  1. サーバーが起動しているか (ym2151-log-play-server server)\
         \n  2. パイプパスが正しいか (\\.\pipe\ym2151-log-play-server)\
         \n  3. 他のプロセスがパイプを使用していないか",
    )
}

/// Basic playback control functions
pub fn stop_playback() -> Result<()> {
    send_command(Command::Stop)
}

pub fn shutdown_server() -> Result<()> {
    send_command(Command::Shutdown)
}
