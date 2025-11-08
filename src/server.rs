//! Server module for the YM2151 playback server
//!
//! This module implements a server that listens for commands via named pipes
//! and controls YM2151 playback. The server runs in the background and accepts
//! commands from clients to play files, stop playback, or shutdown.

use crate::ipc::protocol::{Command, Response};
use anyhow::{Context, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(feature = "realtime-audio")]
use crate::events::EventLog;
#[cfg(feature = "realtime-audio")]
use crate::player::Player;
#[cfg(feature = "realtime-audio")]
use std::sync::mpsc::{self, Receiver, Sender};

#[cfg(feature = "realtime-audio")]
use crate::audio::AudioPlayer;

#[cfg(unix)]
use crate::ipc::pipe_unix::NamedPipe;

/// Internal command for playback control
#[cfg(feature = "realtime-audio")]
enum PlaybackCommand {
    Play(String), // JSON path
    Stop,
    Shutdown,
}

/// Server state indicating current playback status
#[derive(Debug, Clone, PartialEq, Eq)]
enum ServerState {
    /// Server is playing audio
    Playing,
    /// Server is stopped (silent)
    Stopped,
}

/// Server structure that manages the YM2151 playback server
pub struct Server {
    /// Current server state (playing or stopped)
    state: Arc<Mutex<ServerState>>,
    /// Flag to signal server shutdown
    shutdown_flag: Arc<AtomicBool>,
}

impl Server {
    /// Create a new server instance
    ///
    /// # Returns
    /// A new Server instance with initial state set to Stopped
    pub fn new() -> Self {
        Server {
            state: Arc::new(Mutex::new(ServerState::Stopped)),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Run the server with the specified JSON file
    ///
    /// This method creates a named pipe, starts listening for client commands,
    /// and processes them until a shutdown command is received.
    ///
    /// # Arguments
    /// * `json_path` - Path to the initial JSON file to play
    ///
    /// # Returns
    /// * `Ok(())` - Server shutdown successfully
    /// * `Err` - Error during server operation
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::server::Server;
    ///
    /// let server = Server::new();
    /// server.run("sample_events.json").expect("Server failed");
    /// ```
    #[cfg(all(unix, feature = "realtime-audio"))]
    pub fn run(&self, json_path: &str) -> Result<()> {
        eprintln!("🚀 Starting YM2151 server...");
        eprintln!("   Initial file: {}", json_path);

        // Create the named pipe
        let pipe = NamedPipe::create().context("Failed to create named pipe")?;
        eprintln!("✅ Named pipe created at: {:?}", pipe.path());

        // Create a channel for playback commands
        let (cmd_tx, cmd_rx): (Sender<PlaybackCommand>, Receiver<PlaybackCommand>) =
            mpsc::channel();

        // Start the playback controller thread
        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);
        let initial_json = json_path.to_string();
        let controller_handle = thread::spawn(move || {
            Self::playback_controller_thread(initial_json, cmd_rx, state_clone, shutdown_flag_clone)
        });

        // Start the IPC listener thread
        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);
        let listener_handle = thread::spawn(move || {
            Self::ipc_listener_loop(pipe, state_clone, shutdown_flag_clone, cmd_tx)
        });

        eprintln!("✅ Server is ready and listening for commands");

        // Wait for threads to finish
        listener_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Listener thread panicked"))?
            .context("Listener thread error")?;

        controller_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Controller thread panicked"))?
            .context("Controller thread error")?;

        eprintln!("👋 Server shutdown complete");
        Ok(())
    }

    #[cfg(all(unix, not(feature = "realtime-audio")))]
    pub fn run(&self, json_path: &str) -> Result<()> {
        eprintln!("🚀 Starting YM2151 server...");
        eprintln!("   Initial file: {}", json_path);

        // Create the named pipe
        let pipe = NamedPipe::create().context("Failed to create named pipe")?;
        eprintln!("✅ Named pipe created at: {:?}", pipe.path());

        {
            let mut state = self.state.lock().unwrap();
            *state = ServerState::Playing;
        }
        eprintln!("🎵 Initial playback started (audio feature disabled)");

        // Start the IPC listener thread
        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);
        let listener_handle = thread::spawn(move || {
            Self::ipc_listener_loop_no_audio(pipe, state_clone, shutdown_flag_clone)
        });

