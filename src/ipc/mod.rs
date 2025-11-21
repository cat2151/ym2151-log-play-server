pub mod protocol;

#[cfg(windows)]
pub mod pipe_windows {
    //! Backward compatibility re-export for the old pipe_windows module
    pub use super::windows::*;
}

#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows as pipe;
