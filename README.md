# ym2151-log-play-server

Server and client that receive YM2151 (OPM) register event logs and perform real-time playback.

## Target Platforms

- Windows only
- Linux-specific code prohibited
    - In this project, an increase in hallucinations was observed,
        - therefore, Linux-specific code is prohibited.

## Overview

This project is a program that plays YM2151 (OPM) sound chip register event logs.
It operates in both standalone mode and server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client, allowing quick switching to different performances
- Utilizes named pipes for server-client communication

## Usage

### Standalone Mode (Normal Playback)

Play JSON file directly:

```bash
# Build and run
cargo run --release output_ym2151.json

# Or use the already built binary
./target/release/ym2151-log-play-server output_ym2151.json
```

### Server-Client Mode

#### Starting the Server

Run as a persistent server and start playing JSON:

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

### Command-Line Arguments

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server <json_log_file>  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Server shutdown

Options:
  --server <file>    Run as a persistent server and play the specified JSON
  --client <file>    Instruct the server to play a new JSON file
  --client --stop    Instruct the server to stop playback
  --client --shutdown Instruct the server to shut down

Examples:
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

### Example Scenarios

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server output_ym2151.json
Server started: /tmp/ym2151-log-play-server.pipe
Loaded output_ym2151.json (3 events)
Playback started...

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
$ cargo run --release -- --server music1.json

# Switch songs successively (Terminal 2)
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
- zig cc (used as a C compiler)

## Future Outlook
- Currently, the project is considered stable.
- Implementation will occur as needs are identified.

## Project Goals
- Motivation:
  - Previous challenges:
    - Unable to input the next command until playback finishes.
  - Solution:
    - Run as a persistent server, controlled by a client.
  - Use cases:
    - Provide an experience where commands can be input while music is playing, similar to MSX's PLAY statement.
    - From a timbre editor or phrase editor,
      - utilize the crate as a client.
    - Integrate the crate into a player, making it both a server and a client.
      - On first run, launch a duplicate of itself as a server in the background to start playback, then terminate itself.
        - *Conception: Unlike explicit server usage, output messages to logs instead of printing, as logs are easier to track.
      - After the server is launched, act as a client to send JSON to the server, then terminate itself.
- Simple and minimal. Easy to reference when building larger projects.
- If sound stops, the priority will be to restore playback as quickly as possible.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Methodology
- TDD using an agent on Windows.
- For this project specifically, Linux is prohibited.
  - This is because:
    - Early on, virtually Linux-specific code was generated.
      - This might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence/absence branching, other branches, and a large number of associated comments,
      - led to code bloat and became a hotbed for hallucinations.
      - Resulted in low-quality code, with excessive `allow deadcode`, ignored tests, duplicate tests, unnecessary `cfg windows` branching, etc.
      - Hallucinations became frequent, hindering bug fixes and the implementation of Windows-specific features.
    - It was found that TDD with an agent on Windows works well for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Libraries Used

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow individual crate licenses