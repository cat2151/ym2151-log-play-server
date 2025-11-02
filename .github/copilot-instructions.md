# GitHub Copilot Instructions for ym2151-log-player-rust

## Project Overview

This is a Rust implementation of a YM2151 (OPM) chip register event log player. It reads JSON event logs and performs real-time audio playback and WAV file output using the Nuked-OPM emulator.

This is a Rust port of the original C implementation: https://github.com/cat2151/ym2151-log-player

### Key Features
- JSON event log parsing with hex string support ("0x08" format)
- Real-time audio playback via cpal
- WAV file output via hound
- Sample rate conversion (55930 Hz → 48000 Hz) using rubato
- Nuked-OPM emulation via FFI

## Build Instructions

### Prerequisites
- Rust 1.70 or later
- zig cc (required for C compilation)
- **DO NOT USE**: mingw, msys2, or MSVC

### Building the Project

```bash
# Standard build
cargo build

# Release build
cargo build --release

# Run the program
cargo run -- sample_events.json
```

### Cross-compilation to Windows (from Linux)

```bash
# Set up zig cc
export CC="zig cc -target x86_64-windows"
export AR="zig ar"

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test integration_test

# Run with output
cargo test -- --nocapture
```

**Note**: Build may fail on Linux due to ALSA dependencies in cpal. This is expected in CI environments. Tests can still be run on platforms with proper audio support.

## Code Style and Conventions

### General Guidelines
- Follow standard Rust conventions (rustfmt)
- Use descriptive variable names
- Add comments for complex FFI interactions
- Document unsafe code blocks with safety justifications
- Keep functions focused and small

### Error Handling
- Use `anyhow::Result` for error propagation in application code
- Use `Result<T, E>` with custom error types for library code
- Provide context with error messages

### FFI Safety
- All unsafe FFI calls must be wrapped in safe Rust APIs
- Document safety requirements for unsafe functions
- Minimize the unsafe code surface area
- All FFI bindings are in `src/opm_ffi.rs`
- Safe wrappers are in `src/opm.rs`

### Testing
- Write unit tests for each module
- Place integration tests in `tests/` directory
- Use fixtures in `tests/fixtures/` for test data
- Test both success and error cases

## Project Structure

```
ym2151-log-player-rust/
├── src/
│   ├── main.rs          # Entry point with phase demos
│   ├── lib.rs           # Library exports
│   ├── opm_ffi.rs       # Raw FFI bindings to Nuked-OPM
│   ├── opm.rs           # Safe Rust wrapper for OPM chip
│   ├── events.rs        # JSON event parsing with hex support
│   └── (more modules coming in future phases)
├── tests/
│   ├── integration_test.rs  # Integration tests
│   └── fixtures/            # Test JSON files
├── opm.c                # Nuked-OPM C implementation
├── opm.h                # Nuked-OPM header
├── build.rs             # Build script for compiling opm.c
├── sample_events.json   # Sample event log
├── Cargo.toml           # Dependencies (versions pinned for reproducibility)
└── IMPLEMENTATION_PLAN.md  # Detailed implementation roadmap
```

## Technology Stack

### Rust Dependencies
- **serde + serde_json**: JSON deserialization with custom hex string parsing
- **cpal**: Cross-platform audio I/O (pinned to 0.15.3 for stability)
- **hound**: WAV file reading/writing
- **rubato**: High-quality sample rate conversion (pinned to 0.14.1)
- **anyhow**: Error handling with context

### Build Dependencies
- **cc**: Compiles opm.c during build via build.rs

### C Library (via FFI)
- **Nuked-OPM**: YM2151 emulator (LGPL 2.1 licensed)
  - Source: opm.c, opm.h in project root
  - Compiled with zig cc and linked via build.rs

## Development Workflow

### Implementation Phases
The project follows a phased implementation approach documented in `IMPLEMENTATION_PLAN.md`:

- **Phase 0**: ✅ Project initialization
- **Phase 1**: ✅ Nuked-OPM FFI bindings (completed)
- **Phase 2**: ✅ JSON event loading (completed)
- **Phase 3**: Event processing engine (planned)
- **Phase 4**: WAV file output (planned)
- **Phase 5**: Real-time audio playback (planned)
- **Phase 6**: Main application integration (planned)
- **Phase 7**: Windows build and testing (planned)

### Current Status
Phases 1 and 2 are complete. The project can:
- Initialize Nuked-OPM chip
- Generate audio samples
- Load and parse JSON event files with hex strings

### Adding New Features
1. Check the phase plan in `IMPLEMENTATION_PLAN.md`
2. Ensure dependencies are in `Cargo.toml`
3. Write tests first when possible
4. Implement in appropriate module
5. Update main.rs if adding new phase demo
6. Run tests to verify

## Important Implementation Details

### JSON Event Format
Events use hex strings for addresses and data:
```json
{
  "event_count": 2,
  "events": [
    {"time": 0, "addr": "0x08", "data": "0x00"},
    {"time": 2, "addr": "0x20", "data": "0xC7"}
  ]
}
```

**Note**: The current `sample_events.json` file contains pass2 format events with `is_data` field already set. This field is optional and ignored during parsing. Future implementation phases will convert pass1 events (simple register writes without `is_data`) into pass2 events automatically.

### Hex String Parsing
Use custom deserializer in `events.rs`:
```rust
#[serde(deserialize_with = "parse_hex_string")]
pub addr: u8,
```

