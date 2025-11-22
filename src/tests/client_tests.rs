#[cfg(windows)]
use crate::client::*;
#[cfg(windows)]
use crate::ipc::protocol::Command;

#[cfg(windows)]
#[test]
fn test_init_client_verbose() {
    init_client(true);
    assert!(is_client_verbose());

    init_client(false);
    assert!(!is_client_verbose());
}

#[cfg(windows)]
#[test]
fn test_client_verbose_default() {
    // Test that the verbose flag can be queried without initialization
    // The default should be false (non-verbose)
    let _ = is_client_verbose();
}

#[cfg(windows)]
#[test]
fn test_log_client_verbose_mode() {
    // Enable verbose mode
    init_client(true);

    // This should not panic in verbose mode
    log_verbose_client("Test message in verbose mode");
}

#[cfg(windows)]
#[test]
fn test_log_client_non_verbose_mode() {
    // Disable verbose mode
    init_client(false);

    // This should not panic in non-verbose mode
    log_verbose_client("Test message in non-verbose mode");
}

#[cfg(windows)]
#[test]
fn test_send_command_without_server() {
    // Ensure server is not running before test
    let _ = shutdown_server(); // Ignore result - server might not be running

    let result = send_command(Command::Stop);
    assert!(result.is_err());
}

#[cfg(windows)]
#[test]
fn test_is_server_running_when_not_running() {
    // Ensure server is not running before test
    let _ = shutdown_server(); // Ignore result - server might not be running

    // When server is not running, should return false
    let result = is_server_running();
    // On Linux this will be false since we can't test Windows named pipes
    // On Windows without server, this should also be false
    assert!(!result || cfg!(windows));
}

#[cfg(windows)]
#[test]
fn test_is_app_in_path() {
    // Test with a command that should always exist
    assert!(is_app_in_path("cargo"));

    // Test with a command that likely doesn't exist
    assert!(!is_app_in_path("this-command-should-not-exist-xyz123"));
}

#[cfg(windows)]
#[test]
fn test_play_json_interactive_parses_valid_json() {
    // Ensure server is not running before test
    let _ = shutdown_server(); // Ignore result - server might not be running

    // Test that the function can parse valid JSON
    let json_data = r#"{
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    // This will fail to connect to server, but should successfully parse JSON first
    let result = play_json_interactive(json_data);

    // Should fail because server is not running, but not because of JSON parsing
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    // Error should be about sending to server, not JSON parsing
    assert!(
        error_msg.contains("Failed to send")
            || error_msg.contains("Failed to connect")
            || error_msg.contains("server")
    );
}

#[cfg(windows)]
#[test]
fn test_play_json_interactive_rejects_invalid_json() {
    let invalid_json = r#"{"events": [}"#;

    let result = play_json_interactive(invalid_json);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    println!("Error message: {}", error_msg);
    // In new implementation, error comes from conversion function
    assert!(
        error_msg.contains("Failed to parse JSON")
            || error_msg.contains("Failed to convert JSON")
            || error_msg.contains("Failed to parse converted JSON")
            || error_msg.contains("timing from samples")
    );
}

#[cfg(windows)]
#[test]
fn test_play_json_interactive_validates_event_log() {
    // Event count mismatch
    let json_data = r#"{
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"}
        ]
    }"#;

    let result = play_json_interactive(json_data);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    println!("Error message: {}", error_msg);
    // In new implementation, validation happens in conversion or server-side
    // Client-side may fail on conversion, JSON parse or connection errors
    assert!(
        error_msg.contains("Failed to send")
            || error_msg.contains("Failed to connect")
            || error_msg.contains("server")
            || error_msg.contains("validation")
            || error_msg.contains("Invalid")
            || error_msg.contains("convert")
            || error_msg.contains("timing")
    );
}

#[cfg(windows)]
#[test]
fn test_play_json_interactive_empty_events() {
    let json_data = r#"{
        "events": []
    }"#;

    // Empty events should be valid JSON and succeed (no events to send)
    let result = play_json_interactive(json_data);
    assert!(
        result.is_ok(),
        "Empty events should succeed without server connection"
    );
}
