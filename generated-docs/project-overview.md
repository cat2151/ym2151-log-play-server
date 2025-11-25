Last updated: 2025-11-26

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するシステムです。
- Windows専用のサーバー・クライアントアーキテクチャを採用し、柔軟な音楽制御とWAVファイル出力機能を提供します。
- リアルタイムでのインタラクティブな演奏切り替えや音源エディタとの連携を目指しています。

## 技術スタック
- フロントエンド: このプロジェクトはGUIフロントエンドを持たず、コマンドラインインターフェース（CLI）またはRustライブラリとしてプログラムから利用されることを想定しています。
- 音楽・オーディオ: YM2151 (OPM) ハードウェアエミュレーション（Nuked-OPM C言語ライブラリを使用）、リアルタイムオーディオ再生、WAVファイル出力、高精度なオーディオリサンプリング。
- 開発ツール: Rust言語（バージョン1.70以降）、Cargo（Rustのビルドシステムとパッケージマネージャー）、rust-script（開発支援スクリプトの実行）。
- テスト: `cargo test`コマンドで実行されるユニットテストおよび統合テスト。
- ビルドツール: Cargo (Rustプロジェクトのビルド、依存関係管理、テスト実行)。
- 言語機能: Rust (高い安全性とパフォーマンス)、FFI (C言語ライブラリであるNuked-OPMとの連携)。
- 自動化・CI/CD: `setup_ci_environment.sh`スクリプト（CI環境構築用）。
- 開発標準: `.editorconfig`（エディタ設定によるコードスタイル統一）。

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
├── install-ym2151-tools.rs
├── issue-notes/
│   ├── 100.md
│   ├── 101.md
│   ├── ... (他のIssueノートファイル)
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
│   │   ├── debug_wav_tests.rs
│   │   ├── ... (他のsrc内テストファイル)
│   │   └── wav_writer_tests.rs
│   └── wav_writer.rs
└── tests/
    ├── audio/
    │   ├── audio_playback_test.rs
    │   ├── audio_sound_test.rs
    │   └── mod.rs
    ├── clear_schedule_test.rs
    ├── cli_integration_test.rs
    ├── ... (他のルートテストファイル)
    ├── fixtures/
    │   ├── complex.json
    │   └── simple.json
    └── test_util_server_mutex.rs
