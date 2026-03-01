Last updated: 2026-03-02

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- サーバーはバックグラウンドで常駐し、クライアントからJSON形式の音楽データを送信して演奏を制御します。
- プログラマティックな利用やコマンドラインからの操作が可能で、低遅延のインタラクティブな音楽制御もサポートします。

## 技術スタック
- フロントエンド: このプロジェクト自体は直接的なユーザーインターフェースを持たず、外部のクライアントアプリケーション（例: `cat-play-mml`, `ym2151-tone-editor`）からライブラリとして利用されることを想定しています。
- 音楽・オーディオ:
    - YM2151 (OPM): ヤマハYM2151（FM音源チップ）のエミュレーションコア。
    - Nuked-OPM: YM2151エミュレーションに使用されるC言語製のコアライブラリ（LGPL 2.1ライセンス）。
    - リアルタイムオーディオ処理: オーディオストリームを生成し、OSのオーディオデバイスへ出力するためのコンポーネント。
    - WAVファイル出力: `--verbose`モード時に生成される音源をWAV形式で保存する機能。
- 開発ツール:
    - Rust: プロジェクトの主要なプログラミング言語（バージョン1.70以降が必須）。
    - Cargo: Rustのビルドシステムおよびパッケージマネージャー。
    - `rust-script`: Rustスクリプトを実行するためのツール。
    - Visual Studio Code: `.vscode`ディレクトリの存在から、開発環境として利用されていることが伺えます。
- テスト:
    - Cargo Test: Rustの標準テストフレームワーク。
    - Nextest: `.config/nextest.toml`から、高速なテストランナーが使用されていることが伺えます。
- ビルドツール:
    - Cargo: Rustプロジェクトのビルドを管理します。
    - Cコンパイラ: `build.rs`の存在とCソースファイル (`opm.c`, `call_opm_clock_64times.c`) から、C言語コードのコンパイルも行われます。
- 言語機能:
    - Rust FFI (Foreign Function Interface): C言語で書かれたNuked-OPMとの連携に使用されます。
    - 非同期処理: サーバー・クライアント間の通信やオーディオ処理で利用されている可能性があります。
- 自動化・CI/CD:
    - `setup_ci_environment.sh`: CI/CD環境設定のためのシェルスクリプトが存在します。
- 開発標準:
    - EditorConfig: `.editorconfig`ファイルにより、異なるエディタやIDE間でコードのスタイルを統一します。
    - CodeQL: `_codeql_detected_source_root`ディレクトリの存在から、静的解析ツールが導入されていることが示唆されます。

## ファイル階層ツリー
```
📁 .
├── .config/
│   └── nextest.toml
├── .editorconfig
├── .gitignore
├── .vscode/
│   ├── extensions.json
│   └── settings.json
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.ja.md
├── README.md
├── _config.yml
├── _codeql_detected_source_root/  (CodeQL解析ツールによって生成されたディレクトリ。実際のプロジェクト構造とは異なり、冗長な内容が含まれるため、詳細は省略します。)
├── build.rs
├── call_opm_clock_64times.c
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── install-ym2151-tools.rs
├── issue-notes/
│   ├── 100.md
│   ├── 101.md
│   ├── 102.md
│   ├── 110.md
│   ├── 111.md
│   ├── 112.md
│   ├── 113.md
│   ├── 116.md
│   ├── 117.md
│   ├── 118.md
│   ├── 122.md
│   ├── 123.md
│   ├── 138.md
│   ├── 165.md
│   ├── 169.md
│   ├── 178.md
│   ├── 181.md
│   ├── 96.md
│   ├── 97.md
│   ├── 98.md
│   └── 99.md
├── opm.c
├── opm.h
├── output_ym2151.json
├── setup_ci_environment.sh
├── src/
│   ├── audio/
│   │   ├── buffers.rs
│   │   ├── commands.rs
│   │   ├── generator.rs
│   │   ├── mod.rs
│   │   ├── player.rs
│   │   ├── scheduler.rs
│   │   └── stream.rs
│   ├── audio_config.rs
│   ├── client/
│   │   ├── config.rs
│   │   ├── core.rs
│   │   ├── interactive.rs
│   │   ├── json.rs
│   │   ├── mod.rs
│   │   └── server.rs
│   ├── debug_wav.rs
│   ├── demo_client_interactive.rs
│   ├── demo_server_interactive.rs
│   ├── demo_server_non_interactive.rs
│   ├── events.rs
│   ├── ipc/
│   │   ├── mod.rs
│   │   ├── pipe_windows.rs
│   │   ├── protocol.rs
│   │   └── windows/
│   │       ├── mod.rs
│   │       ├── pipe_factory.rs
│   │       ├── pipe_handle.rs
│   │       ├── pipe_reader.rs
│   │       ├── pipe_writer.rs
│   │       └── test_logging.rs
│   ├── lib.rs
│   ├── logging.rs
│   ├── main.rs
│   ├── mmcss.rs
│   ├── opm.rs
│   ├── opm_ffi.rs
│   ├── player.rs
│   ├── resampler.rs
│   ├── scheduler.rs
│   ├── server/
│   │   ├── command_handler.rs
│   │   ├── connection.rs
│   │   ├── mod.rs
│   │   ├── playback.rs
│   │   └── state.rs
│   ├── tests/
│   │   ├── audio_tests.rs
│   │   ├── client_tests.rs
│   │   ├── command_handler_tests.rs
│   │   ├── debug_wav_tests.rs
│   │   ├── demo_server_interactive_tests.rs
│   │   ├── demo_server_non_interactive_tests.rs
│   │   ├── events_tests.rs
│   │   ├── ipc_pipe_windows_tests.rs
│   │   ├── ipc_protocol_tests.rs
│   │   ├── logging_tests.rs
│   │   ├── mmcss_tests.rs
│   │   ├── mod.rs
│   │   ├── opm_ffi_tests.rs
│   │   ├── opm_tests.rs
│   │   ├── play_json_interactive_tests.rs
│   │   ├── player_tests.rs
│   │   ├── resampler_tests.rs
│   │   ├── scheduler_tests.rs
│   │   ├── server_tests.rs
│   │   └── wav_writer_tests.rs
│   └── wav_writer.rs
└── tests/
    ├── audio/
    │   ├── audio_playback_test.rs
    │   ├── audio_sound_test.rs
    │   └── mod.rs
    ├── clear_schedule_test.rs
    ├── cli_integration_test.rs
    ├── client_json_test.rs
    ├── client_test.rs
    ├── client_verbose_test.rs
    ├── debug_wav_test.rs
    ├── duration_test.rs
    ├── ensure_server_ready_test.rs
    ├── events_processing_test.rs
    ├── feature_demonstration_test.rs
    ├── fixtures/
    │   ├── complex.json
    │   └── simple.json
    ├── integration_test.rs
    ├── interactive/
    │   ├── mod.rs
    │   ├── mode_test.rs
    │   ├── play_json_test.rs
    │   ├── shared_mutex.rs
    │   └── step_by_step_test.rs
    ├── interactive_tests.rs
    ├── ipc_pipe_test.rs
    ├── logging_test.rs
    ├── server_basic_test.rs
    ├── server_integration_test.rs
    ├── tail_generation_test.rs
    └── test_util_server_mutex.rs
```

