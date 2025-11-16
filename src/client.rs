use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};

/// Send JSON data directly via named pipe (max ~4KB)
/// Use this for small JSON data that fits in pipe buffer
pub fn send_json_direct(json_data: &str) -> Result<()> {
    send_command(Command::Play(json_data.to_string()))
}

/// Send JSON data via file path (unlimited size)
/// Use this for large JSON files that exceed pipe buffer
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
}
