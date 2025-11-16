use clap::{Parser, Subcommand};
use ym2151_log_play_server::debug_wav;
use ym2151_log_play_server::events::EventLog;
#[cfg(windows)]
use ym2151_log_play_server::logging;
use ym2151_log_play_server::player::Player;
use ym2151_log_play_server::resampler::OPM_SAMPLE_RATE;
use ym2151_log_play_server::wav_writer;

#[cfg(windows)]
use ym2151_log_play_server::client;
#[cfg(windows)]
use ym2151_log_play_server::server::Server;

/// YM2151 Log Player - Rust implementation
#[derive(Parser)]
#[command(name = "ym2151-log-play-server")]
#[command(version)]
#[command(about = "YM2151 Log Player - Rust implementation", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// JSONイベントログファイルを読み込み (スタンドアロン演奏)
    #[arg(value_name = "JSON_FILE")]
    json_file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// サーバーとして起動
    Server {
        /// デバッグ用に詳細なログを出力 (通常時はログファイルのみ)
        #[arg(long)]
        verbose: bool,
    },
    /// サーバーに演奏指示
    Client {
        /// JSONファイルのパス
        #[arg(value_name = "JSON_FILE")]
        json_file: Option<String>,

        /// デバッグ用に詳細な状態メッセージを出力 (デフォルトはサイレント、TUIアプリでは非推奨)
        #[arg(long)]
        verbose: bool,

        /// 演奏を停止
        #[arg(long)]
        stop: bool,

        /// サーバーをシャットダウン
        #[arg(long)]
        shutdown: bool,
    },
}

