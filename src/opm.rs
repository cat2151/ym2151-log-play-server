// Safe Rust wrapper for the Nuked-OPM emulator
//
// This module provides a safe, idiomatic Rust API for the OPM (YM2151) chip emulator.

use crate::opm_ffi;
use std::mem;

/// Port constants for OPM register writes
const OPM_PORT_ADDRESS: u32 = 0;
const OPM_PORT_DATA: u32 = 1;

/// Safe wrapper for the OPM chip emulator.
///
/// This structure manages the lifecycle and safe interaction with the
/// underlying C implementation of the Nuked-OPM emulator.
pub struct OpmChip {
    chip: opm_ffi::opm_t,
}

impl OpmChip {
    /// Create a new OPM chip instance and initialize it.
    ///
    /// # Examples
    ///
    /// ```
    /// use ym2151_log_player_rust::opm::OpmChip;
    ///
    /// let mut chip = OpmChip::new();
    /// ```
    pub fn new() -> Self {
        unsafe {
            let mut chip: opm_ffi::opm_t = mem::zeroed();
            opm_ffi::OPM_Reset(&mut chip);
            Self { chip }
        }
    }

    /// Write to an OPM register.
    ///
    /// This is a low-level function that directly writes to the chip's registers.
    /// For proper operation, you typically need to:
    /// 1. Write the register address to port 0
    /// 2. Wait a few cycles
    /// 3. Write the data value to port 1
    ///
    /// # Parameters
    /// - `port`: Register port (0 = address, 1 = data)
    /// - `data`: Byte value to write
    ///
    /// # Examples
    ///
    /// ```
    /// use ym2151_log_player_rust::opm::OpmChip;
    ///
    /// let mut chip = OpmChip::new();
    /// chip.write(0, 0x08);  // Write address 0x08
    /// chip.write(1, 0x00);  // Write data 0x00
    /// ```
    pub fn write(&mut self, port: u8, data: u8) {
        unsafe {
            opm_ffi::OPM_Write(&mut self.chip, port as u32, data);
        }
    }

    /// Write to an OPM register using address and data separately.
    ///
    /// This is a higher-level convenience function that handles the two-step
    /// write process automatically (address then data).
    ///
    /// # Parameters
    /// - `address`: Register address to write to
    /// - `data`: Data value to write
    ///
    /// # Examples
    ///
    /// ```
    /// use ym2151_log_player_rust::opm::OpmChip;
    ///
    /// let mut chip = OpmChip::new();
    /// chip.write_register(0x08, 0x00);
    /// ```
    pub fn write_register(&mut self, address: u8, data: u8) {
        self.write(OPM_PORT_ADDRESS as u8, address);
        self.write(OPM_PORT_DATA as u8, data);
    }

    /// Generate audio samples from the OPM chip.
    ///
    /// This function advances the chip's internal state and generates stereo
    /// audio samples. The buffer should be sized as `num_samples * 2` to
    /// accommodate interleaved stereo output (left, right, left, right, ...).
    ///
    /// # Parameters
    /// - `buffer`: Output buffer for interleaved stereo i16 samples
    ///
    /// # Panics
    /// Panics if the buffer length is not even (stereo requires pairs).
    ///
    /// # Examples
    ///
    /// ```
    /// use ym2151_log_player_rust::opm::OpmChip;
    ///
    /// let mut chip = OpmChip::new();
    /// let mut buffer = vec![0i16; 1024]; // 512 stereo samples
    /// chip.generate_samples(&mut buffer);
    /// ```
    pub fn generate_samples(&mut self, buffer: &mut [i16]) {
        assert!(buffer.len() % 2 == 0, "Buffer length must be even for stereo output");

        let num_samples = buffer.len() / 2;
        
        // The OPM_Clock function generates one stereo sample per call
        // and outputs 32-bit values that need to be converted to 16-bit
        for i in 0..num_samples {
            let mut output: [i32; 2] = [0; 2];
            
            unsafe {
                opm_ffi::OPM_Clock(
                    &mut self.chip,
                    output.as_mut_ptr(),
                    std::ptr::null_mut(), // sh1 - not used
                    std::ptr::null_mut(), // sh2 - not used
                    std::ptr::null_mut(), // so - not used
                );
            }
            
            // Convert 32-bit samples to 16-bit and store in buffer
            // The OPM outputs values in roughly -16384 to +16384 range,
            // so we need to scale and clamp them to i16 range
            buffer[i * 2] = Self::convert_sample(output[0]);
            buffer[i * 2 + 1] = Self::convert_sample(output[1]);
        }
    }

