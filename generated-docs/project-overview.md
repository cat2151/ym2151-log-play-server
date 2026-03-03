Last updated: 2026-03-04

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタイベントログをリアルタイムで高音質再生するサーバー・クライアントシステムです。
- Windowsプラットフォームに特化し、プロセス間通信に名前付きパイプを利用して高速な演奏制御を実現します。
- ライブラリとしても提供され、他のアプリケーションからの音楽データ送信やリアルタイム操作が可能です。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース)、またはライブラリとして他のアプリケーション (例: `cat-play-mml`, `ym2151-tone-editor`) から利用。
- 音楽・オーディオ:
    - YM2151 (OPM) レジスタイベント: ヤマハFM音源チップYM2151のレジスタ操作データ。
    - Nuked-OPM: YM2151エミュレーションライブラリ (C言語)。
    - WAVファイル出力: 詳細ログモード時における音声出力。
    - リアルタイム演奏: 低遅延での音声再生。
    - リサンプリング: 音声データのサンプリングレート変換。
- 開発ツール:
    - Rust 1.70+: プロジェクトの主要開発言語およびランタイム環境。
    - Cargo: Rustの公式ビルドシステムおよびパッケージマネージャー。
    - rust-script: Rustスクリプトの実行を容易にするツール（インストールスクリプトで使用）。
    - VS Code: 開発環境の推奨エディタ。
- テスト:
    - Cargo test: Rustの標準テストランナー。
    - Nextest: 高速なRustテストランナー (`.config/nextest.toml`)。
    - 単体テスト・統合テスト: `src/tests/` およびトップレベル `tests/` ディレクトリに多数のテストコード。
- ビルドツール:
    - Cargo: Rustクレートのビルドを管理。
    - `build.rs`: C言語ソース (`opm.c`, `call_opm_clock_64times.c`) をコンパイルするためのカスタムビルドスクリプト。
- 言語機能:
    - Rust: 安全性とパフォーマンスを重視したシステムプログラミング言語。
    - C言語: YM2151エミュレーションコア (Nuked-OPM) の実装に使用。
- 自動化・CI/CD:
    - `setup_ci_environment.sh`: 継続的インテグレーション環境のセットアップスクリプト。
- 開発標準:
    - EditorConfig: 異なるエディタやIDE間でのコーディングスタイルを統一。

