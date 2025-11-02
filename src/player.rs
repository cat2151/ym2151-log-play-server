// Event processing engine for YM2151 register operations
//
// This module implements the core playback logic that converts pass1 events
// (simple register writes) into pass2 events (address/data split with delays)
// and executes them with precise timing.

use crate::events::{EventLog, RegisterEvent};
use crate::opm::OpmChip;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// OPM register port constants
const OPM_ADDRESS_REGISTER: u8 = 0;
const OPM_DATA_REGISTER: u8 = 1;

/// Delay in samples between address write and data write
const DELAY_SAMPLES: u32 = 2;

/// Native sample rate of the OPM chip (55.93 kHz)
const OPM_SAMPLE_RATE: u32 = 55930;

/// Represents a processed event with port and timing information.
///
/// This is a pass2 event - after the pass1 to pass2 conversion,
/// each original register write becomes two events: one for the
/// address port and one for the data port.
#[derive(Debug, Clone)]
struct ProcessedEvent {
    /// Sample time when this event should occur
    time: u32,
    /// Register port (0 = address, 1 = data)
    port: u8,
    /// Value to write to the port
    value: u8,
}

/// Pass2 event format for JSON export.
///
/// This structure represents pass2 events in the format expected for debugging:
/// - addr field: YM2151 register address
/// - data field: Data value to write
/// - is_data field indicates whether this is address write (0) or data write (1)
/// Note: In pass2 format, both address and data writes carry the same addr/data values
///       but differ only in the is_data flag and timing
#[derive(Debug, Clone, Serialize)]
pub struct Pass2Event {
    /// Sample time when this event should occur
    pub time: u32,
    /// YM2151 register address (formatted as hex string)
    #[serde(serialize_with = "serialize_hex_u8")]
    pub addr: u8,
    /// Data value (formatted as hex string)
    #[serde(serialize_with = "serialize_hex_u8")]
    pub data: u8,
    /// Flag: 0 = address write, 1 = data write
    pub is_data: u8,
}

/// Pass2 event log for JSON export.
#[derive(Debug, Serialize)]
pub struct Pass2EventLog {
    /// Total number of events in the log
    pub event_count: usize,
    /// List of pass2 events
    pub events: Vec<Pass2Event>,
}

/// Serialize u8 as hex string (e.g., 0x08)
fn serialize_hex_u8<S>(value: &u8, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("0x{:02X}", value))
}

/// The event player that manages playback of YM2151 register events.
///
/// The player converts pass1 events (simple register writes) into pass2 events
/// (split address/data writes with delays), maintains timing state, and
/// generates audio samples while executing events at the correct times.
pub struct Player {
    /// The OPM chip emulator
    chip: OpmChip,
    /// Processed events (pass2 format) sorted by time
    events: Vec<ProcessedEvent>,
    /// Index of the next event to execute
    next_event_idx: usize,
    /// Total number of samples generated so far
    samples_played: u32,
}

impl Player {
    /// Create a new player from an event log.
    ///
    /// This converts the pass1 events in the log to pass2 events with
    /// proper address/data splitting and delay insertion.
    ///
    /// # Parameters
    /// - `log`: Event log loaded from JSON
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::events::EventLog;
    /// use ym2151_log_player_rust::player::Player;
    ///
    /// let log = EventLog::from_file("sample_events.json").unwrap();
    /// let player = Player::new(log);
    /// ```
    pub fn new(log: EventLog) -> Self {
        let events = Self::convert_events(&log.events);
        Self {
            chip: OpmChip::new(),
            events,
            next_event_idx: 0,
            samples_played: 0,
        }
    }

    /// Convert pass1 events to pass2 events.
    ///
    /// Each pass1 event (register write) is converted into two pass2 events:
    /// 1. Address write with accumulated delay for events at same time
    /// 2. Data write after DELAY_SAMPLES from the address write
    ///
    /// For multiple events at the same timestamp, delays accumulate to prevent
    /// overlapping register writes, matching the behavior of the original C implementation.
    ///
    /// This function internally uses `convert_to_pass2_format()` to avoid code duplication.
    ///
    /// # Parameters
    /// - `input`: Slice of pass1 register events
    ///
    /// # Returns
    /// Vector of processed events sorted by time
    fn convert_events(input: &[RegisterEvent]) -> Vec<ProcessedEvent> {
        // Use the shared conversion logic
        let pass2_events = Self::convert_to_pass2_format(input);

        // Convert Pass2Event to ProcessedEvent
        pass2_events
            .into_iter()
            .map(|event| ProcessedEvent {
                time: event.time,
                port: if event.is_data == 0 {
                    OPM_ADDRESS_REGISTER
                } else {
                    OPM_DATA_REGISTER
                },
                value: if event.is_data == 0 {
                    event.addr
                } else {
                    event.data
                },
            })
            .collect()
    }

