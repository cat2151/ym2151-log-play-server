use crate::audio::AudioPlayer;
use crate::events::EventLog;
use crate::ipc::protocol::{Command, Response};
use crate::logging;
use crate::scheduler::TimeTracker;
use crate::server::playback::PlaybackManager;
use crate::server::state::ServerState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Handles processing of client commands
pub struct CommandHandler {
    state: Arc<Mutex<ServerState>>,
    shutdown_flag: Arc<AtomicBool>,
    time_tracker: Arc<Mutex<TimeTracker>>,
    playback_manager: PlaybackManager,
}

impl CommandHandler {
    pub fn new(
        state: Arc<Mutex<ServerState>>,
        shutdown_flag: Arc<AtomicBool>,
        time_tracker: Arc<Mutex<TimeTracker>>,
        playback_manager: PlaybackManager,
    ) -> Self {
        Self {
            state,
            shutdown_flag,
            time_tracker,
            playback_manager,
        }
    }

    /// Process a command and return the response and optionally a new audio player
    pub fn handle_command(
        &self,
        command: Command,
        audio_player: &mut Option<AudioPlayer>,
    ) -> Response {
        let state = self.state.lock().unwrap();
        logging::log_verbose_server(&format!(
            "💡 [{}] 現在のサーバー状態: {:?}",
            std::any::type_name::<Self>(),
            *state
        ));
        drop(state); // Release lock before handling command

        match command {
            Command::PlayJson { data } => self.handle_play_json(data, audio_player),
            Command::Stop => self.handle_stop(audio_player),
            Command::StartInteractive => self.handle_start_interactive(audio_player),
            Command::GetServerTime => self.handle_get_server_time(),
            Command::StopInteractive => self.handle_stop_interactive(audio_player),
            Command::PlayJsonInInteractive { data } => {
                self.handle_play_json_in_interactive(data, audio_player)
            }
            Command::GetServerState => self.handle_get_server_state(),
            Command::Shutdown => {
                // Shutdown is handled specially in the connection loop
                // This should not be reached
                Response::Ok
            }
        }
    }

    fn handle_get_server_state(&self) -> Response {
        let state = self.state.lock().unwrap();
        let current_state = state.as_str().to_string();

        let fn_name = std::any::type_name::<Self>();
        logging::log_verbose_server(&format!("🔍 [{}] サーバー状態: {}", fn_name, current_state));

        Response::ServerState {
            state: current_state,
        }
    }

