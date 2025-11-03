# ALSA Audio Capture for TDD on GitHub Actions

## 概要 / Overview

このドキュメントは、GitHub Actions Linux RunnerなどのCI/CD環境において、sudoを必要とせずにALSA設定ファイル（`~/.asoundrc`）を使用して音声出力をファイルにキャプチャする方法を説明します。

This document explains how to capture audio output to files using ALSA configuration files (`~/.asoundrc`) in CI/CD environments like GitHub Actions Linux Runner without requiring sudo privileges.

## 背景 / Background

### 課題 / Problem

CI/CD環境（GitHub Actionsなど）でオーディオアプリケーションをテストする際の課題：

Challenges when testing audio applications in CI/CD environments (such as GitHub Actions):

1. 音声出力デバイスが存在しない / No audio output devices available
2. sudoやカーネルモジュールのロードが制限されている / Restricted sudo and kernel module loading
3. リアルタイム音声再生をテストできない / Cannot test real-time audio playback

### 解決方法 / Solution

ALSAのユーザー設定ファイル（`~/.asoundrc`）を使用することで、アプリケーションコードを変更せずに音声出力をファイルにリダイレクトできます。

By using ALSA's user configuration file (`~/.asoundrc`), audio output can be redirected to files without modifying application code.

## 実装方法 / Implementation

### ステップ1: ALSA設定ファイルの作成 / Step 1: Create ALSA Configuration File

```bash
cat <<'EOF' > ~/.asoundrc
pcm.!default {
  type file
  slave.pcm "null"
  file "/tmp/alsa_capture.wav"
  format "wav"
}
EOF
```

### ステップ2: アプリケーションの実行 / Step 2: Run Application

```bash
# 通常通りアプリケーションを実行
# Run application normally
cargo run --release -- test_input.json
```

### ステップ3: キャプチャされた音声の検証 / Step 3: Verify Captured Audio

```bash
# ファイルが作成されたか確認
# Check if file was created
ls -lh /tmp/alsa_capture.wav

# ファイル形式を確認
# Check file format
file /tmp/alsa_capture.wav

# 音声データが含まれているか確認
# Verify audio data is present
hexdump -C /tmp/alsa_capture.wav | head -20
```

## テストスクリプト / Test Scripts

このリポジトリには2つのテストスクリプトが含まれています：

This repository includes two test scripts:

### 1. `test_alsa_capture.sh`

包括的なALSAキャプチャのデモンストレーション。

Comprehensive demonstration of ALSA capture.

```bash
./test_alsa_capture.sh
```

**実行内容 / What it does:**
- ALSA設定ファイルを作成 / Creates ALSA configuration file
- アプリケーションを実行 / Runs application
- キャプチャされたファイルを検証 / Verifies captured file
- ファイル特性を比較 / Compares file characteristics
- クリーンアップ / Cleans up

### 2. `test_alsa_tdd_example.sh`

TDD（テスト駆動開発）の例を示すテスト。

Example demonstrating Test-Driven Development (TDD).

```bash
./test_alsa_tdd_example.sh
```

**テスト内容 / Tests performed:**
1. ファイル作成の検証 / Verify file creation
2. ファイルサイズの検証 / Verify file contains data
3. WAVフォーマットの検証 / Verify valid WAV format
4. 音声データの検証 / Verify non-zero audio samples

## GitHub Actions統合 / GitHub Actions Integration

`.github/workflows/alsa_capture_test.yml`ファイルは、GitHub Actions上でALSAキャプチャをテストする方法を示しています。

The `.github/workflows/alsa_capture_test.yml` file demonstrates how to test ALSA capture on GitHub Actions.

**ワークフローの内容 / Workflow contents:**
1. ALSA開発ライブラリのインストール / Install ALSA development libraries
2. プロジェクトのビルド / Build project
3. 標準テストの実行 / Run standard tests
4. ALSAキャプチャテストの実行 / Run ALSA capture tests
5. TDD例の実行 / Run TDD example
6. フォールバック（--no-audioオプション）のテスト / Test fallback with --no-audio

## 検証結果 / Test Results

### ✅ 成功 / SUCCESS

**主要な発見 / Key Findings:**

1. ✅ ALSA `~/.asoundrc` 設定により音声出力をファイルにリダイレクト成功
   / ALSA `~/.asoundrc` configuration successfully redirects audio output to file

2. ✅ アプリケーションコードの変更不要
   / No application code modification required

3. ✅ sudo権限不要
   / No sudo privileges required

4. ✅ ヘッドレスGitHub Actionsランナーで動作
   / Works on headless GitHub Actions runners

5. ✅ リアルタイム音声再生がWAVファイルにキャプチャされる
   / Real-time audio playback captured to WAV file

6. ✅ アプリケーションの直接WAV出力も正常動作
   / Application's direct WAV output also works normally

### ファイル比較 / File Comparison

**ALSAキャプチャファイル / ALSA Captured File:**
- パス / Path: `/tmp/alsa_capture.wav`
- フォーマット / Format: 32-bit PCM, ステレオ, 48000 Hz / stereo, 48000 Hz
- サイズ / Size: ~1.2GB（テストファイルの場合 / for test file）
- 内容 / Content: リアルタイム再生からキャプチャされた音声 / Audio captured from real-time playback

