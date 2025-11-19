# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server-client application that receives YM2151 (OPM) register event logs and performs real-time playback.

## Target Platforms

- Windows only
- Prohibition of Linux-specific code
    - As an increase in hallucinations was observed in this project,
        - Linux-specific code is prohibited.

## Status

Used as a library integrated into `cat-play-mml` and `ym2151-tone-editor`.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (when verbose)
- Stays resident as a server, continuing real-time playback in the background
- Controlled by a client to quickly switch to different performances
- Utilizes named pipes for server-client communication

## Usage

### Using as a Library (Programmatic Control)

The recommended pattern for programmatic use of this library:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready (automatically installs and launches if necessary)
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

### Interactive Mode (Real-time Register Streaming)

Interactive mode enables continuous audio streaming via real-time register writes. It is ideal for applications like tone editors that require immediate audio feedback and want to avoid playback gaps.

#### Basic Interactive Mode

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode
    client::start_interactive()?;
    
    // Write registers with specified timing (in seconds, f64)
    client::write_register(0.0, 0x08, 0x00)?;     // Immediate: All channels key-off
    client::write_register(0.050, 0x28, 0x48)?;   // +50ms: Set pitch
    client::write_register(0.050, 0x08, 0x78)?;   // +50ms: Channel 0 key-on
    client::write_register(0.500, 0x08, 0x00)?;   // +500ms: Key-off
    
    // Get server time for precise synchronization
    let server_time = client::get_server_time()?;
    println!("Server time: {:.6} seconds", server_time);
    
    // Stop interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

#### Interactive Mode using JSON Data (Convenience Function)

For client applications already possessing `ym2151log`-formatted JSON data, the `play_json_interactive()` convenience function eliminates the need for manual implementation of conversion or timing logic. This function only handles JSON parsing and register writes; the user controls the interactive mode's lifecycle:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode once
    client::start_interactive()?;
    
    // Send multiple JSONs without stopping - no audio glitches!
    let json1 = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x28", "data": "0x48"}
        ]
    }"#;
    client::play_json_interactive(json1)?;
    
    let json2 = r#"{
        "event_count": 1,
        "events": [
            {"time": 5594, "addr": "0x08", "data": "0x78"}
        ]
    }"#;
    client::play_json_interactive(json2)?;
    
    // Wait for playback to complete
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Stop interactive mode when done
    client::stop_interactive()?;
    
    Ok(())
}
```

**Key Features:**
- **Continuous Streaming**: Eliminates audio dropouts and silent gaps during parameter changes.
- **Latency Compensation**: 50ms buffer for jitter correction (Web Audio-style scheduling).
- **Sample-Accurate Timing**: Provides precision up to 1/55930 seconds (1 sample) using Float64 seconds (Web Audio API compatible).
- **Server Time Synchronization**: `get_server_time()` allows retrieving the server's time coordinate system for precise scheduling.
- **No WAV Output**: Optimized for real-time with no file I/O overhead.
- **Convenience Function**: `play_json_interactive()` handles JSON parsing and time conversion without interactive mode lifecycle management.

**Benefits:**
- Immediate audio feedback in tone editors (e.g., `ym2151-tone-editor`).
- Smooth parameter changes without playback interruption.
- Can continuously send multiple JSONs without audio glitches.
- Lower latency compared to static event log playback.
- Web Audio-compatible time representation for cross-platform consistency.
- Client controls the start/stop of interactive mode.

Refer to `examples/interactive_demo.rs` and `examples/play_json_interactive_demo.rs` for complete examples.

### Server-Client Mode

#### Starting the Server

Stays resident as a server and starts in a waiting state:

```bash
# Normal mode (log file only)
cargo run --release -- server

# Verbose mode (detailed logs and WAV output)
cargo run --release -- server --verbose
```

#### Client Operations

Operate from another terminal in client mode:

```bash
# Play a new JSON file (switch performance)
cargo run --release -- client test_input.json

# Stop playback (mute)
cargo run --release -- client --stop