    /// Convert pass1 events to pass2 format for JSON export.
    ///
    /// This function performs the same conversion as `convert_events`, but returns
    /// the result in a format suitable for JSON serialization with the `is_data` field.
    ///
    /// For multiple events at the same timestamp, delays accumulate to prevent
    /// overlapping register writes, matching the behavior of the original C implementation.
    ///
    /// # Parameters
    /// - `input`: Slice of pass1 register events
    ///
    /// # Returns
    /// Vector of pass2 events ready for JSON export
    pub fn convert_to_pass2_format(input: &[RegisterEvent]) -> Vec<Pass2Event> {
        let mut output = Vec::with_capacity(input.len() * 2);
        let mut accumulated_delay = 0u32;
        let mut last_time = 0u32;

        for event in input {
            // If this event is at a different time, reset accumulated delay
            if event.time != last_time {
                accumulated_delay = 0;
                last_time = event.time;
            }

            // Address write at original time + accumulated delay (is_data = 0)
            output.push(Pass2Event {
                time: event.time + accumulated_delay,
                addr: event.addr,
                data: event.data,
                is_data: 0,
            });
            accumulated_delay += DELAY_SAMPLES;

            // Data write after delay (is_data = 1)
            output.push(Pass2Event {
                time: event.time + accumulated_delay,
                addr: event.addr,
                data: event.data,
                is_data: 1,
            });
            accumulated_delay += DELAY_SAMPLES;
        }

        output
    }

