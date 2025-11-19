//! Example demonstrating the clear_schedule function
//!
//! This example shows how to use clear_schedule to cancel scheduled events
//! and seamlessly transition between musical phrases without audio gaps.
//!
//! To run this example:
//! 1. Start the server: cargo run --release -- server
//! 2. In another terminal: cargo run --example clear_schedule_demo

#[cfg(windows)]
use std::thread;

#[cfg(windows)]
use ym2151_log_play_server::client;

#[cfg(windows)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Clear Schedule Demo - Seamless Phrase Transitions");
    println!("====================================================\n");

    // Enable verbose output
    client::init_client(true);

    // Check if server is already running
    println!("Checking if server is running...");
    if !client::is_server_running() {
        eprintln!("\n❌ エラー: サーバーが起動していません");
        eprintln!("\n先に別のターミナルでサーバーを起動してください:");
        eprintln!("  cargo run --release -- server --verbose");
        eprintln!("\nまたは:");
        eprintln!("  ym2151-log-play-server server --verbose");
        eprintln!("\nサーバー起動後、このdemoを再実行してください。");
        std::process::exit(1);
    }
    println!("✅ サーバーが起動しています\n");

    println!("✅ Starting interactive mode...\n");
    client::start_interactive()?;

    // Scenario: Play phrase 1, but then decide to cancel it and play phrase 2 instead

    println!("🎵 Scheduling Phrase 1 (long melody with many notes)...\n");

    // Phrase 1: A long melody scheduled over several seconds
    let phrase1 = r#"{
        "event_count": 8,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x28", "data": "0x48"},
            {"time": 2797, "addr": "0x30", "data": "0x00"},
            {"time": 2797, "addr": "0x08", "data": "0x78"},
            {"time": 30762, "addr": "0x08", "data": "0x00"},
            {"time": 33559, "addr": "0x28", "data": "0x4A"},
            {"time": 33559, "addr": "0x08", "data": "0x78"},
            {"time": 61524, "addr": "0x08", "data": "0x00"}
        ]
    }"#;
    client::play_json_interactive(phrase1)?;

    println!("⏳ Phrase 1 scheduled. Waiting a moment...");
    thread::sleep(std::time::Duration::from_millis(500));

    // Decision point: Cancel phrase 1 and play phrase 2 instead
    println!("\n🗑️  Change of plan! Clearing scheduled events for phrase 1...\n");
    client::clear_schedule()?;

    println!("🎵 Scheduling Phrase 2 (different melody) without audio gap...\n");

    // Phrase 2: A completely different melody
    let phrase2 = r#"{
        "event_count": 4,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x28", "data": "0x50"},
            {"time": 2797, "addr": "0x08", "data": "0x78"},
            {"time": 20000, "addr": "0x08", "data": "0x00"}
        ]
    }"#;
    client::play_json_interactive(phrase2)?;

    println!("⏳ Letting phrase 2 play...");
    thread::sleep(std::time::Duration::from_millis(1000));

    // Demonstrate multiple clear and schedule operations
    println!("\n🎵 Scheduling Phrase 3...\n");
    let phrase3 = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x28", "data": "0x55"},
            {"time": 0, "addr": "0x08", "data": "0x78"}
        ]
    }"#;
    client::play_json_interactive(phrase3)?;

    println!("⏳ Wait briefly...");
    thread::sleep(std::time::Duration::from_millis(200));

    println!("\n🗑️  Actually, let's clear this one too and end with phrase 4!\n");
    client::clear_schedule()?;

    // Final phrase
    let phrase4 = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x28", "data": "0x4C"},
            {"time": 0, "addr": "0x08", "data": "0x78"}
        ]
    }"#;
    client::play_json_interactive(phrase4)?;

    println!("⏳ Waiting for final phrase to complete...");
    thread::sleep(std::time::Duration::from_millis(1500));

    println!("\n⏹️  Stopping interactive mode...");
    client::stop_interactive()?;

    println!("\n✅ Demo complete!");
    println!("\n🎯 Key use cases for clear_schedule:");
    println!("  • Cancel scheduled musical phrases");
    println!("  • Respond to user input (e.g., button press changes melody)");
    println!("  • Dynamic music generation based on game state");
    println!("  • Seamless transitions without audio gaps");
    println!("  • Interactive tone editor undo functionality");
    println!("  • Real-time music composition tools");
    println!("\n💡 Important notes:");
    println!("  • clear_schedule only removes FUTURE events");
    println!("  • Events already played cannot be cleared");
    println!("  • No audio gaps between clear and new schedule");
    println!("  • Perfect for interactive applications!");

    Ok(())
}

#[cfg(not(windows))]
fn main() {
    println!("This example requires Windows (named pipe support)");
    println!("On Unix systems, the server/client features are not enabled.");
}
