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
    use std::sync::Arc;

    let test_path = r"\\.\pipe\test_ym2151-log-play-server_rw";

    let pipe = Arc::new(NamedPipe::create_at(test_path).unwrap());

    // Spawn a reader thread FIRST (server waits for client)
    let pipe_clone = Arc::clone(&pipe);
    let reader_thread = thread::spawn(move || {
        // open_read() will block until a client connects
        let mut reader = pipe_clone.open_read().unwrap();
        reader.read_line().unwrap()
    });

    // Small delay to ensure server is waiting
    thread::sleep(Duration::from_millis(100));

    // Client connects AFTER server is waiting
    let write_path = test_path.to_string();
    let mut writer = NamedPipe::connect(&write_path).unwrap();
    writer.write_str("Hello, Windows Pipe!\n").unwrap();

    // Get the result from reader thread
    let line = reader_thread.join().unwrap();

    assert_eq!(line, "Hello, Windows Pipe!\n");
}

#[cfg(windows)]
#[test]
fn test_multiple_messages() {
    use std::sync::Arc;

    let test_path = r"\\.\pipe\test_ym2151-log-play-server_multi";

    let pipe = Arc::new(NamedPipe::create_at(test_path).unwrap());

    // Spawn a reader thread FIRST (server waits for client)
    let pipe_clone = Arc::clone(&pipe);
    let reader_thread = thread::spawn(move || {
        let mut reader = pipe_clone.open_read().unwrap();
        let line1 = reader.read_line().unwrap();
        let line2 = reader.read_line().unwrap();
        let line3 = reader.read_line().unwrap();
        (line1, line2, line3)
    });

    // Small delay to ensure server is waiting
    thread::sleep(Duration::from_millis(100));

    // Client connects AFTER server is waiting
    let write_path = test_path.to_string();
    let mut writer = NamedPipe::connect(&write_path).unwrap();
    writer.write_str("Message 1\n").unwrap();
    writer.write_str("Message 2\n").unwrap();
    writer.write_str("Message 3\n").unwrap();

    let (line1, line2, line3) = reader_thread.join().unwrap();

    assert_eq!(line1, "Message 1\n");
    assert_eq!(line2, "Message 2\n");
    assert_eq!(line3, "Message 3\n");
}

#[cfg(windows)]
#[test]
fn test_binary_protocol_with_logging() {
    use crate::ipc::pipe_windows::{test_set_client_context, test_set_server_context};
    use crate::ipc::protocol::{Command, Response};
    use std::sync::mpsc;

    let test_path = r"\\.\pipe\test_ym2151-log-play-server_binary_log";

    let pipe = NamedPipe::create_at(test_path).unwrap();

    // Channel to signal when server is ready
    let (tx, rx) = mpsc::channel();

    // Spawn server thread FIRST (waits for client)
    let server_thread = thread::spawn(move || {
        // Set server context for logging
        test_set_server_context();

        // Signal that we're about to wait
        tx.send(()).unwrap();

        // Read command from client (blocks until client connects)
        let mut reader = pipe.open_read().unwrap();
        let binary_data = reader.read_binary().unwrap();
        let command = Command::from_binary(&binary_data).unwrap();

        assert_eq!(command, Command::Stop);

        // Send response
        let mut writer = pipe.open_write().unwrap();
        let response = Response::Ok;
        let response_binary = response.to_binary().unwrap();
        writer.write_binary(&response_binary).unwrap();
    });

    // Wait for server to be ready
    rx.recv().unwrap();
    thread::sleep(Duration::from_millis(50));

    // Set client context for logging
    test_set_client_context();

    // Client connects AFTER server is waiting
    let write_path = test_path.to_string();
    let mut writer = NamedPipe::connect(&write_path).unwrap();

    // Send a command
    let command = Command::Stop;
    let binary_data = command.to_binary().unwrap();
    writer.write_binary(&binary_data).unwrap();

    // Read response
    let response_data = writer.read_binary_response().unwrap();
    let response = Response::from_binary(&response_data).unwrap();

    assert_eq!(response, Response::Ok);

    server_thread.join().unwrap();

    // Check that log files were created
    assert!(
        std::path::Path::new("test_server.log").exists(),
        "test_server.log should be created"
    );
    assert!(
        std::path::Path::new("test_client.log").exists(),
        "test_client.log should be created"
    );
}
