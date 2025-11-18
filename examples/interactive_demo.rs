//! Example demonstrating interactive mode
//!
//! This example shows how to use the interactive mode to continuously
//! stream register writes to the YM2151 chip without stopping playback.
//!
//! To run this example:
//! 1. Start the server: cargo run --release -- server
//! 2. In another terminal: cargo run --example interactive_demo

#[cfg(windows)]
use ym2151_log_play_server::client;

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Interactive Mode Demo");
    println!("========================\n");

    // Enable verbose output to see what's happening
    client::init_client(true);

    // Ensure server is running
    println!("Ensuring server is ready...");
    client::ensure_server_ready("ym2151-log-play-server")?;

    println!("\n✅ Starting interactive mode...");
    client::start_interactive()?;

    println!("📝 Sending register writes...\n");

    // Initialize all channels to silent
    for ch in 0..8 {
        client::write_register(0.0, 0x08, ch)?; // Key off all channels
    }

    // Simple melody: play a few notes with timing (using f64 seconds)
    let notes = vec![
        (0.0, 0x28, 0x48),     // Note C4, channel 0
        (0.100, 0x30, 0x00),   // Octave and note on at 100ms
        (0.100, 0x08, 0x78),   // Key on channel 0
        (0.500, 0x08, 0x00),   // Key off at 500ms
        (0.600, 0x28, 0x4A),   // Note D4 at 600ms
        (0.600, 0x08, 0x78),   // Key on
        (1.100, 0x08, 0x00),   // Key off at 1100ms
        (1.200, 0x28, 0x4C),   // Note E4 at 1200ms
        (1.200, 0x08, 0x78),   // Key on
        (1.700, 0x08, 0x00),   // Key off at 1700ms
    ];

    for (time_sec, addr, data) in notes {
        println!(
            "  Writing register 0x{:02X} = 0x{:02X} at +{:.3}s",
            addr, data, time_sec
        );
        client::write_register(time_sec, addr, data)?;
    }

    println!("\n⏳ Waiting for playback to finish...");
    std::thread::sleep(std::time::Duration::from_millis(2000));

    println!("⏹️  Stopping interactive mode...");
    client::stop_interactive()?;

    println!("✅ Demo complete!");
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (named pipe support)");
    println!("On Unix systems, the server/client features are not enabled.");
}
