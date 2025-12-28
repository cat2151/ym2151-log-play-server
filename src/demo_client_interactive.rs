//! Client interactive demo functionality for showcasing interactive mode capabilities
//!
//! This module provides demonstration functions that show how to use the
//! interactive mode for continuous playback with JSON data.
//!
//! Windows専用モジュールです。

use anyhow::Result;
use std::thread;
use std::time::Duration;

use crate::client;

const DEMO_JSON_FILE: &str = "output_ym2151.json";
const DEMO_REPEAT_COUNT: usize = 5;
const DEMO_INTERVAL_SECONDS: f64 = 1.0;

/// Run interactive mode demo with output_ym2151.json
///
/// This function demonstrates the interactive mode by:
/// 1. Ensuring the server is ready
/// 2. Starting interactive mode
/// 3. Playing the JSON file 5 times with 1-second intervals
/// 4. Stopping interactive mode when done
///
/// # Examples
///
/// ```no_run
/// use ym2151_log_play_server::demo_client_interactive;
///
/// demo_client_interactive::run_interactive_demo(true)?; // With verbose output
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn run_interactive_demo(verbose: bool) -> Result<()> {
    // Initialize client with verbose setting
    client::init_client(verbose);

    // Ensure server is ready - use correct binary name for this project
    client::ensure_server_ready("ym2151-log-play-server")?;

    // Read the JSON file
    let json_content = std::fs::read_to_string(DEMO_JSON_FILE)
        .map_err(|e| anyhow::anyhow!("JSONファイルの読み込みに失敗: {}: {}", DEMO_JSON_FILE, e))?;

    client::log_verbose_client("インタラクティブモードデモを開始します...");
    client::log_verbose_client(&format!(
        "ファイル: {} を {}秒間隔で {}回演奏",
        DEMO_JSON_FILE, DEMO_INTERVAL_SECONDS, DEMO_REPEAT_COUNT
    ));

    // Start interactive mode
    client::start_interactive()?;

    // Repeat playback
    for i in 1..=DEMO_REPEAT_COUNT {
        client::log_verbose_client(&format!("演奏回数: {}/{}", i, DEMO_REPEAT_COUNT));

        // Send JSON to interactive mode
        // The server automatically clears conflicting future events
        client::play_json_interactive(&json_content)?;

        // Wait before next iteration (except for the last one)
        if i < DEMO_REPEAT_COUNT {
            thread::sleep(Duration::from_secs_f64(DEMO_INTERVAL_SECONDS));
        }
    }

    // Stop interactive mode
    client::stop_interactive()?;

    client::log_verbose_client("インタラクティブモードデモが完了しました");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_constants() {
        assert_eq!(DEMO_JSON_FILE, "output_ym2151.json");
        assert_eq!(DEMO_REPEAT_COUNT, 5);
        assert_eq!(DEMO_INTERVAL_SECONDS, 1.0);
    }
}
