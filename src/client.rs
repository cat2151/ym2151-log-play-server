//! Client module for communicating with the YM2151 server
//!
//! This module provides functions for clients to send commands to the
//! YM2151 server via named pipes.

use crate::ipc::protocol::Command;
use anyhow::{Context, Result};

#[cfg(unix)]
use crate::ipc::pipe_unix::NamedPipe;

#[cfg(windows)]
use crate::ipc::pipe_windows::NamedPipe;

/// Send a PLAY command to the server to play a new JSON file
///
/// # Arguments
/// * `json_path` - Path to the JSON file to play
///
/// # Returns
/// * `Ok(())` - Command succeeded
/// * `Err` - Server error or communication failure
///
/// # Examples
/// ```no_run
/// use ym2151_log_player_rust::client::play_file;
///
/// play_file("sample_events.json").expect("Failed to play file");
/// ```
pub fn play_file(json_path: &str) -> Result<()> {
    send_command(Command::Play(json_path.to_string()))
}

/// Send a STOP command to the server to stop playback
///
/// # Returns
/// * `Ok(())` - Command succeeded
/// * `Err` - Server error or communication failure
///
/// # Examples
/// ```no_run
/// use ym2151_log_player_rust::client::stop_playback;
///
/// stop_playback().expect("Failed to stop playback");
/// ```
pub fn stop_playback() -> Result<()> {
    send_command(Command::Stop)
}

/// Send a SHUTDOWN command to the server to shut it down
///
/// # Returns
/// * `Ok(())` - Command succeeded
/// * `Err` - Server error or communication failure
///
/// # Examples
/// ```no_run
/// use ym2151_log_player_rust::client::shutdown_server;
///
/// shutdown_server().expect("Failed to shutdown server");
/// ```
pub fn shutdown_server() -> Result<()> {
    send_command(Command::Shutdown)
}

fn send_command(command: Command) -> Result<()> {
    // Connect to the server's named pipe
    let mut writer = NamedPipe::connect_default()
        .context("Failed to connect to server. Is the server running?")?;

    // Serialize and send the command
    let message = command.serialize();
    writer
        .write_str(&message)
        .context("Failed to send command to server")?;

    // For Windows implementation, we use fire-and-forget approach for now
    // The server will process the command, but we don't wait for a response
    // This simplifies the Windows named pipe communication
    eprintln!("✅ Command sent successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // These tests require a running server, so they are marked as ignored
    // To run them manually: cargo test --no-default-features -- --ignored

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
        // This should fail since there's no server running
        let result = send_command(Command::Stop);
        assert!(result.is_err());
    }
}
