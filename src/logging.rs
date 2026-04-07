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
//! Log messages are written to the OS-appropriate app data/config directory.

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Global verbose flag
static VERBOSE: Mutex<bool> = Mutex::new(false);

/// Log file path
const LOG_FILE: &str = "ym2151-server.log";
const LOG_DIR_NAME: &str = "ym2151-log-play-server";

/// Initialize logging with verbose flag
pub fn init(verbose: bool) {
    let mut v = VERBOSE.lock().unwrap();
    *v = verbose;
}

/// Check if verbose mode is enabled
pub fn is_server_verbose() -> bool {
    *VERBOSE.lock().unwrap()
}

fn current_dir_fallback() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

#[cfg(windows)]
pub(crate) fn log_directory_from_env(
    local_appdata: Option<PathBuf>,
    _xdg_config_home: Option<PathBuf>,
    _home: Option<PathBuf>,
) -> PathBuf {
    local_appdata
        .unwrap_or_else(current_dir_fallback)
        .join(LOG_DIR_NAME)
}

#[cfg(target_os = "macos")]
pub(crate) fn log_directory_from_env(
    _local_appdata: Option<PathBuf>,
    _xdg_config_home: Option<PathBuf>,
    home: Option<PathBuf>,
) -> PathBuf {
    home.unwrap_or_else(current_dir_fallback)
        .join("Library")
        .join("Application Support")
        .join(LOG_DIR_NAME)
}

#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) fn log_directory_from_env(
    _local_appdata: Option<PathBuf>,
    xdg_config_home: Option<PathBuf>,
    home: Option<PathBuf>,
) -> PathBuf {
    xdg_config_home
        .or_else(|| home.map(|path| path.join(".config")))
        .unwrap_or_else(current_dir_fallback)
        .join(LOG_DIR_NAME)
}

pub(crate) fn log_directory() -> PathBuf {
    log_directory_from_env(
        std::env::var_os("LOCALAPPDATA").map(PathBuf::from),
        std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from),
        std::env::var_os("HOME").map(PathBuf::from),
    )
}

pub(crate) fn log_file_path(file_name: &str) -> PathBuf {
    log_directory().join(file_name)
}

pub(crate) fn open_log_file(file_name: &str) -> std::io::Result<File> {
    let path = log_file_path(file_name);

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    OpenOptions::new().create(true).append(true).open(&path)
}

fn warn_log_open_failure(path: &Path, error: &std::io::Error) {
    eprintln!(
        "⚠️  Warning: Failed to open log file {}: {}",
        path.display(),
        error
    );
}

/// Write a message to the log file
fn write_to_log(message: &str) {
    let path = log_file_path(LOG_FILE);
    match open_log_file(LOG_FILE) {
        Ok(mut file) => {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            if let Err(e) = writeln!(file, "[{}] {}", timestamp, message) {
                eprintln!("⚠️  Warning: Failed to write to log file: {}", e);
            }
        }
        Err(error) => warn_log_open_failure(&path, &error),
    }
}

fn eprint_with_timestamp(message: &str) {
    let timestamp = chrono::Local::now().format("%H:%M:%S%.3f"); // 備忘、時刻があれば最低限わかる。カラム数を減らして、狭い分割terminalでも読みやすくする用。
    eprintln!("[{}] {}", timestamp, message);
}

/// Log a message that should always be logged to file.
/// Prints to stderr only if verbose mode is enabled.
///
/// Use this for:
/// - Server startup/shutdown
/// - Critical errors
/// - Important state changes
pub fn log_always_server(message: &str) {
    write_to_log(message);

    if is_server_verbose() {
        eprint_with_timestamp(message); // 備忘、非verbose時に表示しないのは、TUIからserverが起動されたり、TUIがclientとしてふるまったりするので、表示崩れさせない用
    }
}

/// Log a message only if verbose mode is enabled.
///
/// Use this for:
/// - Routine operations (receive, playback)
/// - Debug information
/// - Non-critical status updates
pub fn log_verbose_server(message: &str) {
    if is_server_verbose() {
        write_to_log(message); // 備忘、logにも記録する。でないと「printとlogを交互に見ないとわからず混乱」がありうる。logだけ見ればすべてわかるようにする。
        eprint_with_timestamp(message);
    }
}
