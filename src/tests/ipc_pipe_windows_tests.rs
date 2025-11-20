#[cfg(windows)]
use crate::ipc::pipe_windows::NamedPipe;
#[cfg(windows)]
use std::path::Path;
#[cfg(windows)]
use std::thread;
#[cfg(windows)]
use std::time::Duration;

#[cfg(windows)]
#[test]
fn test_create_pipe() {
    let test_path = r"\\.\pipe\test_ym2151-log-play-server_create";
    let pipe = NamedPipe::create_at(test_path).unwrap();
    assert_eq!(pipe.path(), Path::new(test_path));
}

#[cfg(windows)]
#[test]
fn test_write_read_pipe() {
    let test_path = r"\\.\pipe\test_ym2151-log-play-server_rw";

    let pipe = NamedPipe::create_at(test_path).unwrap();

    // Spawn a writer thread
    let write_path = test_path.to_string();
    let writer_thread = thread::spawn(move || {
        // Small delay to ensure server is ready
        thread::sleep(Duration::from_millis(100));
        let mut writer = NamedPipe::connect(&write_path).unwrap();
        writer.write_str("Hello, Windows Pipe!\n").unwrap();
    });

    // Read from the pipe (this will block until writer connects)
    let mut reader = pipe.open_read().unwrap();
    let line = reader.read_line().unwrap();

    writer_thread.join().unwrap();

    assert_eq!(line, "Hello, Windows Pipe!\n");
}

#[cfg(windows)]
#[test]
fn test_multiple_messages() {
    let test_path = r"\\.\pipe\test_ym2151-log-play-server_multi";

    let pipe = NamedPipe::create_at(test_path).unwrap();

    let write_path = test_path.to_string();
    let writer_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let mut writer = NamedPipe::connect(&write_path).unwrap();
        writer.write_str("Message 1\n").unwrap();
        writer.write_str("Message 2\n").unwrap();
        writer.write_str("Message 3\n").unwrap();
    });

    let mut reader = pipe.open_read().unwrap();
    let line1 = reader.read_line().unwrap();
    let line2 = reader.read_line().unwrap();
    let line3 = reader.read_line().unwrap();

    writer_thread.join().unwrap();

    assert_eq!(line1, "Message 1\n");
    assert_eq!(line2, "Message 2\n");
    assert_eq!(line3, "Message 3\n");
}

#[cfg(windows)]
#[test]
fn test_binary_protocol_with_logging() {
    use crate::ipc::pipe_windows::{test_set_client_context, test_set_server_context};
    use crate::ipc::protocol::{Command, Response};

    let test_path = r"\\.\pipe\test_ym2151-log-play-server_binary_log";

    let pipe = NamedPipe::create_at(test_path).unwrap();

    let write_path = test_path.to_string();
    let client_thread = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));

        // Set client context for logging
        test_set_client_context();

        let mut writer = NamedPipe::connect(&write_path).unwrap();

        // Send a command
        let command = Command::Stop;
        let binary_data = command.to_binary().unwrap();
        writer.write_binary(&binary_data).unwrap();

        // Read response
        let response_data = writer.read_binary_response().unwrap();
        let response = Response::from_binary(&response_data).unwrap();

        assert_eq!(response, Response::Ok);
    });

    // Set server context for logging
    test_set_server_context();

    // Read command from client
    let mut reader = pipe.open_read().unwrap();
    let binary_data = reader.read_binary().unwrap();
    let command = Command::from_binary(&binary_data).unwrap();

    assert_eq!(command, Command::Stop);

    // Send response
    let mut writer = pipe.open_write().unwrap();
    let response = Response::Ok;
    let response_binary = response.to_binary().unwrap();
    writer.write_binary(&response_binary).unwrap();

    client_thread.join().unwrap();

    // Check that log files were created
    assert!(std::path::Path::new("test_server.log").exists(), "test_server.log should be created");
    assert!(std::path::Path::new("test_client.log").exists(), "test_client.log should be created");
}
