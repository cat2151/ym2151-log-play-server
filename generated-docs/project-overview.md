Last updated: 2025-12-02

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するシステムです。
- 常駐型サーバーとクライアントで構成され、プログラムから柔軟に再生を制御できます。
- Windowsプラットフォーム専用に設計されており、リアルタイムな音楽制御や音色エディタでの利用を想定しています。

## 技術スタック
- フロントエンド: CLIクライアント (Rust)、ライブラリAPI (Rust)
- 音楽・オーディオ: YM2151 (OPM) エミュレータ (Nuked-OPM, C言語)、リアルタイムオーディオ再生 (Rust crate: cpalなど、具体的なクレート名が情報にないため汎用的に記述)、オーディオリサンプリング、WAVファイル出力
- 開発ツール: Rust (プログラミング言語)、Cargo (Rustのビルドシステムとパッケージマネージャー)、rust-script (インストールスクリプト実行用)
- テスト: Rustの組み込みテストフレームワーク (cargo test)
- ビルドツール: Cargo (Rustプロジェクトのビルド)、`build.rs` (CFFIバインディングのビルド)
- 言語機能: Rust (FFIによるC言語連携、非同期・並行処理、マクロ)、JSON (データ交換フォーマット)
- 自動化・CI/CD: GitHub Actions (推測, `setup_ci_environment.sh`の存在から)
- 開発標準: .editorconfig (コードスタイル定義)

