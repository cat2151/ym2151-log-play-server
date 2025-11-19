# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

A server/client that receives YM2151 (OPM) register event logs and performs real-time playback.

## Target Platform

- Windows only
- Linux-specific code is prohibited
    - Due to observed increases in hallucination within this project,
        - Linux-specific code is prohibited.

## Status

This project is currently integrated and used as a library in `cat-play-mml` and `ym2151-tone-editor`.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in a server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output (when verbose mode is enabled)
- Runs as a persistent server, continuing real-time playback in the background
- Allows clients to control playback and quickly switch to different performances
- Utilizes named pipes for server-client communication

## Usage

### Library Usage (Programmatic Control)

Recommended pattern for using this library programmatically:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure the server is ready (automatically installs and starts if needed)
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
1.  Checks if the server is already running.
2.  Installs the server application via cargo if not found in PATH.
3.  Starts the server in background mode.
4.  Waits until the server is ready to accept commands.

This eliminates the need for library users to manually manage the server's lifecycle.

### Interactive Mode (Real-time Register Streaming)

Interactive mode enables continuous audio streaming via real-time register writes. It is ideal for applications like tone editors that require immediate audio feedback and aim to avoid gaps in playback.

#### Basic Interactive Mode

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode
    client::start_interactive()?;
    
    // Write registers with specified timing (in seconds, f64)
    client::write_register(0.0, 0x08, 0x00)?;     // Immediately: All channels key-off
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

For client applications that already have ym2151log-formatted JSON data, the `play_json_interactive()` convenience function eliminates the need to manually implement conversion and timing logic. This function only parses JSON and writes registers; the interactive mode lifecycle is controlled by the user:

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // Ensure server is ready
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // Start interactive mode once
    client::start_interactive()?;
    
    // Send multiple JSONs without stopping - no audio interruption!
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
-   **Continuous Streaming**: No audio interruptions, eliminates silent gaps during parameter changes.
-   **Latency Compensation**: 50ms buffer for jitter correction (Web Audio-style scheduling).
-   **Sample-Accurate Timing**: Float64 seconds (Web Audio API compatible) provides precision up to 1/55930 seconds (1 sample).
-   **Server Time Synchronization**: `get_server_time()` retrieves the server's time coordinate system, allowing for precise scheduling.
-   **No WAV Output**: Optimized for real-time use without file I/O overhead.
-   **Convenience Function**: `play_json_interactive()` handles JSON parsing and time conversion without requiring interactive mode lifecycle management by the user.

**Benefits:**
-   Immediate audio feedback in tone editors (e.g., ym2151-tone-editor).
-   Smooth parameter changes without playback interruption.
-   Ability to send multiple JSONs sequentially without audio gaps.
-   Lower latency compared to static event log playback.
-   Web Audio-compatible time representation for cross-platform consistency.
-   Client controls the start/stop of interactive mode.

Refer to `examples/interactive_demo.rs` and `examples/play_json_interactive_demo.rs` for complete examples.

### Server-Client Mode

#### Starting the Server

Start as a persistent server, waiting for commands:

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

From another terminal, operate in client mode:

```bash
# Play a new JSON file (switch performance)
cargo run --release -- client test_input.json

# Play a new JSON file in verbose mode
cargo run --release -- client test_input.json --verbose

# Stop playback (mute)
cargo run --release -- client --stop

# Shut down the server
cargo run --release -- client --shutdown
```

### Command Line Argument List

