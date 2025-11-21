//! STEP-BY-STEP INTERACTIVE MODE TESTS
//!
//! These tests break down interactive mode functionality into small,
//! manageable steps following t_wada style TDD approach.

#[cfg(windows)]
mod interactive_step_tests {
    use ym2151_log_play_server::client;

    /// Helper function to clean up test logs
    fn cleanup_test_logs() {
        let _ = std::fs::remove_file("test_client.log");
        let _ = std::fs::remove_file("test_server.log");
        let _ = std::fs::remove_file("ym2151-server.log");
    }

    /// Helper function to display test logs for debugging
    #[allow(dead_code)]
    fn display_test_logs() {
        if let Ok(client_log) = std::fs::read_to_string("test_client.log") {
            println!("📄 test_client.log contents:\n{}", client_log);
        }
        if let Ok(server_log) = std::fs::read_to_string("test_server.log") {
            println!("📄 test_server.log contents:\n{}", server_log);
        }
    }

    /// STEP 1: Test that interactive mode can be started
    ///
    /// Based on the logs, this step should succeed
    #[test]
    fn test_step1_start_interactive_mode() {
        // Use shared mutex to prevent conflicts with other interactive tests
        let _guard = super::super::shared_mutex::lock_interactive_test();
        cleanup_test_logs();

        client::init_client(true);

        // Clean state
        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(1000));

        // Start server
        client::ensure_server_ready("ym2151-log-play-server").expect("Should prepare server");

        std::thread::sleep(std::time::Duration::from_millis(1000));

        // Start interactive mode
        client::start_interactive().expect("Should start interactive mode");

        std::thread::sleep(std::time::Duration::from_millis(500));

        // Stop interactive mode immediately
        client::stop_interactive().expect("Should stop interactive mode");

        // Clean up
        let _ = client::shutdown_server();

        display_test_logs();
        cleanup_test_logs();

        println!("✅ STEP 1: Interactive mode start/stop succeeded");
    }

    /// STEP 2: Test that first register write works
    ///
    /// Based on the logs, the first register write (0x08, 0x00) succeeds
    #[test]
    fn test_step2_first_register_write() {
        // Use shared mutex to prevent conflicts with other interactive tests
        let _guard = super::super::shared_mutex::lock_interactive_test();
        cleanup_test_logs();

        client::init_client(true);

        // Clean state
        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Start server
        client::ensure_server_ready("ym2151-log-play-server").expect("Should prepare server");

        std::thread::sleep(std::time::Duration::from_millis(100));

        // Start interactive mode
        client::start_interactive().expect("Should start interactive mode");

        std::thread::sleep(std::time::Duration::from_millis(100));

        // First register write only (this should succeed based on logs)
        let json_data = r#"{"events": [
            {"time": 0, "addr": "0x08", "data": "0x00"}
        ]}"#;
        client::play_json_interactive(json_data).expect("Should write first register successfully");

        std::thread::sleep(std::time::Duration::from_millis(100));

        // Stop interactive mode
        client::stop_interactive().expect("Should stop interactive mode");

        // Clean up
        let _ = client::shutdown_server();

        display_test_logs();
        cleanup_test_logs();

        println!("✅ STEP 2: First register write succeeded");
    }

    /// STEP 3: Test where second register write fails
    ///
    /// Based on the logs, the second register write fails with "pipe busy"
    /// This should help identify the exact failure point
    #[test]
    fn test_step3_second_register_write_expected_fail() {
        // Use shared mutex to prevent conflicts with other interactive tests
        let _guard = super::super::shared_mutex::lock_interactive_test();
        cleanup_test_logs();

        client::init_client(true);

        // Clean state
        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Start server
        client::ensure_server_ready("ym2151-log-play-server").expect("Should prepare server");

        std::thread::sleep(std::time::Duration::from_millis(100));

        // Start interactive mode
        client::start_interactive().expect("Should start interactive mode");

        std::thread::sleep(std::time::Duration::from_millis(100));

        // First register write (should succeed)
        let json_data = r#"{"events": [
            {"time": 0, "addr": "0x08", "data": "0x00"}
        ]}"#;
        client::play_json_interactive(json_data).expect("Should write first register successfully");

        std::thread::sleep(std::time::Duration::from_millis(100));

        // Second register write (this might fail based on logs)
        let json_data2 = r#"{"events": [
            {"time": 50, "addr": "0x20", "data": "0xC7"}
        ]}"#;
        let result = client::play_json_interactive(json_data2);

        match result {
            Ok(_) => {
                println!("✅ STEP 3: Second register write unexpectedly succeeded!");

                // If it worked, try to clean up properly
                let _ = client::stop_interactive();
            }
            Err(e) => {
                println!("❌ STEP 3: Second register write failed as expected: {}", e);

                // Still try to clean up
                let _ = client::stop_interactive();
            }
        }

        // Clean up
        let _ = client::shutdown_server();

        display_test_logs();
        cleanup_test_logs();

        println!("✅ STEP 3: Second register write test completed");
    }
}

#[cfg(not(windows))]
mod non_windows_tests {
    #[test]
    fn test_interactive_step_windows_only() {
        assert!(true, "Interactive step tests are Windows-only");
    }
}
