# ym2151-log-play-server

A server and client for receiving YM2151 (OPM) register event logs and performing real-time playback.

## Target Platform

- Windows only
- Prohibition of Linux-specific code
    - In this project, an increase in hallucinations was observed,
        - therefore, Linux-specific code is prohibited.

## Overview

This project is a program that plays YM2151 (OPM) sound chip register event logs.
It operates in both standalone mode and server-client mode.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client to quickly switch to another performance
- Utilizes named pipes for server-client communication

## Usage

### Standalone Mode (Normal Playback)

Play a JSON file directly:

```bash
# Build and run
cargo run --release output_ym2151.json

# Or use the already built binary
./target/release/ym2151-log-play-server output_ym2151.json
```

### Server-Client Mode

#### Starting the Server

Start the server as a persistent process and begin playing a JSON file:

```bash
cargo run --release -- --server output_ym2151.json
```

#### Client Operations

From another terminal, operate in client mode:

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
  ym2151-log-play-server --server <json_log_file>  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Shut down server

Options:
  --server <file>    Run as a persistent server, playing the specified JSON
  --client <file>    Instruct the server to play a new JSON file
  --client --stop    Instruct the server to stop playback
  --client --shutdown Instruct the server to shut down

Examples:
  # Play in standalone mode
  ym2151-log-play-server output_ym2151.json

  # Start server
  ym2151-log-play-server --server output_ym2151.json

  # From another terminal: Switch playback
  ym2151-log-play-server --client test_input.json

  # From another terminal: Stop playback
  ym2151-log-play-server --client --stop

  # From another terminal: Terminate server
  ym2151-log-play-server --client --shutdown
```

### Usage Scenario Examples

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start Server
$ cargo run --release -- --server output_ym2151.json
サーバーを起動しました: /tmp/ym2151_server.pipe  # Server started: /tmp/ym2151_server.pipe
output_ym2151.json (3 イベント) を読み込みました # Loaded output_ym2151.json (3 events)
演奏を開始しました...                          # Playback started...

# Terminal 2: Client Operation
$ cargo run --release -- --client test_input.json
✅ サーバーに PLAY コマンドを送信しました # ✅ Sent PLAY command to server

$ cargo run --release -- --client --stop
✅ サーバーに STOP コマンドを送信しました # ✅ Sent STOP command to server

$ cargo run --release -- --client --shutdown
✅ サーバーに SHUTDOWN コマンドを送信しました # ✅ Sent SHUTDOWN command to server
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

## Project Goals
- Motivation:
  - Past challenges:
    - Unable to input the next command until playback finishes
  - Solution:
    - Run as a persistent server, controlled by a client
  - Use cases:
    - Provide an experience where you can enter the next command while music is playing, similar to MSX's PLAY statement.
    - Utilize the crate as a client from timbre editors and phrase editors.
    - Integrate the crate into a player to make it both a server and client.
      - On the first run, launch a clone of itself as a server in the background, start playback, and then exit.
        - (Unlike explicit server use, a concept to output messages to logs instead of printing, as logs would be easier to understand.)
      - After the server starts, it sends JSON to the server as a client and then exits.
- Simple and minimal. Easy to reference when building larger projects.
- If it stops playing, the intention is to prioritize making it play again as much as possible.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with an agent on Windows.
- Linux is forbidden specifically for this project.
  - This is because:
    - In the early stages, effectively Linux-specific code was generated.
      - It might have been useful as a foundation for the Windows version.
    - Unix/Linux/Windows branching, realtime-audio presence branching, other branching, and a large number of associated comments,
      - led to code bloat and became a breeding ground for hallucinations.
      - This resulted in low-quality code, with excessive `allow deadcode`, ignored tests, duplicate tests, and unnecessary `cfg windows` branching.
      - Frequent hallucinations occurred, making bug fixes and Windows-specific feature implementations impossible.
    - It was found that TDD with an agent worked well for this project on Windows.
      - The aforementioned hallucinations and inefficiencies were also resolved through robust refactoring using TDD.

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: Follow each crate's license