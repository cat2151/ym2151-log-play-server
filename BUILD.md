# ビルド手順書 / Build Instructions

このドキュメントは、ym2151-log-player-rust のビルド方法を説明します。

This document explains how to build ym2151-log-player-rust.

## 目次 / Table of Contents

- [必要な環境 / Requirements](#必要な環境--requirements)
- [Linux でのビルド / Building on Linux](#linux-でのビルド--building-on-linux)
- [Linux から Windows へのクロスコンパイル / Cross-compiling to Windows from Linux](#linux-から-windows-へのクロスコンパイル--cross-compiling-to-windows-from-linux)
- [Windows でのネイティブビルド / Native Build on Windows](#windows-でのネイティブビルド--native-build-on-windows)
- [リリースビルドの最適化 / Release Build Optimization](#リリースビルドの最適化--release-build-optimization)
- [トラブルシューティング / Troubleshooting](#トラブルシューティング--troubleshooting)

## 必要な環境 / Requirements

### 共通要件 / Common Requirements

- **Rust**: 1.70 以降 / 1.70 or later
  - インストール / Installation: https://rustup.rs/
- **Cコンパイラ**: zig cc（推奨） / C Compiler: zig cc (recommended)
  - mingw, msys2, MSVC は**使用しないでください** / Do **NOT** use mingw, msys2, or MSVC

### プラットフォーム別要件 / Platform-specific Requirements

#### Linux

- gcc または zig cc
- ALSA開発ライブラリ（リアルタイム音声再生を有効にする場合）

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel
```

#### Windows

- zig cc（必須） / zig cc (required)
- Rustツールチェーン / Rust toolchain

## Linux でのビルド / Building on Linux

### 1. 基本ビルド / Basic Build

```bash
# リポジトリをクローン / Clone the repository
git clone https://github.com/cat2151/ym2151-log-player-rust.git
cd ym2151-log-player-rust

# デバッグビルド / Debug build
cargo build

# リリースビルド / Release build
cargo build --release

# 実行 / Run
./target/release/ym2151-log-player-rust sample_events.json
```

### 2. リアルタイム音声再生を有効化 / Enable Real-time Audio

```bash
# リアルタイム音声機能付きビルド / Build with real-time audio
cargo build --release --features realtime-audio

# 実行 / Run
./target/release/ym2151-log-player-rust sample_events.json
```

**注意 / Note:** Linux環境では、ALSA開発ライブラリが必要です。
In Linux environment, ALSA development libraries are required.

### 3. テストの実行 / Running Tests

```bash
# 基本テスト / Basic tests
cargo test

# リアルタイム音声機能を含むテスト / Tests with real-time audio
cargo test --features realtime-audio

# 詳細出力付きテスト / Tests with verbose output
cargo test -- --nocapture
```

## Linux から Windows へのクロスコンパイル / Cross-compiling to Windows from Linux

### 前提条件 / Prerequisites

1. **zig のインストール / Install zig**

```bash
# zig の最新版をダウンロード / Download latest zig
# https://ziglang.org/download/ から適切なバージョンを選択
# Choose appropriate version from https://ziglang.org/download/

# 例（Linux x86_64の場合） / Example (for Linux x86_64)
wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz
tar -xf zig-linux-x86_64-0.13.0.tar.xz

# PATHに追加 / Add to PATH
export PATH=$PATH:$PWD/zig-linux-x86_64-0.13.0
```

2. **Windows ターゲットの追加 / Add Windows target**

```bash
rustup target add x86_64-pc-windows-gnu
```

### ビルド手順 / Build Steps

```bash
# 環境変数を設定 / Set environment variables
export CC="zig cc -target x86_64-windows"
export AR="zig ar"

# ビルド実行 / Execute build
cargo build --release --target x86_64-pc-windows-gnu

# 生成された実行ファイル / Generated executable
# target/x86_64-pc-windows-gnu/release/ym2151-log-player-rust.exe
```

### リアルタイム音声機能付きクロスコンパイル / Cross-compile with Real-time Audio

```bash
export CC="zig cc -target x86_64-windows"
export AR="zig ar"

cargo build --release --target x86_64-pc-windows-gnu --features realtime-audio
```

### ワンライナースクリプト / One-liner Script

```bash
CC="zig cc -target x86_64-windows" AR="zig ar" cargo build --release --target x86_64-pc-windows-gnu
```

## Windows でのネイティブビルド / Native Build on Windows

### 前提条件 / Prerequisites

1. **Rust のインストール / Install Rust**
   - https://rustup.rs/ からインストーラーをダウンロード
   - Download installer from https://rustup.rs/

2. **zig のインストール / Install zig**
   - https://ziglang.org/download/ から Windows版をダウンロード
   - Download Windows version from https://ziglang.org/download/
   - 展開したディレクトリをPATHに追加 / Add extracted directory to PATH

### ビルド手順（PowerShell） / Build Steps (PowerShell)

```powershell
# リポジトリをクローン / Clone repository
git clone https://github.com/cat2151/ym2151-log-player-rust.git
cd ym2151-log-player-rust

# 環境変数を設定 / Set environment variables
$env:CC = "zig cc"
$env:AR = "zig ar"

# リリースビルド / Release build
cargo build --release

# 実行 / Run
.\target\release\ym2151-log-player-rust.exe sample_events.json
```

### ビルド手順（コマンドプロンプト） / Build Steps (Command Prompt)

```cmd
REM リポジトリをクローン / Clone repository
git clone https://github.com/cat2151/ym2151-log-player-rust.git
cd ym2151-log-player-rust

REM 環境変数を設定 / Set environment variables
set CC=zig cc
set AR=zig ar

REM リリースビルド / Release build
cargo build --release

REM 実行 / Run
.\target\release\ym2151-log-player-rust.exe sample_events.json
```

### リアルタイム音声機能付きビルド / Build with Real-time Audio

```powershell
# PowerShell
$env:CC = "zig cc"
$env:AR = "zig ar"
cargo build --release --features realtime-audio
```

```cmd
REM Command Prompt
set CC=zig cc
set AR=zig ar
cargo build --release --features realtime-audio
```

## リリースビルドの最適化 / Release Build Optimization

### バイナリサイズの削減 / Reducing Binary Size

`Cargo.toml` に以下の設定を追加することで、バイナリサイズを削減できます：
You can reduce binary size by adding the following configuration to `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"        # サイズ最適化 / Optimize for size
lto = true             # Link Time Optimization
codegen-units = 1      # 並列化を無効化（より良い最適化） / Disable parallelization (better optimization)
strip = true           # シンボル情報を削除 / Remove debug symbols
panic = "abort"        # パニック時にアンワインドしない / Don't unwind on panic
```

**注意 / Note:** これらの設定はコンパイル時間を増加させますが、実行ファイルサイズを大幅に削減します。
These settings increase compilation time but significantly reduce executable size.

### 追加の最適化オプション / Additional Optimization Options

より積極的なサイズ削減を行いたい場合：
For more aggressive size reduction:

```bash
# upx を使用した圧縮（オプション） / Compression using upx (optional)
# https://upx.github.io/
cargo build --release
upx --best --lzma target/release/ym2151-log-player-rust
```

## トラブルシューティング / Troubleshooting

### 問題: ALSA エラー（Linux） / Issue: ALSA errors (Linux)

**エラーメッセージ / Error message:**
```
error: failed to run custom build command for `alsa-sys v0.x.x`
```

**解決方法 / Solution:**
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev pkg-config

# Fedora
sudo dnf install alsa-lib-devel
```

### 問題: zig cc が見つからない / Issue: zig cc not found

**エラーメッセージ / Error message:**
```
error: linker `zig cc` not found
```

**解決方法 / Solution:**

1. zig が正しくインストールされているか確認
   Verify zig is properly installed:
   ```bash
   zig version
   ```

2. PATH が正しく設定されているか確認
   Verify PATH is correctly set:
   ```bash
   echo $PATH  # Linux/macOS
   echo %PATH%  # Windows CMD
   $env:Path   # Windows PowerShell
   ```

3. 環境変数を再設定
   Reset environment variables:
   ```bash
   export CC="zig cc"
   export AR="zig ar"
   ```

### 問題: Windows クロスコンパイルの失敗 / Issue: Windows cross-compilation failure

**エラーメッセージ / Error message:**
```
error: linking with `zig cc` failed
```

**確認事項 / Check:**

1. zig のバージョンが 0.11.0 以降であること
   zig version is 0.11.0 or later

2. Windows ターゲットが追加されているか確認
   Verify Windows target is added:
   ```bash
   rustup target list --installed | grep windows
   ```

3. 環境変数が正しく設定されているか確認
   Verify environment variables are set correctly:
   ```bash
   echo $CC
   echo $AR
   ```

### 問題: opm.c のコンパイル警告 / Issue: opm.c compilation warnings

**警告メッセージ / Warning message:**
```
warning: unused variable 'channel'
```

**説明 / Explanation:**
これらは Nuked-OPM ライブラリ由来の警告で、プログラムの動作には影響しません。無視して構いません。

These warnings come from the Nuked-OPM library and do not affect program behavior. They can be safely ignored.

### 問題: リアルタイム音声が動作しない / Issue: Real-time audio not working

**確認事項 / Check:**

1. realtime-audio 機能が有効になっているか
   Verify realtime-audio feature is enabled:
   ```bash
   cargo build --features realtime-audio
   ```

2. 音声出力デバイスが利用可能か確認
   Verify audio output device is available

3. Linux の場合、ALSA ライブラリがインストールされているか確認
   On Linux, verify ALSA libraries are installed

## ビルド成果物 / Build Artifacts

### 通常のビルド / Standard Build

- デバッグビルド / Debug build: `target/debug/ym2151-log-player-rust[.exe]`
- リリースビルド / Release build: `target/release/ym2151-log-player-rust[.exe]`

### クロスコンパイル / Cross-compilation

- Windows向け / For Windows: `target/x86_64-pc-windows-gnu/release/ym2151-log-player-rust.exe`

## 参考情報 / References

- [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) - 実装計画書 / Implementation plan
- [README.md](README.md) - プロジェクト概要 / Project overview
- [Rust公式サイト](https://www.rust-lang.org/)
- [zig公式サイト](https://ziglang.org/)
- [Nuked-OPM](https://github.com/nukeykt/Nuked-OPM)
- [元のC実装 / Original C implementation](https://github.com/cat2151/ym2151-log-player)

## ライセンス / License

このプロジェクトは MIT ライセンスの下で公開されています。
This project is released under the MIT License.

詳細は [LICENSE](LICENSE) ファイルを参照してください。
See [LICENSE](LICENSE) file for details.
