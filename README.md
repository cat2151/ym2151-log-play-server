# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server and client that receive YM2151 (OPM) register event logs and perform real-time playback.

## Target Platforms

- Windows only
- Linux-specific code prohibited
    - Due to an observed increase in hallucinations within this project,
        - Linux-specific code is prohibited.

## Status

This is currently used as a library integrated into `cat-play-mml` and `ym2151-tone-editor`.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in a server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (when verbose)
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client to quickly switch to different performances
- Uses named pipes for server-client communication

## Usage

### Usage as a Library (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server readiness (automatically installs and launches if necessary)
    client::ensure_server_ready("cat-play-mml")?;
    
    // Send JSON data
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
1. Checks if the server is already running.
2. Installs the server application via cargo if not found in PATH.
3. Launches the server in background mode.
4. Waits until the server is ready to accept commands.

This eliminates the need for library users to manually manage the server's lifecycle.

### Interactive Mode (Real-time Register Streaming)

Interactive mode enables continuous audio streaming via real-time register writes. It is ideal for applications like tone editors that require immediate audio feedback and wish to avoid playback gaps.

#### Basic Interactive Mode

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server readiness
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
    println!("Server time: {:.6} seconds", server_time);
    
    // Stop interactive mode
    client::stop_interactive()?;
    
    Ok(())
}
```

#### Interactive Mode with JSON Data (Convenience Function)

For client applications already holding ym2151log-formatted JSON data, the `play_json_interactive()` convenience function eliminates the need to manually implement conversion and timing logic. This function only parses the JSON and writes registers; the user controls the interactive mode's lifecycle:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server readiness
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode once
    client::start_interactive()?;
    
    // Send multiple JSONs without stopping - No audio interruptions!
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
- **Continuous Streaming**: No audio interruptions, eliminates silent gaps during parameter changes.
- **Latency Compensation**: 50ms buffer for jitter correction (Web Audio-style scheduling).
- **Sample-Accurate Timing**: Float64 seconds (Web Audio API compatible) provides precision up to 1/55930 seconds (1 sample).
- **Server Time Synchronization**: `get_server_time()` retrieves the server's time coordinate system, allowing for precise scheduling.
- **No WAV Output**: Optimized for real-time without file I/O overhead.
- **Convenience Function**: `play_json_interactive()` handles JSON parsing and time conversion without requiring interactive mode lifecycle management by the user.

**Benefits:**
- Immediate audio feedback in tone editors (e.g., ym2151-tone-editor).
- Smooth parameter changes without interrupting playback.
- Ability to send multiple JSONs consecutively without audio breaks.
- Lower latency compared to static event log playback.
- Web Audio-compatible time representation for cross-platform consistency.
- Client controls start/stop of interactive mode.

See `examples/interactive_demo.rs` and `examples/play_json_interactive_demo.rs` for complete examples.

### Server-Client Mode

#### Starting the Server

Launch as a persistent server, waiting for commands:

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

### Command-Line Arguments

```
Usage:
  ym2151-log-play-server server [--verbose]         # Server mode
  ym2151-log-play-server client <json_log_file>     # Play new JSON
  ym2151-log-play-server client --stop              # Stop playback
  ym2151-log-play-server client --shutdown          # Shut down server

Options:
  server           Start as a server in standby mode
  server --verbose Start server in verbose logging mode (outputs WAV files)
  client <file>    Instruct the server to play a new JSON file
  client --stop    Instruct the server to stop playback
  client --shutdown Instruct the server to shut down

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

### Usage Scenarios

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
- zig cc (used as a C compiler)

## Future Outlook
- Currently, the project is considered stable.
- Implement new features as needed.

## Project Goals
- Motivation:
  - Past Challenges:
    - Unable to input the next command until the current playback finished.
  - Solution:
    - Run as a persistent server, controlled by a client.
  - Use Cases:
    - Provide an experience similar to MSX's PLAY statement, where you can input the next command while music is playing.
    - Utilize the crate as a client from tone editors and phrase editors.
    - Integrate the crate into a player to make it both a server and a client.
      - On the first run, launch a clone of itself as a background server to start playback, then terminate itself.
        - *Note: Unlike explicit server usage, the concept is to output messages to a log instead of printing them, as logs are easier to track.*
      - After the server is launched, it acts as a client to send JSON to the server, then terminates itself.
- Simple and minimal. Intended to be easy to reference when building larger projects.
- If it stops playing, the intention is to prioritize getting it to play again as much as possible.

## Project Rationale
- Why this module split?
  - To enable GitHub Copilot Coding Agent to perform TDD on the layers above this (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback, and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner, requiring TDD by a Windows local agent instead, which increases the workload.
  - Therefore, this workload-heavy layer was separated to allow other layers to be developed more efficiently.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with an agent on Windows.
- Linux prohibited for this project only.
  - This is because:
    - Early on, effectively Linux-specific code was generated.
      - Although it might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, real-time audio presence branching, and other branches, along with extensive comments,
      - Led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code with many unnecessary `allow deadcode`, `ignored` tests, duplicate tests, and redundant `cfg windows` branches.
      - Frequent hallucinations prevented bug fixes and the implementation of Windows-specific features.
    - It was discovered that TDD with an agent works well on Windows for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Subject to each crate's license