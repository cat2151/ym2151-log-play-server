use ym2151_log_play_server::debug_wav;
use ym2151_log_play_server::events::{EventLog, RegisterEvent};
use ym2151_log_play_server::logging;
use ym2151_log_play_server::resampler::ResamplingQuality;

#[test]
fn test_debug_wav_enabled_flag() {
    // Reset verbose to off first
    logging::init(false);
    assert!(!debug_wav::is_debug_wav_enabled());

    // Enable verbose
    logging::init(true);
    assert!(debug_wav::is_debug_wav_enabled());

    // Disable verbose
    logging::init(false);
    assert!(!debug_wav::is_debug_wav_enabled());
}

#[test]
fn test_post_playback_buffer_generation() {
    let log = EventLog {
        events: vec![
            RegisterEvent {
                time: 0.0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 100.0 / ym2151_log_play_server::resampler::OPM_SAMPLE_RATE as f64,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
            RegisterEvent {
                time: 200.0 / ym2151_log_play_server::resampler::OPM_SAMPLE_RATE as f64,
                addr: 0x28,
                data: 0x3E,
                is_data: None,
            },
        ],
    };

    let result = debug_wav::generate_post_playback_buffers(&log, ResamplingQuality::Linear);
    assert!(result.is_ok(), "Failed to generate post playback buffers");

    let (buffer_55k, buffer_48k) = result.unwrap();

    // Both buffers should have data
    assert!(!buffer_55k.is_empty(), "55kHz buffer is empty");
    assert!(!buffer_48k.is_empty(), "48kHz buffer is empty");

    // Both should be stereo (even number of samples)
    assert_eq!(buffer_55k.len() % 2, 0, "55kHz buffer is not stereo");
    assert_eq!(buffer_48k.len() % 2, 0, "48kHz buffer is not stereo");

    // The 48kHz buffer should be smaller than the 55kHz buffer due to resampling
    assert!(
        buffer_48k.len() < buffer_55k.len(),
        "48kHz buffer should be smaller than 55kHz buffer after resampling"
    );
}

#[test]
fn test_debug_wav_file_creation() {
    let temp_dir = std::env::temp_dir();
    let original_dir = std::env::current_dir().unwrap();

    // Change to temp directory
    std::env::set_current_dir(&temp_dir).unwrap();

    // Create simple test buffers
    let samples_55k = vec![0i16; 55930 * 2]; // 1 second stereo
    let samples_48k = vec![0i16; 48000 * 2]; // 1 second stereo

    let result =
        debug_wav::save_debug_wav_files(&samples_55k, &samples_48k, &samples_55k, &samples_48k);
    assert!(
        result.is_ok(),
        "Failed to save debug WAV files: {:?}",
        result.err()
    );

    // Verify all files were created
    assert!(
        temp_dir.join("realtime_55k.wav").exists(),
        "realtime_55k.wav not created"
    );
    assert!(
        temp_dir.join("realtime_48k.wav").exists(),
        "realtime_48k.wav not created"
    );
    assert!(
        temp_dir.join("post_55k.wav").exists(),
        "post_55k.wav not created"
    );
    assert!(
        temp_dir.join("post_48k.wav").exists(),
        "post_48k.wav not created"
    );

    // Clean up
    let _ = std::fs::remove_file(temp_dir.join("realtime_55k.wav"));
    let _ = std::fs::remove_file(temp_dir.join("realtime_48k.wav"));
    let _ = std::fs::remove_file(temp_dir.join("post_55k.wav"));
    let _ = std::fs::remove_file(temp_dir.join("post_48k.wav"));

    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_complete_debug_workflow() {
    // This test simulates the complete debug workflow
    let log = EventLog {
        events: vec![
            RegisterEvent {
                time: 0.0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 1000.0 / ym2151_log_play_server::resampler::OPM_SAMPLE_RATE as f64,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
        ],
    };

    // Step 1: Check if debug is enabled (default is off)
    assert!(!debug_wav::is_debug_wav_enabled());

    // Step 2: Enable debug via verbose mode
    logging::init(true);
    assert!(debug_wav::is_debug_wav_enabled());

    // Step 3: Generate post-playback buffers (simulating what happens after realtime playback)
    let result = debug_wav::generate_post_playback_buffers(&log, ResamplingQuality::Linear);
    assert!(result.is_ok());

    let (post_55k, post_48k) = result.unwrap();

    // Verify buffers have expected properties
    assert!(!post_55k.is_empty());
    assert!(!post_48k.is_empty());
    assert_eq!(post_55k.len() % 2, 0);
    assert_eq!(post_48k.len() % 2, 0);

    // Clean up
    logging::init(false);
}
