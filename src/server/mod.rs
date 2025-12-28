mod command_handler;
mod connection;
mod playback;
mod state;

pub use command_handler::CommandHandler;
pub use playback::PlaybackManager;
pub use state::ServerState;

use crate::audio::AudioPlayer;
use crate::logging;
use crate::resampler::ResamplingQuality;
use crate::scheduler::TimeTracker;
use anyhow::Result;
use connection::ConnectionManager;
use std::sync::atomic::AtomicBool;
#[cfg(test)]
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

/// Main server structure
pub struct Server {
    state: Arc<Mutex<ServerState>>,
    shutdown_flag: Arc<AtomicBool>,
    resampling_quality: ResamplingQuality,
    time_tracker: Arc<Mutex<TimeTracker>>,
}

impl Server {
    pub fn new() -> Self {
        Self::new_with_resampling_quality(false)
    }

    pub fn new_with_resampling_quality(low_quality: bool) -> Self {
        let quality = if low_quality {
            ResamplingQuality::Linear
        } else {
            ResamplingQuality::HighQuality
        };

        logging::log_always_server(&format!(
            "🎵 リサンプリング品質: {}",
            match quality {
                ResamplingQuality::Linear => "低品質 (線形補間)",
                ResamplingQuality::HighQuality => "標準 (Rubato FFTベース)",
            }
        ));

        Server {
            state: Arc::new(Mutex::new(ServerState::Stopped)),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
            resampling_quality: quality,
            time_tracker: Arc::new(Mutex::new(TimeTracker::new())),
        }
    }

    /// Run the server main loop
    pub fn run(&self) -> Result<()> {
        // Initialize state
        {
            let mut state = self.state.lock().unwrap();
            *state = ServerState::Stopped;
        }

        // Create managers
        let playback_manager = PlaybackManager::new(self.resampling_quality);
        let command_handler = CommandHandler::new(
            Arc::clone(&self.state),
            Arc::clone(&self.shutdown_flag),
            Arc::clone(&self.time_tracker),
            playback_manager,
        );
        let connection_manager = ConnectionManager::new(command_handler);

        // Run connection loop
        connection_manager.run()
    }

    #[cfg(test)]
    pub fn get_state(&self) -> ServerState {
        self.state.lock().unwrap().clone()
    }

    #[cfg(test)]
    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }

    /// Start interactive mode for demo purposes
    /// This is a public wrapper for the private start_interactive_mode method
    /// to be used by demo_server module for standalone testing
    pub fn start_interactive_mode_demo(&self) -> Result<AudioPlayer> {
        let playback_manager = PlaybackManager::new(self.resampling_quality);
        playback_manager.start_interactive_mode()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
