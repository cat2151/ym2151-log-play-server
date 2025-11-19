#[cfg(windows)]
use ym2151_log_play_server::server::{Server, ServerState};

#[cfg(windows)]
#[test]
fn test_server_creation() {
    let server = Server::new();
    assert_eq!(server.get_state(), ServerState::Stopped);
    assert!(!server.is_shutdown_requested());
}

#[cfg(windows)]
#[test]
fn test_server_default() {
    let server = Server::default();
    assert_eq!(server.get_state(), ServerState::Stopped);
}
