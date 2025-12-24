Last updated: 2025-12-25

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタイベントログをリアルタイムで再生するWindows専用のサーバー・クライアントシステムです。
- サーバーはバックグラウンドで常駐し、クライアントからの命令で演奏の開始、停止、切り替えを動的に行います。
- JSON形式の音楽データを扱い、WAVファイル出力やインタラクティブな音響制御機能も提供します。

## 技術スタック
- フロントエンド: このプロジェクトはCLIツールおよびライブラリとして提供されており、特定のグラフィカルなフロントエンド技術は使用していません。
- 音楽・オーディオ:
    - **YM2151 (OPM)**: ヤマハのFM音源チップのシミュレーション。
    - **Nuked-OPM**: YM2151のハードウェアエミュレーションを提供するC言語ライブラリ。高い再現性で音源をシミュレートします。
    - **Rodio**: Rust製のクロスプラットフォームオーディオ再生ライブラリ。リアルタイムオーディオ出力に使用されます。
    - **Hound**: Rust製のWAVファイル読み書きライブラリ。演奏結果のWAVファイル出力に利用されます。
    - **Rubato**: Rust製の高品質なオーディオリサンプリングライブラリ。異なるサンプリングレート間の変換を行います。
    - **MMCSS (Multimedia Class Scheduler Service)**: Windows専用のAPIで、リアルタイムオーディオ処理に高いCPU優先度を割り当てるために利用されます。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語。パフォーマンスと安全性を提供します。
    - **Cargo**: Rustのビルドシステムおよびパッケージマネージャー。依存関係の管理、ビルド、テスト、ドキュメント生成を行います。
    - **Anyhow**: Rustのエラーハンドリングライブラリ。柔軟なエラー伝播とレポートを可能にします。
    - **Serde**: Rustの強力なシリアライズ/デシリアライズフレームワーク。JSONデータとの間でRust構造体を簡単に変換するために使用されます。
    - **Clap**: Rust製のコマンドライン引数パーサー。サーバーとクライアントのCLIインターフェースを定義するために使用されます。
    - **Log**: Rustのロギングファサード。さまざまなロギング実装と連携するための統一されたインターフェースを提供します。
    - **Env_logger**: Logファサードの具体的な実装。環境変数に基づいてロギングを設定します。
    - **Once_cell**: Rustの「一度だけ初期化される」メカニズムを提供するクレート。遅延初期化やグローバルシングルトンの実現に便利です。
    - **Atomic-wait**: Rustの同期プリミティブ。スレッド間の待機と通知に使用されます。
    - **Interprocess**: Rust製のプロセス間通信（IPC）ライブラリ。名前付きパイプによるサーバー・クライアント通信を実現します。
    - **Symlink**: Windows上でシンボリックリンクを扱うためのRustクレート。
- テスト:
    - **Cargo test**: Rustに組み込まれたテストフレームワーク。単体テスト、結合テスト、ドキュメンテーションテストを実行します。
    - **Nextest**: Rustプロジェクト向けの高速なテストランナー。大規模なテストスイートの実行を最適化します。
- ビルドツール:
    - **Cargo**: Rustの標準ビルドツール。
- 言語機能:
    - **FFI (Foreign Function Interface)**: RustからC言語で書かれたNuked-OPMライブラリを呼び出すために使用されます。
- 自動化・CI/CD:
    - **Cargo**: `install-ym2151-tools.rs`などのスクリプトで関連ツールのインストールを自動化できます。
