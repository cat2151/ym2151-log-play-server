//! WAV buffer management for audio recording and debugging
//!
//! This module handles the management of audio buffers used for WAV file generation
//! at both the native OPM sampling rate (55930 Hz) and the output sampling rate (48000 Hz).

use std::sync::{Arc, Mutex};

/// Buffer manager for WAV file generation and audio debugging
pub struct WavBuffers {
    /// Buffer for 55930 Hz samples (OPM native rate)
    buffer_55k: Arc<Mutex<Vec<i16>>>,
    /// Buffer for 48000 Hz samples (resampled output rate)
    buffer_48k: Arc<Mutex<Vec<i16>>>,
}

impl WavBuffers {
    /// Create new WAV buffer manager
    pub fn new() -> Self {
        Self {
            buffer_55k: Arc::new(Mutex::new(Vec::new())),
            buffer_48k: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get clones of the buffer handles for sharing with threads
    pub fn get_handles(&self) -> (Arc<Mutex<Vec<i16>>>, Arc<Mutex<Vec<i16>>>) {
        (self.buffer_55k.clone(), self.buffer_48k.clone())
    }

    /// Get a copy of the 55kHz buffer contents
    pub fn get_buffer_55k(&self) -> Vec<i16> {
        self.buffer_55k
            .lock()
            .expect("Failed to lock WAV buffer - mutex poisoned")
            .clone()
    }

    /// Get a copy of the 48kHz buffer contents
    pub fn get_buffer_48k(&self) -> Vec<i16> {
        self.buffer_48k
            .lock()
            .expect("Failed to lock WAV buffer - mutex poisoned")
            .clone()
    }

    /// Clear both buffers
    pub fn clear(&self) {
        if let Ok(mut buf_55k) = self.buffer_55k.lock() {
            buf_55k.clear();
        }
        if let Ok(mut buf_48k) = self.buffer_48k.lock() {
            buf_48k.clear();
        }
    }
}

impl Default for WavBuffers {
    fn default() -> Self {
        Self::new()
    }
}
