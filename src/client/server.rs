//! Server management functionality for the client
//!
//! This module handles server lifecycle management including checking if the server
//! is running, starting the server, and installing server applications.

use super::config::log_verbose_client;
use crate::ipc::pipe_windows::NamedPipe;
use anyhow::{Context, Result};
use std::process::Command as ProcessCommand;

const RETRY_INITIAL_WAIT_MS: u64 = 1;
const RETRY_MAX_WAIT_MS: u64 = 50; // 指数関数的バックオフを利用し、応答速度と堅牢性のバランスを取る

/// Ensure the server is running and ready to accept commands
///
/// This function ensures that the YM2151 server is running and ready to accept
/// commands. It provides a seamless developer experience by automatically:
/// 1. Checking if the server is already running
/// 2. Locating the server executable (test binary in test builds, PATH, or install via cargo)
/// 3. Starting the server if not running
/// 4. Waiting until the server is ready to accept commands
///
/// # Test Context Support (Windows only, cfg(test) builds)
/// When compiled with cfg(test) (e.g., `cargo test`), this function automatically
/// detects and uses the test-built binary from `target/debug` or `target/debug/deps`
/// instead of requiring the binary to be in PATH. This enables seamless integration
/// testing without manual setup.
///
/// In non-test builds (e.g., `cargo build`, `cargo run`), this function uses the
/// standard PATH search or cargo install, ensuring predictable behavior in production.
///
/// # Arguments
/// * `server_app_name` - Name of the server application (e.g., "cat-play-mml")
///
/// # Example
/// ```no_run
/// # use ym2151_log_play_server::client::server;
/// // Ensure server is ready before playing music
/// server::ensure_server_ready("cat-play-mml")?;
///
/// // Now the server is guaranteed to be running and ready
/// # Ok::<(), anyhow::Error>(())
/// ```
///
/// # Errors
/// Returns an error if:
/// - Failed to install the server application
/// - Failed to start the server
/// - Server doesn't become ready within a reasonable timeout
pub fn ensure_server_ready(server_app_name: &str) -> Result<()> {
    log_verbose_client("🔍 サーバーの状態を確認中...");

    if is_server_running_with_retry() {
        log_verbose_client("✅ サーバーは既に起動しています");
        return Ok(());
    }

    log_verbose_client("⚙️  サーバーが起動していません。起動準備中...");

    #[cfg(all(windows, test))]
    let server_path = {
        // In test builds, try to find the binary in test context first
        if let Some(test_binary) = get_test_binary_path(server_app_name) {
            log_verbose_client(&format!("🧪 テストコンテキストを検出: {:?}", test_binary));
            test_binary.to_string_lossy().to_string()
        } else if is_app_in_path(server_app_name) {
            // Use the app from PATH
            server_app_name.to_string()
        } else {
            // Not in test context and not in PATH, install it
            log_verbose_client(&format!(
                "📦 {} が見つかりません。cargo経由でインストール中...",
                server_app_name
            ));
            install_app_via_cargo(server_app_name)
                .with_context(|| format!("Failed to install {}", server_app_name))?;
            log_verbose_client(&format!(
                "✅ {} のインストールが完了しました",
                server_app_name
            ));
            server_app_name.to_string()
        }
    };

    #[cfg(all(windows, not(test)))]
    let server_path = {
        // In non-test builds, use PATH or install
        if is_app_in_path(server_app_name) {
            // Use the app from PATH
            server_app_name.to_string()
        } else {
            // Not in PATH, install it
            log_verbose_client(&format!(
                "📦 {} が見つかりません。cargo経由でインストール中...",
                server_app_name
            ));
            install_app_via_cargo(server_app_name)
                .with_context(|| format!("Failed to install {}", server_app_name))?;
            log_verbose_client(&format!(
                "✅ {} のインストールが完了しました",
                server_app_name
            ));
            server_app_name.to_string()
        }
    };

    #[cfg(not(windows))]
    let server_path = {
        // On non-Windows platforms, use the original logic
        if !is_app_in_path(server_app_name) {
            log_verbose_client(&format!(
                "📦 {} が見つかりません。cargo経由でインストール中...",
                server_app_name
            ));
            install_app_via_cargo(server_app_name)
                .with_context(|| format!("Failed to install {}", server_app_name))?;
            log_verbose_client(&format!(
                "✅ {} のインストールが完了しました",
                server_app_name
            ));
        }
        server_app_name.to_string()
    };

    log_verbose_client("🚀 サーバーを起動中...");
    start_server(&server_path)
        .with_context(|| format!("Failed to start server: {}", server_app_name))?;

    log_verbose_client("⏳ サーバーの起動完了を待機中...");
    if !is_server_running_with_retry() {
        return Err(anyhow::anyhow!(
            "Server failed to become ready within timeout"
        ));
    }

    log_verbose_client("✅ サーバーが起動し、コマンド受付可能になりました");
    Ok(())
}