- 開発標準:
    - **.editorconfig**: 複数のエディタやIDE間で一貫したコーディングスタイルを維持するためのファイル。
    - **.gitignore**: Gitリポジトリで追跡しないファイルを指定するためのファイル。
    - **.vscode/extensions.json, .vscode/settings.json**: Visual Studio Codeの推奨拡張機能やワークスペース設定を定義するファイル。

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
├── install-ym2151-tools.rs
├── opm.c
├── opm.h
├── output_ym2151.json
└── src/
    ├── audio/
    │   ├── buffers.rs
    │   ├── commands.rs
    │   ├── generator.rs
    │   ├── mod.rs
    │   ├── player.rs
    │   ├── scheduler.rs
    │   └── stream.rs
    ├── audio_config.rs
    ├── client/
    │   ├── config.rs
    │   ├── core.rs
    │   ├── interactive.rs
    │   ├── json.rs
    │   ├── mod.rs
    │   └── server.rs
    ├── debug_wav.rs
    ├── demo_client_interactive.rs
    ├── demo_server_interactive.rs
    ├── demo_server_non_interactive.rs
    ├── events.rs
    ├── ipc/
    │   ├── mod.rs
    │   ├── pipe_windows.rs
    │   ├── protocol.rs
    │   └── windows/
    │       ├── mod.rs
    │       ├── pipe_factory.rs
    │       ├── pipe_handle.rs
    │       ├── pipe_reader.rs
    │       ├── pipe_writer.rs
    │       └── test_logging.rs
    ├── lib.rs
    ├── logging.rs
    ├── main.rs
    ├── mmcss.rs
    ├── opm.rs
    ├── opm_ffi.rs
    ├── player.rs
    ├── resampler.rs
    ├── scheduler.rs
    ├── server/
    │   ├── command_handler.rs
    │   ├── connection.rs
    │   ├── mod.rs
    │   ├── playback.rs
    │   └── state.rs
    ├── tests/ # 単体テストなど
    │   ├── audio_tests.rs
    │   ├── client_tests.rs
    │   ├── debug_wav_tests.rs
    │   ├── demo_server_interactive_tests.rs
    │   ├── demo_server_non_interactive_tests.rs
    │   ├── events_tests.rs
    │   ├── ipc_pipe_windows_tests.rs
    │   ├── ipc_protocol_tests.rs
    │   ├── logging_tests.rs
    │   ├── mmcss_tests.rs
    │   ├── mod.rs
    │   ├── opm_ffi_tests.rs
    │   ├── opm_tests.rs
    │   ├── play_json_interactive_tests.rs
    │   ├── player_tests.rs
    │   ├── resampler_tests.rs
    │   ├── scheduler_tests.rs
    │   ├── server_tests.rs
    │   └── wav_writer_tests.rs
    └── wav_writer.rs
