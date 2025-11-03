# ym2151-log-player-rust

YM2151（OPM）レジスタイベントログをJSONファイルから読み込んで、リアルタイム再生とWAVファイル出力を行うプログラムのRust実装版。

[ym2151-log-player](https://github.com/cat2151/ym2151-log-player) のRust版です。

## 状況

音は鳴っていますが不具合があります。issuesを参照ください。

pass2 json出力は、シンプル化のために削除済みです。必要になったらagentに実装させてください。

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
- ✅ **起動直後から即時リアルタイムオーディオ再生**（デフォルト、C言語版と同じロジック）
- ✅ **再生と同時にWAVファイルをキャプチャ保存**（output.wav）
- ✅ Nuked-OPMライブラリによる正確なYM2151エミュレーション
- ✅ 高品質サンプルレート変換（55930 Hz → 48000 Hz、再生用）
- ✅ CI/ヘッドレス環境用の `--no-audio` オプション

## クイックスタート / Quick Start

詳細なビルド手順については **[BUILD.md](BUILD.md)** を参照してください。

For detailed build instructions, please refer to **[BUILD.md](BUILD.md)**.

## 使い方

### 基本的な使い方（リアルタイム再生 + WAV保存）

```bash
cargo run --release sample_events.json
```

または、ビルド済みの場合：

```bash
./target/release/ym2151-log-player-rust sample_events.json
```

**動作:**
1. イベントログを読み込み
2. **即座にリアルタイム音声再生を開始**（C言語版と同じ）
3. 再生と同時にWAVファイルをキャプチャ
4. 再生完了後、`output.wav` を保存

### CI/ヘッドレス環境用（音声デバイスなし）

音声デバイスが利用できない環境では、`--no-audio` オプションを使用できます：

```bash
cargo run --release -- --no-audio sample_events.json
```

このモードでは、リアルタイム再生なしでWAVファイルのみ生成します。

### コマンドライン引数

```
使用方法:
  ym2151-log-player-rust [オプション] <json_log_file>

オプション:
  --no-audio    音声デバイスなしでWAVファイルのみ生成
                (デフォルトはリアルタイム再生+WAV保存)

例:
  ym2151-log-player-rust sample_events.json
  ym2151-log-player-rust --no-audio sample_events.json
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

### ビルド要件

**注意:** リアルタイムオーディオ再生（デフォルト）には音声出力デバイスが必要です。
Linux環境では、ALSA開発ライブラリのインストールが必要です：

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
# 標準テスト（realtime-audio機能込み）
cargo test
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
