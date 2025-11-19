use ym2151_log_play_server::opm::OpmChip;

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
fn test_default() {
    let _chip = OpmChip::default();
}