## ファイル階層ツリー
```
.
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
│   ├── 119.md
│   ├── 120.md
│   ├── 121.md
│   ├── 122.md
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
- **`.editorconfig`**: さまざまなエディタやIDE間で一貫したコーディングスタイルを定義するファイル。
- **`.gitignore`**: Gitが追跡すべきでないファイルやディレクトリを指定するファイル。
- **`.vscode/extensions.json`**: VS Code推奨拡張機能を定義するファイル。
- **`.vscode/settings.json`**: VS Codeのワークスペース固有の設定を定義するファイル。
- **`Cargo.lock`**: Cargoがビルドに使用する依存関係の正確なバージョンを記録するファイル。
- **`Cargo.toml`**: Rustプロジェクトのパッケージ情報、依存関係、ビルド設定を定義するファイル。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載したファイル。
- **`README.ja.md`**: プロジェクトの日本語版説明書。
- **`README.md`**: プロジェクトの英語版説明書。
- **`_config.yml`**: Jekyllなどの静的サイトジェネレータで使われる設定ファイル（本プロジェクトでの具体的な用途は不明）。
- **`build.rs`**: Rustプロジェクトのビルドプロセス中に実行されるスクリプト。主にCライブラリ（Nuked-OPM）のコンパイルとFFIバインディング生成に利用される。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発状況に関するドキュメントの一部。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のHTMLファイル。
- **`install-ym2151-tools.rs`**: `rust-script`を使って関連ツールを一括インストールするためのスクリプト。
- **`issue-notes/*.md`**: 開発中の特定の問題や検討事項に関するメモファイル。
- **`opm.c`**: YM2151 (OPM) 音源チップのエミュレーションロジックを実装したC言語ソースファイル（Nuked-OPMの一部）。
- **`opm.h`**: `opm.c`に対応するC言語ヘッダファイル。
- **`output_ym2151.json`**: YM2151のレジスタイベントログデータを含むJSONファイルの例。
- **`setup_ci_environment.sh`**: CI/CD環境をセットアップするためのシェルスクリプト。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングに関する構造体やロジックを定義するモジュール。
- **`src/audio/commands.rs`**: オーディオエンジンに送られるコマンド（例: 再生開始、停止、スケジュールクリアなど）を定義するモジュール。
- **`src/audio/generator.rs`**: YM2151エミュレータから実際のオーディオサンプルを生成するロジックを含むモジュール。
- **`src/audio/mod.rs`**: `src/audio`モジュールのルートファイルで、サブモジュールを公開する。
- **`src/audio/player.rs`**: オーディオ再生の中核を担うロジックを定義するモジュール。
- **`src/audio/scheduler.rs`**: YM2151レジスタイベントの実行タイミングを管理するスケジューリングロジックを含むモジュール。
- **`src/audio/stream.rs`**: リアルタイムオーディオストリームの管理と低レベルI/Oに関するモジュール。
- **`src/audio_config.rs`**: オーディオ設定（サンプリングレート、バッファサイズなど）に関する構造体とロジックを定義するモジュール。
- **`src/client/config.rs`**: クライアントの設定（例: サーバー接続情報）を定義するモジュール。
- **`src/client/core.rs`**: クライアントの基本的な操作（サーバーの起動確認、JSON送信、停止、シャットダウンなど）に関する共通ロジックを定義するモジュール。
- **`src/client/interactive.rs`**: インタラクティブモードに特化したクライアントロジックを定義するモジュール。連続的な音声ストリームを維持し、リアルタイム制御を可能にする。
- **`src/client/json.rs`**: クライアントがサーバーとやり取りするJSONデータの形式やパース処理を定義するモジュール。
- **`src/client/mod.rs`**: `src/client`モジュールのルートファイルで、クライアントのサブモジュールと公開APIを定義する。
- **`src/client/server.rs`**: クライアントがサーバーのライフサイクル（起動、シャットダウン）を管理するためのヘルパー関数やロジックを定義するモジュール。
- **`src/debug_wav.rs`**: デバッグ目的でWAVファイルを生成・保存するための機能を含むモジュール。
- **`src/demo_client_interactive.rs`**: インタラクティブモードのクライアント機能を示すデモコード。
- **`src/demo_server_interactive.rs`**: インタラクティブモードのサーバー機能を示すデモコード。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードのサーバー機能を示すデモコード。
- **`src/events.rs`**: YM2151レジスタイベント（アドレス、データ、タイムスタンプ）のデータ構造を定義するモジュール。
- **`src/ipc/mod.rs`**: `src/ipc`モジュールのルートファイルで、プロセス間通信の抽象化を提供する。
- **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを使用したプロセス間通信の実装。
- **`src/ipc/protocol.rs`**: クライアントとサーバー間でやり取りされるコマンドやメッセージのプロトコルを定義するモジュール。
- **`src/ipc/windows/mod.rs`**: `src/ipc/windows`モジュールのルートファイル。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプの生成を担当するモジュール。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsの名前付きパイプハンドルをラップするモジュール。
- **`src/ipc/windows/pipe_reader.rs`**: Windowsの名前付きパイプからの読み取りを処理するモジュール。
- **`src/ipc/windows/pipe_writer.rs`**: Windowsの名前付きパイプへの書き込みを処理するモジュール。
- **`src/ipc/windows/test_logging.rs`**: IPCのWindows実装におけるテスト用のロギングモジュール。
- **`src/lib.rs`**: このRustクレートのライブラリコードのメインエントリポイント。公開APIを定義する。
- **`src/logging.rs`**: アプリケーション全体のロギング設定と初期化を行うモジュール。
- **`src/main.rs`**: アプリケーションのメインエントリーポイントで、コマンドライン引数を解析し、サーバーまたはクライアントの役割を起動する。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用して、オーディオスレッドの優先度を上げるためのモジュール。
- **`src/opm.rs`**: Nuked-OPMエミュレータのRustラッパーおよびFFIバインディング。
- **`src/opm_ffi.rs`**: `opm.c`とのForeign Function Interface (FFI) 定義を扱うモジュール。
- **`src/player.rs`**: YM2151のレジスタ操作イベントを受け取り、オーディオを生成する高レベルなプレイヤーロジック。
- **`src/resampler.rs`**: オーディオデータのサンプリングレートを変換するリサンプリング機能を提供するモジュール。
- **`src/scheduler.rs`**: YM2151イベントのタイムベースなスケジューリングと実行を管理するモジュール。
- **`src/server/command_handler.rs`**: クライアントからのコマンドを受け取り、サーバーの内部状態を更新したり、再生を制御したりするロジック。
- **`src/server/connection.rs`**: クライアントとのIPC接続の確立と管理を担うモジュール。
- **`src/server/mod.rs`**: `src/server`モジュールのルートファイルで、サーバーのサブモジュールを公開する。
- **`src/server/playback.rs`**: サーバー側のオーディオ再生ループと、YM2151イベント処理の中核ロジックを定義するモジュール。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中か、インタラクティブモードかなど）を管理するモジュール。
- **`src/wav_writer.rs`**: オーディオデータをWAVファイルとして書き込むためのユーティリティモジュール。
- **`tests/`**: プロジェクト全体の各種テストコードを格納するディレクトリ。

## 関数詳細説明
READMEの情報に基づき、主要なクライアントAPI関数を説明します。
- **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**
  - **役割**: サーバーが起動しているかを確認し、必要であれば自動的にインストールしてバックグラウンドで起動します。サーバーがコマンドを受け付けられる状態になるまで待機し、シームレスなサーバー利用を可能にします。
  - **引数**:
    - `app_name`: クライアントアプリケーションの名前。サーバーの識別や設定に利用される可能性があります。
  - **戻り値**: 処理が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーの存在確認、`cargo install`によるサーバーのインストール、バックグラウンドでのサーバー起動、サーバーの準備完了待機。
- **`client::send_json(json_data: &str) -> anyhow::Result<()>`**
  - **役割**: 非インタラクティブモードで、YM2151レジスタイベントを含むJSONデータをサーバーに送信し、再生を開始します。
  - **引数**:
    - `json_data`: YM2151レジスタイベントを記述したJSON文字列。
  - **戻り値**: 処理が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーへのJSONデータの送信、既存の再生を停止して新しい再生を開始。
- **`client::stop_playback() -> anyhow::Result<()>`**
  - **役割**: サーバーに現在行われているYM2151の再生を停止するよう指示します。
  - **引数**: なし
  - **戻り値**: 処理が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーへの再生停止コマンド送信。
- **`client::shutdown_server() -> anyhow::Result<()>`**
  - **役割**: サーバープロセスを安全にシャットダウンするよう指示します。
  - **引数**: なし
  - **戻り値**: 処理が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーへのシャットダウンコマンド送信。
- **`client::start_interactive() -> anyhow::Result<()>`**
  - **役割**: サーバーをインタラクティブモードで開始します。これにより、連続した音声ストリームを維持しながらリアルタイムな音響制御が可能になります。
  - **引数**: なし
  - **戻り値**: 処理が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーへのインタラクティブモード開始コマンド送信、連続音声ストリームの初期化。
- **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**
  - **役割**: インタラクティブモードで、YM2151レジスタイベントを含むJSONデータをサーバーに送信し、既存の音声ストリームを中断せずにイベントをスケジュールします。
  - **引数**:
    - `json_data`: YM2151レジスタイベントを記述したJSON文字列。タイムスタンプは内部でf64秒単位に変換されます。
  - **戻り値**: 処理が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーへのJSONデータ送信（インタラクティブモード用）、イベントの動的スケジューリング。
- **`client::clear_schedule() -> anyhow::Result<()>`**
  - **役割**: インタラクティブモードで、サーバーにスケジュールされているまだ処理されていない全てのYM2151イベントをキャンセルするよう指示します。これにより、演奏を無音ギャップなしで素早く別のフレーズに切り替えることができます。
  - **引数**: なし
  - **戻り値**: 処理が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーへのスケジュールクリアコマンド送信。
- **`client::get_server_time() -> anyhow::Result<f64>`**
  - **役割**: サーバーの現在の再生時刻（Web Audioの`currentTime`に相当）を取得します。正確なタイミング制御に利用できます。
  - **引数**: なし
  - **戻り値**: サーバーの時刻を秒単位の`f64`で返すか、エラーが発生した場合は`anyhow::Result`を通じてエラー情報が返されます。
  - **機能**: サーバーからの時刻情報取得。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-12-02 07:02:24 JST
