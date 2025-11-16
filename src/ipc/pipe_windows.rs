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

        let wide_path: Vec<u16> = OsStr::new(path.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let handle = unsafe {
            CreateNamedPipeW(
                PCWSTR(wide_path.as_ptr()),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                PIPE_UNLIMITED_INSTANCES,
                4096,
                4096,
                0,
                None,
            )
        };

        if handle.is_invalid() || handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        Ok(NamedPipe { path, handle })
    }

    pub fn open_read(&self) -> io::Result<PipeReader> {
        unsafe {
            ConnectNamedPipe(self.handle, None).map_err(io::Error::other)?;
        }

        Ok(PipeReader {
            handle: self.handle,
        })
    }

    pub fn open_write(&self) -> io::Result<PipeWriter> {
        Ok(PipeWriter {
            handle: self.handle,
        })
    }

    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<PipeWriter> {
        let path = path.as_ref();

        let wide_path: Vec<u16> = OsStr::new(path.as_os_str())
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

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
            return Err(io::Error::other(e));
        }

        let handle = handle.unwrap();
        if handle.is_invalid() || handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        Ok(PipeWriter { handle })
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

pub struct PipeReader {
    handle: HANDLE,
}

impl PipeReader {
    pub fn read_line(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            let mut bytes_read = 0u32;

            let result =
                unsafe { ReadFile(self.handle, Some(&mut byte), Some(&mut bytes_read), None) };

            if let Err(e) = result {
                return Err(io::Error::other(e));
            }

            if bytes_read == 0 {
                break;
            }

            buffer.push(byte[0]);

            if byte[0] == b'\n' {
                break;
            }
        }

        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Read binary data with length prefix (u32 little-endian + data)
    pub fn read_binary(&mut self) -> io::Result<Vec<u8>> {
        // Read 4-byte length prefix
        let mut len_bytes = [0u8; 4];
        self.read_exact(&mut len_bytes)?;

        let len = u32::from_le_bytes(len_bytes) as usize;

        // Validate reasonable length (max 10MB to prevent memory issues)
        if len > 10 * 1024 * 1024 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Message length too large: {} bytes", len),
            ));
        }

        // Read the data
        let mut data = vec![0u8; len];
        self.read_exact(&mut data)?;

        // Return length prefix + data
        let mut result = Vec::with_capacity(4 + len);
        result.extend_from_slice(&len_bytes);
        result.extend_from_slice(&data);

        Ok(result)
    }

    /// Read exact number of bytes
    fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        let mut total_read = 0;

        while total_read < buffer.len() {
            let mut bytes_read = 0u32;
            let remaining = &mut buffer[total_read..];

            let result =
                unsafe { ReadFile(self.handle, Some(remaining), Some(&mut bytes_read), None) };

            if let Err(e) = result {
                return Err(io::Error::other(e));
            }

            if bytes_read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Pipe closed before reading complete message",
                ));
            }

            total_read += bytes_read as usize;
        }

        Ok(())
    }
}

pub struct PipeWriter {
    handle: HANDLE,
}

impl PipeWriter {
    pub fn write_str(&mut self, data: &str) -> io::Result<()> {
        let bytes = data.as_bytes();
        let mut bytes_written = 0u32;

        let result = unsafe { WriteFile(self.handle, Some(bytes), Some(&mut bytes_written), None) };

        if let Err(e) = result {
            return Err(io::Error::other(e));
        }

        if bytes_written as usize != bytes.len() {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "Failed to write all bytes",
            ));
        }

        unsafe {
            FlushFileBuffers(self.handle).map_err(io::Error::other)?;
        }

        Ok(())
    }

    /// Write binary data (already includes length prefix)
    pub fn write_binary(&mut self, data: &[u8]) -> io::Result<()> {
        let mut bytes_written = 0u32;

        let result = unsafe { WriteFile(self.handle, Some(data), Some(&mut bytes_written), None) };

        if let Err(e) = result {
            return Err(io::Error::other(e));
        }

        if bytes_written as usize != data.len() {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "Failed to write all bytes",
            ));
        }

        unsafe {
            FlushFileBuffers(self.handle).map_err(io::Error::other)?;
        }

        Ok(())
    }

    pub fn read_response(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];

        loop {
            let mut bytes_read = 0u32;

            let result =
                unsafe { ReadFile(self.handle, Some(&mut byte), Some(&mut bytes_read), None) };

            if let Err(e) = result {
                return Err(io::Error::other(e));
            }

            if bytes_read == 0 {
                break;
            }

            buffer.push(byte[0]);

            if byte[0] == b'\n' {
                break;
            }
        }

        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Read binary response with length prefix
    pub fn read_binary_response(&mut self) -> io::Result<Vec<u8>> {
        // Read 4-byte length prefix
        let mut len_bytes = [0u8; 4];
        self.read_exact(&mut len_bytes)?;

        let len = u32::from_le_bytes(len_bytes) as usize;

        // Validate reasonable length
        if len > 10 * 1024 * 1024 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Response length too large: {} bytes", len),
            ));
        }

        // Read the data
        let mut data = vec![0u8; len];
        self.read_exact(&mut data)?;

        // Return length prefix + data
        let mut result = Vec::with_capacity(4 + len);
        result.extend_from_slice(&len_bytes);
        result.extend_from_slice(&data);

        Ok(result)
    }

    /// Read exact number of bytes
    fn read_exact(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        let mut total_read = 0;

        while total_read < buffer.len() {
            let mut bytes_read = 0u32;
            let remaining = &mut buffer[total_read..];

            let result =
                unsafe { ReadFile(self.handle, Some(remaining), Some(&mut bytes_read), None) };

            if let Err(e) = result {
                return Err(io::Error::other(e));
            }

            if bytes_read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Pipe closed before reading complete response",
                ));
            }

            total_read += bytes_read as usize;
        }

        Ok(())
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
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
    fn test_create_pipe() {
        let test_path = r"\\.\pipe\test_ym2151-log-play-server_create";
        let pipe = NamedPipe::create_at(test_path).unwrap();
        assert_eq!(pipe.path(), Path::new(test_path));
    }

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
}
