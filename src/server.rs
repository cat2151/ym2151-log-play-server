use crate::ipc::protocol::{Command, Response};
use crate::logging;
use crate::resampler::ResamplingQuality;
use crate::scheduler::TimeTracker;
use anyhow::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use anyhow::Context;
use std::sync::atomic::Ordering;

use crate::events::EventLog;
use crate::player::Player;

use crate::audio::AudioPlayer;
use crate::ipc::pipe_windows::NamedPipe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerState {
    Playing,
    Stopped,
    Interactive,
}

pub struct Server {
    state: Arc<Mutex<ServerState>>,
    shutdown_flag: Arc<AtomicBool>,
    resampling_quality: ResamplingQuality,
    time_tracker: Arc<Mutex<TimeTracker>>,
}

impl Server {
    pub fn new() -> Self {
        Self::new_with_resampling_quality(false)
    }

    pub fn new_with_resampling_quality(low_quality: bool) -> Self {
        let quality = if low_quality {
            ResamplingQuality::Linear
        } else {
            ResamplingQuality::HighQuality
        };

        logging::log_always(&format!(
            "🎵 リサンプリング品質: {}",
            match quality {
                ResamplingQuality::Linear => "低品質 (線形補間)",
                ResamplingQuality::HighQuality => "標準 (Rubato FFTベース)",
            }
        ));

        Server {
            state: Arc::new(Mutex::new(ServerState::Stopped)),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
            resampling_quality: quality,
            time_tracker: Arc::new(Mutex::new(TimeTracker::new())),
        }
    }

