Last updated: 2025-11-16

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタログを、リアルタイムで再生するWindows専用アプリケーションです。
- スタンドアロンまたはサーバー・クライアントモードで、JSON音楽データを演奏し、WAV出力も可能です。
- クライアントからは、再生開始・停止・サーバーシャットダウンといった操作を柔軟に行えます。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース) ベースであり、特定のフロントエンド技術は使用していません。
- 音楽・オーディオ: YM2151 (OPM) 音源エミュレーション (Nuked-OPMライブラリを使用)、リアルタイムオーディオ再生、WAVファイル出力機能。
- 開発ツール: Rust (プログラミング言語)、Cargo (Rustのビルドシステムとパッケージマネージャー)、zig cc (Cコンパイラ)。
- テスト: `cargo test`コマンドによるユニットテストおよび統合テスト。
- ビルドツール: Cargo (Rustプロジェクトのビルド管理)、`build.rs` (C言語ライブラリのビルド処理)。
- 言語機能: FFI (Foreign Function Interface) (RustからC言語のNuked-OPMライブラリを呼び出すため)、名前付きパイプ (Windows固有のプロセス間通信)。
- 自動化・CI/CD: CI環境構築のための`setup_ci_environment.sh`スクリプト (ただし、プロジェクトの意図によりWindows環境での開発・テストが推奨されています)。
- 開発標準: `.editorconfig` (複数開発環境でのコードスタイル統一設定)。

## ファイル階層ツリー
```
📁 .cargo/
  📄 config.toml
📄 .editorconfig
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📄 _config.yml
📄 build.rs
📁 generated-docs/
📁 issue-notes/
  📖 34.md
  📖 36.md
📄 opm.c
📄 opm.h
📄 setup_ci_environment.sh
📁 src/
  📄 audio.rs
  📄 client.rs
  📄 events.rs
  📁 ipc/
    📄 mod.rs
    📄 pipe_windows.rs
    📄 protocol.rs
  📄 lib.rs
  📄 main.rs
  📄 opm.rs
  📄 opm_ffi.rs
  📄 player.rs
  📄 resampler.rs
  📄 server.rs
  📄 wav_writer.rs
📁 tests/
  📄 client_test.rs
  📄 duration_test.rs
  📁 fixtures/
    📊 complex.json
    📊 simple.json
  📄 integration_test.rs
  📄 ipc_pipe_test.rs
  📄 phase3_test.rs
  📄 phase4_test.rs
  📄 phase5_test.rs
  📄 phase6_cli_test.rs
  📄 server_basic_test.rs
  📄 server_windows_fix_test.rs
  📄 tail_generation_test.rs
  📄 test_utils.rs
```

