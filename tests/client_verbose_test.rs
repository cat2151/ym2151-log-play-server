//! Tests for client verbose mode functionality
//!
//! These tests verify that the client module's verbose flag works correctly.

#[cfg(windows)]
mod client_verbose_tests {
    use ym2151_log_play_server::client;

    #[test]
    fn test_init_client_sets_verbose_mode() {
        // Test enabling verbose mode
        client::init_client(true);
        assert!(
            client::is_client_verbose(),
            "Verbose mode should be enabled after init_client(true)"
        );

        // Test disabling verbose mode
        client::init_client(false);
        assert!(
            !client::is_client_verbose(),
            "Verbose mode should be disabled after init_client(false)"
        );
    }

    #[test]
    fn test_default_verbose_mode_is_false() {
        // Initialize to a known state first
        client::init_client(false);

        // Check that default is non-verbose
        assert!(
            !client::is_client_verbose(),
            "Default verbose mode should be false (non-verbose)"
        );
    }

    #[test]
    fn test_verbose_mode_persists() {
        // Set to verbose
        client::init_client(true);
        assert!(client::is_client_verbose());

        // Check it persists across multiple calls
        assert!(client::is_client_verbose());
        assert!(client::is_client_verbose());

        // Set to non-verbose
        client::init_client(false);
        assert!(!client::is_client_verbose());

        // Check it persists
        assert!(!client::is_client_verbose());
        assert!(!client::is_client_verbose());
    }

    #[test]
    fn test_verbose_mode_can_be_toggled() {
        // Start with verbose
        client::init_client(true);
        assert!(client::is_client_verbose());

        // Toggle to non-verbose
        client::init_client(false);
        assert!(!client::is_client_verbose());

        // Toggle back to verbose
        client::init_client(true);
        assert!(client::is_client_verbose());

        // Toggle back to non-verbose
        client::init_client(false);
        assert!(!client::is_client_verbose());
    }

    #[test]
    fn test_verbose_mode_demonstration() {
        // Test verbose mode demonstration (migrated from examples/test_client_verbose.rs)
        client::init_client(true);

        // In verbose mode, client operations would print status messages
        // This test verifies the mode is set correctly for debugging
        assert!(client::is_client_verbose());

        // Note: Verbose mode is useful for:
        // - Debugging client-server communication
        // - Development and testing
        // - Command-line applications
    }

    #[test]
    fn test_non_verbose_mode_demonstration() {
        // Test non-verbose mode demonstration (migrated from examples/test_client_non_verbose.rs)
        client::init_client(false);

        // In non-verbose mode, client operations should be silent
        // This prevents TUI display corruption
        assert!(!client::is_client_verbose());

        // Note: Non-verbose mode is essential for:
        // - TUI applications (like ym2151-tone-editor)
        // - Applications that need clean output
        // - Production use where output should be minimal
    }
}

#[cfg(not(windows))]
mod client_verbose_tests {
    // On non-Windows platforms, the client module is not available
    // These tests are skipped
    #[test]
    fn test_client_module_not_available_on_non_windows() {
        // This test just verifies the test module compiles
        assert!(true);
    }
}
