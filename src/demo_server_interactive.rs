//! Interactive demo functionality for server testing
//!
//! This module provides demonstration functions that run server operations
//! in interactive demo mode. It reads JSON files with floating-point time format
//! and uses the server's interactive mode for real-time event scheduling.

use anyhow::{Context, Result};
use std::thread;
use std::time::Duration;

use crate::audio_config::timing::*;
use crate::events::EventLog;
use crate::logging;
use crate::server::Server;

/// Demo file to use for interactive server-side testing
pub const DEMO_F64_JSON_FILE: &str = "output_ym2151_f64seconds.json";

/// Demo configuration
pub const DEMO_INTERVAL_SECONDS: u64 = 2;

/// Number of times to re-schedule the entire JSON
const RESCHEDULE_COUNT: usize = 5;

/// Interval between each re-scheduling (seconds)
const RESCHEDULE_INTERVAL_SEC: f64 = 1.1;



/// Buffer time added to total duration (seconds)
const DURATION_BUFFER_SEC: f64 = 1.5;

/// Number of events to display at start and end in verbose mode
const VERBOSE_EVENT_DISPLAY_COUNT: usize = 5;

/// Schedule all events in the event log
///
/// This function schedules all events from the provided event log using the current
/// audio stream elapsed time as the base for scheduling.
fn schedule_all_events(
    audio_player: &crate::audio::AudioPlayer,
    event_log: &EventLog,
    audio_stream_elapsed_sec: f64,
    verbose: bool,
) -> Result<()> {
    // Show initial queue state
    if let Some(queue_count) = audio_player.get_scheduled_event_count() {
        logging::log_always(&format!(
            "📊 スケジュール開始前: キューに{}個のイベント",
            queue_count
        ));
    }

    for (i, event) in event_log.events.iter().enumerate() {
        let (addr_time, data_time) = audio_player
            .schedule_register_write_fixed_time_with_future_offset(
                audio_stream_elapsed_sec,
                FUTURE_SCHEDULING_OFFSET_SEC,
                event.time,
                event.addr,
                event.data,
            )
            .with_context(|| format!("イベント{}のスケジュールに失敗", i))?;

        if verbose
            && (i < VERBOSE_EVENT_DISPLAY_COUNT
                || i >= event_log.events.len().saturating_sub(VERBOSE_EVENT_DISPLAY_COUNT))
        {
            let prefix = if i < VERBOSE_EVENT_DISPLAY_COUNT {
                "📝 [デバッグ] "
            } else {
                "📝 [デバッグ/最後] "
            };

            // Convert sample times to audio elapsed time
            let addr_elapsed_sec = crate::scheduler::samples_to_sec(addr_time);
            let data_elapsed_sec = crate::scheduler::samples_to_sec(data_time);

            // Format time values with trailing zeros trimmed
            let time_str = format!("{:.6}", event.time).trim_end_matches('0').trim_end_matches('.').to_string();
            let addr_time_str = format!("{:.6}", addr_elapsed_sec).trim_end_matches('0').trim_end_matches('.').to_string();
            let data_time_str = format!("{:.6}", data_elapsed_sec).trim_end_matches('0').trim_end_matches('.').to_string();

            logging::log_always(&format!(
                "{}イベント{}: time={}秒, スケジュール=addr:{}samples({}秒), data:{}samples({}秒), addr=0x{:02x}, data=0x{:02x}",
                prefix, i, time_str, addr_time, addr_time_str, data_time, data_time_str, event.addr, event.data
            ));
        }
    }

    // Show final queue state
    if let Some(queue_count) = audio_player.get_scheduled_event_count() {
        logging::log_always(&format!(
            "📊 スケジュール完了後: キューに{}個のイベント ({}個追加)",
            queue_count,
            event_log.events.len()
        ));
    }

    logging::log_always(&format!(
        "📝 {}個のイベントをスケジュールしました",
        event_log.events.len()
    ));

    Ok(())
}

