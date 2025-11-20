use crate::events::{EventLog, RegisterEvent};
use crate::opm::OpmChip;
use crate::resampler::OPM_SAMPLE_RATE;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const OPM_ADDRESS_REGISTER: u8 = 0;
const OPM_DATA_REGISTER: u8 = 1;

const DELAY_SAMPLES: u32 = 2;

const SILENCE_DURATION_MS: u32 = 100;
const SILENCE_SAMPLES: u32 = SILENCE_DURATION_MS * OPM_SAMPLE_RATE / 1000;

#[derive(Debug, Clone)]
pub struct ProcessedEvent {
    pub time: u32,

    pub addr: u8,

    pub data: u8,
}

pub struct Player {
    chip: OpmChip,

    // Static event playback (original mode)
    events: Vec<ProcessedEvent>,
    next_event_idx: usize,

    // Interactive mode support
    interactive_mode: bool,
    scheduled_events: Arc<Mutex<VecDeque<ProcessedEvent>>>,

    samples_played: u32,

    consecutive_silent_samples: u32,

    // Track last address register write for key on/off logging
    last_address_register: u8,

    // Track next available write time for 2-sample delay enforcement
    // This ensures proper spacing between all chip writes
    next_available_write_time: u32,

    // Track pending data write for addr-data pair processing
    // When Some, contains (data_value, scheduled_time) waiting to be written
    pending_data_write: Option<(u8, u32)>,
}

impl Player {
    pub fn new(log: EventLog) -> Self {
        let events = Self::convert_events(&log.events);
        Self {
            chip: OpmChip::new(),
            events,
            next_event_idx: 0,
            interactive_mode: false,
            scheduled_events: Arc::new(Mutex::new(VecDeque::new())),
            samples_played: 0,
            consecutive_silent_samples: 0,
            last_address_register: 0,
            next_available_write_time: 0,
            pending_data_write: None,
        }
    }

    /// Create a new Player in interactive mode
    pub fn new_interactive() -> Self {
        Self {
            chip: OpmChip::new(),
            events: Vec::new(),
            next_event_idx: 0,
            interactive_mode: true,
            scheduled_events: Arc::new(Mutex::new(VecDeque::new())),
            samples_played: 0,
            consecutive_silent_samples: 0,
            last_address_register: 0,
            next_available_write_time: 0,
            pending_data_write: None,
        }
    }

    /// Get a handle to the scheduled events queue for interactive mode
    pub fn get_event_queue(&self) -> Arc<Mutex<VecDeque<ProcessedEvent>>> {
        self.scheduled_events.clone()
    }

    /// Add a register write to the interactive event queue
    pub fn schedule_register_write(&self, scheduled_time_samples: u32, addr: u8, data: u8) {
        if !self.interactive_mode {
            return;
        }

        let mut queue = self.scheduled_events.lock().unwrap();

        // Store addr-data pair directly
        // The 2-sample delay between address and data writes will be applied
        // at the final stage in generate_samples()
        queue.push_back(ProcessedEvent {
            time: scheduled_time_samples,
            addr,
            data,
        });

        // Keep queue sorted by time
        // Check if we need to sort (new event might be out of order)
        let len = queue.len();
        if len >= 2 {
            // Check if the newly added event is out of order with existing events
            let needs_sort = queue[len - 1].time < queue[len - 2].time;

            if needs_sort {
                // Need to sort - convert to vec, sort, and rebuild
                let mut vec: Vec<_> = queue.drain(..).collect();
                vec.sort_by_key(|e| e.time);
                queue.extend(vec);
            }
        }
    }

    /// Check if running in interactive mode
    pub fn is_interactive(&self) -> bool {
        self.interactive_mode
    }

    /// Clear all scheduled events in interactive mode
    /// This allows seamless phrase transitions without audio gaps
    pub fn clear_schedule(&self) {
        if !self.interactive_mode {
            return;
        }

        let mut queue = self.scheduled_events.lock().unwrap();
        queue.clear();
    }

