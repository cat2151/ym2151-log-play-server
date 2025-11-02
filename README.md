# ym2151-log-player-rust

YM2151（OPM）レジスタイベントログをJSONファイルから読み込んで、リアルタイム再生とWAVファイル出力を行うプログラムのRust実装版。

[ym2151-log-player](https://github.com/cat2151/ym2151-log-player) のRust版です。

## ステータス

⚠️ **開発中** - 実装計画書を作成し、初期プロジェクトをセットアップしました。

詳細は [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) を参照してください。

## 予定機能

- ✅ JSONログファイルからイベントを読み込み（計画済み）
- ✅ リアルタイムオーディオ再生（計画済み）
- ✅ WAVファイル出力（計画済み）
- ✅ Nuked-OPMライブラリによる正確なYM2151エミュレーション（計画済み）

## ビルド要件

- Rust 1.70以降
- zig cc（Cコンパイラとして使用）

詳細は [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) を参照してください。

## ライセンス

MIT License

## 利用ライブラリ

- Nuked-OPM: LGPL 2.1
- その他のRustクレート: 各クレートのライセンスに従う
