# ym2151-log-play-server

A server and client that receive YM2151 (OPM) register event logs and perform real-time playback.

## Target Platforms

- Windows only
- Prohibition of Linux-specific code
    - As an increase in hallucinations was observed in this project,
        - Linux-specific code is prohibited.

## Overview

This project is a program that plays back register event logs from the YM2151 (OPM) sound chip.
It operates in both standalone mode and server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Resides as a server and continues real-time playback in the background
- Controlled by a client to quickly switch to different performances
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

Reside as a server and start playing JSON:

```bash
cargo run --release -- --server output_ym2151.json
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

### Command Line Arguments List

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server <json_log_file>  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Shut down server

Options:
  --server <file>    Resides as a server and plays the specified JSON
  --client <file>    Instructs the server to play a new JSON file
  --client --stop    Instructs the server to stop playback
  --client --shutdown Instructs the server to shut down

Example:
  # Play in standalone mode
  ym2151-log-play-server output_ym2151.json

  # Start server
  ym2151-log-play-server --server output_ym2151.json

  # From another terminal: Switch performance
  ym2151-log-play-server --client test_input.json

  # From another terminal: Stop playback
  ym2151-log-play-server --client --stop

  # From another terminal: Shut down server
  ym2151-log-play-server --client --shutdown
```

### Usage Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server output_ym2151.json
Server started: /tmp/ym2151-log-play-server.pipe
output_ym2151.json (3 events) loaded
Playback started...

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
$ cargo run --release -- --server music1.json

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
./target/release/ym2151-log-play-server --server output_ym2151.json
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
- Currently perceived as stable
- Implement as needs are identified

## Project Goals
- Motivation:
  - Previous challenges:
    - Cannot input the next command until playback finishes
  - Solution:
    - Reside as a server and control from a client
  - Use cases:
    - Provide an experience where commands can be input while music is playing, similar to MSX's PLAY statement.
    - From a tone editor, phrase editor:
      - Utilize the crate as a client
    - Embed the crate into a player to make it both a server and a client:
      - For the first run, launch a copy of itself as a server in the background, start playback, and then terminate itself.
        - *Unlike explicit server usage, the idea is to output messages to logs instead of printing, as logs make it easier to understand.*
      - After the server is started, it acts as a client to send JSON to the server and then terminates itself.
- Simple and minimal. Useful as a reference when building larger projects.
- If it stops playing, the intention is to prioritize actions to make it play again.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Methodology
- TDD with an agent on Windows
- Linux is prohibited specifically for this project
  - Because,
    - In the early stages, effectively Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments,
      - Led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code with many unnecessary `allow deadcode`, ignored tests, duplicate tests, and redundant `cfg windows` branches.
      - Hallucinations occurred frequently, making bug fixes and feature implementation for the Windows version impossible.
    - It was found that TDD with an agent on Windows worked well for this project.
      - The aforementioned hallucinations and inefficiencies were also resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow the license of each crate