# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://deepwiki.com/cat2151/ym2151-log-play-server"><img src="https://deepwiki.com/badge.svg" alt="Ask DeepWiki"></a>
</p>

A server and client that receive YM2151 (OPM) register event logs and perform real-time playback. Written in Rust.

## Supported Platforms

- Windows only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed, therefore,
        - Linux-specific code is prohibited.

## Development Status

It is used as a library, integrated into `cat-play-mml` and `ym2151-tone-editor`.

Frequent breaking changes occur, especially regarding the client-server protocol and server operation modes.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (in verbose mode)
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client to quickly switch to different performances
- Utilizes named pipes for server-client communication

## Usage

### Usage as a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server is ready (automatically installs and starts if necessary)
    client::ensure_server_ready("cat-play-mml")?;
    
    // Send JSON data
    let json_data = r#"{"event_count": 2, "events": [...]}"#;
    client::send_json(json_data)?;
    
    // Playback control
    client::stop_playback()?;
    
    // Shutdown on exit
    client::shutdown_server()?;
    
    Ok(())
}
```

The `ensure_server_ready()` function automatically performs the following, providing a seamless development experience:
1. Checks if the server is already running
2. Installs the server application via cargo if not found in PATH
3. Starts the server in background mode
4. Waits until the server is ready to accept commands

This eliminates the need for library users to manually manage the server's lifecycle.

## Client Implementation Guide

This section describes two primary client implementation patterns.

### Pattern 1: Non-Interactive Mode

Non-interactive mode is a simple mode suitable for one-shot JSON data transmission.
Playback stops and restarts with each JSON transmission.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server is ready (automatically installs and starts if necessary)
    client::ensure_server_ready("your-app-name")?;
    
    // Send JSON data (start playback)
    let json_data = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x00"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::send_json(json_data)?;
    
    // Control playback as needed
    std::thread::sleep(std::time::Duration::from_secs(5));
    client::stop_playback()?;
    
    // Play another JSON
    let json_data2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::send_json(json_data2)?;
    
    // Shutdown on exit
    client::shutdown_server()?;
    
    Ok(())
}
```

#### Characteristics
- **Simple**: Each JSON is processed independently
- **Playback switching**: The previous performance automatically stops with each JSON transmission
- **Intervals**: Short periods of silence may occur between JSONs
- **Use cases**: Switching tracks, applications where continuity is not critical, WAV saving (verbose mode)

### Pattern 2: Interactive Mode

Interactive mode is an advanced mode suitable for real-time audio control.
It allows dynamic scheduling of register events while maintaining a continuous audio stream.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Server preparation
    client::ensure_server_ready("your-app-name")?;
    
    // Start interactive mode (begin continuous audio stream)
    client::start_interactive()?;
    
    // Send multiple JSONs without audio gaps
    let phrase1 = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x78"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::play_json_interactive(phrase1)?;
    
    // Switch to another phrase mid-phrase (no audio gap)
    client::clear_schedule()?; // Cancel future events
    let phrase2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::play_json_interactive(phrase2)?;
    
    // Synchronous acquisition of server time (equivalent to Web Audio's currentTime)
    let server_time = client::get_server_time()?;
    println!("Current server time: {:.6} seconds", server_time);
    
    // End interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

#### Advanced Features

**Schedule Clear Function**
```rust
// Start phrase 1
client::play_json_interactive(phrase1_json)?;

// Switch to phrase 2 mid-phrase without audio gaps
client::clear_schedule()?; // Clear events not yet processed
client::play_json_interactive(phrase2_json)?; // Immediately schedule the new phrase
```

**Server Time Synchronization**
```rust
// Get server time for precise timing control
let current_time = client::get_server_time()?;
// Functionality equivalent to Web Audio's currentTime property
```

#### Characteristics
- **Continuity**: Audio stream remains uninterrupted
- **Real-time control**: Dynamic scheduling of events
- **No audio gaps**: Smooth transitions between phrases
- **Time synchronization**: Precise timing control with the server
- **Use cases**: Real-time music control, timbre editors, live performance

#### Timing Conversion
In interactive mode, JSON in ym2151log format (sample units, 55930 Hz) is automatically converted to f64 seconds and sent to the server:

```rust
// Input: Sample units (i64, 55930 Hz)
let input_json = r#"{"event_count": 1, "events": [
    {"time": 2797, "addr": "0x08", "data": "0x00"}  // 2797 samples = approx. 0.05 seconds
]}"#;

// Automatic internal conversion: f64 seconds
// Sent to server as {"time": 0.050027, ...}
client::play_json_interactive(input_json)?;
```

### Server-Client Mode

#### Starting the Server

Starts as a persistent server in a waiting state:

```bash
# Normal mode (log file only)
cargo run --release -- server

# Verbose mode (detailed logs and WAV output)
cargo run --release -- server --verbose

# Low-quality resampling mode (for comparison)
cargo run --release -- server --low-quality-resampling

# Verbose + low-quality resampling
cargo run --release -- server --verbose --low-quality-resampling
```

#### Client Operations

Operate from another terminal in client mode:

```bash
# Play a new JSON file (switch performance)
cargo run --release -- client output_ym2151.json

# Play a new JSON file in verbose mode
cargo run --release -- client output_ym2151.json --verbose

# Stop playback (mute)
cargo run --release -- client --stop

# Shut down the server
cargo run --release -- client --shutdown
```

### Command Line Arguments

