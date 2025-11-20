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

/// Test-only logging infrastructure for named pipe communication
#[cfg(test)]
mod test_logging {
    use std::cell::RefCell;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::sync::Mutex;

    static SERVER_LOG: Mutex<()> = Mutex::new(());
    static CLIENT_LOG: Mutex<()> = Mutex::new(());

    thread_local! {
        static PIPE_CONTEXT: RefCell<PipeContext> = RefCell::new(PipeContext::Unknown);
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum PipeContext {
        Server,
        Client,
        Unknown,
    }

    pub fn set_server_context() {
        PIPE_CONTEXT.with(|ctx| *ctx.borrow_mut() = PipeContext::Server);
    }

    pub fn set_client_context() {
        PIPE_CONTEXT.with(|ctx| *ctx.borrow_mut() = PipeContext::Client);
    }

    pub fn log_server(message: &str) {
        let _guard = SERVER_LOG.lock().unwrap();
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("test_server.log")
        {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let _ = writeln!(file, "[{}] {}", timestamp, message);
        }
    }

    pub fn log_client(message: &str) {
        let _guard = CLIENT_LOG.lock().unwrap();
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("test_client.log")
        {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let _ = writeln!(file, "[{}] {}", timestamp, message);
        }
    }

    pub fn log_write(message: &str) {
        PIPE_CONTEXT.with(|ctx| match *ctx.borrow() {
            PipeContext::Server => log_server(message),
            PipeContext::Client => log_client(message),
            PipeContext::Unknown => {
                // Log to both if context unknown
                log_server(&format!("[UNKNOWN_CTX] {}", message));
                log_client(&format!("[UNKNOWN_CTX] {}", message));
            }
        });
    }
}

#[cfg(test)]
use test_logging::{log_client, log_server, log_write, set_client_context, set_server_context};

/// Set the current thread's pipe context to server mode (test builds only)
#[cfg(test)]
pub fn test_set_server_context() {
    set_server_context();
}

/// Set the current thread's pipe context to client mode (test builds only)
#[cfg(test)]
pub fn test_set_client_context() {
    set_client_context();
}

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

        #[cfg(feature = "verbose_pipe_debug")]
        eprintln!("🔧 [PIPE DEBUG] Creating named pipe at: {:?}", path);

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
            let err = io::Error::last_os_error();
            #[cfg(feature = "verbose_pipe_debug")]
            eprintln!("❌ [PIPE DEBUG] Failed to create pipe: {:?}", err);
            return Err(err);
        }

        #[cfg(feature = "verbose_pipe_debug")]
        eprintln!("✅ [PIPE DEBUG] Pipe created successfully");

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

        #[cfg(feature = "verbose_pipe_debug")]
        eprintln!("🔌 [PIPE DEBUG] Attempting to connect to pipe: {:?}", path);

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
            #[cfg(feature = "verbose_pipe_debug")]
            eprintln!("❌ [PIPE DEBUG] CreateFileW failed: {:?}", e);
            return Err(io::Error::other(e));
        }

        let handle = handle.unwrap();
        if handle.is_invalid() || handle == INVALID_HANDLE_VALUE {
            let err = io::Error::last_os_error();
            #[cfg(feature = "verbose_pipe_debug")]
            eprintln!("❌ [PIPE DEBUG] Invalid handle returned: {:?}", err);
            return Err(err);
        }

        #[cfg(feature = "verbose_pipe_debug")]
        eprintln!("✅ [PIPE DEBUG] Successfully connected to pipe");

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
        #[cfg(test)]
        log_server("📥 [SERVER] 開始: クライアントからバイナリデータ受信");

        // Read 4-byte length prefix
        let mut len_bytes = [0u8; 4];
        self.read_exact(&mut len_bytes)?;

        let len = u32::from_le_bytes(len_bytes) as usize;

        #[cfg(test)]
        log_server(&format!("📥 [SERVER] 受信データ長: {} bytes", len));

        // Validate reasonable length (max 10MB to prevent memory issues)
        if len > 10 * 1024 * 1024 {
            #[cfg(test)]
            log_server(&format!("❌ [SERVER] エラー: データ長が大きすぎます: {} bytes", len));
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

        #[cfg(test)]
        log_server(&format!("✅ [SERVER] 完了: {} bytes 受信しました", result.len()));

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
        #[cfg(test)]
        log_write(&format!("📤 [WRITE] 開始: バイナリデータ送信 ({} bytes)", data.len()));

        let mut bytes_written = 0u32;

        let result = unsafe { WriteFile(self.handle, Some(data), Some(&mut bytes_written), None) };

        if let Err(e) = result {
            #[cfg(test)]
            log_write(&format!("❌ [WRITE] エラー: 書き込み失敗: {:?}", e));
            return Err(io::Error::other(e));
        }

        if bytes_written as usize != data.len() {
            #[cfg(test)]
            log_write(&format!(
                "❌ [WRITE] エラー: 不完全な書き込み: {} / {} bytes",
                bytes_written,
                data.len()
            ));
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "Failed to write all bytes",
            ));
        }

        unsafe {
            FlushFileBuffers(self.handle).map_err(io::Error::other)?;
        }

        #[cfg(test)]
        log_write(&format!("✅ [WRITE] 完了: {} bytes 送信しました", bytes_written));

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
        #[cfg(test)]
        log_client("📥 [CLIENT] 開始: サーバーからレスポンス受信");

        // Read 4-byte length prefix
        let mut len_bytes = [0u8; 4];
        self.read_exact(&mut len_bytes)?;

        let len = u32::from_le_bytes(len_bytes) as usize;

        #[cfg(test)]
        log_client(&format!("📥 [CLIENT] レスポンス長: {} bytes", len));

        // Validate reasonable length
        if len > 10 * 1024 * 1024 {
            #[cfg(test)]
            log_client(&format!("❌ [CLIENT] エラー: レスポンス長が大きすぎます: {} bytes", len));
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

        #[cfg(test)]
        log_client(&format!("✅ [CLIENT] 完了: {} bytes 受信しました", result.len()));

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
