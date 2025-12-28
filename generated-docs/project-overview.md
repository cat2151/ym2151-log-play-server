Last updated: 2025-12-29

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するシステムです。
- サーバー・クライアントアーキテクチャを採用し、柔軟な音楽再生制御とWAVファイル出力に対応します。
- Rustで実装され、Windowsプラットフォーム上で高い応答性と安定した動作を提供します。

## 技術スタック
- フロントエンド: このプロジェクトには専用のGUIフロントエンドはなく、コマンドラインインターフェースまたはRustライブラリとして利用されます。
- 音楽・オーディオ:
    - YM2151 (OPM): ヤマハのFM音源チップ。このプロジェクトは、そのレジスタイベントログをエミュレートして再生します。
    - Nuked-OPM: YM2151チップの動作を忠実に再現するC言語ベースのエミュレーションライブラリ。
    - MMCSS (Multimedia Class Scheduler Service): WindowsOS上でオーディオ処理の優先度を管理し、リアルタイム性が要求される再生品質を保証します。
- 開発ツール:
    - Rust: プロジェクトの主要なプログラミング言語。パフォーマンスと安全性に優れています。
    - Cargo: Rustのビルドシステムおよびパッケージマネージャー。依存関係の管理とプロジェクトのビルドを効率化します。
    - anyhow: Rustでエラーハンドリングを簡潔かつ強力に行うためのライブラリ。
    - clap: コマンドライン引数をパースし、CLIアプリケーションの構築を容易にするライブラリ。
    - serde & serde_json: Rustのデータ構造をJSON形式でシリアライズ・デシリアライズするためのライブラリ。
    - rust-script: Rustスクリプトを直接実行するためのツール。開発環境のセットアップスクリプトに利用されます。
- テスト:
    - Cargo test: Rustプロジェクトに組み込まれた単体テストおよび統合テストフレームワーク。
    - Nextest: 高速なRustテストランナー。
- ビルドツール:
    - Rust (Cargo): プロジェクトのビルドとパッケージ管理を行います。
    - build.rs: Rustのカスタムビルドスクリプト。C言語ライブラリ（Nuked-OPM）のFFIバインディング生成などに利用されます。
- 言語機能:
    - Rust 1.70以降: プロジェクトのビルドに必要なRustコンパイラのバージョン。
- 自動化・CI/CD:
    - setup_ci_environment.sh: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。
- 開発標準:
    - .editorconfig: 開発チーム全体でコードスタイルを統一するための設定ファイル。
    - .gitignore: Gitバージョン管理システムで無視するファイルやディレクトリを指定します。

