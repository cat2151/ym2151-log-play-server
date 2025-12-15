# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server/client that receives YM2151 (OPM) register event logs and performs real-time playback. Written in Rust.

## Target Platforms

- Windows only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed,
        - therefore, Linux-specific code is prohibited.

## Development Status

It is used as a library, integrated into projects like `cat-play-mml` and `ym2151-tone-editor`.

Frequent breaking changes are expected, especially regarding the client-server protocol and server operating modes.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in a server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (in verbose mode)
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client, allowing quick switching to different performances
- Utilizes named pipes for server-client communication

## Usage

### Usage as a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensures the server is ready (automatically installs and starts if needed)
    client::ensure_server_ready("cat-play-mml")?;
    
    // Sends JSON data
    let json_data = r#"{"event_count": 2, "events": [...]}"#;
    client::send_json(json_data)?;
    
    // Playback control
    client::stop_playback()?;
    
    // Shuts down on exit
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

This section describes two main client implementation patterns.

### Pattern 1: Non-Interactive Mode

Non-interactive mode is a simple mode suitable for one-shot JSON data transmission.
Playback stops and restarts with each JSON transmission.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensures the server is ready (automatically installs and starts if needed)
    client::ensure_server_ready("your-app-name")?;
    
    // Sends JSON data (starts playback)
    let json_data = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x00"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::send_json(json_data)?;
    
    // Controls playback as needed
    std::thread::sleep(std::time::Duration::from_secs(5));
    client::stop_playback()?;
    
    // Plays another JSON
    let json_data2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::send_json(json_data2)?;
    
    // Shuts down on exit
    client::shutdown_server()?;
    
    Ok(())
}
```

#### Characteristics
- **Simple**: Each JSON is processed independently
- **Playback Switching**: The previous playback automatically stops with each JSON transmission
- **Gaps Present**: Short periods of silence may occur between JSON transmissions
- **Use Cases**: Switching between songs, applications where continuity is not critical, WAV saving (verbose mode)

### Pattern 2: Interactive Mode

Interactive mode is an advanced mode suitable for real-time audio control.
It allows dynamic scheduling of register events while maintaining a continuous audio stream.

#### Basic Usage

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Prepares the server
    client::ensure_server_ready("your-app-name")?;
    
    // Starts interactive mode (continuous audio stream begins)
    client::start_interactive()?;
    
    // Sends multiple JSONs without silent gaps
    let phrase1 = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x78"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::play_json_interactive(phrase1)?;
    
    // Switches to another phrase mid-phrase (no audio gap)
    client::clear_schedule()?; // Cancels future events
    let phrase2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::play_json_interactive(phrase2)?;
    
    // Synchronously gets server time (equivalent to Web Audio's currentTime)
    let server_time = client::get_server_time()?;
    println!("Current server time: {:.6} seconds", server_time);
    
    // Ends interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

#### Advanced Features

**Schedule Clear Functionality**
```rust
// Starts phrase 1
client::play_json_interactive(phrase1_json)?;

// Switches to phrase 2 mid-phrase with no silent gap
client::clear_schedule()?; // Clears events that haven't been processed yet
client::play_json_interactive(phrase2_json)?; // Schedules the new phrase immediately
```

**Server Time Synchronization**
```rust
// Retrieves server time for precise timing control
let current_time = client::get_server_time()?;
// Equivalent functionality to Web Audio's currentTime property
```

#### Characteristics
- **Continuity**: Audio stream remains uninterrupted
- **Real-time Control**: Dynamic event scheduling
- **Seamless Transitions**: Smooth transitions between phrases with no silent gaps
- **Time Synchronization**: Precise timing control with the server
- **Use Cases**: Real-time music control, tone editors, live performances

#### Timing Conversion
In interactive mode, JSON in the ym2151log format (sample units, 55930 Hz) is automatically converted to `f64` seconds and sent to the server:

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

Operate from a separate terminal in client mode:

```bash
# Play a new JSON file (switches performance)
cargo run --release -- client output_ym2151.json

# Play a new JSON file in verbose mode
cargo run --release -- client output_ym2151.json --verbose

# Stop playback (mute)
cargo run --release -- client --stop

# Shutdown the server
cargo run --release -- client --shutdown
```

### Command-Line Arguments

```
Usage:
  ym2151-log-play-server server [OPTIONS]           # Server mode
  ym2151-log-play-server client [OPTIONS] [FILE]    # Client mode