    /// Export pass2 events to JSON file.
    ///
    /// This function writes the pass2 events to a JSON file for debugging purposes.
    /// The format matches the expected pass2 format with `is_data` field.
    ///
    /// # Parameters
    /// - `pass2_events`: Slice of pass2 events to export
    /// - `output_path`: Path to the output JSON file
    ///
    /// # Returns
    /// Result indicating success or error
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::events::EventLog;
    /// use ym2151_log_player_rust::player::Player;
    ///
    /// let log = EventLog::from_file("sample_events.json").unwrap();
    /// let pass2_events = Player::convert_to_pass2_format(&log.events);
    /// Player::export_pass2_json(&pass2_events, "output_pass2.json").unwrap();
    /// ```
    pub fn export_pass2_json<P: AsRef<Path>>(
        pass2_events: &[Pass2Event],
        output_path: P,
    ) -> anyhow::Result<()> {
        let event_count = pass2_events.len();
        let log = Pass2EventLog {
            event_count,
            events: pass2_events.to_vec(),
        };

        let json = serde_json::to_string_pretty(&log)?;
        let mut file = File::create(output_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Generate audio samples and execute events.
    ///
    /// This function generates the requested number of stereo samples and
    /// executes any events that should occur during this time period.
    ///
    /// # Parameters
    /// - `buffer`: Output buffer for interleaved stereo i16 samples (length must be even)
    ///
    /// # Returns
    /// `true` if there are more events to process, `false` if playback is complete
    ///
    /// # Examples
    /// ```no_run
    /// use ym2151_log_player_rust::events::EventLog;
    /// use ym2151_log_player_rust::player::Player;
    ///
    /// let log = EventLog::from_file("sample_events.json").unwrap();
    /// let mut player = Player::new(log);
    ///
    /// let mut buffer = vec![0i16; 1024]; // 512 stereo samples
    /// while player.generate_samples(&mut buffer) {
    ///     // Process buffer (e.g., write to file or audio output)
    /// }
    /// ```
    pub fn generate_samples(&mut self, buffer: &mut [i16]) -> bool {
        let num_samples = buffer.len() / 2; // Stereo samples

        // Generate each sample individually, processing events at precise times
        // This matches the behavior of the original C implementation
        for i in 0..num_samples {
            // Process all events that should occur at or before the current sample time
            // Using <= is correct: events scheduled for sample N must be processed
            // BEFORE generating sample N, to ensure the chip state is updated properly
            while self.next_event_idx < self.events.len() {
                let event = &self.events[self.next_event_idx];

                if event.time <= self.samples_played {
                    // This event should be executed before or at the current sample
                    self.chip.write(event.port, event.value);
                    self.next_event_idx += 1;
                } else {
                    // This event is in the future
                    break;
                }
            }

            // Generate one stereo sample
            let sample_buffer = &mut buffer[i * 2..(i + 1) * 2];
            self.chip.generate_samples(sample_buffer);

            // Update playback position after generating this sample
            self.samples_played += 1;
        }

        // Return true if there are more events to process
        self.next_event_idx < self.events.len()
    }

    /// Get the total number of samples needed for complete playback.
    ///
    /// This includes all events plus an additional second of audio after
    /// the last event to allow notes to decay naturally.
    ///
    /// # Returns
    /// Total number of mono samples needed
    pub fn total_samples(&self) -> u32 {
        self.events
            .last()
            .map(|e| e.time + OPM_SAMPLE_RATE) // Add 1 second after last event
            .unwrap_or(0)
    }

    /// Get the current playback position in samples.
    pub fn current_sample(&self) -> u32 {
        self.samples_played
    }

    /// Get the number of events processed so far.
    pub fn events_processed(&self) -> usize {
        self.next_event_idx
    }

    /// Get the total number of events.
    pub fn total_events(&self) -> usize {
        self.events.len()
    }

    /// Check if playback is complete.
    pub fn is_complete(&self) -> bool {
        self.next_event_idx >= self.events.len()
    }

    /// Get the OPM sample rate.
    pub const fn sample_rate() -> u32 {
        OPM_SAMPLE_RATE
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

        // Should create 2 events: address write and data write
        assert_eq!(processed.len(), 2);

        // First event: address write at time 100
        assert_eq!(processed[0].time, 100);
        assert_eq!(processed[0].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[0].value, 0x08);

        // Second event: data write at time 102 (100 + DELAY_SAMPLES)
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

        // Should create 6 events (2 per original event)
        assert_eq!(processed.len(), 6);

        // Verify timing and order
        assert_eq!(processed[0].time, 0); // addr write
        assert_eq!(processed[1].time, 2); // data write (0 + 2)
        assert_eq!(processed[2].time, 10); // addr write
        assert_eq!(processed[3].time, 12); // data write (10 + 2)
        assert_eq!(processed[4].time, 20); // addr write
        assert_eq!(processed[5].time, 22); // data write (20 + 2)
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

        // Verify DELAY_SAMPLES is applied correctly
        assert_eq!(processed[1].time - processed[0].time, DELAY_SAMPLES);
    }

    #[test]
    fn test_convert_events_same_time_accumulation() {
        // Test that multiple events at the same time accumulate delays
        // This matches the behavior of the original C implementation
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

        // Should create 6 events (2 per original event)
        assert_eq!(processed.len(), 6);

        // Verify timing with accumulated delays (matching C implementation):
        // Event 1: addr at 0, data at 2
        // Event 2: addr at 4, data at 6 (accumulated)
        // Event 3: addr at 8, data at 10 (accumulated)
        assert_eq!(processed[0].time, 0); // event 1 addr write
        assert_eq!(processed[0].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[0].value, 0x08);

        assert_eq!(processed[1].time, 2); // event 1 data write
        assert_eq!(processed[1].port, OPM_DATA_REGISTER);
        assert_eq!(processed[1].value, 0x00);

        assert_eq!(processed[2].time, 4); // event 2 addr write (accumulated)
        assert_eq!(processed[2].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[2].value, 0x20);

        assert_eq!(processed[3].time, 6); // event 2 data write (accumulated)
        assert_eq!(processed[3].port, OPM_DATA_REGISTER);
        assert_eq!(processed[3].value, 0xC7);

        assert_eq!(processed[4].time, 8); // event 3 addr write (accumulated)
        assert_eq!(processed[4].port, OPM_ADDRESS_REGISTER);
        assert_eq!(processed[4].value, 0x28);

        assert_eq!(processed[5].time, 10); // event 3 data write (accumulated)
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

        assert_eq!(player.total_events(), 2); // 1 pass1 event -> 2 pass2 events
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

        // Generate samples - should process events
        let _has_more = player.generate_samples(&mut buffer);

        // Should have processed some events
        assert!(player.events_processed() > 0);

        // Buffer should be filled with samples (not all zeros)
        // Note: actual values depend on chip state
    }

    #[test]
    fn test_generate_samples_timing() {
        // Create events at specific times
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

        // Events at times: 0, 2, 1000, 1002
        assert_eq!(player.total_events(), 4);

        // Generate first 100 samples
        let mut buffer = vec![0i16; 200]; // 100 stereo samples
        player.generate_samples(&mut buffer);

        // Should have processed the first 2 events (at times 0 and 2)
        assert_eq!(player.events_processed(), 2);
        assert_eq!(player.current_sample(), 100);

        // Generate more samples to reach time 1000
        let mut buffer = vec![0i16; 2000]; // 1000 stereo samples
        player.generate_samples(&mut buffer);

        // Should have processed all 4 events
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

        // Last pass2 event is at time 1002 (1000 + DELAY_SAMPLES)
        // Total samples should be 1002 + OPM_SAMPLE_RATE (1 second buffer)
        let expected = 1002 + OPM_SAMPLE_RATE;
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

        // Generate samples until complete
        let mut iterations = 0;
        while !player.is_complete() && iterations < 100 {
            player.generate_samples(&mut buffer);
            iterations += 1;
        }

        // Should have completed
        assert!(player.is_complete());
        assert_eq!(player.events_processed(), player.total_events());
    }

    #[test]
    fn test_sample_rate() {
        assert_eq!(Player::sample_rate(), 55930);
    }
}
