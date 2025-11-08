//! Windows named pipe implementation
//!
//! This module provides Windows-specific named pipe functionality using
//! the Windows API CreateNamedPipe and related functions.

use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FlushFileBuffers, ReadFile, WriteFile, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE,
    OPEN_EXISTING, PIPE_ACCESS_DUPLEX,
};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_TYPE_BYTE,
    PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};

/// Default pipe path for Windows
pub const DEFAULT_PIPE_PATH: &str = r"\\.\pipe\ym2151_server";

/// Named pipe for Windows systems
pub struct NamedPipe {
    path: PathBuf,
    handle: HANDLE,
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
    /// use ym2151_log_player_rust::ipc::pipe_windows::NamedPipe;
    ///
    /// let pipe = NamedPipe::create().unwrap();
    /// ```
    pub fn create() -> io::Result<Self> {
        Self::create_at(DEFAULT_PIPE_PATH)
    }

    /// Create a new named pipe at a specific path
    ///
    /// # Arguments
    /// * `path` - Path where the named pipe should be created
    ///
    /// # Returns
    /// * `Ok(NamedPipe)` - Successfully created pipe
    /// * `Err(io::Error)` - Failed to create pipe
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::ipc::pipe_windows::NamedPipe;
    ///
    /// let pipe = NamedPipe::create_at(r"\\.\pipe\custom").unwrap();
    /// ```
    pub fn create_at<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();

        // Convert path to wide string for Windows API
        let wide_path: Vec<u16> = OsStr::new(path.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // Create the named pipe
        // SAFETY: Calling Windows API with valid parameters
        let handle = unsafe {
            CreateNamedPipeW(
                PCWSTR(wide_path.as_ptr()),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                PIPE_UNLIMITED_INSTANCES,
                4096, // Output buffer size
                4096, // Input buffer size
                0,    // Default timeout
                None, // Default security attributes
            )
        };

        if handle.is_invalid() || handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        Ok(NamedPipe { path, handle })
    }

    /// Open the named pipe for reading (server side, blocking)
    ///
    /// This will block until a writer connects to the pipe.
    ///
    /// # Returns
    /// * `Ok(PipeReader)` - Successfully opened for reading
    /// * `Err(io::Error)` - Failed to open
    pub fn open_read(&self) -> io::Result<PipeReader> {
        // Wait for a client to connect
        // SAFETY: handle is valid and owned by this NamedPipe
        unsafe {
            ConnectNamedPipe(self.handle, None)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        Ok(PipeReader {
            handle: self.handle,
        })
    }

    /// Open the named pipe for writing (not typically used on server side)
    ///
    /// # Returns
    /// * `Ok(PipeWriter)` - Successfully opened for writing
    /// * `Err(io::Error)` - Failed to open
    pub fn open_write(&self) -> io::Result<PipeWriter> {
        Ok(PipeWriter {
            handle: self.handle,
        })
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
        let path = path.as_ref();

        // Convert path to wide string
        let wide_path: Vec<u16> = OsStr::new(path.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // Open the named pipe for read/write
        // SAFETY: Calling Windows API with valid parameters
        let handle = unsafe {
            CreateFileW(
                PCWSTR(wide_path.as_ptr()),
                windows::Win32::Storage::FileSystem::FILE_GENERIC_READ.0
                    | windows::Win32::Storage::FileSystem::FILE_GENERIC_WRITE.0,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        };

        if let Err(e) = handle {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }

        let handle = handle.unwrap();
        if handle.is_invalid() || handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        Ok(PipeWriter { handle })
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
        // Close the pipe handle
        // SAFETY: handle is valid and owned by this NamedPipe
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

/// Reader for a named pipe
pub struct PipeReader {
    handle: HANDLE,
}

impl PipeReader {
    /// Read a line from the pipe
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully read a line
    /// * `Err(io::Error)` - Read error or EOF
    pub fn read_line(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            let mut bytes_read = 0u32;

            // SAFETY: handle is valid, buffer is valid
            let result =
                unsafe { ReadFile(self.handle, Some(&mut byte), Some(&mut bytes_read), None) };

            if let Err(e) = result {
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }

            if bytes_read == 0 {
                break; // EOF
            }

            buffer.push(byte[0]);

            // Stop at newline
            if byte[0] == b'\n' {
                break;
            }
        }

        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

/// Writer for a named pipe
pub struct PipeWriter {
    handle: HANDLE,
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
        let bytes = data.as_bytes();
        let mut bytes_written = 0u32;

        // SAFETY: handle is valid, buffer is valid
        let result = unsafe { WriteFile(self.handle, Some(bytes), Some(&mut bytes_written), None) };

        if let Err(e) = result {
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }

        if bytes_written as usize != bytes.len() {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "Failed to write all bytes",
            ));
        }

        // Flush to ensure data is sent
        // SAFETY: handle is valid
        unsafe {
            FlushFileBuffers(self.handle).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        }

        Ok(())
    }

    /// Read a response line from the pipe
    ///
    /// # Returns
    /// * `Ok(String)` - Successfully read a line
    /// * `Err(io::Error)` - Read error or EOF
    pub fn read_response(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            let mut bytes_read = 0u32;

            // SAFETY: handle is valid, buffer is valid
            let result =
                unsafe { ReadFile(self.handle, Some(&mut byte), Some(&mut bytes_read), None) };

            if let Err(e) = result {
                return Err(io::Error::new(io::ErrorKind::Other, e));
            }

            if bytes_read == 0 {
                break; // EOF
            }

            buffer.push(byte[0]);

            // Stop at newline
            if byte[0] == b'\n' {
                break;
            }
        }

        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        // Close the pipe handle
        // SAFETY: handle is valid and owned by this PipeWriter
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    #[cfg(windows)]
    fn test_create_pipe() {
        let test_path = r"\\.\pipe\test_ym2151_create";
        let pipe = NamedPipe::create_at(test_path).unwrap();
        assert_eq!(pipe.path(), Path::new(test_path));
    }

    #[test]
    #[cfg(windows)]
    fn test_write_read_pipe() {
        let test_path = r"\\.\pipe\test_ym2151_rw";

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

    #[test]
    #[cfg(windows)]
    fn test_multiple_messages() {
        let test_path = r"\\.\pipe\test_ym2151_multi";

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
}
