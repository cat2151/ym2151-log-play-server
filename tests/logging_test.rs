use ym2151_log_play_server::logging;

// Import the server test mutex from test utilities
mod test_util_server_mutex;
use test_util_server_mutex::server_test_lock;

#[test]
fn test_logging_verbose_mode() {
    let _lock = server_test_lock();

    // Test verbose mode enabled
    logging::init(true);
    assert!(logging::is_server_verbose());

    // Test verbose mode disabled
    logging::init(false);
    assert!(!logging::is_server_verbose());
}

#[test]
fn test_logging_functions_dont_panic() {
    let _lock = server_test_lock();

    // Initialize with verbose mode
    logging::init(true);

    // These should not panic
    logging::log_always_server("Test always message");
    logging::log_verbose_server("Test verbose message");

    // Initialize with non-verbose mode
    logging::init(false);

    // These should also not panic
    logging::log_always_server("Test always message");
    logging::log_verbose_server("Test verbose message");
}

#[test]
fn test_logging_verbose_mode_demonstration() {
    let _lock = server_test_lock();

    // Test verbose mode demonstration (migrated from examples/test_logging_verbose.rs)
    logging::init(true);

    // Verify verbose mode is enabled
    assert!(logging::is_server_verbose());

    // In verbose mode:
    // - log_always() would print and log
    // - log_verbose() would also print
    logging::log_always_server("Test message for verbose mode");
    logging::log_verbose_server("Test verbose-specific message");
}

#[test]
fn test_logging_non_verbose_mode_demonstration() {
    let _lock = server_test_lock();

    // Test non-verbose mode demonstration (migrated from examples/test_logging_non_verbose.rs)
    logging::init(false);

    // Verify non-verbose mode is enabled
    assert!(!logging::is_server_verbose());

    // In non-verbose mode:
    // - log_always() should only log to file, not print
    // - log_verbose() should not print or log
    logging::log_always_server("Test message for non-verbose mode (should only log to file)");
    logging::log_verbose_server("Test verbose message (should not print or log)");
}