    /// Convert a 32-bit OPM output sample to 16-bit signed integer.
    ///
    /// The OPM generates samples in a larger range than typical 16-bit audio,
    /// so we need to clamp the values to prevent overflow.
    #[inline]
    fn convert_sample(sample: i32) -> i16 {
        sample.clamp(i16::MIN as i32, i16::MAX as i32) as i16
    }

    /// Reset the OPM chip to its initial state.
    ///
    /// This clears all registers and internal state.
    ///
    /// # Examples
    ///
    /// ```
    /// use ym2151_log_player_rust::opm::OpmChip;
    ///
    /// let mut chip = OpmChip::new();
    /// // ... do some operations ...
    /// chip.reset();
    /// ```
    pub fn reset(&mut self) {
        unsafe {
            opm_ffi::OPM_Reset(&mut self.chip);
        }
    }
}

impl Default for OpmChip {
    fn default() -> Self {
        Self::new()
    }
}

// OpmChip is safe to send between threads as it doesn't use
// any thread-local state or raw pointers that escape its lifetime
unsafe impl Send for OpmChip {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let _chip = OpmChip::new();
        // If we get here without panicking, creation works
    }

    #[test]
    fn test_chip_write() {
        let mut chip = OpmChip::new();
        // Test basic register writes - should not panic
        chip.write(0, 0x08);
        chip.write(1, 0x00);
    }

    #[test]
    fn test_write_register() {
        let mut chip = OpmChip::new();
        // Test convenience function
        chip.write_register(0x08, 0x00);
    }

    #[test]
    fn test_generate_samples() {
        let mut chip = OpmChip::new();
        let mut buffer = vec![0i16; 1024]; // 512 stereo samples
        
        // Should generate samples without panicking
        chip.generate_samples(&mut buffer);
        
        // Initially, with no setup, samples should be mostly silent
        // (not all zeros due to chip initialization state)
    }

    #[test]
    #[should_panic(expected = "Buffer length must be even")]
    fn test_generate_samples_odd_buffer() {
        let mut chip = OpmChip::new();
        let mut buffer = vec![0i16; 1023]; // Odd length - should panic
        chip.generate_samples(&mut buffer);
    }

    #[test]
    fn test_reset() {
        let mut chip = OpmChip::new();
        chip.write_register(0x08, 0xFF);
        chip.reset();
        
        // After reset, chip should be in initial state
        let mut buffer = vec![0i16; 100];
        chip.generate_samples(&mut buffer);
    }

    #[test]
    fn test_default() {
        let _chip = OpmChip::default();
    }

    #[test]
    fn test_sample_generation_with_register_writes() {
        let mut chip = OpmChip::new();
        
        // Set up a simple tone (following sample_events.json pattern)
        // Configure operator settings
        chip.write_register(0x20, 0xC7); // RL/FB/CON for channel 0
        chip.write_register(0x38, 0x00); // PMS/AMS for channel 0  
        chip.write_register(0x40, 0x01); // DT1/MUL for operator M1
        chip.write_register(0x60, 0x00); // TL for operator M1
        chip.write_register(0x80, 0x1F); // KS/AR for operator M1
        chip.write_register(0xA0, 0x05); // AMS_EN/D1R for operator M1
        chip.write_register(0xC0, 0x05); // DT2/D2R for operator M1
        chip.write_register(0xE0, 0xF7); // D1L/RR for operator M1
        chip.write_register(0x28, 0x3E); // KC (key code) for channel 0
        chip.write_register(0x30, 0x00); // KF (key fraction) for channel 0
        chip.write_register(0x08, 0x78); // Key on for channel 0, all operators
        
        // Generate samples - the chip should process these registers
        let mut buffer = vec![0i16; 10000];
        chip.generate_samples(&mut buffer);
        
        // We've successfully exercised the register write and sample generation paths
        // The actual audio output depends on proper operator configuration which is
        // beyond the scope of this basic FFI binding test
    }
}
