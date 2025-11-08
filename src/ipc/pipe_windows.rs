//! Windows named pipe implementation (stub)
//!
//! This module provides a stub for Windows named pipe functionality.
//! Full implementation is planned for Phase 8.

use std::io;
use std::path::Path;

/// Default pipe path for Windows
pub const DEFAULT_PIPE_PATH: &str = r"\\.\pipe\ym2151_server";

/// Named pipe for Windows systems (stub)
pub struct NamedPipe;

impl NamedPipe {
    /// Create a new named pipe (stub)
    pub fn create() -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented. This is planned for Phase 8.",
        ))
    }

    /// Create a new named pipe at a specific path (stub)
    pub fn create_at<P: AsRef<Path>>(_path: P) -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented. This is planned for Phase 8.",
        ))
    }

    /// Open for reading (stub)
    pub fn open_read(&self) -> io::Result<PipeReader> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented.",
        ))
    }

    /// Open for writing (stub)
    pub fn open_write(&self) -> io::Result<PipeWriter> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented.",
        ))
    }

    /// Connect to an existing pipe (stub)
    pub fn connect<P: AsRef<Path>>(_path: P) -> io::Result<PipeWriter> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented. This is planned for Phase 8.",
        ))
    }

    /// Connect to the default pipe (stub)
    pub fn connect_default() -> io::Result<PipeWriter> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented. This is planned for Phase 8.",
        ))
    }
}

/// Reader for a named pipe (stub)
pub struct PipeReader;

impl PipeReader {
    /// Read a line (stub)
    pub fn read_line(&mut self) -> io::Result<String> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented.",
        ))
    }
}

/// Writer for a named pipe (stub)
pub struct PipeWriter;

impl PipeWriter {
    /// Write a string (stub)
    pub fn write_str(&mut self, _data: &str) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented.",
        ))
    }

    /// Read response (stub)
    pub fn read_response(&mut self) -> io::Result<String> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Windows named pipes are not yet implemented.",
        ))
    }
}
