# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server and client that receives YM2151 (OPM) register event logs and performs real-time playback.

## Target Platforms

- Windows Only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed, therefore,
        - Linux-specific code is prohibited.

## Status

It is currently used as a library integrated into projects such as `cat-play-mml` and `ym2151-tone-editor`.

## Overview

This project is a program that plays back YM2151 (OPM) sound chip register event logs.
It operates in a server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (when in verbose mode)
- Runs as a resident server, continuing real-time playback in the background
- Controlled by clients to quickly switch to different performances
- Utilizes named pipes for server-client communication

## Usage

### Usage as a Library (Programmatic Control)

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

The `ensure_server_ready()` function automatically performs the following actions, providing a seamless development experience:
1. Checks if the server is already running
2. Installs the server application via cargo if it's not found in PATH
3. Starts the server in background mode
4. Waits until the server is ready to accept commands

This eliminates the need for library users to manually manage the server's lifecycle.

### Interactive Mode (Real-time Register Streaming)

Interactive mode enables continuous audio streaming through real-time register writes. It is ideal for applications requiring immediate audio feedback and wishing to avoid playback gaps, such as tone editors.

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode
    client::start_interactive()?;
    
    // Write registers with specified timing
    client::write_register(0, 0x08, 0x00)?;     // Immediate: All channels key off
    client::write_register(50, 0x28, 0x48)?;    // +50ms: Set pitch
    client::write_register(50, 0x08, 0x78)?;    // +50ms: Channel 0 key on
    client::write_register(500, 0x08, 0x00)?;   // +500ms: Key off
    
    // Stop interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

**Key Features:**
- **Continuous Streaming**: Uninterrupted audio, eliminating silent gaps during parameter changes
- **Latency Compensation**: 50ms buffer for jitter correction (Web Audio-style scheduling)
- **Time Scheduling**: Millisecond-precision register write scheduling
- **No WAV Output**: Optimized for real-time operation without file I/O overhead

**Benefits:**
- Immediate audio feedback in tone editors (e.g., ym2151-tone-editor)
- Smooth parameter changes without playback interruption
- Lower latency compared to static event log playback

See `examples/interactive_demo.rs` for a complete example.

### Server-Client Mode

#### Starting the Server

To start the server as a resident process in a waiting state:

```bash
# Normal mode (log file only)
cargo run --release -- server

# Verbose mode (detailed logs and WAV output)
cargo run --release -- server --verbose
```

#### Client Operations

From another terminal, operate in client mode:

```bash
# Play a new JSON file (switches performance)
cargo run --release -- client test_input.json

# Stop playback (mute)
cargo run --release -- client --stop

# Shut down the server
cargo run --release -- client --shutdown
```

### Command Line Arguments

```
Usage:
  ym2151-log-play-server server [--verbose]         # Server mode
  ym2151-log-play-server client <json_log_file>     # Play new JSON
  ym2151-log-play-server client --stop              # Stop playback
  ym2151-log-play-server client --shutdown          # Shut down server

Options:
  server           Starts the server in a waiting state
  server --verbose Starts the server in verbose log mode (outputs WAV files)
  client <file>    Instructs the server to play a new JSON file
  client --stop    Instructs the server to stop playback
  client --shutdown Instructs the server to shut down

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

### Usage Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server
Server started: /tmp/ym2151-log-play-server.pipe
Server is running. Waiting for client connections...

# Terminal 2: Client operation
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

# Switch songs continuously (Terminal 2)
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

## Future Outlook
- Currently, the situation is considered stable.
- Implementation will proceed as needs are identified.

## Project Goals
- Motivation:
  - Past challenges:
    - Cannot input the next command until the current performance finishes
  - Solution:
    - Run as a resident server, controlled by clients
  - Use cases:
    - Provide an experience similar to MSX's PLAY statement, allowing input of the next command while music is playing.
    - From tone editors, phrase editors, etc.,
      - Use the crate as a client
    - Integrate the crate into a player to make it a server-client
      - Initially, launch a duplicate of itself as a server in the background to start playback, then the original process exits.
        - *Unlike explicit server usage, the idea is to output messages to logs instead of printing, as logs are easier to track.
      - After the server is launched, act as a client to send JSON to the server, then the client process exits.
- Simple and minimal. Easy to reference when building larger projects.
- If it stops producing sound, the intention is to prioritize actions to restore sound playback.

## Project Intent
- Why was this module split designed this way?
  - To enable the GitHub Copilot Coding Agent to perform TDD on the layers above this one (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by the GitHub Copilot Coding Agent on GitHub Linux Runner; instead, it requires TDD by a Windows local agent, resulting in a somewhat higher workload.
  - Therefore, this high-workload layer was separated to allow more efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing music

## Development Methodology
- TDD with agent on Windows
- Linux is prohibited for this project specifically.
  - Because:
    - Early on, virtually Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence/absence branching, other branches, and a large number of associated comments,
      - led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code, with excessive `allow deadcode`, ignored tests, duplicate tests, unnecessary `cfg windows` branches, etc.
      - Frequent hallucinations occurred, making bug fixes and Windows feature implementation impossible.
    - It was found that TDD with an agent works well on Windows for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow each crate's license