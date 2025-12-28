use crate::client::config;
use std::io;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{FlushFileBuffers, ReadFile, WriteFile};

#[cfg(test)]
use super::test_logging::{log_client, log_write};

pub struct PipeWriter {
    handle: HANDLE,
    owns_handle: bool,
}

impl PipeWriter {
    /// Create a PipeWriter that borrows a handle (doesn't close on drop)
    pub fn new(handle: HANDLE) -> Self {
        PipeWriter {
            handle,
            owns_handle: false,
        }
    }

    /// Create a PipeWriter that owns a handle (closes on drop)
    pub fn new_owned(handle: HANDLE) -> Self {
        PipeWriter {
            handle,
            owns_handle: true,
        }
    }

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
        log_write(&format!(
            "📤 [WRITE] 開始: バイナリデータ送信 ({} bytes)",
            data.len()
        ));

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
        log_write(&format!(
            "✅ [WRITE] 完了: {} bytes 送信しました",
            bytes_written
        ));

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
            log_client(&format!(
                "❌ [CLIENT] エラー: レスポンス長が大きすぎます: {} bytes",
                len
            ));
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

        config::log_verbose_client(&format!("✅ [CLIENT] 受信内容: {:?}", result));

        #[cfg(test)]
        log_client(&format!(
            "✅ [CLIENT] 完了: {} bytes 受信しました",
            result.len()
        ));

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
        // Only close the handle if we own it
        if self.owns_handle {
            unsafe {
                let _ = CloseHandle(self.handle);
            }
        }
    }
}