```
Usage:
  ym2151-log-play-server server [OPTIONS]           # Server mode
  ym2151-log-play-server client [OPTIONS] [FILE]    # Client mode

Server mode:
  server                    Starts as a server in standby mode
  server --verbose          Starts in verbose log mode (outputs WAV files)
  server --low-quality-resampling  Uses low-quality resampling (linear interpolation, for comparison)

Client mode:
  client <json_file>        Instructs the server to play a new JSON file
  client <json_file> --verbose  Instructs to play with detailed status messages
  client --stop             Instructs the server to stop playback
  client --stop --verbose   Instructs to stop playback with detailed status messages
  client --shutdown         Instructs the server to shut down
  client --shutdown --verbose  Instructs the server to shut down with detailed status messages

Examples:
  # Start server
  ym2151-log-play-server server

  # Start server (verbose, with WAV output)
  ym2151-log-play-server server --verbose

  # Start server (low-quality resampling)
  ym2151-log-play-server server --low-quality-resampling

  # From another terminal: switch performance
  ym2151-log-play-server client test_input.json

  # From another terminal: play in verbose mode
  ym2151-log-play-server client test_input.json --verbose

  # From another terminal: stop playback
  ym2151-log-play-server client --stop

  # From another terminal: shut down server
  ym2151-log-play-server client --shutdown
```

### Usage Scenario Examples

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- server
Server started: \pipe\ym2151-log-play-server.pipe
Server is running. Waiting for client connections...

# Terminal 2: Client operations
$ cargo run --release -- client test_input.json
✅ Play command sent to server

$ cargo run --release -- client --stop
✅ Stop command sent to server

$ cargo run --release -- client --shutdown
✅ Shutdown command sent to server
```

#### Scenario 2: Continuous Playback

```bash
# Start server (Terminal 1)
$ cargo run --release -- server

# Switch songs one after another (Terminal 2)
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

-   Rust 1.70 or later
-   zig cc (used as a C compiler)

## Future Prospects
-   Currently considered stable.
-   Will implement features as needed.

## Project Goals
-   Motivation:
    -   Previous challenge:
        -   Unable to input the next command until playback finished (`ym2151-log-player-rust`).
    -   Solution:
        -   Run as a persistent server controlled by a client.
    -   Use cases:
        -   Provide an experience like MSX's PLAY statement, where the next command can be input while music is playing.
        -   Tone editors, phrase editors:
            -   Utilize the crate as a client.
        -   Integrate the crate into a player, making it both server and client:
            -   First, launch a clone of itself as a background server to start playback, then terminate itself.
                -   *Unlike explicit server usage, the plan is to output messages to a log instead of printing, as logs are easier to track.*
            -   After the server is launched, it sends JSON to the server as a client, then terminates itself.
-   Simple and minimal. To serve as a good reference when building larger projects.
-   If it stops producing sound, I intend to prioritize fixing it to ensure sound plays.

## Project Intent
-   Why this modularization?
    -   To enable GitHub Copilot Coding Agent to perform TDD on layers above this (from MML input to log generation) using GitHub Linux Runner.
    -   This layer (Windows real-time playback and Windows client-server) cannot be TDDed by GitHub Copilot Coding Agent on GitHub Linux Runner, requiring TDD by a Windows local agent, which incurs higher workload.
    -   Therefore, by separating this higher-workload layer, other layers can be developed more efficiently.

## Out of Scope
-   Advanced features
-   Reproduction of existing music

## Development Method
-   TDD with an agent on Windows.
-   Linux is prohibited specifically for this project.
    -   Because:
        -   Early on, effectively Linux-specific code was generated.
            -   Though it might have served as a foundation for the Windows version.
        -   Unix/Linux/Windows branching, real-time audio presence branching, other branches, and the large number of associated comments
            -   Led to code bloat, becoming a hotbed for hallucinations.
            -   Resulted in low-quality code with unnecessary `allow(dead_code)`, ignored tests, duplicate tests, and redundant `cfg(windows)` branches.
            -   Frequent hallucinations prevented bug fixes and implementation of Windows-specific features.
        -   It was discovered that agent-based TDD works well on Windows for this project.
            -   The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.
-   Batch installation of related applications
    -   Convenient for usage and development.
    -   Prerequisite: `cargo install rust-script`
```powershell
rust-script install-ym2151-tools.rs
```

## License

MIT License

## Used Libraries

-   Nuked-OPM: LGPL 2.1
-   Other Rust crates: According to their respective licenses