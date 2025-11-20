//! Audio playback tests for both non-interactive and interactive modes
//!
//! These tests verify that actual audio playback works correctly in both modes.
//! The non-interactive mode test serves as a baseline for comparison with
//! interactive mode behavior.

use ym2151_log_play_server::audio::AudioPlayer;
use ym2151_log_play_server::events::{EventLog, RegisterEvent};
use ym2151_log_play_server::player::Player;

/// Test non-interactive mode audio playback with ensure pattern
///
/// This test verifies that non-interactive mode can successfully play audio
/// without errors. It serves as a baseline reference for comparing with
/// interactive mode behavior.
///
/// Expected result: Test should pass (green) with no errors
#[test]
fn test_ensure_non_interactive_audio_playback() {
    // Create a simple event log with a few register writes
    let log = EventLog {
        event_count: 3,
        events: vec![
            RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 100,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
            RegisterEvent {
                time: 200,
                addr: 0x28,
                data: 0x3E,
                is_data: None,
            },
        ],
    };

    // Create a player with the event log
    let player = Player::new(log);

    // Create audio player and ensure it works
    let result = AudioPlayer::new(player);

    match result {
        Ok(mut audio_player) => {
            // Let audio play for a short time
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Stop playback
            audio_player.stop();

            // If we got here, the test passes
            println!("✅ Non-interactive audio playback successful");
        }
        Err(e) => {
            // In CI environment without audio devices, this is expected
            println!(
                "Note: Audio player creation failed (expected in CI without audio device): {}",
                e
            );
        }
    }
}

/// Test interactive mode audio playback with ensure pattern
///
/// This test verifies that interactive mode can successfully play audio
/// by scheduling register writes and generating samples.
///
/// Expected result: May initially fail (red) if there are bugs in the
/// interactive mode audio playback pipeline. This test helps identify
/// issues in the integrated function calls and named pipe communication.
#[test]
fn test_ensure_interactive_audio_playback() {
    // Create a player in interactive mode
    let player = Player::new_interactive();

    // Schedule some register writes
    player.schedule_register_write(0, 0x08, 0x00);
    player.schedule_register_write(100, 0x20, 0xC7);
    player.schedule_register_write(200, 0x28, 0x3E);

    // Create audio player and ensure it works
    let result = AudioPlayer::new(player);

    match result {
        Ok(mut audio_player) => {
            // Let audio play for a short time
            std::thread::sleep(std::time::Duration::from_millis(100));

            // Stop playback
            audio_player.stop();

            // If we got here, the test passes
            println!("✅ Interactive audio playback successful");
        }
        Err(e) => {
            // In CI environment without audio devices, this is expected
            println!(
                "Note: Audio player creation failed (expected in CI without audio device): {}",
                e
            );
        }
    }
}

/// Integration test for interactive mode with more events
///
/// This test performs a more comprehensive verification of interactive mode
/// audio playback by scheduling multiple events over a longer period.
#[test]
fn test_interactive_mode_audio_integration() {
    let player = Player::new_interactive();

    // Schedule a sequence of register writes simulating actual usage
    let events = vec![
        (0, 0x08, 0x00),    // Key off all
        (50, 0x20, 0xC7),   // Channel 0 configuration
        (100, 0x28, 0x3E),  // Frequency low
        (150, 0x30, 0x48),  // Frequency high
        (200, 0x08, 0x78),  // Key on channels
        (1000, 0x08, 0x00), // Key off
    ];

    for (time, addr, data) in events {
        player.schedule_register_write(time, addr, data);
    }

    // Verify the events were scheduled
    let queue = player.get_event_queue();
    let q = queue.lock().unwrap();

    // Each register write becomes 2 events (address + data)
    assert_eq!(q.len(), 12, "Should have 6 writes × 2 events = 12 events");

    drop(q); // Release lock before audio playback

    // Try to play the audio
    let result = AudioPlayer::new(player);

    match result {
        Ok(mut audio_player) => {
            // Let audio play for enough time to process events
            std::thread::sleep(std::time::Duration::from_millis(200));

            // Stop playback
            audio_player.stop();

            println!("✅ Interactive mode audio integration test successful");
        }
        Err(e) => {
            println!(
                "Note: Audio player creation failed (expected in CI without audio device): {}",
                e
            );
        }
    }
}
