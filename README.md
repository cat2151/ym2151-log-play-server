# ym2151-log-player-rust

YM2151（OPM）レジスタイベントログをJSONファイルから読み込んで、リアルタイム再生とWAVファイル出力を行うプログラムのRust実装版。

[ym2151-log-player](https://github.com/cat2151/ym2151-log-player) のRust版です。

## 状況

開発中です。致命的な不具合が複数発生しています

## ステータス

✅ **全フェーズ完了** - すべての機能が実装され、動作可能です。

- ✅ Phase 1: Nuked-OPM FFIバインディング
- ✅ Phase 2: JSONイベント読み込み
- ✅ Phase 3: イベント処理エンジン
- ✅ Phase 4: WAVファイル出力
- ✅ Phase 5: リアルタイムオーディオ再生
- ✅ Phase 6: メインアプリケーション統合
- ✅ Phase 7: Windows ビルドとテスト

詳細は [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) を参照してください。

## 機能

- ✅ JSONログファイルからイベントを読み込み
- ✅ リアルタイムオーディオ再生（cpal使用、要: realtime-audio機能）
- ✅ WAVファイル出力（output.wav）
- ✅ Nuked-OPMライブラリによる正確なYM2151エミュレーション
- ✅ 高品質サンプルレート変換（55930 Hz → 48000 Hz）

## クイックスタート / Quick Start

詳細なビルド手順については **[BUILD.md](BUILD.md)** を参照してください。

For detailed build instructions, please refer to **[BUILD.md](BUILD.md)**.

## 使い方

### 注意

メンテ中です。以降の情報は長くて読みづらいことがあります

#### 主な使い方

ビルドして音を鳴らします

```
cargo run --release --features realtime-audio output_ym2151.json
```

ビルド済みのexeで音を鳴らします

```
./target/release/ym2151-log-player-rust output_ym2151.json
```

### 基本的な使い方

```bash
cargo run <json_log_file>
```

例:
```bash
cargo run sample_events.json
```

これにより、JSONイベントログファイルを読み込み、`output.wav` ファイルが生成されます。

### コマンドライン引数

```
使用方法:
  ym2151-log-player-rust <json_log_file>

例:
  ym2151-log-player-rust events.json
  ym2151-log-player-rust sample_events.json
```

### JSONイベントログファイル形式

```json
{
  "event_count": 100,
  "events": [
    {"time": 0, "addr": "0x08", "data": "0x00"},
    {"time": 2, "addr": "0x20", "data": "0xC7"}
  ]
}
```

- `event_count`: イベント総数
- `events`: イベント配列
  - `time`: サンプル時刻（絶対時刻）
  - `addr`: YM2151レジスタアドレス（16進数文字列）
  - `data`: レジスタに書き込むデータ（16進数文字列）
  - `is_data`: （オプション、読み込み時は無視されます）

**注意:** プログラムは入力イベントを自動的に2段階（アドレス書き込み→データ書き込み）に分割し、必要な遅延を挿入します。

### リアルタイムオーディオ再生を有効化

```bash
cargo run --features realtime-audio sample_events.json
```

リアルタイムオーディオ再生とWAVファイル出力の両方を実行します。

**注意:** リアルタイムオーディオ再生には音声出力デバイスが必要です。
Linux環境では、ALSA開発ライブラリのインストールが必要な場合があります：

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel
```

### リリースビルド

```bash
cargo build --release
./target/release/ym2151-log-player-rust sample_events.json
```

### テストの実行

```bash
# 基本テスト（realtime-audio機能なし）
cargo test

# realtime-audio機能を含むテスト
cargo test --features realtime-audio
```

## ビルド要件

- Rust 1.70以降
- zig cc（Cコンパイラとして使用）
- （オプション）ALSA開発ライブラリ（Linux環境でrealtime-audio機能を使用する場合）

**詳細なビルド手順は [BUILD.md](BUILD.md) を参照してください。**

その他の詳細は [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) を参照してください。

## ライセンス

MIT License

## 利用ライブラリ

- Nuked-OPM: LGPL 2.1
- その他のRustクレート: 各クレートのライセンスに従う
