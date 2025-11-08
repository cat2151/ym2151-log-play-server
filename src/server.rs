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

#[cfg(unix)]
use crate::ipc::pipe_unix::NamedPipe;

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
    #[cfg(unix)]
    pub fn run(&self, json_path: &str) -> Result<()> {
        eprintln!("🚀 Starting YM2151 server...");
        eprintln!("   Initial file: {}", json_path);

        // Create the named pipe
        let pipe = NamedPipe::create().context("Failed to create named pipe")?;
        eprintln!("✅ Named pipe created at: {:?}", pipe.path());

        // TODO: In Phase 5, start playback of the initial JSON file
        // For now, just set state to Playing
        {
            let mut state = self.state.lock().unwrap();
            *state = ServerState::Playing;
        }
        eprintln!("🎵 Initial playback started (stub)");

        // Start the IPC listener thread
        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);

        let listener_handle =
            thread::spawn(move || Self::ipc_listener_loop(pipe, state_clone, shutdown_flag_clone));

        eprintln!("✅ Server is ready and listening for commands");

        // Wait for the listener thread to finish (on shutdown)
        listener_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Listener thread panicked"))?
            .context("Listener thread error")?;

        eprintln!("👋 Server shutdown complete");
        Ok(())
    }

    /// IPC listener loop that processes incoming commands
    ///
    /// This runs in a separate thread and continuously accepts connections
    /// on the named pipe, processing each command until shutdown is signaled.
    #[cfg(unix)]
    fn ipc_listener_loop(
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
                    // Send error response
                    // Note: In this temporary implementation, we don't have bidirectional
                    // communication, so we just log the error
                    continue;
                }
            };

            eprintln!("📩 Received command: {:?}", command);

            // Process the command
            let response = Self::process_command(&command, &state, &shutdown_flag);

            eprintln!("📤 Response: {:?}", response);

            // Note: In this temporary implementation, we don't send responses back
            // A production implementation would use bidirectional communication

            // If shutdown was requested, break the loop
            if shutdown_flag.load(Ordering::Relaxed) {
                break;
            }
        }

        Ok(())
    }

    /// Process a command and return the appropriate response
    ///
    /// # Arguments
    /// * `command` - The command to process
    /// * `state` - Shared server state
    /// * `shutdown_flag` - Shared shutdown flag
    ///
    /// # Returns
    /// Response indicating success or failure
    fn process_command(
        command: &Command,
        state: &Arc<Mutex<ServerState>>,
        shutdown_flag: &Arc<AtomicBool>,
    ) -> Response {
        match command {
            Command::Play(json_path) => {
                eprintln!("🎵 Processing PLAY command: {}", json_path);
                // TODO: In Phase 5, implement actual playback control
                // For now, just update state
                match state.lock() {
                    Ok(mut s) => {
                        *s = ServerState::Playing;
                        Response::Ok
                    }
                    Err(e) => Response::Error(format!("Failed to update state: {}", e)),
                }
            }
            Command::Stop => {
                eprintln!("⏸️  Processing STOP command");
                // TODO: In Phase 5, implement actual playback stop
                // For now, just update state
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
                // TODO: In Phase 5, stop any active playback
                // For now, just set the shutdown flag
                shutdown_flag.store(true, Ordering::Relaxed);
                Response::Ok
            }
        }
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

    #[test]
    fn test_process_play_command() {
        let state = Arc::new(Mutex::new(ServerState::Stopped));
        let shutdown_flag = Arc::new(AtomicBool::new(false));

        let command = Command::Play("test.json".to_string());
        let response = Server::process_command(&command, &state, &shutdown_flag);

        assert_eq!(response, Response::Ok);
        assert_eq!(*state.lock().unwrap(), ServerState::Playing);
        assert!(!shutdown_flag.load(Ordering::Relaxed));
    }

    #[test]
    fn test_process_stop_command() {
        let state = Arc::new(Mutex::new(ServerState::Playing));
        let shutdown_flag = Arc::new(AtomicBool::new(false));

        let command = Command::Stop;
        let response = Server::process_command(&command, &state, &shutdown_flag);

        assert_eq!(response, Response::Ok);
        assert_eq!(*state.lock().unwrap(), ServerState::Stopped);
        assert!(!shutdown_flag.load(Ordering::Relaxed));
    }

    #[test]
    fn test_process_shutdown_command() {
        let state = Arc::new(Mutex::new(ServerState::Playing));
        let shutdown_flag = Arc::new(AtomicBool::new(false));

        let command = Command::Shutdown;
        let response = Server::process_command(&command, &state, &shutdown_flag);

        assert_eq!(response, Response::Ok);
        assert!(shutdown_flag.load(Ordering::Relaxed));
    }
}
