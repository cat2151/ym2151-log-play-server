# ym2151-log-play-server

A server and client for receiving YM2151 (OPM) register event logs and playing them in real-time.

## Target Platform

- Windows only
- Linux-specific code prohibited
    - In this project, an increase in hallucinations was observed, therefore
        - Linux-specific code is prohibited.

## Overview

This project is a program that plays back YM2151 (OPM) sound chip register event logs.
It operates in both standalone and server-client modes.

### Key Features

- Real-time playback of JSON music data
- WAV file output
- Runs as a persistent server, continuing real-time playback in the background
- Controlled by a client for quick switching to different performances
- Utilizes named pipes for server-client communication

## How to Use

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

Run as a persistent server in idle state:

```bash
cargo run --release -- --server
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

### Command Line Argument List

```
Usage:
  ym2151-log-play-server <json_log_file>           # Standalone mode
  ym2151-log-play-server --server                  # Server mode
  ym2151-log-play-server --client <json_log_file>  # Play new JSON
  ym2151-log-play-server --client --stop           # Stop playback
  ym2151-log-play-server --client --shutdown       # Shutdown server

Options:
  --server           Run as a server in idle state
  --client <file>    Instruct the server to play a new JSON file
  --client --stop    Instruct the server to stop playback
  --client --shutdown Instruct the server to shut down

Examples:
  # Play in standalone mode
  ym2151-log-play-server output_ym2151.json

  # Start server
  ym2151-log-play-server --server

  # From another terminal: switch performance
  ym2151-log-play-server --client test_input.json

  # From another terminal: stop playback
  ym2151-log-play-server --client --stop

  # From another terminal: terminate server
  ym2151-log-play-server --client --shutdown
```

### Usage Scenario Examples

#### Scenario 1: Basic Usage

```bash
# Terminal 1: Start server
$ cargo run --release -- --server
サーバーを起動しました: /tmp/ym2151-log-play-server.pipe
サーバーが起動しました。クライアントからの接続を待機中...

# Terminal 2: Client operations
$ cargo run --release -- --client test_input.json
✅ サーバーに PLAY コマンドを送信しました

$ cargo run --release -- --client --stop
✅ サーバーに STOP コマンドを送信しました

$ cargo run --release -- --client --shutdown
✅ サーバーに SHUTDOWN コマンドを送信しました
```

#### Scenario 2: Continuous Playback

```bash
# Start server (Terminal 1)
$ cargo run --release -- --server

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
- Currently considered stable.
- Will implement as needed when new requirements arise.

## Project Goals
- Motivation:
  - Previous challenge:
    - Unable to input the next command until playback finished.
  - Solution:
    - Run as a persistent server, controlled by a client.
  - Use cases:
    - Provide an experience similar to MSX's PLAY statement, where commands can be input while music is playing.
    - Utilize the crate as a client from a sound editor or phrase editor.
    - Embed the crate into a player, making it both a server and a client.
      - First, start a clone of itself as a server in the background to begin playback, then the original terminates.
        - *Instead of `print`, output messages to a `log` in this case, as `log` would make it easier to understand.*
      - After the server starts, it acts as a client to send JSON to the server, then it terminates.
- Simple and minimal, to serve as a reference for building larger projects.
- If it stops producing sound, the intention is to prioritize actions to get it working again.

## Project Intent
- Why this module split?
  - To enable GitHub Copilot Coding Agent to perform TDD on layers above this (from MML input to log generation) using GitHub Linux Runner.
  - This layer (Windows real-time playback and Windows client-server) cannot be TDDed by GitHub Copilot Coding Agent on GitHub Linux Runner. Instead, it requires TDD by a Windows local agent, which has a higher workload.
  - Therefore, this layer with higher workload is isolated to allow efficient development of other layers.

## Out of Scope
- Advanced features
- Reproduction of existing songs

## Development Method
- TDD with an agent on Windows.
- Linux is prohibited specifically for this project.
  - Because:
    - Early on, essentially Linux-specific code was generated.
      - It might have served as a foundation for the Windows version.
    - Unix/Linux/Windows branching, real-time audio presence branching, other branching, and a large number of associated comments,
      - Led to code bloat and became a breeding ground for hallucinations.
      - Resulted in low-quality code with unnecessary `allow deadcode`, ignored tests, duplicate tests, and useless `cfg windows` branches.
      - Frequent hallucinations made bug fixing and Windows feature implementation impossible.
    - It was found that TDD with an agent on Windows worked well for this project.
      - The aforementioned hallucinations and inefficiencies were resolved through robust refactoring using TDD.

## License

MIT License

## Used Libraries

- Nuked-OPM: LGPL 2.1
- Other Rust crates: According to each crate's license