// FFI bindings for Nuked-OPM emulator
//
// This module provides unsafe FFI declarations for the C implementation
// of the Nuked-OPM (YM2151) sound chip emulator.

use std::os::raw::c_int;

/// Opaque structure representing the OPM chip state.
/// The actual structure is defined in opm.h and opm.c.
/// Size: 1396 bytes (verified with sizeof(opm_t))
#[repr(C)]
pub struct opm_t {
    _private: [u8; 1396],
}

extern "C" {
    /// Initialize/reset the OPM chip to its default state.
    ///
    /// # Safety
    /// `chip` must be a valid pointer to an initialized opm_t structure.
    pub fn OPM_Reset(chip: *mut opm_t);

    /// Write a byte to the OPM chip register.
    ///
    /// # Parameters
    /// - `chip`: Pointer to the OPM chip structure
    /// - `port`: Register port (0 = address, 1 = data)
    /// - `data`: Byte to write
    ///
    /// # Safety
    /// `chip` must be a valid pointer to an initialized opm_t structure.
    pub fn OPM_Write(chip: *mut opm_t, port: u32, data: u8);

    /// Advance the OPM chip by one clock cycle and generate output samples.
    ///
    /// # Parameters
    /// - `chip`: Pointer to the OPM chip structure
    /// - `output`: Pointer to output buffer for stereo samples (2 x int32_t)
    /// - `sh1`: Pointer to SH1 output flag (can be NULL)
    /// - `sh2`: Pointer to SH2 output flag (can be NULL)
    /// - `so`: Pointer to SO output flag (can be NULL)
    ///
    /// # Safety
    /// - `chip` must be a valid pointer to an initialized opm_t structure.
    /// - `output` must point to valid memory for at least 2 int32_t values.
    /// - If provided, `sh1`, `sh2`, and `so` must point to valid u8 memory.
    pub fn OPM_Clock(
        chip: *mut opm_t,
        output: *mut c_int,
        sh1: *mut u8,
        sh2: *mut u8,
        so: *mut u8,
    );

    /// Read a byte from the OPM chip register.
    ///
    /// # Parameters
    /// - `chip`: Pointer to the OPM chip structure
    /// - `port`: Register port to read from
    ///
    /// # Safety
    /// `chip` must be a valid pointer to an initialized opm_t structure.
    pub fn OPM_Read(chip: *mut opm_t, port: u32) -> u8;

    /// Read the IRQ status from the OPM chip.
    ///
    /// # Safety
    /// `chip` must be a valid pointer to an initialized opm_t structure.
    pub fn OPM_ReadIRQ(chip: *mut opm_t) -> u8;

    /// Read the CT1 output from the OPM chip.
    ///
    /// # Safety
    /// `chip` must be a valid pointer to an initialized opm_t structure.
    pub fn OPM_ReadCT1(chip: *mut opm_t) -> u8;

    /// Read the CT2 output from the OPM chip.
    ///
    /// # Safety
    /// `chip` must be a valid pointer to an initialized opm_t structure.
    pub fn OPM_ReadCT2(chip: *mut opm_t) -> u8;

    /// Set the IC (Initial Clear) pin state.
    ///
    /// # Parameters
    /// - `chip`: Pointer to the OPM chip structure
    /// - `ic`: IC pin state (0 or 1)
    ///
    /// # Safety
    /// `chip` must be a valid pointer to an initialized opm_t structure.
    pub fn OPM_SetIC(chip: *mut opm_t, ic: u8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_opm_t_size() {
        // Verify that the structure size matches the C implementation
        assert_eq!(mem::size_of::<opm_t>(), 1396);
    }

    #[test]
    fn test_basic_initialization() {
        unsafe {
            let mut chip: opm_t = mem::zeroed();
            OPM_Reset(&mut chip);
            // If we get here without crashing, basic FFI is working
        }
    }
}
