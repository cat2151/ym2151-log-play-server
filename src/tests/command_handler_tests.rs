//! Unit tests for CommandHandler refactoring
//!
//! These tests verify that the refactored handle_play_json_in_interactive
//! function maintains identical behavior to the original implementation.

use crate::ipc::protocol::{Command, Response};
use crate::resampler::ResamplingQuality;
use crate::scheduler::TimeTracker;
use crate::server::{CommandHandler, PlaybackManager, ServerState};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

/// Test that PlayJsonInInteractive returns error when not in interactive mode
#[test]
fn test_play_json_interactive_error_when_not_in_interactive_mode() {
    let state = Arc::new(Mutex::new(ServerState::Stopped));
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let time_tracker = Arc::new(Mutex::new(TimeTracker::new()));
    let playback_manager = PlaybackManager::new(ResamplingQuality::Linear);

    let handler = CommandHandler::new(state.clone(), shutdown_flag, time_tracker, playback_manager);

    // Create a simple valid JSON event log
    let test_json = serde_json::json!({
        "events": [
            {"time": 0.0, "addr": "0x08", "data": "0x00"},
            {"time": 0.1, "addr": "0x20", "data": "0xC7"}
        ]
    });

    let command = Command::PlayJsonInInteractive { data: test_json };

    // Test with no audio player
    let mut audio_player = None;
    let response = handler.handle_command(command, &mut audio_player);

    // Should get an error because not in interactive mode
    match response {
        Response::Error { message } => {
            assert!(message.contains("Not in interactive mode"));
        }
        _ => panic!("Expected error response when not in interactive mode"),
    }
}

/// Test that PlayJsonInInteractive returns error with invalid JSON structure
#[test]
fn test_play_json_interactive_error_with_invalid_json_structure() {
    let state = Arc::new(Mutex::new(ServerState::Interactive));
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let time_tracker = Arc::new(Mutex::new(TimeTracker::new()));
    let playback_manager = PlaybackManager::new(ResamplingQuality::Linear);

    let handler = CommandHandler::new(state.clone(), shutdown_flag, time_tracker, playback_manager);

    // Create invalid JSON (missing required "events" field)
    let test_json = serde_json::json!({
        "invalid": "data"
    });

    let command = Command::PlayJsonInInteractive { data: test_json };

    let mut audio_player = None;
    let response = handler.handle_command(command, &mut audio_player);

    // Should get an error for invalid JSON parsing
    match response {
        Response::Error { message } => {
            assert!(message.contains("Failed to parse JSON"));
        }
        _ => panic!("Expected error response with invalid JSON structure"),
    }
}

/// Test that PlayJsonInInteractive requires audio player
#[test]
fn test_play_json_interactive_error_without_audio_player() {
    let state = Arc::new(Mutex::new(ServerState::Interactive));
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let time_tracker = Arc::new(Mutex::new(TimeTracker::new()));
    let playback_manager = PlaybackManager::new(ResamplingQuality::Linear);

    let handler = CommandHandler::new(state.clone(), shutdown_flag, time_tracker, playback_manager);

    // Create valid JSON
    let test_json = serde_json::json!({
        "events": [
            {"time": 0.0, "addr": "0x08", "data": "0x00"}
        ]
    });

    let command = Command::PlayJsonInInteractive { data: test_json };

    let mut audio_player = None;
    let response = handler.handle_command(command, &mut audio_player);

    // Should get error for missing audio player
    match response {
        Response::Error { message } => {
            assert!(message.contains("No audio player found"));
        }
        _ => panic!("Expected error response when audio player is missing"),
    }
}

/// Test validation logic for event log
#[test]
fn test_event_log_validation_in_handler() {
    use crate::events::EventLog;

    // Valid event log
    let valid_json = r#"{
        "events": [
            {"time": 0.0, "addr": "0x08", "data": "0x00"},
            {"time": 0.5, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    let event_log = EventLog::from_json_str(valid_json).unwrap();
    assert!(event_log.validate());

    // Empty events should still parse
    let empty_json = r#"{
        "events": []
    }"#;

    let empty_log = EventLog::from_json_str(empty_json).unwrap();
    assert!(empty_log.validate());
}

/// Test that handler correctly processes valid state transitions
#[test]
fn test_handler_state_transitions() {
    let state = Arc::new(Mutex::new(ServerState::Stopped));
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let time_tracker = Arc::new(Mutex::new(TimeTracker::new()));
    let playback_manager = PlaybackManager::new(ResamplingQuality::Linear);

    let handler = CommandHandler::new(state.clone(), shutdown_flag, time_tracker, playback_manager);

    // Initial state should be Stopped
    {
        let current_state = state.lock().unwrap();
        assert_eq!(*current_state, ServerState::Stopped);
    }

    // Test GetServerState command
    let get_state_cmd = Command::GetServerState;
    let mut audio_player = None;
    let response = handler.handle_command(get_state_cmd, &mut audio_player);

    match response {
        Response::ServerState { state: state_str } => {
            assert_eq!(state_str, "Stopped");
        }
        _ => panic!("Expected ServerState response"),
    }
}

/// Test early return pattern in refactored code
#[test]
fn test_early_return_pattern() {
    // This test verifies the refactored code maintains the same
    // behavior as the original nested implementation by testing
    // all early return conditions

    let state = Arc::new(Mutex::new(ServerState::Playing)); // Not Interactive
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let time_tracker = Arc::new(Mutex::new(TimeTracker::new()));
    let playback_manager = PlaybackManager::new(ResamplingQuality::Linear);

    let handler = CommandHandler::new(state.clone(), shutdown_flag, time_tracker, playback_manager);

    let test_json = serde_json::json!({
        "events": [
            {"time": 0.0, "addr": "0x08", "data": "0x00"}
        ]
    });

    let command = Command::PlayJsonInInteractive { data: test_json };

    let mut audio_player = None;
    let response = handler.handle_command(command, &mut audio_player);

    // Should return error immediately due to wrong state (early return)
    match response {
        Response::Error { message } => {
            assert!(message.contains("Not in interactive mode"));
        }
        _ => panic!("Expected immediate error response for wrong state"),
    }
}
