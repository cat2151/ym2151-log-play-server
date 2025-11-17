use clap::{Parser, Subcommand};
#[cfg(windows)]
use ym2151_log_play_server::logging;

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
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// サーバーとして起動
    Server {
        /// デバッグ用に詳細なログを出力 (通常時はログファイルのみ)
        #[arg(long)]
        verbose: bool,

        /// 高品位リサンプリングを使用 (Rubato FFTベース、折り返しノイズを低減)
        #[arg(long)]
        high_quality_resampling: bool,
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
                        eprintln!("❌ エラー: 不明なオプション: {}", arg);
                        eprintln!();
                        eprintln!("使用方法:");
                        eprintln!("  ym2151-log-play-server server [--verbose]");
                        eprintln!("  ym2151-log-play-server client <JSON_FILE> [--verbose]");
                        eprintln!("  ym2151-log-play-server client --stop [--verbose]");
                        eprintln!("  ym2151-log-play-server client --shutdown [--verbose]");
                    } else {
                        eprintln!("{}", e);
                    }
                }
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    print!("{}", e);
                    std::process::exit(0);
                }
                clap::error::ErrorKind::MissingSubcommand => {
                    eprintln!("YM2151 Log Player - Rust implementation");
                    eprintln!();
                    eprintln!("使用方法:");
                    eprintln!(
                        "  ym2151-log-play-server server [--verbose] [--high-quality-resampling]         # サーバーとして起動"
                    );
                    eprintln!(
                        "  ym2151-log-play-server client <json_file> [--verbose]  # サーバーに演奏指示"
                    );
                    eprintln!(
                        "  ym2151-log-play-server client --stop [--verbose]       # 演奏を停止"
                    );
                    eprintln!("  ym2151-log-play-server client --shutdown [--verbose]   # サーバーをシャットダウン");
                    eprintln!();
                    eprintln!("例:");
                    eprintln!("  ym2151-log-play-server server");
                    eprintln!("  ym2151-log-play-server server --verbose");
                    eprintln!("  ym2151-log-play-server server --high-quality-resampling");
                    eprintln!(
                        "  ym2151-log-play-server server --verbose --high-quality-resampling"
                    );
                    eprintln!("  ym2151-log-play-server client test_input.json");
                    eprintln!("  ym2151-log-play-server client test_input.json --verbose");
                    eprintln!("  ym2151-log-play-server client --stop");
                    eprintln!("  ym2151-log-play-server client --shutdown");
                    eprintln!();
                    eprintln!("機能:");
                    eprintln!("  - サーバー/クライアントモード (Windows)");
                    eprintln!("  - JSONイベントログファイルを読み込み");
                    eprintln!("  - YM2151レジスタ操作を再現");
                    eprintln!("  - リアルタイム音声再生");
                    eprintln!("  - WAVファイル (output.wav) を生成 (verbose時)");
                    eprintln!();
                    eprintln!("サーバーオプション:");
                    eprintln!(
                        "  --verbose                 デバッグ用に詳細なログを出力 (通常時はログファイルのみ)"
                    );
                    eprintln!("                            verbose時にWAVファイルを出力します");
                    eprintln!(
                        "  --high-quality-resampling 高品位リサンプリングを使用 (Rubato FFTベース、折り返しノイズを低減)"
                    );
                    eprintln!();
                    eprintln!("クライアントオプション:");
                    eprintln!("  --verbose  デバッグ用に詳細な状態メッセージを出力");
                    eprintln!("             (デフォルトはサイレント、TUIアプリでは非推奨)");
                }
                _ => {
                    eprintln!("{}", e);
                }
            }
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Server {
            verbose,
            high_quality_resampling,
        } => {
            #[cfg(windows)]
            {
                // Initialize logging with verbose flag
                logging::init(verbose);

                let server = Server::new_with_resampling_quality(high_quality_resampling);
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
                let _ = (verbose, high_quality_resampling);
                eprintln!("❌ エラー: サーバーモードはWindowsでのみサポートされています");
                std::process::exit(1);
            }
        }
        Commands::Client {
            json_file,
            verbose,
            stop,
            shutdown,
        } => {
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
                    eprintln!("❌ エラー: client コマンドには引数が必要です");
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
    }
}
