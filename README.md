# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server/client application that receives YM2151 (OPM) register event logs and performs real-time playback.

## Target Platform

- Windows only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed, therefore,
        - Linux-specific code is prohibited.

## Status

This project is used as a library integrated into `cat-play-mml` and `ym2151-tone-editor`.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (in verbose mode)
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client to quickly switch to different playback
- Utilizes named pipes for server-client communication

## Usage

### Using as a Library (Programmatic Control)

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
    
    // Shut down at the end
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

### Interactive Mode (Real-time Register Streaming)

Interactive mode enables continuous audio streaming through real-time register writes. It is ideal for applications requiring immediate audio feedback and wishing to avoid gaps in playback, such as tone editors.

#### Basic Interactive Mode

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode
    client::start_interactive()?;
    
    // Write registers with specified timing (in seconds, f64)
    client::write_register(0.0, 0x08, 0x00)?;     // Immediate: All channels key off
    client::write_register(0.050, 0x28, 0x48)?;   // +50ms: Set pitch
    client::write_register(0.050, 0x08, 0x78)?;   // +50ms: Channel 0 key on
    client::write_register(0.500, 0x08, 0x00)?;   // +500ms: Key off
    
    // Get server time for precise synchronization
    let server_time = client::get_server_time()?;
    println!("Server Time: {:.6} seconds", server_time);
    
    // Stop interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

#### Interactive Mode with JSON Data (Convenience Function)

For client applications that already have ym2151log format JSON data, the `play_json_interactive()` convenience function eliminates the need to manually implement conversion and timing logic:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Play JSON data directly in interactive mode
    let json_data = r#"{
        "event_count": 3,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x28", "data": "0x48"},
            {"time": 5594, "addr": "0x08", "data": "0x78"}
        ]
    }"#;
    
    // This single function handles:
    // - JSON parsing and validation
    // - Starting interactive mode
    // - Time conversion (samples → seconds)
    // - Sending all register writes
    client::play_json_interactive(json_data)?;
    
    // Wait for playback
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Stop interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

**Key Features:**
- **Continuous Streaming**: Uninterrupted audio, eliminating silences during parameter changes.
- **Latency Compensation**: 50ms buffer for jitter correction (Web Audio-style scheduling).
- **Sample-Accurate Timing**: Provides accuracy up to 1/55930 seconds (1 sample) using Float64 seconds (Web Audio API compatible).
- **Server Time Synchronization**: `get_server_time()` allows retrieving the server's time coordinate system for precise scheduling.
- **No WAV Output**: Optimized for real-time use with no file I/O overhead.
- **Convenience Function**: `play_json_interactive()` automates common processing tasks, reducing code duplication.

**Benefits:**
- Immediate audio feedback in tone editors (e.g., ym2151-tone-editor).
- Smooth parameter changes without playback interruption.
- Lower latency compared to static event log playback.
- Web Audio compatible time representation for cross-platform consistency.
- Simplified client code with the convenience function.

See `examples/interactive_demo.rs` and `examples/play_json_interactive_demo.rs` for complete examples.

### Server-Client Mode

#### Starting the Server

Start as a persistent server in a waiting state:

```bash
# Normal mode (log file only)
cargo run --release -- server

# Verbose mode (detailed logs and WAV output)
cargo run --release -- server --verbose
```

#### Client Operations

Operate from another terminal in client mode:

```bash
# Play a new JSON file (switch playback)
cargo run --release -- client test_input.json

# Stop playback (mute)
cargo run --release -- client --stop

# Shut down the server
cargo run --release -- client --shutdown
```

### Command Line Argument List

```
Usage:
  ym2151-log-play-server server [--verbose]         # Server mode
  ym2151-log-play-server client <json_log_file>     # Play new JSON
  ym2151-log-play-server client --stop              # Stop playback
  ym2151-log-play-server client --shutdown          # Shutdown server

Options:
  server           Start as a server in a waiting state
  server --verbose Start server in verbose log mode (outputs WAV files)
  client <file>    Instruct server to play a new JSON file
  client --stop    Instruct server to stop playback
  client --shutdown Instruct server to shut down server

Examples:
  # Start server
  ym2151-log-play-server server

  # Start server (verbose, with WAV output)
  ym2151-log-play-server server --verbose

  # From another terminal: Switch playback
  ym2151-log-play-server client test_input.json

  # From another terminal: Stop playback
  ym2151-log-play-server client --stop

  # From another terminal: Shut down server
  ym2151-log-play-server --client --shutdown
```

### Usage Scenario Examples

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server
Server started: /tmp/ym2151-log-play-server.pipe
Server started. Waiting for client connections...

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

# Switch songs sequentially (Terminal 2)
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
- Currently, development is considered stable.
- Implementation will occur as needs are identified.

## Project Goals
- Motivation:
  - Previous challenges:
    - Unable to input the next command until playback finished.
  - Solution:
    - Run as a persistent server, controlled by a client.
  - Use cases:
    - Provide an experience similar to MSX's PLAY statement, allowing command input during playback.
    - From a tone editor, phrase editor:
      - Utilize the crate as a client.
    - Integrate the crate into a player, making it both a server and client.
      - On first run, launch a clone of itself as a background server to start playback, then terminate itself.
        - *Unlike explicit server usage, the idea is to output messages to a log instead of print, as logs are easier to track.
      - After the server starts, act as a client, send JSON to the server, and then terminate itself.
- Simple and minimal, to serve as a reference for building larger projects.
- If it stops making sound, the intention is to prioritize getting it to work again.

## Project Rationale
- Why was this module split designed this way?
  - To enable GitHub Copilot Coding Agent to perform TDD on the layers above this one (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client/server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner; instead, TDD by a local Windows agent is required, making the workload somewhat higher.
  - Therefore, to isolate this higher-workload layer and enable efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with agent on Windows
- Linux is prohibited specifically for this project.
  - Because:
    - In the early stages, virtually Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments,
      - led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code with excessive `allow deadcode`, ignored tests, duplicate tests, and unnecessary `cfg windows` branching.
      - Frequent hallucinations occurred, preventing bug fixes and the implementation of Windows-specific features.
    - It was found that TDD with an agent on Windows works well for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Subject to their respective licenses