```

## ファイル詳細説明
-   **`.editorconfig`**: コードエディタの設定ファイルで、インデントスタイルや文字コードなど、プロジェクト全体のコーディング規約を定義します。
-   **`.gitignore`**: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
-   **`.vscode/`**: Visual Studio Code用の開発設定を格納するディレクトリです。
    -   **`extensions.json`**: プロジェクト推奨のVS Code拡張機能を定義します。
    -   **`settings.json`**: VS Codeのワークスペース固有の設定を定義します。
-   **`Cargo.lock`**: Cargoがビルド時に使用する正確な依存関係のバージョンを記録します。
-   **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、依存クレート、ビルド設定などを定義します。
-   **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載しています。
-   **`README.ja.md` / `README.md`**: プロジェクトの概要、機能、使用方法、開発状況などを説明するドキュメントです（日本語版と英語版）。
-   **`_config.yml`**: GitHub Pagesなどの設定ファイル（プロジェクト情報には直接記載なし）。
-   **`build.rs`**: Rustプロジェクトのビルドスクリプトで、主にC言語の`opm.c`をコンパイルするために使用されます。
-   **`generated-docs/`**: 生成されたドキュメントを格納するディレクトリです。
-   **`install-ym2151-tools.rs`**: 関連ツールを一括インストールするためのRustスクリプトです。
-   **`issue-notes/`**: 開発中の課題やメモを記録したMarkdownファイル群です。
-   **`opm.c` / `opm.h`**: YM2151 (OPM) 音源チップのエミュレーションを行うC言語のソースファイルとヘッダファイルです（Nuked-OPMライブラリ）。
-   **`output_ym2151.json`**: YM2151のレジスタイベントログを含むJSONファイルのサンプルです。
-   **`setup_ci_environment.sh`**: CI (継続的インテグレーション) 環境をセットアップするためのシェルスクリプトです。
-   **`src/main.rs`**: アプリケーションのエントリポイントで、コマンドライン引数を解析し、サーバーまたはクライアントモードの実行を制御します。
-   **`src/lib.rs`**: クレートのメインライブラリファイルで、外部から利用されるAPI（特にクライアント機能）を定義します。
-   **`src/audio/`**: オーディオ再生関連のモジュール群です。
    -   **`buffers.rs`**: オーディオデータのバッファリングに関する処理を定義します。
    -   **`commands.rs`**: オーディオエンジンに送信されるコマンド（再生開始、停止など）を定義します。
    -   **`generator.rs`**: YM2151レジスタの状態からオーディオ波形を生成するロジックを扱います。
    -   **`mod.rs`**: `audio`モジュールのルートファイルです。
    -   **`player.rs`**: オーディオ再生全体の制御を担うプレーヤーロジックを実装します。
    -   **`scheduler.rs`**: YM2151イベントを時間に基づいてスケジューリングする機能を提供します。
    -   **`stream.rs`**: オーディオストリームの管理と出力を担当します。
-   **`src/audio_config.rs`**: オーディオ設定（サンプリングレートなど）を定義します。
-   **`src/client/`**: クライアント側の機能を提供するモジュール群です。
    -   **`config.rs`**: クライアントの設定を管理します。
    -   **`core.rs`**: クライアントの基本的な制御コマンド（停止、シャットダウンなど）を実装します。
    -   **`interactive.rs`**: インタラクティブモードでのリアルタイム再生制御ロジックを実装します。
    -   **`json.rs`**: JSON形式の音楽データのパースと送信を扱います。
    -   **`mod.rs`**: `client`モジュールのルートファイルです。
    -   **`server.rs`**: クライアントからサーバーとの通信を確立・管理するロジックを定義します。
-   **`src/debug_wav.rs`**: デバッグ目的でWAVファイルを生成する機能を提供します。
-   **`src/demo_client_interactive.rs`**: インタラクティブクライアントモードのデモコードです。
-   **`src/demo_server_interactive.rs`**: インタラクティブサーバーモードのデモコードです。
-   **`src/demo_server_non_interactive.rs`**: 非インタラクティブサーバーモードのデモコードです。
-   **`src/events.rs`**: YM2151レジスタイベントの構造と処理を定義します。
-   **`src/ipc/`**: プロセス間通信（IPC）に関するモジュール群です。
    -   **`mod.rs`**: `ipc`モジュールのルートファイルです。
    -   **`pipe_windows.rs`**: Windowsの名前付きパイプを使用したIPCの実装です。
    -   **`protocol.rs`**: クライアントとサーバー間の通信プロトコル（コマンド、データ形式）を定義します。
    -   **`windows/`**: Windows固有のパイプ関連モジュール群です。
        -   **`pipe_factory.rs`**: 名前付きパイプの生成を担当します。
        -   **`pipe_handle.rs`**: パイプハンドルのラッパーを提供します。
        -   **`pipe_reader.rs`**: パイプからのデータ読み込みを扱います。
        -   **`pipe_writer.rs`**: パイプへのデータ書き込みを扱います。
        -   **`test_logging.rs`**: パイプテスト用のロギング機能です。
-   **`src/logging.rs`**: アプリケーション全体のロギング設定と機能を提供します。
-   **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用して、オーディオ再生の優先度を上げるための機能を提供します。
-   **`src/opm.rs`**: YM2151エミュレータ（Nuked-OPM）のRustラッパーです。
-   **`src/opm_ffi.rs`**: C言語のNuked-OPMライブラリとのFFI（Foreign Function Interface）バインディングを定義します。
-   **`src/player.rs`**: (おそらく`src/audio/player.rs`と統合されるか、高レベルのプレーヤーロジックを扱う) オーディオ再生の主要ロジックを扱います。
-   **`src/resampler.rs`**: オーディオサンプリングレート変換（リサンプリング）のロジックを実装します。
-   **`src/scheduler.rs`**: (おそらく`src/audio/scheduler.rs`と統合されるか、高レベルのイベントスケジューラ) 音楽イベントのスケジュール管理を行います。
-   **`src/server/`**: サーバー側の機能を提供するモジュール群です。
    -   **`command_handler.rs`**: クライアントからのコマンドを処理するロジックを実装します。
    -   **`connection.rs`**: クライアントとの接続管理を扱います。
    -   **`mod.rs`**: `server`モジュールのルートファイルです。
    -   **`playback.rs`**: サーバーでの実際の音楽再生フローを管理します。
    -   **`state.rs`**: サーバーの現在の状態（再生中、停止中など）を管理します。
-   **`src/tests/`**: 開発中のユニットテストやモジュールごとのテストコードを格納します。
-   **`src/wav_writer.rs`**: オーディオデータをWAVファイルとして出力する機能を提供します。
-   **`tests/`**: 統合テストやアプリケーション全体のエンドツーエンドテストを格納するディレクトリです。

## 関数詳細説明
-   **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**:
    -   **役割**: YM2151再生サーバーが起動しているか確認し、必要に応じて自動的にインストールおよびバックグラウンドで起動します。
    -   **引数**: `app_name` - サーバーがインストールされているアプリケーションの名前。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: サーバーの存在確認、`cargo`経由でのインストール、バックグラウンド起動、コマンド受付待機を自動化し、シームレスな開発体験を提供します。
-   **`client::send_json(json_data: &str) -> anyhow::Result<()>`**:
    -   **役割**: 指定されたJSON形式のYM2151レジスタイベントデータをサーバーに送信し、再生を開始します（非インタラクティブモード）。
    -   **引数**: `json_data` - YM2151レジスタイベントを含むJSON文字列。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: JSONデータをサーバーに渡し、新しい演奏を開始します。前の演奏は自動的に停止します。
-   **`client::stop_playback() -> anyhow::Result<()>`**:
    -   **役割**: サーバーに対して現在再生中の音楽を停止するよう指示します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: サーバーのオーディオ出力を無音化します。
-   **`client::shutdown_server() -> anyhow::Result<()>`**:
    -   **役割**: サーバープロセスを安全にシャットダウンするよう指示します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: サーバーアプリケーションを終了させます。
-   **`client::start_interactive() -> anyhow::Result<()>`**:
    -   **役割**: サーバーをインタラクティブモードに切り替え、連続した音声ストリームを開始します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: リアルタイムな音響制御を可能にするためのサーバー状態に移行します。
-   **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**:
    -   **役割**: インタラクティブモードで、JSON形式のYM2151レジスタイベントデータをサーバーのスケジュールに追加します。音声ギャップなしでの連続再生が可能です。
    -   **引数**: `json_data` - YM2151レジスタイベントを含むJSON文字列（サンプル単位の時間が自動で秒単位に変換されます）。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: サーバーの再生キューにイベントを動的に追加し、滑らかな音楽の切り替えや複雑なシーケンスを可能にします。
-   **`client::clear_schedule() -> anyhow::Result<()>`**:
    -   **役割**: インタラクティブモードにおいて、サーバーにスケジュールされている未処理のイベントをすべてクリアします。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: 進行中の演奏を中断し、新しいイベントシーケンスに素早く移行する際に使用されます。
-   **`client::get_server_time() -> anyhow::Result<f64>`**:
    -   **役割**: サーバーの現在の再生時刻を秒単位で取得します。
    -   **引数**: なし。
    -   **戻り値**: 現在のサーバー時刻（秒）を`f64`で返します。エラーが発生した場合は`anyhow::Error`を返します。
    -   **機能**: クライアントとサーバー間で正確なタイミング同期を行うために使用されます（Web Audioの`currentTime`に相当）。
-   **`main()`**:
    -   **役割**: プログラムのメインエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントのどちらかのモードでプログラムを起動します。
    -   **引数**: コマンドライン引数。
    -   **戻り値**: 実行結果を示す`anyhow::Result<()>`。
    -   **機能**: CLIアプリケーションとして、ユーザーの指示に応じたモード（サーバー/クライアント、verbose/non-verbose、インタラクティブ/非インタラクティブ）でプログラムを初期化し、実行します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-11-26 07:02:38 JST
