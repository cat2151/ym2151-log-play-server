# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server and client that receive YM2151 (OPM) register event logs and perform real-time playback.

## Target Platform

- Windows only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed, therefore,
        - Linux-specific code is prohibited.

## Overview

This project is a program that plays register event logs of the YM2151 (OPM) sound chip.
It operates in both standalone mode and server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Resides as a server and continues real-time playback in the background
- Controlled by a client to quickly switch to another performance
- Utilizes named pipes for server-client communication

## Usage

### Usage as a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready (automatically installs and starts if necessary)
    client::ensure_server_ready("cat-play-mml")?;
    
    // Play a music file
    client::play_file("music.json")?;
    
    // Or, send JSON data directly
    let json_data = r#"{"event_count": 2, "events": [...]}"#;
    client::send_json(json_data)?;
    
    // Send JSON data in silent mode (no server log output)
    // Used by library users like ym2151-tone-editor to prevent display corruption
    client::send_json_silent(json_data)?;
    
    // Playback control
    client::stop_playback()?;
    
    // Shut down on exit
    client::shutdown_server()?;
    
    Ok(())
}
```

The `ensure_server_ready()` function automatically performs the following, providing a seamless development experience:
1. Checks if the server is already running
2. If the server application is not found in PATH, installs it via cargo
3. Starts the server in background mode
4. Waits until the server is ready to accept commands

This eliminates the need for library users to manually manage the server's lifecycle.

### Standalone Mode (Regular Playback)

Play a JSON file directly:

```bash
# Build and run
cargo run --release output_ym2151.json

# Or use an already built binary
./target/release/ym2151-log-play-server output_ym2151.json
```

### Server-Client Mode

#### Starting the Server

Start as a resident server, in a waiting state:

```bash
cargo run --release -- --server
```

#### Operating from a Client

Operate from another terminal, in client mode:

```bash
# Play a new JSON file (switch performance)
cargo run --release -- --client test_input.json

# Stop playback (mute)
cargo run --release -- --client --stop

# Shut down the server
cargo run --release -- --client --shutdown
```

### Command-Line Arguments List

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server                  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Server shutdown

Options:
  --server           Start as a server in a waiting state
  --client <file>    Instruct the server to play a new JSON file
  --client --stop    Instruct the server to stop playback
  --client --shutdown Instruct the server to shut down

Examples:
  # Play in standalone mode
  ym2151-log-play-server output_ym2151.json

  # Start the server
  ym2151-log-play-server --server

  # From another terminal: Switch playback
  ym2151-log-play-server --client test_input.json

  # From another terminal: Stop playback
  ym2151-log-play-server --client --stop

  # From another terminal: Shut down the server
  ym2151-log-play-server --client --shutdown
```

### Usage Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server
Server started: /tmp/ym2151-log-play-server.pipe
Server started. Waiting for client connection...

# Terminal 2: Operate from client
$ cargo run --release -- --client test_input.json
✅ PLAY command sent to server

$ cargo run --release -- --client --stop
✅ STOP command sent to server

$ cargo run --release -- --client --shutdown
✅ SHUTDOWN command sent to server
```

#### Scenario 2: Continuous Playback

```bash
# Start server (Terminal 1)
$ cargo run --release -- --server

# Switch songs one after another (Terminal 2)
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
- Currently, development is considered stable.
- Will implement features as needed.

## Project Goals
- Motivation:
  - Previous challenges:
    - Unable to input the next command until playback ends
  - Solution:
    - Reside as a server and control it from a client
  - Use cases:
    - Provide an experience similar to MSX's PLAY statement, where commands can be input while music is playing.
    - From tone editors, phrase editors, etc.,
      - Use the crate as a client
    - Incorporate the crate into a player to make it a server and client.
      - Initially, start a copy of itself as a background server to begin playback, then terminate.
        - *Unlike explicit server usage, the idea is to output messages to a log instead of printing them, as a log is easier to understand.
      - After the server starts, send JSON to the server as a client and then terminate.
- Simple and minimal. Easy to reference when building larger projects.
- If it stops making sound, I intend to prioritize efforts to get it working again.

## Project Intent
- Why was this module division made?
  - To enable GitHub Copilot Coding Agent to perform TDD on layers above this one (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner; instead, it requires TDD by a Windows local agent, which results in a somewhat higher workload.
  - Therefore, to isolate this high-workload layer, allowing other layers to be developed more efficiently.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with agent on Windows
- Linux prohibited for this project only
  - Because,
    - In the early stages, effectively Linux-specific code was generated.
      - It might have helped as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments,
      - led to code bloat and became a hotbed for hallucinations.
      - This resulted in low-quality code, with many unnecessary `allow(dead_code)`, ignored tests, duplicate tests, and redundant `cfg(windows)` branches.
      - Hallucinations occurred frequently, making bug fixes and Windows-specific feature implementation impossible.
    - It was found that TDD with an agent works well on Windows for this project.
      - The aforementioned hallucinations and inefficiencies were also resolved through robust refactoring using TDD.

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Subject to each crate's license