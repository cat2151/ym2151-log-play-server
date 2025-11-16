# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server and client that receive YM2151 (OPM) register event logs and perform real-time playback.

## Target Platforms

- Windows-only
- Prohibition of Linux-specific code
    - As an increase in hallucinations was observed in this project,
        - Linux-specific code is prohibited.

## Status

It is used as a library integrated into `cat-play-mml` and `ym2151-tone-editor`.

## Overview

This project is a program that plays back YM2151 (OPM) sound chip register event logs.
It operates in both standalone and server-client modes.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Resides as a server and continues real-time playback in the background
- Controlled by a client, allowing quick switching to different performances
- Uses named pipes for server-client communication

## Usage

### As a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready (automatically installs and starts if necessary)
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
1. Checks if the server is already running
2. Installs the server application via cargo if not found in PATH
3. Starts the server in background mode
4. Waits until the server is ready to accept commands

This eliminates the need for library users to manually manage the server's lifecycle.

### Standalone Mode (Normal Playback)

Plays JSON files directly:

```bash
# Build and run
cargo run --release output_ym2151.json

# Or use an already built binary
./target/release/ym2151-log-play-server output_ym2151.json
```

### Server-Client Mode

#### Starting the Server

Starts as a resident server in a waiting state:

```bash
cargo run --release -- --server
```

#### Client Operations

Operate from a separate terminal in client mode:

```bash
# Play a new JSON file (switch performance)
cargo run --release -- --client test_input.json

# Stop playback (mute)
cargo run --release -- --client --stop

# Shut down the server
cargo run --release -- --client --shutdown
```

### Command-Line Argument List

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server                  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Shut down server

Options:
  --server           Start as a server in a waiting state
  --client <file>    Instruct server to play a new JSON file
  --client --stop    Instruct server to stop playback
  --client --shutdown Instruct server to shut down

Examples:
  # Play standalone
  ym2151-log-play-server output_ym2151.json

  # Start server
  ym2151-log-play-server --server

  # From another terminal: Switch performance
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
Server is running. Waiting for client connections...

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
- zig cc (used as C compiler)

## Future Prospects
- Currently, the situation is considered stable.
- Will implement as needed, once requirements are identified.

## Project Goals
- Motivation:
  - Previous challenges:
    - Unable to input the next command until playback finishes
  - Solution:
    - Reside as a server and be controlled by a client
  - Use cases:
    - Provide an experience like MSX's PLAY statement, where commands can be input while music is playing.
    - From a tone editor, phrase editor,
      - Use the crate as a client
    - Integrate the crate into a player, making it both a server and a client
      - First time, start a duplicate of itself as a server in the background, begin playback, and then terminate itself.
        - *Unlike explicit server usage, the idea is to output messages to a log instead of printing them; logs are easier to follow.*
      - After the server starts, it sends JSON to the server as a client and then terminates itself.
- Simple and minimal. Designed to be easy to reference when building larger projects.
- If it stops playing, I intend to prioritize getting it to play again.

## Project Intent
- Why was this module split designed this way?
  - To enable GitHub Copilot Coding Agent to perform TDD on the layers above this (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner; instead, it requires TDD by a local Windows agent, which entails a somewhat higher workload.
  - Therefore, this high-workload layer was separated to allow more efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with agent on Windows
- Linux is prohibited specifically for this project.
  - Because:
    - In the early stages, essentially Linux-specific code was generated.
      - It might have been useful as a foundation for the Windows version.
    - Unix/Linux/Windows branches, real-time audio presence/absence branches, other branches, and a large number of associated comments,
      - Led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code, with excessive `allow(dead_code)`, ignored tests, duplicate tests, unnecessary `cfg(windows)` branches, etc.
      - Frequent hallucinations made bug fixing and implementation of Windows-specific features impossible.
    - It was discovered that TDD with an agent works well on Windows for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: According to each crate's license