└── tests/ # 統合テストなど
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
-   **`.config/nextest.toml`**: `nextest`テストランナーの設定ファイル。テストの実行方法やレポート形式などを指定します。
-   **`.editorconfig`**: コードエディタ間でインデントスタイル、文字コードなどの基本的な書式設定を統一するための設定ファイル。
-   **`.gitignore`**: Gitが追跡しないファイルやディレクトリのパターンを定義するファイル。ビルド成果物や一時ファイルなどが含まれます。
-   **`.vscode/extensions.json`**: Visual Studio Codeでこのプロジェクトを開発する際に推奨される拡張機能のリスト。
-   **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定。リンターやフォーマッターの挙動などが設定されます。
-   **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンとその依存関係ツリーを記録するファイル。再現性のあるビルドを保証します。
-   **`Cargo.toml`**: Rustプロジェクトのメイン設定ファイル。プロジェクト名、バージョン、依存クレート、ビルド設定などを記述します。
-   **`LICENSE`**: プロジェクトのライセンス情報（MIT License）が記載されたファイル。
-   **`README.ja.md`**: プロジェクトの日本語による概要、使い方、開発状況などを説明するマークダウンファイル。
-   **`README.md`**: プロジェクトの英語による概要、使い方、開発状況などを説明するマークダウンファイル。
-   **`_config.yml`**: 通常、Jekyllなどの静的サイトジェネレーターの設定ファイル。このプロジェクトのドキュメント生成やウェブサイトに関連する可能性があります。
-   **`build.rs`**: Rustのビルドスクリプト。ビルドプロセス中に特定のタスク（例えば、C言語ライブラリのビルド）を実行するために使用されます。
-   **`install-ym2151-tools.rs`**: Rustスクリプトとして実行可能な、関連開発ツールの一括インストールを自動化するためのスクリプト。
-   **`opm.c`**: Nuked-OPMエミュレータのC言語ソースコード。YM2151音源チップの挙動をシミュレートする核心部分です。
-   **`opm.h`**: `opm.c`に対応するC言語ヘッダーファイル。RustからNuked-OPMの関数を呼び出すためのFFI定義に利用されます。
-   **`output_ym2151.json`**: YM2151のレジスタイベントログのJSON形式サンプルデータファイル。クライアントからの再生テストなどに使用されます。
-   **`src/audio/buffers.rs`**: オーディオデータのバッファリングに関する処理を定義するモジュール。
-   **`src/audio/commands.rs`**: オーディオ再生に関連するコマンドやイベントの定義。
-   **`src/audio/generator.rs`**: YM2151音源からオーディオサンプルを生成するロジックを実装するモジュール。Nuked-OPMと連携します。
-   **`src/audio/mod.rs`**: `src/audio`ディレクトリのルートモジュール。関連するオーディオ処理モジュールを集約します。
-   **`src/audio/player.rs`**: 実際のオーディオ再生デバイスへの出力を管理するモジュール。Rodioライブラリを使用します。
-   **`src/audio/scheduler.rs`**: YM2151レジスタイベントを時間軸に沿ってスケジュールし、適切なタイミングで`generator`に渡すモジュール。
-   **`src/audio/stream.rs`**: オーディオデータストリームの管理と処理に関するモジュール。
-   **`src/audio_config.rs`**: オーディオ出力のサンプリングレート、バッファサイズなどの設定を定義するモジュール。
-   **`src/client/config.rs`**: クライアント側の設定、例えばサーバーへの接続情報などを管理するモジュール。
-   **`src/client/core.rs`**: クライアントの基本的なロジック、サーバーとの接続確立や共通コマンドの送信などを担当するモジュール。
-   **`src/client/interactive.rs`**: インタラクティブモードクライアントの機能（リアルタイムイベント送信、スケジュールクリア、サーバー時刻取得など）を実装するモジュール。
-   **`src/client/json.rs`**: クライアントがサーバーに送るJSONデータの構造定義とシリアライズ・デシリアライズを扱うモジュール。
-   **`src/client/mod.rs`**: `src/client`ディレクトリのルートモジュール。クライアント関連モジュールを集約し、公開APIを提供します。
-   **`src/client/server.rs`**: クライアントがサーバーの起動や停止を制御するためのヘルパー関数を提供するモジュール。
-   **`src/debug_wav.rs`**: デバッグ目的でWAVファイルを生成する機能を持つモジュール。
-   **`src/demo_client_interactive.rs`**: インタラクティブモードクライアントのデモンストレーションコード。
-   **`src/demo_server_interactive.rs`**: インタラクティブモードサーバーのデモンストレーションコード。
-   **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードサーバーのデモンストレーションコード。
-   **`src/events.rs`**: YM2151のレジスタイベント（時間、アドレス、データ）のデータ構造を定義するモジュール。
-   **`src/ipc/mod.rs`**: `src/ipc`ディレクトリのルートモジュール。プロセス間通信関連モジュールを集約します。
-   **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを使用したプロセス間通信の実装モジュール。
-   **`src/ipc/protocol.rs`**: サーバーとクライアント間でやり取りされる通信プロトコルのメッセージ構造を定義するモジュール。
-   **`src/ipc/windows/mod.rs`**: Windows固有のIPC実装のルートモジュール。
-   **`src/ipc/windows/pipe_factory.rs`**: 名前付きパイプの生成を担当するモジュール。
-   **`src/ipc/windows/pipe_handle.rs`**: 名前付きパイプのハンドル管理に関するモジュール。
-   **`src/ipc/windows/pipe_reader.rs`**: 名前付きパイプからのデータ読み込みを担当するモジュール。
-   **`src/ipc/windows/pipe_writer.rs`**: 名前付きパイプへのデータ書き込みを担当するモジュール。
-   **`src/ipc/windows/test_logging.rs`**: Windows IPCテスト時のロギングヘルパー。
-   **`src/lib.rs`**: プロジェクトのライブラリクレートのルートファイル。他のモジュールを公開します。
-   **`src/logging.rs`**: プロジェクト全体のロギング設定とヘルパー関数を提供するモジュール。
-   **`src/main.rs`**: 実行可能クレートのエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントのモードを開始します。
-   **`src/mmcss.rs`**: WindowsのMMCSS (Multimedia Class Scheduler Service) を利用して、オーディオ処理スレッドの優先度を上げるためのモジュール。
-   **`src/opm.rs`**: Nuked-OPMエミュレータをRustから利用するためのラッパーおよび高レベルインターフェースを提供するモジュール。
-   **`src/opm_ffi.rs`**: `opm.c`および`opm.h`のC関数をRustから呼び出すためのFFI定義を直接扱うモジュール。
-   **`src/player.rs`**: (重複する可能性のある名前だが、`src/audio/player.rs`とは異なる高レベルなプレイヤ機能、または以前の実装が残っている可能性) オーディオ再生の全体的な制御フローを管理するモジュール。
-   **`src/resampler.rs`**: オーディオデータのサンプリングレート変換ロジックを実装するモジュール（Rubatoを使用）。
-   **`src/scheduler.rs`**: (重複する可能性のある名前だが、`src/audio/scheduler.rs`とは異なる高レベルなスケジューラ機能、または以前の実装が残っている可能性) YM2151イベントの全体的なスケジュール管理を行うモジュール。
-   **`src/server/command_handler.rs`**: クライアントから受け取ったコマンドを解析し、サーバーの内部状態や再生に適用するロジックを実装するモジュール。
-   **`src/server/connection.rs`**: サーバーとクライアント間の個別のIPC接続を管理するモジュール。
-   **`src/server/mod.rs`**: `src/server`ディレクトリのルートモジュール。サーバー関連モジュールを集約します。
-   **`src/server/playback.rs`**: サーバーでのYM2151演奏ロジック、オーディオストリームの管理、スケジューラーとの連携などを担当するモジュール。
-   **`src/server/state.rs`**: サーバーの現在の状態（再生中かどうか、現在のスケジュールなど）を保持するモジュール。
-   **`src/tests/` (内部テスト)**: `src`内の各モジュールに対する単体テストや結合テストを格納するディレクトリ。
-   **`src/wav_writer.rs`**: 生成されたオーディオデータをWAVファイルとして保存するための機能を提供するモジュール（Houndを使用）。
-   **`tests/` (統合テスト)**: プロジェクト全体の統合テストやシステムテストを格納するディレクトリ。`fixtures/`にはテスト用のJSONデータなどが含まれます。

