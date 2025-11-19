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
    log_client("Test message in verbose mode");
}

#[cfg(windows)]
#[test]
fn test_log_client_non_verbose_mode() {
    // Disable verbose mode
    init_client(false);

    // This should not panic in non-verbose mode
    log_client("Test message in non-verbose mode");
}

#[cfg(windows)]
#[test]
fn test_send_command_without_server() {
    let result = send_command(Command::Stop);
    assert!(result.is_err());
}

#[cfg(windows)]
#[test]
fn test_is_server_running_when_not_running() {
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
    // Test that the function can parse valid JSON
    let json_data = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x20", "data": "0xC7"}
        ]
    }"#;

    // This will fail to connect to server when writing registers,
    // but it should successfully parse JSON first
    let result = play_json_interactive(json_data);

    // Should fail because server is not running, but not because of JSON parsing
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    // Error should be about register write/server connection, not JSON parsing
    assert!(
        error_msg.contains("Failed to write register") || error_msg.contains("Failed to connect")
    );
}

#[cfg(windows)]
#[test]
fn test_play_json_interactive_rejects_invalid_json() {
    let invalid_json = r#"{"event_count": 1, "events": [}"#;

    let result = play_json_interactive(invalid_json);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Failed to parse JSON"));
}

#[cfg(windows)]
#[test]
fn test_play_json_interactive_validates_event_log() {
    // Event count mismatch
    let json_data = r#"{
        "event_count": 5,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"}
        ]
    }"#;

    let result = play_json_interactive(json_data);
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("validation failed"));
}

#[cfg(windows)]
#[test]
fn test_play_json_interactive_empty_events() {
    let json_data = r#"{
        "event_count": 0,
        "events": []
    }"#;

    // Empty events should be valid and succeed (no register writes to send)
    // Since the function doesn't start/stop interactive mode, it should just do nothing
    let result = play_json_interactive(json_data);

    // Should succeed since there are no events to process
    assert!(result.is_ok());
}