    fn convert_events(input: &[RegisterEvent]) -> Vec<ProcessedEvent> {
        let mut output = Vec::with_capacity(input.len());

        for event in input {
            // Store addr-data pairs directly
            // The 2-sample delay between address and data writes will be applied
            // at the final stage in generate_samples()
            output.push(ProcessedEvent {
                time: event.time,
                addr: event.addr,
                data: event.data,
            });
        }

        output
    }

    pub fn generate_samples(&mut self, buffer: &mut [i16]) -> bool {
        let num_samples = buffer.len() / 2;

        for i in 0..num_samples {
            // First, check if we have a pending data write from a previous addr write
            if let Some((data_value, scheduled_time)) = self.pending_data_write {
                if self.samples_played >= self.next_available_write_time {
                    // Time to write the data register
                    // Log key event if this is a key on/off
                    if self.last_address_register == 0x08 {
                        self.log_key_event_with_timing(data_value, scheduled_time);
                    }

                    self.chip.write(OPM_DATA_REGISTER, data_value);
                    self.next_available_write_time = self.samples_played + DELAY_SAMPLES;
                    self.pending_data_write = None;
                }
            }

            // Process events from the appropriate source
            if self.interactive_mode {
                // Interactive mode: process from VecDeque
                let mut queue = self.scheduled_events.lock().unwrap();
                while let Some(event) = queue.front() {
                    if event.time <= self.samples_played && self.pending_data_write.is_none() {
                        let event = queue.pop_front().unwrap();

                        // Apply 2-sample delay at final stage
                        // Ensure this write doesn't happen before next_available_write_time
                        if self.samples_played < self.next_available_write_time {
                            // Not enough time has passed - re-queue this event for later
                            let deferred_event = ProcessedEvent {
                                time: self.next_available_write_time,
                                addr: event.addr,
                                data: event.data,
                            };
                            
                            // Find the correct position to insert (maintain sorted order)
                            let insert_pos = queue.iter().position(|e| e.time > self.next_available_write_time).unwrap_or(queue.len());
                            queue.insert(insert_pos, deferred_event);
                            continue;
                        }

                        // Write address register first
                        self.last_address_register = event.addr;
                        self.chip.write(OPM_ADDRESS_REGISTER, event.addr);
                        self.next_available_write_time = self.samples_played + DELAY_SAMPLES;
                        
                        // Schedule data write for later (after 2-sample delay)
                        self.pending_data_write = Some((event.data, event.time));
                    } else {
                        break;
                    }
                }
            } else {
                // Static mode: process from Vec
                while self.next_event_idx < self.events.len() && self.pending_data_write.is_none() {
                    let event = &self.events[self.next_event_idx];

                    if event.time <= self.samples_played {
                        // Apply 2-sample delay at final stage
                        // Ensure this write doesn't happen before next_available_write_time
                        if self.samples_played < self.next_available_write_time {
                            // Not enough time has passed - break and wait
                            break;
                        }

                        // Write address register first
                        self.last_address_register = event.addr;
                        self.chip.write(OPM_ADDRESS_REGISTER, event.addr);
                        self.next_available_write_time = self.samples_played + DELAY_SAMPLES;
                        
                        // Schedule data write for later (after 2-sample delay)
                        self.pending_data_write = Some((event.data, event.time));
                        
                        self.next_event_idx += 1;
                    } else {
                        break;
                    }
                }
            }

            let sample_buffer = &mut buffer[i * 2..(i + 1) * 2];
            self.chip.generate_samples(sample_buffer);

            let left = sample_buffer[0];
            let right = sample_buffer[1];
            if Self::is_sample_silent(left, right) {
                self.consecutive_silent_samples += 1;
            } else {
                self.consecutive_silent_samples = 0;
            }

            self.samples_played += 1;
        }

        // In interactive mode, always return true (continuous streaming)
        // In static mode, return whether there are more events or pending writes
        if self.interactive_mode {
            true
        } else {
            self.next_event_idx < self.events.len() || self.pending_data_write.is_some()
        }
    }

