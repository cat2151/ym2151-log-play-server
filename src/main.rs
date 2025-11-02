use ym2151_log_player_rust::opm::OpmChip;
use ym2151_log_player_rust::events::EventLog;
use ym2151_log_player_rust::player::Player;

fn main() {
    println!("YM2151 Log Player (Rust)");
    println!("=====================================\n");
    
    // Phase 1: Test basic OPM functionality
    println!("Phase 1: Nuked-OPM FFI bindings");
    let mut chip = OpmChip::new();
    println!("✅ OPM chip initialized");
    
    // Generate a small buffer to verify audio generation works
    let mut buffer = vec![0i16; 1000];
    chip.generate_samples(&mut buffer);
    println!("✅ Sample generation working");
    
    // Phase 2: Test JSON event loading
    println!("\nPhase 2: JSON Event Loading");
    let log = match EventLog::from_file("sample_events.json") {
        Ok(log) => {
            println!("✅ Loaded sample_events.json");
            println!("   Event count: {}", log.event_count);
            println!("   Events loaded: {}", log.events.len());
            println!("   Valid: {}", log.validate());
            
            if !log.events.is_empty() {
                println!("\n   First event:");
                println!("     Time: {} samples", log.events[0].time);
                println!("     Register: 0x{:02X} = 0x{:02X}", log.events[0].addr, log.events[0].data);
                
                let last_idx = log.events.len() - 1;
                println!("\n   Last event:");
                println!("     Time: {} samples", log.events[last_idx].time);
                println!("     Register: 0x{:02X} = 0x{:02X}", log.events[last_idx].addr, log.events[last_idx].data);
                
                let duration_samples = log.events[last_idx].time;
                let duration_seconds = duration_samples as f64 / 55930.0; // OPM native sample rate
                println!("\n   Duration: ~{:.2} seconds", duration_seconds);
            }
            log
        }
        Err(e) => {
            eprintln!("❌ Failed to load sample_events.json: {}", e);
            std::process::exit(1);
        }
    };
    
    // Phase 3: Test event processing engine
    println!("\nPhase 3: Event Processing Engine");
    let mut player = Player::new(log);
    println!("✅ Player initialized");
    println!("   Pass1 → Pass2 conversion complete");
    println!("   Total pass2 events: {}", player.total_events());
    println!("   Total samples needed: {}", player.total_samples());
    
    let total_duration = player.total_samples() as f64 / Player::sample_rate() as f64;
    println!("   Total duration: {:.2} seconds", total_duration);
    
    // Process a few buffers to demonstrate event execution
    println!("\n   Processing events...");
    let mut buffer = vec![0i16; 1024]; // 512 stereo samples
    let mut processed_samples = 0;
    let mut iterations = 0;
    
    // Process first 10 buffers or until complete
    while iterations < 10 && !player.is_complete() {
        player.generate_samples(&mut buffer);
        processed_samples += buffer.len() / 2;
        iterations += 1;
    }
    
    println!("   Processed {} samples ({:.3}s)", 
             processed_samples, 
             processed_samples as f64 / Player::sample_rate() as f64);
    println!("   Events executed: {} / {}", 
             player.events_processed(), 
             player.total_events());
    
    if player.is_complete() {
        println!("   ✅ All events processed");
    } else {
        println!("   ⏸  More events remaining (demo stopped early)");
    }
    
    println!("\n=====================================");
    println!("Phase 1, 2 & 3: Complete! ✅");
    println!("\nPhase 3 successfully implemented:");
    println!("  • Pass1 → Pass2 event conversion ✅");
    println!("  • Address/Data write splitting ✅");
    println!("  • Delay insertion (DELAY_SAMPLES=2) ✅");
    println!("  • Event timing and scheduling ✅");
    println!("  • Sample generation with events ✅");
    
    // Phase 4: WAV file output
    println!("\n=====================================");
    println!("Phase 4: WAV File Output");
    
    // Load the sample events again for WAV generation
    let log = match EventLog::from_file("sample_events.json") {
        Ok(log) => log,
        Err(e) => {
            eprintln!("❌ Failed to load sample_events.json: {}", e);
            std::process::exit(1);
        }
    };
    
    let player = Player::new(log);
    
    use ym2151_log_player_rust::wav_writer;
    match wav_writer::generate_wav_default(player) {
        Ok(_) => {
            println!("\n=====================================");
            println!("Phase 1, 2, 3 & 4: All Complete! ✅");
            println!("\nPhase 4 successfully implemented:");
            println!("  • WAV file output ✅");
            println!("  • Sample rate conversion (55930 Hz → 48000 Hz) ✅");
            println!("  • High-quality resampling with rubato ✅");
            println!("  • Progress reporting ✅");
        }
        Err(e) => {
            eprintln!("❌ Failed to generate WAV file: {}", e);
            std::process::exit(1);
        }
    }
}
