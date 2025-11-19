#[cfg(windows)]
use ym2151_log_play_server::ipc::pipe_windows::NamedPipe;
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
