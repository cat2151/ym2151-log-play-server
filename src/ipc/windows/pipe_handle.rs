use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;
use windows::Win32::Foundation::{
    CloseHandle, ERROR_IO_PENDING, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT,
};
use windows::Win32::System::Pipes::ConnectNamedPipe;
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};
use windows::Win32::System::IO::GetOverlappedResult;

use super::pipe_factory::{connect_to_pipe, create_named_pipe};
use super::pipe_reader::PipeReader;
use super::pipe_writer::PipeWriter;

pub const DEFAULT_PIPE_PATH: &str = r"\\.\pipe\ym2151-log-play-server";

// Default timeout for pipe connections (5 seconds for tests)
const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

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
        self.open_read_with_timeout(DEFAULT_CONNECT_TIMEOUT)
    }

    pub fn open_read_with_timeout(&self, timeout: Duration) -> io::Result<PipeReader> {
        // Create an event for overlapped I/O
        // Parameters: security_attributes=None, manual_reset=true, initial_state=false, name=None
        // manual_reset=true: Event must be manually reset (stays signaled until ResetEvent)
        // initial_state=false: Event starts in non-signaled state
        let event = unsafe {
            CreateEventW(None, true, false, None)
                .map_err(|e| io::Error::other(format!("Failed to create event: {}", e)))?
        };

        // Prepare overlapped structure
        let mut overlapped = windows::Win32::System::IO::OVERLAPPED {
            hEvent: event,
            ..Default::default()
        };

        // Try to connect with overlapped I/O
        let connect_result = unsafe { ConnectNamedPipe(self.handle, Some(&mut overlapped)) };

        let wait_result = match connect_result {
            Ok(_) => {
                // Connection succeeded immediately
                unsafe { CloseHandle(event) }.ok();
                return Ok(PipeReader::new(self.handle));
            }
            Err(e) => {
                // Check if operation is pending (ERROR_IO_PENDING)
                let error_code = e.code().0 as u32;
                if error_code != ERROR_IO_PENDING.0 {
                    unsafe { CloseHandle(event) }.ok();
                    return Err(io::Error::other(format!("ConnectNamedPipe failed: {}", e)));
                }

                // Wait for the connection with timeout
                unsafe { WaitForSingleObject(event, timeout.as_millis() as u32) }
            }
        };

        match wait_result {
            WAIT_OBJECT_0 => {
                // Wait succeeded, verify the operation completed successfully
                let mut bytes_transferred = 0u32;
                let overlapped_result = unsafe {
                    GetOverlappedResult(self.handle, &overlapped, &mut bytes_transferred, false)
                };

                // Clean up the event handle
                unsafe { CloseHandle(event) }.ok();

                match overlapped_result {
                    Ok(_) => Ok(PipeReader::new(self.handle)),
                    Err(e) => Err(io::Error::other(format!(
                        "GetOverlappedResult failed: {}",
                        e
                    ))),
                }
            }
            WAIT_TIMEOUT => {
                // Clean up the event handle
                unsafe { CloseHandle(event) }.ok();

                // Timeout occurred
                Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!("Timeout waiting for pipe connection after {:?}", timeout),
                ))
            }
            _ => {
                // Clean up the event handle
                unsafe { CloseHandle(event) }.ok();

                // Other error
                Err(io::Error::last_os_error())
            }
        }
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
