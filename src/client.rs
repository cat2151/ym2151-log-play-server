use crate::ipc::pipe_windows::NamedPipe;
use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};

pub fn play_file(json_path: &str) -> Result<()> {
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
        Command::Play(path) => eprintln!("⏳ サーバーに演奏要求を送信中: {}", path),
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
            Command::Play(path) => eprintln!("✅ 演奏開始: {}", path),
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
