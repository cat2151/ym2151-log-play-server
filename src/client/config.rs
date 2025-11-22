//! Client configuration module
//!
//! This module handles client-side configuration such as verbose mode.

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

/// Log file path
const LOG_FILE: &str = "ym2151-client.log";

/// Global verbose flag for client operations
static CLIENT_VERBOSE: Mutex<bool> = Mutex::new(false);

/// Initialize client with verbose flag
///
/// This function controls whether the client prints status messages to stderr.
/// By default, the client operates in non-verbose mode to prevent disrupting TUI applications.
///
/// # Arguments
/// * `verbose` - Enable verbose output if true, disable if false
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::config;
/// // Enable verbose mode for debugging
/// config::init_client(true);
///
/// // Disable verbose mode for TUI applications
/// config::init_client(false);
/// ```
pub fn init_client(verbose: bool) {
    let mut v = CLIENT_VERBOSE.lock().unwrap();
    *v = verbose;
}

/// Check if client verbose mode is enabled
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::config;
/// config::init_client(true);
/// assert!(config::is_client_verbose());
/// ```
pub fn is_client_verbose() -> bool {
    *CLIENT_VERBOSE.lock().unwrap()
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

fn eprint_with_timestamp(message: &str) {
    let timestamp = chrono::Local::now().format("%H:%M:%S%.3f"); // 備忘、時刻があれば最低限わかる。カラム数を減らして、狭い分割terminalでも読みやすくする用。
    eprintln!("[{}] {}", timestamp, message);
}

pub fn log_always_client(message: &str) {
    write_to_log(message);

    if is_client_verbose() {
        eprint_with_timestamp(message); // 備忘、非verbose時に表示しないのは、TUIからserverが起動されたり、TUIがclientとしてふるまったりするので、表示崩れさせない用
    }
}

/// Print a message to stderr only if verbose mode is enabled
pub fn log_verbose_client(message: &str) {
    if is_client_verbose() {
        write_to_log(message);
        eprint_with_timestamp(message);
    }
}
