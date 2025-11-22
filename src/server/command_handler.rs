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
        match command {
            Command::PlayJson { data } => self.handle_play_json(data, audio_player),
            Command::Stop => self.handle_stop(audio_player),
            Command::StartInteractive => self.handle_start_interactive(audio_player),
            Command::GetServerTime => self.handle_get_server_time(),
            Command::StopInteractive => self.handle_stop_interactive(audio_player),
            Command::ClearSchedule => self.handle_clear_schedule(audio_player),
            Command::PlayJsonInInteractive { data } => {
                self.handle_play_json_in_interactive(data, audio_player)
            }
            Command::GetInteractiveModeState => {
                self.handle_get_interactive_mode_state()
            }
            Command::Shutdown => {
                // Shutdown is handled specially in the connection loop
                // This should not be reached
                Response::Ok
            }
        }
    }

    /// Returns whether the server is currently in interactive mode
    fn handle_get_interactive_mode_state(&self) -> Response {
        let state = self.state.lock().unwrap();
        let is_interactive = *state == ServerState::Interactive;
        Response::InteractiveModeState { is_interactive }
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
        logging::log_verbose("🎵 JSON データを読み込み中...");

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
                    logging::log_verbose("✅ JSON データから音声再生を開始しました");

                    let mut state = self.state.lock().unwrap();
                    *state = ServerState::Playing;

                    Response::Ok
                }
                Err(e) => {
                    logging::log_always(&format!("❌ 音声再生の開始に失敗しました: {}", e));
                    Response::Error {
                        message: format!("Failed to start playback: {}", e),
                    }
                }
            },
            Err(e) => {
                logging::log_always(&format!("❌ JSONシリアライズに失敗しました: {}", e));
                Response::Error {
                    message: format!("Failed to serialize JSON: {}", e),
                }
            }
        }
    }

    fn handle_stop(&self, audio_player: &mut Option<AudioPlayer>) -> Response {
        logging::log_verbose("⏹️  音声再生を停止中...");
        if let Some(mut player) = audio_player.take() {
            player.stop();
        }

        let mut state = self.state.lock().unwrap();
        *state = ServerState::Stopped;

        logging::log_verbose("✅ 音声再生を停止しました");
        Response::Ok
    }

    fn handle_start_interactive(&self, audio_player: &mut Option<AudioPlayer>) -> Response {
        logging::log_verbose("🎮 インタラクティブモードを開始中...");
        logging::log_verbose(&format!(
            "🔍現在のサーバー状態: {:?}",
            *self.state.lock().unwrap()
        ));

        // Stop any existing playback
        if let Some(mut player) = audio_player.take() {
            logging::log_verbose("⏹️ 既存の再生を停止中...");
            player.stop();
        }

        // Reset time tracker for new interactive session
        {
            let mut tracker = self.time_tracker.lock().unwrap();
            tracker.reset();
            logging::log_verbose("🕐タイムトラッカーをリセットしました");
        }

        // Start interactive mode
        logging::log_verbose("🎵インタラクティブオーディオプレーヤーを作成中...");
        match self.playback_manager.start_interactive_mode() {
            Ok(player) => {
                *audio_player = Some(player);
                logging::log_verbose("✅ インタラクティブモードを開始しました");
                logging::log_verbose("🔊音声ストリーミング開始");

                let mut state = self.state.lock().unwrap();
                *state = ServerState::Interactive;
                logging::log_verbose(&format!("📊サーバー状態を更新: {:?}", *state));

                Response::Ok
            }
            Err(e) => {
                logging::log_always(&format!(
                    "❌ インタラクティブモードの開始に失敗しました: {}",
                    e
                ));
                logging::log_always("💡 [デバッグ情報] 以下を確認してください:");
                logging::log_always("   1. 音声デバイスが利用可能か");
                logging::log_always("   2. 他のアプリケーションが音声デバイスを使用していないか");
                logging::log_always("   3. システムの音量設定");
                Response::Error {
                    message: format!("Failed to start interactive mode: {}", e),
                }
            }
        }
    }

    fn handle_get_server_time(&self) -> Response {
        let tracker = self.time_tracker.lock().unwrap();
        let time_sec = tracker.elapsed_sec();
        logging::log_verbose(&format!("⏰ サーバー時刻を取得: {:.6} 秒", time_sec));
        Response::ServerTime { time_sec }
    }

    fn handle_stop_interactive(&self, audio_player: &mut Option<AudioPlayer>) -> Response {
        logging::log_verbose("⏹️  インタラクティブモードを停止中...");
        logging::log_verbose(&format!(
            "🔍現在のサーバー状態: {:?}",
            *self.state.lock().unwrap()
        ));

        if let Some(mut player) = audio_player.take() {
            logging::log_verbose("🔊オーディオプレーヤーを停止中...");
            player.stop();
            logging::log_verbose("✅オーディオプレーヤー停止完了");
        } else {
            logging::log_verbose("⚠️ 停止するオーディオプレーヤーがありません");
        }

        let mut state = self.state.lock().unwrap();
        *state = ServerState::Stopped;
        logging::log_verbose(&format!("📊サーバー状態を更新: {:?}", *state));

        logging::log_verbose("✅ インタラクティブモードを停止しました");
        Response::Ok
    }

    fn handle_clear_schedule(&self, audio_player: &Option<AudioPlayer>) -> Response {
        let state = self.state.lock().unwrap();
        if *state != ServerState::Interactive {
            Response::Error {
                message: "Not in interactive mode".to_string(),
            }
        } else {
            drop(state); // Release lock before clearing

            if let Some(ref player_ref) = audio_player {
                player_ref.clear_schedule();
                logging::log_verbose("🗑️  スケジュール済みイベントをクリアしました");
                Response::Ok
            } else {
                Response::Error {
                    message: "No active audio player".to_string(),
                }
            }
        }
    }

    fn handle_play_json_in_interactive(
        &self,
        data: serde_json::Value,
        audio_player: &Option<AudioPlayer>,
    ) -> Response {
        let state = self.state.lock().unwrap();
        if *state != ServerState::Interactive {
            logging::log_always(&format!(
                "⚠️  インタラクティブモードではありません。現在の状態: {:?}",
                *state
            ));
            Response::Error {
                message: "Not in interactive mode".to_string(),
            }
        } else {
            drop(state);

            // Convert JSON value to string for parsing
            let json_result = serde_json::to_string(&data);

            match json_result {
                Ok(json_str) => {
                    logging::log_verbose("🎵 インタラクティブモードでJSONを処理中...");

                    // Parse the JSON event log (time in seconds)
                    match EventLog::from_json_str(&json_str) {
                        Ok(event_log) => {
                            if !event_log.validate() {
                                logging::log_always("❌ 無効なイベントログです");
                                Response::Error {
                                    message: "Invalid event log: validation failed".to_string(),
                                }
                            } else if let Some(ref player_ref) = audio_player {
                                // Get current server time
                                let current_time_sec = {
                                    let tracker = self.time_tracker.lock().unwrap();
                                    tracker.elapsed_sec()
                                };

                                logging::log_verbose(&format!(
                                    "📝 {}個のイベントをスケジュール中...",
                                    event_log.events.len()
                                ));

                                let mut success_count = 0;

                                // Schedule all events (time is already in seconds)
                                for event in &event_log.events {
                                    // Time is already in seconds, just add current time offset
                                    let scheduled_samples = crate::scheduler::sec_to_samples(
                                        current_time_sec + event.time,
                                    );

                                    player_ref.schedule_register_write(
                                        scheduled_samples,
                                        event.addr,
                                        event.data,
                                    );
                                    success_count += 1;
                                }

                                logging::log_verbose(&format!(
                                    "✅ {}個のイベントを正常にスケジュールしました",
                                    success_count
                                ));
                                Response::Ok
                            } else {
                                logging::log_always("⚠️  音声プレーヤーがありません");
                                Response::Error {
                                    message: "No audio player found".to_string(),
                                }
                            }
                        }
                        Err(e) => {
                            logging::log_always(&format!("❌ JSONの解析に失敗しました: {}", e));
                            Response::Error {
                                message: format!("Failed to parse JSON: {}", e),
                            }
                        }
                    }
                }
                Err(e) => {
                    logging::log_always(&format!("❌ JSONシリアライズに失敗しました: {}", e));
                    Response::Error {
                        message: format!("Failed to serialize JSON: {}", e),
                    }
                }
            }
        }
    }
}