    pub fn run(&self) -> Result<()> {
        logging::log_always("🚀 YM2151サーバーを起動中...");
        logging::log_always(&format!(
            "   名前付きパイプ: {}",
            crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
        ));

        let mut audio_player: Option<AudioPlayer> = None;

        {
            let mut state = self.state.lock().unwrap();
            *state = ServerState::Stopped;
        }

        logging::log_always("🎵 サーバーが起動しました。クライアントからの接続を待機中...");

        loop {
            if self.shutdown_flag.load(Ordering::Relaxed) {
                break;
            }

            // 各接続ごとに新しいパイプを作成
            let connection_pipe = match NamedPipe::create() {
                Ok(p) => p,
                Err(e) => {
                    logging::log_always(&format!(
                        "⚠️  警告: 接続用の新しいパイプの作成に失敗しました: {}",
                        e
                    ));
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
            };

            logging::log_verbose("💬 クライアント接続を待機中...");

            let mut reader = match connection_pipe.open_read() {
                Ok(r) => r,
                Err(e) => {
                    logging::log_always(&format!(
                        "⚠️  警告: パイプの読み取りオープンに失敗しました: {}",
                        e
                    ));
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
            };

            logging::log_verbose("📞 クライアントが接続されました");

            // レスポンス送信用のライターも取得
            let mut writer = match connection_pipe.open_write() {
                Ok(w) => w,
                Err(e) => {
                    logging::log_always(&format!(
                        "⚠️  警告: パイプの書き込みオープンに失敗しました: {}",
                        e
                    ));
                    continue;
                }
            };

            // 一つのクライアント接続からの複数メッセージを処理
            loop {
                // Read binary command from client
                let binary_data = match reader.read_binary() {
                    Ok(data) => data,
                    Err(e) => {
                        logging::log_verbose(&format!("📞 クライアントが切断されました: {}", e));
                        break; // 内側のループを抜けて新しい接続を待機
                    }
                };

                let command = match Command::from_binary(&binary_data) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        logging::log_always(&format!(
                            "⚠️  警告: コマンドの解析に失敗しました: {}",
                            e
                        ));
                        let response = Response::Error {
                            message: format!("Parse error: {}", e),
                        };
                        if let Ok(response_binary) = response.to_binary() {
                            let _ = writer.write_binary(&response_binary);
                        }
                        continue;
                    }
                };

                // コマンドの内容をログ出力
                match &command {
                    Command::PlayJson { data } => {
                        // JSON データの場合、末尾要素だけを表示
                        if let Ok(log_str) = serde_json::to_string(data) {
                            match EventLog::from_json_str(&log_str) {
                                Ok(log) if !log.events.is_empty() => {
                                    let last_event = &log.events[log.events.len() - 1];
                                    logging::log_verbose(&format!(
                                        "📩 コマンドを受信しました: PlayJson (末尾要素: time:{}, addr:0x{:02X}, data:0x{:02X})",
                                        last_event.time, last_event.addr, last_event.data
                                    ));
                                }
                                Ok(_) => {
                                    logging::log_verbose(
                                        "📩 コマンドを受信しました: PlayJson (空のイベント配列)",
                                    );
                                }
                                Err(_) => {
                                    logging::log_verbose(
                                        "📩 コマンドを受信しました: PlayJson (解析エラー)",
                                    );
                                }
                            }
                        } else {
                            logging::log_verbose("📩 コマンドを受信しました: PlayJson");
                        }
                    }
                    other => {
                        logging::log_verbose(&format!("📩 コマンドを受信しました: {:?}", other));
                    }
                }

                let response = match command {
                    Command::PlayJson { data } => {
                        logging::log_verbose("🎵 JSON データを読み込み中...");

                        // Stop any existing playback
                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        // Convert JSON value to string for parsing
                        let json_result = serde_json::to_string(&data);

                        match json_result {
                            Ok(json_str) => match self.load_and_start_playback(&json_str, true) {
                                Ok(player) => {
                                    audio_player = Some(player);
                                    logging::log_verbose(
                                        "✅ JSON データから音声再生を開始しました",
                                    );

                                    let mut state = self.state.lock().unwrap();
                                    *state = ServerState::Playing;

                                    Response::Ok
                                }
                                Err(e) => {
                                    logging::log_always(&format!(
                                        "❌ 音声再生の開始に失敗しました: {}",
                                        e
                                    ));
                                    Response::Error {
                                        message: format!("Failed to start playback: {}", e),
                                    }
                                }
                            },
                            Err(e) => {
                                logging::log_always(&format!(
                                    "❌ JSONシリアライズに失敗しました: {}",
                                    e
                                ));
                                Response::Error {
                                    message: format!("Failed to serialize JSON: {}", e),
                                }
                            }
                        }
                    }
                    Command::Stop => {
                        logging::log_verbose("⏹️  音声再生を停止中...");
                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        let mut state = self.state.lock().unwrap();
                        *state = ServerState::Stopped;

                        logging::log_verbose("✅ 音声再生を停止しました");
                        Response::Ok
                    }
                    Command::StartInteractive => {
                        logging::log_verbose("🎮 インタラクティブモードを開始中...");
                        logging::log_verbose(&format!(
                            "🔍 [デバッグ] 現在のサーバー状態: {:?}",
                            *self.state.lock().unwrap()
                        ));

                        // Stop any existing playback
                        if let Some(mut player) = audio_player.take() {
                            logging::log_verbose("⏹️  [デバッグ] 既存の再生を停止中...");
                            player.stop();
                        }

                        // Reset time tracker for new interactive session
                        {
                            let mut tracker = self.time_tracker.lock().unwrap();
                            tracker.reset();
                            logging::log_verbose(
                                "🕐 [デバッグ] タイムトラッカーをリセットしました",
                            );
                        }

                        // Start interactive mode
                        logging::log_verbose(
                            "🎵 [デバッグ] インタラクティブオーディオプレーヤーを作成中...",
                        );
                        match self.start_interactive_mode() {
                            Ok(player) => {
                                audio_player = Some(player);
                                logging::log_verbose("✅ インタラクティブモードを開始しました");
                                logging::log_verbose("🔊 [デバッグ] 音声ストリーミング開始");

                                let mut state = self.state.lock().unwrap();
                                *state = ServerState::Interactive;
                                logging::log_verbose(&format!(
                                    "📊 [デバッグ] サーバー状態を更新: {:?}",
                                    *state
                                ));

                                Response::Ok
                            }
                            Err(e) => {
                                logging::log_always(&format!(
                                    "❌ インタラクティブモードの開始に失敗しました: {}",
                                    e
                                ));
                                logging::log_always("💡 [デバッグ情報] 以下を確認してください:");
                                logging::log_always("   1. 音声デバイスが利用可能か");
                                logging::log_always(
                                    "   2. 他のアプリケーションが音声デバイスを使用していないか",
                                );
                                logging::log_always("   3. システムの音量設定");
                                Response::Error {
                                    message: format!("Failed to start interactive mode: {}", e),
                                }
                            }
                        }
                    }
                    Command::WriteRegister {
                        time_offset_sec,
                        addr,
                        data,
                    } => {
                        let state = self.state.lock().unwrap();
                        logging::log_verbose(&format!(
                            "📝 [デバッグ] WriteRegisterコマンド受信: state={:?}",
                            *state
                        ));
                        if *state != ServerState::Interactive {
                            logging::log_always(&format!(
                                "⚠️  インタラクティブモードではありません。現在の状態: {:?}",
                                *state
                            ));
                            Response::Error {
                                message: "Not in interactive mode".to_string(),
                            }
                        } else {
                            drop(state); // Release lock before potentially slow operation

                            if let Some(ref player_ref) = audio_player {
                                // Get current server time
                                let current_time_sec = {
                                    let tracker = self.time_tracker.lock().unwrap();
                                    tracker.elapsed_sec()
                                };

                                // Convert time offset to scheduled sample time
                                let scheduled_samples = crate::scheduler::sec_to_samples(
                                    current_time_sec + time_offset_sec,
                                );

                                logging::log_verbose(&format!(
                                    "⏰ [デバッグ] 時刻計算: current={:.6}s, offset={:.6}s, scheduled={:.6}s ({}サンプル)",
                                    current_time_sec,
                                    time_offset_sec,
                                    current_time_sec + time_offset_sec,
                                    scheduled_samples
                                ));

                                // Schedule the register write
                                player_ref.schedule_register_write(scheduled_samples, addr, data);

                                logging::log_verbose(&format!(
                                    "📝 レジスタ書き込みをスケジュール: server_time={:.6}秒, offset={:.6}秒, scheduled_time={:.6}秒, addr:0x{:02X}, data:0x{:02X}",
                                    current_time_sec,
                                    time_offset_sec,
                                    current_time_sec + time_offset_sec,
                                    addr,
                                    data
                                ));
                                Response::Ok
                            } else {
                                logging::log_always("❌ [デバッグ] audio_playerが存在しません");
                                Response::Error {
                                    message: "No active audio player".to_string(),
                                }
                            }
                        }
                    }
                    Command::GetServerTime => {
                        let tracker = self.time_tracker.lock().unwrap();
                        let time_sec = tracker.elapsed_sec();
                        logging::log_verbose(&format!("⏰ サーバー時刻を取得: {:.6} 秒", time_sec));
                        Response::ServerTime { time_sec }
                    }
                    Command::StopInteractive => {
                        logging::log_verbose("⏹️  インタラクティブモードを停止中...");
                        logging::log_verbose(&format!(
                            "🔍 [デバッグ] 現在のサーバー状態: {:?}",
                            *self.state.lock().unwrap()
                        ));

                        if let Some(mut player) = audio_player.take() {
                            logging::log_verbose("🔊 [デバッグ] オーディオプレーヤーを停止中...");
                            player.stop();
                            logging::log_verbose("✅ [デバッグ] オーディオプレーヤー停止完了");
                        } else {
                            logging::log_verbose(
                                "⚠️  [デバッグ] 停止するオーディオプレーヤーがありません",
                            );
                        }

                        let mut state = self.state.lock().unwrap();
                        *state = ServerState::Stopped;
                        logging::log_verbose(&format!(
                            "📊 [デバッグ] サーバー状態を更新: {:?}",
                            *state
                        ));

                        logging::log_verbose("✅ インタラクティブモードを停止しました");
                        Response::Ok
                    }
                    Command::ClearSchedule => {
                        let state = self.state.lock().unwrap();
                        if *state != ServerState::Interactive {
                            Response::Error {
                                message: "Not in interactive mode".to_string(),
                            }
                        } else {
                            drop(state); // Release lock before clearing

                            if let Some(ref player_ref) = audio_player {
                                player_ref.clear_schedule();
                                logging::log_verbose(
                                    "🗑️  スケジュール済みイベントをクリアしました",
                                );
                                Response::Ok
                            } else {
                                Response::Error {
                                    message: "No active audio player".to_string(),
                                }
                            }
                        }
                    }
                    Command::Shutdown => {
                        logging::log_always("🛑 シャットダウン要求を受信しました");
                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }
                        self.shutdown_flag.store(true, Ordering::Relaxed);

                        // シャットダウンレスポンスを送信
                        if let Ok(response_binary) = Response::Ok.to_binary() {
                            let _ = writer.write_binary(&response_binary);
                        }
                        logging::log_always("✅ シャットダウン完了");
                        return Ok(()); // 外側のループも抜けて終了
                    }
                };

                // レスポンスを送信
                if let Ok(response_binary) = response.to_binary() {
                    if let Err(e) = writer.write_binary(&response_binary) {
                        logging::log_always(&format!(
                            "⚠️  警告: レスポンス送信に失敗しました: {}",
                            e
                        ));
                        break; // 書き込みに失敗したら接続を閉じる
                    }
                } else {
                    logging::log_always("⚠️  警告: レスポンスのシリアライズに失敗しました");
                    break;
                }

                logging::log_verbose(&format!("📤 レスポンスを送信しました: {:?}", response));
            }

            logging::log_verbose("🔄 次の接続を待機中...");
        }