## ファイル階層ツリー
```
📁 .config/
  📄 nextest.toml
📄 .editorconfig
📄 .gitignore
📁 .vscode/
  📊 extensions.json
  📊 settings.json
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📄 _config.yml
📄 build.rs
📄 call_opm_clock_64times.c
📁 generated-docs/
  📖 development-status-generated-prompt.md
🌐 googled947dc864c270e07.html
📄 install-ym2151-tools.rs
📁 issue-notes/
  📖 100.md
  📖 101.md
  📖 102.md
  📖 110.md
  📖 111.md
  📖 112.md
  📖 113.md
  📖 116.md
  📖 117.md
  📖 118.md
  📖 122.md
  📖 123.md
  📖 138.md
  📖 165.md
  📖 169.md
  📖 178.md
  📖 181.md
  📖 188.md
  📖 96.md
  📖 97.md
  📖 98.md
  📖 99.md
📄 opm.c
📄 opm.h
📊 output_ym2151.json
📄 setup_ci_environment.sh
📁 src/
  📁 audio/
    📄 buffers.rs
    📄 commands.rs
    📄 generator.rs
    📄 mod.rs
    📄 player.rs
    📄 scheduler.rs
    📄 stream.rs
  📄 audio_config.rs
  📁 client/
    📄 config.rs
    📄 core.rs
    📄 interactive.rs
    📄 json.rs
    📄 mod.rs
    📄 server.rs
  📄 debug_wav.rs
  📄 demo_client_interactive.rs
  📄 demo_server_interactive.rs
  📄 demo_server_non_interactive.rs
  📄 events.rs
  📁 ipc/
    📄 mod.rs
    📄 pipe_windows.rs
    📄 protocol.rs
    📁 windows/
      📄 mod.rs
      📄 pipe_factory.rs
      📄 pipe_handle.rs
      📄 pipe_reader.rs
      📄 pipe_writer.rs
      📄 test_logging.rs
  📄 lib.rs
  📄 logging.rs
  📄 main.rs
  📄 mmcss.rs
  📄 opm.rs
  📄 opm_ffi.rs
  📄 player.rs
  📄 resampler.rs
  📄 scheduler.rs
  📁 server/
    📄 command_handler.rs
    📄 connection.rs
    📄 mod.rs
    📄 playback.rs
    📄 state.rs
  📁 tests/
    📄 audio_tests.rs
    📄 client_tests.rs
    📄 command_handler_tests.rs
    📄 debug_wav_tests.rs
    📄 demo_server_interactive_tests.rs
    📄 demo_server_non_interactive_tests.rs
    📄 events_tests.rs
    📄 ipc_pipe_windows_tests.rs
    📄 ipc_protocol_tests.rs
    📄 logging_tests.rs
    📄 mmcss_tests.rs
    📄 mod.rs
    📄 opm_ffi_tests.rs
    📄 opm_tests.rs
    📄 play_json_interactive_tests.rs
    📄 player_tests.rs
    📄 resampler_tests.rs
    📄 scheduler_tests.rs
    📄 server_tests.rs
    📄 wav_writer_tests.rs
  📄 wav_writer.rs
📁 tests/
  📁 audio/
    📄 audio_playback_test.rs
    📄 audio_sound_test.rs
    📄 mod.rs
  📄 clear_schedule_test.rs
  📄 cli_integration_test.rs
  📄 client_json_test.rs
  📄 client_test.rs
  📄 client_verbose_test.rs
  📄 debug_wav_test.rs
  📄 duration_test.rs
  📄 ensure_server_ready_test.rs
  📄 events_processing_test.rs
  📄 feature_demonstration_test.rs
  📁 fixtures/
    📊 complex.json
    📊 simple.json
  📄 integration_test.rs
  📁 interactive/
    📄 mod.rs
    📄 mode_test.rs
    📄 play_json_test.rs
    📄 shared_mutex.rs
    📄 step_by_step_test.rs
  📄 interactive_tests.rs
  📄 ipc_pipe_test.rs
  📄 logging_test.rs
  📄 server_basic_test.rs
  📄 server_integration_test.rs
  📄 tail_generation_test.rs
  📄 test_util_server_mutex.rs
```

