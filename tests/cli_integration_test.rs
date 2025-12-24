//! CLI Integration Tests
//!
//! These tests validate command-line argument parsing for server and client modes.

use std::process::Command;

mod test_util_server_mutex;

/// Helper function to get the path to the compiled binary
fn get_binary_path() -> String {
    let mut path = std::env::current_exe()
        .expect("Failed to get current exe path")
        .parent()
        .expect("Failed to get parent dir")
        .parent()
        .expect("Failed to get grandparent dir")
        .to_path_buf();

    // In debug mode
    if path.ends_with("deps") {
        path = path
            .parent()
            .expect("Failed to get deps parent")
            .to_path_buf();
    }

    // Add .exe extension for Windows
    if cfg!(windows) {
        path.push("ym2151-log-play-server.exe");
    } else {
        path.push("ym2151-log-play-server");
    }
    path.to_str().expect("Invalid path").to_string()
}

#[test]
fn test_help_message_displays() {
    let _guard = test_util_server_mutex::server_test_lock();
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined_output = format!("{}{}", stdout, stderr);

    // Check that help message contains key phrases (clap's default help)
    assert!(combined_output.contains("YM2151 Log Player") || combined_output.contains("Usage"));

    // Check for key commands
    assert!(combined_output.contains("server"));
    assert!(combined_output.contains("client"));

    // Exit code should be 1 or 0 (depending on if it's help or error)
    assert!(output.status.code() == Some(1) || output.status.code() == Some(0));
}

#[test]
fn test_unknown_option_error() {
    let _guard = test_util_server_mutex::server_test_lock();
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("--unknown-option")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("不明なオプション"));
    assert!(stderr.contains("--unknown-option"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_too_many_arguments() {
    let _guard = test_util_server_mutex::server_test_lock();
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("client")
        .arg("file1.json")
        .arg("file2.json")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should report unexpected argument or error
    assert!(stderr.contains("エラー") || stderr.contains("error"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_client_without_server_fails() {
    let _guard = test_util_server_mutex::server_test_lock();
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("client")
        .arg("--stop")
        .output()
        .expect("Failed to execute binary");

    // Exit code should be 1 (error) when server is not running
    // Note: The implementation may not output error messages to stderr in all cases
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_server_shutdown_without_server_fails() {
    let _guard = test_util_server_mutex::server_test_lock();
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("client")
        .arg("--shutdown")
        .output()
        .expect("Failed to execute binary");

    // Exit code should be 1 (error) when server is not running
    // Note: The implementation may not output error messages to stderr in all cases
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_server_option_with_argument_fails() {
    let _guard = test_util_server_mutex::server_test_lock();
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("server")
        .arg("test.json")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should report that argument is not expected
    assert!(stderr.contains("エラー") || stderr.contains("error"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_client_option_without_argument_fails() {
    let _guard = test_util_server_mutex::server_test_lock();
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("client")
        .output()
        .expect("Failed to execute binary");

    // Exit code should be 1 (error) when no argument is provided
    // Note: The implementation may not output error messages to stderr in all cases
    assert_eq!(output.status.code(), Some(1));
}
