//! Example demonstrating the play_json_interactive convenience function
//!
//! This example shows how to use the high-level play_json_interactive function
//! to play ym2151log format JSON data in interactive mode without manually
//! handling the conversion and timing logic.
//!
//! To run this example:
//! 1. Start the server: cargo run --release -- server
//! 2. In another terminal: cargo run --example play_json_interactive_demo

#[cfg(windows)]
use ym2151_log_play_server::client;

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 play_json_interactive Convenience Function Demo");
    println!("==================================================\n");

    // Enable verbose output to see what's happening
    client::init_client(true);

    // Ensure server is running
    println!("Ensuring server is ready...");
    client::ensure_server_ready("ym2151-log-play-server")?;

    println!("\n✅ Using play_json_interactive convenience function...\n");

    // Create a simple melody in ym2151log format
    // Time values are in YM2151 sample units (at 55930 Hz)
    // 55930 samples = 1 second
    // 2797 samples = ~50ms
    // 5593 samples = ~100ms
    let json_data = r#"{
        "event_count": 10,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x28", "data": "0x48"},
            {"time": 2797, "addr": "0x30", "data": "0x00"},
            {"time": 2797, "addr": "0x08", "data": "0x78"},
            {"time": 30762, "addr": "0x08", "data": "0x00"},
            {"time": 33559, "addr": "0x28", "data": "0x4A"},
            {"time": 33559, "addr": "0x08", "data": "0x78"},
            {"time": 61524, "addr": "0x08", "data": "0x00"},
            {"time": 67117, "addr": "0x28", "data": "0x4C"},
            {"time": 67117, "addr": "0x08", "data": "0x78"}
        ]
    }"#;

    println!("📝 Sending JSON data to server via play_json_interactive...\n");

    // This single function call:
    // 1. Parses the JSON
    // 2. Validates the event log
    // 3. Starts interactive mode
    // 4. Converts time stamps to time offsets
    // 5. Sends all register writes with proper timing
    client::play_json_interactive(json_data)?;

    println!("\n⏳ Waiting for playback to finish...");
    std::thread::sleep(std::time::Duration::from_millis(2500));

    println!("⏹️  Stopping interactive mode...");
    client::stop_interactive()?;

    println!("\n✅ Demo complete!");
    println!("\nKey benefits of play_json_interactive:");
    println!("  • Single function call instead of multiple steps");
    println!("  • Automatic JSON parsing and validation");
    println!("  • Automatic time conversion (samples → seconds)");
    println!("  • No need to manually handle event iteration");
    println!("  • Reduces code duplication across client applications");

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (named pipe support)");
    println!("On Unix systems, the server/client features are not enabled.");
}
