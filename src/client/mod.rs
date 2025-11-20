//! Client module for sending commands to the YM2151 log player server.
//!
//! This module provides functions for communicating with a running server instance
//! to control playback of YM2151 register event logs.
//!
//! # Verbose Mode
//!
//! By default, the client operates in non-verbose mode to prevent disrupting TUI applications.
//! Use [`init_client`] to enable verbose output:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Enable verbose mode for debugging
//! client::init_client(true);
//!
//! // Or disable verbose mode for TUI applications (default)
//! client::init_client(false);
//! ```
//!
//! # Usage
//!
//! ## Playing JSON Data
//!
//! Use [`send_json`] to send JSON data:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! let json_data = r#"{"event_count": 2, "events": [...]}"#;
//! client::send_json(json_data)?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Interactive Mode with JSON Data
//!
//! Use [`play_json_interactive`] to send ym2151log format JSON data to interactive mode:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Start interactive mode once
//! client::start_interactive()?;
//!
//! // Send multiple JSONs without stopping - no audio gaps!
//! let json1 = r#"{"event_count": 2, "events": [
//!     {"time": 0, "addr": "0x08", "data": "0x00"},
//!     {"time": 100, "addr": "0x20", "data": "0xC7"}
//! ]}"#;
//! client::play_json_interactive(json1)?;
//!
//! // Stop when done
//! client::stop_interactive()?;
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! ## Ensuring Server is Ready
//!
//! Use [`ensure_server_ready`] to automatically ensure the server is running and ready:
//!
//! ```no_run
//! use ym2151_log_play_server::client;
//!
//! // Ensure server is ready (installs and starts if needed)
//! client::ensure_server_ready("cat-play-mml")?;
//!
//! // Now you can send JSON data
//! let json_data = r#"{"event_count": 1, "events": [...]}"#;
//! client::send_json(json_data)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

// Submodules
pub mod config;
pub mod core;
pub mod interactive;
pub mod json;
pub mod server;

// Re-export commonly used functions from submodules
// This maintains backward compatibility while organizing code by responsibility

// Configuration functions
pub use config::{init_client, is_client_verbose, log_client};

// Core client communication
pub use core::{send_command, shutdown_server, stop_playback};

// JSON-related functionality
pub use json::send_json;

// Interactive mode functionality
pub use interactive::{
    clear_schedule, get_server_time, play_json_interactive, start_interactive, stop_interactive,
};

// Server management functionality
pub use server::{ensure_server_ready, is_app_in_path, is_server_running};

// Test helpers (only available in test builds)
#[cfg(all(test, windows))]
pub use server::test_helpers;
