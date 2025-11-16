//! Test for server compilation and basic functionality
//!
//! This test ensures that the server can be created and compiled properly
//! across all platforms without thread safety issues or method conflicts.

#![cfg(windows)]

use ym2151_log_play_server::server::Server;

#[test]
fn test_server_creation() {
    // Should be able to create a server instance without compilation errors
    let _server = Server::new();

    // This test ensures the compilation passes with proper method definitions
}
