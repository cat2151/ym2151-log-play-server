//! Global mutex for interactive tests
//!
//! This module provides a shared mutex to ensure that interactive tests
//! that use the server are executed sequentially to prevent conflicts.

use std::sync::Mutex;

/// Global mutex for interactive tests that use the server
/// This ensures only one test accesses the named pipe at a time
pub static INTERACTIVE_TEST_MUTEX: Mutex<()> = Mutex::new(());

/// Acquire the interactive test lock
/// This should be called at the beginning of any test that uses the server
pub fn lock_interactive_test() -> std::sync::MutexGuard<'static, ()> {
    INTERACTIVE_TEST_MUTEX.lock().unwrap_or_else(|poison| {
        eprintln!("⚠️  Warning: interactive test mutex was poisoned, recovering...");
        poison.into_inner()
    })
}
