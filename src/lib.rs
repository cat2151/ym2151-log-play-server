// YM2151 Log Player - Rust implementation
//
// This library provides a safe Rust interface to the Nuked-OPM emulator
// for playing back YM2151 register event logs.

pub mod events;
pub mod opm;
pub mod opm_ffi;
pub mod perf_monitor;
pub mod player;
pub mod resampler;
pub mod wav_writer;

#[cfg(feature = "realtime-audio")]
pub mod audio;
