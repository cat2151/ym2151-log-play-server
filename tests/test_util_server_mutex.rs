//! Test utilities for server mutex coordination
//!
//! This module provides test utilities with a global mutex to ensure tests that use shared resources
//! (specifically the server executable and named pipes) are executed
//! sequentially to prevent race conditions and conflicts.
//!
//! The primary shared resource is the server process that creates and manages
//! named pipes for IPC communication. Multiple tests cannot safely start/stop
//! the server or use the same named pipe simultaneously.

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global mutex for server-related tests that use the server process and named pipes
// This prevents race conditions when different tests try to:
// - Start/stop the server executable
// - Connect to or use the same named pipe
// - Send commands through IPC
static SERVER_TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// Guard for server tests that ensures sequential execution
///
/// This ensures sequential execution for tests that use:
/// - Server process startup/shutdown
/// - Named pipe creation and communication
/// - IPC commands (PLAY, STOP, SHUTDOWN)
/// - Any functionality that requires the server to be running
///
/// Use this lock for any test that starts the server, connects to it,
/// or sends commands through the named pipe.
pub fn server_test_lock() -> std::sync::MutexGuard<'static, ()> {
    SERVER_TEST_MUTEX.lock().unwrap_or_else(|poison| {
        eprintln!("⚠️  Warning: server test mutex was poisoned, recovering...");
        poison.into_inner()
    })
}
