# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server and client that receive YM2151 (OPM) register event logs and play them back in real-time. Written in Rust.

## Target Platforms

- Windows only
- Restriction on Linux-Specific Code
    - In this project, an increase in hallucinations was observed, therefore:
        - Linux-specific code is prohibited.

## Development Status

This library is integrated and used in `cat-play-mml` and `ym2151-tone-editor`.

Frequent breaking changes occur, especially regarding the client-server protocol and server operating modes.

## Overview

This project is a program that plays YM2151 (OPM) sound chip register event logs.
It operates in a server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (in verbose mode)
- Runs as a resident server, continuing real-time playback in the background
- Controlled by a client, allowing quick switching to different playback
- Utilizes named pipes for server-client communication

## Usage

### Usage as a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server readiness (automatically installs and starts if needed)
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
1. Checks if the server is already running.
2. Installs the server application via cargo if not found in PATH.
3. Starts the server in background mode.
4. Waits until the server is ready to accept commands.

This eliminates the need for library users to manually manage the server's lifecycle.

## Client Implementation Guide

This section describes two main client implementation patterns.

### Pattern 1: Non-Interactive Mode

Non-interactive mode is a simple mode suitable for one-shot JSON data transmission.
Playback stops and restarts with each JSON transmission.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server readiness (automatically installs and starts if needed)
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
    
    // Shutdown on exit
    client::shutdown_server()?;
    
    Ok(())
}
```

#### Characteristics
- **Simple**: Each JSON is processed independently.
- **Playback Switching**: Previous playback automatically stops with each JSON transmission.
- **Potential for Gaps**: Short periods of silence may occur between JSONs.
- **Use Cases**: Switching songs, non-continuous applications, WAV saving (verbose mode).

### Pattern 2: Interactive Mode

Interactive mode is an advanced mode suitable for real-time audio control.
It maintains a continuous audio stream while dynamically scheduling register events.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Prepare server
    client::ensure_server_ready("your-app-name")?;
    
    // Start interactive mode (starts continuous audio stream)
    client::start_interactive()?;
    
    // Send multiple JSONs without audio gaps
    let phrase1 = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x78"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::play_json_interactive(phrase1)?;
    
    // Switch to a different phrase mid-phrase (without audio gaps)
    client::clear_schedule()?; // Cancels future events
    let phrase2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::play_json_interactive(phrase2)?;
    
    // Synchronous retrieval of server time (equivalent to Web Audio's currentTime)
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
client::clear_schedule()?; // Clears events not yet processed
client::play_json_interactive(phrase2_json)?; // Schedules the new phrase immediately
```

**Server Time Synchronization**
```rust
// Retrieve server time for precise timing control
let current_time = client::get_server_time()?;
// Equivalent functionality to Web Audio's currentTime property
```

#### Characteristics
- **Continuity**: The audio stream is uninterrupted.
- **Real-time Control**: Dynamic scheduling of events.
- **No Audio Gaps**: Smooth transitions between phrases.
- **Time Synchronization**: Precise timing control with the server.
- **Use Cases**: Real-time music control, tone editors, live performance.

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

Starts as a resident server, in a waiting state:

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
# Play a new JSON file (switches playback)
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

Server Mode:
  server                    Starts as a server in a waiting state
  server --verbose          Starts in verbose log mode (outputs WAV files)
  server --low-quality-resampling  Uses low-quality resampling (linear interpolation, for comparison)

Client Mode:
  client <json_file>        Instructs the server to play a new JSON file
  client <json_file> --verbose  Instructs to play with detailed status messages
  client --stop             Instructs the server to stop playback
  client --stop --verbose   Stops playback with detailed status messages
  client --shutdown         Instructs the server to shut down
  client --shutdown --verbose  Shuts down the server with detailed status messages

Examples:
  # Server startup
  ym2151-log-play-server server

  # Server startup (verbose, with WAV output)
  ym2151-log-play-server server --verbose

  # Server startup (low-quality resampling)
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
- Undergoing breaking changes
  - JSON format change planned
  - Plans to simplify the specification for default cycle consumption after register writes, applying it in bulk at the final stage.
- // Currently perceived as stable
- // Will implement as needed

## Project Goals
- Motivation:
  - Previous Challenges:
    - Cannot input next command until playback finishes (ym2151-log-player-rust)
  - Solution:
    - Run as a resident server, controlled by clients
  - Use Cases:
    - Provide an experience where new commands can be input while music is playing, similar to MSX's PLAY statement.
    - Utilize the crate as a client from a tone editor or phrase editor.
    - Integrate the crate into a player, making it both a server and a client.
      - First time, start a copy of itself as a background server to begin playback, then the original process terminates.
        - ※Unlike explicit server usage, the idea is to output messages to logs instead of stdout for easier understanding.
      - After the server starts, send JSON to the server as a client, then the original process terminates.
- Simple and minimal. Intended to be easy to reference when building larger projects.
- If it stops playing, the intention is to prioritize actions to get it playing again.

## Project Intent
- Why this module separation?
  - To enable GitHub Copilot Coding Agent to perform TDD on layers above this (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner, instead requiring TDD by a Windows local agent, which results in a slightly higher workload.
  - Therefore, this high-workload layer is separated to allow for more efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with an agent on Windows
- Linux is forbidden for this project alone.
  - Because:
    - Early on, virtually Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments led to code bloat and became a hotbed for hallucinations.
    - Resulted in low-quality code with unnecessary `allow deadcode`, `test ignored`, duplicate tests, and redundant `cfg windows` branching.
    - Frequent hallucinations prevented bug fixes and the implementation of Windows-specific features.
    - It was found that TDD with an agent works well on Windows for this project.
    - The aforementioned hallucinations and redundancies were resolved through robust refactoring using TDD.
- Bulk Installation of Related Apps
    - Useful for purposes and development
    - Prerequisite: `cargo install rust-script`
```powershell
rust-script install-ym2151-tools.rs
```

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow their respective licenses