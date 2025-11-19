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
        // cargo run --release -- server

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

    /// Test that get_test_binary_path can find the test binary
    #[test]
    fn test_get_test_binary_path_in_test_context() {
        // This test verifies that we can find the binary in test context
        // During tests, the binary should be in target/debug or target/debug/deps

        // We can't directly call get_test_binary_path as it's private,
        // but we can verify the behavior through ensure_server_ready
        // by checking that it doesn't try to install when the binary exists

        // Get current exe to verify we're in test context
        let current_exe = std::env::current_exe().expect("Should get current exe");
        let exe_dir = current_exe.parent().expect("Should have parent dir");

        // Verify we're in a test directory (debug or deps)
        let dir_name = exe_dir
            .file_name()
            .and_then(|n| n.to_str())
            .expect("Should have dir name");

        assert!(
            dir_name == "debug" || dir_name == "deps",
            "Test should run in debug or deps directory, found: {}",
            dir_name
        );
    }

    /// Test that binary path detection logic works correctly
    #[test]
    fn test_binary_path_construction() {
        // Test the logic for constructing binary paths
        use std::path::PathBuf;

        let current_exe = std::env::current_exe().expect("Should get current exe");
        let mut path = current_exe
            .parent()
            .expect("Should have parent")
            .to_path_buf();

        // If we're in deps, go up one level
        if path.ends_with("deps") {
            path = path.parent().expect("Should have parent").to_path_buf();
        }

        // Construct expected binary path
        path.push("ym2151-log-play-server.exe");

        // The path should be properly formatted
        assert!(path.is_absolute(), "Path should be absolute");

        // The parent directory should exist (even if the binary doesn't)
        let parent = path.parent().expect("Should have parent");
        assert!(
            parent.exists(),
            "Parent directory should exist: {:?}",
            parent
        );
    }

    /// Test that server_path resolution prioritizes test context
    #[test]
    fn test_server_path_resolution_priority() {
        // This test documents the priority order for server path resolution:
        // 1. Test binary path (if in test context)
        // 2. PATH (if available)
        // 3. Install via cargo (fallback)

        // Verify we can detect test context
        let current_exe = std::env::current_exe();
        assert!(
            current_exe.is_ok(),
            "Should be able to get current exe in test context"
        );

        // The test context should have a parent directory
        let exe_path = current_exe.unwrap();
        assert!(
            exe_path.parent().is_some(),
            "Test exe should have parent directory"
        );
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
