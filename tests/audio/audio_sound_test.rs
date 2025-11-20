//! Comprehensive audio sound playback tests
//!
//! This file contains all audio sound playback tests organized by category:
//! - ✅ GREEN TESTS: Reliable reference implementations (known to work)
//! - 🧪 INTEGRATION TESTS: Interactive mode and advanced functionality
//! - 🔧 LIFECYCLE TESTS: Server management and state transitions
//!
//! All tests use ensure_server_ready pattern and run sequentially to avoid conflicts.

use super::super::test_util_server_mutex;

#[cfg(windows)]
mod audio_sound_tests {
    use ym2151_log_play_server::client;

    use super::test_util_server_mutex::server_test_lock;    /// Helper function to clean up test logs
    fn cleanup_test_logs() {
        let _ = std::fs::remove_file("test_client.log");
        let _ = std::fs::remove_file("test_server.log");
        let _ = std::fs::remove_file("ym2151-server.log");
    }

    /// Helper function to read and display test logs for debugging
    #[allow(dead_code)]
    fn display_test_logs() {
        if let Ok(client_log) = std::fs::read_to_string("test_client.log") {
            println!("📄 test_client.log contents:\n{}", client_log);
        }
        if let Ok(server_log) = std::fs::read_to_string("test_server.log") {
            println!("📄 test_server.log contents:\n{}", server_log);
        }
    }

    // ========================================
    // ✅ GREEN TESTS (Known working reference implementations)
    // ========================================

    /// ✅ GREEN TEST: Basic JSON sound playback - REFERENCE IMPLEMENTATION
    ///
    /// This is the most reliable test that serves as a reference for:
    /// 1. ensure_server_ready pattern
    /// 2. JSON file playback
    /// 3. Proper cleanup procedures
    ///
    /// Expected result: ✅ Always passes and makes sound
    #[test]
    fn test_green_basic_json_playback() {
        let _guard = server_test_lock();
        cleanup_test_logs();

        client::init_client(true);
        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(1000));

        client::ensure_server_ready("ym2151-log-play-server")
            .expect("✅ Should prepare server automatically");

        std::thread::sleep(std::time::Duration::from_millis(1000));

        let json_data = std::fs::read_to_string("output_ym2151.json")
            .expect("✅ Should read output_ym2151.json");

        client::send_json(&json_data)
            .expect("✅ Should send JSON data to server");

        std::thread::sleep(std::time::Duration::from_millis(2000));

        let _ = client::stop_playback();
        let _ = client::shutdown_server();
        display_test_logs();
        cleanup_test_logs();

        println!("✅ GREEN: Basic JSON playback completed successfully");
    }

    /// ✅ GREEN TEST: Server lifecycle management
    ///
    /// This test verifies the server lifecycle operations work correctly:
    /// 1. ensure_server_ready works
    /// 2. Server shutdown works
    /// 3. Server restart works
    #[test]
    fn test_green_server_lifecycle() {
        let _guard = server_test_lock();
        cleanup_test_logs();

        client::init_client(true);
        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(1000));

        client::ensure_server_ready("ym2151-log-play-server")
            .expect("✅ Should start server");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        client::shutdown_server()
            .expect("✅ Should shutdown server");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        client::ensure_server_ready("ym2151-log-play-server")
            .expect("✅ Should restart server");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let _ = client::shutdown_server();
        cleanup_test_logs();

        println!("✅ GREEN: Server lifecycle test completed successfully");
    }

    // ========================================
    // 🧪 INTEGRATION TESTS (Interactive mode and advanced functionality)
    // ========================================

    /// 🧪 Test interactive mode with existing JSON data
    ///
    /// This test verifies that interactive mode works correctly by
    /// sending existing output_ym2151.json data to create sound.
    #[test]
    fn test_interactive_mode_manual_registers() {
        let _guard = server_test_lock();
        cleanup_test_logs();

        client::init_client(true);
        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(2000));

        client::ensure_server_ready("ym2151-log-play-server")
            .expect("Should prepare server");
        std::thread::sleep(std::time::Duration::from_millis(2000));

        client::start_interactive()
            .expect("Should start interactive mode");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        // Use existing output_ym2151.json for register sequence
        let json_data = std::fs::read_to_string("output_ym2151.json")
            .expect("Should read output_ym2151.json");

        client::play_json_interactive(&json_data)
            .expect("Should send JSON register sequence to interactive mode");

        std::thread::sleep(std::time::Duration::from_millis(2000));

        client::stop_interactive()
            .expect("Should stop interactive mode");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(2000));
        display_test_logs();
        cleanup_test_logs();

        println!("🧪 Interactive mode existing JSON data test completed");
    }

    /// 🧪 Test JSON playback through interactive mode
    ///
    /// This test uses the play_json_interactive convenience function
    /// to verify that JSON data can be played through interactive mode.
    #[test]
    fn test_interactive_mode_json_playback() {
        let _guard = server_test_lock();
        cleanup_test_logs();

        client::init_client(true);
        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(3000));

        client::ensure_server_ready("ym2151-log-play-server")
            .expect("Should prepare server");
        std::thread::sleep(std::time::Duration::from_millis(2000));

        client::start_interactive()
            .expect("Should start interactive mode");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let json_data = std::fs::read_to_string("output_ym2151.json")
            .expect("Should read output_ym2151.json");

        client::play_json_interactive(&json_data)
            .expect("Should play JSON in interactive mode");

        std::thread::sleep(std::time::Duration::from_millis(3000));

        client::stop_interactive()
            .expect("Should stop interactive mode");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(3000));
        display_test_logs();
        cleanup_test_logs();

        println!("🧪 Interactive mode JSON playback test completed");
    }
}

#[cfg(not(windows))]
mod non_windows_tests {
    /// Placeholder test for non-Windows platforms
    #[test]
    fn test_audio_sound_windows_only() {
        // Audio sound tests are Windows-only in this project
        assert!(true, "Audio sound tests are Windows-only");
    }
}
