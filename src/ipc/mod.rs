#[cfg(windows)]
pub mod pipe_windows;
#[cfg(windows)]
pub use pipe_windows as pipe;

pub mod protocol;
