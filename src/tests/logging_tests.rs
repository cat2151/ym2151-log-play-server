use crate::logging::*;
use std::path::PathBuf;

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

#[test]
fn test_open_log_file_at_creates_parent_directory() {
    let unique = format!(
        "ym2151-log-test-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let log_dir = std::env::temp_dir().join(unique);
    let log_path = log_dir.join("nested").join("test.log");

    let file = open_log_file_at(&log_path).unwrap();
    drop(file);

    assert!(log_path.exists());

    let _ = std::fs::remove_file(&log_path);
    let _ = std::fs::remove_dir_all(&log_dir);
}

#[cfg(windows)]
#[test]
fn test_log_directory_uses_local_appdata_on_windows() {
    let local_appdata = PathBuf::from(r"C:\Users\tester\AppData\Local");

    assert_eq!(
        log_directory_from_env(Some(local_appdata.clone()), None, None),
        local_appdata.join("ym2151-log-play-server")
    );
}

#[cfg(target_os = "macos")]
#[test]
fn test_log_directory_uses_application_support_on_macos() {
    let home = PathBuf::from("/Users/tester");

    assert_eq!(
        log_directory_from_env(None, None, Some(home.clone())),
        home.join("Library")
            .join("Application Support")
            .join("ym2151-log-play-server")
    );
}

#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn test_log_directory_uses_xdg_config_home_on_unix() {
    let xdg_config_home = PathBuf::from("/tmp/xdg-config-home");
    let home = PathBuf::from("/tmp/home");

    assert_eq!(
        log_directory_from_env(None, Some(xdg_config_home.clone()), Some(home)),
        xdg_config_home.join("ym2151-log-play-server")
    );
}

#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn test_log_directory_falls_back_to_home_config_on_unix() {
    let home = PathBuf::from("/tmp/home");

    assert_eq!(
        log_directory_from_env(None, None, Some(home.clone())),
        home.join(".config").join("ym2151-log-play-server")
    );
}