## ファイル詳細説明
- **`.cargo/config.toml`**: RustのCargoビルド設定ファイル。プロジェクト固有のビルドオプションや環境設定を定義します。
- **`.editorconfig`**: 異なるエディタやIDE間で一貫したコードスタイルを維持するための設定ファイル。
- **`.gitignore`**: Gitバージョン管理システムが無視すべきファイルやディレクトリを指定します。
- **`Cargo.lock`**: Cargoによって管理される依存関係の正確なバージョンとチェックサムが記録されており、再現性のあるビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存関係、ビルド設定などが含まれます。
- **`LICENSE`**: プロジェクトがMITライセンスで提供されていることを示します。
- **`README.ja.md`**: プロジェクトの目的、機能、使い方などを日本語で説明する主要なドキュメント。
- **`README.md`**: プロジェクトの目的、機能、使い方などを英語（または主要言語）で説明する主要なドキュメント。
- **`_config.yml`**: (具体的な用途は提供情報から不明ですが、設定関連のファイルと推測されます。)
- **`build.rs`**: Rustのカスタムビルドスクリプト。C言語で書かれたNuked-OPMライブラリをRustプロジェクトに統合するためにコンパイル処理を行います。
- **`generated-docs/`**: プロジェクトから生成されるドキュメントを格納するためのディレクトリ。(内容の詳細は不明)
- **`issue-notes/`**: プロジェクトの課題や開発メモを記録するためのディレクトリです。（来訪者向けの概要には含めません）
- **`opm.c`**: YM2151音源チップのエミュレーションを行うNuked-OPMライブラリのC言語ソースコードです。
- **`opm.h`**: `opm.c`に対応するヘッダファイルで、C言語関数のインターフェースを定義し、RustからのFFI呼び出しに使用されます。
- **`setup_ci_environment.sh`**: 継続的インテグレーション(CI)環境をセットアップするためのシェルスクリプトです。
- **`src/audio.rs`**: オーディオデバイスへの出力に関連するロジックを実装しており、YM2151からの音声をリアルタイムで再生します。
- **`src/client.rs`**: サーバー・クライアントモードにおけるクライアント側の処理を定義。サーバーに再生コマンドや制御コマンドを送信します。
- **`src/events.rs`**: YM2151のレジスタ操作イベントのデータ構造（JSON形式）と、そのパース・処理ロジックを扱います。
- **`src/ipc/mod.rs`**: プロセス間通信（IPC）機能のルートモジュール。`ipc`ディレクトリ内の他のモジュールをまとめます。
- **`src/ipc/pipe_windows.rs`**: Windowsの「名前付きパイプ」を利用したプロセス間通信の実装を提供します。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間でやり取りされるコマンド（再生、停止、シャットダウンなど）のプロトコルを定義します。
- **`src/lib.rs`**: このRustプロジェクトのライブラリクレートのエントリポイントで、他のモジュールが利用する共通機能を提供する可能性があります。
- **`src/main.rs`**: アプリケーションのメインエントリポイント。コマンドライン引数を解析し、スタンドアロンモード、サーバーモード、クライアントモードのいずれかを実行します。
- **`src/opm.rs`**: YM2151エミュレータのRustラッパーを提供し、`opm_ffi.rs`を通じてC言語のNuked-OPMと連携します。
- **`src/opm_ffi.rs`**: C言語で書かれたNuked-OPMライブラリの関数をRustから呼び出すためのForeign Function Interface (FFI) バインディングを定義します。
- **`src/player.rs`**: 音楽イベントのスケジューリングと、YM2151エミュレータへのレジスタ書き込みを管理し、音楽再生の中心的なロジックを担います。
- **`src/resampler.rs`**: オーディオデータのサンプリングレート変換に関連する機能を提供します。
- **`src/server.rs`**: サーバー・クライアントモードにおけるサーバー側の処理を定義。クライアントからのコマンドを受信し、音楽再生をバックグラウンドで管理します。
- **`src/wav_writer.rs`**: 生成されたオーディオデータをWAVファイル形式で出力するためのロジックを実装します。
- **`tests/`**: プロジェクトのテストコードを格納するディレクトリです。
- **`tests/client_test.rs`**: クライアント機能のテストコード。
- **`tests/duration_test.rs`**: 音楽の再生時間やイベントタイミングに関するテストコード。
- **`tests/fixtures/complex.json`**: 複雑なパターンを含むYM2151イベントログのテストデータ。
- **`tests/fixtures/simple.json`**: シンプルなYM2151イベントログのテストデータ。
- **`tests/integration_test.rs`**: アプリケーションの主要なコンポーネントを結合した統合テストコード。
- **`tests/ipc_pipe_test.rs`**: プロセス間通信（名前付きパイプ）の機能テストコード。
- **`tests/phaseX_test.rs`**: 特定の開発フェーズに関連するテスト群。（例: `phase3_test.rs`, `phase4_test.rs` など）
- **`tests/server_basic_test.rs`**: サーバーの基本的な動作を確認するテストコード。
- **`tests/server_windows_fix_test.rs`**: Windows環境特有の問題に対するサーバーの修正を検証するテストコード。
- **`tests/tail_generation_test.rs`**: 音源のリリース（余韻）生成に関するテストコード。
- **`tests/test_utils.rs`**: テストコード内で共通して使用されるユーティリティ関数やヘルパー。

## 関数詳細説明
提供されたプロジェクト情報からは、各関数の具体的な役割、引数、戻り値、機能を詳細に特定できませんでした。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした
```

---
Generated at: 2025-11-16 07:02:07 JST
