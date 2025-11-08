//! Unix named pipe (FIFO) implementation
//!
//! This module provides Unix-specific named pipe functionality using FIFO.
//! It creates, reads from, and writes to named pipes on Unix-like systems.

use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Default path for the named pipe on Unix systems
pub const DEFAULT_PIPE_PATH: &str = "/tmp/ym2151_server.pipe";

/// Named pipe for Unix systems (FIFO)
pub struct NamedPipe {
    path: PathBuf,
}

impl NamedPipe {
    /// Create a new named pipe at the default path
    ///
    /// # Returns
    /// * `Ok(NamedPipe)` - Successfully created pipe
    /// * `Err(io::Error)` - Failed to create pipe
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;
    ///
    /// let pipe = NamedPipe::create().unwrap();
    /// ```
    pub fn create() -> io::Result<Self> {
        Self::create_at(DEFAULT_PIPE_PATH)
    }

    /// Create a new named pipe at a specific path
    ///
    /// # Arguments
    /// * `path` - Path where the FIFO should be created
    ///
    /// # Returns
    /// * `Ok(NamedPipe)` - Successfully created pipe
    /// * `Err(io::Error)` - Failed to create pipe
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::ipc::pipe_unix::NamedPipe;
    ///
    /// let pipe = NamedPipe::create_at("/tmp/custom.pipe").unwrap();
    /// ```
    pub fn create_at<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Remove existing pipe if present
        if path.exists() {
            fs::remove_file(&path)?;
        }

        // Create FIFO with read/write permissions for owner, read for group and others
        mkfifo(
            &path,
            Mode::S_IRUSR | Mode::S_IWUSR | Mode::S_IRGRP | Mode::S_IROTH,
        )
        .map_err(io::Error::other)?;

        Ok(NamedPipe { path })
    }

    /// Open the named pipe for reading (server side, blocking)
    ///
    /// This will block until a writer connects to the pipe.
    ///
    /// # Returns
    /// * `Ok(PipeReader)` - Successfully opened for reading
    /// * `Err(io::Error)` - Failed to open
    pub fn open_read(&self) -> io::Result<PipeReader> {
        let file = File::open(&self.path)?;
        Ok(PipeReader {
            reader: BufReader::new(file),
        })
    }

    /// Open the named pipe for writing (client side, blocking)
    ///
    /// This will block until a reader is available on the pipe.
    ///
    /// # Returns
    /// * `Ok(PipeWriter)` - Successfully opened for writing
    /// * `Err(io::Error)` - Failed to open
    pub fn open_write(&self) -> io::Result<PipeWriter> {
        let file = OpenOptions::new().write(true).open(&self.path)?;
        Ok(PipeWriter { file })
    }

    /// Connect to an existing named pipe for writing (client side)
    ///
    /// # Arguments
    /// * `path` - Path to the existing named pipe
    ///
    /// # Returns
    /// * `Ok(PipeWriter)` - Successfully connected
    /// * `Err(io::Error)` - Failed to connect (pipe may not exist)
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<PipeWriter> {
        let file = OpenOptions::new().write(true).open(path)?;
        Ok(PipeWriter { file })
    }

    /// Connect to the default named pipe for writing (client side)
    ///
    /// # Returns
    /// * `Ok(PipeWriter)` - Successfully connected
    /// * `Err(io::Error)` - Failed to connect (server may not be running)
    pub fn connect_default() -> io::Result<PipeWriter> {
        Self::connect(DEFAULT_PIPE_PATH)
    }

    /// Get the path of this named pipe
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        // Clean up the pipe file when dropped
        let _ = fs::remove_file(&self.path);
    }
}

/// Reader for a named pipe
pub struct PipeReader {
    reader: BufReader<File>,
}

impl PipeReader {
    /// Read a line from the pipe
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully read a line
    /// * `Err(io::Error)` - Read error or EOF
    pub fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
}

/// Writer for a named pipe
pub struct PipeWriter {
    file: File,
}

impl PipeWriter {
    /// Write a string to the pipe
    ///
    /// # Arguments
    /// * `data` - String to write
    ///
    /// # Returns
    /// * `Ok(())` - Successfully written
    /// * `Err(io::Error)` - Write error
    pub fn write_str(&mut self, data: &str) -> io::Result<()> {
        self.file.write_all(data.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    /// Read a response line from the pipe
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully read a line
    /// * `Err(io::Error)` - Read error or EOF
    pub fn read_response(&mut self) -> io::Result<String> {
        // Reopen the pipe for reading to get the response
        // This is a simple implementation; in production, bidirectional
        // communication would use separate pipes or a more sophisticated protocol
        let mut buffer = String::new();
        let mut reader = BufReader::new(&self.file);
        reader.read_line(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_create_and_cleanup_pipe() {
        let test_path = "/tmp/test_ym2151_pipe_create.pipe";

        // Create pipe
        let pipe = NamedPipe::create_at(test_path).unwrap();
        assert!(Path::new(test_path).exists());
        assert_eq!(pipe.path(), Path::new(test_path));

        // Drop should clean up
        drop(pipe);
        // Give a moment for cleanup
        thread::sleep(Duration::from_millis(10));
        assert!(!Path::new(test_path).exists());
    }

    #[test]
    fn test_recreate_existing_pipe() {
        let test_path = "/tmp/test_ym2151_pipe_recreate.pipe";

        // Create first pipe
        let pipe1 = NamedPipe::create_at(test_path).unwrap();
        drop(pipe1);

        // Should be able to create again even if file exists
        let pipe2 = NamedPipe::create_at(test_path).unwrap();
        assert!(Path::new(test_path).exists());

        drop(pipe2);
        thread::sleep(Duration::from_millis(10));
        assert!(!Path::new(test_path).exists());
    }

    #[test]
    fn test_write_read_pipe() {
        let test_path = "/tmp/test_ym2151_pipe_rw.pipe";

        let pipe = NamedPipe::create_at(test_path).unwrap();

        // Spawn a writer thread
        let write_path = test_path.to_string();
        let writer_thread = thread::spawn(move || {
            // Small delay to ensure reader is ready
            thread::sleep(Duration::from_millis(100));
            let mut writer = NamedPipe::connect(&write_path).unwrap();
            writer.write_str("Hello, FIFO!\n").unwrap();
        });

        // Read from the pipe (this will block until writer connects and writes)
        let mut reader = pipe.open_read().unwrap();
        let line = reader.read_line().unwrap();

        writer_thread.join().unwrap();

        assert_eq!(line, "Hello, FIFO!\n");

        drop(pipe);
        thread::sleep(Duration::from_millis(10));
    }

    #[test]
    fn test_multiple_messages() {
        let test_path = "/tmp/test_ym2151_pipe_multi.pipe";

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

        drop(pipe);
        thread::sleep(Duration::from_millis(10));
    }
}