## ファイル詳細説明
- **`.config/nextest.toml`**: Rustのテストランナー「Nextest」の設定を定義するファイル。テストの並列実行や出力形式などを制御します。
- **`.editorconfig`**: さまざまなエディタやIDE間で一貫したコーディングスタイル（インデント、改行コードなど）を維持するための設定ファイル。
- **`.gitignore`**: Gitがバージョン管理の対象から除外すべきファイルやディレクトリのパターンを定義するファイル。
- **`.vscode/extensions.json`**: Visual Studio Codeのワークスペースで推奨される拡張機能をリストアップするファイル。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義するファイル。
- **`Cargo.lock`**: Rustプロジェクトの依存関係ツリーと、ビルドに使用された各クレートの正確なバージョンを記録するファイル。再現性のあるビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存クレート、ビルド設定などが記述されています。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）が記載されています。
- **`README.ja.md`**: プロジェクトの目的、機能、使用方法などを日本語で説明する主要なドキュメント。
- **`README.md`**: プロジェクトの目的、機能、使用方法などを英語で説明する主要なドキュメント。
- **`_config.yml`**: 通常、Jekyllなどの静的サイトジェネレータで使用される設定ファイルですが、このプロジェクトでの具体的な用途は不明です。
- **`build.rs`**: Rustプロジェクトのカスタムビルドスクリプト。主にC言語のソースコード（例: `opm.c`）をコンパイルし、Rustから利用できるようにするために使用されます。
- **`call_opm_clock_64times.c`**: YM2151エミュレータのクロック処理に関連するC言語のソースファイル。Nuked-OPMとの連携に使用される可能性があります。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成されたドキュメントや開発状況に関するプロンプトが含まれるファイル。
- **`googled947dc864c270e07.html`**: Googleのサイト認証プロセスで使用されるHTMLファイルです。
- **`install-ym2151-tools.rs`**: `rust-script`を用いて実行されるRustスクリプト。YM2151関連ツールのインストール手順を自動化します。
- **`issue-notes/`**: 開発中の課題や設計上の考慮事項を記録したノート群を格納するディレクトリ。
- **`opm.c`**: Nuked-OPMライブラリの一部として、YM2151 (OPM) チップの低レベルエミュレーションロジックを実装するC言語ソースファイル。
- **`opm.h`**: `opm.c`に対応するC言語ヘッダファイルで、YM2151エミュレーションに関連する関数や構造体の宣言を含みます。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルデータを含むJSONファイル。プロジェクトの動作テストやデモンストレーションに使用されます。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするために使用されるシェルスクリプト。必要な依存関係のインストールなどを自動化します。
- **`src/audio/buffers.rs`**: 音声データの一時的な保持や管理を行うバッファに関連するロジックを定義するモジュール。
- **`src/audio/commands.rs`**: 音声再生システムに送られる内部コマンド（再生開始、停止など）の定義と処理を行うモジュール。
- **`src/audio/generator.rs`**: YM2151エミュレータから出力されるレジスタ情報を元に、実際のPCM音声データを生成するロジックを含むモジュール。
- **`src/audio/mod.rs`**: `src/audio`モジュールのルートファイル。内部のサブモジュールをまとめて公開します。
- **`src/audio/player.rs`**: システムのオーディオデバイスを制御し、生成された音声データを実際に再生する役割を担うモジュール。
- **`src/audio/scheduler.rs`**: YM2151のレジスタイベントを時間軸に沿って正確にスケジューリングし、再生タイミングを管理するモジュール。
- **`src/audio/stream.rs`**: オーディオストリームのライフサイクル管理や、オーディオデバイスとのインターフェースを提供するモジュール。
- **`src/audio_config.rs`**: オーディオ再生に関する各種設定（サンプリングレート、バッファサイズなど）を管理するモジュール。
- **`src/client/config.rs`**: クライアントアプリケーションの動作に関する設定（サーバー接続情報など）を定義するモジュール。
- **`src/client/core.rs`**: クライアントの基本的な操作ロジック（サーバーへのコマンド送信、レスポンス処理など）を実装するモジュール。
- **`src/client/interactive.rs`**: 連続的な音声ストリームを維持しつつ、リアルタイムでYM2151イベントを送信・制御する「インタラクティブモード」のクライアントロジック。
- **`src/client/json.rs`**: YM2151レジスタイベントログを含むJSONデータのパースやシリアライズを行うモジュール。
- **`src/client/mod.rs`**: `src/client`モジュールのルートファイル。クライアント関連のサブモジュールをまとめます。
- **`src/client/server.rs`**: クライアント側からサーバーとの通信を抽象化し、サーバーへの接続やコマンド送信を担当するモジュール。
- **`src/debug_wav.rs`**: デバッグ目的で、生成されたYM2151の音声データをWAVファイルとして出力する機能を提供するモジュール。
- **`src/demo_client_interactive.rs`**: インタラクティブモードクライアントの機能を示すデモンストレーションコード。
- **`src/demo_server_interactive.rs`**: インタラクティブモードでのサーバー動作を示すデモンストレーションコード。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードでのサーバー動作を示すデモンストレーションコード。
- **`src/events.rs`**: YM2151レジスタのイベントデータ構造とその処理ロジックを定義するモジュール。
- **`src/ipc/mod.rs`**: プロセス間通信（IPC）に関連する機能群をまとめるモジュール。
- **`src/ipc/pipe_windows.rs`**: Windowsのネイティブな「名前付きパイプ」機構を利用したIPCの実装モジュール。
- **`src/ipc/protocol.rs`**: クライアントとサーバー間でやり取りされるメッセージの形式やコマンドの種類を定義する通信プロトコルモジュール。
- **`src/ipc/windows/mod.rs`**: `src/ipc/windows`モジュールのルートファイル。Windows固有のIPC実装をまとめます。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを作成・初期化するファクトリパターンを実装したモジュール。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsのパイプハンドルを安全に管理するためのラッパー構造体と関連ロジックを提供するモジュール。
- **`src/ipc/windows/pipe_reader.rs`**: Windowsの名前付きパイプからデータを受信するための読み込みロジックを実装したモジュール。
- **`src/ipc/windows/pipe_writer.rs`**: Windowsの名前付きパイプにデータを送信するための書き込みロジックを実装したモジュール。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプのテスト時に利用されるデバッグログ出力機能を提供するモジュール。
- **`src/lib.rs`**: クレートのライブラリ部分のエントリポイント。他のモジュールを公開し、クレート全体の構造を定義します。
- **`src/logging.rs`**: アプリケーション全体のログ出力（情報、警告、エラーなど）を管理するためのユーティリティモジュール。
- **`src/main.rs`**: プログラムのメインエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントモードのどちらかで起動するロジックを含みます。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用し、オーディオ再生スレッドの優先度を上げてリアルタイム性能を確保するモジュール。
- **`src/opm.rs`**: YM2151エミュレータ（C言語のNuked-OPM）をRustから呼び出すためのセーフラッパーと高レベルインターフェースを提供するモジュール。
- **`src/opm_ffi.rs`**: RustからC言語のNuked-OPMライブラリ関数を直接呼び出すためのForeign Function Interface (FFI) 定義を含むモジュール。
- **`src/player.rs`**: YM2151イベントの処理、音声生成、スケジューリング、オーディオ出力までの一連の再生フローを統括する上位レベルのプレイヤーモジュール。
- **`src/resampler.rs`**: 音声のサンプリングレート変換（リサンプリング）アルゴリズムを実装するモジュール。低品質リサンプリングモードもサポートします。
- **`src/scheduler.rs`**: YM2151のレジスタイベントを正確な時間に実行するためのスケジューリングロジックを管理するモジュール。
- **`src/server/command_handler.rs`**: クライアントから受信したコマンドを解析し、サーバーの内部状態や再生処理に反映させるロジックを実装するモジュール。
- **`src/server/connection.rs`**: 複数のクライアントからの接続を管理し、それぞれとの通信を処理するサーバー側のモジュール。
- **`src/server/mod.rs`**: `src/server`モジュールのルートファイル。サーバー関連のサブモジュールをまとめます。
- **`src/server/playback.rs`**: サーバー上での実際のYM2151音声再生状態を管理し、再生開始・停止・切り替えなどの制御を行うモジュール。
- **`src/server/state.rs`**: サーバーの全体的な状態（再生状況、設定、イベントキューなど）を保持し、複数のスレッドから安全にアクセスできるように管理するモジュール。
- **`src/tests/`**: `src`モジュール内の各コンポーネントに対する単体テストコードを格納するディレクトリ。
- **`src/wav_writer.rs`**: 生の音声データをWAVファイル形式で書き出すためのユーティリティモジュール。
- **`tests/`**: プロジェクト全体の統合テストやシステムテストを格納するトップレベルディレクトリ。
- **`tests/audio/audio_playback_test.rs`**: 音声再生機能の統合的なテスト。
- **`tests/audio/audio_sound_test.rs`**: 生成される音の正確性や品質に関するテスト。
- **`tests/audio/mod.rs`**: `tests/audio`モジュールのルートファイル。
- **`tests/clear_schedule_test.rs`**: インタラクティブモードでのスケジュールクリア機能のテスト。
- **`tests/cli_integration_test.rs`**: コマンドラインインターフェースの動作検証を行う統合テスト。
- **`tests/client_json_test.rs`**: クライアントがJSONデータを送信する機能に関するテスト。
- **`tests/client_test.rs`**: クライアント機能全般のテスト。
- **`tests/client_verbose_test.rs`**: クライアントのverboseモード（詳細出力）のテスト。
- **`tests/debug_wav_test.rs`**: デバッグWAV出力機能のテスト。
- **`tests/duration_test.rs`**: 再生時間の正確性に関するテスト。
- **`tests/ensure_server_ready_test.rs`**: サーバーが起動し、コマンドを受け付けられる状態になるかを確認する機能のテスト。
- **`tests/events_processing_test.rs`**: YM2151イベントのパースと処理の正確性に関するテスト。
- **`tests/feature_demonstration_test.rs`**: プロジェクトの主要機能が正しく動作することを示すデモンストレーションテスト。
- **`tests/fixtures/complex.json`**: 複雑なYM2151イベントを含むテスト用のJSONデータ。
- **`tests/fixtures/simple.json`**: シンプルなYM2151イベントを含むテスト用のJSONデータ。
- **`tests/integration_test.rs`**: 主要なコンポーネント間の連携を検証する総合的な統合テスト。
- **`tests/interactive/mod.rs`**: `tests/interactive`モジュールのルートファイル。
- **`tests/interactive/mode_test.rs`**: インタラクティブモードの切り替えや状態遷移に関するテスト。
- **`tests/interactive/play_json_test.rs`**: インタラクティブモードでのJSON再生機能のテスト。
- **`tests/interactive/shared_mutex.rs`**: インタラクティブモードで利用される共有ミューテックスのテストユーティリティ。
- **`tests/interactive/step_by_step_test.rs`**: インタラクティブモードでの段階的なイベント処理に関するテスト。
- **`tests/interactive_tests.rs`**: インタラクティブモード機能全般の統合テスト。
- **`tests/ipc_pipe_test.rs`**: プロセス間通信（名前付きパイプ）の機能テスト。
- **`tests/logging_test.rs`**: ロギング機能が正しく動作するかどうかのテスト。
- **`tests/server_basic_test.rs`**: サーバーの基本的な起動、停止、コマンド処理に関するテスト。
- **`tests/server_integration_test.rs`**: サーバーの主要なコンポーネントの連携を検証する統合テスト。
- **`tests/tail_generation_test.rs`**: 音声データが終了した後も、残響音などを適切に処理する「テール生成」機能のテスト。
- **`tests/test_util_server_mutex.rs`**: サーバーテストで使用されるミューテックス関連のユーティリティ。

