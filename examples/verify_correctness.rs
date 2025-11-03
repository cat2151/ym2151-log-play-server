// Verification program to ensure batched and unbatched implementations produce identical output
//
// This program generates audio samples using both implementations and compares them
// to ensure the batch optimization doesn't change the audio output.

use ym2151_log_player_rust::opm::OpmChip;

fn main() {
    println!("=== Batched vs Unbatched Correctness Verification ===\n");

    // Test with multiple buffer sizes
    let test_sizes = vec![64, 128, 1024, 4096];

    let mut all_passed = true;

    for &size in &test_sizes {
        println!("Testing buffer size: {} samples ({} stereo samples)", size, size / 2);

        // Generate samples with unbatched implementation
        let mut chip1 = OpmChip::new();
        // Configure the chip with some register writes to produce non-silent output
        chip1.write_register(0x20, 0xC7); // RL/FB/CON for channel 0
        chip1.write_register(0x28, 0x3E); // KC for channel 0
        chip1.write_register(0x40, 0x01); // DT1/MUL for operator M1
        chip1.write_register(0x60, 0x00); // TL for operator M1
        chip1.write_register(0x80, 0x1F); // KS/AR for operator M1
        chip1.write_register(0x08, 0x78); // Key on for channel 0

        let mut buffer1 = vec![0i16; size];
        chip1.generate_samples_unbatched(&mut buffer1);

        // Generate samples with batched implementation
        let mut chip2 = OpmChip::new();
        // Apply same configuration
        chip2.write_register(0x20, 0xC7);
        chip2.write_register(0x28, 0x3E);
        chip2.write_register(0x40, 0x01);
        chip2.write_register(0x60, 0x00);
        chip2.write_register(0x80, 0x1F);
        chip2.write_register(0x08, 0x78);

        let mut buffer2 = vec![0i16; size];
        chip2.generate_samples(&mut buffer2);

        // Compare buffers
        let mut differences = 0;
        let mut max_diff = 0i32;
        for i in 0..size {
            let diff = (buffer1[i] as i32 - buffer2[i] as i32).abs();
            if diff != 0 {
                differences += 1;
                max_diff = max_diff.max(diff);
            }
        }

        if differences == 0 {
            println!("  ✓ PASS: Output is identical");
        } else {
            println!("  ✗ FAIL: Found {} differences, max difference: {}", differences, max_diff);
            all_passed = false;
            
            // Show first few differences for debugging
            if differences <= 10 {
                println!("  Differences:");
                for i in 0..size {
                    if buffer1[i] != buffer2[i] {
                        println!("    [{}]: unbatched={}, batched={}", i, buffer1[i], buffer2[i]);
                    }
                }
            }
        }
        println!();
    }

    if all_passed {
        println!("=== ✓ All tests passed! ===");
        println!("The batched implementation produces identical output to the unbatched version.");
    } else {
        println!("=== ✗ Some tests failed! ===");
        println!("The batched implementation produces different output.");
        std::process::exit(1);
    }
}
