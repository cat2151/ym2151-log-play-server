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

    pub fn open_read(&self) -> io::Result<PipeReader> {
        unsafe {
            ConnectNamedPipe(self.handle, None).map_err(io::Error::other)?;
        }

        Ok(PipeReader::new(self.handle))
    }

    pub fn open_write(&self) -> io::Result<PipeWriter> {
        Ok(PipeWriter::new(self.handle))
    }

    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<PipeWriter> {
        let handle = connect_to_pipe(path)?;
        Ok(PipeWriter::new(handle))
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
