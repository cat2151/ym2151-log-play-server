use std::env;
use ym2151_log_player_rust::events::EventLog;
use ym2151_log_player_rust::player::Player;
use ym2151_log_player_rust::resampler::OPM_SAMPLE_RATE;
use ym2151_log_player_rust::wav_writer;

fn print_usage(program_name: &str) {
    eprintln!("YM2151 Log Player - Rust implementation");
    eprintln!();
    eprintln!("使用方法:");
    eprintln!("  {} [オプション] <json_log_file>", program_name);
    eprintln!();
    eprintln!("オプション:");
    eprintln!("  --no-audio    音声デバイスなしでWAVファイルのみ生成");
    eprintln!("                (デフォルトはリアルタイム再生+WAV保存)");
    eprintln!();
    eprintln!("例:");
    eprintln!("  {} events.json", program_name);
    eprintln!("  {} --no-audio sample_events.json", program_name);
    eprintln!();
    eprintln!("機能:");
    eprintln!("  - JSONイベントログファイルを読み込み");
    eprintln!("  - YM2151レジスタ操作を再現");
    #[cfg(feature = "realtime-audio")]
    eprintln!("  - リアルタイム音声再生 (デフォルト)");
    eprintln!("  - WAVファイル (output.wav) を生成");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let mut no_audio = false;
    let mut json_path = None;

    for arg in args.iter().skip(1) {
        if arg == "--no-audio" {
            no_audio = true;
        } else if !arg.starts_with("--") {
            json_path = Some(arg.as_str());
        }
    }

    let json_path = match json_path {
        Some(path) => path,
        None => {
            print_usage(&args[0]);
            std::process::exit(1);
        }
    };

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

    #[cfg(feature = "realtime-audio")]
    {
        if no_audio {
            println!("\n⚠️  --no-audio モード: 音声デバイスなしでWAVファイルを生成");
            println!("WAVファイルを生成中...");
            let player = Player::new(log);
            match wav_writer::generate_wav_default(player) {
                Ok(_) => {
                    println!("✅ WAVファイルを作成しました: output.wav");
                }
                Err(e) => {
                    eprintln!("❌ エラー: WAVファイルの生成に失敗しました: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            println!("\nオーディオを初期化中...");

            let player = Player::new(log);

            use ym2151_log_player_rust::audio::AudioPlayer;
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
                    eprintln!("   (音声デバイスが必要です)");
                    eprintln!();
                    eprintln!("ヒント: --no-audio オプションを使用すると、");
                    eprintln!("       音声デバイスなしでWAVファイルのみ生成できます");
                    std::process::exit(1);
                }
            }
        }
    }

    #[cfg(not(feature = "realtime-audio"))]
    {
        println!("\nWAVファイルを生成中...");
        let player = Player::new(log);
        match wav_writer::generate_wav_default(player) {
            Ok(_) => {
                println!("✅ WAVファイルを作成しました: output.wav");
            }
            Err(e) => {
                eprintln!("❌ エラー: WAVファイルの生成に失敗しました: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("\n✅ 再生完了!");
}
