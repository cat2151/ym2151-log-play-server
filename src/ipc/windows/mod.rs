//! Windows named pipe implementation
//!
//! This module provides a platform-specific implementation of named pipes for Windows,
//! organized according to the Single Responsibility Principle:
//!
//! - `pipe_factory` - Low-level pipe creation and connection
//! - `pipe_handle` - High-level pipe management and coordination
//! - `pipe_reader` - Data reading operations
//! - `pipe_writer` - Data writing operations
//! - `test_logging` - Test-only logging infrastructure

pub mod pipe_factory;
pub mod pipe_handle;
pub mod pipe_reader;
pub mod pipe_writer;

#[cfg(test)]
pub mod test_logging;

// Re-export main types and constants for backward compatibility
pub use pipe_handle::{NamedPipe, DEFAULT_PIPE_PATH};
pub use pipe_reader::PipeReader;
pub use pipe_writer::PipeWriter;

// Re-export test functions when in test mode
#[cfg(test)]
pub use test_logging::{set_client_context as test_set_client_context, set_server_context as test_set_server_context};