# Shut down the server
cargo run --release -- client --shutdown
```

### Command Line Arguments List

```
Usage:
  ym2151-log-play-server server [--verbose]         # Server mode
  ym2151-log-play-server client <json_log_file>     # Play new JSON
  ym2151-log-play-server client --stop              # Stop playback
  ym2151-log-play-server client --shutdown          # Shut down server

Options:
  server           Starts the server in a waiting state.
  server --verbose Starts the server in verbose mode (outputs WAV files).
  client <file>    Instructs the server to play a new JSON file.
  client --stop    Instructs the server to stop playback.
  client --shutdown Instructs the server to shut down.

Examples:
  # Start server
  ym2151-log-play-server server

  # Start server (verbose, with WAV output)
  ym2151-log-play-server server --verbose

  # From another terminal: Switch performance
  ym2151-log-play-server client test_input.json

  # From another terminal: Stop playback
  ym2151-log-play-server client --stop

  # From another terminal: Shut down server
  ym2151-log-play-server --client --shutdown
```

### Usage Example Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start Server
$ cargo run --release -- --server
Server started: /tmp/ym2151-log-play-server.pipe
Server is running. Waiting for client connections...

# Terminal 2: Client Operations
$ cargo run --release -- --client test_input.json
✅ Sent PLAY command to server

$ cargo run --release -- --client --stop
✅ Sent STOP command to server

$ cargo run --release -- --client --shutdown
✅ Sent SHUTDOWN command to server
```

#### Scenario 2: Continuous Playback

```bash
# Start server (Terminal 1)
$ cargo run --release -- --server

# Switch songs consecutively (Terminal 2)
$ cargo run --release -- --client music2.json
$ sleep 5
$ cargo run --release -- --client music3.json
$ sleep 5
$ cargo run --release -- --client music1.json
```

### Release Build

```bash
cargo build --release
./target/release/ym2151-log-play-server output_ym2151.json
./target/release/ym2151-log-play-server --server
./target/release/ym2151-log-play-server --client output_ym2151.json
./target/release/ym2151-log-play-server --client --stop
./target/release/ym2151-log-play-server --client --shutdown
```

### Running Tests

```bash
cargo test
```

## Build Requirements

- Rust 1.70 or later
- zig cc (used as a C compiler)

## Future Prospects
- Currently, the status is considered stable.
- Will implement as needed, upon discovery.

## Project Goals
- Motivation:
  - Previous challenges:
    - Cannot input the next command until playback finishes (`ym2151-log-player-rust`).
  - Solution:
    - Stays resident as a server and is controlled by a client.
  - Use Cases:
    - Provides an experience like MSX's PLAY statement, where the next command can be entered while playing.
    - From a tone editor or phrase editor,
      - Utilize the crate as a client.
    - Integrate the crate into a player, making it both a server and a client.
      - Initially, start a copy of itself as a background server to begin playback, then the original process exits.
        - *Unlike explicit server usage, the idea is to output messages to a log instead of printing to console; logs are easier to track.*
      - After the server is launched, act as a client to send JSON to the server, then the client process exits.
- Simple and minimal. Designed to be easy to reference when building larger projects.
- If it stops playing sound, the intention is to prioritize getting it to play again.

## Project Intent
- Why was such module partitioning performed?
  - To enable the GitHub Copilot Coding Agent to perform TDD on layers above this (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by the GitHub Copilot Coding Agent on GitHub Linux Runner, and instead requires TDD by a local Windows agent, which entails a somewhat higher workload.
  - Therefore, this high-workload layer was separated to enable efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with agent on Windows.
- Linux prohibited specifically for this project.
  - This is because:
    - In the early stages, virtually Linux-specific code was generated.
      - (It might have been helpful for the Windows version's foundation.)
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments,
      - led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code, including unnecessary `allow deadcode`, ignored tests, duplicate tests, and redundant `cfg windows` branching.
      - Frequent hallucinations made bug fixes and feature implementation for the Windows version impossible.
    - It was discovered that TDD with an agent on Windows works well for this project.
      - The aforementioned hallucinations and redundancies were resolved through robust refactoring using TDD.
- Batch installation of related applications.
    - Useful for usage and development.
    - Prerequisite: `cargo install rust-script`.
```powershell
rust-script install-ym2151-tools.rs
```

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow individual crate licenses