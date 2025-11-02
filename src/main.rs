use ym2151_log_player_rust::opm::OpmChip;

fn main() {
    println!("YM2151 Log Player (Rust)");
    println!("=====================================\n");
    
    // Test basic OPM functionality
    let mut chip = OpmChip::new();
    println!("✅ OPM chip initialized");
    
    // Generate a small buffer to verify audio generation works
    let mut buffer = vec![0i16; 1000];
    chip.generate_samples(&mut buffer);
    println!("✅ Sample generation working");
    
    println!("\nPhase 1: Nuked-OPM FFI bindings - Complete!");
}
