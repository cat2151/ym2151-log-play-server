use crate::ipc::protocol::Command;
use anyhow::{Context, Result};
use crate::ipc::pipe_windows::NamedPipe;

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

    eprintln!("✅ Command sent successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_play_file_with_server() {
        let result = play_file("sample_events.json");
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn test_stop_playback_with_server() {
        let result = stop_playback();
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn test_shutdown_server_with_server() {
        let result = shutdown_server();
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_command_without_server() {
        let result = send_command(Command::Stop);
        assert!(result.is_err());
    }
}
