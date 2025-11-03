# GitHub Copilot 指示書 - ym2151-log-player-rust

## プロジェクト概要

YM2151 (OPM) チップレジスタイベントログプレイヤーのRust実装です。JSONイベントログを読み込み、Nuked-OPMエミュレータを使用してリアルタイム音声再生とWAVファイル出力を行います。

オリジナルC実装: https://github.com/cat2151/ym2151-log-player

### 主要機能
- 16進文字列対応JSONイベントログ解析 ("0x08"形式)
- cpalによるリアルタイム音声再生
- houndによるWAVファイル出力
- rubato使用のサンプルレート変換 (55930 Hz → 48000 Hz)
- FFI経由のNuked-OPMエミュレーション

## ビルド手順

### 前提条件
- Rust 1.70以降
- zig cc (C言語コンパイル用)
- **使用禁止**: mingw, msys2, MSVC

### ビルドとテスト

```bash
# 標準ビルド
cargo build

# リリースビルド
cargo build --release

# プログラム実行
cargo run -- sample_events.json

# 全テスト実行
cargo test

# 特定テスト実行
cargo test integration_test
```

## コーディング規約とプロジェクト構造

### 一般ガイドライン
- 標準Rust規約に従う (rustfmt)
- 分かりやすい変数名を使用
- 複雑なFFI操作にはコメント追加
- unsafeコードブロックには安全性の根拠を文書化

### エラーハンドリング
- アプリケーションコードでは `anyhow::Result` を使用
- ライブラリコードでは独自エラー型と `Result<T, E>` を使用

### FFI安全性
- 全てのunsafe FFI呼び出しを安全なRust APIでラップ
- 全FFIバインディングは `src/omp_ffi.rs` に配置
- 安全ラッパーは `src/opm.rs` に配置

### 主要ファイル
- `src/main.rs` - エントリーポイント
- `src/events.rs` - 16進対応JSONイベント解析
- `src/player.rs` - イベント処理エンジン
- `src/wav_writer.rs` - WAVファイル出力
- `src/audio.rs` - リアルタイム音声再生
- `omp.c`, `opm.h` - Nuked-OPM C実装

## 実装の詳細

### JSONイベント形式
イベントはアドレスとデータに16進文字列を使用:
```json
{
  "event_count": 2,
  "events": [
    {"time": 0, "addr": "0x08", "data": "0x00"},
    {"time": 2, "addr": "0x20", "data": "0xC7"}
  ]
}
```

### 16進文字列解析
`events.rs`でカスタムデシリアライザーを使用:
```rust
#[serde(deserialize_with = "parse_hex_string")]
pub addr: u8,
```

### イベント処理
- **入力**: Pass1形式イベント (単純レジスタ書き込み)
- **処理**: Pass2形式に変換 (アドレス書き込み + データ書き込みペア)
- **タイミング**: アドレスとデータ書き込み間にDELAY_SAMPLES (2サンプル) 挿入
- **OPMポート**: ポート0 = アドレスレジスタ, ポート1 = データレジスタ

### 音声仕様
- OPM内部レート: 55930 Hz
- 出力レート: 48000 Hz (リサンプリング必要)
- フォーマット: 16ビット符号付きステレオ

## よくあるタスク

### 新しいモジュールの追加
1. `src/module_name.rs` を作成
2. `src/lib.rs` に追加: `pub mod module_name;`
3. 必要に応じてmain.rsを更新
4. 同ファイルまたは `tests/` にテストを記述

### FFIの操作
1. `src/opm_ffi.rs` にC関数宣言を追加
2. `src/opm.rs` に安全なラッパーを作成
3. 安全性要件を文書化
4. 単体テストでテスト

### 依存関係の追加
1. 特定バージョンで `Cargo.toml` を更新
2. 音声/信号処理クレートのバージョンを固定
3. ライセンス互換性を確認 (プロジェクトはMIT)
4. 依存関係が必要な理由を文書化

## コード品質とリンティング

### リンターの実行

コミット前に必ずリンターを実行:

```bash
# rustfmtでコードをフォーマット
cargo fmt

# ファイルを変更せずにフォーマットをチェック
cargo fmt -- --check

# Clippyでコード品質をチェック
cargo clippy --all-targets

# 警告をエラーとして扱うClippy (CI用)
cargo clippy --all-targets -- -D warnings
```

### コミット前チェックリスト

コミットまたはコードレビュー要求前に:

1. **コードフォーマット**: `cargo fmt` を実行して一貫したフォーマットを確保
2. **リンティング問題修正**: `cargo clippy` を実行して警告に対処
3. **ビルド成功**: `cargo build` (または `cargo build --release`) を実行
4. **テスト実行**: `cargo test` を実行して全テストが通ることを確認
5. **ドキュメント更新**: パブリックAPIを追加した場合、docコメントを更新

## 参考資料

- オリジナル実装: https://github.com/cat2151/ym2151-log-player
- Nuked-OPM: https://github.com/nukeykt/Nuked-OPM
- 実装計画: 詳細なフェーズ分けは `IMPLEMENTATION_PLAN.md` を参照
- YM2151仕様: Yamaha YM2151データシート

# userからの指示
- 作業報告は、プルリクエストのコメントに書く。document作成禁止
  - DRY原則に準拠し、「codeやbuild scriptと同じことを、documentに書いたせいで、そのdocumentが陳腐化してハルシネーションやuserレビューコスト増大や混乱ほか様々なトラブル原因になる」を防止する
  - なおissue-notes/は、userがissueごとの意図を記録する用途で使う
