//! Integration tests for ensure_server_ready functionality
//!
//! These tests verify that the ensure_server_ready function provides
//! a good developer experience for library users.

#[cfg(windows)]
mod ensure_server_ready_tests {

    use ym2151_log_play_server::client;

    /// Test that ensure_server_ready works when server is already running
    #[test]
    #[ignore] // Requires manual server to be running
    fn test_ensure_server_ready_with_running_server() {
        // This test demonstrates the usage pattern when server is already running
        // Start a server manually before running this test:
        // cargo run --release -- --server

        let result = client::ensure_server_ready("ym2151-log-play-server");
        assert!(result.is_ok(), "Should succeed when server is running");
    }

    /// Test documentation example
    #[test]
    #[ignore] // Requires Windows and proper setup
    fn test_documentation_example() {
        // This test demonstrates the documented usage pattern
        // It should work seamlessly without any manual setup

        // Ensure server is ready (installs and starts if needed)
        client::ensure_server_ready("cat-play-mml").unwrap();

        // Now you can play files immediately
        // client::play_file("music.json").unwrap();

        // Cleanup
        let _ = client::shutdown_server();
    }
}

#[cfg(not(windows))]
mod non_windows_tests {
    /// Placeholder test for non-Windows platforms
    #[test]
    fn test_windows_only_feature() {
        // ensure_server_ready is a Windows-only feature
        // This test exists to document that behavior
        assert!(true, "ensure_server_ready is Windows-only");
    }
}
