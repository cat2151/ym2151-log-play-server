use crate::ipc::protocol::{Command, Response};
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
enum ServerState {
    Playing,
    Stopped,
}

pub struct Server {
    state: Arc<Mutex<ServerState>>,
    shutdown_flag: Arc<AtomicBool>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            state: Arc::new(Mutex::new(ServerState::Stopped)),
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn run(&self) -> Result<()> {
        eprintln!("🚀 YM2151サーバーを起動中...");
        eprintln!(
            "   名前付きパイプ: {}",
            crate::ipc::pipe_windows::DEFAULT_PIPE_PATH
        );

        let mut audio_player: Option<AudioPlayer> = None;

        {
            let mut state = self.state.lock().unwrap();
            *state = ServerState::Stopped;
        }

        eprintln!("🎵 サーバーが起動しました。クライアントからの接続を待機中...");

        loop {
            if self.shutdown_flag.load(Ordering::Relaxed) {
                break;
            }

            // 各接続ごとに新しいパイプを作成
            let connection_pipe = match NamedPipe::create() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("⚠️  警告: 接続用の新しいパイプの作成に失敗しました: {}", e);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
            };

            eprintln!("💬 クライアント接続を待機中...");

            let mut reader = match connection_pipe.open_read() {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("⚠️  警告: パイプの読み取りオープンに失敗しました: {}", e);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }
            };

            eprintln!("📞 クライアントが接続されました");

            // レスポンス送信用のライターも取得
            let mut writer = match connection_pipe.open_write() {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("⚠️  警告: パイプの書き込みオープンに失敗しました: {}", e);
                    continue;
                }
            };

            // 一つのクライアント接続からの複数メッセージを処理
            loop {
                // Read binary command from client
                let binary_data = match reader.read_binary() {
                    Ok(data) => data,
                    Err(e) => {
                        eprintln!("📞 クライアントが切断されました: {}", e);
                        break; // 内側のループを抜けて新しい接続を待機
                    }
                };

                let command = match Command::from_binary(&binary_data) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        eprintln!("⚠️  警告: コマンドの解析に失敗しました: {}", e);
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
                                    eprintln!("📩 コマンドを受信しました: PlayJson (末尾要素: time:{}, addr:0x{:02X}, data:0x{:02X})",
                                             last_event.time, last_event.addr, last_event.data);
                                }
                                Ok(_) => {
                                    eprintln!("📩 コマンドを受信しました: PlayJson (空のイベント配列)");
                                }
                                Err(_) => {
                                    eprintln!("📩 コマンドを受信しました: PlayJson (解析エラー)");
                                }
                            }
                        } else {
                            eprintln!("📩 コマンドを受信しました: PlayJson");
                        }
                    }
                    Command::PlayFile { path } => {
                        eprintln!("📩 コマンドを受信しました: PlayFile({})", path);
                    }
                    other => {
                        eprintln!("📩 コマンドを受信しました: {:?}", other);
                    }
                }

                let response = match command {
                    Command::PlayJson { data } => {
                        eprintln!("🎵 JSON データを読み込み中...");

                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        // Convert JSON value to string for parsing
                        let json_result = serde_json::to_string(&data);

                        match json_result {
                            Ok(json_str) => {
                                match Self::load_and_start_playback(&json_str, true) {
                                    Ok(player) => {
                                        audio_player = Some(player);
                                        eprintln!("✅ JSON データから音声再生を開始しました");

                                        let mut state = self.state.lock().unwrap();
                                        *state = ServerState::Playing;

                                        Response::Ok
                                    }
                                    Err(e) => {
                                        eprintln!("❌ 音声再生の開始に失敗しました: {}", e);
                                        Response::Error {
                                            message: format!("Failed to start playback: {}", e),
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ JSONシリアライズに失敗しました: {}", e);
                                Response::Error {
                                    message: format!("Failed to serialize JSON: {}", e),
                                }
                            }
                        }
                    }
                    Command::PlayFile { path } => {
                        eprintln!("🎵 新しい音声ファイルを読み込み中: {}", path);

                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        match Self::load_and_start_playback(&path, false) {
                            Ok(player) => {
                                audio_player = Some(player);
                                eprintln!("✅ 音声再生を開始しました: {}", path);

                                let mut state = self.state.lock().unwrap();
                                *state = ServerState::Playing;

                                Response::Ok
                            }
                            Err(e) => {
                                eprintln!("❌ 音声再生の開始に失敗しました: {}", e);
                                Response::Error {
                                    message: format!("Failed to start playback: {}", e),
                                }
                            }
                        }
                    }
                    Command::Stop => {
                        eprintln!("⏹️  音声再生を停止中...");
                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        let mut state = self.state.lock().unwrap();
                        *state = ServerState::Stopped;

                        eprintln!("✅ 音声再生を停止しました");
                        Response::Ok
                    }
                    Command::Shutdown => {
                        eprintln!("🛑 シャットダウン要求を受信しました");
                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }
                        self.shutdown_flag.store(true, Ordering::Relaxed);

                        // シャットダウンレスポンスを送信
                        if let Ok(response_binary) = Response::Ok.to_binary() {
                            let _ = writer.write_binary(&response_binary);
                        }
                        eprintln!("✅ シャットダウン完了");
                        return Ok(()); // 外側のループも抜けて終了
                    }
                };

                // レスポンスを送信
                if let Ok(response_binary) = response.to_binary() {
                    if let Err(e) = writer.write_binary(&response_binary) {
                        eprintln!("⚠️  警告: レスポンス送信に失敗しました: {}", e);
                        break; // 書き込みに失敗したら接続を閉じる
                    }
                } else {
                    eprintln!("⚠️  警告: レスポンスのシリアライズに失敗しました");
                    break;
                }

                eprintln!("📤 レスポンスを送信しました: {:?}", response);
            }

            eprintln!("🔄 次の接続を待機中...");
        }

        eprintln!("👋 サーバーのシャットダウンが完了しました");
        Ok(())
    }

    #[cfg(test)]
    fn get_state(&self) -> ServerState {
        self.state.lock().unwrap().clone()
    }

    #[cfg(test)]
    fn is_shutdown_requested(&self) -> bool {
        self.shutdown_flag.load(Ordering::Relaxed)
    }

    fn load_and_start_playback(data: &str, is_json_string: bool) -> Result<AudioPlayer> {
        let log = if is_json_string {
            // Parse as JSON string directly
            EventLog::from_json_str(data)
                .with_context(|| "Failed to parse JSON string data")?
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

        let player = Player::new(log);
        AudioPlayer::new(player).context("Failed to create audio player")
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = Server::new();
        assert_eq!(server.get_state(), ServerState::Stopped);
        assert!(!server.is_shutdown_requested());
    }

    #[test]
    fn test_server_default() {
        let server = Server::default();
        assert_eq!(server.get_state(), ServerState::Stopped);
    }
}
