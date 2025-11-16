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
                let line = match reader.read_line() {
                    Ok(l) => l,
                    Err(e) => {
                        eprintln!("📞 クライアントが切断されました: {}", e);
                        break; // 内側のループを抜けて新しい接続を待機
                    }
                };

                let command = match Command::parse(&line) {
                    Ok(cmd) => cmd,
                    Err(e) => {
                        eprintln!("⚠️  警告: コマンドの解析に失敗しました: {}", e);
                        let _ = writer
                            .write_str(&Response::Error(format!("Parse error: {}", e)).serialize());
                        continue;
                    }
                };

                // コマンドの内容をログ出力（JSON文字列の場合は末尾要素のみ表示）
                match &command {
                    Command::Play(json_data) => {
                        if Command::is_json_string(json_data) {
                            // JSON文字列の場合、末尾要素だけを表示
                            match EventLog::from_json_str(json_data) {
                                Ok(log) if !log.events.is_empty() => {
                                    let last_event = &log.events[log.events.len() - 1];
                                    eprintln!("📩 コマンドを受信しました: PLAY <JSON文字列データ> (末尾要素: time:{}, addr:0x{:02X}, data:0x{:02X})",
                                             last_event.time, last_event.addr, last_event.data);
                                }
                                Ok(_) => {
                                    eprintln!("📩 コマンドを受信しました: PLAY <JSON文字列データ> (空のイベント配列)");
                                }
                                Err(_) => {
                                    eprintln!("📩 コマンドを受信しました: PLAY <JSON文字列データ> (解析エラー)");
                                }
                            }
                        } else {
                            eprintln!("📩 コマンドを受信しました: PLAY {}", json_data);
                        }
                    }
                    other => {
                        eprintln!("📩 コマンドを受信しました: {:?}", other);
                    }
                }

                let response = match command {
                    Command::Play(json_data) => {
                        use crate::ipc::protocol::Command;

                        if Command::is_json_string(&json_data) {
                            eprintln!("🎵 JSON文字列データを読み込み中...");
                        } else {
                            eprintln!("🎵 新しい音声ファイルを読み込み中: {}", json_data);
                        }

                        if let Some(mut player) = audio_player.take() {
                            player.stop();
                        }

                        match Self::load_and_start_playback(&json_data) {
                            Ok(player) => {
                                audio_player = Some(player);

                                if Command::is_json_string(&json_data) {
                                    eprintln!("✅ JSON文字列から音声再生を開始しました");
                                } else {
                                    eprintln!("✅ 音声再生を開始しました: {}", json_data);
                                }

                                let mut state = self.state.lock().unwrap();
                                *state = ServerState::Playing;

                                Response::Ok
                            }
                            Err(e) => {
                                eprintln!("❌ 音声再生の開始に失敗しました: {}", e);
                                Response::Error(format!("Failed to start playback: {}", e))
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
                        let _ = writer.write_str(&Response::Ok.serialize());
                        eprintln!("✅ シャットダウン完了");
                        return Ok(()); // 外側のループも抜けて終了
                    }
                };

                // レスポンスを送信
                if let Err(e) = writer.write_str(&response.serialize()) {
                    eprintln!("⚠️  警告: レスポンス送信に失敗しました: {}", e);
                    break; // 書き込みに失敗したら接続を閉じる
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

    fn load_and_start_playback(json_data: &str) -> Result<AudioPlayer> {
        use crate::ipc::protocol::Command;

        let log = if Command::is_json_string(json_data) {
            // Parse as JSON string directly
            EventLog::from_json_str(json_data)
                .with_context(|| "Failed to parse JSON string data")?
        } else {
            // Load from file path
            EventLog::from_file(json_data)
                .with_context(|| format!("Failed to load JSON file: {}", json_data))?
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
