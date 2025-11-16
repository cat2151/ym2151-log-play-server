use ym2151_log_play_server::logging;

#[test]
fn test_logging_verbose_mode() {
    // Test verbose mode enabled
    logging::init(true);
    assert!(logging::is_verbose());

    // Test verbose mode disabled
    logging::init(false);
    assert!(!logging::is_verbose());
}

#[test]
fn test_logging_functions_dont_panic() {
    // Initialize with verbose mode
    logging::init(true);

    // These should not panic
    logging::log_always("Test always message");
    logging::log_verbose("Test verbose message");

    // Initialize with non-verbose mode
    logging::init(false);

    // These should also not panic
    logging::log_always("Test always message");
    logging::log_verbose("Test verbose message");
}