        logging::log_always("👋 サーバーのシャットダウンが完了しました");
        Ok(())
    }

    #[cfg(test)]
    pub fn get_state(&self) -> ServerState {
        self.state.lock().unwrap().clone()
    }

    #[cfg(test)]
    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }

    fn load_and_start_playback(&self, data: &str, is_json_string: bool) -> Result<AudioPlayer> {
        let log = if is_json_string {
            // Parse as JSON string directly
            EventLog::from_json_str(data).with_context(|| "Failed to parse JSON string data")?
        } else {
            // Load from file path
            EventLog::from_file(data)
                .with_context(|| format!("Failed to load JSON file: {}", data))?
        };

        if !log.validate() {
            return Err(anyhow::anyhow!(
                "Event log validation failed: event_count doesn't match events array length"
            ));
        }

        let player = Player::new(log.clone());
        // Pass the event log to AudioPlayer if in verbose mode
        let event_log = if logging::is_verbose() {
            Some(log)
        } else {
            None
        };
        AudioPlayer::new_with_quality(player, event_log, self.resampling_quality)
            .context("Failed to create audio player")
    }

    fn start_interactive_mode(&self) -> Result<AudioPlayer> {
        let player = Player::new_interactive();
        // No event log in interactive mode, and no WAV output
        AudioPlayer::new_with_quality(player, None, self.resampling_quality)
            .context("Failed to create interactive audio player")
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}
