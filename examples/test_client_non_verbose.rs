//! Example demonstrating non-verbose mode for client operations
//!
//! This example shows how to disable verbose output when using the client module.
//! Non-verbose mode (the default) is essential for TUI applications to prevent
//! display corruption.

#[cfg(windows)]
fn main() {
    use ym2151_log_play_server::client;

    // Disable verbose mode for TUI applications (this is the default)
    client::init_client(false);

    println!("Client non-verbose mode enabled (default).");
    println!("When calling client functions, NO status messages will be printed.");
    println!("This prevents TUI display corruption.");
    println!();
    println!("Non-verbose mode is essential for:");
    println!("  - TUI applications (like ym2151-tone-editor)");
    println!("  - Applications that need clean output");
    println!("  - Production use where output should be minimal");
    println!();
    println!("In non-verbose mode, client operations are silent.");
    println!("Only errors returned as Result::Err will need to be handled by the application.");
}

#[cfg(not(windows))]
fn main() {
    println!("Client module is only available on Windows.");
    println!("This example cannot run on non-Windows platforms.");
}
