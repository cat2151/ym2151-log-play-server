use ym2151_log_play_server::debug_wav::{generate_post_playback_buffers, is_debug_wav_enabled, save_debug_wav_files};
use ym2151_log_play_server::events::{EventLog, RegisterEvent};
use ym2151_log_play_server::logging;
use ym2151_log_play_server::resampler::ResamplingQuality;

#[test]
fn test_is_debug_wav_enabled_false() {
    // Verbose is off by default
    assert!(!is_debug_wav_enabled());
}

#[test]
fn test_is_debug_wav_enabled_true() {
    logging::init(true);
    assert!(is_debug_wav_enabled());
    logging::init(false);
}

#[test]
fn test_generate_post_playback_buffers() {
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

    let result = generate_post_playback_buffers(&log, ResamplingQuality::Linear);
    assert!(result.is_ok());

    let (buffer_55k, buffer_48k) = result.unwrap();
    assert!(!buffer_55k.is_empty());
    assert!(!buffer_48k.is_empty());
    assert_eq!(buffer_55k.len() % 2, 0); // Stereo
    assert_eq!(buffer_48k.len() % 2, 0); // Stereo
}

#[test]
fn test_generate_post_playback_buffers_high_quality() {
    // Test that high-quality resampling can be used in post-playback generation
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

    let result = generate_post_playback_buffers(&log, ResamplingQuality::HighQuality);
    assert!(result.is_ok());

    let (buffer_55k, buffer_48k) = result.unwrap();
    assert!(!buffer_55k.is_empty());
    assert!(!buffer_48k.is_empty());
    assert_eq!(buffer_55k.len() % 2, 0); // Stereo
    assert_eq!(buffer_48k.len() % 2, 0); // Stereo
}

#[test]
fn test_save_debug_wav_files() {
    let temp_dir = std::env::temp_dir();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&temp_dir).unwrap();

    let samples_55k = vec![0i16; 55930 * 2]; // 1 second
    let samples_48k = vec![0i16; 48000 * 2]; // 1 second

    let result = save_debug_wav_files(&samples_55k, &samples_48k, &samples_55k, &samples_48k);
    assert!(result.is_ok());

    // Clean up
    let _ = std::fs::remove_file(temp_dir.join("realtime_55k.wav"));
    let _ = std::fs::remove_file(temp_dir.join("realtime_48k.wav"));
    let _ = std::fs::remove_file(temp_dir.join("post_55k.wav"));
    let _ = std::fs::remove_file(temp_dir.join("post_48k.wav"));

    std::env::set_current_dir(original_dir).unwrap();
}
