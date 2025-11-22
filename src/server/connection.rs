use crate::audio::AudioPlayer;
use crate::ipc::protocol::{Command, Response};
use crate::logging;
use crate::server::command_handler::CommandHandler;
use anyhow::Result;

#[cfg(target_os = "windows")]
use crate::ipc::pipe_windows::NamedPipe;

/// Manages client connections via named pipes
pub struct ConnectionManager {
    command_handler: CommandHandler,
}

impl ConnectionManager {
    pub fn new(command_handler: CommandHandler) -> Self {
        Self { command_handler }
    }

    /// Run the main connection loop in atomic mode
    /// Each connection processes exactly one command and then closes
    #[cfg(target_os = "windows")]
    pub fn run(&self) -> Result<()> {
        logging::log_always("🚀 YM2151サーバーを起動中...");
        logging::log_always(&format!(
            "   名前付きパイプ: {}",
            crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
        ));
        logging::log_always("   モード: アトミック（1接続=1コマンド）");

        let mut audio_player: Option<AudioPlayer> = None;

        logging::log_always("🎵 サーバーが起動しました。クライアントからの接続を待機中...");

        loop {
            if self.command_handler.is_shutdown_requested() {
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
                    logging::log_verbose(&format!(
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
                    logging::log_verbose(&format!(
                        "⚠️  警告: パイプの書き込みオープンに失敗しました: {}",
                        e
                    ));
                    continue;
                }
            };

            // アトミックモード: 1コマンドだけ処理
            // Read binary command from client
            let binary_data = match reader.read_binary() {
                Ok(data) => data,
                Err(e) => {
                    logging::log_verbose(&format!("📞 コマンド読み取りエラー: {}", e));
                    continue; // 次の接続を待機
                }
            };

            let command = match Command::from_binary(&binary_data) {
                Ok(cmd) => cmd,
                Err(e) => {
                    logging::log_always(&format!("⚠️  警告: コマンドの解析に失敗しました: {}", e));
                    let response = Response::Error {
                        message: format!("Parse error: {}", e),
                    };
                    if let Ok(response_binary) = response.to_binary() {
                        let _ = writer.write_binary(&response_binary);
                    }
                    continue; // 次の接続を待機
                }
            };

            // Log command content
            self.log_command(&command);

            // Handle shutdown specially
            let response = if matches!(command, Command::Shutdown) {
                logging::log_always("🛑 シャットダウン要求を受信しました");
                if let Some(mut player) = audio_player.take() {
                    player.stop();
                }
                self.command_handler.request_shutdown();

                // シャットダウンレスポンスを送信
                if let Ok(response_binary) = Response::Ok.to_binary() {
                    let _ = writer.write_binary(&response_binary);
                }
                logging::log_always("✅ シャットダウン完了");
                return Ok(()); // ループを抜けて終了
            } else {
                self.command_handler
                    .handle_command(command, &mut audio_player)
            };

            // レスポンスを送信
            if let Ok(response_binary) = response.to_binary() {
                if let Err(e) = writer.write_binary(&response_binary) {
                    logging::log_verbose(&format!("⚠️  警告: レスポンス送信に失敗しました: {}", e));
                }
            } else {
                logging::log_verbose("⚠️  警告: レスポンスのシリアライズに失敗しました");
            }

            logging::log_verbose(&format!("📤 レスポンスを送信しました: {:?}", response));

            // 接続は自動的にクローズされる（スコープ外）
            logging::log_verbose("🔄 次の接続を待機中...");
        }

        logging::log_always("👋 サーバーのシャットダウンが完了しました");
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn run(&self) -> Result<()> {
        anyhow::bail!("Server is only supported on Windows")
    }

    fn log_command(&self, command: &Command) {
        match command {
            Command::PlayJson { data } => {
                // JSON データの場合、末尾要素だけを表示
                if let Ok(log_str) = serde_json::to_string(data) {
                    match crate::events::EventLog::from_json_str(&log_str) {
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
    }
}
