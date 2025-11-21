use std::io;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::ReadFile;

#[cfg(test)]
use super::test_logging::log_server;

pub struct PipeReader {
    handle: HANDLE,
}

impl PipeReader {
    pub fn new(handle: HANDLE) -> Self {
        PipeReader { handle }
    }

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
            log_server(&format!(
                "❌ [SERVER] エラー: データ長が大きすぎます: {} bytes",
                len
            ));
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
        log_server(&format!(
            "✅ [SERVER] 完了: {} bytes 受信しました",
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
                    "Pipe closed before reading complete message",
                ));
            }

            total_read += bytes_read as usize;
        }

        Ok(())
    }
}