    /// Check if shutdown has been requested
    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }

    /// Request shutdown
    pub fn request_shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
    }

    fn handle_play_json(
        &self,
        data: serde_json::Value,
        audio_player: &mut Option<AudioPlayer>,
    ) -> Response {
        logging::log_verbose_server("🎵 JSON データを読み込み中...");

        // Stop any existing playback
        if let Some(mut player) = audio_player.take() {
            player.stop();
        }

        // Convert JSON value to string for parsing
        let json_result = serde_json::to_string(&data);

        match json_result {
            Ok(json_str) => match self
                .playback_manager
                .load_and_start_playback(&json_str, true)
            {
                Ok(player) => {
                    *audio_player = Some(player);
                    logging::log_verbose_server("✅ JSON データから音声再生を開始しました");

                    let mut state = self.state.lock().unwrap();
                    *state = ServerState::Playing;

                    Response::Ok
                }
                Err(e) => {
                    logging::log_always_server(&format!("❌ 音声再生の開始に失敗しました: {}", e));
                    Response::Error {
                        message: format!("Failed to start playback: {}", e),
                    }
                }
            },
            Err(e) => {
                logging::log_always_server(&format!("❌ JSONシリアライズに失敗しました: {}", e));
                Response::Error {
                    message: format!("Failed to serialize JSON: {}", e),
                }
            }
        }
    }

    fn handle_stop(&self, audio_player: &mut Option<AudioPlayer>) -> Response {
        logging::log_verbose_server("⏹️  音声再生を停止中...");
        if let Some(mut player) = audio_player.take() {
            player.stop();
        }

        let mut state = self.state.lock().unwrap();
        *state = ServerState::Stopped;

        logging::log_verbose_server("✅ 音声再生を停止しました");
        Response::Ok
    }

    fn handle_start_interactive(&self, audio_player: &mut Option<AudioPlayer>) -> Response {
        logging::log_verbose_server("🎮 インタラクティブモードを開始中...");
        logging::log_verbose_server(&format!(
            "🔍現在のサーバー状態: {:?}",
            *self.state.lock().unwrap()
        ));

        // Stop any existing playback
        if let Some(mut player) = audio_player.take() {
            logging::log_verbose_server("⏹️ 既存の再生を停止中...");
            player.stop();
        }

        // Reset time tracker for new interactive session
        {
            let mut tracker = self.time_tracker.lock().unwrap();
            tracker.reset();
            logging::log_verbose_server("🕐タイムトラッカーをリセットしました");
        }

        // Start interactive mode
        logging::log_verbose_server("🎵インタラクティブオーディオプレーヤーを作成中...");
        match self.playback_manager.start_interactive_mode() {
            Ok(player) => {
                *audio_player = Some(player);
                logging::log_verbose_server("✅ インタラクティブモードを開始しました");
                logging::log_verbose_server("🔊音声ストリーミング開始");

                let mut state = self.state.lock().unwrap();
                *state = ServerState::Interactive;
                logging::log_verbose_server(&format!("📊サーバー状態を更新: {:?}", *state));

                Response::Ok
            }
            Err(e) => {
                logging::log_always_server(&format!(
                    "❌ インタラクティブモードの開始に失敗しました: {}",
                    e
                ));
                logging::log_always_server("💡 [デバッグ情報] 以下を確認してください:");
                logging::log_always_server("   1. 音声デバイスが利用可能か");
                logging::log_always_server(
                    "   2. 他のアプリケーションが音声デバイスを使用していないか",
                );
                logging::log_always_server("   3. システムの音量設定");
                Response::Error {
                    message: format!("Failed to start interactive mode: {}", e),
                }
            }
        }
    }

    fn handle_get_server_time(&self) -> Response {
        let tracker = self.time_tracker.lock().unwrap();
        let time_sec = tracker.elapsed_sec();
        logging::log_verbose_server(&format!("⏰ サーバー時刻を取得: {:.6} 秒", time_sec));
        Response::ServerTime { time_sec }
    }

    fn handle_stop_interactive(&self, audio_player: &mut Option<AudioPlayer>) -> Response {
        logging::log_verbose_server("⏹️  インタラクティブモードを停止中...");
        logging::log_verbose_server(&format!(
            "🔍現在のサーバー状態: {:?}",
            *self.state.lock().unwrap()
        ));

        if let Some(mut player) = audio_player.take() {
            logging::log_verbose_server("🔊オーディオプレーヤーを停止中...");
            player.stop();
            logging::log_verbose_server("✅オーディオプレーヤー停止完了");
        } else {
            logging::log_verbose_server("⚠️ 停止するオーディオプレーヤーがありません");
        }

        let mut state = self.state.lock().unwrap();
        *state = ServerState::Stopped;
        logging::log_verbose_server(&format!("📊サーバー状態を更新: {:?}", *state));

        logging::log_verbose_server("✅ インタラクティブモードを停止しました");
        Response::Ok
    }

    fn handle_play_json_in_interactive(
        &self,
        data: serde_json::Value,
        audio_player: &Option<AudioPlayer>,
    ) -> Response {
        // Early return: Check if in interactive mode
        let state = self.state.lock().unwrap();
        if *state != ServerState::Interactive {
            logging::log_always_server(&format!(
                "⚠️  インタラクティブモードではありません。現在の状態: {:?}",
                *state
            ));
            return Response::Error {
                message: format!("Not in interactive mode (current state: {:?})", *state),
            };
        }
        drop(state);

        // Early return: Serialize JSON
        let json_str = match serde_json::to_string(&data) {
            Ok(s) => s,
            Err(e) => {
                logging::log_always_server(&format!("❌ JSONシリアライズに失敗しました: {}", e));
                return Response::Error {
                    message: format!("Failed to serialize JSON: {}", e),
                };
            }
        };

        logging::log_verbose_server("🎵 インタラクティブモードでJSONを処理中...");

        // Early return: Parse event log
        let event_log = match EventLog::from_json_str(&json_str) {
            Ok(log) => log,
            Err(e) => {
                logging::log_always_server(&format!("❌ JSONの解析に失敗しました: {}", e));
                return Response::Error {
                    message: format!("Failed to parse JSON: {}", e),
                };
            }
        };

        // Early return: Validate event log
        if !event_log.validate() {
            logging::log_always_server("❌ 無効なイベントログです");
            return Response::Error {
                message: "Invalid event log: validation failed".to_string(),
            };
        }

        // Early return: Check audio player exists
        let player_ref = match audio_player {
            Some(ref p) => p,
            None => {
                logging::log_always_server("⚠️  音声プレーヤーがありません");
                return Response::Error {
                    message: "No audio player found".to_string(),
                };
            }
        };

        // Process and schedule events
        self.schedule_events_for_interactive(&event_log, player_ref)
    }

    /// Schedule events for interactive playback
    fn schedule_events_for_interactive(
        &self,
        event_log: &EventLog,
        player_ref: &AudioPlayer,
    ) -> Response {
        // Get audio stream elapsed time (not wall-clock time)
        // This prevents timing jitter caused by variable IPC/parsing latency
        let audio_stream_elapsed_sec = match player_ref.get_audio_elapsed_sec() {
            Some(elapsed) => elapsed,
            None => {
                logging::log_always_server("⚠️  音声経過時間の取得に失敗しました");
                return Response::Error {
                    message: "Failed to get audio elapsed time".to_string(),
                };
            }
        };

        // Determine scheduling mode based on first event time
        // If first event is at time 0.0, use ASAP mode (like Web Audio API)
        // Otherwise, use future-scheduled mode with safety buffer
        let is_asap_mode = event_log
            .events
            .first()
            .map(|e| e.time == 0.0)
            .unwrap_or(false);

        let future_offset_sec = if is_asap_mode {
            // ASAP mode: no future offset, play as soon as possible
            0.0
        } else {
            // Future-scheduled mode: use safety buffer to prevent dropouts
            crate::audio_config::timing::FUTURE_SCHEDULING_OFFSET_SEC
        };

        if is_asap_mode {
            logging::log_verbose_server("⚡ ASAPモード: 最速で再生開始");
        } else {
            logging::log_verbose_server(&format!(
                "⏰ 未来スケジュールモード: {}秒後に再生開始",
                future_offset_sec
            ));
        }

        // Clear schedule from first event time if events exist
        if let Some(first_event) = event_log.events.first() {
            let first_scheduled_samples = crate::scheduler::sec_to_samples(
                audio_stream_elapsed_sec + future_offset_sec + first_event.time,
            );

            player_ref.clear_schedule_from(first_scheduled_samples);
            logging::log_verbose_server(&format!(
                "🗑️  サンプル時刻 {} 以降のスケジュール済みイベントをクリアしました",
                first_scheduled_samples
            ));
        }

        logging::log_verbose_server(&format!(
            "📝 {}個のイベントをスケジュール中...",
            event_log.events.len()
        ));

        // Schedule all events using audio stream time
        let success_count = Self::schedule_all_events(
            event_log,
            player_ref,
            audio_stream_elapsed_sec,
            future_offset_sec,
        );

        logging::log_verbose_server(&format!(
            "✅ {}個のイベントを正常にスケジュールしました",
            success_count
        ));
        Response::Ok
    }

    /// Schedule all events with audio stream time and future offset
    fn schedule_all_events(
        event_log: &EventLog,
        player_ref: &AudioPlayer,
        audio_stream_elapsed_sec: f64,
        future_offset_sec: f64,
    ) -> usize {
        for event in &event_log.events {
            // Use audio stream time instead of wall-clock time to prevent jitter
            if let Err(e) = player_ref.schedule_register_write_fixed_time_with_future_offset(
                audio_stream_elapsed_sec,
                future_offset_sec,
                event.time,
                event.addr,
                event.data,
            ) {
                logging::log_always_server(&format!(
                    "⚠️  イベントのスケジュールに失敗: addr=0x{:02X}, data=0x{:02X}, error={}",
                    event.addr, event.data, e
                ));
            }
        }

        event_log.events.len()
    }
}
