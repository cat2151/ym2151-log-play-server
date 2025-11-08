use std::path::Path;
use ym2151_log_play_server::events::EventLog;
use ym2151_log_play_server::player::Player;
use ym2151_log_play_server::resampler::{AudioResampler, OPM_SAMPLE_RATE, OUTPUT_SAMPLE_RATE};
use ym2151_log_play_server::wav_writer::{generate_wav, write_wav, DEFAULT_OUTPUT_FILENAME};

#[test]
fn test_resampler_initialization() {
    let resampler = AudioResampler::new();
    assert!(resampler.is_ok(), "Failed to create resampler");

    let resampler = resampler.unwrap();
    assert_eq!(resampler.input_rate(), OPM_SAMPLE_RATE);
    assert_eq!(resampler.output_rate(), OUTPUT_SAMPLE_RATE);
}

#[test]
fn test_resampler_downsampling() {
    let mut resampler = AudioResampler::new().expect("Failed to create resampler");

    let input_frames = 1024;
    let input_samples = vec![0i16; input_frames * 2];

    let output = resampler
        .resample(&input_samples)
        .expect("Resampling failed");

    let output_frames = output.len() / 2;
    assert!(
        (700..=910).contains(&output_frames),
        "Expected ~750-880 frames, got {}",
        output_frames
    );
}

#[test]
fn test_resampler_sine_wave_preservation() {
    let mut resampler = AudioResampler::new().expect("Failed to create resampler");

    let freq = 440.0;
    let duration_frames = 1000;
    let mut input = Vec::with_capacity(duration_frames * 2);

    for i in 0..duration_frames {
        let t = i as f32 / OPM_SAMPLE_RATE as f32;
        let sample = (2.0 * std::f32::consts::PI * freq * t).sin();
        let i16_sample = (sample * 16384.0) as i16;
        input.push(i16_sample);
        input.push(i16_sample);
    }

    let output = resampler.resample(&input).expect("Resampling failed");

    assert!(!output.is_empty());
    let max_amplitude = output.iter().map(|&s| s.abs()).max().unwrap();
    assert!(max_amplitude > 10000, "Signal lost during resampling");
}

#[test]
fn test_write_wav_simple() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("phase4_test_simple.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let _ = std::fs::remove_file(temp_path_str);

    let samples = vec![0i16; 4800 * 2];
    let result = write_wav(temp_path_str, &samples, 48000);

    assert!(result.is_ok(), "Failed to write WAV: {:?}", result.err());
    assert!(
        Path::new(temp_path_str).exists(),
        "WAV file was not created"
    );

    let metadata = std::fs::metadata(temp_path_str).unwrap();
    assert!(metadata.len() > 100, "WAV file too small");

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_write_wav_with_audio() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("phase4_test_audio.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let _ = std::fs::remove_file(temp_path_str);

    let mut samples = Vec::with_capacity(48000 * 2);
    for i in 0..48000 {
        let sample = ((i % 1000) as i16 - 500) * 32;
        samples.push(sample);
        samples.push(sample);
    }

    let result = write_wav(temp_path_str, &samples, 48000);

    assert!(result.is_ok(), "Failed to write WAV: {:?}", result.err());
    assert!(
        Path::new(temp_path_str).exists(),
        "WAV file was not created"
    );

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_generate_wav_from_simple_events() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("phase4_test_generated.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let _ = std::fs::remove_file(temp_path_str);

    let log =
        EventLog::from_file("tests/fixtures/simple.json").expect("Failed to load simple.json");

    let player = Player::new(log);
    let result = generate_wav(player, temp_path_str);

    assert!(result.is_ok(), "Failed to generate WAV: {:?}", result.err());
    assert!(
        Path::new(temp_path_str).exists(),
        "WAV file was not created"
    );

    let metadata = std::fs::metadata(temp_path_str).unwrap();
    assert!(metadata.len() > 1000, "Generated WAV file too small");

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_generate_wav_from_sample_events() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("phase4_test_sample_events.wav");
    let temp_path_str = temp_path.to_str().unwrap();

    let _ = std::fs::remove_file(temp_path_str);

    let log = EventLog::from_file("output_ym2151.json").expect("Failed to load output_ym2151.json");

    let player = Player::new(log);
    let result = generate_wav(player, temp_path_str);

    assert!(result.is_ok(), "Failed to generate WAV: {:?}", result.err());
    assert!(
        Path::new(temp_path_str).exists(),
        "WAV file was not created"
    );

    let metadata = std::fs::metadata(temp_path_str).unwrap();
    assert!(metadata.len() > 10000, "Generated WAV file too small");

    let _ = std::fs::remove_file(temp_path_str);
}

#[test]
fn test_wav_format_verification() {
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("phase4_test_format.wav");
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
fn test_default_filename_constant() {
    assert_eq!(DEFAULT_OUTPUT_FILENAME, "output.wav");
}

#[test]
fn test_resampler_multiple_chunks() {
    let mut resampler = AudioResampler::new().expect("Failed to create resampler");

    let chunk_size = 1024;
    for iteration in 0..10 {
        let mut input = Vec::with_capacity(chunk_size * 2);
        for i in 0..chunk_size {
            let sample = ((i + iteration * chunk_size) % 32768) as i16;
            input.push(sample);
            input.push(sample);
        }

        let output = resampler.resample(&input).expect("Resampling failed");
        assert!(!output.is_empty(), "Chunk {} produced no output", iteration);
        assert_eq!(
            output.len() % 2,
            0,
            "Output not stereo at chunk {}",
            iteration
        );
    }
}

#[test]
fn test_expected_output_frames_accuracy() {
    let resampler = AudioResampler::new().expect("Failed to create resampler");

    let test_cases = vec![(1000, 858), (5593, 4800), (55930, 48000)];

    for (input_frames, expected_output) in test_cases {
        let predicted = resampler.expected_output_frames(input_frames);
        let tolerance = (expected_output as f64 * 0.02) as usize;

        assert!(
            predicted >= expected_output - tolerance && predicted <= expected_output + tolerance,
            "For {} input frames, expected ~{} output, got {}",
            input_frames,
            expected_output,
            predicted
        );
    }
}