## 関数詳細説明
プロジェクト情報とファイル構造から推測される主要な関数とその役割を説明します。

-   **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**
    -   **役割**: YM2151再生サーバーが利用可能な状態であることを確認します。サーバーが起動していない場合は自動的にインストールとバックグラウンド起動を試み、コマンドを受け付けられるまで待機します。
    -   **引数**: `app_name` (アプリケーション名を示す文字列)。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   **`client::send_json(json_data: &str) -> anyhow::Result<()>`**
    -   **役割**: 提供されたJSON形式のYM2151レジスタイベントデータをサーバーに送信し、非インタラクティブモードでの再生を開始します。既存の演奏があれば停止し、新しい演奏に切り替わります。
    -   **引数**: `json_data` (YM2151イベントを含むJSON文字列)。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   **`client::stop_playback() -> anyhow::Result<()>`**
    -   **役割**: サーバーに現在再生中のYM2151イベントを停止するよう指示します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   **`client::shutdown_server() -> anyhow::Result<()>`**
    -   **役割**: バックグラウンドで動作しているYM2151再生サーバーをシャットダウンするよう指示します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   **`client::start_interactive() -> anyhow::Result<()>`**
    -   **役割**: サーバーをインタラクティブモードに切り替えます。これにより、音声ストリームを途切れさせずに複数のJSONデータを連続的にスケジューリングできるようになります。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**
    -   **役割**: インタラクティブモードでYM2151レジスタイベントのJSONデータをサーバーに送信し、現在の音声ストリームにスケジュールします。既存の演奏を中断せず、滑らかな切り替えや追加が可能です。
    -   **引数**: `json_data` (YM2151イベントを含むJSON文字列)。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   **`client::clear_schedule() -> anyhow::Result<()>`**
    -   **役割**: インタラクティブモードにおいて、サーバーにスケジュールされている未処理のYM2151イベントをすべてキャンセルするよう指示します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   **`client::get_server_time() -> anyhow::Result<f64>`**
    -   **役割**: サーバーが現在再生しているタイムライン上の時刻を秒単位で取得します。リアルタイムなタイミング同期に利用されます。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は現在のサーバー時刻 (f64)、エラーが発生した場合は`anyhow::Result`。
