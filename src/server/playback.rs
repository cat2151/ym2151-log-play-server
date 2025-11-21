use crate::audio::AudioPlayer;
use crate::events::EventLog;
use crate::logging;
use crate::player::Player;
use crate::resampler::ResamplingQuality;
use anyhow::{Context, Result};

/// Manages audio playback initialization
pub struct PlaybackManager {
    resampling_quality: ResamplingQuality,
}

impl PlaybackManager {
    pub fn new(resampling_quality: ResamplingQuality) -> Self {
        Self { resampling_quality }
    }

    /// Load event log and start playback
    pub fn load_and_start_playback(&self, data: &str, is_json_string: bool) -> Result<AudioPlayer> {
        let log = if is_json_string {
            // Parse as JSON string directly
            EventLog::from_json_str(data).with_context(|| "Failed to parse JSON string data")?
        } else {
            // Load from file path
            EventLog::from_file(data)
                .with_context(|| format!("Failed to load JSON file: {}", data))?
        };

        if !log.validate() {
            return Err(anyhow::anyhow!(
                "Event log validation failed: events are not in chronological order"
            ));
        }

        let player = Player::new(log.clone());
        // Pass the event log to AudioPlayer if in verbose mode
        let event_log = if logging::is_verbose() {
            Some(log)
        } else {
            None
        };
        AudioPlayer::new_with_quality(player, event_log, self.resampling_quality)
            .context("Failed to create audio player")
    }

    /// Start interactive mode
    pub fn start_interactive_mode(&self) -> Result<AudioPlayer> {
        let player = Player::new_interactive();
        // No event log in interactive mode, and no WAV output
        AudioPlayer::new_with_quality(player, None, self.resampling_quality)
            .context("Failed to create interactive audio player")
    }
}
