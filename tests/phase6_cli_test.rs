//! Phase 6 CLI Integration Tests
//!
//! These tests validate command-line argument parsing for server and client modes.

use std::process::Command;

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

    path.push("ym2151-log-play-server");
    path.to_str().expect("Invalid path").to_string()
}

#[test]
fn test_help_message_displays() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Check that help message contains key phrases
    assert!(stderr.contains("YM2151 Log Player"));
    assert!(stderr.contains("使用方法"));
    assert!(stderr.contains("スタンドアロン演奏"));

    // Check for key options
    assert!(stderr.contains("--server"));
    assert!(stderr.contains("--client"));
    assert!(stderr.contains("--shutdown"));
    assert!(stderr.contains("--stop"));

    // Exit code should be 1 (error - no arguments)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_unknown_option_error() {
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
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("file1.json")
        .arg("file2.json")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("引数が多すぎます"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_client_without_server_fails() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("--client")
        .arg("--stop")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should report connection failure (no server running)
    assert!(stderr.contains("失敗") || stderr.contains("エラー"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_server_shutdown_without_server_fails() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("--client")
        .arg("--shutdown")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should report connection failure (no server running)
    assert!(stderr.contains("失敗") || stderr.contains("エラー"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_server_option_with_argument_fails() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("--server")
        .arg("test.json")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should report that argument is not needed
    assert!(stderr.contains("引数は不要") || stderr.contains("エラー"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_client_option_without_argument_fails() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("--client")
        .output()
        .expect("Failed to execute binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should report missing argument
    assert!(stderr.contains("引数が必要") || stderr.contains("エラー"));

    // Exit code should be 1 (error)
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_standalone_mode_with_valid_file() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("test_sample_correct.json")
        .output()
        .expect("Failed to execute binary");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should successfully load the file
    assert!(
        stdout.contains("イベントを読み込みました") || stderr.contains("イベントを読み込みました")
    );

    // Exit code should be 0 (success)
    assert_eq!(output.status.code(), Some(0));
}
