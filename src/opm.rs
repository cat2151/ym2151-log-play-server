use crate::opm_ffi;
use std::mem;
use std::sync::atomic::{AtomicU64, Ordering};

const OPM_PORT_ADDRESS: u32 = 0;
const OPM_PORT_DATA: u32 = 1;

const CYCLES_PER_SAMPLE: usize = 64;

static FFI_CALL_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct OpmChip {
    chip: opm_ffi::opm_t,
}

impl OpmChip {
    pub fn new() -> Self {
        unsafe {
            let mut chip: opm_ffi::opm_t = mem::zeroed();
            opm_ffi::OPM_Reset(&mut chip);
            Self { chip }
        }
    }

    pub fn write(&mut self, port: u8, data: u8) {
        unsafe {
            opm_ffi::OPM_Write(&mut self.chip, port as u32, data);
        }
    }

    pub fn write_register(&mut self, address: u8, data: u8) {
        self.write(OPM_PORT_ADDRESS as u8, address);
        self.write(OPM_PORT_DATA as u8, data);
    }

    pub fn generate_samples(&mut self, buffer: &mut [i16]) {
        assert!(
            buffer.len().is_multiple_of(2),
            "Buffer length must be even for stereo output"
        );

        let num_samples = buffer.len() / 2;

        for i in 0..num_samples {
            let mut output: [i32; 2] = [0; 2];

            for _ in 0..CYCLES_PER_SAMPLE {
                unsafe {
                    opm_ffi::OPM_Clock(
                        &mut self.chip,
                        output.as_mut_ptr(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                    );
                }
            }

            FFI_CALL_COUNTER.fetch_add(CYCLES_PER_SAMPLE as u64, Ordering::Relaxed);

            buffer[i * 2] = Self::convert_sample(output[0]);
            buffer[i * 2 + 1] = Self::convert_sample(output[1]);
        }
    }

    #[inline]
    fn convert_sample(sample: i32) -> i16 {
        sample.clamp(i16::MIN as i32, i16::MAX as i32) as i16
    }

    pub fn reset(&mut self) {
        unsafe {
            opm_ffi::OPM_Reset(&mut self.chip);
        }
    }

    pub fn get_ffi_call_count() -> u64 {
        FFI_CALL_COUNTER.load(Ordering::Relaxed)
    }

    pub fn reset_ffi_call_count() {
        FFI_CALL_COUNTER.store(0, Ordering::Relaxed);
    }
}

impl Default for OpmChip {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for OpmChip {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let _chip = OpmChip::new();
    }

    #[test]
    fn test_chip_write() {
        let mut chip = OpmChip::new();

        chip.write(0, 0x08);
        chip.write(1, 0x00);
    }

    #[test]
    fn test_write_register() {
        let mut chip = OpmChip::new();

        chip.write_register(0x08, 0x00);
    }

    #[test]
    fn test_generate_samples() {
        let mut chip = OpmChip::new();
        let mut buffer = vec![0i16; 1024];

        chip.generate_samples(&mut buffer);
    }

    #[test]
    #[should_panic(expected = "Buffer length must be even")]
    fn test_generate_samples_odd_buffer() {
        let mut chip = OpmChip::new();
        let mut buffer = vec![0i16; 1023];
        chip.generate_samples(&mut buffer);
    }

    #[test]
    fn test_reset() {
        let mut chip = OpmChip::new();
        chip.write_register(0x08, 0xFF);
        chip.reset();

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

        chip.write_register(0x20, 0xC7);
        chip.write_register(0x38, 0x00);
        chip.write_register(0x40, 0x01);
        chip.write_register(0x60, 0x00);
        chip.write_register(0x80, 0x1F);
        chip.write_register(0xA0, 0x05);
        chip.write_register(0xC0, 0x05);
        chip.write_register(0xE0, 0xF7);
        chip.write_register(0x28, 0x3E);
        chip.write_register(0x30, 0x00);
        chip.write_register(0x08, 0x78);

        let mut buffer = vec![0i16; 10000];
        chip.generate_samples(&mut buffer);
    }
}
