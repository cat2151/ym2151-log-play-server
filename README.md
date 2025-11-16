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

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in both standalone and server-client modes.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client, allowing quick switching to different playback
- Utilizes named pipes for server-client communication

## Usage

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

Launch as a persistent server in a waiting state:

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

### Command Line Argument List

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server                  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play a new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Shutdown server

Options:
  --server           Launch as a server in a waiting state
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
```

### Usage Scenario Examples

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server
Server started: /tmp/ym2151-log-play-server.pipe
Server started. Waiting for client connection...

# Terminal 2: Operate from client
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
- zig cc (used as a C compiler)

## Future Prospects
- Currently, development is considered stable.
- Will implement new features as needed.

## Project Goals
- Motivation:
  - Previous Challenges:
    - Unable to input the next command until playback finishes.
  - Solution:
    - Run as a persistent server controlled by a client.
  - Use Cases:
    - Provide an experience where commands can be input while music is playing, similar to MSX's PLAY statement.
    - From a timbre editor or phrase editor,
      - Utilize the crate as a client.
    - Integrate the crate into a player to make it a server and client.
      - On the first run, launch a clone of itself as a background server to start playback, then terminate the original process.
        - *Unlike explicit server usage, the idea is to output messages to logs instead of stdout for easier understanding.
      - After the server starts, act as a client to send JSON to the server, then terminate.
- Simple and minimal. Easy to reference when building larger projects.
- If it stops making sound, the intention is to prioritize getting it to play again.

## Project Intent
- Why was such module separation performed?
  - To enable GitHub Copilot Coding Agent to perform TDD on layers above this one (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner, and instead requires TDD by a local Windows agent, resulting in higher workload.
  - Therefore, to isolate this high-workload layer and enable efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with an agent on Windows
- Linux is prohibited specifically for this project.
  - This is because:
    - In the early stages, essentially Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, real-time audio presence branching, other branching, and a large number of associated comments,
      - led to code bloat and became a hotbed for hallucinations.
      - Resulted in low-quality code, including unnecessary `allow deadcode`, ignored tests, duplicate tests, and redundant `cfg(windows)` branching.
      - Frequent hallucinations made bug fixes and feature implementation for the Windows version impossible.
    - It was found that agent-based TDD on Windows works well for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow their respective licenses