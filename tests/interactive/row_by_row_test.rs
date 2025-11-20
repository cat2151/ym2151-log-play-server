//! Tests for sending output_ym2151.json row by row in interactive mode
//!
//! This test file verifies that output_ym2151.json events can be sent
//! one by one to the server in interactive mode, simulating row-by-row
//! real-time playback scenarios.

#[cfg(windows)]
mod windows_tests {
    use ym2151_log_play_server::client;
    use ym2151_log_play_server::events::{RegisterEvent, EventLog};    /// Load output_ym2151.json and extract events for testing
    fn load_output_ym2151_events() -> Result<Vec<RegisterEvent>, anyhow::Error> {
        let json_path = std::path::Path::new("output_ym2151.json");
        if !json_path.exists() {
            return Err(anyhow::anyhow!("output_ym2151.json not found in workspace root"));
        }

        let json_content = std::fs::read_to_string(json_path)?;
        let event_log = EventLog::from_json_str(&json_content)?;

        if !event_log.validate() {
            return Err(anyhow::anyhow!("output_ym2151.json validation failed"));
        }

        Ok(event_log.events)
    }

    #[test]
    fn test_output_ym2151_json_exists_and_parseable() {
        // First, verify that output_ym2151.json exists and can be parsed
        let events = load_output_ym2151_events();
        assert!(events.is_ok(), "Failed to load output_ym2151.json: {:?}", events.err());

        let events = events.unwrap();
        assert!(!events.is_empty(), "output_ym2151.json should contain events");

        // Log some basic info about the file
        println!("📊 output_ym2151.json contains {} events", events.len());
        if !events.is_empty() {
            println!("  First event: time={}, addr=0x{:02X}, data=0x{:02X}",
                events[0].time, events[0].addr, events[0].data);
            println!("  Last event:  time={}, addr=0x{:02X}, data=0x{:02X}",
                events[events.len()-1].time, events[events.len()-1].addr, events[events.len()-1].data);
        }
    }

    #[test]
    fn test_send_output_ym2151_row_by_row_parse_only() {
        // Test that we can convert each event to individual JSON and parse it back
        let events = load_output_ym2151_events().expect("Failed to load output_ym2151.json");

        for (index, event) in events.iter().enumerate() {
            // Create single-event JSON
            let single_event_json = format!(
                r#"{{"event_count": 1, "events": [{{"time": {}, "addr": "0x{:02X}", "data": "0x{:02X}"}}]}}"#,
                event.time, event.addr, event.data
            );

            // Verify it parses correctly
            let parsed = EventLog::from_json_str(&single_event_json);
            assert!(parsed.is_ok(), "Failed to parse single event JSON for event {}: {:?}", index, parsed.err());

            let parsed = parsed.unwrap();
            assert_eq!(parsed.event_count, 1);
            assert_eq!(parsed.events.len(), 1);
            assert_eq!(parsed.events[0].time, event.time);
            assert_eq!(parsed.events[0].addr, event.addr);
            assert_eq!(parsed.events[0].data, event.data);
            assert!(parsed.validate());
        }

        println!("✅ All {} events can be converted to valid single-event JSON", events.len());
    }