    /// Log key on/off events for debugging with timing comparison
    fn log_key_event_with_timing(&self, key_data: u8, scheduled_time: u32) {
        use crate::logging;

        // YM2151 key on/off register (0x08) data format:
        // Bit 7-3: Key on/off flags for channels
        // Bit 2-0: Channel selection for key operations
        // Key off: bit3-7 are all 0 (data value 0-7)
        // Key on: any of bit3-7 is 1 (data value 8 or higher)

        let samples_sec = self.samples_played as f64 / crate::resampler::OPM_SAMPLE_RATE as f64;
        let samples_str = format!("{:.6}", samples_sec).trim_end_matches('0').trim_end_matches('.').to_string();

        let scheduled_sec = scheduled_time as f64 / crate::resampler::OPM_SAMPLE_RATE as f64;
        let scheduled_str = format!("{:.6}", scheduled_sec).trim_end_matches('0').trim_end_matches('.').to_string();

        let delay_samples = self.samples_played.saturating_sub(scheduled_time);
        let delay_sec = delay_samples as f64 / crate::resampler::OPM_SAMPLE_RATE as f64;
        let delay_str = format!("{:.6}", delay_sec).trim_end_matches('0').trim_end_matches('.').to_string();

        // Check if bit3-7 are all 0 (key off condition)
        if key_data & 0xF8 == 0 {
            logging::log_verbose(&format!(
                "🎹 Key OFF実行: 実行={}秒({}samples), 予定={}秒({}samples), 遅延={}秒({}samples) - data=0x{:02x}",
                samples_str, self.samples_played, scheduled_str, scheduled_time, delay_str, delay_samples, key_data
            ));
        } else {
            logging::log_verbose(&format!(
                "🎹 Key ON実行: 実行={}秒({}samples), 予定={}秒({}samples), 遅延={}秒({}samples) - data=0x{:02x}",
                samples_str, self.samples_played, scheduled_str, scheduled_time, delay_str, delay_samples, key_data
            ));
        }
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
        // Interactive mode never completes
        if self.interactive_mode {
            return false;
        }
        self.next_event_idx >= self.events.len() && self.pending_data_write.is_none()
    }

    pub const fn sample_rate() -> u32 {
        OPM_SAMPLE_RATE
    }

    fn is_sample_silent(left: i16, right: i16) -> bool {
        left == 0 && right == 0
    }

    pub fn should_continue_tail(&self) -> bool {
        if !self.is_complete() {
            return true;
        }

        self.consecutive_silent_samples < SILENCE_SAMPLES
    }

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

        assert_eq!(processed.len(), 1);

        // Single event with addr-data pair
        assert_eq!(processed[0].time, 100);
        assert_eq!(processed[0].addr, 0x08);
        assert_eq!(processed[0].data, 0x00);
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

        assert_eq!(processed.len(), 3);

        // All events at their original times (no delay during conversion)
        assert_eq!(processed[0].time, 0);
        assert_eq!(processed[0].addr, 0x08);
        assert_eq!(processed[0].data, 0x00);

        assert_eq!(processed[1].time, 10);
        assert_eq!(processed[1].addr, 0x20);
        assert_eq!(processed[1].data, 0xC7);

        assert_eq!(processed[2].time, 20);
        assert_eq!(processed[2].addr, 0x28);
        assert_eq!(processed[2].data, 0x3E);
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

        // Single event with addr-data pair (delay applied in generate_samples)
        assert_eq!(processed.len(), 1);
        assert_eq!(processed[0].time, 0);
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

        assert_eq!(processed.len(), 3);

        // All events at time 0 as addr-data pairs (delay applied in generate_samples)
        assert_eq!(processed[0].time, 0);
        assert_eq!(processed[0].addr, 0x08);
        assert_eq!(processed[0].data, 0x00);

