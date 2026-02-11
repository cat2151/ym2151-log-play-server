use std::os::raw::c_int;

#[repr(C)]
pub struct opm_t {
    _private: [u8; 1396],
}

extern "C" {

    pub fn OPM_Reset(chip: *mut opm_t);

    pub fn OPM_Write(chip: *mut opm_t, port: u32, data: u8);

    pub fn OPM_Clock(chip: *mut opm_t, output: *mut c_int, sh1: *mut u8, sh2: *mut u8, so: *mut u8);

    pub fn call_opm_clock_64times(chip: *mut opm_t, output: *mut c_int);

    pub fn OPM_Read(chip: *mut opm_t, port: u32) -> u8;

    pub fn OPM_ReadIRQ(chip: *mut opm_t) -> u8;

    pub fn OPM_ReadCT1(chip: *mut opm_t) -> u8;

    pub fn OPM_ReadCT2(chip: *mut opm_t) -> u8;

    pub fn OPM_SetIC(chip: *mut opm_t, ic: u8);
}