-   **`main()`** (Server Mode)
    -   **役割**: プログラムをサーバーとして起動し、名前付きパイプを通じてクライアントからのコマンドを待機し、YM2151イベントの再生をバックグラウンドで管理します。
    -   **引数**: コマンドライン引数 (`--verbose`, `--low-quality-resampling`など)。
    -   **戻り値**: 実行結果を示すステータスコード。
-   **`main()`** (Client Mode)
    -   **役割**: コマンドライン引数に従い、サーバーにJSONファイルの再生、演奏停止、またはシャットダウンなどの特定のコマンドを送信します。
    -   **引数**: コマンドライン引数 (`<json_file>`, `--stop`, `--shutdown`など)。
    -   **戻り値**: 実行結果を示すステータスコード。
-   **`src::opm::*`** (Nuked-OPMラッパー関数)
    -   **役割**: C言語で実装されたNuked-OPMライブラリのYM2151エミュレータに対して、Rustから安全に初期化、レジスタ書き込み、音声生成などの操作を行うための関数群。
    -   **引数**: YM2151レジスタのアドレス、データ、または内部状態を操作するための情報。
    -   **戻り値**: 操作結果や生成された音声データなど。
-   **`src::audio::player::Player::play(...)`**
    -   **役割**: 初期化されたオーディオプレイヤーを起動し、オーディオストリームを開始してYM2151イベントに基づいた音声生成と再生を行います。
    -   **引数**: オーディオ設定、イベントキュー、YM2151エミュレータの状態など。
    -   **戻り値**: 再生ストリームハンドルなど。