fn main() {
    // Parse arguments with custom error handling
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // Convert clap error to Japanese error message
            match e.kind() {
                clap::error::ErrorKind::UnknownArgument => {
                    // Extract the unknown argument from the error message
                    let error_msg = e.to_string();
                    if let Some(arg) = error_msg
                        .split('\'')
                        .nth(1)
                        .or_else(|| error_msg.split('\u{2018}').nth(1))
                    {
                        // Check if it looks like a second positional argument (likely too many args)
                        if !arg.starts_with("--") && !arg.starts_with('-') {
                            eprintln!("❌ エラー: 引数が多すぎます");
                            eprintln!();
                            eprintln!("使用方法: ym2151-log-play-server <JSON_FILE>");
                            eprintln!("または --help でヘルプを表示");
                        } else {
                            eprintln!("❌ エラー: 不明なオプション: {}", arg);
                            eprintln!();
                            if arg == "--no-audio" {
                                eprintln!("ヒント: --no-audio オプションは廃止されました。");
                                eprintln!(
                                    "      CI/ヘッドレス環境では、ALSA設定を使用してください。"
                                );
                                eprintln!(
                                    "      詳細は CI_TDD_GUIDE.md または README.md を参照してください。"
                                );
                            } else {
                                eprintln!(
                                    "ヒント: スタンドアロン演奏モードではオプションは不要です。"
                                );
                                eprintln!("      JSONファイルのパスを直接指定してください。");
                            }
                        }
                    } else {
                        eprintln!("{}", e);
                    }
                }
                clap::error::ErrorKind::TooManyValues => {
                    eprintln!("❌ エラー: 引数が多すぎます");
                    eprintln!();
                    eprintln!("使用方法: ym2151-log-play-server <JSON_FILE>");
                    eprintln!("または --help でヘルプを表示");
                }
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    print!("{}", e);
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("{}", e);
                }
            }
            std::process::exit(1);
        }
    };

    match cli.command {
        Some(Commands::Server { verbose }) => {
            #[cfg(windows)]
            {
                // Initialize logging with verbose flag
                logging::init(verbose);

                let server = Server::new();
                match server.run() {
                    Ok(_) => {
                        std::process::exit(0);
                    }
                    Err(e) => {
                        logging::log_always(&format!(
                            "❌ エラー: サーバーの起動に失敗しました: {}",
                            e
                        ));
                        std::process::exit(1);
                    }
                }
            }
            #[cfg(not(windows))]
            {
                let _ = verbose;
                eprintln!("❌ エラー: サーバーモードはWindowsでのみサポートされています");
                std::process::exit(1);
            }
        }
        Some(Commands::Client {
            json_file,
            verbose,
            stop,
            shutdown,
        }) => {
            #[cfg(windows)]
            {
                // Initialize client with verbose flag
                client::init_client(verbose);

                // Handle different client commands
                if stop {
                    match client::stop_playback() {
                        Ok(_) => {
                            std::process::exit(0);
                        }
                        Err(e) => {
                            eprintln!("❌ エラー: 停止要求の送信に失敗しました: {}", e);
                            eprintln!("   サーバーが起動しているか確認してください");
                            std::process::exit(1);
                        }
                    }
                } else if shutdown {
                    match client::shutdown_server() {
                        Ok(_) => {
                            std::process::exit(0);
                        }
                        Err(e) => {
                            eprintln!("❌ エラー: サーバーシャットダウンに失敗しました: {}", e);
                            eprintln!("   サーバーが起動しているか確認してください");
                            std::process::exit(1);
                        }
                    }
                } else if let Some(json_path) = json_file {
                    // Read JSON file content
                    match std::fs::read_to_string(&json_path) {
                        Ok(json_content) => match client::send_json(&json_content) {
                            Ok(_) => {
                                std::process::exit(0);
                            }
                            Err(e) => {
                                eprintln!("❌ エラー: 演奏要求の送信に失敗しました: {}", e);
                                eprintln!("   サーバーが起動しているか確認してください");
                                std::process::exit(1);
                            }
                        },
                        Err(e) => {
                            eprintln!("❌ エラー: JSONファイルの読み込みに失敗しました: {}", e);
                            std::process::exit(1);
                        }
                    }
                } else {
                    eprintln!("❌ エラー: --client オプションには引数が必要です");
                    eprintln!(
                        "   --stop または --shutdown を使用するか、JSONファイルを指定してください"
                    );
                    std::process::exit(1);
                }
            }
            #[cfg(not(windows))]
            {
                let _ = (json_file, verbose, stop, shutdown);
                eprintln!("❌ エラー: クライアントモードはWindowsでのみサポートされています");
                std::process::exit(1);
            }
        }
        None => {
            // Fall through to standalone mode handling below
        }
    }

    // Standalone mode
    let json_path = match cli.json_file {
        Some(path) => path,
        None => {
            // When no arguments provided, show Japanese help message
            eprintln!("YM2151 Log Player - Rust implementation");
            eprintln!();
            eprintln!("使用方法:");
            eprintln!("  ym2151-log-play-server <json_log_file>              # スタンドアロン演奏");
            eprintln!("  ym2151-log-play-server --server [--verbose]         # サーバーとして起動");
            eprintln!(
                "  ym2151-log-play-server --client <json_file> [--verbose]  # サーバーに演奏指示"
            );
            eprintln!("  ym2151-log-play-server --client --shutdown [--verbose]   # サーバーにシャットダウン指示");
            eprintln!();
            eprintln!("例:");
            eprintln!("  ym2151-log-play-server events.json");
            eprintln!("  ym2151-log-play-server sample_events.json");
            eprintln!("  ym2151-log-play-server --server");
            eprintln!("  ym2151-log-play-server --server --verbose");
            eprintln!("  ym2151-log-play-server --client test_input.json");
            eprintln!("  ym2151-log-play-server --client test_input.json --verbose");
            eprintln!("  ym2151-log-play-server --client --stop");
            eprintln!("  ym2151-log-play-server --client --shutdown");
            eprintln!();
            eprintln!("機能:");
            eprintln!("  - JSONイベントログファイルを読み込み");
            eprintln!("  - YM2151レジスタ操作を再現");
            eprintln!("  - リアルタイム音声再生");
            eprintln!("  - WAVファイル (output.wav) を生成");
            eprintln!("  - サーバー/クライアントモード (Windows)");
            eprintln!();
            eprintln!("サーバーオプション:");
            eprintln!("  --verbose  デバッグ用に詳細なログを出力 (通常時はログファイルのみ)");
            eprintln!();
            eprintln!("クライアントオプション:");
            eprintln!("  --verbose  デバッグ用に詳細な状態メッセージを出力");
            eprintln!("             (デフォルトはサイレント、TUIアプリでは非推奨)");
            std::process::exit(1);
        }
    };

    // Check for invalid flags in standalone mode
    if json_path.starts_with("--") {
        eprintln!("❌ エラー: 不明なオプション: {}", json_path);
        eprintln!();
        if json_path == "--no-audio" {
            eprintln!("ヒント: --no-audio オプションは廃止されました。");
            eprintln!("      CI/ヘッドレス環境では、ALSA設定を使用してください。");
            eprintln!("      詳細は CI_TDD_GUIDE.md または README.md を参照してください。");
        } else {
            eprintln!("ヒント: スタンドアロン演奏モードではオプションは不要です。");
            eprintln!("      JSONファイルのパスを直接指定してください。");
        }
        eprintln!();
        std::process::exit(1);
    }

    println!("YM2151 Log Player (Rust)");
    println!("=====================================\n");

    println!("イベントログを読み込み中: {}...", json_path);
    let log = match EventLog::from_file(&json_path) {
        Ok(log) => {
            if !log.validate() {
                eprintln!("❌ エラー: イベントログの検証に失敗しました");
                eprintln!("   event_count と events 配列の長さが一致しません");
                std::process::exit(1);
            }
            println!("✅ {} イベントを読み込みました", log.event_count);

            if let Some(last_event) = log.events.last() {
                let duration_samples = last_event.time;
                let duration_seconds = duration_samples as f64 / OPM_SAMPLE_RATE as f64;
                println!("   再生時間: 約 {:.2} 秒", duration_seconds);
            }
            log
        }
        Err(e) => {
            eprintln!("❌ エラー: JSONファイルの読み込みに失敗しました: {}", e);
            eprintln!("   ファイルが存在し、正しいJSON形式か確認してください");
            std::process::exit(1);
        }
    };

    println!("\nオーディオを初期化中...");

    let player = Player::new(log.clone());

    use ym2151_log_play_server::audio::AudioPlayer;
    match AudioPlayer::new(player) {
        Ok(mut audio_player) => {
            println!("✅ オーディオを初期化しました\n");

            audio_player.wait();

            println!("\nWAVファイルを保存中...");
            let wav_samples_55k = audio_player.get_wav_buffer_55k();
            match wav_writer::write_wav(
                wav_writer::DEFAULT_OUTPUT_FILENAME,
                &wav_samples_55k,
                Player::sample_rate(),
            ) {
                Ok(_) => {
                    println!(
                        "✅ WAVファイルを作成しました: {}",
                        wav_writer::DEFAULT_OUTPUT_FILENAME
                    );
                }
                Err(e) => {
                    eprintln!("❌ エラー: WAVファイルの保存に失敗しました: {}", e);
                    std::process::exit(1);
                }
            }

            // Debug WAV output if enabled
            if debug_wav::is_debug_wav_enabled() {
                println!("\nデバッグWAVファイルを生成中...");
                let realtime_55k = audio_player.get_wav_buffer_55k();
                let realtime_48k = audio_player.get_wav_buffer_48k();

                match debug_wav::generate_post_playback_buffers(&log) {
                    Ok((post_55k, post_48k)) => {
                        match debug_wav::save_debug_wav_files(
                            &realtime_55k,
                            &realtime_48k,
                            &post_55k,
                            &post_48k,
                        ) {
                            Ok(_) => {
                                println!("✅ デバッグWAVファイルの生成が完了しました");
                            }
                            Err(e) => {
                                eprintln!(
                                    "⚠️  警告: デバッグWAVファイルの保存に失敗しました: {}",
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  警告: デバッグWAVファイルの生成に失敗しました: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ エラー: オーディオの初期化に失敗しました: {}", e);
            eprintln!();
            eprintln!("ヒント: Linux/CI環境では、ALSA設定ファイル (~/.asoundrc) を使用して");
            eprintln!("       音声出力をファイルにリダイレクトできます。");
            eprintln!("       詳細はREADME.mdを参照してください。");
            std::process::exit(1);
        }
    }

    println!("\n✅ 再生完了!");
}
