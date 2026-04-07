Last updated: 2026-04-08

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- クライアントからは、JSON音楽データの送信、再生制御、サーバーの自動起動・シャットダウンが可能です。
- Windows専用として設計され、他のRustプロジェクトのオーディオエンジンライブラリとしても利用できます。

## 技術スタック
- フロントエンド: Rustクライアントライブラリ (プログラムからのAPI制御)、コマンドラインインターフェース (CLI)
- 音楽・オーディオ: YM2151 (OPM) エミュレータ (Nuked-OPM Cライブラリ)、リアルタイムオーディオ出力、WAVファイル出力、オーディオリサンプリング
- 開発ツール: Rust (バージョン 1.70以降)、Cargo (Rustビルドシステム/パッケージマネージャ)、rust-script (開発用スクリプト実行)
- テスト: Cargo Test (Rust組み込みテストフレームワーク)、nextest (高速テストランナー)
- ビルドツール: Cargo (Rustプロジェクトビルド)、build.rs (C言語コードとの結合ビルド)
- 言語機能: Rust (FFI、非同期処理、モジュールシステムなどの言語機能)
- 自動化・CI/CD: setup_ci_environment.sh (CI/CD環境セットアップスクリプト)
- 開発標準: .editorconfig (コーディングスタイル統一)、MIT License (プロジェクト本体)、LGPL 2.1 (Nuked-OPM)

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
│   ├── 194.md
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
- **`.config/nextest.toml`**: `nextest`テストランナーの設定ファイル。
- **`.editorconfig`**: エディタのコーディングスタイル設定を定義するファイル。
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定。
- **`.vscode/extensions.json`**: Visual Studio Codeの推奨拡張機能リスト。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース設定。
- **`Cargo.lock`**: `Cargo.toml`に基づいて解決された依存関係の正確なバージョンを記録。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ、依存関係、ビルド設定を定義。
- **`LICENSE`**: プロジェクトのライセンス情報 (MIT License)。
- **`README.ja.md`**: プロジェクトの概要、使い方、開発状況などを日本語で説明するメインドキュメント。
- **`README.md`**: プロジェクトの概要、使い方、開発状況などを英語で説明するメインドキュメント。
- **`_config.yml`**: Jekyllなどの静的サイトジェネレータの設定ファイル（GitHub Pagesなどで使われる可能性）。
- **`build.rs`**: Rustプロジェクトのビルドプロセス中に実行されるカスタムビルドスクリプト。主にC言語の依存ライブラリ（Nuked-OPM）のコンパイルとリンクを設定。
- **`call_opm_clock_64times.c`**: OPMクロックを64回呼び出すためのC言語コード。Nuked-OPMとのFFI（Foreign Function Interface）連携の一部。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発ステータスやプロンプト関連のドキュメント。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイル。
- **`install-ym2151-tools.rs`**: 開発ツールの一括インストールに使うRustスクリプト。
- **`issue-notes/`**: 開発中の課題やメモがMarkdown形式で記録されているディレクトリ。
- **`opm.c`**: YM2151音源チップをエミュレートするC言語ライブラリ (Nuked-OPM) のソースコード。
- **`opm.h`**: `opm.c`のヘッダーファイル。RustからNuked-OPMを呼び出すためのFFI定義に使用。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルJSONデータ。クライアントからの再生テストなどで利用。
- **`setup_ci_environment.sh`**: 継続的インテグレーション(CI)環境をセットアップするためのシェルスクリプト。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングを管理するロジック。
- **`src/audio/commands.rs`**: オーディオ再生システムに対するコマンド（再生開始、停止など）の定義。
- **`src/audio/generator.rs`**: YM2151チップエミュレータからオーディオサンプルを生成するロジック。
- **`src/audio/mod.rs`**: `src/audio`モジュールのルートファイル。サブモジュールを公開。
- **`src/audio/player.rs`**: 実際のオーディオ再生を実行するプレイヤーロジック。
- **`src/audio/scheduler.rs`**: YM2151レジスタイベントを時間軸に沿ってスケジューリングするロジック。
- **`src/audio/stream.rs`**: オーディオデバイスへの出力ストリームを管理するロジック。
- **`src/audio_config.rs`**: オーディオ設定（サンプリングレート、バッファサイズなど）を管理。
- **`src/client/config.rs`**: クライアントアプリケーションの構成設定。
- **`src/client/core.rs`**: クライアントの基本的な機能とロジック。
- **`src/client/interactive.rs`**: インタラクティブモードクライアントの専用ロジック。
- **`src/client/json.rs`**: JSON形式の音楽データをパース・処理するロジック。
- **`src/client/mod.rs`**: `src/client`モジュールのルートファイル。サブモジュールを公開し、外部から利用されるクライアントAPIを提供。
- **`src/client/server.rs`**: クライアントがサーバーと通信するためのロジック。
- **`src/debug_wav.rs`**: デバッグ目的でWAVファイルを生成・保存する機能。
- **`src/demo_client_interactive.rs`**: インタラクティブクライアントのデモンストレーション用コード。
- **`src/demo_server_interactive.rs`**: インタラクティブサーバーのデモンストレーション用コード。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブサーバーのデモンストレーション用コード。
- **`src/events.rs`**: YM2151レジスタイベントのデータ構造を定義。
- **`src/ipc/mod.rs`**: `src/ipc`モジュールのルートファイル。サブモジュールを公開。
- **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを使用したプロセス間通信の実装。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間で交換される通信プロトコル（コマンドやデータ形式）の定義。
- **`src/ipc/windows/mod.rs`**: `src/ipc/windows`モジュールのルートファイル。Windows固有のIPCコンポーネントを公開。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを作成するためのファクトリパターン実装。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsのパイプハンドルを安全に管理する構造体。
- **`src/ipc/windows/pipe_reader.rs`**: Windowsの名前付きパイプからデータを読み込むロジック。
- **`src/ipc/windows/pipe_writer.rs`**: Windowsの名前付きパイプにデータを書き込むロジック。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプテスト用のロギングユーティリティ。
- **`src/lib.rs`**: クレートのライブラリ部分のエントリポイント。公開APIを定義。
- **`src/logging.rs`**: アプリケーション全体で使用されるロギング機能。
- **`src/main.rs`**: アプリケーションのメインエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントモードを起動。
- **`src/mmcss.rs`**: Windows Multimedia Class Scheduler Service (MMCSS) を利用してオーディオ処理の優先度を設定する機能。
- **`src/opm.rs`**: Nuked-OPM CライブラリとRust間のFFIバインディング。
- **`src/opm_ffi.rs`**: `opm.c`への直接的なFFI（Foreign Function Interface）呼び出しを定義。
- **`src/player.rs`**: オーディオ再生の中核となるロジック。
- **`src/resampler.rs`**: 異なるサンプリングレート間でオーディオデータを変換するリサンプリング機能。
- **`src/scheduler.rs`**: オーディオイベントのスケジューリングを管理する汎用的なロジック。
- **`src/server/command_handler.rs`**: クライアントから受信したコマンドを解釈し、適切なサーバーアクションを実行するロジック。
- **`src/server/connection.rs`**: クライアントとの接続を確立・維持するロジック。
- **`src/server/mod.rs`**: `src/server`モジュールのルートファイル。サブモジュールを公開。
- **`src/server/playback.rs`**: サーバーサイドでのオーディオ再生処理。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中かどうか、インタラクティブモードかなど）を管理する構造体。
- **`src/wav_writer.rs`**: オーディオデータをWAVファイル形式で書き出す機能。
- **`tests/`**: プロジェクトの各種テストコードを格納するディレクトリ。