/// Check if the server is currently running
pub fn is_server_running_with_retry() -> bool {
    // 前提として、当関数は「サーバーが起動しているにも関わらずfalseをreturnするリスク」が常にある。connect_defaultが非決定論的ふるまいのため。race conditionにより、サーバーがpipeをcreateする直前でconnect_defaultがErrとなる可能性が常にあるため。リスク対策として指数関数的バックオフを利用しており、処理速度を犠牲にするほどにリスクを低減できる。匙加減は今後検証でチューニング予定。
    log_verbose_client("🔍 [Server存在チェック] サーバーへの接続を試行中...");

    let mut wait_ms = RETRY_INITIAL_WAIT_MS;
    loop {
        match NamedPipe::connect_default() {
            Ok(_) => {
                log_verbose_client(
                    "✅ [Server存在チェック] サーバーが起動していることを確認しました",
                );
                return true;
            }
            Err(e) => {
                log_verbose_client(&format!(
                    "❌ [Server存在チェック] サーバーが起動していないか、起動していてもrace conditionです: {:?}",
                    e
                ));
                if wait_ms >= RETRY_MAX_WAIT_MS {
                    log_verbose_client(&format!(
                        "❌ [Server存在チェック] 最大待機時間({}ms)を超過。おそらくサーバーが起動していません。",
                        RETRY_MAX_WAIT_MS
                    ));
                    return false;
                }
                std::thread::sleep(std::time::Duration::from_millis(wait_ms));
                wait_ms *= 2;
            }
        }
    }
}

/// Get the binary path when running in test context
///
/// During testing, the binary is located in target/debug or target/debug/deps,
/// not in PATH. This function locates the binary in the test build directory.
///
/// This function is only available when compiled with cfg(test), ensuring
/// predictable behavior - it only searches for test binaries in test builds.
///
/// # Arguments
/// * `binary_name` - Name of the binary to find (e.g., "ym2151-log-play-server")
///
/// # Returns
/// * `Some(PathBuf)` - Path to the binary if found in test context
/// * `None` - Not in test context or binary not found
#[cfg(all(windows, test))]
fn get_test_binary_path(binary_name: &str) -> Option<std::path::PathBuf> {
    // Try to get current exe path (works in test context)
    let current_exe = std::env::current_exe().ok()?;

    // Get the directory containing the test executable
    let mut path = current_exe.parent()?.to_path_buf();

    // In debug/test mode, we might be in deps directory
    if path.ends_with("deps") {
        path = path.parent()?.to_path_buf();
    }

    // Try with .exe extension (Windows)
    let exe_name = format!("{}.exe", binary_name);
    path.push(&exe_name);

    // Check if the binary exists
    if path.exists() {
        log_verbose_client(&format!("🔍 テストバイナリを検出: {:?}", path));
        return Some(path);
    }

    // Try without .exe extension
    path.pop();
    path.push(binary_name);
    if path.exists() {
        log_verbose_client(&format!("🔍 テストバイナリを検出: {:?}", path));
        return Some(path);
    }

    log_verbose_client(&format!(
        "⚠️  テストバイナリが見つかりません: {} (検索場所: {:?})",
        binary_name, path
    ));
    None
}

/// Check if an application is available in PATH
pub fn is_app_in_path(app_name: &str) -> bool {
    which::which(app_name).is_ok()
}

/// Install an application via cargo
fn install_app_via_cargo(app_name: &str) -> Result<()> {
    let output = ProcessCommand::new("cargo")
        .args([
            "install",
            "--git",
            &format!("https://github.com/cat2151/{}", app_name),
        ])
        .output()
        .context("Failed to execute cargo install")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("cargo install failed: {}", stderr));
    }

    Ok(())
}

/// Start the server application in background mode
///
/// # Arguments
/// * `server_path` - Path to the server executable (can be name in PATH or full path)
fn start_server(server_path: &str) -> Result<()> {
    let arg = if server_path.contains("ym2151-log-play-server") {
        "server"
    } else {
        "--server"
    };

    ProcessCommand::new(server_path)
        .arg(arg)
        .spawn()
        .context("Failed to spawn server process")?;

    Ok(())
}

// Test-only helper functions
#[cfg(all(test, windows))]
pub mod test_helpers {
    /// Expose get_test_binary_path for testing
    pub fn get_test_binary_path_helper(binary_name: &str) -> Option<std::path::PathBuf> {
        super::get_test_binary_path(binary_name)
    }
}