## ファイル階層ツリー
```
.
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
├── build.rs
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
├── install-ym2151-tools.rs
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
- **`.config/nextest.toml`**: Nextestテストランナーのカスタム設定ファイル。テストの並列実行や出力形式などを定義します。
- **`.editorconfig`**: さまざまなエディタやIDE間でコードのフォーマットスタイル（インデント、改行など）を統一するための設定ファイル。
- **`.gitignore`**: Gitバージョン管理システムが無視すべきファイルやディレクトリのパターンを指定します。ビルド生成物や一時ファイルなどが含まれます。
- **`.vscode/extensions.json`**: Visual Studio Codeのワークスペースで推奨される拡張機能をリストアップします。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義します。コードフォーマットやリンティングルールなど。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンとチェックサムを記録し、再現可能なビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョン、著者など）と、依存関係、機能フラグ、ビルドターゲットなどの設定を定義するマニフェストファイル。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載したファイル。
- **`README.ja.md`**: プロジェクトの日本語による概要、機能、使用方法などが記述されたマークダウンファイル。
- **`README.md`**: プロジェクトの英語による概要、機能、使用方法などが記述されたマークダウンファイル。
- **`_config.yml`**: Jekyllなどの静的サイトジェネレーターの設定ファイルで、ドキュメントサイトの構築に使用されます（存在する場合）。
- **`build.rs`**: Cargoがビルドの初期段階で実行するカスタムビルドスクリプト。CFFIバインディングの生成や環境設定などを行います。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発状況に関する情報の断片。
- **`googled947dc864c270e07.html`**: Googleサイトの所有権確認などに使われる検証用HTMLファイル。
- **`install-ym2151-tools.rs`**: 関連するRustツールやクレートを一括でインストールするためのスクリプト。
- **`opm.c`**: Nuked-OPM YM2151エミュレーターのC言語ソースコード。実際のYM2151音源チップの動作をシミュレートします。
- **`opm.h`**: `opm.c`に対応するC言語ヘッダーファイル。Nuked-OPMエミュレーターのAPI定義やデータ構造を含みます。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルデータ、またはテスト用として使用されるJSONファイル。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境を初期設定するためのシェルスクリプト。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングメカニズムを実装し、スムーズな音声再生を可能にします。
- **`src/audio/commands.rs`**: オーディオ再生システムに対するコマンド（例: 再生開始、停止、一時停止）のデータ構造と処理ロジックを定義します。
- **`src/audio/generator.rs`**: YM2151エミュレーターからの生データをオーディオストリームとして生成する主要なコンポーネントです。
- **`src/audio/mod.rs`**: `src/audio`モジュール全体のルートファイルであり、サブモジュールを公開します。
- **`src/audio/player.rs`**: オーディオ再生のコアロジックをカプセル化し、生成されたオーディオデータをデバイスに出力する役割を担います。
- **`src/audio/scheduler.rs`**: YM2151レジスタイベントを正確な時間（サンプル単位）で実行するためのスケジューリング機能を提供します。
- **`src/audio/stream.rs`**: オーディオデバイスとのインターフェースを提供し、生成されたオーディオデータをリアルタイムでOSのオーディオ出力に流し込みます。
- **`src/audio_config.rs`**: オーディオ再生に関する設定（サンプルレート、チャンネル数、バッファサイズなど）を管理します。
- **`src/client/config.rs`**: クライアントアプリケーションの実行時設定（例えば、サーバー接続設定など）を定義します。
- **`src/client/core.rs`**: クライアントの基本的な機能、特にサーバーとの低レベルな通信プロトコルを処理します。
- **`src/client/interactive.rs`**: インタラクティブモードでのクライアントサイドのロジックを実装します。リアルタイムでのイベント送信やスケジュールクリアなどの機能を提供します。
- **`src/client/json.rs`**: YM2151イベントログのJSONフォーマットと、そのパースおよびシリアライズのロジックを定義します。
- **`src/client/mod.rs`**: `src/client`モジュール全体のルートファイルであり、サブモジュールを公開します。
- **`src/client/server.rs`**: クライアントがサーバープロセスのライフサイクル（起動、停止、状態確認など）を管理するための機能を提供します。
- **`src/debug_wav.rs`**: デバッグ目的で生成されるWAVファイルに関するユーティリティ機能。
- **`src/demo_client_interactive.rs`**: インタラクティブモードのクライアント機能のデモンストレーションコード。
- **`src/demo_server_interactive.rs`**: インタラクティブモードのサーバー機能のデモンストレーションコード。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードのサーバー機能のデモンストレーションコード。
- **`src/events.rs`**: YM2151レジスタイベントのデータ構造を定義します。これは音楽データを表現する基本単位です。
- **`src/ipc/mod.rs`**: `src/ipc`モジュール全体のルートファイルであり、プロセス間通信（IPC）に関連するサブモジュールを公開します。
- **`src/ipc/pipe_windows.rs`**: Windowsオペレーティングシステムに特化した名前付きパイプによるプロセス間通信の実装を提供します。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間で交換されるメッセージのプロトコル（コマンド、データ形式など）を定義します。
- **`src/ipc/windows/mod.rs`**: `src/ipc/windows`モジュール全体のルートファイル。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを作成・管理するためのファクトリパターンを実装します。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsのネイティブパイプハンドルを抽象化し、より安全でRustらしいインターフェースを提供します。
- **`src/ipc/windows/pipe_reader.rs`**: Windowsの名前付きパイプからデータを非同期的に読み取るための機能を実装します。
- **`src/ipc/windows/pipe_writer.rs`**: Windowsの名前付きパイプにデータを非同期的に書き込むための機能を実装します。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプ通信のテスト時におけるログ記録を補助するユーティリティ。
- **`src/lib.rs`**: プロジェクトがライブラリとして提供される場合のエントリポイント。公開APIを定義します。
- **`src/logging.rs`**: アプリケーション全体で使用されるロギングシステム（例: `env_logger`など）の設定とヘルパー機能を提供します。
- **`src/main.rs`**: 実行可能バイナリのエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントとしての役割を決定して起動します。
- **`src/mmcss.rs`**: WindowsのMMCSSサービスと連携し、オーディオ再生スレッドの優先度をリアルタイム処理に適したレベルに昇格させます。
- **`src/opm.rs`**: FFI (Foreign Function Interface) を通じて`opm.c`のNuked-OPMライブラリをRustコードから安全に利用するためのラッパー層を提供します。
- **`src/opm_ffi.rs`**: Nuked-OPM Cライブラリへの低レベルなFFIバインディングを定義します。RustとCの境界を確立します。
- **`src/player.rs`**: 高レベルなオーディオ再生制御ロジックを実装します。スケジューラーからのイベントを受け取り、オーディオジェネレーターを駆動します。
- **`src/resampler.rs`**: YM2151エミュレーターが出力するオーディオのサンプルレートを、OSやオーディオデバイスが要求するサンプルレートに変換（リサンプリング）する機能を提供します。
- **`src/scheduler.rs`**: 時間ベースでYM2151レジスタイベントの処理を管理し、イベントを適切なタイミングで`player`に供給します。
- **`src/server/command_handler.rs`**: クライアントからIPC経由で送信されたコマンドを解釈し、サーバーの内部状態や再生に反映させるロジックを実装します。
- **`src/server/connection.rs`**: クライアントからのIPC接続を管理し、複数のクライアントからのリクエストを同時に処理できるメカニズムを提供します。
- **`src/server/mod.rs`**: `src/server`モジュール全体のルートファイルであり、サーバーサイドのサブモジュールを公開します。
- **`src/server/playback.rs`**: サーバーサイドでのオーディオ再生を管理する中核コンポーネント。再生モード（インタラクティブ/非インタラクティブ）の切り替えや状態維持を行います。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中か、どのモードか、スケジュールされたイベントなど）を保持し、スレッドセーフにアクセスするためのデータ構造とロジックを定義します。
- **`src/tests/*.rs`**: `src/`ディレクトリ内の各モジュールに対する単体テストファイル。
- **`src/wav_writer.rs`**: リアルタイムで生成されるオーディオデータを標準WAVファイル形式で記録する機能を提供します。
- **`tests/audio/*.rs`**: オーディオ再生やサウンド出力に関するインテグレーションテストファイル。
- **`tests/clear_schedule_test.rs`**: インタラクティブモードでのスケジュールクリア機能の動作を検証するインテグレーションテスト。
- **`tests/cli_integration_test.rs`**: コマンドラインインターフェース（CLI）の機能が正しく動作するかを検証するインテグレーションテスト。
- **`tests/client_json_test.rs`**: クライアントからJSONデータを送信し、サーバーが正しく処理するかを検証するインテグレーションテスト。
- **`tests/client_test.rs`**: クライアント側の主要な機能の動作を検証するインテグレーションテスト。
- **`tests/client_verbose_test.rs`**: クライアントの冗長（verbose）モードの出力と動作を検証するインテグレーションテスト。
- **`tests/debug_wav_test.rs`**: デバッグWAV出力機能が期待通りに動作するかを検証するインテグレーションテスト。
- **`tests/duration_test.rs`**: 演奏時間の計測や同期に関する機能のインテグレーションテスト。
- **`tests/ensure_server_ready_test.rs`**: サーバーの自動起動・確認機能が正しく動作するかを検証するインテグレーションテスト。
- **`tests/events_processing_test.rs`**: YM2151イベントの処理ロジックが正しく動作するかを検証するインテグレーションテスト。
- **`tests/feature_demonstration_test.rs`**: 特定の機能や使用例をデモンストレーションする形式のインテグレーションテスト。
- **`tests/fixtures/complex.json`**: 複雑なパターンを含むYM2151イベントログのテスト用JSONデータ。
- **`tests/fixtures/simple.json`**: シンプルなパターンを含むYM2151イベントログのテスト用JSONデータ。
- **`tests/integration_test.rs`**: プロジェクト全体の主要なコンポーネント間の連携を検証する総合的なインテグレーションテスト。
- **`tests/interactive/mod.rs`**: `tests/interactive`モジュール全体のルートファイル。
- **`tests/interactive/mode_test.rs`**: インタラクティブモードへの切り替えやその状態の動作を検証するテスト。
- **`tests/interactive/play_json_test.rs`**: インタラクティブモードでのJSON再生機能の動作を検証するテスト。
- **`tests/interactive/shared_mutex.rs`**: インタラクティブモードで使用される共有ミューテックスの動作と安全性に関するテスト。
- **`tests/interactive/step_by_step_test.rs`**: インタラクティブモードの機能をステップバイステップで確認する詳細なテスト。
- **`tests/interactive_tests.rs`**: インタラクティブ機能全般のインテグレーションテスト。
- **`tests/ipc_pipe_test.rs`**: プロセス間通信（IPC）パイプの機能と信頼性を検証するインテグレーションテスト。
- **`tests/logging_test.rs`**: ロギング機能が適切に動作し、必要な情報が出力されるかを検証するインテグレーションテスト。
- **`tests/server_basic_test.rs`**: サーバーの基本的な起動、待機、シャットダウンなどの機能を検証するインテグレーションテスト。
- **`tests/server_integration_test.rs`**: サーバーがクライアントからのコマンドを適切に処理し、オーディオ再生を行うかなど、サーバーの総合的な動作を検証するインテグレーションテスト。
- **`tests/tail_generation_test.rs`**: 音楽終了後の余韻（tail）が適切に生成されるかを検証するテスト。
- **`tests/test_util_server_mutex.rs`**: テストで使用されるサーバーの排他制御メカニズムに関するユーティリティテスト。

## 関数詳細説明
- `main()`
    - 役割: アプリケーションの開始点。コマンドライン引数に基づき、サーバーまたはクライアントとして実行を分岐させます。
    - 引数: なし (コマンドライン引数は内部でパースされます)。
    - 戻り値: `anyhow::Result<()>` (操作の成功または失敗を示す結果)。
    - 機能: プログラムの初期化を行い、ユーザーが指定したモード（サーバー/クライアント）に応じて適切な処理ロジックを呼び出します。
- `client::ensure_server_ready(app_name: &str)`
    - 役割: YM2151再生サーバーが実行中であることを確認し、必要に応じて自動的に起動します。
    - 引数: `app_name: &str` - サーバーアプリケーションを識別し、インストール・起動する際の参照名。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: サーバープロセスが存在しない場合、`cargo`経由でサーバーをインストールし、バックグラウンドで起動します。その後、サーバーがコマンドを受け付けられる状態になるまで待機します。
- `client::send_json(json_data: &str)`
    - 役割: 非インタラクティブモードで、YM2151レジスタイベントログのJSONデータをサーバーに送信し、再生を開始します。
    - 引数: `json_data: &str` - 再生するYM2151イベントデータを含むJSON形式の文字列。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: サーバーにJSONデータを送信し、既存の再生を停止させて新しい音楽データでの再生に切り替えます。
- `client::stop_playback()`
    - 役割: サーバーに現在のYM2151音楽再生を即座に停止するよう指示します。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: サーバー上で実行中のオーディオストリームとイベントスケジュールを中断し、無音状態にします。
- `client::shutdown_server()`
    - 役割: サーバープロセスを安全に終了させるためのコマンドを送信します。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: サーバーが現在の処理を完了し、リソースを解放して正常にシャットダウンするように促します。
- `client::start_interactive()`
    - 役割: サーバーをインタラクティブモードに切り替え、連続したオーディオストリームを維持する再生を開始します。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: リアルタイムでの音響制御を可能にするために、サーバーの再生状態を連続ストリームモードに設定します。
- `client::play_json_interactive(json_data: &str)`
    - 役割: インタラクティブモードでYM2151レジスタイベントログのJSONデータをサーバーに送信し、現在のオーディオストリームにイベントを動的にスケジューリングします。
    - 引数: `json_data: &str` - スケジュールするYM2151イベントデータを含むJSON形式の文字列。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: 既に再生中の音楽を中断することなく、新しいイベントをリアルタイムで再生キューに追加します。サンプル単位の時間は自動的に秒単位に変換されます。
- `client::clear_schedule()`
    - 役割: インタラクティブモードにおいて、サーバーの将来のYM2151イベントスケジュールをすべてキャンセルします。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: 現在再生中のイベントは継続しますが、まだ処理されていない全てのイベントを破棄し、即座に新しいイベントのスケジューリングを可能にします。
- `client::get_server_time()`
    - 役割: サーバーの現在のオーディオ再生時刻を問い合わせ、クライアントに同期情報を提供します。
    - 引数: なし。
    - 戻り値: `anyhow::Result<f64>` - 現在のサーバー時刻を秒単位の`f64`値で返します。
    - 機能: Web Audio APIの`currentTime`プロパティのように、正確なタイミング制御が必要な場合に利用されます。
- `client::stop_interactive()`
    - 役割: サーバーのインタラクティブモードを終了し、連続オーディオストリームを停止します。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>`。
    - 機能: リアルタイム音楽制御セッションを終了し、サーバーをアイドル状態に戻します。

## 関数呼び出し階層ツリー
```
[利用可能な詳細情報が提供されていないため、関数呼び出し階層ツリーを生成することはできません。
プロジェクト情報からは、`main`関数がクライアントモードとサーバーモードの開始点となり、
クライアントモードでは`client::ensure_server_ready`、`client::send_json`、
`client::stop_playback`、`client::shutdown_server`などの関数が呼び出されることが示唆されています。
インタラクティブモードでは、`client::start_interactive`、`client::play_json_interactive`、
`client::clear_schedule`、`client::get_server_time`、`client::stop_interactive`などが呼び出されます。
サーバー側では、クライアントからのコマンドを受け取る`server::command_handler`が中心となり、
`audio::player`や`audio::scheduler`などのモジュール内の関数を呼び出して実際の音源再生を制御すると推測されます。]

---
Generated at: 2025-12-29 07:02:56 JST
