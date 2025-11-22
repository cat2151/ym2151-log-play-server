use crate::logging::*;

#[test]
fn test_init_verbose() {
    init(true);
    assert!(is_server_verbose());

    init(false);
    assert!(!is_server_verbose());
}

#[test]
fn test_verbose_default() {
    // Don't rely on initialization order
    // Just test that the function works
    let _ = is_server_verbose();
}
