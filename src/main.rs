use ym2151_log_player_rust::opm::OpmChip;
use ym2151_log_player_rust::events::EventLog;

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
    match EventLog::from_file("sample_events.json") {
        Ok(log) => {
            println!("✅ Loaded sample_events.json");
            println!("   Event count: {}", log.event_count);
            println!("   Events loaded: {}", log.events.len());
            println!("   Valid: {}", log.validate());
            
            if log.events.len() > 0 {
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
        }
        Err(e) => {
            eprintln!("❌ Failed to load sample_events.json: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("\n=====================================");
    println!("Phase 1 & Phase 2: Complete! ✅");
}
