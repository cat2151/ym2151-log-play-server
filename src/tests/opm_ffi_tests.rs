use crate::opm_ffi::*;
use std::mem;

#[test]
fn test_opm_t_size() {
    assert_eq!(mem::size_of::<opm_t>(), 1396);
}

#[test]
fn test_basic_initialization() {
    unsafe {
        let mut chip: opm_t = mem::zeroed();
        OPM_Reset(&mut chip);
    }
}

#[test]
fn test_call_opm_clock_64times() {
    unsafe {
        let mut chip: opm_t = mem::zeroed();
        let mut output = [0i32; 2];

        OPM_Reset(&mut chip);
        call_opm_clock_64times(&mut chip, output.as_mut_ptr());
    }
}
