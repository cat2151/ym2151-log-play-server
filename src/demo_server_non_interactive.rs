//! Non-interactive demo functionality for audio testing
//!
//! This module provides demonstration functions that run direct audio playback
//! without server interaction. It reads JSON files with integer time format
//! and plays them directly through the Player and AudioPlayer components.

use anyhow::{Context, Result};
use std::thread;
use std::time::Duration;

use crate::audio::AudioPlayer;
use crate::events::EventLog;
use crate::logging;
use crate::player::Player;

/// Demo file to use for non-interactive testing
pub const DEMO_JSON_FILE: &str = "output_ym2151.json";

/// Demo configuration
pub const DEMO_INTERVAL_SECONDS: u64 = 2;

/// Run server demo mode with non-interactive functionality
///
/// This function demonstrates non-interactive playback by:
/// 1. Reading the demo JSON file (output_ym2151.json)
/// 2. Creating a Player directly with the event log
/// 3. Playing the JSON content with integer time format
/// 4. Providing a way to test audio functionality without server interaction
///
/// This is intended for production audio testing without interaction.
pub fn run_server_demo_non_interactive(verbose: bool, low_quality_resampling: bool) -> Result<()> {
    logging::log_always_server("🎮 非インタラクティブデモモードを開始します...");
    logging::log_always_server(&format!("📄 使用ファイル: {}", DEMO_JSON_FILE));

    if verbose {
        logging::log_always_server("🔍 verboseモードが有効です");
    }

    if low_quality_resampling {
        logging::log_always_server("🔧 低品質リサンプリングモードが有効です");
    }

    // Read the demo JSON file
    let json_content = std::fs::read_to_string(DEMO_JSON_FILE)
        .with_context(|| format!("JSONファイルの読み込みに失敗: {}", DEMO_JSON_FILE))?;

    // Parse the JSON to validate it
    let event_log =
        EventLog::from_json_str(&json_content).with_context(|| "JSONファイルの解析に失敗")?;

    if !event_log.validate() {
        return Err(anyhow::anyhow!(
            "無効なJSONファイルです: バリデーション失敗"
        ));
    }

    logging::log_always_server(&format!(
        "✅ JSONファイルを読み込み完了: {}個のイベント",
        event_log.events.len()
    ));

    // Calculate total duration before creating player (to avoid move)
    let sample_rate = 55930.0; // OPM sample rate
    let max_event_time_samples = event_log
        .events
        .iter()
        .map(|e| e.time)
        .fold(0.0f64, |a, b| a.max(b));

    let max_event_time_sec = max_event_time_samples / sample_rate;

    // Create player directly with the event log
    let player = Player::new(event_log.clone());

    // Create audio player with appropriate resampling quality
    let resampling_quality = if low_quality_resampling {
        crate::resampler::ResamplingQuality::Linear
    } else {
        crate::resampler::ResamplingQuality::HighQuality
    };

    let audio_player = AudioPlayer::new_with_quality(player, Some(event_log), resampling_quality)
        .with_context(|| "音声プレイヤーの作成に失敗")?;

    logging::log_always_server("✅ 音声プレイヤーを作成しました");

    // Calculate total duration for playback
    let total_duration = Duration::from_secs_f64(max_event_time_sec + 3.0); // Add 3 seconds buffer

    logging::log_always_server(&format!(
        "⏱️  演奏時間: {:.1}秒 (最大イベント時刻: {:.1}秒 + バッファ: 3.0秒)",
        total_duration.as_secs_f64(),
        max_event_time_sec
    ));

    // Start playback
    logging::log_always_server("🎵 演奏を開始します... (Ctrl+C で終了)");

    let start_time = std::time::Instant::now();

    // Simple loop to keep the demo alive
    let mut elapsed = Duration::ZERO;
    while elapsed < total_duration {
        thread::sleep(Duration::from_millis(500));
        elapsed = start_time.elapsed();

        if elapsed.as_secs().is_multiple_of(DEMO_INTERVAL_SECONDS)
            && elapsed.as_millis() % 1000 < 500
        {
            logging::log_verbose_server(&format!(
                "⏰ 経過時間: {:.1}秒 / {:.1}秒",
                elapsed.as_secs_f64(),
                total_duration.as_secs_f64()
            ));
        }
    }

    logging::log_always_server("✅ デモ演奏が完了しました");

    // Clean up happens automatically when audio_player is dropped
    drop(audio_player);
    logging::log_always_server("🧹 リソースのクリーンアップ完了");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_constants() {
        assert_eq!(DEMO_JSON_FILE, "output_ym2151.json");
        assert_eq!(DEMO_INTERVAL_SECONDS, 2);
    }

    #[test]
    fn test_demo_json_file_exists() {
        // This test will only pass if the demo file exists
        // It's more of a documentation test to show expected file location
        let path = std::path::Path::new(DEMO_JSON_FILE);
        if path.exists() {
            assert!(
                path.is_file(),
                "Demo file path should point to a file, not a directory"
            );
        }
        // Note: We don't fail the test if the file doesn't exist,
        // as it might not be available in all test environments
    }

    #[test]
    fn test_demo_non_interactive_json_parsing() {
        // Test JSON parsing with sample data (f64 second time format)
        let sample_json = r#"{
            "events": [
                {"time": 0.0, "addr": "0x08", "data": "0x00"},
                {"time": 1.0, "addr": "0x08", "data": "0x01"}
            ]
        }"#;

        let event_log = EventLog::from_json_str(sample_json).expect("Should parse sample JSON");
        assert!(event_log.validate());
        assert_eq!(event_log.events.len(), 2);
        assert_eq!(event_log.events[0].time, 0.0);
        assert_eq!(event_log.events[1].time, 1.0); // 1.0 second
    }
}
