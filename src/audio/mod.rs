//! Audio playback module with real-time priority optimization
//!
//! This module implements a dual-thread audio architecture with priority boosting
//! to minimize audio dropouts:
//!
//! 1. **Generator Thread** (see `generator` module):
//!    - Runs OPM emulation: `player.generate_samples()` → `chip.generate_samples()` → `OPM_Clock()`
//!    - Priority boost: Windows MMCSS "Pro Audio" task (via `mmcss` module)
//!    - Generates samples at OPM's native 55930 Hz rate
//!
//! 2. **CPAL Callback Thread** (managed by cpal library):
//!    - Sends resampled audio to hardware at 48000 Hz
//!    - Priority boost: Automatic via cpal's `audio_thread_priority` feature
//!
//! Both threads run with elevated priority to ensure smooth, glitch-free playback.

pub mod buffers;
pub mod commands;
pub mod generator;
pub mod player;
pub mod scheduler;
pub mod stream;

// Re-export the main public interfaces
pub use buffers::WavBuffers;
pub use commands::AudioCommand;
pub use player::AudioPlayer;
pub use scheduler::AudioScheduler;
