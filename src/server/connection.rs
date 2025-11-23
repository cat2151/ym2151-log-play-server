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
        logging::log_always_server("🚀 YM2151サーバーを起動中...");
        logging::log_always_server(&format!(
            "   名前付きパイプ: {}",
            crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
        ));
        logging::log_always_server("   モード: アトミック（1接続=1コマンド）");

        let mut audio_player: Option<AudioPlayer> = None;
        logging::log_always_server("🎵 サーバーが起動しました");

        loop {
            if self.command_handler.is_shutdown_requested() {
                break;
            }
            if self.handle_connection_once(&mut audio_player)? {
                // シャットダウン要求で終了
                break;
            }
        }

        logging::log_always_server("👋 サーバーのシャットダウンが完了しました");
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn handle_connection_once(&self, audio_player: &mut Option<AudioPlayer>) -> Result<bool> {
        // シングルスレッド用、複数クライアントからの接続も可能、シンプル優先、アトミック接続。この関数内で、1回の接続用のcreateからcloseまでのライフサイクルを完結。なおWindows名前付きパイプは1回のcreate～closeにつき、単一クライアントからの接続しか受け付けられないため、このような実装になる。
        logging::log_verbose_server("💬 パイプを作成します...");

        let connection_pipe = match NamedPipe::create() {
            Ok(p) => p,
            Err(e) => {
                logging::log_always_server(&format!(
                    "⚠️  警告: 接続用の新しいパイプの作成に失敗しました: {}",
                    e
                ));
                std::thread::sleep(std::time::Duration::from_millis(100));
                return Ok(false);
            }
        };

        logging::log_verbose_server("💬 パイプを作成しました。クライアント接続を待機中...");

        // blocking。このopen_readは、呼び出すと、クライアントが接続してくるまではreturnしない。つまりここで1秒～数分の待ち時間もありうる。
        let mut reader = match connection_pipe.open_read() {
            Ok(r) => r,
            Err(e) => {
                logging::log_verbose_server(&format!(
                    "⚠️  警告: パイプの読み取りオープンに失敗しました: {}",
                    e
                ));
                std::thread::sleep(std::time::Duration::from_millis(100));
                return Ok(false);
            }
        };

        logging::log_verbose_server("📞 クライアントが接続されました");

        // レスポンス送信用の準備をあらかじめ行う
        let mut writer = match connection_pipe.open_write() {
            Ok(w) => w,
            Err(e) => {
                logging::log_verbose_server(&format!(
                    "⚠️  警告: パイプの書き込みオープンに失敗しました: {}",
                    e
                ));
                return Ok(false);
            }
        };

        let binary_data = match reader.read_binary() {
            Ok(data) => data,
            Err(e) => {
                logging::log_verbose_server(&format!("📞 コマンド読み取りエラー: {}", e));
                return Ok(false); // 次の接続を待機
            }
        };

        let command = match Command::from_binary(&binary_data) {
            Ok(cmd) => cmd,
            Err(e) => {
                logging::log_always_server(&format!(
                    "⚠️  警告: コマンドの解析に失敗しました: {}",
                    e
                ));
                let response = Response::Error {
                    message: format!("Parse error: {}", e),
                };
                if let Ok(response_binary) = response.to_binary() {
                    let _ = writer.write_binary(&response_binary);
                }
                return Ok(false); // 次の接続を待機
            }
        };

        self.log_command(&command);

        let response = if matches!(command, Command::Shutdown) {
            // シャットダウン要求の処理
            logging::log_always_server("🛑 シャットダウン要求を受信しました");
            if let Some(mut player) = audio_player.take() {
                player.stop();
            }
            self.command_handler.request_shutdown();

            // シャットダウンレスポンスを送信
            if let Ok(response_binary) = Response::Ok.to_binary() {
                let _ = writer.write_binary(&response_binary);
            }
            logging::log_always_server("✅ シャットダウン完了");
            return Ok(true); // ループを抜けて終了
        } else {
            // 通常のコマンド処理
            self.command_handler.handle_command(command, audio_player)
        };

        // レスポンスを送信
        if let Ok(response_binary) = response.to_binary() {
            if let Err(e) = writer.write_binary(&response_binary) {
                logging::log_verbose_server(&format!(
                    "⚠️  警告: レスポンス送信に失敗しました: {}",
                    e
                ));
            }
        } else {
            logging::log_verbose_server("⚠️  警告: レスポンスのシリアライズに失敗しました");
        }
        // 接続が自動的にクローズされる（writerがスコープ外になったので）

        logging::log_verbose_server(&format!("📤 レスポンスを送信しました: {:?}", response));
        logging::log_verbose_server("🔄 次の接続待機に進みます...");
        Ok(false)
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
                            logging::log_verbose_server(&format!(
                                "📩 コマンドを受信しました: PlayJson (末尾要素: time:{}, addr:0x{:02X}, data:0x{:02X})",
                                last_event.time, last_event.addr, last_event.data
                            ));
                        }
                        Ok(_) => {
                            logging::log_verbose_server(
                                "📩 コマンドを受信しました: PlayJson (空のイベント配列)",
                            );
                        }
                        Err(_) => {
                            logging::log_verbose_server(
                                "📩 コマンドを受信しました: PlayJson (解析エラー)",
                            );
                        }
                    }
                } else {
                    logging::log_verbose_server("📩 コマンドを受信しました: PlayJson");
                }
            }
            other => {
                logging::log_verbose_server(&format!("📩 コマンドを受信しました: {:?}", other));
            }
        }
    }
}
