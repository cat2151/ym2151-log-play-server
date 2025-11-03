// FFI benchmark to measure and compare batched vs unbatched performance
//
// This program demonstrates the performance difference between calling
// OPM_Clock individually vs using OPM_Clock_Batch.

use std::time::Instant;
use ym2151_log_player_rust::opm::OpmChip;
use ym2151_log_player_rust::player::Player;

// Use constants from the main module
const SAMPLE_RATE: u32 = Player::sample_rate();
const CYCLES_PER_SAMPLE: u32 = 64; // This is internal to OpmChip, not exposed
const DURATION_SECONDS: f64 = 1.0;

fn main() {
    println!("=== YM2151 OPM FFI Benchmark ===\n");
    println!("Sample rate: {} Hz", SAMPLE_RATE);
    println!("Cycles per sample: {}", CYCLES_PER_SAMPLE);
    println!("Test duration: {} seconds\n", DURATION_SECONDS);

    // Calculate expected values
    let samples_per_second = SAMPLE_RATE as f64;
    let expected_ffi_calls_unbatched = samples_per_second * CYCLES_PER_SAMPLE as f64;
    let expected_ffi_calls_batched = samples_per_second;

    println!("Expected FFI calls per second:");
    println!("  Unbatched (individual OPM_Clock): {:.0}", expected_ffi_calls_unbatched);
    println!("  Batched (OPM_Clock_Batch):        {:.0}", expected_ffi_calls_batched);
    println!("  Reduction factor:                 {}x\n", CYCLES_PER_SAMPLE);

    // Test unbatched implementation
    println!("--- Testing Unbatched Implementation ---");
    let samples = (SAMPLE_RATE as f64 * DURATION_SECONDS) as usize;
    let buffer_size = samples * 2; // stereo
    let mut buffer = vec![0i16; buffer_size];

    let mut chip = OpmChip::new();
    OpmChip::reset_ffi_call_count();

    let start = Instant::now();
    chip.generate_samples_unbatched(&mut buffer);
    let elapsed = start.elapsed();

    let ffi_calls = OpmChip::get_ffi_call_count();
    let ffi_calls_per_second = ffi_calls as f64 / elapsed.as_secs_f64();

    println!("Elapsed time:         {:.3} seconds", elapsed.as_secs_f64());
    println!("Total FFI calls:      {}", ffi_calls);
    println!("FFI calls per second: {:.0}", ffi_calls_per_second);
    println!("Time per FFI call:    {:.3} µs", elapsed.as_secs_f64() * 1_000_000.0 / ffi_calls as f64);
    
    // Verify expected call count
    let expected_calls = samples as u64 * CYCLES_PER_SAMPLE as u64;
    if ffi_calls == expected_calls {
        println!("✓ FFI call count matches expected: {} calls", expected_calls);
    } else {
        println!("⚠ FFI call count mismatch: expected {}, got {}", expected_calls, ffi_calls);
    }

    // Test batched implementation
    println!("\n--- Testing Batched Implementation ---");
    let mut chip = OpmChip::new();
    let mut buffer = vec![0i16; buffer_size];
    OpmChip::reset_ffi_call_count();

    let start = Instant::now();
    chip.generate_samples(&mut buffer);
    let elapsed = start.elapsed();

    let ffi_calls = OpmChip::get_ffi_call_count();
    let ffi_calls_per_second = ffi_calls as f64 / elapsed.as_secs_f64();

    println!("Elapsed time:         {:.3} seconds", elapsed.as_secs_f64());
    println!("Total FFI calls:      {}", ffi_calls);
    println!("FFI calls per second: {:.0}", ffi_calls_per_second);
    println!("Time per FFI call:    {:.3} µs", elapsed.as_secs_f64() * 1_000_000.0 / ffi_calls as f64);
    
    // Verify expected call count
    let expected_calls = samples as u64;
    if ffi_calls == expected_calls {
        println!("✓ FFI call count matches expected: {} calls", expected_calls);
    } else {
        println!("⚠ FFI call count mismatch: expected {}, got {}", expected_calls, ffi_calls);
    }

    println!("\n=== Summary ===");
    println!("The unbatched version makes ~{:.1}M FFI calls per second", expected_ffi_calls_unbatched / 1_000_000.0);
    println!("The batched version reduces this to ~{:.0}K FFI calls per second", expected_ffi_calls_batched / 1_000.0);
    println!("This is a {}x reduction in FFI overhead!", CYCLES_PER_SAMPLE);
}
