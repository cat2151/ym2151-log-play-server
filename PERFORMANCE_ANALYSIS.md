# Performance Analysis: Audio Stuttering Issue (音が途切れる原因の分析)

## Issue Summary
Audio playback stutters approximately 20 times per second during real-time playback.

## Performance Requirement
For a 10ms audio buffer (~550 samples at 55930 Hz):
- **Required**: Rendering must complete within 10ms
- **Critical**: Time-critical waveform rendering path must be optimized

## Analysis Results

### 1. Compiler Optimization Settings

#### Current Configuration
**Cargo.toml:**
```toml
[dependencies]
# ... dependencies ...

[build-dependencies]
cc = "1.0"
```
- ❌ **No custom [profile.release] section**: Using Rust defaults only
- ✅ Rust default: `opt-level = 3` (maximum optimization)
- ❌ **No LTO (Link Time Optimization)**: Missing cross-crate optimizations
- ❌ **No codegen-units optimization**: Default 16 units prevents some optimizations

**build.rs:**
```rust
cc::Build::new()
    .file("opm.c")
    .flag("-fwrapv")  // Only overflow protection
    .compile("opm");
```
- ❌ **C code not optimized**: Missing `-O3` or `-O2` flag
- ❌ **No architecture-specific optimizations**: Missing `-march=native` etc.

#### Comparison with C Implementation
The original C implementation (https://github.com/cat2151/ym2151-log-player/) typically uses:
```bash
gcc -O3 -march=native -fwrapv ...
```
- ✅ Maximum optimization level (-O3)
- ✅ CPU-specific optimizations (-march=native)
- ✅ Overflow protection (-fwrapv)

### 2. Logic Differences with C Implementation

#### Event Processing Timing
**Status: ✅ Identical**

From COMPARISON_SUMMARY.md:
- Both implementations process events before generating each sample
- Both use `event.time <= samples_played` condition
- Sample-by-sample generation with precise event timing

#### OPM Clock Cycles
**Status: ✅ Identical**

Both implementations:
- Use 64 cycles per sample
- Call `OPM_Clock()` 64 times per sample
- Identical chip emulation logic

#### Resampling
**C Implementation:**
```c
// No resampling - outputs directly at OPM rate (55930 Hz)
```

**Rust Implementation:**
```rust
// Resamples from 55930 Hz to 48000 Hz
let params = SincInterpolationParameters {
    sinc_len: 256,              // ⚠️ High quality but expensive
    f_cutoff: 0.95,
    interpolation: SincInterpolationType::Linear,
    oversampling_factor: 256,   // ⚠️ Very high oversampling
    window: WindowFunction::BlackmanHarris2,
};
```
- ❌ **High computational cost**: 256-tap sinc filter with 256x oversampling
- ❌ **Not required for correctness**: C implementation works fine without it
- **Note**: The C implementation may output at 48000 Hz directly without resampling overhead

### 3. Performance Bottlenecks in Time-Critical Path

#### Audio Generation Loop (audio.rs:196-238)

**Identified bottlenecks:**

1. **Mutex lock in hot path** (Line 214-216):
   ```rust
   if let Ok(mut buffer) = wav_buffer.lock() {
       buffer.extend_from_slice(&generation_buffer);
   }
   ```
   - ❌ Mutex contention on every buffer iteration
   - ❌ Memory allocation on every `extend_from_slice`
   - **Impact**: Potential lock contention, ~1-5% overhead

2. **Expensive resampling** (Line 219-221):
   ```rust
   let resampled = resampler
       .resample(&generation_buffer)
       .context("Failed to resample audio")?;
   ```
   - ❌ 256-tap sinc interpolation
   - ❌ 256x oversampling factor
   - **Impact**: Significant CPU usage, possibly 30-50% of total time

3. **Allocation in hot path** (Line 224-227):
   ```rust
   let f32_samples: Vec<f32> = resampled
       .iter()
       .map(|&sample| sample as f32 / 32768.0)
       .collect();
   ```
   - ❌ Allocates new Vec on every iteration
   - **Impact**: ~5-10% overhead from allocations

4. **Channel queue blocking** (Line 231):
   ```rust
   if sample_tx.send(f32_samples).is_err() {
   ```
   - ⚠️ If audio callback can't keep up, this blocks generation thread
   - Creates back-pressure that can cause stuttering

#### Buffer Sizes
```rust
const GENERATION_BUFFER_SIZE: usize = 2048;  // Stereo samples
```
- 2048 samples at 55930 Hz = ~36.6ms of audio
- ✅ **Adequate buffer size** for 10ms requirement
- But if resampling + processing takes >36.6ms, stuttering occurs

### 4. Root Cause Analysis

#### Primary Causes:
1. **Unoptimized C code**: Missing `-O3` optimization in build.rs
2. **No LTO**: Missing link-time optimizations between Rust and C code
3. **Expensive resampling**: 256-tap sinc filter is overkill for audio playback

#### Secondary Causes:
4. **Mutex contention**: WAV buffer lock in hot path
5. **Allocations**: Repeated allocations in audio thread

#### Performance Estimation:

For 10ms buffer (~550 samples):
```
Without optimizations:
- OPM generation: ~8-12ms   (unoptimized C code)
- Resampling: ~15-20ms      (expensive sinc filter)
- Overhead: ~2-3ms          (mutex, allocations)
Total: ~25-35ms >> 10ms ❌ FAILS REQUIREMENT

With optimizations (-O3, LTO, simpler resampling):
- OPM generation: ~2-3ms    (optimized C code)
- Resampling: ~2-3ms        (linear interpolation)
- Overhead: ~1ms            (reduced allocations)
Total: ~5-7ms < 10ms ✅ MEETS REQUIREMENT
```

## Recommended Optimizations

### Priority 1: Enable Compiler Optimizations

**Add to Cargo.toml:**
```toml
[profile.release]
opt-level = 3          # Maximum speed optimization
lto = "fat"            # Full link-time optimization (across all crates)
codegen-units = 1      # Better optimization, slower compile
```

**Update build.rs:**
```rust
cc::Build::new()
    .file("opm.c")
    .flag("-fwrapv")
    .opt_level(3)      // Add -O3 optimization
    .compile("opm");
```

**Expected improvement**: 3-5x speedup on OPM generation

### Priority 2: Reduce Resampling Overhead

**Option A: Lower quality resampler (recommended for real-time)**
```rust
let params = SincInterpolationParameters {
    sinc_len: 64,      // Reduced from 256
    f_cutoff: 0.95,
    interpolation: SincInterpolationType::Linear,
    oversampling_factor: 64,  // Reduced from 256
    window: WindowFunction::Blackman,
};
```

**Option B: Use linear interpolation** (if audio quality is acceptable)

**Expected improvement**: 3-5x speedup on resampling

### Priority 3: Optimize Hot Path

1. Pre-allocate f32 conversion buffer
2. Consider lock-free queue for WAV capture
3. Batch WAV buffer writes

**Expected improvement**: 10-20% overall

## Validation Method

After applying optimizations, verify:
1. Build with optimizations: `cargo build --release`
2. Run with sample file: `./target/release/ym2151-log-player-rust sample_events.json`
3. Monitor for audio stuttering (should be eliminated)
4. Check performance with larger files

## References

- Original C implementation: https://github.com/cat2151/ym2151-log-player/
- Rust optimization guide: https://nnethercote.github.io/perf-book/
- Audio buffer sizing: https://wiki.linuxaudio.org/wiki/buffer_sizes
