use crate::audio::AudioPlayer;
use crate::events::{EventLog, RegisterEvent};
use crate::player::Player;

#[test]
fn test_audio_player_creation() {
    let log = EventLog {
        event_count: 1,
        events: vec![RegisterEvent {
            time: 0,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }],
    };

    let player = Player::new(log);

    let result = AudioPlayer::new(player);

    match result {
        Ok(mut audio_player) => {
            std::thread::sleep(std::time::Duration::from_millis(100));
            audio_player.stop();
        }
        Err(e) => {
            println!("Note: Audio player creation failed (expected in CI): {}", e);
        }
    }
}

#[test]
fn test_audio_player_short_playback() {
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

    match AudioPlayer::new(player) {
        Ok(mut audio_player) => {
            std::thread::sleep(std::time::Duration::from_millis(200));
            audio_player.stop();
        }
        Err(e) => {
            println!("Note: Audio player creation failed (expected in CI): {}", e);
        }
    }
}