## 関数詳細説明
プロジェクト情報と提供されたコードスニペットに基づき、主要な公開関数を説明します。

-   **`main()`** (src/main.rs):
    -   役割: プログラムのエントリポイント。コマンドライン引数を解析し、アプリケーションをサーバーモードまたはクライアントモードとして起動します。
    -   引数: なし（`std::env::args()`経由でコマンドライン引数を取得）。
    -   戻り値: `anyhow::Result<()>` (処理結果を示す)。
    -   機能: `clap`クレートを使用して引数を解析し、それに応じて`server`モジュールまたは`client`モジュール内の適切な関数を呼び出します。
-   **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`** (src/client/core.rs):
    -   役割: YM2151再生サーバーが利用可能であることを保証します。サーバーが未起動の場合は自動的にインストールし、バックグラウンドで起動します。
    -   引数: `app_name` - クライアントアプリケーションの名前。サーバーの識別に使用されます。
    -   戻り値: `anyhow::Result<()>` (サーバーの準備が成功したかを示す)。
    -   機能: サーバーの起動状態を確認し、必要であれば`cargo install`でサーバーアプリケーションをインストールし、バックグラウンドプロセスとして起動します。サーバーがコマンドを受け付けられる状態になるまで待機します。
-   **`client::send_json(json_data: &str) -> anyhow::Result<()>`** (src/client/json.rs):
    -   役割: 非インタラクティブモードでJSON形式のYM2151レジスタイベントログをサーバーに送信し、再生を開始させます。
    -   引数: `json_data` - 再生するYM2151イベントを含むJSON文字列。
    -   戻り値: `anyhow::Result<()>` (JSONデータの送信が成功したかを示す)。
    -   機能: サーバーとのIPC接続を確立し、JSONデータをプロトコルメッセージに変換してサーバーに送信します。送信後、サーバーは現在の再生を停止し、新しいデータで再生を開始します。
-   **`client::stop_playback() -> anyhow::Result<()>`** (src/client/core.rs):
    -   役割: サーバーに対して現在のYM2151再生を停止するよう指示します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` (停止指示の送信が成功したかを示す)。
    -   機能: サーバーに停止コマンドを送信し、音源を無音状態にします。
