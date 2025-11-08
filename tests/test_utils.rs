//! Test utilities for coordinating test execution
//!
//! This module provides utilities to ensure tests that use shared resources
//! (like named pipes or audio devices) are executed sequentially to prevent
//! race conditions and conflicts.

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global mutex for server-related tests that use named pipes
static SERVER_TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

// Global mutex for audio-related tests that might access audio devices
#[allow(dead_code)] // Used in tests, but not detected in main compilation
static AUDIO_TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// Guard for server tests that ensures sequential execution
pub fn server_test_lock() -> std::sync::MutexGuard<'static, ()> {
    SERVER_TEST_MUTEX.lock().unwrap_or_else(|poison| {
        eprintln!("Warning: server test mutex was poisoned, recovering...");
        poison.into_inner()
    })
}

/// Guard for audio tests that ensures sequential execution
#[allow(dead_code)] // Used in tests, but not detected in main compilation
pub fn audio_test_lock() -> std::sync::MutexGuard<'static, ()> {
    AUDIO_TEST_MUTEX.lock().unwrap_or_else(|poison| {
        eprintln!("Warning: audio test mutex was poisoned, recovering...");
        poison.into_inner()
    })
}
