use crate::events::{EventLog, RegisterEvent};
use crate::opm::OpmChip;
use crate::resampler::OPM_SAMPLE_RATE;

const OPM_ADDRESS_REGISTER: u8 = 0;
const OPM_DATA_REGISTER: u8 = 1;

const DELAY_SAMPLES: u32 = 2;

/// Minimum duration to generate after events complete (500ms in samples at OPM_SAMPLE_RATE)
const TAIL_MIN_DURATION_MS: u32 = 500;
const TAIL_MIN_SAMPLES: u32 =
    (OPM_SAMPLE_RATE as f64 * TAIL_MIN_DURATION_MS as f64 / 1000.0) as u32;

/// Threshold for silence detection (absolute value of sample)
const SILENCE_THRESHOLD: i16 = 10;

/// Number of consecutive silent samples required to stop tail generation
const SILENCE_DURATION_MS: u32 = 100;
const SILENCE_SAMPLES: u32 = (OPM_SAMPLE_RATE as f64 * SILENCE_DURATION_MS as f64 / 1000.0) as u32;

#[derive(Debug, Clone)]
struct ProcessedEvent {
    time: u32,

    port: u8,

    value: u8,
}

pub struct Player {
    chip: OpmChip,

    events: Vec<ProcessedEvent>,

    next_event_idx: usize,

    samples_played: u32,

    /// Count of consecutive silent samples
    consecutive_silent_samples: u32,
}

impl Player {
    pub fn new(log: EventLog) -> Self {
        let events = Self::convert_events(&log.events);
        Self {
            chip: OpmChip::new(),
            events,
            next_event_idx: 0,
            samples_played: 0,
            consecutive_silent_samples: 0,
        }
    }

    fn convert_events(input: &[RegisterEvent]) -> Vec<ProcessedEvent> {
        let mut output = Vec::with_capacity(input.len() * 2);
        let mut accumulated_delay = 0u32;
        let mut last_time = 0u32;

        for event in input {
            if event.time != last_time {
                accumulated_delay = 0;
                last_time = event.time;
            }

            output.push(ProcessedEvent {
                time: event.time + accumulated_delay,
                port: OPM_ADDRESS_REGISTER,
                value: event.addr,
            });
            accumulated_delay += DELAY_SAMPLES;

            output.push(ProcessedEvent {
                time: event.time + accumulated_delay,
                port: OPM_DATA_REGISTER,
                value: event.data,
            });
            accumulated_delay += DELAY_SAMPLES;
        }

        output
    }

    pub fn generate_samples(&mut self, buffer: &mut [i16]) -> bool {
        let num_samples = buffer.len() / 2;

        for i in 0..num_samples {
            while self.next_event_idx < self.events.len() {
                let event = &self.events[self.next_event_idx];

                if event.time <= self.samples_played {
                    self.chip.write(event.port, event.value);
                    self.next_event_idx += 1;
                } else {
                    break;
                }
            }

            let sample_buffer = &mut buffer[i * 2..(i + 1) * 2];
            self.chip.generate_samples(sample_buffer);

            // Track silence for tail generation
            let left = sample_buffer[0];
            let right = sample_buffer[1];
            if Self::is_sample_silent(left, right) {
                self.consecutive_silent_samples += 1;
            } else {
                self.consecutive_silent_samples = 0;
            }

            self.samples_played += 1;
        }

        self.next_event_idx < self.events.len()
    }

    pub fn total_samples(&self) -> u32 {
        self.events.last().map(|e| e.time).unwrap_or(0)
    }

    pub fn current_sample(&self) -> u32 {
        self.samples_played
    }

    pub fn events_processed(&self) -> usize {
        self.next_event_idx
    }

    pub fn total_events(&self) -> usize {
        self.events.len()
    }

    pub fn is_complete(&self) -> bool {
        self.next_event_idx >= self.events.len()
    }

    pub const fn sample_rate() -> u32 {
        OPM_SAMPLE_RATE
    }

    /// Check if a stereo sample pair is below the silence threshold
    fn is_sample_silent(left: i16, right: i16) -> bool {
        left.abs() < SILENCE_THRESHOLD && right.abs() < SILENCE_THRESHOLD
    }

    /// Check if tail generation should continue
    /// Returns true if we should keep generating samples after events complete
    pub fn should_continue_tail(&self) -> bool {
        if !self.is_complete() {
            // Still processing events
            return true;
        }

        let samples_after_events = self.samples_played - self.total_samples();

        // Always generate at least TAIL_MIN_SAMPLES after events complete
        if samples_after_events < TAIL_MIN_SAMPLES {
            return true;
        }

        // After minimum tail duration, check for silence
        self.consecutive_silent_samples < SILENCE_SAMPLES
    }

