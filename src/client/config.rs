//! Client configuration module
//!
//! This module handles client-side configuration such as verbose mode.

use std::sync::Mutex;

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

/// Print a message to stderr only if verbose mode is enabled
pub fn log_client(message: &str) {
    if is_client_verbose() {
        eprintln!("{}", message);
    }
}