```
Usage:
  ym2151-log-play-server check                     # Check for updates
  ym2151-log-play-server server [OPTIONS]           # Server mode
  ym2151-log-play-server client [OPTIONS] [FILE]    # Client mode
  ym2151-log-play-server update                    # Update to latest version

Server Mode:
  server                    Start as a server in waiting state
  server --verbose          Start in verbose log mode (outputs WAV files)
  server --low-quality-resampling  Use low-quality resampling (linear interpolation, for comparison)

Client Mode:
  client <json_file>        Instruct the server to play a new JSON file
  client <json_file> --verbose  Instruct playback with detailed status messages
  client --stop             Instruct the server to stop playback
  client --stop --verbose   Stop playback with detailed status messages
  client --shutdown         Instruct the server to shut down
  client --shutdown --verbose  Shut down the server with detailed status messages

Examples:
  # Check for updates
  ym2151-log-play-server check

  # Start server
  ym2151-log-play-server server

  # Start server (verbose, with WAV output)
  ym2151-log-play-server server --verbose

  # Start server (low-quality resampling)
  ym2151-log-play-server server --low-quality-resampling

  # From another terminal: Switch playback
  ym2151-log-play-server client output_ym2151.json

  # From another terminal: Play in verbose mode
  ym2151-log-play-server client output_ym2151.json --verbose

  # From another terminal: Stop playback
  ym2151-log-play-server client --stop

  # From another terminal: Terminate server
  ym2151-log-play-server client --shutdown

  # Update to latest version
  ym2151-log-play-server update
```

### Usage Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- server

# Terminal 2: Operate from client
$ cargo run --release -- client output_ym2151.json

$ cargo run --release -- client --stop

$ cargo run --release -- client --shutdown
```

#### Scenario 2: Continuous Playback

```bash
# Start server (Terminal 1)
$ cargo run --release -- server

# Switch songs one after another (Terminal 2)
$ cargo run --release -- client music2.json
$ Start-Sleep 5
$ cargo run --release -- client music3.json
$ Start-Sleep 5
$ cargo run --release -- client music1.json
```

### Release Build

```bash
cargo build --release
.\target\release\ym2151-log-play-server.exe server
.\target\release\ym2151-log-play-server.exe server --verbose
.\target\release\ym2151-log-play-server.exe client output_ym2151.json
.\target\release\ym2151-log-play-server.exe client --stop
.\target\release\ym2151-log-play-server.exe client --shutdown
```

### Running Tests

```bash
cargo test
```

## Build Requirements

- Rust 1.70 or later

## Future Outlook
- Currently undergoing breaking changes
  - JSON format is subject to change
  - The specification for the default cycle consumption after register writes will be simplified to apply it uniformly at the final stage.
- // Currently, it is considered stable.
- // Will implement as needs arise.

## Project Goals
- **Motivation:**
  - **Past Challenges:**
    - Unable to input the next command until playback finishes (`ym2151-log-player-rust`)
  - **Solution:**
    - Reside as a server and be controlled by a client
  - **Use Cases:**
    - Provide an experience where commands can be input while playback is ongoing, similar to MSX's PLAY statement.
    - From timbre editors, phrase editors:
      - Utilize the library (crate) as a client
    - Integrate the library (crate) into a player to make it both a server and a client.
      - On the first run, launch a clone of itself as a background server, start playback, and then terminate itself.
        - *Unlike explicit server usage, the idea is to output messages to logs instead of printing, as logs are easier to grasp.
      - After the server starts, it acts as a client, sends JSON to the server, and then terminates.
- Simple and minimal, making it easy to reference when building larger projects.
- If it stops playing, the intention is to prioritize making it play again.

## Non-Goals (Out of Scope)
- Extreme optimization. Sacrificing ease of development to pursue speed, aiming for zero audio dropouts under any environment or high load.
- Extensive features. Sacrificing ease of development to input and automatically convert all kinds of music data for playback. Controlling multiple YM2151 chips. MIDI input/output.
- High fidelity reproduction. Sacrificing ease of development to perfectly reproduce all existing YM2151 songs.

## Project Intent
- Why was this module division chosen?
  - To allow the layers above this (from MML input to log generation) to be TDD'd by the GitHub Copilot Coding Agent on GitHub Linux Runner.
  - This layer (Windows real-time playback, and Windows client-server) cannot be TDD'd by the GitHub Copilot Coding Agent on GitHub Linux Runner. Instead, TDD by a local Windows agent is required, making the work slightly more demanding.
  - Therefore, this more demanding layer was separated to enable efficient development of other layers.

## Development Method
- TDD with agent on Windows
- Linux is prohibited specifically for this project
  - Because:
    - In the early stages of development, essentially Linux-specific code was generated
      - It might have been useful as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence/absence branching, other branching, and a large number of associated comments,
      - led to code bloat, becoming a breeding ground for hallucinations.
      - resulted in low-quality code with many unnecessary `allow deadcode`, ignored tests, duplicate tests, and redundant `cfg windows` branches.
      - Hallucinations occurred frequently, making bug fixes and Windows-specific feature implementation impossible.
    - It was discovered that agent TDD on Windows works well for this project.
      - The aforementioned hallucinations and redundancies were also resolved through robust refactoring using TDD.
- Batch installation of related applications
  - Convenient for usage and development.
  - Prerequisite: `cargo install rust-script`.
```powershell
rust-script install-ym2151-tools.rs
```

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust libraries: According to each library's license