**アプリケーション直接出力 / Application Direct Output:**
- パス / Path: `output.wav`
- フォーマット / Format: 16-bit PCM, ステレオ, 55930 Hz / stereo, 55930 Hz
- サイズ / Size: ~553KB（テストファイルの場合 / for test file）
- 内容 / Content: ネイティブOPMレートの音声 / Audio at native OPM rate

**注意 / Note:** ALSAキャプチャファイルが大きい理由：
/ ALSA captured file is larger due to:

- 32ビットPCMフォーマット（直接出力は16ビット）
  / 32-bit PCM format (vs 16-bit in direct output)
- 48kHzサンプルレート（ネイティブ55930 Hzからリサンプリング）
  / 48kHz sample rate (resampled from native 55930 Hz)
- オーディオバッファタイミングからの無音パディングを含む
  / Includes silence padding from audio buffer timing

## 使用例 / Use Cases

### CI/CDでの自動テスト / Automated Testing in CI/CD

```yaml
- name: Test audio output
  run: |
    cat <<'EOF' > ~/.asoundrc
    pcm.!default {
      type file
      slave.pcm "null"
      file "/tmp/test_output.wav"
      format "wav"
    }
    EOF
    
    cargo run --release -- input.json
    
    # Verify output
    test -f /tmp/test_output.wav && echo "✅ Audio captured"
```

### ローカル開発でのテスト / Testing in Local Development

```bash
# ALSA設定を作成 / Create ALSA config
cat <<'EOF' > ~/.asoundrc
pcm.!default {
  type file
  slave.pcm "null"
  file "/tmp/debug_audio.wav"
  format "wav"
}
EOF

# アプリケーションを実行 / Run application
cargo run -- input.json

# 音声を検証 / Verify audio
file /tmp/debug_audio.wav

# クリーンアップ / Cleanup
rm ~/.asoundrc
```

## 技術的詳細 / Technical Details

### ALSA設定パラメータ / ALSA Configuration Parameters

```
pcm.!default {
  type file          # ALSAファイルプラグインを使用 / Use ALSA file plugin
  slave.pcm "null"   # nullデバイスに送信（タイミング制御用） / Send to null device (for timing)
  file "<path>"      # 出力ファイルパス / Output file path
  format "wav"       # WAVフォーマット / WAV format
}
```

### 制限事項 / Limitations

1. **ファイルサイズ** / **File Size**
   - キャプチャファイルは非常に大きくなる可能性がある
   - Captured files can be very large
   - 無音パディングを含む / Includes silence padding

2. **形式の違い** / **Format Differences**
   - ALSA: 32-bit PCM @ 48kHz（リサンプリング後 / after resampling）
   - Direct: 16-bit PCM @ 55930 Hz（ネイティブレート / native rate）

3. **タイミング** / **Timing**
   - リアルタイム再生の速度に影響される可能性がある
   - May be affected by real-time playback speed
   - バッファアンダーランによる無音が含まれる可能性がある
   - May include silence from buffer underruns

## トラブルシューティング / Troubleshooting

### 問題: ALSAエラーが表示される / Problem: ALSA errors displayed

```
ALSA lib confmisc.c:1342:(snd_func_refer) error evaluating name
```

**解決策 / Solution:** これらのエラーは無視できます。ALSA設定によりファイル出力は正常に動作します。
/ These errors can be ignored. File output works correctly with ALSA configuration.

### 問題: ファイルが作成されない / Problem: File not created

**確認事項 / Check:**
1. `~/.asoundrc` が正しく作成されているか / `~/.asoundrc` is correctly created
2. ファイルパスに書き込み権限があるか / Write permissions for file path
3. アプリケーションが正常に終了したか / Application completed normally

### 問題: ファイルが空または小さい / Problem: File is empty or small

**確認事項 / Check:**
1. アプリケーションが音声を生成しているか / Application is generating audio
2. アプリケーションが完全に終了したか（バッファフラッシュ用） / Application completed fully (for buffer flush)
3. hexdumpで音声データを確認 / Check audio data with hexdump

## まとめ / Conclusion

ALSA設定ファイル（`~/.asoundrc`）を使用することで、GitHub ActionsなどのCI/CD環境において：

By using ALSA configuration files (`~/.asoundrc`), in CI/CD environments like GitHub Actions:

✅ sudo不要で音声出力をキャプチャ可能 / Can capture audio output without sudo
✅ コード変更不要 / No code changes required
✅ TDD（テスト駆動開発）が実現可能 / Enables Test-Driven Development (TDD)
✅ 音声処理の自動テストが可能 / Enables automated testing of audio processing

この方法は、音声アプリケーションのCI/CDパイプラインに統合するのに理想的です。

This method is ideal for integrating audio applications into CI/CD pipelines.

## 参考資料 / References

- [ALSA Project](https://www.alsa-project.org/)
- [ALSA Configuration](https://www.alsa-project.org/wiki/Asoundrc)
- [GitHub Actions](https://docs.github.com/en/actions)
- [YM2151 Log Player (Rust)](https://github.com/cat2151/ym2151-log-player-rust)
