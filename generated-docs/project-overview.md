Last updated: 2025-12-28

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するシステムです。
- サーバー・クライアント方式で動作し、JSON形式の音楽データを受け付けて演奏を制御します。
- 他のアプリケーションへの組み込みを容易にするライブラリとして、リアルタイム音楽制御とWAVファイル出力を提供します。

## 技術スタック
- フロントエンド: Rust CLI/API (クライアントアプリケーションとしてのコマンドラインインターフェースおよびプログラムからの呼び出し可能なAPI)
- 音楽・オーディオ:
    - Nuked-OPM: YM2151音源チップをエミュレートするためのC言語ライブラリ。
    - CPAL (Cross-platform Audio Library): Rust製のクロスプラットフォームオーディオライブラリで、リアルタイムオーディオストリームの管理に使用されていると推測されます。
    - WAVファイル出力: verboseモードでのオーディオデータ保存機能。
- 開発ツール:
    - Rust: プロジェクトの主要なプログラミング言語。
    - Cargo: Rustのビルドシステムとパッケージマネージャー。
    - rust-script: Rustスクリプトの実行を容易にするツール。
    - Visual Studio Code (`.vscode`): 開発環境設定ファイル。
- テスト:
    - Cargo Test: Rustの標準テストフレームワーク。
    - Nextest: 高速なRustテストランナー。
- ビルドツール:
    - Cargo: Rustプロジェクトのビルドを管理。
    - build.rs: Rustのカスタムビルドスクリプト。CFFIを扱うNuked-OPMのビルド設定に利用されていると推測されます。
- 言語機能:
    - FFI (Foreign Function Interface): RustからC言語で書かれたNuked-OPMを呼び出すために使用。
    - anyhow: 汎用エラーハンドリングライブラリ。
    - Serde: Rustのシリアライズ・デシリアライズフレームワーク（JSONデータ処理に利用）。
    - std::sync (Mutex, Arc): スレッド間の安全なデータ共有と同期。
    - tokio (非同期処理): サーバー・クライアント間の非同期通信や並行処理に利用されている可能性があります。
- 自動化・CI/CD:
    - setup_ci_environment.sh: CI環境設定用のシェルスクリプト。
- 開発標準:
    - .editorconfig: エディタの設定を統一するためのファイル。
    - .gitignore: Gitで管理しないファイルを指定。
    - CodeQL: 静的コード解析ツール（リポジトリ構造から推測）。

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
📁 generated-docs/
  📖 development-status-generated-prompt.md
