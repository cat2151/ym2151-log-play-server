# Performance Monitoring Guide (パフォーマンスモニタリングガイド)

このドキュメントは、音が途切れる問題を診断するためのパフォーマンスモニタリング機能の使用方法を説明します。

This document explains how to use the performance monitoring features to diagnose audio stuttering issues.

## Quick Start (クイックスタート)

パフォーマンスモニタリングを有効化してプログラムを実行：

Enable performance monitoring and run the program:

```bash
# Release buildでビルド（最適化有効）
# Build with release profile (optimizations enabled)
cargo build --release

# パフォーマンスモニタリング有効化
# Enable performance monitoring
PERF_MONITOR=1 ./target/release/ym2151-log-player-rust sample_events.json
```

## What is Measured (測定項目)

パフォーマンスモニタは以下の処理時間を測定します：

The performance monitor measures the following operation times:

1. **OPM Generation** (OPM波形生成)
   - YM2151チップのエミュレーション処理
   - 最も重要な time-critical 処理
   - Emulation of the YM2151 chip
   - Most critical time-sensitive operation

2. **Resampling** (リサンプリング)
   - 55930 Hz から 48000 Hz への変換
   - 高品質なsinc補間を使用
   - Conversion from 55930 Hz to 48000 Hz
   - Uses high-quality sinc interpolation

3. **WAV Capture** (WAV保存)
   - WAVバッファへのサンプルコピー
   - mutexロックを含む
   - Copying samples to WAV buffer
   - Includes mutex lock

4. **Format Conversion** (フォーマット変換)
   - i16からf32への変換
   - オーディオコールバックへの送信準備
   - Conversion from i16 to f32
   - Preparation for audio callback

5. **Total Iteration** (全体のイテレーション)
   - 上記すべての処理の合計時間
   - Total time for all operations above

## Understanding the Report (レポートの読み方)

### Example Output (出力例)

```
=== Performance Report ===
Total monitoring time: 5.23s
Threshold: 36.60ms

OPM Generation: avg=2.15ms, min=1.98ms, max=3.42ms, violations=0/142 (0.0%)
Resampling: avg=1.87ms, min=1.65ms, max=2.54ms, violations=0/142 (0.0%)
WAV Capture: avg=0.12ms, min=0.08ms, max=0.31ms, violations=0/142 (0.0%)
Format Conversion: avg=0.45ms, min=0.38ms, max=0.62ms, violations=0/142 (0.0%)
Total Iteration: avg=4.59ms, min=4.12ms, max=6.89ms, violations=0/142 (0.0%)

=== Time Breakdown ===
OPM Generation:    46.8%
Resampling:        40.7%
WAV Capture:       2.6%
Format Conversion: 9.8%

✅ Performance requirement met!
   Audio should play smoothly without stuttering.
==========================
```

### Key Metrics (重要な指標)

#### Threshold (閾値)
- バッファサイズから自動計算
- 例：2048サンプル ÷ 55930 Hz ≈ 36.6ms
- Automatically calculated from buffer size
- Example: 2048 samples ÷ 55930 Hz ≈ 36.6ms

#### Violations (違反)
- 閾値を超えた処理の回数
- 違反率が1%以上の場合、音が途切れる可能性が高い
- Number of times operation exceeded threshold
- If violation rate > 1%, likely causes stuttering

#### Time Breakdown (時間内訳)
- 各処理が全体に占める割合
- ボトルネックの特定に有用
- Percentage of time spent in each operation
- Useful for identifying bottlenecks

## Interpreting Results (結果の解釈)

### Good Performance (良好なパフォーマンス) ✅

```
Total Iteration: avg=4.59ms, violations=0/142 (0.0%)
✅ Performance requirement met!
```

- 平均時間が閾値よりも十分小さい (< 50%)
- 違反率が1%未満
- 音は途切れない
- Average time well below threshold (< 50%)
- Violation rate < 1%
- Audio plays smoothly

### Poor Performance (不良なパフォーマンス) ❌

```
Total Iteration: avg=42.15ms, violations=87/142 (61.3%)
⚠️  WARNING: Performance requirement NOT met!
```

- 平均時間が閾値に近い、または超えている
- 違反率が1%以上
- 音が途切れる
- Average time near or exceeds threshold
- Violation rate > 1%
- Audio stutters

## Troubleshooting Performance Issues (パフォーマンス問題のトラブルシューティング)

### Problem: OPM Generation is slow (OPM生成が遅い)

**原因 / Cause:**
- C code not optimized (C コードが最適化されていない)

**Solution:**
```toml
# build.rs
cc::Build::new()
    .file("opm.c")
    .opt_level(3)      // ← これを追加 / Add this
    .flag("-fwrapv")
    .compile("opm");
```

### Problem: Resampling is slow (リサンプリングが遅い)

**原因 / Cause:**
- High-quality sinc interpolation is expensive (高品質なsinc補間は重い)

**Solution:**
- Reduce `sinc_len` from 256 to 64-128
- Reduce `oversampling_factor` from 256 to 64-128

### Problem: Total time exceeds threshold (合計時間が閾値を超える)

**原因 / Cause:**
- Missing compiler optimizations (コンパイラ最適化が欠けている)

**Solution:**
```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = "fat"           # ← これを追加 / Add this
codegen-units = 1     # ← これを追加 / Add this
```

## Compiler Optimization Settings (コンパイラ最適化設定)

現在の推奨設定は `Cargo.toml` に既に適用されています：

Current recommended settings are already applied in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3          # Maximum speed optimization
lto = "fat"            # Full link-time optimization
codegen-units = 1      # Better optimization, slower compile
```

また、`build.rs` でC コードも最適化されています：

C code is also optimized in `build.rs`:

```rust
cc::Build::new()
    .file("opm.c")
    .opt_level(3)      // -O3 optimization
    .flag("-fwrapv")
    .compile("opm");
```

## Comparing with C Implementation (C実装との比較)

C言語版の実装でも同様にパフォーマンステストを行ってください：

Perform similar performance testing with the C implementation:

```bash
# C implementation (if available)
time ./ym2151-log-player sample_events.json
```

### Expected Performance (期待されるパフォーマンス)

最適化済みRust実装は、C実装と同等以上の性能を持つはずです：

Optimized Rust implementation should have performance equal to or better than C:

- OPM Generation: Similar to C (same underlying code)
- Resampling: May be slower if C version doesn't resample
- Overall: Within 10-20% of C implementation

## Advanced Monitoring (高度なモニタリング)

### Using external profilers (外部プロファイラの使用)

さらに詳細な分析が必要な場合：

For more detailed analysis:

```bash
# perf (Linux)
perf record -g ./target/release/ym2151-log-player-rust sample_events.json
perf report

# cargo-flamegraph
cargo install flamegraph
cargo flamegraph --bin ym2151-log-player-rust -- sample_events.json
```

## References (参考資料)

- [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md) - 詳細な分析結果
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Original C Implementation](https://github.com/cat2151/ym2151-log-player/)
