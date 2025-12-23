use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING, PIPE_ACCESS_DUPLEX,
};
use windows::Win32::System::Pipes::{
    CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_TYPE_BYTE, PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};

/// Create a Windows named pipe at the specified path
pub fn create_named_pipe<P: AsRef<Path>>(path: P) -> io::Result<HANDLE> {
    let path = path.as_ref();

    #[cfg(feature = "verbose_pipe_debug")]
    eprintln!("🔧 [PIPE DEBUG] Creating named pipe at: {:?}", path);

    let wide_path = path_to_wide_string(path);

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

    Ok(handle)
}

/// Connect to an existing Windows named pipe
pub fn connect_to_pipe<P: AsRef<Path>>(path: P) -> io::Result<HANDLE> {
    let path = path.as_ref();

    #[cfg(feature = "verbose_pipe_debug")]
    eprintln!("🔌 [PIPE DEBUG] Attempting to connect to pipe: {:?}", path);

    let wide_path = path_to_wide_string(path);

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

    Ok(handle)
}

/// Convert path to null-terminated wide string for Windows API
fn path_to_wide_string<P: AsRef<Path>>(path: P) -> Vec<u16> {
    OsStr::new(path.as_ref().as_os_str())
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
