use ym2151_log_play_server::opm_ffi::*;
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