        eprintln!("✅ Server is ready and listening for commands");

        // Wait for the listener thread to finish (on shutdown)
        listener_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Listener thread panicked"))?
            .context("Listener thread error")?;

        eprintln!("👋 Server shutdown complete");
        Ok(())
    }

    /// Playback controller thread that manages the AudioPlayer
    ///
    /// This thread owns the AudioPlayer and processes playback commands from the IPC listener.
    #[cfg(feature = "realtime-audio")]
    fn playback_controller_thread(
        initial_json: String,
        cmd_rx: Receiver<PlaybackCommand>,
        state: Arc<Mutex<ServerState>>,
        shutdown_flag: Arc<AtomicBool>,
    ) -> Result<()> {
        // Start initial playback
        let mut audio_player: Option<AudioPlayer> = None;
        match Self::load_and_start_playback(&initial_json) {
            Ok(player) => {
                audio_player = Some(player);
                if let Ok(mut s) = state.lock() {
                    *s = ServerState::Playing;
                }
                eprintln!("🎵 Initial playback started");
            }
            Err(e) => {
                eprintln!("⚠️  Failed to start initial playback: {}", e);
            }
        }

        // Process commands
        loop {
            match cmd_rx.recv() {
                Ok(PlaybackCommand::Play(json_path)) => {
                    eprintln!("🎵 Controller: Processing PLAY command: {}", json_path);

                    // Stop current playback
                    if let Some(ref mut player) = audio_player {
                        player.stop();
                    }
                    audio_player = None;

                    // Start new playback
                    match Self::load_and_start_playback(&json_path) {
                        Ok(player) => {
                            audio_player = Some(player);
                            if let Ok(mut s) = state.lock() {
                                *s = ServerState::Playing;
                            }
                            eprintln!("✅ Playback started: {}", json_path);
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to start playback: {}", e);
                            if let Ok(mut s) = state.lock() {
                                *s = ServerState::Stopped;
                            }
                        }
                    }
                }
                Ok(PlaybackCommand::Stop) => {
                    eprintln!("⏸️  Controller: Processing STOP command");
                    if let Some(ref mut player) = audio_player {
                        player.stop();
                    }
                    audio_player = None;
                    if let Ok(mut s) = state.lock() {
                        *s = ServerState::Stopped;
                    }
                }
                Ok(PlaybackCommand::Shutdown) => {
                    eprintln!("🛑 Controller: Processing SHUTDOWN command");
                    if let Some(ref mut player) = audio_player {
                        player.stop();
                    }
                    #[allow(unused_assignments)]
                    {
                        audio_player = None;
                    }
                    break;
                }
                Err(_) => {
                    // Channel closed, probably server shutting down
                    if shutdown_flag.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load a JSON file and create an AudioPlayer
    #[cfg(feature = "realtime-audio")]
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

    /// IPC listener loop that processes incoming commands
    ///
    /// This runs in a separate thread and continuously accepts connections
    /// on the named pipe, processing each command until shutdown is signaled.
    #[cfg(all(unix, feature = "realtime-audio"))]
    fn ipc_listener_loop(
        pipe: NamedPipe,
        _state: Arc<Mutex<ServerState>>,
        shutdown_flag: Arc<AtomicBool>,
        cmd_tx: Sender<PlaybackCommand>,
    ) -> Result<()> {
        loop {
            // Check if shutdown was requested
            if shutdown_flag.load(Ordering::Relaxed) {
                break;
            }

            // Open the pipe for reading (this blocks until a client connects)
            let mut reader = match pipe.open_read() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to open pipe for reading: {}", e);
                    continue;
                }
            };

            // Read the command from the client
            let line = match reader.read_line() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to read from pipe: {}", e);
                    continue;
                }
            };

            // Parse the command
            let command = match Command::parse(&line) {
                Ok(cmd) => cmd,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to parse command: {}", e);
                    continue;
                }
            };

            eprintln!("📩 Received command: {:?}", command);

            // Send command to controller thread
            let response = match command {
                Command::Play(ref json_path) => {
                    match cmd_tx.send(PlaybackCommand::Play(json_path.clone())) {
                        Ok(_) => Response::Ok,
                        Err(e) => Response::Error(format!("Failed to send command: {}", e)),
                    }
                }
                Command::Stop => match cmd_tx.send(PlaybackCommand::Stop) {
                    Ok(_) => Response::Ok,
                    Err(e) => Response::Error(format!("Failed to send command: {}", e)),
                },
                Command::Shutdown => match cmd_tx.send(PlaybackCommand::Shutdown) {
                    Ok(_) => {
                        shutdown_flag.store(true, Ordering::Relaxed);
                        Response::Ok
                    }
                    Err(e) => Response::Error(format!("Failed to send command: {}", e)),
                },
            };

            eprintln!("📤 Response: {:?}", response);

            // If shutdown was requested, break the loop
            if shutdown_flag.load(Ordering::Relaxed) {
                break;
            }
        }

        Ok(())
    }

    /// IPC listener loop (without realtime-audio feature)
    #[cfg(all(unix, not(feature = "realtime-audio")))]
    fn ipc_listener_loop_no_audio(
        pipe: NamedPipe,
        state: Arc<Mutex<ServerState>>,
        shutdown_flag: Arc<AtomicBool>,
    ) -> Result<()> {
        loop {
            // Check if shutdown was requested
            if shutdown_flag.load(Ordering::Relaxed) {
                break;
            }

            // Open the pipe for reading (this blocks until a client connects)
            let mut reader = match pipe.open_read() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to open pipe for reading: {}", e);
                    continue;
                }
            };

            // Read the command from the client
            let line = match reader.read_line() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to read from pipe: {}", e);
                    continue;
                }
            };

            // Parse the command
            let command = match Command::parse(&line) {
                Ok(cmd) => cmd,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to parse command: {}", e);
                    continue;
                }
            };

            eprintln!("📩 Received command: {:?}", command);

            // Process the command (simplified without audio)
            let response = match command {
                Command::Play(ref json_path) => {
                    eprintln!("🎵 Processing PLAY command: {} (audio disabled)", json_path);
                    match state.lock() {
                        Ok(mut s) => {
                            *s = ServerState::Playing;
                            Response::Ok
                        }
                        Err(e) => Response::Error(format!("Failed to update state: {}", e)),
                    }
                }
                Command::Stop => {
                    eprintln!("⏸️  Processing STOP command (audio disabled)");
                    match state.lock() {
                        Ok(mut s) => {
                            *s = ServerState::Stopped;
                            Response::Ok
                        }
                        Err(e) => Response::Error(format!("Failed to update state: {}", e)),
                    }
                }
                Command::Shutdown => {
                    eprintln!("🛑 Processing SHUTDOWN command");
                    shutdown_flag.store(true, Ordering::Relaxed);
                    Response::Ok
                }
            };

            eprintln!("📤 Response: {:?}", response);

            // If shutdown was requested, break the loop
            if shutdown_flag.load(Ordering::Relaxed) {
                break;
            }
        }

        Ok(())
    }

    /// Get the current server state
    ///
    /// This is primarily useful for testing
    #[cfg(test)]
    fn get_state(&self) -> ServerState {
        self.state.lock().unwrap().clone()
    }

    /// Check if shutdown has been requested
    ///
    /// This is primarily useful for testing
    #[cfg(test)]
    fn is_shutdown_requested(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(windows)]
impl Server {
    pub fn run(&self, _json_path: &str) -> Result<()> {
        Err(anyhow::anyhow!(
            "Windows server is not yet implemented. Use Unix/Linux systems."
        ))
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

    // Note: The process_command tests have been removed because the new architecture
    // uses a channel-based system with separate controller and listener threads.
    // The command processing is now tested through integration tests that exercise
    // the full server/client interaction.
}