🌐 googled947dc864c270e07.html
📄 install-ym2151-tools.rs
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
  📁 tests/ (ユニットテスト用)
    📄 audio_tests.rs
    📄 client_tests.rs
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
📁 tests/ (インテグレーションテスト用)
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
-   **README.ja.md**: プロジェクトの日本語による概要、機能、使用方法、開発状況、ライセンスなどを説明するドキュメントです。
-   **README.md**: プロジェクトの英語による概要、機能、使用方法、開発状況、ライセンスなどを説明するドキュメントです。
-   **.editorconfig**: さまざまなエディタやIDE間でコードスタイルを統一するための設定ファイルです。
-   **.gitignore**: Gitバージョン管理システムが追跡しないファイルやディレクトリを指定するファイルです。
-   **.config/nextest.toml**: RustのテストランナーであるNextestの設定ファイルです。
-   **.vscode/extensions.json**: VS Codeで推奨される拡張機能のリストを定義します。
-   **.vscode/settings.json**: VS Codeのワークスペース固有の設定を定義します。
-   **Cargo.toml**: Rustプロジェクトのマニフェストファイル。プロジェクトのメタデータ、依存関係、ビルド設定などが記述されています。
-   **Cargo.lock**: Cargo.tomlに基づいて、プロジェクトの依存関係の正確なバージョンを記録します。
-   **LICENSE**: プロジェクトのライセンス情報（MIT License）が記述されています。
-   **_config.yml**: GitHub Pagesなどのサイトジェネレータで使われる設定ファイルと推測されます。
-   **build.rs**: カスタムビルドロジックを含むRustスクリプト。Nuked-OPMのようなCライブラリをビルドする際に利用されます。
-   **generated-docs/development-status-generated-prompt.md**: 生成されたドキュメントのステータスに関するメモやプロンプト情報と推測されます。
-   **googled947dc864c270e07.html**: Googleサイト検証用のHTMLファイルと推測されます。プロジェクトの機能とは直接関係ありません。
-   **install-ym2151-tools.rs**: 関連ツールを一括インストールするためのRustスクリプトです。
-   **opm.c**: YM2151音源チップのエミュレーションを行うC言語のソースファイル（Nuked-OPMの一部）。
-   **opm.h**: `opm.c`に対応するC言語のヘッダーファイル。
-   **output_ym2151.json**: YM2151レジスタイベントログのサンプルデータを含むJSONファイル。
-   **setup_ci_environment.sh**: CI/CD環境をセットアップするためのシェルスクリプトです。
-   **src/main.rs**: アプリケーションのエントリーポイント。コマンドライン引数をパースし、サーバーまたはクライアントとしての動作を制御します。
-   **src/lib.rs**: このクレートのライブラリ部分のエントリーポイント。他のモジュールを公開し、クライアントAPIなどを定義します。
-   **src/audio_config.rs**: オーディオ再生に関する設定（サンプリングレート、バッファサイズなど）を定義します。
-   **src/debug_wav.rs**: デバッグ目的でWAVファイルを書き出す機能を提供します。
-   **src/demo_client_interactive.rs**: インタラクティブモードクライアントのデモンストレーションコードです。
-   **src/demo_server_interactive.rs**: インタラクティブモードサーバーのデモンストレーションコードです。
-   **src/demo_server_non_interactive.rs**: 非インタラクティブモードサーバーのデモンストレーションコードです。
-   **src/events.rs**: YM2151レジスタイベントのデータ構造と、それらをJSONからパースするロジックを定義します。
-   **src/logging.rs**: アプリケーション全体のログ出力に関する設定と機能を提供します。
-   **src/mmcss.rs**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用してオーディオスレッドの優先度を上げる機能に関連するコードです。
-   **src/opm.rs**: YM2151エミュレータのRustラッパー。`opm_ffi.rs`を介してNuked-OPMと連携します。
-   **src/opm_ffi.rs**: RustとC言語のNuked-OPMライブラリ間のFFI（Foreign Function Interface）バインディングを定義します。
-   **src/player.rs**: YM2151レジスタイベントを処理し、オーディオ出力を生成する主要なプレイヤーロジックをカプセル化します。
-   **src/resampler.rs**: オーディオサンプルのリサンプリング（サンプリングレート変換）機能を提供します。
-   **src/scheduler.rs**: YM2151レジスタイベントを指定された時刻にスケジューリングし、リアルタイム再生を管理します。
-   **src/wav_writer.rs**: オーディオデータをWAVファイル形式で書き込む機能を提供します。
-   **src/audio/buffers.rs**: オーディオバッファの管理に関する構造体や関数を定義します。
-   **src/audio/commands.rs**: オーディオ関連の内部コマンド（再生開始、停止など）の定義です。
-   **src/audio/generator.rs**: YM2151のレジスタ操作に基づいてオーディオサンプルを生成する機能を提供します。
-   **src/audio/mod.rs**: `src/audio`モジュールのルートファイルで、サブモジュールを公開します。
-   **src/audio/player.rs**: オーディオ再生のメインループと、生成されたオーディオデータのストリーム処理を管理します。
-   **src/audio/scheduler.rs**: オーディオイベントのスケジューリングロジックを具体的に実装します。
-   **src/audio/stream.rs**: オーディオデバイスへの出力ストリームを管理します（CPALなどのライブラリを使用）。
-   **src/client/config.rs**: クライアント側の設定を定義します。
-   **src/client/core.rs**: クライアントの中核となるロジックや共通機能を提供します。
-   **src/client/interactive.rs**: インタラクティブモードのクライアント機能に関連するコードです。
-   **src/client/json.rs**: クライアントがサーバーに送信するJSONデータの構造と処理を定義します。
-   **src/client/mod.rs**: `src/client`モジュールのルートファイルで、クライアントの公開APIを定義します。
-   **src/client/server.rs**: クライアントがサーバーの起動や停止、状態確認を行うためのヘルパー関数群です。
-   **src/ipc/mod.rs**: IPC（プロセス間通信）モジュールのルートファイルです。
-   **src/ipc/pipe_windows.rs**: Windowsの名前付きパイプを利用したIPC通信の実装を提供します。
-   **src/ipc/protocol.rs**: クライアントとサーバー間の通信プロトコル（コマンドやデータ形式）を定義します。
-   **src/ipc/windows/mod.rs**: Windows固有のIPC実装に関連するサブモジュールを公開します。
-   **src/ipc/windows/pipe_factory.rs**: 名前付きパイプの生成を担当するファクトリパターンを実装します。
-   **src/ipc/windows/pipe_handle.rs**: 名前付きパイプのハンドル管理を行います。
-   **src/ipc/windows/pipe_reader.rs**: 名前付きパイプからのデータ読み込みを担当します。
-   **src/ipc/windows/pipe_writer.rs**: 名前付きパイプへのデータ書き込みを担当します。
-   **src/ipc/windows/test_logging.rs**: Windowsパイプのテスト用ロギング機能です。
-   **src/server/command_handler.rs**: クライアントから受信したコマンドを解釈し、サーバーの適切なアクションにディスパッチします。
-   **src/server/connection.rs**: クライアントとの個別のIPC接続を管理します。
-   **src/server/mod.rs**: `src/server`モジュールのルートファイルで、サーバーの主要機能を構成します。
-   **src/server/playback.rs**: サーバー側での実際のYM2151サウンド生成とオーディオ再生を管理します。
-   **src/server/state.rs**: サーバーの現在の状態（再生中か、インタラクティブモードかなど）を保持します。
-   **src/tests/** (ユニットテスト用ディレクトリ): `src/`内のモジュールに対するユニットテストファイル群。
-   **tests/** (インテグレーションテスト用ディレクトリ): `src/`の外部にあるインテグレーションテストファイル群。
    -   **tests/audio/**: オーディオ関連のインテグレーションテスト。
    -   **tests/fixtures/**: テストで使用するサンプルデータ（例: JSONファイル）。
    -   **tests/interactive/**: インタラクティブモードに関するインテグレーションテスト。

## 関数詳細説明
-   **client::ensure_server_ready(app_name: &str)**
    -   **役割**: YM2151再生サーバーが利用可能であることを確認し、必要に応じて自動的にインストールおよびバックグラウンドで起動します。
    -   **引数**: `app_name` (文字列スライス) - サーバーを識別するためのアプリケーション名。
    -   **戻り値**: `anyhow::Result<()>` - 成功またはエラー。
    -   **機能**: サーバーの起動状態チェック、PATHからのアプリケーション検索、cargoによるインストール、バックグラウンド起動、コマンド受付待機を自動で行います。

-   **client::send_json(json_data: &str)**
    -   **役割**: 非インタラクティブモードでYM2151レジスタイベントを含むJSONデータをサーバーに送信し、再生を開始します。
    -   **引数**: `json_data` (文字列スライス) - YM2151イベントログを含むJSON文字列。
    -   **戻り値**: `anyhow::Result<()>` - 成功またはエラー。
    -   **機能**: サーバーにJSONデータを渡し、既存の演奏を停止して新しいJSONデータの演奏を開始させます。

-   **client::stop_playback()**
    -   **役割**: サーバーに対して現在のYM2151演奏を停止するよう指示します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` - 成功またはエラー。
    -   **機能**: サーバーの再生を中断し、無音状態にします。

-   **client::shutdown_server()**
    -   **役割**: サーバープロセスを安全にシャットダウンするよう指示します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` - 成功またはエラー。
    -   **機能**: サーバープロセスを終了させます。

-   **client::start_interactive()**
    -   **役割**: サーバーをインタラクティブモードに切り替え、連続したオーディオストリームでのリアルタイムイベントスケジューリングを開始します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` - 成功またはエラー。
    -   **機能**: 音声が途切れない連続再生を可能にし、動的なイベント追加に備えます。

-   **client::play_json_interactive(json_data: &str)**
    -   **役割**: インタラクティブモードで、YM2151レジスタイベントを含むJSONデータをサーバーのスケジュールに動的に追加します。
    -   **引数**: `json_data` (文字列スライス) - YM2151イベントログを含むJSON文字列。
    -   **戻り値**: `anyhow::Result<()>` - 成功またはエラー。
    -   **機能**: 音声ギャップなしで、複数のJSONフレーズを連続して再生するために使用されます。サンプル単位の`time`はf64秒単位に自動変換されます。

-   **client::clear_schedule()**
    -   **役割**: インタラクティブモードにおいて、まだ処理されていない将来のYM2151イベントをすべてキャンセルします。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` - 成功またはエラー。
    -   **機能**: 急な楽曲やフレーズの切り替え時に、古いイベントが意図せず再生されるのを防ぎます。

-   **client::get_server_time()**
    -   **役割**: サーバーの現在の再生時刻（オーディオストリームの現在位置）を秒単位で取得します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<f64>` - 現在のサーバー時刻（秒）またはエラー。
    -   **機能**: Web Audio APIの`currentTime`プロパティに相当し、正確なタイミング制御に利用されます。

-   **main() (in src/main.rs)**
    -   **役割**: アプリケーションのエントリーポイント。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのどちらで動作するかを決定し、それぞれのロジックを実行します。
    -   **引数**: なし (OSによって提供されるコマンドライン引数)。
    -   **戻り値**: `anyhow::Result<()>` - 実行の成否。
    -   **機能**: `ym2151-log-play-server`プログラムが最初に実行される際に、全体のフローを制御します。

-   **Player::new(...) (in src/player.rs)**
    -   **役割**: YM2151のオーディオプレイヤーインスタンスを生成し、初期化します。
    -   **引数**: YM2151エミュレータ設定、オーディオ出力設定など。
    -   **戻り値**: `Player`構造体のインスタンス。
    -   **機能**: YM2151チップのレジスタ操作に基づき音を生成する準備を整えます。

-   **Scheduler::add_event(...) (in src/scheduler.rs)**
    -   **役割**: YM2151イベントを指定した時刻に再生するようにスケジューラに登録します。
    -   **引数**: イベントオブジェクト、再生時刻など。
    -   **戻り値**: なし。
    -   **機能**: 複数のイベントを時系列に沿って管理し、正確なタイミングでの音源エミュレータへの書き込みを準備します。

-   **Generator::generate_sample(...) (in src/audio/generator.rs)**
    -   **役割**: YM2151エミュレータの状態に基づいて、単一のオーディオサンプルを生成します。
    -   **引数**: なし (内部状態を使用)。
    -   **戻り値**: `f32` (オーディオサンプル値)。
    -   **機能**: 周期的に呼び出され、YM2151の音源状態をシミュレートしてオーディオ波形を生成します。

## 関数呼び出し階層ツリー
関数呼び出し階層ツリーを自動的に分析できませんでした。
ただし、提供されたプロジェクト情報から以下の主要な呼び出し関係が示唆されます：

```
main (src/main.rs)
├── client::ensure_server_ready
│   └── (内部的にcargoコマンド、サーバーのバックグラウンド起動)
├── client::send_json
│   └── ipc::pipe_windows::send_command (推定)
├── client::stop_playback
│   └── ipc::pipe_windows::send_command (推定)
├── client::shutdown_server
│   └── ipc::pipe_windows::send_command (推定)
├── client::start_interactive
│   └── ipc::pipe_windows::send_command (推定)
├── client::play_json_interactive
│   ├── (JSONデータ変換ロジック)
│   └── ipc::pipe_windows::send_command (推定)
├── client::clear_schedule
│   └── ipc::pipe_windows::send_command (推定)
└── client::get_server_time
    └── ipc::pipe_windows::send_command (推定)
```

**サーバー側 (推定される主要な流れ):**
```
main (src/main.rs, serverモード)
├── server::connection::handle_client_connection (IPC接続確立と待機)
│   └── server::command_handler::handle_command (受信コマンドの処理)
│       ├── server::playback::start_playback (再生開始)
│       │   ├── audio::player::Player::new
│       │   └── audio::stream::start_audio_stream
│       │       └── audio::player::Player::generate_samples (オーディオサンプル生成コールバック)
│       │           ├── audio::scheduler::Scheduler::process_events
│       │           │   └── opm::write_register (FFI経由でNuked-OPMへ)
│       │           └── audio::generator::Generator::generate_sample
│       ├── server::playback::stop_playback (再生停止)
│       ├── server::playback::clear_schedule (スケジュールクリア)
│       └── server::state::update_state (サーバー状態更新)
└── (オーディオスレッドは別途mmcss::enable_mmcssなどにより優先度管理)

---
Generated at: 2025-12-28 07:02:15 JST
