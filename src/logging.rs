//! Logging utilities for the YM2151 server
//!
//! This module provides logging functions that control output based on verbose mode.
//! It helps prevent TUI disruption when the server runs in background mode.
//!
//! # Logging Strategy
//!
//! - **Verbose mode**: Print to stderr + write to log file
//! - **Non-verbose mode**: Only write critical events to log file (no print)
//!
//! # Log File
//!
//! Log messages are written to `ym2151-server.log` in the current directory.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

/// Global verbose flag
static VERBOSE: Mutex<bool> = Mutex::new(false);

/// Log file path
const LOG_FILE: &str = "ym2151-server.log";

/// Initialize logging with verbose flag
pub fn init(verbose: bool) {
    let mut v = VERBOSE.lock().unwrap();
    *v = verbose;
}

/// Check if verbose mode is enabled
pub fn is_verbose() -> bool {
    *VERBOSE.lock().unwrap()
}

/// Write a message to the log file
fn write_to_log(message: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(LOG_FILE) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        if let Err(e) = writeln!(file, "[{}] {}", timestamp, message) {
            eprintln!("⚠️  Warning: Failed to write to log file: {}", e);
        }
    }
}

/// Log a message that should always be logged to file.
/// Prints to stderr only if verbose mode is enabled.
///
/// Use this for:
/// - Server startup/shutdown
/// - Critical errors
/// - Important state changes
pub fn log_always(message: &str) {
    write_to_log(message);

    if is_verbose() {
        eprintln!("{}", message);
    }
}

/// Log a message only if verbose mode is enabled.
/// Does not write to log file.
///
/// Use this for:
/// - Routine operations (receive, playback)
/// - Debug information
/// - Non-critical status updates
pub fn log_verbose(message: &str) {
    if is_verbose() {
        eprintln!("{}", message);
    }
}
