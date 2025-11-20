use crate::events::{EventLog, RegisterEvent};
use crate::player::Player;
use crate::wav_writer::{generate_wav, write_wav, DEFAULT_OUTPUT_FILENAME};
use std::path::Path;

#[test]
fn test_write_wav_basic() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("test_write_wav.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let samples = vec![0i16; 48000 * 2];
    let result = write_wav(temp_path_str, &samples, 48000);

    assert!(result.is_ok());
    assert!(Path::new(temp_path_str).exists());

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_write_wav_empty() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("test_write_wav_empty.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let samples = vec![];
    let result = write_wav(temp_path_str, &samples, 48000);

    assert!(result.is_ok());
    assert!(Path::new(temp_path_str).exists());

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_write_wav_non_zero() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("test_write_wav_nonzero.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let mut samples = Vec::with_capacity(1000);
    for i in 0..500 {
        samples.push((i * 100) as i16);
        samples.push((i * 100) as i16);
    }

    let result = write_wav(temp_path_str, &samples, 48000);

    assert!(result.is_ok());
    assert!(Path::new(temp_path_str).exists());

    let metadata = std::fs::metadata(temp_path_str).unwrap();
    assert!(metadata.len() > 100);

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_generate_wav_with_player() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("test_generate_wav.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let log = EventLog {
        event_count: 2,
        events: vec![
            RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 100,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
        ],
    };

    let player = Player::new(log);
    let result = generate_wav(player, temp_path_str);

    assert!(result.is_ok(), "Failed to generate WAV: {:?}", result.err());
    assert!(Path::new(temp_path_str).exists());

    let metadata = std::fs::metadata(temp_path_str).unwrap();
    assert!(metadata.len() > 1000);

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_generate_wav_from_simple_events() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("test_generate_from_simple.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let _ = std::fs::remove_file(temp_path_str);

    let log = EventLog::from_file("tests/fixtures/simple.json").expect("Failed to load simple.json");

    let player = Player::new(log);
    let result = generate_wav(player, temp_path_str);

    assert!(result.is_ok(), "Failed to generate WAV: {:?}", result.err());
    assert!(Path::new(temp_path_str).exists());

    let metadata = std::fs::metadata(temp_path_str).unwrap();
    assert!(metadata.len() > 1000);

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_generate_wav_from_sample_events() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("test_generate_from_sample.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let _ = std::fs::remove_file(temp_path_str);

    let log = EventLog::from_file("output_ym2151.json").expect("Failed to load output_ym2151.json");

    let player = Player::new(log);
    let result = generate_wav(player, temp_path_str);

    assert!(result.is_ok(), "Failed to generate WAV: {:?}", result.err());
    assert!(Path::new(temp_path_str).exists());

    let metadata = std::fs::metadata(temp_path_str).unwrap();
    assert!(metadata.len() > 10000);

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_wav_format_verification() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("test_wav_format.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let _ = std::fs::remove_file(temp_path_str);

    let samples = vec![0i16; 48000 * 2];
    write_wav(temp_path_str, &samples, 48000).expect("Failed to write WAV");

    let reader = hound::WavReader::open(temp_path_str).expect("Failed to open WAV");
    let spec = reader.spec();

    assert_eq!(spec.channels, 2, "Expected stereo");
    assert_eq!(spec.sample_rate, 48000, "Expected 48kHz");
    assert_eq!(spec.bits_per_sample, 16, "Expected 16-bit");
    assert_eq!(
        spec.sample_format,
        hound::SampleFormat::Int,
        "Expected integer samples"
    );

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_default_output_filename() {
    assert_eq!(DEFAULT_OUTPUT_FILENAME, "output.wav");
}
