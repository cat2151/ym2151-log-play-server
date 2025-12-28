use clap::{Parser, Subcommand};
use ym2151_log_play_server::client;
use ym2151_log_play_server::demo_client_interactive;
use ym2151_log_play_server::demo_server_interactive;
use ym2151_log_play_server::demo_server_non_interactive;
use ym2151_log_play_server::logging;
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

        /// 低品位リサンプリングを使用 (線形補間、比較用)
        #[arg(long)]
        low_quality_resampling: bool,

        /// インタラクティブデモモード (output_ym2151.jsonを使用してサーバー単体テスト)
        #[arg(long)]
        demo_interactive: bool,

        /// 非インタラクティブデモモード (output_ym2151.jsonを使用して音響テスト)
        #[arg(long)]
        demo_non_interactive: bool,
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

        /// インタラクティブモードデモ（output_ym2151.jsonを1秒ごとに5回繰り返し演奏）
        #[arg(long)]
        demo_interactive: bool,
    },
}

/// Display usage information and examples
fn print_usage() {
    eprintln!("YM2151 Log Player - Rust implementation");
    eprintln!();
    eprintln!("使用方法:");
    eprintln!(
        "  ym2151-log-play-server server [--verbose] [--low-quality-resampling] [--demo-interactive] [--demo-non-interactive]  # サーバーとして起動"
    );
    eprintln!(
        "  ym2151-log-play-server client <json_file> [--verbose] [--demo-interactive]  # サーバーに演奏指示"
    );
    eprintln!(
        "  ym2151-log-play-server client --stop [--verbose]       # 演奏を停止"
    );
    eprintln!("  ym2151-log-play-server client --shutdown [--verbose]   # サーバーをシャットダウン");
    eprintln!();
    eprintln!("例:");
    eprintln!("  ym2151-log-play-server server");
    eprintln!("  ym2151-log-play-server server --verbose");
    eprintln!("  ym2151-log-play-server server --low-quality-resampling");
    eprintln!("  ym2151-log-play-server server --verbose --low-quality-resampling");
    eprintln!("  ym2151-log-play-server server --demo-interactive");
    eprintln!("  ym2151-log-play-server server --demo-non-interactive");
    eprintln!("  ym2151-log-play-server client test_input.json");
    eprintln!("  ym2151-log-play-server client test_input.json --verbose");
    eprintln!("  ym2151-log-play-server client --stop");
    eprintln!("  ym2151-log-play-server client --shutdown");
    eprintln!("  ym2151-log-play-server client --demo-interactive");
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
        "  --low-quality-resampling  低品位リサンプリングを使用 (線形補間、比較用)"
    );
    eprintln!(
        "                            デフォルトは高品位リサンプリング (Rubato FFTベース、折り返しノイズを低減)"
    );
    eprintln!(
        "  --demo-interactive        インタラクティブデモモード (output_ym2151.jsonを使用してサーバー単体テスト)"
    );
    eprintln!(
        "  --demo-non-interactive    非インタラクティブデモモード (output_ym2151.jsonを使用して音響テスト)"
    );
    eprintln!();
    eprintln!("クライアントオプション:");
    eprintln!("  --verbose          デバッグ用に詳細な状態メッセージを出力");
    eprintln!("                     (デフォルトはサイレント、TUIアプリでは非推奨)");
    eprintln!("  --demo-interactive インタラクティブモードデモ");
    eprintln!("                     (output_ym2151.jsonを1秒ごとに5回繰り返し演奏)");
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
                        print_usage();
                    } else {
                        eprintln!("{}", e);
                    }
                }
                clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion => {
                    print!("{}", e);
                    std::process::exit(0);
                }
                clap::error::ErrorKind::MissingSubcommand => {
                    print_usage();
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
            low_quality_resampling,
            demo_interactive,
            demo_non_interactive,
        } => {
            // Initialize logging with verbose flag
            logging::init(verbose);

            if demo_interactive && demo_non_interactive {
                logging::log_always_server("❌ エラー: --demo-interactive と --demo-non-interactive は同時に使用できません");
                std::process::exit(1);
            } else if demo_interactive {
                // Run server demo mode
                match demo_server_interactive::run_server_demo(verbose, low_quality_resampling) {
                    Ok(_) => {
                        std::process::exit(0);
                    }
                    Err(e) => {
                        logging::log_always_server(&format!(
                            "❌ エラー: サーバーデモモードの実行に失敗しました: {}",
                            e
                        ));
                        std::process::exit(1);
                    }
                }
            } else if demo_non_interactive {
                // Run non-interactive demo mode
                match demo_server_non_interactive::run_server_demo_non_interactive(
                    verbose,
                    low_quality_resampling,
                ) {
                    Ok(_) => {
                        std::process::exit(0);
                    }
                    Err(e) => {
                        logging::log_always_server(&format!(
                            "❌ エラー: 非インタラクティブデモモードの実行に失敗しました: {}",
                            e
                        ));
                        std::process::exit(1);
                    }
                }
            } else {
                // Run normal server mode
                let server = Server::new_with_resampling_quality(low_quality_resampling);
                match server.run() {
                    Ok(_) => {
                        std::process::exit(0);
                    }
                    Err(e) => {
                        logging::log_always_server(&format!(
                            "❌ エラー: サーバーの起動に失敗しました: {}",
                            e
                        ));
                        std::process::exit(1);
                    }
                }
            }
        }
        Commands::Client {
            json_file,
            verbose,
            stop,
            shutdown,
            demo_interactive,
        } => {
            // Initialize client with verbose flag
            client::init_client(verbose);

            // Handle different client commands
            if demo_interactive {
                match demo_client_interactive::run_interactive_demo(verbose) {
                    Ok(_) => {
                        std::process::exit(0);
                    }
                    Err(e) => {
                        logging::log_always_server(&format!(
                            "❌ エラー: インタラクティブモードデモの実行に失敗しました: {}",
                            e
                        ));
                        std::process::exit(1);
                    }
                }
            } else if stop {
                match client::stop_playback() {
                    Ok(_) => {
                        std::process::exit(0);
                    }
                    Err(e) => {
                        logging::log_always_server(&format!(
                            "❌ エラー: 演奏停止に失敗しました: {}",
                            e
                        ));
                        logging::log_always_server("   サーバーが起動しているか確認してください");
                        std::process::exit(1);
                    }
                }
            } else if shutdown {
                match client::shutdown_server() {
                    Ok(_) => {
                        std::process::exit(0);
                    }
                    Err(e) => {
                        logging::log_always_server(&format!(
                            "❌ エラー: サーバーのシャットダウンに失敗しました: {}",
                            e
                        ));
                        logging::log_always_server("   サーバーが起動しているか確認してください");
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
                            logging::log_always_server(&format!(
                                "❌ エラー: JSONファイルの送信に失敗しました: {}",
                                e
                            ));
                            logging::log_always_server(
                                "   サーバーが起動しているか確認してください",
                            );
                            std::process::exit(1);
                        }
                    },
                    Err(e) => {
                        logging::log_always_server(&format!(
                            "❌ エラー: JSONファイルの読み込みに失敗しました: {}",
                            e
                        ));
                        logging::log_always_server(&format!("   ファイルパス: {}", json_path));
                        std::process::exit(1);
                    }
                }
            } else {
                logging::log_always_server("❌ エラー: client コマンドには引数が必要です");
                logging::log_always_server("   --stop, --shutdown, --demo-interactive を使用するか、JSONファイルを指定してください");
                std::process::exit(1);
            }
        }
    }
}
