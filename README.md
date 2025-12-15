# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server and client that receive YM2151 (OPM) register event logs and perform real-time playback. Written in Rust.

## Target Platforms

- Windows only
- Linux-specific code is prohibited
    - Due to an observed increase in hallucinations within this project:
        - Linux-specific code is prohibited.

## Development Status

This library is currently integrated into and used by `cat-play-mml` and `ym2151-tone-editor`.

Frequent breaking changes are to be expected, especially concerning the client-server protocol and server operating modes.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in a server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (when verbose mode is enabled)
- Runs as a resident server, continuing real-time playback in the background
- Client control for quick switching to different performances
- Utilizes named pipes for server-client communication

## Usage

### Using as a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready (automatically installs and starts if needed)
    client::ensure_server_ready("cat-play-mml")?;
    
    // Send JSON data
    let json_data = r#"{"event_count": 2, "events": [...]}"#;
    client::send_json(json_data)?;
    
    // Playback control
    client::stop_playback()?;
    
    // Shut down on exit
    client::shutdown_server()?;
    
    Ok(())
}
```

The `ensure_server_ready()` function automatically performs the following, providing a seamless development experience:
1. Checks if the server is already running.
2. Installs the server application via cargo if not found in PATH.
3. Launches the server in background mode.
4. Waits until the server is ready to accept commands.

This eliminates the need for library users to manually manage the server's lifecycle.

## Client Implementation Guide

This section describes two primary client implementation patterns.

### Pattern 1: Non-Interactive Mode

Non-interactive mode is a simple mode suitable for one-off JSON data transmissions.
Playback stops and restarts with each JSON transmission.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready (automatically installs and starts if needed)
    client::ensure_server_ready("your-app-name")?;
    
    // Send JSON data (starts playback)
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
    
    // Shut down on exit
    client::shutdown_server()?;
    
    Ok(())
}
```

#### Characteristics
- **Simple**: Each JSON is processed independently.
- **Playback Switching**: Previous playback automatically stops with each new JSON transmission.
- **Intervals**: Short periods of silence may occur between JSON transmissions.
- **Use Cases**: Switching songs, applications not sensitive to continuity, WAV saving (verbose mode).

### Pattern 2: Interactive Mode

Interactive mode is an advanced mode suitable for real-time audio control.
It allows dynamic scheduling of register events while maintaining a continuous audio stream.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Prepare the server
    client::ensure_server_ready("your-app-name")?;
    
    // Start interactive mode (starts continuous audio stream)
    client::start_interactive()?;
    
    // Send multiple JSONs without silent gaps
    let phrase1 = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x78"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::play_json_interactive(phrase1)?;
    
    // Switch to another phrase mid-phrase (no audio gap)
    client::clear_schedule()?; // Cancels future events
    let phrase2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::play_json_interactive(phrase2)?;
    
    // Get synchronized server time (equivalent to Web Audio's currentTime)
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

// Switch to phrase 2 mid-phrase without a silent gap
client::clear_schedule()?; // Clears events that haven't been processed yet
client::play_json_interactive(phrase2_json)?; // Immediately schedules the new phrase
```

**Server Time Synchronization**
```rust
// Get server time for precise timing control
let current_time = client::get_server_time()?;
// Functionality equivalent to Web Audio's currentTime property
```

#### Characteristics
- **Continuity**: The audio stream is uninterrupted.
- **Real-time Control**: Dynamic scheduling of events.
- **No Silent Gaps**: Smooth transitions between phrases.
- **Time Synchronization**: Precise timing control with the server.
- **Use Cases**: Real-time music control, sound editor, live performance.

#### Timing Conversion
In interactive mode, JSON in ym2151log format (sample units, 55930 Hz) is automatically converted to f64 seconds and sent to the server:

```rust
// Input: Sample units (i64, 55930 Hz)
let input_json = r#"{"event_count": 1, "events": [
    {"time": 2797, "addr": "0x08", "data": "0x00"}  // 2797 samples = approx. 0.05 seconds
]}"#;

// Automatically converted internally: f64 seconds
// Sent to server as {"time": 0.050027, ...}
client::play_json_interactive(input_json)?;
```

### Server-Client Mode

#### Server Startup

Start as a resident server, in a waiting state:

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

From another terminal, operate in client mode:

```bash
# Play a new JSON file (switches performance)
cargo run --release -- client output_ym2151.json

