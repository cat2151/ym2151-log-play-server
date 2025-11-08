





use crate::ipc::protocol::{Command, Response};
use anyhow::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use std::thread;
use anyhow::Context;
use std::sync::atomic::Ordering;

use crate::events::EventLog;
use crate::player::Player;
use std::sync::mpsc::{self, Receiver, Sender};

use crate::audio::AudioPlayer;

#[cfg(unix)]
use crate::ipc::pipe_unix::NamedPipe;

#[cfg(windows)]
use crate::ipc::pipe_windows::NamedPipe;


#[allow(dead_code)]
enum PlaybackCommand {
    Play(String),
    Stop,
    Shutdown,
}


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




















    #[cfg(unix)]
    pub fn run(&self, json_path: &str) -> Result<()> {
        eprintln!("🚀 Starting YM2151 server...");
        eprintln!("   Initial file: {}", json_path);


        let pipe = NamedPipe::create().context("Failed to create named pipe")?;
        eprintln!("✅ Named pipe created at: {:?}", pipe.path());


        let (cmd_tx, cmd_rx): (Sender<PlaybackCommand>, Receiver<PlaybackCommand>) =
            mpsc::channel();


        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);
        let initial_json = json_path.to_string();
        let controller_handle = thread::spawn(move || {
            Self::playback_controller_thread(initial_json, cmd_rx, state_clone, shutdown_flag_clone)
        });


        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);
        let listener_handle = thread::spawn(move || {
            Self::ipc_listener_loop(pipe, state_clone, shutdown_flag_clone, cmd_tx)
        });

        eprintln!("✅ Server is ready and listening for commands");


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




    #[allow(dead_code)]
    fn playback_controller_thread(
        initial_json: String,
        cmd_rx: Receiver<PlaybackCommand>,
        state: Arc<Mutex<ServerState>>,
        shutdown_flag: Arc<AtomicBool>,
    ) -> Result<()> {

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


        loop {
            match cmd_rx.recv() {
                Ok(PlaybackCommand::Play(json_path)) => {
                    eprintln!("🎵 Controller: Processing PLAY command: {}", json_path);


                    if let Some(ref mut player) = audio_player {
                        player.stop();
                    }
                    audio_player = None;


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

                    if shutdown_flag.load(Ordering::Relaxed) {
                        break;
                    }
                }
            }
        }

        Ok(())
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





    #[cfg(unix)]
    fn ipc_listener_loop(
        pipe: NamedPipe,
        _state: Arc<Mutex<ServerState>>,
        shutdown_flag: Arc<AtomicBool>,
        cmd_tx: Sender<PlaybackCommand>,
    ) -> Result<()> {
        loop {

            if shutdown_flag.load(Ordering::Relaxed) {
                break;
            }


            let mut reader = match pipe.open_read() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to open pipe for reading: {}", e);
                    continue;
                }
            };


            let line = match reader.read_line() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to read from pipe: {}", e);
                    continue;
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


            if shutdown_flag.load(Ordering::Relaxed) {
                break;
            }
        }

        Ok(())
    }




    #[cfg(test)]
    fn get_state(&self) -> ServerState {
        self.state.lock().unwrap().clone()
    }




    #[cfg(test)]
    fn is_shutdown_requested(&self) -> bool {
        use std::sync::atomic::Ordering;
        self.shutdown_flag.load(Ordering::Relaxed)
    }


    #[cfg(windows)]
    pub fn run(&self, json_path: &str) -> Result<()> {

        self.run_windows(json_path)
    }


    #[cfg(windows)]
    fn run_windows(&self, json_path: &str) -> Result<()> {
        eprintln!("🚀 Starting YM2151 server (Windows)...");
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


        let (cmd_tx, cmd_rx): (Sender<PlaybackCommand>, Receiver<PlaybackCommand>) =
            mpsc::channel();


        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);
        let initial_json = json_path.to_string();
        let controller_handle = thread::spawn(move || {
            Self::playback_controller_thread(initial_json, cmd_rx, state_clone, shutdown_flag_clone)
        });


        let state_clone = Arc::clone(&self.state);
        let shutdown_flag_clone = Arc::clone(&self.shutdown_flag);
        let listener_handle = thread::spawn(move || {
            Self::ipc_listener_loop_windows(pipe, state_clone, shutdown_flag_clone, cmd_tx)
        });

        eprintln!("✅ Server is ready and listening for commands");


        controller_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Controller thread panicked"))?
            .context("Controller thread error")?;

        listener_handle
            .join()
            .map_err(|_| anyhow::anyhow!("Listener thread panicked"))?
            .context("Listener thread error")?;

        eprintln!("👋 Server shutdown complete");
        Ok(())
    }


    #[cfg(windows)]
    fn ipc_listener_loop_windows(
        pipe: NamedPipe,
        _state: Arc<Mutex<ServerState>>,
        shutdown_flag: Arc<AtomicBool>,
        cmd_tx: Sender<PlaybackCommand>,
    ) -> Result<()> {
        loop {

            if shutdown_flag.load(Ordering::Relaxed) {
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


                let playback_cmd = match command {
                    Command::Play(json_path) => Some(PlaybackCommand::Play(json_path)),
                    Command::Stop => Some(PlaybackCommand::Stop),
                    Command::Shutdown => {
                        eprintln!("🛑 Processing SHUTDOWN command");
                        shutdown_flag.store(true, Ordering::Relaxed);
                        Some(PlaybackCommand::Shutdown)
                    }
                };


                if let Some(cmd) = playback_cmd {
                    if cmd_tx.send(cmd).is_err() {
                        eprintln!("⚠️  Warning: Failed to send command to playback controller");
                        break;
                    }
                }

                eprintln!("📤 Response: OK");





                if shutdown_flag.load(Ordering::Relaxed) {
                    return Ok(());
                }
            }
        }

        Ok(())
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
