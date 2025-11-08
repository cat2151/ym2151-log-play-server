use crate::ipc::protocol::Command;
use anyhow::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use std::sync::atomic::Ordering;

use crate::events::EventLog;
use crate::player::Player;

use crate::audio::AudioPlayer;
use crate::ipc::pipe_windows::NamedPipe;


#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
enum ServerState {
    Playing,
    Stopped,
}


pub struct Server {
    #[allow(dead_code)]
    state: Arc<Mutex<ServerState>>,
    #[allow(dead_code)]
    shutdown_flag: Arc<AtomicBool>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            state: Arc::new(Mutex::new(ServerState::Stopped)),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&self, json_path: &str) -> Result<()> {
        eprintln!("🚀 Starting YM2151 server...");
        eprintln!("   Initial file: {}", json_path);

        let pipe = NamedPipe::create().context("Failed to create named pipe")?;
        eprintln!("✅ Named pipe created at: {:?}", pipe.path());

        let mut audio_player: Option<AudioPlayer> = None;
        match Self::load_and_start_playback(json_path) {
            Ok(player) => {
                audio_player = Some(player);
                eprintln!("✅ Initial audio playback started");
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to start initial audio playback: {}", e);
            }
        }

        {
            let mut state = self.state.lock().unwrap();
            *state = ServerState::Playing;
        }

        loop {
            if self.shutdown_flag.load(Ordering::Relaxed) {
                break;
            }

            let mut reader = match pipe.open_read() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to open pipe for reading: {}", e);
                    continue;
                }
            };

            loop {
                let line = match reader.read_line() {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("⚠️  Warning: Failed to read from pipe: {}", e);
                        break;
                    }
                };

                let command = match Command::parse(&line) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        eprintln!("⚠️  Warning: Failed to parse command: {}", e);
                        continue;
                    }
                };

                eprintln!("📩 Received command: {:?}", command);

                match command {
                    Command::Play(json_path) => {
                        eprintln!("🎵 Loading new audio file: {}", json_path);

                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        match Self::load_and_start_playback(&json_path) {
                            Ok(player) => {
                                audio_player = Some(player);
                                eprintln!("✅ Audio playback started: {}", json_path);

                                let mut state = self.state.lock().unwrap();
                                *state = ServerState::Playing;
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to start audio playback: {}", e);
                            }
                        }
                    }
                    Command::Stop => {
                        eprintln!("⏹️  Stopping audio playback");
                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        let mut state = self.state.lock().unwrap();
                        *state = ServerState::Stopped;
                    }
                    Command::Shutdown => {
                        eprintln!("🛑 Shutdown requested");
                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }
                        self.shutdown_flag.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            }
        }

        eprintln!("👋 Server shutdown complete");
        Ok(())
    }

    #[cfg(test)]
    fn get_state(&self) -> ServerState {
        self.state.lock().unwrap().clone()
    }

    #[cfg(test)]
    fn is_shutdown_requested(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }

    #[allow(dead_code)]
    fn load_and_start_playback(json_path: &str) -> Result<AudioPlayer> {
        let log = EventLog::from_file(json_path)
            .with_context(|| format!("Failed to load JSON file: {}", json_path))?;

        if !log.validate() {
            return Err(anyhow::anyhow!(
                "Event log validation failed: event_count doesn't match events array length"
            ));
        }

        let player = Player::new(log);
        AudioPlayer::new(player).context("Failed to create audio player")
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = Server::new();
        assert_eq!(server.get_state(), ServerState::Stopped);
        assert!(!server.is_shutdown_requested());
    }

    #[test]
    fn test_server_default() {
        let server = Server::default();
        assert_eq!(server.get_state(), ServerState::Stopped);
    }
}
