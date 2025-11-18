//! Example demonstrating the play_json_interactive convenience function
//!
//! This example shows how to use the play_json_interactive function to send
//! ym2151log format JSON data to interactive mode. The function handles JSON
//! parsing and time conversion, allowing you to send multiple JSONs continuously
//! without audio gaps.
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

    println!("\n✅ Starting interactive mode...\n");

    // Start interactive mode once
    client::start_interactive()?;

    // Send multiple JSONs without stopping - no audio gaps!
    println!("📝 Sending first melody via play_json_interactive...\n");

    // First melody
    let json1 = r#"{
        "event_count": 5,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x28", "data": "0x48"},
            {"time": 2797, "addr": "0x30", "data": "0x00"},
            {"time": 2797, "addr": "0x08", "data": "0x78"},
            {"time": 30762, "addr": "0x08", "data": "0x00"}
        ]
    }"#;
    client::play_json_interactive(json1)?;

    println!("📝 Sending second melody (seamlessly continues)...\n");

    // Second melody - continues seamlessly without audio gap
    let json2 = r#"{
        "event_count": 3,
        "events": [
            {"time": 33559, "addr": "0x28", "data": "0x4A"},
            {"time": 33559, "addr": "0x08", "data": "0x78"},
            {"time": 61524, "addr": "0x08", "data": "0x00"}
        ]
    }"#;
    client::play_json_interactive(json2)?;

    println!("📝 Sending third melody (seamlessly continues)...\n");

    // Third melody - continues seamlessly
    let json3 = r#"{
        "event_count": 2,
        "events": [
            {"time": 67117, "addr": "0x28", "data": "0x4C"},
            {"time": 67117, "addr": "0x08", "data": "0x78"}
        ]
    }"#;
    client::play_json_interactive(json3)?;

    println!("\n⏳ Waiting for playback to finish...");
    std::thread::sleep(std::time::Duration::from_millis(2500));

    println!("⏹️  Stopping interactive mode...");
    client::stop_interactive()?;

    println!("\n✅ Demo complete!");
    println!("\nKey benefits of play_json_interactive:");
    println!("  • No start/stop of interactive mode between JSONs");
    println!("  • Continuous playback without audio gaps");
    println!("  • Automatic JSON parsing and validation");
    println!("  • Automatic time conversion (samples → seconds)");
    println!("  • Client controls interactive mode lifecycle");
    println!("  • Perfect for dynamic music generation and tone editing");

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (named pipe support)");
    println!("On Unix systems, the server/client features are not enabled.");
}