-   **`src::audio::scheduler::Scheduler::add_events(...)`**
    -   **役割**: YM2151レジスタイベントのリストを受け取り、それらを再生タイムライン上の適切な位置にスケジュールキューに追加します。
    -   **引数**: スケジュールするイベントのリスト、開始時刻など。
    -   **戻り値**: なし。
-   **`src::resampler::Resampler::resample(...)`**
    -   **役割**: 入力された音声データに対してサンプリングレート変換（リサンプリング）処理を適用し、指定された出力サンプリングレートに適合させます。
    -   **引数**: 入力音声バッファ、入力/出力サンプリングレートなど。
    -   **戻り値**: リサンプリングされた音声データ。
-   **`src::ipc::pipe_windows::NamedPipeServer::listen(...)`**
    -   **役割**: Windowsの名前付きパイプサーバーを起動し、クライアントからの新しい接続要求を待ち受けます。
    -   **引数**: サーバー名、コマンドハンドラーなど。
    -   **戻り値**: 新しい接続があった際のタスクなど。

## 関数呼び出し階層ツリー
```
main (Client Mode)
├── client::ensure_server_ready
│   ├── (サーバーが未起動の場合) cargo install ...
│   └── (サーバーが未起動の場合) サーバーをバックグラウンド起動
│       └── main (Server Mode)
│           └── src::server::connection::ConnectionHandler::listen_for_clients
│               └── src::ipc::pipe_windows::NamedPipeServer::listen
├── client::send_json (または client::play_json_interactive, client::stop_playback, client::shutdown_server)
│   └── src::client::core::send_command_to_server
│       └── src::ipc::pipe_windows::send_command
│           └── src::ipc::windows::pipe_writer::write_to_pipe
└── (必要に応じて) client::stop_playback / client::shutdown_server

main (Server Mode)
├── src::logging::init_logging
├── src::audio_config::load_config
├── src::server::state::ServerState::new
├── src::audio::player::Player::new
├── src::audio::player::Player::play
│   ├── src::audio::stream::start_audio_stream
│   │   └── src::audio::generator::AudioGenerator::render_audio_frames (ループ内で呼び出し)
│   │       ├── src::audio::scheduler::Scheduler::process_events
│   │       │   └── src::opm::write_register (FFI経由でNuked-OPMへ)
│   │       ├── src::opm::generate_samples (FFI経由でNuked-OPMへ)
│   │       └── src::resampler::Resampler::resample
│   └── src::mmcss::enable_mmcss (Windowsのみ)
└── src::server::connection::ConnectionHandler::listen_for_clients
    └── src::ipc::pipe_windows::NamedPipeServer::listen
        └── src::server::command_handler::handle_command (受信コマンドに応じた処理)
            ├── src::server::playback::start_playback
            │   └── src::audio::scheduler::Scheduler::add_events
            ├── src::server::playback::stop_playback
            ├── src::server::playback::clear_schedule
            └── src::server::state::shutdown

---
Generated at: 2026-03-04 07:04:35 JST
