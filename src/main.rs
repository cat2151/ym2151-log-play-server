use std::env;
use ym2151_log_player_rust::events::EventLog;
use ym2151_log_player_rust::player::Player;
use ym2151_log_player_rust::wav_writer;

/// Display usage information
fn print_usage(program_name: &str) {
    eprintln!("YM2151 Log Player - Rust implementation");
    eprintln!();
    eprintln!("使用方法:");
    eprintln!("  {} <json_log_file>", program_name);
    eprintln!();
    eprintln!("例:");
    eprintln!("  {} events.json", program_name);
    eprintln!("  {} sample_events.json", program_name);
    eprintln!();
    eprintln!("機能:");
    eprintln!("  - JSONイベントログファイルを読み込み");
    eprintln!("  - YM2151レジスタ操作を再現");
    eprintln!("  - WAVファイル (output.wav) を生成");
    #[cfg(feature = "realtime-audio")]
    eprintln!("  - リアルタイム音声再生");
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }
    
    let json_path = &args[1];
    
    // Print banner
    println!("YM2151 Log Player (Rust)");
    println!("=====================================\n");
    
    // Load event log
    println!("イベントログを読み込み中: {}...", json_path);
    let log = match EventLog::from_file(json_path) {
        Ok(log) => {
            if !log.validate() {
                eprintln!("❌ エラー: イベントログの検証に失敗しました");
                eprintln!("   event_count と events 配列の長さが一致しません");
                std::process::exit(1);
            }
            println!("✅ {} イベントを読み込みました", log.event_count);
            
            if !log.events.is_empty() {
                let duration_samples = log.events.last().unwrap().time;
                let duration_seconds = duration_samples as f64 / 55930.0; // OPM native sample rate
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
    
    // Generate WAV file
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
    
    // Real-time audio playback (if enabled)
    #[cfg(feature = "realtime-audio")]
    {
        println!("\nリアルタイム再生中...");
        println!("(Ctrl+C で停止)");
        
        // Reload events for playback
        let log = match EventLog::from_file(json_path) {
            Ok(log) => log,
            Err(e) => {
                eprintln!("❌ エラー: イベントログの再読み込みに失敗しました: {}", e);
                std::process::exit(1);
            }
        };
        
        let player = Player::new(log);
        
        use ym2151_log_player_rust::audio::AudioPlayer;
        match AudioPlayer::new(player) {
            Ok(mut audio_player) => {
                println!("▶  再生開始");
                
                // Wait for playback to complete
                audio_player.wait();
                
                println!("■  再生完了");
            }
            Err(e) => {
                eprintln!("⚠️  リアルタイム音声再生が利用できません: {}", e);
                eprintln!("   (音声デバイスが必要です)");
            }
        }
    }
    
    #[cfg(not(feature = "realtime-audio"))]
    {
        println!("\n注: リアルタイム音声再生は有効ではありません");
        println!("    --features realtime-audio でビルドすると有効になります");
    }
    
    println!("\n✅ 完了!");
}