        assert_eq!(processed[1].time, 0);
        assert_eq!(processed[1].addr, 0x20);
        assert_eq!(processed[1].data, 0xC7);

        assert_eq!(processed[2].time, 0);
        assert_eq!(processed[2].addr, 0x28);
        assert_eq!(processed[2].data, 0x3E);
    }

    #[test]
    fn test_player_creation() {
        let log = EventLog {
            events: vec![RegisterEvent {
                time: 0,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            }],
        };

        let player = Player::new(log);

        assert_eq!(player.total_events(), 1);
        assert_eq!(player.events_processed(), 0);
        assert!(!player.is_complete());
    }

    #[test]
    fn test_generate_samples_basic() {
        let log = EventLog {
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

        assert_eq!(player.total_events(), 2);

        let mut buffer = vec![0i16; 200];
        player.generate_samples(&mut buffer);

        assert_eq!(player.events_processed(), 1);
        assert_eq!(player.current_sample(), 100);

        let mut buffer = vec![0i16; 2000];
        player.generate_samples(&mut buffer);

        assert_eq!(player.events_processed(), 2);
        assert!(player.is_complete());
    }

    #[test]
    fn test_total_samples() {
        let log = EventLog {
            events: vec![RegisterEvent {
                time: 1000,
                addr: 0x08,
                data: 0x00,
                is_data: None,
            }],
        };

        let player = Player::new(log);

        // Events are at their original time (delay applied in generate_samples)
        let expected = 1000;
        assert_eq!(player.total_samples(), expected);
    }

    #[test]
    fn test_empty_event_log() {
        let log = EventLog {
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

    #[test]
    fn test_interactive_mode_creation() {
        let player = Player::new_interactive();
        assert!(player.is_interactive());
        assert_eq!(player.total_events(), 0);
        assert!(!player.is_complete()); // Interactive mode never completes
    }

    #[test]
    fn test_schedule_register_write() {
        let player = Player::new_interactive();

        // Schedule a register write
        player.schedule_register_write(100, 0x08, 0x78);

        // Check that event was added to the queue
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();
        assert_eq!(q.len(), 1); // One addr-data pair event

        // Check addr-data pair
        assert_eq!(q[0].time, 100);
        assert_eq!(q[0].addr, 0x08);
        assert_eq!(q[0].data, 0x78);
    }

    #[test]
    fn test_clear_schedule() {
        let player = Player::new_interactive();

        // Schedule some events
        player.schedule_register_write(100, 0x08, 0x78);
        player.schedule_register_write(200, 0x20, 0xC7);

        // Verify events were added
        {
            let queue = player.get_event_queue();
            let q = queue.lock().unwrap();
            assert_eq!(q.len(), 2); // 2 register writes = 2 addr-data pair events
        }

        // Clear the schedule
        player.clear_schedule();

        // Verify queue is empty
        {
            let queue = player.get_event_queue();
            let q = queue.lock().unwrap();
            assert_eq!(q.len(), 0);
        }
    }

    #[test]
    fn test_clear_schedule_non_interactive_mode() {
        let log = EventLog {
            events: vec![],
        };
        let player = Player::new(log);

        // clear_schedule should do nothing in non-interactive mode
        player.clear_schedule(); // Should not panic
        assert!(!player.is_interactive());
    }

    #[test]
    fn test_schedule_events_are_sorted() {
        let player = Player::new_interactive();

        // Schedule events out of order
        player.schedule_register_write(200, 0x20, 0xC7);
        player.schedule_register_write(100, 0x08, 0x78);
        player.schedule_register_write(150, 0x28, 0x3E);

        // Check that events are sorted by time
        let queue = player.get_event_queue();
        let q = queue.lock().unwrap();

        // Should have 3 events (3 register writes as addr-data pairs)
        assert_eq!(q.len(), 3);

        // Verify they are in time order
        for i in 1..q.len() {
            assert!(q[i].time >= q[i - 1].time);
        }
    }
}