Server Mode:
  server                    Starts as a persistent server in a waiting state
  server --verbose          Starts in verbose log mode (outputs WAV files)
  server --low-quality-resampling  Uses low-quality resampling (linear interpolation, for comparison)

Client Mode:
  client <json_file>        Instructs the server to play a new JSON file
  client <json_file> --verbose  Instructs playback with detailed status messages
  client --stop             Instructs the server to stop playback
  client --stop --verbose   Stops playback with detailed status messages
  client --shutdown         Instructs the server to shut down
  client --shutdown --verbose  Shuts down the server with detailed status messages

Examples:
  # Start server
  ym2151-log-play-server server

  # Start server (verbose, with WAV output)
  ym2151-log-play-server server --verbose

  # Start server (low-quality resampling)
  ym2151-log-play-server server --low-quality-resampling

  # From another terminal: Switch performance
  ym2151-log-play-server client output_ym2151.json

  # From another terminal: Play in verbose mode
  ym2151-log-play-server client output_ym2151.json --verbose

  # From another terminal: Stop playback
  ym2151-log-play-server client --stop

  # From another terminal: Shut down server
  ym2151-log-play-server client --shutdown
```

### Example Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- server

# Terminal 2: Client operations
$ cargo run --release -- client output_ym2151.json

$ cargo run --release -- client --stop

$ cargo run --release -- client --shutdown
```

#### Scenario 2: Continuous Playback

```bash
# Start server (Terminal 1)
$ cargo run --release -- server

# Switch songs sequentially (Terminal 2)
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

## Future Prospects
- Undergoing breaking changes
  - JSON format is planned to change
  - The specification for default cycle consumption after register writes is planned to be simplified, applying it in bulk at the final stage
- //Currently, it's considered stable.
- //Will be implemented as needed

## Project Goals
- Motivation:
  - Previous Challenges:
    - Cannot input the next command until playback finishes (`ym2151-log-player-rust`)
  - Solution:
    - Runs as a persistent server and is controlled by a client
  - Use Cases:
    - Provides an experience where you can input the next command while playing, similar to MSX's PLAY statement
    - From tone editors and phrase editors,
      - uses the library (crate) as a client
    - Integrates the library (crate) into a player, making it both a server and a client
      - Initially, it starts a duplicate of itself as a background server to begin playback, then exits itself.
        - *Unlike explicit server use, the concept is to output messages to a log instead of `print`, as logs are easier to understand.
      - After the server starts, it sends JSON to the server as a client, then exits itself.
- Simple and minimal, making it easy to reference when building larger projects.
- If playback stops, the intention is to prioritize actions to restore it as quickly as possible.

## Out-of-Scope (What the Project Does Not Aim For)
- Optimization for speed. Sacrificing ease of development to pursue speed, aiming for zero audio dropouts regardless of environment or load.
- High functionality. Sacrificing ease of development to input and automatically convert all kinds of music data for playback. Controlling multiple YM2151s. MIDI input/output.
- High-fidelity reproduction. Sacrificing ease of development to perfectly reproduce and play all existing YM2151 songs.

## Project Intent
- Why was this module split designed this way?
  - To enable the GitHub Copilot Coding Agent to perform TDD on layers above this (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by the GitHub Copilot Coding Agent on GitHub Linux Runner. Instead, it requires TDD by a local Windows agent, resulting in a somewhat higher workload.
  - Therefore, this high-workload layer was separated to allow more efficient development of other layers.

## Development Method
- TDD with an agent on Windows
- Linux is prohibited for this project only
  - Because:
    - In the early stages of development, essentially Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments,
      - led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code, with excessive `allow(dead_code)`, ignored tests, duplicate tests, unnecessary `cfg(windows)` branching, etc.
      - Frequent hallucinations occurred, making bug fixing and Windows-specific feature implementation impossible.
    - It was discovered that TDD with an agent works well on Windows for this project.
      - The aforementioned hallucinations and inefficiencies were also resolved through robust refactoring using TDD.
- Bulk Installation of Related Applications
    - Useful for purposes and development
    - Prerequisite: `cargo install rust-script`
```powershell
rust-script install-ym2151-tools.rs
```

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust libraries: Follow each library's license