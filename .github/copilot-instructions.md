# GitHub Copilot 指示書 - ym2151-log-play-server

## プロジェクト概要

YM2151 (OPM) チップレジスタイベントログのサーバー・クライアントシステム。JSONイベントログを読み込み、Nuked-OPMエミュレータを使用してリアルタイム音声再生とWAVファイル出力を行います。

オリジナル実装（サーバー・クライアント組み込み前）: https://github.com/cat2151/ym2151-log-player-rust

### 主要機能
- サーバー・クライアントモード（名前付きパイプ経由IPC）
- 16進文字列対応JSONイベントログ解析 ("0x08"形式)
- cpalによるリアルタイム音声再生
- houndによるWAVファイル出力

## ビルド手順

### 前提条件
- Rust 1.70以降
- **使用禁止**: mingw, msys2, MSVC

### ビルドとテスト

```bash
# 標準ビルド
cargo build

# リリースビルド（本番推奨）
cargo build --release

# サーバーモード起動
cargo run --release -- server

# クライアントからファイル再生
cargo run --release -- client test_input.json

# サーバーシャットダウン
cargo run --release -- client --shutdown

# 全テスト実行
cargo test

# 特定テスト実行（例: サーバー関連テスト）
cargo test server_basic_test
cargo test phase7_integration_test
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
- `src/main.rs` - エントリーポイント（サーバー/クライアント分岐）
- `src/server.rs` - サーバーモード実装（名前付きパイプ待機、マルチスレッド）
- `src/client.rs` - クライアント機能（サーバーへのコマンド送信）
- `src/events.rs` - 16進対応JSONイベント解析
- `src/player.rs` - イベント処理エンジン
- `src/wav_writer.rs` - WAVファイル出力
- `src/audio.rs` - リアルタイム音声再生
- `src/ipc/` - プロセス間通信モジュール
  - `protocol.rs` - コマンド/レスポンス定義
  - `pipe_unix.rs` - Unix名前付きパイプ実装
  - `pipe_windows.rs` - Windows名前付きパイプ実装（未実装）
- `omp.c`, `opm.h` - Nuked-OPM C実装

## 実装の詳細

### イベント処理
- **入力**: Pass1形式イベント (単純レジスタ書き込み)
- **処理**: Pass2形式に変換 (アドレス書き込み + データ書き込みペア)
- **タイミング**: アドレスとデータ書き込み間にDELAY_SAMPLES (2サンプル) 挿入
- **OPMポート**: ポート0 = アドレスレジスタ, ポート1 = データレジスタ

### 音声仕様
- OPM内部レート: 55930 Hz
- 出力レート: 48000 Hz (リサンプリング必要)
- フォーマット: 16ビット符号付きステレオ

### サーバー・クライアント通信
- **通信方式**: 名前付きパイプ（Unix: `/tmp/ym2151-log-play-server.pipe`）
- **プロトコル**: テキストベース（改行区切り）
- **コマンド**: `PLAY <path>`, `STOP`, `SHUTDOWN`
- **レスポンス**: `OK`, `ERROR <message>`
- **スレッド構成**: IPCリスナー + 再生コントローラーの2スレッド分離
- **状態管理**: `Arc<Mutex<ServerState>>` + `AtomicBool`（shutdown）

### 重要なアーキテクチャ決定
- **Pass1→Pass2変換**: 単一レジスタ書き込み → アドレス+データペア
- **デュアルスレッド**: IPCブロッキングと音声処理の分離
- **チャンネル通信**: `mpsc::channel`でスレッド間PlaybackCommand送信
- **プラットフォーム分岐**: cfg(unix) で Unix 限定機能を条件分岐

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

- オリジナル実装: https://github.com/cat2151/ym2151-log-player-rust
- Nuked-OPM: https://github.com/nukeykt/Nuked-OPM
- YM2151仕様: Yamaha YM2151データシート

# userからの指示
- PRコメント
  - 作業報告は、プルリクエストのコメントに書く。document作成禁止
    - DRY原則に準拠し、「codeやbuild scriptと同じことを、documentに書いたせいで、そのdocumentが陳腐化してハルシネーションやuserレビューコスト増大や混乱ほか様々なトラブル原因になる」を防止する
    - なおissue-notes/は、userがissueごとの意図を記録する用途で使う
- test
  - Rustのunit testは、本体codeとは別ファイル（src/tests/配下）に書く。agentハルシネーションのリスクを下げる用。
  - test時は、test_client.logと、test_server.logも参考にすること。それをtest codeに含めてもよい。その場合はtest並列動作させず、clean upすること
  - 調査のbuild時は、`Get-Process | Where-Object {$_.ProcessName -eq "ym2151-log-play-server"} | Stop-Process -Force` して、exeのlockを解除してからbuildすること
  - 調査用に、rust-scriptを使ったscriptを書くときは、install-ym2151-tools.rsを参考に、依存クレートをscript内にコメント形式で記述すること。このときCargo.tomlファイルの変更は禁止する
  - test codeを書くとき、server起動系test codeでは、test_util_server_mutex.rsを利用して、他のtest codeと競合しないようmutex lockを取ること
