//! Audio command definitions for inter-thread communication
//!
//! This module defines commands that can be sent to the audio generation thread
//! to control playback behavior.

/// Commands for controlling the audio generation thread
#[derive(Debug, Clone)]
pub enum AudioCommand {
    /// Stop audio playback and terminate the generation thread
    Stop,
}
