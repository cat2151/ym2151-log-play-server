# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server and client that receives YM2151 (OPM) register event logs and performs real-time playback.

## Target Platforms

- Windows only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed, therefore,
        - Linux-specific code is prohibited.

## Overview

This project is a program that plays register event logs from the YM2151 (OPM) sound chip.
It operates in both standalone mode and server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Resides as a server and continues real-time playback in the background
- Controlled by a client to quickly switch to different playback
- Utilizes named pipes for server-client communication

## Usage

### Usage as a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready (automatically installs and starts if needed)
    client::ensure_server_ready("cat-play-mml")?;
    
    // Play a music file
    client::play_file("music.json")?;
    
    // Or, send JSON data directly
    let json_data = r#"{"event_count": 2, "events": [...]}"#;
    client::send_json(json_data)?;
    
    // Control playback
    client::stop_playback()?;
    
    // Shut down on exit
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

### Standalone Mode (Normal Playback)

Play a JSON file directly:

```bash
# Build and run
cargo run --release output_ym2151.json

# Or use an already built binary
./target/release/ym2151-log-play-server output_ym2151.json
```

### Server-Client Mode

#### Starting the Server

Launch as a persistent server in standby mode:

```bash
cargo run --release -- --server
```

#### Client Operations

Operate from another terminal in client mode:

```bash
# Play a new JSON file (switch playback)
cargo run --release -- --client test_input.json

# Stop playback (mute)
cargo run --release -- --client --stop

# Shut down the server
cargo run --release -- --client --shutdown
```

### Command-Line Arguments

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server                  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play a new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Shut down server

Options:
  --server           Launch as a persistent server in standby mode
  --client <file>    Instruct the server to play a new JSON file
  --client --stop    Instruct the server to stop playback
  --client --shutdown Instruct the server to shut down

Examples:
  # Play in standalone mode
  ym2151-log-play-server output_ym2151.json

  # Start server
  ym2151-log-play-server --server

  # From another terminal: Switch playback
  ym2151-log-play-server --client test_input.json

  # From another terminal: Stop playback
  ym2151-log-play-server --client --stop

  # From another terminal: Shut down server
  ym2151-log-play-server --client --shutdown
```

### Example Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server
Server started: /tmp/ym2151-log-play-server.pipe
Server is running. Waiting for client connection...

# Terminal 2: Client operations
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

## Future Plans
- Currently perceived as stable
- Implement as needs arise

## Project Goals
- Motivation:
  - Previous challenges:
    - Unable to input the next command until playback finishes
  - Solution:
    - Reside as a server and be controlled by a client
  - Use cases:
    - Provide an experience similar to MSX's PLAY statement, where commands can be input while music is playing
    - From a timbre editor or phrase editor,
      - Use the crate as a client
    - Integrate the crate into a player to make it both a server and a client
      - On first run, launch a clone of itself as a background server to start playback, then exit.
        - *Unlike explicit server usage, the concept is to output messages to a log instead of printing, making it easier to track.
      - After the server starts, send JSON to the server as a client, then exit.
- Simple and minimal, making it easy to reference when building larger projects.
- If sound stops, the intention is to prioritize actions to restore it.

## Project Intent
- Why was this module division chosen?
  - To enable GitHub Copilot Coding Agent to perform TDD on layers above this (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback, and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner, requiring TDD by a local Windows agent instead, which implies a slightly higher workload.
  - Therefore, this higher-workload layer was separated to enable efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Methodology
- TDD with an agent on Windows
- Linux is prohibited specifically for this project.
  - Because:
    - In the early stages, virtually Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments,
      - led to code bloat and became a hotbed for hallucinations.
      - This resulted in low-quality code with unnecessary `allow(dead_code)`, ignored tests, duplicate tests, and redundant `cfg(windows)` branches.
      - Hallucinations occurred frequently, making bug fixes and implementation of Windows-specific features impossible.
    - It was discovered that TDD with an agent on Windows functions well for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Subject to each crate's license