### Event Processing (Planned for Phase 3)
The player will process events as follows:
- **Input**: Pass1 format events (simple register writes: `{"time": 0, "addr": "0x08", "data": "0x00"}`)
- **Processing**: Convert to pass2 format (address write + data write pairs)
- **Timing**: Insert DELAY_SAMPLES (2 samples) between address and data writes
- **OPM Ports**: Port 0 = address register, Port 1 = data register

Current Status: The `sample_events.json` file contains pass2 format data for testing purposes. Phase 3 will implement the pass1→pass2 conversion logic.

### Audio Specifications
- OPM internal rate: 55930 Hz
- Output rate: 48000 Hz (requires resampling)
- Format: 16-bit signed stereo
- Channels: 2 (stereo)

### Safety and Security
- All `unsafe` code must be documented
- FFI boundaries are the only unsafe areas
- Use safe wrappers for all public APIs
- Validate all external inputs (JSON, user args)

## Common Tasks

### Adding a New Module
1. Create `src/module_name.rs`
2. Add to `src/lib.rs`: `pub mod module_name;`
3. Update main.rs to use if needed
4. Write tests in same file or `tests/`

### Working with FFI
1. Add C function declaration to `src/opm_ffi.rs`
2. Create safe wrapper in `src/opm.rs`
3. Document safety requirements
4. Test with unit tests

### Adding Dependencies
1. Update `Cargo.toml` with specific version
2. Pin versions for audio/signal processing crates
3. Check license compatibility (project is MIT)
4. Document why dependency is needed

## References

- Original implementation: https://github.com/cat2151/ym2151-log-player
- Nuked-OPM: https://github.com/nukeykt/Nuked-OPM
- Implementation plan: See `IMPLEMENTATION_PLAN.md` for detailed phase breakdown
- YM2151 specs: Yamaha YM2151 datasheet

## Language and Documentation

- Code comments: Use English for code comments and documentation
- README files: Japanese (as per project convention)
- User-facing messages: Japanese
- Technical documentation: English is acceptable
- Commit messages: English preferred

## Code Quality and Linting

### Running Linters

Always run linters before committing code:

```bash
# Format code with rustfmt
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run Clippy for code quality checks
cargo clippy --all-targets

# Run Clippy with warnings as errors (for CI)
cargo clippy --all-targets -- -D warnings
```

### Pre-commit Checklist

Before committing or requesting code review:

1. **Format code**: Run `cargo fmt` to ensure consistent formatting
2. **Fix linting issues**: Run `cargo clippy` and address warnings
3. **Build successfully**: Run `cargo build` (or `cargo build --release`)
4. **Run tests**: Execute `cargo test` to ensure all tests pass
5. **Test with features**: If modifying audio code, test with `cargo test --features realtime-audio`
6. **Update documentation**: If adding public APIs, update doc comments

### Continuous Integration

Currently, this repository doesn't have automated CI/CD workflows. When contributing:

- Ensure your local build and tests pass before pushing
- Code review is manual; maintainers will check for:
  - Proper formatting (rustfmt)
  - No clippy warnings
  - All tests passing
  - Documentation updated for new features

## Acceptance Criteria for Changes

### Bug Fixes

A bug fix is complete when:
- [ ] The bug is reproducible with a test case
- [ ] The fix resolves the issue without breaking existing functionality
- [ ] Tests are added to prevent regression
- [ ] Code passes all linting checks
- [ ] Documentation is updated if the bug was in documented behavior

### New Features

A new feature is complete when:
- [ ] Implementation matches the specification in IMPLEMENTATION_PLAN.md
- [ ] All public APIs have documentation comments
- [ ] Unit tests cover the new functionality
- [ ] Integration tests verify end-to-end behavior
- [ ] Code follows project conventions and passes linting
- [ ] README.md or relevant documentation is updated
- [ ] No new clippy warnings are introduced

### Refactoring

A refactoring is complete when:
- [ ] Behavior is unchanged (all existing tests still pass)
- [ ] Code is more maintainable/readable
- [ ] No new warnings or errors introduced
- [ ] Performance is not negatively impacted (if relevant)

### Documentation Updates

Documentation changes are complete when:
- [ ] Information is accurate and up-to-date
- [ ] Examples compile and work correctly
- [ ] Language conventions are followed (see Language and Documentation section)
- [ ] No broken links or references

## Working with GitHub Copilot

### Well-Suited Tasks for Copilot

Copilot works best on issues that are:

- **Well-scoped**: Clear description of what needs to change
- **Specific**: Affected files or components are identified
- **Testable**: Expected behavior can be verified with tests
- **Incremental**: Changes build on existing code rather than large rewrites

Examples of good Copilot tasks:
- "Add a new JSON field to the event format with parsing and tests"
- "Fix clippy warning in src/player.rs about unused variable"
- "Add validation for sample rate values in AudioResampler"
- "Update documentation for the EventLog::from_file method"

### Tasks Less Suited for Copilot

Avoid assigning to Copilot:
- Major architectural changes or redesigns
- Tasks requiring domain expertise in audio processing or YM2151 chip behavior
- Issues with ambiguous requirements or unclear success criteria
- Tasks that require creative problem-solving without clear constraints

### Providing Feedback on Copilot PRs

When reviewing Copilot-generated code:

1. Check that all acceptance criteria are met
2. Verify tests are comprehensive and actually test the right behavior
3. Ensure code follows project conventions
4. Look for edge cases that might not be covered
5. Confirm documentation is accurate and complete

Leave specific, actionable feedback in PR comments. Copilot coding agent will respond to direct feedback and can make corrections based on your review comments.