    /// Get information about tail generation progress
    pub fn tail_info(&self) -> Option<(u32, u32)> {
        if self.is_complete() {
            let samples_after_events = self.samples_played.saturating_sub(self.total_samples());
            if samples_after_events > 0 {
                return Some((samples_after_events, self.consecutive_silent_samples));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_events_empty() {
        let events = vec![];
        let processed = Player::convert_events(&events);
        assert_eq!(processed.len(), 0);
    }

    #[test]
    fn test_convert_events_single() {
        let events = vec![RegisterEvent {
            time: 100,
            addr: 0x08,
            data: 0x00,
            is_data: None,
        }];

        let processed = Player::convert_events(&events);

        assert_eq!(processed.len(), 2);

        assert_eq!(processed[0].time, 100);
        assert_eq!(processed[0].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[0].value, 0x08);

        assert_eq!(processed[1].time, 102);
        assert_eq!(processed[1].port, OPM_DATA_REGISTER);
        assert_eq!(processed[1].value, 0x00);
    }

    #[test]
    fn test_convert_events_multiple() {
        let events = vec![
            RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 10,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
            RegisterEvent {
                time: 20,
                addr: 0x28,
                data: 0x3E,
                is_data: None,
            },
        ];

        let processed = Player::convert_events(&events);

        assert_eq!(processed.len(), 6);

        assert_eq!(processed[0].time, 0);
        assert_eq!(processed[1].time, 2);
        assert_eq!(processed[2].time, 10);
        assert_eq!(processed[3].time, 12);
        assert_eq!(processed[4].time, 20);
        assert_eq!(processed[5].time, 22);
    }

    #[test]
    fn test_convert_events_delay() {
        let events = vec![RegisterEvent {
            time: 0,
            addr: 0xFF,
            data: 0xAA,
            is_data: None,
        }];

        let processed = Player::convert_events(&events);

        assert_eq!(processed[1].time - processed[0].time, DELAY_SAMPLES);
    }

    #[test]
    fn test_convert_events_same_time_accumulation() {
        let events = vec![
            RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            },
            RegisterEvent {
                time: 0,
                addr: 0x20,
                data: 0xC7,
                is_data: None,
            },
            RegisterEvent {
                time: 0,
                addr: 0x28,
                data: 0x3E,
                is_data: None,
            },
        ];

        let processed = Player::convert_events(&events);

        assert_eq!(processed.len(), 6);

        assert_eq!(processed[0].time, 0);
        assert_eq!(processed[0].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[0].value, 0x08);

        assert_eq!(processed[1].time, 2);
        assert_eq!(processed[1].port, OPM_DATA_REGISTER);
        assert_eq!(processed[1].value, 0x00);

        assert_eq!(processed[2].time, 4);
        assert_eq!(processed[2].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[2].value, 0x20);

        assert_eq!(processed[3].time, 6);
        assert_eq!(processed[3].port, OPM_DATA_REGISTER);
        assert_eq!(processed[3].value, 0xC7);

        assert_eq!(processed[4].time, 8);
        assert_eq!(processed[4].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[4].value, 0x28);

        assert_eq!(processed[5].time, 10);
        assert_eq!(processed[5].port, OPM_DATA_REGISTER);
        assert_eq!(processed[5].value, 0x3E);
    }

    #[test]
    fn test_player_creation() {
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

        assert_eq!(player.total_events(), 2);
        assert_eq!(player.events_processed(), 0);
        assert!(!player.is_complete());
    }

    #[test]
    fn test_generate_samples_basic() {
        let log = EventLog {
            event_count: 1,
            events: vec![RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            }],
        };

        let mut player = Player::new(log);
        let mut buffer = vec![0i16; 1024];

        let _has_more = player.generate_samples(&mut buffer);

        assert!(player.events_processed() > 0);
    }

    #[test]
    fn test_generate_samples_timing() {
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
                    time: 1000,
                    addr: 0x20,
                    data: 0xC7,
                    is_data: None,
                },
            ],
        };

        let mut player = Player::new(log);

        assert_eq!(player.total_events(), 4);

        let mut buffer = vec![0i16; 200];
        player.generate_samples(&mut buffer);

        assert_eq!(player.events_processed(), 2);
        assert_eq!(player.current_sample(), 100);

        let mut buffer = vec![0i16; 2000];
        player.generate_samples(&mut buffer);

        assert_eq!(player.events_processed(), 4);
        assert!(player.is_complete());
    }

    #[test]
    fn test_total_samples() {
        let log = EventLog {
            event_count: 1,
            events: vec![RegisterEvent {
                time: 1000,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            }],
        };

        let player = Player::new(log);

        let expected = 1002;
        assert_eq!(player.total_samples(), expected);
    }

    #[test]
    fn test_empty_event_log() {
        let log = EventLog {
            event_count: 0,
            events: vec![],
        };

        let player = Player::new(log);

        assert_eq!(player.total_events(), 0);
        assert_eq!(player.total_samples(), 0);
        assert!(player.is_complete());
    }

    #[test]
    fn test_playback_completion() {
        let log = EventLog {
            event_count: 1,
            events: vec![RegisterEvent {
                time: 10,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            }],
        };

        let mut player = Player::new(log);
        let mut buffer = vec![0i16; 1024];

        let mut iterations = 0;
        while !player.is_complete() && iterations < 100 {
            player.generate_samples(&mut buffer);
            iterations += 1;
        }

        assert!(player.is_complete());
        assert_eq!(player.events_processed(), player.total_events());
    }

    #[test]
    fn test_sample_rate() {
        assert_eq!(Player::sample_rate(), 55930);
    }
}
