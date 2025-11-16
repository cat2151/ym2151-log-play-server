//! Example demonstrating verbose mode for client operations
//!
//! This example shows how to enable verbose output when using the client module.
//! Verbose mode is useful for debugging and development.

#[cfg(windows)]
fn main() {
    use ym2151_log_play_server::client;

    // Enable verbose mode for debugging
    client::init_client(true);

    println!("Client verbose mode enabled.");
    println!("When calling client functions, status messages will be printed to stderr.");
    println!();
    println!("Example: Checking if server is running...");

    // This would print status messages if a server operation was attempted
    // (We're not actually connecting to avoid requiring a running server for this example)

    println!();
    println!("Verbose mode is useful for:");
    println!("  - Debugging client-server communication");
    println!("  - Development and testing");
    println!("  - Command-line applications");
}

#[cfg(not(windows))]
fn main() {
    println!("Client module is only available on Windows.");
    println!("This example cannot run on non-Windows platforms.");
}
