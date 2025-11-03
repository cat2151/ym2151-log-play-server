# Buffer Size Investigation and Fix

## Problem Statement

Audio stuttering was occurring on Windows despite performance measurements showing processing time (18ms) well below the reported threshold (36ms). This document explains the root cause and the fix.

## Root Cause Analysis

### The Issue

The performance monitoring system was calculating the threshold incorrectly:

1. **Internal threshold calculation**: Based on `GENERATION_BUFFER_SIZE` (2048 samples at 55930 Hz = ~36.6ms)
2. **Actual audio system**: Using `cpal::BufferSize::Default` (platform-dependent, unknown size)
3. **Mismatch**: The real deadline is determined by the audio device buffer, not our internal generation buffer

### Why This Caused Stuttering

Example scenario on Windows:
- **Generation buffer**: 2048 samples → 36.6ms at 55930 Hz
- **Audio device buffer**: 1024 samples → 21.3ms at 48000 Hz ← **This is the real deadline!**
- **Processing time**: 18ms
- **Result**: 18ms < 36.6ms (appears OK) but 18ms < 21.3ms is cutting it too close!

When the audio device requests samples every 21.3ms but we're measuring against a 36.6ms threshold, we have a false sense of safety margin.

## The Fix

### Changes Made

1. **Capture actual audio buffer size** (`src/audio.rs`)
   - Added shared state to capture buffer size from first audio callback
   - Wait for buffer size detection before starting generation
   - Print detected buffer size with duration calculation

2. **Use actual buffer for threshold** (`src/audio.rs`)
   - Calculate threshold from actual audio device buffer size
   - Threshold = (audio_buffer_size / 2) / OUTPUT_SAMPLE_RATE
   - Fallback to generation buffer if detection fails

3. **Enhanced performance reporting** (`src/perf_monitor.rs`)
   - Added buffer configuration section
   - Show both audio device buffer and generation buffer
   - Explain threshold rationale clearly

### Sample Output

```
⏳ Waiting for audio device buffer size...
✅ Audio device buffer detected:
   Buffer size: 2048 samples (1024 stereo frames)
   Buffer duration: 21.33ms at 48000 Hz
   Generation buffer: 4096 samples (2048 stereo frames) at 55930 Hz
   Generation duration: 36.60ms

📊 Performance monitoring enabled (PERF_MONITOR=1)
   Performance threshold: 21.33ms (based on audio device buffer)
   This is the time we have to generate audio before underrun occurs
```

Performance Report:
```
=== Buffer Configuration ===
Audio device buffer: 2048 samples (1024 stereo frames)
Audio buffer duration: 21.33ms at 48000 Hz
Generation buffer: 4096 samples (2048 stereo frames) at 55930 Hz
Generation buffer duration: 36.60ms

=== Performance Threshold ===
Threshold: 21.33ms
(Based on actual audio device buffer size)
Rationale: Processing must complete within this time to avoid audio underruns
```

## Testing on Windows

To test this fix on Windows:

1. Build and run with performance monitoring:
   ```bash
   set PERF_MONITOR=1
   cargo run --release sample_events.json
   ```

2. Look for the buffer size output at startup:
   - Verify the audio device buffer size is detected
   - Note the buffer duration (this is the real threshold)

3. Check the performance report at the end:
   - Verify the threshold matches the audio buffer duration
   - Compare processing time against this threshold
   - If violations occur, this now accurately indicates audio issues

## Future Improvements

### Making Buffer Size Configurable

If needed, we can allow users to specify buffer size:

```rust
let config = cpal::StreamConfig {
    channels: 2,
    sample_rate: cpal::SampleRate(OUTPUT_SAMPLE_RATE),
    buffer_size: cpal::BufferSize::Fixed(4096), // User-specified
};
```

Larger buffers = more latency but more safety margin for processing.

### Environment Variable Control

Could add `AUDIO_BUFFER_SIZE` environment variable:
```bash
set AUDIO_BUFFER_SIZE=4096
cargo run --release sample_events.json
```

## Technical Details

### Why Two Different Buffer Sizes?

1. **Audio device buffer** (platform-dependent, e.g., 1024-4096 samples at 48000 Hz)
   - Determined by OS audio subsystem
   - Real-time deadline we must meet
   - Controls latency and CPU usage

2. **Generation buffer** (2048 samples at 55930 Hz)
   - Our internal batch size for OPM emulation
   - Optimized for efficient processing
   - Can be different from audio buffer

### Sample Rate Conversion

- OPM generates at 55930 Hz (native chip rate)
- Audio output at 48000 Hz (standard sample rate)
- Resampling happens in between
- Threshold must be based on the OUTPUT rate (48000 Hz)

## References

- Issue: "音が途切れる問題が続いている。64回のcycles消費をbatch化しても、処理時間にはまったく影響がなかった"
- Agent instructions: Request for threshold calculation transparency and buffer size visibility
- Related files:
  - `src/audio.rs` - Audio playback implementation
  - `src/perf_monitor.rs` - Performance monitoring
  - `PERFORMANCE_MONITORING.md` - Performance monitoring guide
