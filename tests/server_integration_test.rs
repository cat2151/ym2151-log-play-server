//! Server Integration Tests
//!
//! These tests validate the server functionality including playback control
//! and command processing via named pipes.

mod test_util_server_mutex;

/// Server integration tests for playback control
#[cfg(windows)]
mod server_playback_tests {
    use std::thread;
    use std::time::Duration;
    use ym2151_log_play_server::ipc::pipe_windows::NamedPipe;
    use ym2151_log_play_server::ipc::protocol::Command;
    use ym2151_log_play_server::server::Server;

    // Import test utilities from the parent module
    use super::test_util_server_mutex::server_test_lock;

    /// Test server can start in idle state and accept PLAY command
    #[test]
    fn test_server_play_command() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();
        eprintln!("Starting server PLAY command test...");

        let server = Server::new();

        // Start server in a separate thread
        let server_handle = thread::spawn(move || {
            eprintln!("Server thread starting...");
            let result = server.run();
            eprintln!("Server thread finished with result: {:?}", result);
            result
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(500));

        eprintln!("Attempting to send PLAY command...");

        // Send PLAY command to play JSON data using binary protocol
        // Use a scope to ensure the pipe connection is closed before opening a new one
        {
            let mut writer =
                NamedPipe::connect_default().expect("Failed to connect to server for PLAY command");
            eprintln!("Connected to server, sending PlayJson command...");

            // Read and send JSON data
            let json_content = std::fs::read_to_string("output_ym2151.json")
                .expect("Failed to read output_ym2151.json");

            let json_data: serde_json::Value = serde_json::from_str(&json_content)
                .expect("Failed to parse JSON from output_ym2151.json");

            let cmd = Command::PlayJson { data: json_data };
            let binary_data = cmd
                .to_binary()
                .expect("Failed to serialize PlayJson command");
            writer
                .write_binary(&binary_data)
                .expect("Failed to send PlayJson command to server");
            eprintln!("PlayJson command sent successfully");
        } // writer is dropped here, releasing the pipe connection

        // Wait a bit for the new file to start playing
        thread::sleep(Duration::from_millis(500));

        eprintln!("Sending shutdown command...");

        // Send shutdown using binary protocol
        {
            let mut writer =
                NamedPipe::connect_default().expect("Failed to connect to server for shutdown");
            let cmd = Command::Shutdown;
            let binary_data = cmd
                .to_binary()
                .expect("Failed to serialize Shutdown command");
            writer
                .write_binary(&binary_data)
                .expect("Failed to send Shutdown command to server");
            eprintln!("Shutdown command sent successfully");
        } // writer is dropped here

        // Wait for server to finish
        thread::sleep(Duration::from_millis(500));

        // Wait for server thread and verify it shuts down cleanly
        match server_handle.join() {
            Ok(_) => eprintln!("Server thread finished successfully"),
            Err(e) => {
                eprintln!("Server thread panicked: {:?}", e);
                panic!("Server thread should not panic");
            }
        }

        eprintln!("Test complete");
    }

    /// Test server STOP command
    #[test]
    fn test_server_stop_command() {
        // Acquire lock to prevent parallel execution of server tests
        let _lock = server_test_lock();
        eprintln!("Starting server STOP command test...");

        let server = Server::new();

        // Start server
        let server_handle = thread::spawn(move || {
            eprintln!("Server thread starting...");
            server.run()
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(500));

        eprintln!("Attempting to send STOP command...");

        // Send STOP command using binary protocol
        // Use a scope to ensure the pipe connection is closed before opening a new one
        {
            let mut writer =
                NamedPipe::connect_default().expect("Failed to connect to server for STOP command");
            eprintln!("Connected to server, sending STOP command...");
            let cmd = Command::Stop;
            let binary_data = cmd.to_binary().expect("Failed to serialize STOP command");
            writer
                .write_binary(&binary_data)
                .expect("Failed to send STOP command to server");
            eprintln!("STOP command sent successfully");
        } // writer is dropped here, releasing the pipe connection

        // Wait a bit
        thread::sleep(Duration::from_millis(300));

        eprintln!("Sending shutdown command...");

        // Send shutdown using binary protocol
        {
            let mut writer =
                NamedPipe::connect_default().expect("Failed to connect to server for shutdown");
            let cmd = Command::Shutdown;
            let binary_data = cmd
                .to_binary()
                .expect("Failed to serialize Shutdown command");
            writer
                .write_binary(&binary_data)
                .expect("Failed to send Shutdown command to server");
        } // writer is dropped here

        // Wait for server to finish
        thread::sleep(Duration::from_millis(500));

        // Wait for server thread and verify it shuts down cleanly
        match server_handle.join() {
            Ok(_) => eprintln!("Server thread finished successfully"),
            Err(e) => {
                eprintln!("Server thread panicked: {:?}", e);
                panic!("Server thread should not panic");
            }
        }

        eprintln!("Test complete");
    }
}
