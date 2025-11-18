# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

Server/client that receives YM2151 (OPM) register event logs and performs real-time playback.

## Supported Platforms

- Windows Only
- Prohibition of Linux-specific code
  - To mitigate an observed increase in AI code generation "hallucinations" (incorrect or irrelevant code) within this project, Linux-specific code is prohibited.

## Status

It is integrated and used as a library in projects like `cat-play-mml` and `ym2151-tone-editor`.

## Overview

This project is a program for playing back YM2151 (OPM) sound chip register event logs. It operates in server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (when in verbose mode)
- Runs as a resident server, continuing real-time playback in the background
- Controlled by clients to quickly switch to different performances
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
    
    // Shut down at termination
    client::shutdown_server()?;
    
    Ok(())
}
```

The `ensure_server_ready()` function automatically performs the following actions to provide a seamless development experience:
1. Checks if the server is already running
2. Installs the server application via cargo if not found in PATH
3. Starts the server in background mode
4. Waits until the server is ready to accept commands

This eliminates the need for library users to manually manage the server's lifecycle.

### Interactive Mode (for Real-Time Register Streaming)

Interactive mode enables continuous audio streaming with real-time register writes, ideal for applications like tone editors that need immediate audio feedback without playback gaps.

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode
    client::start_interactive()?;
    
    // Send register writes with timing
    client::write_register(0, 0x08, 0x00)?;     // Immediate: Key off all channels
    client::write_register(50, 0x28, 0x48)?;    // +50ms: Set note frequency
    client::write_register(50, 0x08, 0x78)?;    // +50ms: Key on channel 0
    client::write_register(500, 0x08, 0x00)?;   // +500ms: Key off
    
    // Stop interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

**Key Features:**
- **Continuous Streaming**: Audio never stops, eliminating silence gaps during parameter changes
- **Latency Compensation**: 50ms buffer for jitter compensation (Web Audio-style scheduling)
- **Time Scheduling**: Register writes scheduled with millisecond precision
- **No WAV Output**: Optimized for real-time use without file I/O overhead

**Benefits:**
- Instant audio feedback for tone editors (e.g., ym2151-tone-editor)
- Smooth parameter changes without playback interruption
- Reduced latency compared to static event log playback

See `examples/interactive_demo.rs` for a complete example.

### Server-Client Mode

#### Starting the Server

Starts as a resident server in a waiting state:

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

### Command Line Argument Reference

```
Usage:
  ym2151-log-play-server server [--verbose]         # Server mode
  ym2151-log-play-server client <json_log_file>     # Play a new JSON
  ym2151-log-play-server client --stop              # Stop playback
  ym2151-log-play-server client --shutdown          # Shut down server

Options:
  server           Start as a resident server in a waiting state
  server --verbose Start server in verbose log mode (outputs WAV files)
  client <file>    Instruct server to play a new JSON file
  client --stop    Instruct server to stop playback
  client --shutdown Instruct server to shut down

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

### Example Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server
Server started: /tmp/ym2151-log-play-server.pipe
Server started. Waiting for client connections...

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

## Future Prospects
- Currently, the project is considered stable.
- Implement features as needed.

## Project Goals
- Motivation:
  - Past Challenges:
    - Unable to input the next command until playback finished
  - Solution:
    - Run as a resident server, controlled by clients
  - Use Cases:
    - Provide an experience similar to MSX's PLAY statement, allowing command input during playback
    - From tone editors and phrase editors:
      - Utilize the crate as a client
    - Integrate the crate into a player to make it both server and client
      - On first run, start a duplicate of itself as a background server to begin playback, then the original process exits.
        - *Note: Unlike explicit server usage, this envisions outputting messages to a log instead of print, as logs are easier to track.
      - After the server starts, act as a client to send JSON to the server, then the client process exits.
- Simple and minimal. Designed to be easy to reference when building larger projects.
- If it stops playing sound, the intention is to prioritize actions to restore playback as quickly as possible.

## Project Rationale
- Why was this module division chosen?
  - To enable the GitHub Copilot Coding Agent to perform TDD on the layers above this one (from MML input to log generation) using GitHub Linux Runner.
  - This specific layer (Windows real-time playback and Windows client-server) cannot be TDD'd by the GitHub Copilot Coding Agent on GitHub Linux Runner. Instead, TDD by a local Windows agent is required, which results in a slightly higher workload.
  - Therefore, this higher-workload layer was separated to allow for more efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with agent on Windows
- Linux is prohibited specifically for this project.
  - Because:
    - In the initial stages, essentially Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, branching based on the presence of real-time audio, other branching, and the large amount of associated comments,
      - led to code bloat and became a breeding ground for hallucinations.
      - This resulted in low-quality code, with excessive `allow(dead_code)`, ignored tests, duplicate tests, unnecessary `cfg(windows)` branching, etc.
      - Hallucinations became frequent, making bug fixing and Windows-specific feature implementation difficult.
    - It was found that TDD with an agent works well on Windows for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: According to each crate's license