use std::env;
use ym2151_log_play_server::events::EventLog;
use ym2151_log_play_server::logging;
use ym2151_log_play_server::player::Player;
use ym2151_log_play_server::resampler::OPM_SAMPLE_RATE;
use ym2151_log_play_server::wav_writer;

use ym2151_log_play_server::client;
use ym2151_log_play_server::server::Server;

fn print_usage(program_name: &str) {
    eprintln!("YM2151 Log Player - Rust implementation");
    eprintln!();
    eprintln!("使用方法:");
    eprintln!(
        "  {} <json_log_file>              # スタンドアロン演奏",
        program_name
    );
    eprintln!(
        "  {} --server [--verbose]         # サーバーとして起動",
        program_name
    );
    eprintln!(
        "  {} --client <json_file>         # サーバーに演奏指示",
        program_name
    );
    eprintln!(
        "  {} --client --shutdown          # サーバーにシャットダウン指示",
        program_name
    );
    eprintln!();
    eprintln!("例:");
    eprintln!("  {} events.json", program_name);
    eprintln!("  {} sample_events.json", program_name);
    eprintln!("  {} --server", program_name);
    eprintln!("  {} --server --verbose", program_name);
    eprintln!("  {} --client test_input.json", program_name);
    eprintln!("  {} --client --stop", program_name);
    eprintln!("  {} --client --shutdown", program_name);
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
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        if args[1] == "--server" {
            // Check for --verbose flag
            let verbose = args.iter().any(|arg| arg == "--verbose");

            // Validate arguments
            let valid_args = args
                .iter()
                .skip(1)
                .all(|arg| arg == "--server" || arg == "--verbose");
            if !valid_args {
                eprintln!("❌ エラー: --server に不明なオプションが指定されています");
                eprintln!();
                print_usage(&args[0]);
                std::process::exit(1);
            }

            // Initialize logging with verbose flag
            logging::init(verbose);

            let server = Server::new();
            match server.run() {
                Ok(_) => {
                    std::process::exit(0);
                }
                Err(e) => {
                    logging::log_always(&format!("❌ エラー: サーバーの起動に失敗しました: {}", e));
                    std::process::exit(1);
                }
            }
        } else if args[1] == "--client" {
            if args.len() == 3 && args[2] == "--stop" {
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
            } else if args.len() == 3 && args[2] == "--shutdown" {
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
            } else if args.len() == 3 {
                let json_path = &args[2];

                // Read JSON file content
                match std::fs::read_to_string(json_path) {
                    Ok(json_content) => {
                        match client::send_json(&json_content) {
                            Ok(_) => {
                                std::process::exit(0);
                            }
                            Err(e) => {
                                eprintln!("❌ エラー: 演奏要求の送信に失敗しました: {}", e);
                                eprintln!("   サーバーが起動しているか確認してください");
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ エラー: JSONファイルの読み込みに失敗しました: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("❌ エラー: --client オプションには引数が必要です");
                eprintln!();
                print_usage(&args[0]);
                std::process::exit(1);
            }
        }
    }

    if args.len() != 2 {
        print_usage(&args[0]);
        if args.len() > 2 {
            eprintln!("\n❌ エラー: 引数が多すぎます");
        }
        std::process::exit(1);
    }

    let json_path = &args[1];

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
        print_usage(&args[0]);
        std::process::exit(1);
    }

    println!("YM2151 Log Player (Rust)");
    println!("=====================================\n");

    println!("イベントログを読み込み中: {}...", json_path);
    let log = match EventLog::from_file(json_path) {
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

    let player = Player::new(log);

    use ym2151_log_play_server::audio::AudioPlayer;
    match AudioPlayer::new(player) {
        Ok(mut audio_player) => {
            println!("✅ オーディオを初期化しました\n");

            audio_player.wait();

            println!("\nWAVファイルを保存中...");
            let wav_samples = audio_player.get_wav_buffer();
            match wav_writer::write_wav(
                wav_writer::DEFAULT_OUTPUT_FILENAME,
                &wav_samples,
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
