//! Windows MMCSS (Multimedia Class Scheduler Service) support
//!
//! This module provides thread priority boosting for audio processing threads
//! on Windows using the MMCSS Pro Audio task profile. This helps reduce
//! audio dropouts and glitches by giving real-time priority to audio threads.

#[cfg(windows)]
use windows::Win32::Media::Audio::{
    AvSetMmThreadCharacteristicsW, AvRevertMmThreadCharacteristics,
};
#[cfg(windows)]
use windows::Win32::Foundation::HANDLE;
#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use std::ffi::OsStr;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

/// RAII wrapper for MMCSS thread characteristics
///
/// When created, this sets the current thread to use the "Pro Audio" MMCSS task.
/// When dropped, it automatically reverts the thread characteristics.
#[cfg(windows)]
pub struct MmcssHandle {
    task_handle: isize,
}

#[cfg(windows)]
impl MmcssHandle {
    /// Set the current thread to use the "Pro Audio" MMCSS task
    ///
    /// This raises the thread priority for real-time audio processing.
    /// Returns None if MMCSS is not available or the operation fails.
    pub fn set_pro_audio_priority() -> Option<Self> {
        unsafe {
            // Convert "Pro Audio" to wide string for Windows API
            let task_name: Vec<u16> = OsStr::new("Pro Audio")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            
            let mut task_index = 0u32;
            
            match AvSetMmThreadCharacteristicsW(
                PCWSTR(task_name.as_ptr()),
                &mut task_index,
            ) {
                Ok(handle) => {
                    crate::logging::log_verbose(&format!(
                        "MMCSS Pro Audio priority enabled for generator thread (handle: {:?}, index: {})",
                        handle, task_index
                    ));
                    Some(MmcssHandle {
                        task_handle: handle.0,
                    })
                }
                Err(e) => {
                    crate::logging::log_verbose(&format!(
                        "Warning: Failed to enable MMCSS Pro Audio priority: {}",
                        e
                    ));
                    None
                }
            }
        }
    }
}

#[cfg(windows)]
impl Drop for MmcssHandle {
    fn drop(&mut self) {
        unsafe {
            if self.task_handle != 0 {
                let handle = HANDLE(self.task_handle);
                if let Err(e) = AvRevertMmThreadCharacteristics(handle) {
                    crate::logging::log_verbose(&format!(
                        "Warning: Failed to revert MMCSS characteristics: {}",
                        e
                    ));
                } else {
                    crate::logging::log_verbose("MMCSS Pro Audio priority reverted");
                }
            }
        }
    }
}

/// Non-Windows stub for MmcssHandle
///
/// On non-Windows platforms, this is a no-op type that does nothing.
#[cfg(not(windows))]
pub struct MmcssHandle;

#[cfg(not(windows))]
impl MmcssHandle {
    /// No-op implementation for non-Windows platforms
    pub fn set_pro_audio_priority() -> Option<Self> {
        // On non-Windows platforms, just return None (no MMCSS available)
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mmcss_handle_creation() {
        // This test verifies that the API doesn't panic
        // On Windows it may or may not succeed depending on system configuration
        // On non-Windows it should always return None
        let _handle = MmcssHandle::set_pro_audio_priority();
        // Test passes if we get here without panicking
    }

    #[test]
    fn test_mmcss_handle_drop() {
        // Test that drop doesn't panic
        if let Some(handle) = MmcssHandle::set_pro_audio_priority() {
            drop(handle);
        }
        // Test passes if we get here without panicking
    }
}
