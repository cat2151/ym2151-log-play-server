# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server-client application that receives YM2151 (OPM) register event logs and performs real-time playback.

## Target Platforms

- Windows-only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed,
        - therefore, Linux-specific code is prohibited.

## Overview

This project is a program that plays register event logs of the YM2151 (OPM) sound chip.
It operates in both standalone and server-client modes.

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
    // Ensure server is ready (automatically installs and starts if needed)
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

Play a JSON file directly:

```bash
# Build and run
cargo run --release output_ym2151.json

# Or use an already built binary
./target/release/ym2151-log-play-server output_ym2151.json
```

### Server-Client Mode

#### Starting the Server

Start as a resident server in a waiting state:

```bash
cargo run --release -- --server
```

#### Client Operations

Operate in client mode from a separate terminal:

```bash
# Play a new JSON file (switch performance)
cargo run --release -- --client test_input.json

# Stop playback (mute)
cargo run --release -- --client --stop

# Shut down the server
cargo run --release -- --client --shutdown
```

### Command Line Argument List

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server                  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Server shutdown

Options:
  --server           Start as a server in waiting state
  --client <file>    Instruct the server to play a new JSON file
  --client --stop    Instruct the server to stop playback
  --client --shutdown Instruct the server to shut down

Examples:
  # Play in standalone mode
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
- zig cc (used as C compiler)

## Future Prospects
- Current status is considered stable
- Implement features as needed

## Project Goals
- Motivation:
  - Previous challenges:
    - Unable to input the next command until playback finishes
  - Solution:
    - Reside as a server and be controlled by a client
  - Use cases:
    - Provide an experience where commands can be input while music is playing, similar to MSX's PLAY statement
    - From a tone editor or phrase editor,
      - use the crate as a client
    - Embed the crate into a player to make it both a server and a client
      - On first run, start a duplicate of itself as a server in the background, begin playback, and then terminate itself.
        - *Note: Unlike explicit server usage, the plan is to output messages to a log instead of printing them; logs make it easier to understand the state.
      - After the server is launched, send JSON to the server as a client and then terminate itself.
- Simple and minimal. Designed to be easy to reference when building larger projects.
- If it stops playing, the intention is to prioritize making it play again as much as possible.

## Project Intent
- Why was such a module split performed?
  - To enable the GitHub Copilot Coding Agent to perform TDD on the layers above this one (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD-ed by the GitHub Copilot Coding Agent on GitHub Linux Runner, and instead requires TDD by a Windows local agent, which results in a slightly higher workload.
  - Therefore, this higher-workload layer was isolated to allow for efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Approach
- TDD with an agent on Windows
- Linux is prohibited specifically for this project
  - This is because:
    - Early on, essentially Linux-specific code was generated
      - which might have served as a foundation for the Windows version
    - Unix/Linux/Windows branching, real-time audio presence/absence branching, other branching, and the large number of comments associated with them,
      - led to code bloat and became a breeding ground for hallucinations.
      - This resulted in low-quality code with excessive `allow(dead_code)`, ignored tests, duplicate tests, and unnecessary `cfg(windows)` branching.
      - Frequent hallucinations made bug fixing and implementing Windows-specific features impossible.
    - It was found that TDD with an agent on Windows works well for this project.
      - The aforementioned hallucinations and redundancies were also resolved through robust refactoring using TDD.

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow each crate's license