    #[test]
    fn test_send_output_ym2151_row_by_row_interactive_real() {
        // Test row-by-row sending with actual server (real playback mode)
        // This follows the same pattern as test_interactive_mode_json_playback

        // Use the shared mutex to prevent parallel test execution across ALL interactive tests
        let _guard = super::super::shared_mutex::lock_interactive_test();

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

        // Load output_ym2151.json events
        let events = load_output_ym2151_events().expect("Failed to load output_ym2151.json");

        println!("🎮 Row-by-row sending of {} events from output_ym2151.json", events.len());

        // Send each event as individual register write
        for (index, event) in events.iter().enumerate() {
            // Convert sample time to seconds (same as play_json_interactive does)
            let time_offset_sec = event.time as f64 / 55930.0; // OPM_SAMPLE_RATE

            println!("📝 [レジスタ書き込み] offset={:.6}s, addr=0x{:02X}, data=0x{:02X}",
                time_offset_sec, event.addr, event.data);

            // Send register write command
            let json_data = format!(
                r#"{{"event_count": 1, "events": [
                    {{"time": {}, "addr": "0x{:02X}", "data": "0x{:02X}"}}
                ]}}"#,
                (time_offset_sec * 1000.0) as u32, // Convert to milliseconds
                event.addr, event.data
            );
            client::play_json_interactive(&json_data)
                .expect(&format!("Should write register {}: addr=0x{:02X}, data=0x{:02X}",
                    index, event.addr, event.data));

            // Small delay to prevent pipe congestion (same as play_json_interactive)
            std::thread::sleep(std::time::Duration::from_millis(10));

            // Log progress every 10 events
            if index % 10 == 0 || index == events.len() - 1 {
                println!("  📝 Sent register write {}/{}: time={:.6}s, addr=0x{:02X}, data=0x{:02X}",
                    index + 1, events.len(), time_offset_sec, event.addr, event.data);
            }
        }

        println!("✅ All {} register writes sent successfully", events.len());

        // Wait for playback to complete (same duration as the original JSON)
        std::thread::sleep(std::time::Duration::from_millis(3000));

        client::stop_interactive()
            .expect("Should stop interactive mode");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let _ = client::shutdown_server();
        std::thread::sleep(std::time::Duration::from_millis(3000));
        display_test_logs();
        cleanup_test_logs();

        println!("🎮 Row-by-row interactive test completed successfully");
    }

    /// Helper function to clean up test logs
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

    #[test]
    fn test_output_ym2151_event_time_distribution() {
        // Analyze the time distribution of events for better understanding
        let events = load_output_ym2151_events().expect("Failed to load output_ym2151.json");

        if events.is_empty() {
            println!("⚠️  No events to analyze");
            return;
        }

        let mut time_values: Vec<u32> = events.iter().map(|e| e.time).collect();
        time_values.sort();

        let min_time = time_values[0];
        let max_time = time_values[time_values.len() - 1];
        let total_duration_sec = max_time as f64 / 55930.0; // OPM sample rate

        // Count unique time values
        time_values.dedup();
        let unique_times = time_values.len();

        println!("📊 Event time analysis:");
        println!("  Total events: {}", events.len());
        println!("  Unique times: {}", unique_times);
        println!("  Time range: {} - {} samples", min_time, max_time);
        println!("  Duration: {:.3} seconds", total_duration_sec);

        // Group events by time to see how many events happen simultaneously
        let mut events_by_time = std::collections::HashMap::new();
        for event in &events {
            let count = events_by_time.entry(event.time).or_insert(0);
            *count += 1;
        }

        let mut simultaneous_counts: Vec<_> = events_by_time.values().cloned().collect();
        simultaneous_counts.sort();
        simultaneous_counts.dedup();

        println!("  Simultaneous events per time: {:?}", simultaneous_counts);

        // This info helps understand the complexity of row-by-row processing
        assert!(total_duration_sec >= 0.0);
        assert!(events.len() > 0);
        assert!(unique_times > 0);
    }

    #[test]
    fn test_output_ym2151_register_usage_analysis() {
        // Analyze which registers are used in output_ym2151.json
        let events = load_output_ym2151_events().expect("Failed to load output_ym2151.json");

        let mut register_usage = std::collections::HashMap::new();
        let mut data_usage = std::collections::HashMap::new();

        for event in &events {
            *register_usage.entry(event.addr).or_insert(0) += 1;
            *data_usage.entry(event.data).or_insert(0) += 1;
        }

        // Sort by register address
        let mut registers: Vec<_> = register_usage.keys().cloned().collect();
        registers.sort();

        println!("📊 Register usage analysis:");
        println!("  Registers used: {} different", registers.len());
        for &addr in &registers {
            let count = register_usage[&addr];
            println!("    0x{:02X}: {} times", addr, count);
        }

        // Sort data values by frequency
        let mut data_freq: Vec<_> = data_usage.iter().collect();
        data_freq.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));

        println!("  Most common data values:");
        for (data, count) in data_freq.iter().take(10) {
            println!("    0x{:02X}: {} times", data, count);
        }

        // This analysis helps understand the content we're sending row by row
        assert!(!registers.is_empty());
        assert!(!data_usage.is_empty());
    }
}

#[cfg(not(windows))]
mod non_windows_tests {
    #[test]
    fn test_row_by_row_not_available_on_non_windows() {
        // On non-Windows platforms, the interactive client functionality is not available
        // This test just documents that behavior
        println!("ℹ️  Row-by-row testing requires Windows platform for named pipe support");
        assert!(true);
    }
}

// Cross-platform test for JSON structure compatibility
#[test]
fn test_output_ym2151_json_structure() {
    // Test that output_ym2151.json has the expected structure (cross-platform)
    use ym2151_log_play_server::events::EventLog;

    let json_path = std::path::Path::new("output_ym2151.json");
    if !json_path.exists() {
        println!("⚠️  output_ym2151.json not found, skipping structure test");
        return;
    }

    let json_content = std::fs::read_to_string(json_path)
        .expect("Failed to read output_ym2151.json");

    // Parse and validate
    let event_log = EventLog::from_json_str(&json_content)
        .expect("Failed to parse output_ym2151.json");

    assert!(event_log.validate(), "output_ym2151.json validation failed");

    // Verify structure
    assert_eq!(event_log.events.len(), event_log.event_count as usize);

    // Verify all events have valid timestamps
    for (index, event) in event_log.events.iter().enumerate() {
        // Time should be reasonable (not absurdly large)
        assert!(event.time <= 1_000_000_000, "Event {}: Time {} seems too large", index, event.time);
    }

    println!("✅ output_ym2151.json structure is valid for row-by-row processing");
}
