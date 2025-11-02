# ym2151-log-player-rust

YM2151（OPM）レジスタイベントログをJSONファイルから読み込んで、リアルタイム再生とWAVファイル出力を行うプログラムのRust実装版。

[ym2151-log-player](https://github.com/cat2151/ym2151-log-player) のRust版です。

## ステータス

✅ **Phase 1-5 完了** - 基本機能が実装され、動作可能です。

- ✅ Phase 1: Nuked-OPM FFIバインディング
- ✅ Phase 2: JSONイベント読み込み
- ✅ Phase 3: イベント処理エンジン
- ✅ Phase 4: WAVファイル出力
- ✅ Phase 5: リアルタイムオーディオ再生

詳細は [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) を参照してください。

## 機能

- ✅ JSONログファイルからイベントを読み込み
- ✅ リアルタイムオーディオ再生（cpal使用、要: realtime-audio機能）
- ✅ WAVファイル出力（output.wav）
- ✅ Nuked-OPMライブラリによる正確なYM2151エミュレーション
- ✅ 高品質サンプルレート変換（55930 Hz → 48000 Hz）

## 使い方

### WAVファイル出力のみ（デフォルト）

```bash
cargo run
```

WAVファイル出力のデモを実行し、`output.wav` が生成されます。

### リアルタイムオーディオ再生を有効化

```bash
cargo run --features realtime-audio
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

詳細は [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) を参照してください。

## ライセンス

MIT License

## 利用ライブラリ

- Nuked-OPM: LGPL 2.1
- その他のRustクレート: 各クレートのライセンスに従う