## 関数詳細説明
提供されたプロジェクト情報およびファイル詳細分析からは具体的な関数シグネチャは検出されていませんが、プロジェクトの説明とファイル構造から、以下のような主要な機能を持つ関数群が存在すると推測されます。

- **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**:
    - 役割: YM2151再生サーバーが起動しているかを確認し、必要であれば自動的にインストールしてバックグラウンドで起動します。
    - 引数: `app_name` - クライアントアプリケーションの名前。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`client::send_json(json_data: &str) -> anyhow::Result<()>`**:
    - 役割: 非インタラクティブモードで、YM2151レジスタイベントを含むJSONデータをサーバーに送信し、再生を開始します。
    - 引数: `json_data` - YM2151レジスタイベントのJSON文字列。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`client::stop_playback() -> anyhow::Result<()>`**:
    - 役割: 現在サーバーで再生中のYM2151音楽を停止します。
    - 引数: なし。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`client::start_interactive() -> anyhow::Result<()>`**:
    - 役割: サーバーをインタラクティブモードで開始し、連続的な音声ストリームの準備をします。
    - 引数: なし。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**:
    - 役割: インタラクティブモードで、YM2151レジスタイベントを含むJSONデータをサーバーの再生キューにスケジュールします。これにより、音響ギャップなしでの連続再生や動的な切り替えが可能になります。
    - 引数: `json_data` - YM2151レジスタイベントのJSON文字列。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`client::clear_schedule() -> anyhow::Result<()>`**:
    - 役割: インタラクティブモードで、まだ処理されていない未来の再生イベントをすべてキャンセルし、スケジュールをクリアします。
    - 引数: なし。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`client::get_server_time() -> anyhow::Result<f64>`**:
    - 役割: サーバーの内部オーディオ再生時間（秒単位）を取得し、正確なタイミング制御に利用できます。
    - 引数: なし。
    - 戻り値: 現在のサーバー時刻 (`f64`) またはエラー (`Err(...)`)。

- **`client::stop_interactive() -> anyhow::Result<()>`**:
    - 役割: インタラクティブモードでの再生を終了し、通常のサーバー状態に戻します。
    - 引数: なし。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`client::shutdown_server() -> anyhow::Result<()>`**:
    - 役割: 起動中のYM2151再生サーバーをシャットダウンします。
    - 引数: なし。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

- **`main()` (src/main.rs)**:
    - 役割: プログラムのエントリポイント。コマンドライン引数をパースし、アプリケーションをサーバーモードまたはクライアントモードで起動します。
    - 引数: コマンドライン引数。
    - 戻り値: 成功 (`Ok(())`) またはエラー (`Err(...)`)。

その他、各モジュールファイル (`src/audio/*.rs`, `src/server/*.rs`, `src/ipc/*.rs` など) には、それぞれの機能を実現するためのプライベートおよび公開関数、構造体メソッドが多数定義されていると推測されます。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2026-04-08 07:10:29 JST
