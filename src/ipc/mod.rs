/// IPC (Inter-Process Communication) module for server-client communication
///
/// This module provides the protocol and communication primitives for
/// the YM2151 server-client architecture.
pub mod protocol;

// Platform-specific named pipe implementations
#[cfg(unix)]
pub mod pipe_unix;

#[cfg(windows)]
pub mod pipe_windows;

// Re-export platform-specific pipe module for convenience
#[cfg(unix)]
pub use pipe_unix as pipe;

#[cfg(windows)]
pub use pipe_windows as pipe;
