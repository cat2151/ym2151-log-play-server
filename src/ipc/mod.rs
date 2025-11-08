



pub mod protocol;


#[cfg(unix)]
pub mod pipe_unix;

#[cfg(windows)]
pub mod pipe_windows;


#[cfg(unix)]
pub use pipe_unix as pipe;

#[cfg(windows)]
pub use pipe_windows as pipe;