# Play a new JSON file in verbose mode
cargo run --release -- client output_ym2151.json --verbose

# Stop playback (mute)
cargo run --release -- client --stop

# Shut down the server
cargo run --release -- client --shutdown
```

### Command-Line Argument List

```
Usage:
  ym2151-log-play-server server [OPTIONS]           # Server mode
  ym2151-log-play-server client [OPTIONS] [FILE]    # Client mode

Server mode:
  server                    Start as a server in a waiting state
  server --verbose          Start in verbose log mode (outputs WAV files)
  server --low-quality-resampling  Use low-quality resampling (linear interpolation, for comparison)

Client mode:
  client <json_file>        Instruct the server to play a new JSON file
  client <json_file> --verbose  Instruct to play with detailed status messages
  client --stop             Instruct the server to stop playback
  client --stop --verbose   Instruct to stop playback with detailed status messages
  client --shutdown         Instruct the server to shut down
  client --shutdown --verbose  Instruct the server to shut down with detailed status messages

Examples:
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

  # From another terminal: Shut down server
  ym2151-log-play-server client --shutdown
```

### Usage Example Scenarios

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

# Continuously switch songs (Terminal 2)
$ cargo run --release -- client music2.json
$ Start-Sleep 5
$ cargo run --release -- client music3.json
$ Start-Sleep 5
$ cargo run --release -- client music1.json
```

### Release Build

```powershell
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
- Undergoing breaking changes
  - JSON format is subject to change.
  - The specification for predefined cycle consumption after register writes is planned to be simplified for batch application at the final stage.
- // Currently considered stable.
- // Will implement features as needs arise.

## Project Goals
- Motivation:
  - Previous challenges:
    - Could not input the next command until playback finished (`ym2151-log-player-rust`).
  - Solution:
    - Operate as a resident server, controlled by a client.
  - Applications:
    - Provide an experience similar to MSX's PLAY statement, where the next command can be input during playback.
    - Sound editors, phrase editors:
      - Use the crate as a client.
    - Integrate the crate into a player, making it both server and client:
      - On first run, launch a clone of itself in the background as a server to start playback, then the original instance exits.
        - *Conception: instead of printing, output messages to a log for better understanding, unlike explicit server use.*
      - After the server is launched, act as a client to send JSON to the server, then the client instance exits.
- Simple and minimal. Easy to reference when building larger projects.
- If it stops producing sound, the intention is to prioritize fixing it to ensure sound plays.

## What the Project Does NOT Aim For (Out of Scope)
- High speed. Sacrificing ease of development to pursue speed. Zero audio glitches regardless of environment or load.
- High functionality. Sacrificing ease of development to input all kinds of music data, automatically convert, and play. Control multiple YM2151 chips. MIDI input/output.
- High accuracy reproduction. Sacrificing ease of development to perfectly reproduce and play all existing YM2151 songs.

## Project Intent
- Why such module separation?
  - To enable the GitHub Copilot Coding Agent to perform TDD on GitHub Linux Runner for layers above this (from MML input to log generation).
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner, instead requiring TDD by a local Windows agent, which is somewhat more labor-intensive.
  - Therefore, to efficiently develop other layers, this more labor-intensive layer was isolated.

## Development Method
- TDD with an agent on Windows.
- For this project specifically, Linux is prohibited.
  - Because:
    - Early on, code that was effectively Linux-specific was generated.
      - Although it might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, real-time audio presence branching, other branching, and associated numerous comments,
      - Led to code bloat, becoming a breeding ground for hallucinations.
      - Resulted in low-quality code, with unnecessary `allow deadcode`, ignored tests, duplicate tests, useless `cfg windows` branching, etc.
      - Frequent hallucinations made bug fixing and Windows feature implementation impossible.
    - It was discovered that agent-based TDD on Windows functions well for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.
- Bulk Installation of Related Apps
    - Useful for purposes and development.
    - Prerequisite: `cargo install rust-script`
```powershell
rust-script install-ym2151-tools.rs
```

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Subject to their respective licenses.