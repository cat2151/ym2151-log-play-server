use ym2151_log_play_server::events::{EventLog, RegisterEvent};
use ym2151_log_play_server::player::Player;
use ym2151_log_play_server::wav_writer::{generate_wav, write_wav, DEFAULT_OUTPUT_FILENAME};
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
fn test_default_output_filename() {
    assert_eq!(DEFAULT_OUTPUT_FILENAME, "output.wav");
}