## ファイル詳細説明
- **`.config/nextest.toml`**: テストランナー`Nextest`の設定ファイル。テスト実行に関する詳細な設定を定義します。
- **`.editorconfig`**: コードエディタ間でインデントや文字コードなどのコーディングスタイルを統一するための設定ファイル。
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定するファイル。
- **`.vscode/extensions.json`**: Visual Studio Codeの推奨拡張機能を定義するファイル。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定ファイル。
- **`Cargo.lock`**: `Cargo.toml`に基づいて解決された依存関係の正確なバージョンを記録するファイル。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョン、依存クレートなど）を定義するマニフェストファイル。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載したファイル。
- **`README.ja.md`**: プロジェクトの日本語版説明書。
- **`README.md`**: プロジェクトの英語版説明書。
- **`_config.yml`**: Jekyllなどの静的サイトジェネレーターの設定ファイルである可能性があり、ドキュメンテーション生成に関連する場合があります。
- **`_codeql_detected_source_root/`**: CodeQLなどの静的解析ツールによって生成または検出されたソースルートの繰り返しで、実際のプロジェクト構造とは異なります。解析の一時ファイルやシンボリックリンクの結果として発生した可能性があります。
- **`build.rs`**: Rustプロジェクトのビルドプロセス中に実行されるカスタムビルドスクリプト。主にC言語ライブラリのビルドなどを設定します。
- **`call_opm_clock_64times.c`**: Nuked-OPMエミュレータのクロックを64回呼び出すためのC言語ソースファイル。主にFFIを通じてRustから利用されます。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発状況に関するドキュメントの一部。
- **`googled947dc864c270e07.html`**: Googleサイト認証などの目的で使用される可能性のあるHTMLファイル。
- **`install-ym2151-tools.rs`**: 関連ツールを一括インストールするためのRustスクリプト。
- **`issue-notes/`**: 開発中の課題やメモが格納されたディレクトリ。来訪者向けのため詳細は割愛します。
- **`opm.c`**: Nuked-OPMエミュレータのコアとなるC言語ソースファイル。YM2151のレジスタ操作と音源生成ロジックを含みます。
- **`opm.h`**: `opm.c`に対応するC言語ヘッダーファイル。Nuked-OPMエミュレータのAPIを提供します。
- **`output_ym2151.json`**: YM2151のレジスタイベントログのサンプルデータとして使用されるJSONファイル。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングを管理するロジックが含まれます。
- **`src/audio/commands.rs`**: オーディオ処理に関連するコマンドやメッセージの定義が含まれます。
- **`src/audio/generator.rs`**: YM2151エミュレータからオーディオサンプルを生成するロジックが含まれます。
- **`src/audio/mod.rs`**: `audio`モジュールのルートファイル。サブモジュールを公開し、オーディオ関連機能の全体を構成します。
- **`src/audio/player.rs`**: オーディオ再生を担当するメインロジック。オーディオデバイスへの出力などを管理します。
- **`src/audio/scheduler.rs`**: YM2151イベントのスケジューリング（いつどのレジスタに書き込むか）を管理します。
- **`src/audio/stream.rs`**: オーディオストリームの管理とオーディオデバイスとのインターフェースを提供します。
- **`src/audio_config.rs`**: オーディオ出力に関する設定（サンプリングレート、バッファサイズなど）を定義します。
- **`src/client/config.rs`**: クライアント側の設定（サーバー接続情報など）を定義します。
- **`src/client/core.rs`**: クライアント機能のコアロジック。サーバーとの通信や基本的なコマンド送信を処理します。
- **`src/client/interactive.rs`**: リアルタイムかつ連続的な音楽制御を行うインタラクティブモードのクライアントロジック。
- **`src/client/json.rs`**: JSON形式の音楽データをパースし、YM2151イベントに変換するロジック。
- **`src/client/mod.rs`**: `client`モジュールのルートファイル。クライアント関連機能の全体を構成します。
- **`src/client/server.rs`**: クライアントからサーバーの起動・停止を制御するためのロジック。
- **`src/debug_wav.rs`**: デバッグ目的でWAVファイルを生成・保存する機能。
- **`src/demo_client_interactive.rs`**: インタラクティブクライアントのデモンストレーションコード。
- **`src/demo_server_interactive.rs`**: インタラクティブサーバーモードのデモンストレーションコード。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブサーバーモードのデモンストレーションコード。
- **`src/events.rs`**: YM2151のレジスタイベントのデータ構造を定義します。
- **`src/ipc/mod.rs`**: IPC（プロセス間通信）モジュールのルートファイル。サブモジュールを公開します。
- **`src/ipc/pipe_windows.rs`**: Windows版の名前付きパイプを利用したIPCの実装。
- **`src/ipc/protocol.rs`**: サーバー・クライアント間で交換される通信プロトコル（コマンドやデータ形式）を定義します。
- **`src/ipc/windows/mod.rs`**: Windows固有のIPC実装のサブモジュール群。
- **`src/ipc/windows/pipe_factory.rs`**: 名前付きパイプを作成するためのファクトリ機能。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsの名前付きパイプハンドルを抽象化する機能。
- **`src/ipc/windows/pipe_reader.rs`**: 名前付きパイプからデータを読み込む機能。
- **`src/ipc/windows/pipe_writer.rs`**: 名前付きパイプにデータを書き込む機能。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプ関連のテスト用ロギング機能。
- **`src/lib.rs`**: プロジェクトのライブラリクレートのルートファイル。他のモジュールを公開し、外部から利用可能なAPIを定義します。
- **`src/logging.rs`**: アプリケーション全体のロギング設定と機能。
- **`src/main.rs`**: アプリケーションのエントリポイント。サーバーまたはクライアントモードの実行を制御します。
- **`src/mmcss.rs`**: Windowsのマルチメディアクラススケジューリングサービス (MMCSS) を利用して、オーディオ処理のスレッド優先度を上げるためのロジック。
- **`src/opm.rs`**: Nuked-OPMエミュレータのRustラッパー。C言語の機能とRustを橋渡しします。
- **`src/opm_ffi.rs`**: `opm.c`と`call_opm_clock_64times.c`のC関数をRustから呼び出すためのFFI定義。
- **`src/player.rs`**: YM2151イベントを処理し、Nuked-OPMを駆動してオーディオサンプルを生成する上位レベルのプレイヤーロジック。
- **`src/resampler.rs`**: オーディオデータのサンプリングレートを変換するためのリサンプリング機能。
- **`src/scheduler.rs`**: YM2151イベントのタイムベースのスケジューリングを管理し、適切なタイミングでエミュレータにレジスタ値を書き込みます。
- **`src/server/command_handler.rs`**: クライアントから受信したコマンドを解釈し、サーバーの動作を制御するロジック。
- **`src/server/connection.rs`**: サーバーとクライアント間の個々のIPC接続を管理します。
- **`src/server/mod.rs`**: `server`モジュールのルートファイル。サーバー関連機能の全体を構成します。
- **`src/server/playback.rs`**: サーバー上での実際のオーディオ再生ロジック（プレイヤーとスケジューラーの統合）。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中か、インタラクティブモードかなど）を保持するデータ構造。
- **`src/tests/`**: `src`ディレクトリ直下にあるインテグレーションテストやユニットテストのファイル群。
- **`src/wav_writer.rs`**: オーディオデータをWAVファイル形式で書き出すためのユーティリティ。
- **`tests/`**: プロジェクトのルートにある大規模なインテグレーションテストのファイル群。特にクライアント・サーバー間の連携テストやインタラクティブモードの動作検証が含まれます。

## 関数詳細説明
提供されたプロジェクト情報からは、個々の関数の詳細な役割、引数、戻り値、機能を抽出できませんでした。これは、ソースコード自体が提供されていないためです。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2026-03-02 07:02:08 JST