-   **`client::shutdown_server() -> anyhow::Result<()>`** (src/client/core.rs):
    -   役割: サーバープロセスをシャットダウンするよう指示します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` (シャットダウン指示の送信が成功したかを示す)。
    -   機能: サーバーにシャットダウンコマンドを送信し、サーバープロセスを安全に終了させます。
-   **`client::start_interactive() -> anyhow::Result<()>`** (src/client/interactive.rs):
    -   役割: サーバーをインタラクティブモードに移行させ、連続したオーディオストリームの再生を開始します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` (インタラクティブモード開始が成功したかを示す)。
    -   機能: サーバーにインタラクティブモード開始コマンドを送信し、リアルタイムなイベントスケジューリングの準備をします。
-   **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`** (src/client/interactive.rs):
    -   役割: インタラクティブモードでYM2151レジスタイベントログをサーバーに送信し、連続するオーディオストリーム内で再生をスケジュールします。
    -   引数: `json_data` - 再生をスケジュールするYM2151イベントを含むJSON文字列。イベントの`time`はサンプル単位（55930 Hz）で、サーバー側で自動的に秒単位に変換されます。
    -   戻り値: `anyhow::Result<()>` (JSONデータのスケジュール送信が成功したかを示す)。
    -   機能: JSONデータをパースし、サンプル単位の`time`を秒単位に変換した後、サーバーに送信してリアルタイムで再生キューに追加します。
-   **`client::clear_schedule() -> anyhow::Result<()>`** (src/client/interactive.rs):
    -   役割: インタラクティブモードで、サーバーの現在の再生スケジュールから、まだ処理されていないすべての未来のイベントをクリアします。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` (スケジュールクリア指示の送信が成功したかを示す)。
    -   機能: 進行中の演奏は継続しつつ、キューにある今後のイベントを削除することで、即座に新しいフレーズに切り替えることを可能にします。
-   **`client::get_server_time() -> anyhow::Result<f64>`** (src/client/interactive.rs):
    -   役割: サーバーの現在の再生時刻（Web Audioの`currentTime`に相当）を秒単位で取得します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<f64>` (現在のサーバー時刻、またはエラー)。
    -   機能: サーバーに時刻要求コマンドを送信し、サーバーからの応答を待って現在の再生時刻（f64秒）を返します。正確なタイミング制御に使用されます。

## 関数呼び出し階層ツリー
提供されたプロジェクト情報とコードスニペットから、主要な関数呼び出し階層を以下のように推測できます。

```
main()
├── server::run()  (サーバーモード選択時)
│   ├── server::connection::accept_connections()
│   │   └── server::command_handler::handle_command()
│   │       ├── server::playback::start_playback()
│   │       ├── server::playback::stop_playback()
│   │       ├── server::playback::start_interactive_mode()
│   │       ├── server::playback::play_interactive_json()
│   │       └── server::playback::clear_interactive_schedule()
│   └── audio::player::start_audio_stream()
│       └── audio::stream::process_audio_frame()
│           └── audio::generator::generate_samples()
│               └── opm::synthesize()
└── client::run()  (クライアントモード選択時)
    ├── client::ensure_server_ready()
    │   ├── cargo::install_ym2151_tools()  (必要に応じて)
    │   └── process::spawn_server_in_background()
    └── client::handle_client_command()
        ├── client::send_json()
        │   └── ipc::protocol::send_json_command()
        ├── client::stop_playback()
        │   └── ipc::protocol::send_stop_command()
        ├── client::shutdown_server()
        │   └── ipc::protocol::send_shutdown_command()
        ├── client::start_interactive()
        │   └── ipc::protocol::send_start_interactive_command()
        ├── client::play_json_interactive()
        │   ├── events::convert_to_seconds()
        │   └── ipc::protocol::send_play_interactive_json_command()
        ├── client::clear_schedule()
        │   └── ipc::protocol::send_clear_schedule_command()
        └── client::get_server_time()
            └── ipc::protocol::send_get_time_command()

---
Generated at: 2025-12-25 07:02:30 JST
