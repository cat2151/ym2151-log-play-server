# ym2151-log-play-server

A server and client that receive YM2151 (OPM) register event logs and perform real-time playback.

## Target Platform

- Windows only
- Linux-specific code prohibited
    - In this project, an increase in hallucinations was observed, therefore:
        - Linux-specific code is prohibited.

## Overview

This project is a program that plays back register event logs from a YM2151 (OPM) sound chip.
It operates in both standalone and server-client modes.

### Key Features

- Real-time performance of JSON music data
- WAV file output
- Runs persistently as a server, continuing real-time playback in the background
- Controlled by a client for quick switching to different performances
- Utilizes named pipes for server-client communication

## Usage

### Standalone Mode (Normal Playback)

Directly play a JSON file:

```bash
# Build and run
cargo run --release output_ym2151.json

# Or use an already built binary
./target/release/ym2151-log-play-server output_ym2151.json
```

### Server-Client Mode

#### Starting the Server

Launch as a persistent server, in a waiting state:

```bash
cargo run --release -- --server
```

#### Client Operations

Operate from another terminal in client mode:

```bash
# Play a new JSON file (switch performance)
cargo run --release -- --client test_input.json

# Stop playback (mute)
cargo run --release -- --client --stop

# Shut down the server
cargo run --release -- --client --shutdown
```

### Command Line Arguments

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server                  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Shut down server

Options:
  --server           Start as a server in a waiting state
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

  # From another terminal: Stop performance
  ym2151-log-play-server --client --stop

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

## Future Prospects
- Currently deemed stable.
- Will implement new features as needed.

## Project Goals
- Motivation:
  - Previous challenges:
    - Inability to input the next command until the current performance ended.
  - Solution:
    - Run as a persistent server, controlled by a client.
  - Use cases:
    - Provide an experience similar to MSX's PLAY statement, allowing command input during performance.
    - Utilize the crate as a client from a timbre editor or phrase editor.
    - Embed the crate into a player to make it both a server and a client:
      - Initially, launch a copy of itself as a server in the background to start playback, then terminate itself.
        - *Note: Unlike explicit server usage, the plan is to output messages to a log instead of printing, as logs are easier to track.*
      - After the server is launched, it sends JSON to the server as a client, then terminates itself.
- Keep it simple and minimal. Easier to reference when building larger projects.
- If sound stops, I will prioritize actions to restore it.

## Project Intent
- Why this module separation?
  - To enable GitHub Copilot Coding Agent to perform TDD on the layers above this one (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDD'd by GitHub Copilot Coding Agent on GitHub Linux Runner; instead, TDD by a Windows local agent is required, which implies a higher workload.
  - Therefore, this higher-workload layer is separated to allow efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing music

## Development Method
- TDD with an agent on Windows.
- Linux is prohibited specifically for this project.
  - Because:
    - In the early stages, virtually Linux-specific code was generated.
      - Although it might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, real-time audio presence branching, other branches, and the accompanying large number of comments
      - Led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code with unnecessary `allow deadcode`, `test ignored`, duplicate tests, and useless `cfg windows` branches.
      - Frequent hallucinations occurred, preventing bug fixes and implementation of Windows-specific features.
    - It was found that agent-based TDD on Windows works well for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Subject to individual crate licenses