/// Run server demo mode with interactive functionality
///
/// This function demonstrates server functionality by:
/// 1. Reading the demo JSON file (output_ym2151_f64seconds.json)
/// 2. Starting the server in interactive mode internally
/// 3. Playing the JSON content with real-time event scheduling
/// 4. Providing a way to test server functionality without client communication
///
/// This is intended for server-side testing and demonstration purposes.
pub fn run_server_demo(verbose: bool, low_quality_resampling: bool) -> Result<()> {
    logging::log_always("🎮 サーバーデモモード（インタラクティブ）を開始します...");
    logging::log_always(&format!("📄 使用ファイル: {}", DEMO_F64_JSON_FILE));

    if verbose {
        logging::log_always("🔍 [デバッグ] verboseモードが有効です");
    }

    // Read the demo JSON file
    let json_content = std::fs::read_to_string(DEMO_F64_JSON_FILE)
        .with_context(|| format!("JSONファイルの読み込みに失敗: {}", DEMO_F64_JSON_FILE))?;

    // Parse the JSON to validate it
    let event_log = EventLog::from_json_str(&json_content)
        .with_context(|| "JSONファイルの解析に失敗")?;

    if !event_log.validate() {
        return Err(anyhow::anyhow!("無効なJSONファイルです: バリデーション失敗"));
    }

    logging::log_always(&format!(
        "✅ JSONファイルを読み込み完了: {}個のイベント",
        event_log.events.len()
    ));

    // Create server instance
    let server = Server::new_with_resampling_quality(low_quality_resampling);

    logging::log_always("🎵 サーバー内部でインタラクティブモードを開始中...");

    // Start interactive mode internally
    let audio_player = server.start_interactive_mode_demo()
        .with_context(|| "インタラクティブモードの開始に失敗")?;

    logging::log_always("✅ インタラクティブモード開始完了");

    // Get initial server time for scheduling
    let start_time = std::time::Instant::now();

    // Wait for audio system to stabilize
    std::thread::sleep(std::time::Duration::from_millis(AUDIO_STABILIZATION_WAIT_MS));

    logging::log_always(&format!(
        "🎶 デモ演奏を開始します... ({}回の再スケジューリング、{:.1}秒間隔)",
        RESCHEDULE_COUNT, RESCHEDULE_INTERVAL_SEC
    ));

    // Multi-round scheduling with 1-second intervals
    for round in 0..RESCHEDULE_COUNT {
        let current_audio_elapsed = audio_player
            .get_audio_elapsed_sec()
            .ok_or_else(|| anyhow::anyhow!("音声経過時間の取得に失敗"))?;

        logging::log_always(&format!(
            "🔄 ラウンド {}/{}: {}個のイベントを{:.1}秒後にスケジュール (音声経過時間: {:.6}秒)",
            round + 1,
            RESCHEDULE_COUNT,
            event_log.events.len(),
            if round == 0 { 0.0 } else { RESCHEDULE_INTERVAL_SEC },
            current_audio_elapsed
        ));

        // Clear previous schedule (except for first round)
        if round > 0 {
            // Show queue state before clearing
            if let Some(queue_count) = audio_player.get_scheduled_event_count() {
                logging::log_always(&format!(
                    "🧹 スケジュールクリア前: キューに{}個のイベント",
                    queue_count
                ));
            }

            audio_player.clear_schedule();

            // Show queue state after clearing
            if let Some(queue_count) = audio_player.get_scheduled_event_count() {
                logging::log_always(&format!(
                    "🧹 スケジュールクリア後: キューに{}個のイベント",
                    queue_count
                ));
            } else {
                logging::log_always("🧹 前回のスケジュールをクリアしました");
            }
        }

        if verbose && round == 0 {
            logging::log_always(&format!(
                "🕐 [デバッグ] タイミング情報:"
            ));
            logging::log_always(&format!(
                "   - 音声開始からの経過時間: {:.6}秒",
                current_audio_elapsed
            ));
            logging::log_always(&format!(
                "   - 未来オフセット: {:.3}秒 ({}ms, {}samples)",
                FUTURE_SCHEDULING_OFFSET_SEC,
                (FUTURE_SCHEDULING_OFFSET_SEC * 1000.0) as u32,
                crate::scheduler::sec_to_samples(FUTURE_SCHEDULING_OFFSET_SEC)
            ));
            logging::log_always(&format!(
                "   - 音声安定化待機: {}ms",
                AUDIO_STABILIZATION_WAIT_MS
            ));
            logging::log_always(&format!(
                "   - タイムスタンプの種類: 音声ストリーム基準 (連続時間)"
            ));
            logging::log_always(&format!(
                "   - OPMサンプルレート: {}Hz",
                crate::resampler::OPM_SAMPLE_RATE
            ));
            if let Some(first_event) = event_log.events.first() {
                let first_scheduled_samples = crate::scheduler::sec_to_samples(
                    current_audio_elapsed + FUTURE_SCHEDULING_OFFSET_SEC + first_event.time,
                );
                logging::log_always(&format!(
                    "   - 最初のイベント: time={:.6}秒, scheduled_samples={}",
                    first_event.time, first_scheduled_samples
                ));
            }
        }

        // Schedule all events
        schedule_all_events(&audio_player, &event_log, current_audio_elapsed, verbose)?;

        // Wait for next round (except for last round)
        if round < RESCHEDULE_COUNT - 1 {
            logging::log_always(&format!(
                "⏳ {:.1}秒待機中...",
                RESCHEDULE_INTERVAL_SEC
            ));
            thread::sleep(Duration::from_secs_f64(RESCHEDULE_INTERVAL_SEC));
        }
    }

    // Calculate total duration and wait
    let max_event_time = event_log.events
        .iter()
        .map(|e| e.time)
        .fold(0.0f64, |a, b| a.max(b));

    let total_duration = Duration::from_secs_f64(max_event_time + DURATION_BUFFER_SEC); // Add buffer

    logging::log_always(&format!(
        "⏱️  演奏時間: {:.1}秒 (最大イベント時刻: {:.1}秒 + バッファ: {:.1}秒)",
        total_duration.as_secs_f64(),
        max_event_time,
        DURATION_BUFFER_SEC
    ));

    // Keep the demo running
    logging::log_always("🎵 演奏中... (Ctrl+C で終了)");

    // Simple loop to keep the demo alive
    let mut elapsed = Duration::ZERO;
    while elapsed < total_duration {
        thread::sleep(Duration::from_millis(500));
        elapsed = start_time.elapsed();

        if elapsed.as_secs() % DEMO_INTERVAL_SECONDS == 0 && elapsed.as_millis() % 1000 < 500 {
            if let Some(audio_elapsed) = audio_player.get_audio_elapsed_sec() {
                logging::log_verbose(&format!(
                    "⏰ 経過時間: {:.1}秒 / {:.1}秒 (音声基準: {:.1}秒)",
                    elapsed.as_secs_f64(),
                    total_duration.as_secs_f64(),
                    audio_elapsed
                ));
            } else {
                logging::log_verbose(&format!(
                    "⏰ 経過時間: {:.1}秒 / {:.1}秒",
                    elapsed.as_secs_f64(),
                    total_duration.as_secs_f64()
                ));
            }
        }
    }

    logging::log_always("✅ デモ演奏が完了しました");

    // Clean up
    drop(audio_player);
    logging::log_always("🧹 リソースのクリーンアップ完了");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_constants() {
        assert_eq!(DEMO_F64_JSON_FILE, "output_ym2151_f64seconds.json");
        assert_eq!(DEMO_INTERVAL_SECONDS, 2);
    }

    #[test]
    fn test_demo_json_file_exists() {
        // This test will only pass if the demo file exists
        // It's more of a documentation test to show expected file location
        let path = std::path::Path::new(DEMO_F64_JSON_FILE);
        if path.exists() {
            assert!(path.is_file(), "Demo file path should point to a file, not a directory");
        }
        // Note: We don't fail the test if the file doesn't exist,
        // as it might not be available in all test environments
    }

    #[test]
    fn test_demo_json_parsing() {
        // Test JSON parsing with sample data (floating-point time format)
        let sample_json = r#"{
            "events": [
                {"time": 0.0, "addr": "0x08", "data": "0x00"},
                {"time": 0.5, "addr": "0x08", "data": "0x01"}
            ]
        }"#;

        let event_log = EventLog::from_json_str(sample_json).expect("Should parse sample JSON");
        assert!(event_log.validate());
        assert_eq!(event_log.events.len(), 2);
        assert_eq!(event_log.events[0].time, 0.0);
        assert_eq!(event_log.events[1].time, 0.5);
    }
}
