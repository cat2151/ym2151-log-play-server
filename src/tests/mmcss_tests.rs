//! Tests for the MMCSS module
//!
//! These tests verify that the MMCSS API doesn't panic and behaves correctly
//! on both Windows and non-Windows platforms.

use crate::mmcss::MmcssHandle;

#[test]
fn test_mmcss_handle_creation_no_panic() {
    // This test verifies that the API doesn't panic
    // On Windows it may or may not succeed depending on system configuration
    // On non-Windows it should always return None
    let _handle = MmcssHandle::set_pro_audio_priority();
    // Test passes if we get here without panicking
}

#[test]
fn test_mmcss_handle_drop_no_panic() {
    // Test that drop doesn't panic
    if let Some(handle) = MmcssHandle::set_pro_audio_priority() {
        drop(handle);
    }
    // Test passes if we get here without panicking
}

#[test]
fn test_mmcss_handle_multiple_creation() {
    // Test that we can create multiple handles
    let _handle1 = MmcssHandle::set_pro_audio_priority();
    let _handle2 = MmcssHandle::set_pro_audio_priority();
    // Test passes if we get here without panicking
}

#[cfg(not(windows))]
#[test]
fn test_mmcss_returns_none_on_non_windows() {
    // On non-Windows platforms, MMCSS should always return None
    let handle = MmcssHandle::set_pro_audio_priority();
    assert!(
        handle.is_none(),
        "MMCSS should return None on non-Windows platforms"
    );
}

#[test]
fn test_mmcss_handle_scope() {
    // Test that the handle works correctly in different scopes
    {
        let _handle = MmcssHandle::set_pro_audio_priority();
        // Handle should be alive here
    }
    // Handle should be dropped here

    // Create another one after the first is dropped
    let _handle2 = MmcssHandle::set_pro_audio_priority();
    // Test passes if we get here without panicking
}
