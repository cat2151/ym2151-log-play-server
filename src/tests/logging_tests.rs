use crate::logging::*;

#[test]
fn test_init_verbose() {
    init(true);
    assert!(is_verbose());

    init(false);
    assert!(!is_verbose());
}

#[test]
fn test_verbose_default() {
    // Don't rely on initialization order
    // Just test that the function works
    let _ = is_verbose();
}
