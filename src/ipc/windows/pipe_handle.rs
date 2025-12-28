//! Windows named pipe handle management
//!
//! This module manages Windows named pipes with careful attention to handle ownership.
//!
//! # Handle Ownership
//!
//! There are two distinct use cases:
//!
//! 1. **Server-side (NamedPipe)**: Creates a pipe handle that is shared between
//!    PipeReader and PipeWriter via borrowing. Only the NamedPipe owns and closes
//!    the handle.
//!
//! 2. **Client-side (connect)**: Creates a connection with its own handle that
//!    is owned by the returned PipeWriter, which closes it on drop.

use std::io;
use std::path::{Path, PathBuf};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Pipes::ConnectNamedPipe;

use super::pipe_factory::{connect_to_pipe, create_named_pipe};
use super::pipe_reader::PipeReader;
use super::pipe_writer::PipeWriter;

pub const DEFAULT_PIPE_PATH: &str = r"\\.\pipe\ym2151-log-play-server";

#[derive(Debug)]
pub struct NamedPipe {
    path: PathBuf,
    handle: HANDLE,
}

unsafe impl Send for NamedPipe {}
unsafe impl Sync for NamedPipe {}

impl NamedPipe {
    pub fn create() -> io::Result<Self> {
        Self::create_at(DEFAULT_PIPE_PATH)
    }

    pub fn create_at<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let handle = create_named_pipe(&path)?;

        Ok(NamedPipe { path, handle })
    }

    /// Connect to a server and wait for it to accept the connection
    ///
    /// Returns a PipeReader that borrows the handle from this NamedPipe.
    /// The handle will NOT be closed when the PipeReader is dropped.
    pub fn open_read(&self) -> io::Result<PipeReader> {
        unsafe {
            ConnectNamedPipe(self.handle, None).map_err(io::Error::other)?;
        }

        Ok(PipeReader::new(self.handle))
    }

    /// Create a PipeWriter that uses this NamedPipe's handle
    ///
    /// Returns a PipeWriter that borrows the handle from this NamedPipe.
    /// The handle will NOT be closed when the PipeWriter is dropped.
    pub fn open_write(&self) -> io::Result<PipeWriter> {
        Ok(PipeWriter::new(self.handle))
    }

    /// Create a connection to server and wait for it to be ready
    ///
    /// Returns a PipeWriter that owns its handle and will close it on drop.
    /// This is used by clients to connect to the server.
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<PipeWriter> {
        let handle = connect_to_pipe(path)?;
        Ok(PipeWriter::new_owned(handle))
    }

    pub fn connect_default() -> io::Result<PipeWriter> {
        Self::connect(DEFAULT_PIPE_PATH)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for NamedPipe {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}
