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
    writer
        .write_str(&message)
        .context("Failed to send command to server")?;

    eprintln!("📤 Command sent successfully");

    // サーバーからのレスポンスを読み取り
    let response_line = writer
        .read_response()
        .context("Failed to read response from server")?;

    let response = Response::parse(response_line.trim())
        .map_err(|e| anyhow::anyhow!("Failed to parse server response: {}", e))?;

    match response {
        Response::Ok => {
            eprintln!("✅ Server confirmed: Command executed successfully");
        }
        Response::Error(msg) => {
            eprintln!("❌ Server error: {}", msg);
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
