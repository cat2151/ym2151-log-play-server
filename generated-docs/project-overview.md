Last updated: 2025-12-22

# Project Overview

## プロジェクト概要
- YM2151音源のレジスタ操作ログをリアルタイムで再生するサーバー/クライアントアプリケーションです。
- クライアントからの指示で演奏の開始、停止、モード切り替えをシームレスに行えます。
- リアルタイム性の高いインタラクティブな音楽制御や音色エディタでの利用を想定しています。

## 技術スタック
- フロントエンド: コマンドラインインターフェース (CLI) およびライブラリ形式でのプログラム制御。GUIは含みません。
- 音楽・オーディオ: YM2151 (OPM) レジスタログを基にしたリアルタイム音源エミュレーション、Nuked-OPM (C言語ライブラリ) によるYM2151エミュレーション、オーディオバッファリング、スケジューリング、リサンプリング、WAVファイル出力 (デバッグ/verbose時)。
- 開発ツール: Rust (プログラミング言語), Cargo (Rustのビルドシステムおよびパッケージマネージャー), Rust-script (Rustスクリプト実行)。
- テスト: Cargo Test (Rust標準のテストフレームワーク), Nextest (高速なRustテストランナー)。
- ビルドツール: Cargo (Rustのビルドシステム)。C言語のコンパイルには`build.rs`を通じて`cc`クレートなどが利用されている可能性があります。
- 言語機能: Rustの非同期プログラミング、FFI (Foreign Function Interface) を使用したC言語ライブラリの結合、強力な型システムとエラーハンドリング (anyhowクレート)。
- 自動化・CI/CD: `setup_ci_environment.sh` (CI環境設定スクリプト)。
- 開発標準: .editorconfig (コードスタイルの統一), .gitignore (バージョン管理対象外ファイルの指定)。

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
├── issue-notes/ (※来訪者向けのため、このディレクトリ内のファイルは詳細説明を省略)
│   ├── 100.md
│   ├── ... (多数のissueノート)
│   └── 99.md
├── opm.c
├── opm.h
├── output_ym2151.json
├── setup_ci_environment.sh
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
    ├── tests/ (※来訪者向けのため、このディレクトリ内のファイルは詳細説明を省略)
    │   ├── audio_tests.rs
    │   ├── ... (多数のテストファイル)
    │   └── wav_writer_tests.rs
    └── wav_writer.rs
```

## ファイル詳細説明
- **`.config/nextest.toml`**: `cargo nextest`という高速なテストランナーの設定ファイル。テストの並列実行や出力形式などを定義します。
- **`.editorconfig`**: 異なるエディタやIDE間で一貫したコーディングスタイル（インデント、改行コードなど）を維持するための設定ファイル。
- **`.gitignore`**: Gitがバージョン管理対象から除外すべきファイルやディレクトリを指定するファイル。ビルド生成物や一時ファイルなどが含まれます。
- **`.vscode/extensions.json`**: VS Codeを使用する際に推奨される拡張機能のリスト。
- **`.vscode/settings.json`**: VS Codeのワークスペース固有の設定ファイル。特定のプロジェクトでのエディタの挙動をカスタマイズします。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンとそれらの推移的依存関係を記録し、ビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのビルド設定、依存関係、パッケージのメタデータなどを定義するマニフェストファイル。
- **`LICENSE`**: プロジェクトがMIT Licenseで提供されることを示すライセンスファイル。
- **`README.ja.md`**: プロジェクトの目的、機能、使用方法などを日本語で説明する主要なドキュメントファイル。
- **`README.md`**: プロジェクトの目的、機能、使用方法などを英語で説明する主要なドキュメントファイル。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレーターでドキュメントサイトを構築する際の設定ファイル。
- **`build.rs`**: Rustプロジェクトのコンパイル時に実行されるカスタムビルドスクリプト。主にC言語の`Nuked-OPM`エミュレータをRustから利用するためのビルドプロセスを管理します。
- **`generated-docs/`**: 自動生成されたドキュメントを格納するためのディレクトリ。
- **`googled947dc864c270e07.html`**: Googleサービスによるウェブサイトの所有権確認に使用されるファイル。
- **`install-ym2151-tools.rs`**: 関連するRustツールやクレートを一括でインストールするためのスクリプト。
- **`issue-notes/`**: 開発中に記録された課題や検討事項を整理するためのディレクトリ。
- **`opm.c`**: YM2151 (OPM) 音源チップのエミュレーションロジックを実装したC言語ソースファイル。`Nuked-OPM`の一部です。
- **`opm.h`**: `opm.c`に対応するC言語ヘッダーファイル。Nuked-OPMエミュレータのインターフェースを定義します。
- **`output_ym2151.json`**: YM2151レジスタへの書き込みイベントを記述したJSON形式のサンプルデータファイル。
- **`setup_ci_environment.sh`**: 継続的インテグレーション (CI) 環境でプロジェクトをビルド・テストするために必要な環境設定を行うシェルスクリプト。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリング管理に関連する機能を提供します。
- **`src/audio/commands.rs`**: オーディオ再生システム内で使用されるコマンド（例: 再生開始、停止など）の定義を扱います。
- **`src/audio/generator.rs`**: YM2151エミュレータから出力されるオーディオ信号を生成するロジックを含みます。
- **`src/audio/mod.rs`**: `src/audio`モジュールのエントリポイント。関連するサブモジュールを公開します。
- **`src/audio/player.rs`**: オーディオ再生の中核的なロジックを担い、イベントスケジューラーと連携して音源を制御します。
- **`src/audio/scheduler.rs`**: YM2151レジスタイベントの発生タイミングを管理し、正確な再生を可能にするスケジューリングロジックを提供します。
- **`src/audio/stream.rs`**: オーディオデバイスへのストリーミング出力や、WAVファイルへの書き込みなど、オーディオデータ出力に関する機能を提供します。
- **`src/audio_config.rs`**: オーディオ再生の各種設定（サンプリングレート、バッファサイズなど）を定義するデータ構造と関連ロジック。
- **`src/client/config.rs`**: クライアントアプリケーションの設定（例: サーバーとの通信設定）を管理します。
- **`src/client/core.rs`**: クライアントの基本的な機能、サーバーへのコマンド送信やサーバー状態の問い合わせなどを担当します。
- **`src/client/interactive.rs`**: インタラクティブモードクライアントの機能を提供します。リアルタイム性の高い動的な再生制御を可能にします。
- **`src/client/json.rs`**: YM2151レジスタイベントログのJSONデータをパースし、アプリケーション内部のデータ構造に変換する処理を担います。
- **`src/client/mod.rs`**: `src/client`モジュールのエントリポイント。クライアント関連のサブモジュールを公開します。
- **`src/client/server.rs`**: クライアント側からサーバーの起動、インストール確認、シャットダウンなどを制御する機能を提供します。
- **`src/debug_wav.rs`**: デバッグ用途で、生成されたオーディオデータをWAVファイルとして保存する機能を提供します。
- **`src/demo_client_interactive.rs`**: インタラクティブモードクライアントの具体的な使用例を示すデモンストレーションコード。
- **`src/demo_server_interactive.rs`**: インタラクティブモードサーバーの具体的な使用例を示すデモンストレーションコード。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードサーバーの具体的な使用例を示すデモンストレーションコード。
- **`src/events.rs`**: YM2151レジスタへの書き込みイベントを表すデータ構造と、それに関連する処理ロジック。
- **`src/ipc/mod.rs`**: プロセス間通信 (IPC) モジュールのエントリポイント。
- **`src/ipc/pipe_windows.rs`**: Windowsの特定機能である名前付きパイプを利用したIPCの実装を提供します。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間でやり取りされるメッセージのプロトコル（フォーマット）を定義します。
- **`src/ipc/windows/mod.rs`**: `src/ipc/windows`モジュールのエントリポイント。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを生成、管理するファクトリ機能を提供します。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsのパイプハンドルを安全に扱うためのラッパー。
- **`src/ipc/windows/pipe_reader.rs`**: 名前付きパイプからデータを非同期的に読み込む機能を提供します。
- **`src/ipc/windows/pipe_writer.rs`**: 名前付きパイプにデータを非同期的に書き込む機能を提供します。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプのテスト時にデバッグ情報をログ出力するための機能。
- **`src/lib.rs`**: このクレートのライブラリ部分のルートファイル。他のモジュールをエクスポートし、クレート全体のインターフェースを定義します。
- **`src/logging.rs`**: アプリケーション全体で使用されるロギングシステムの設定と機能。
- **`src/main.rs`**: アプリケーションの実行エントリポイント。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのいずれかでアプリケーションを起動します。
- **`src/mmcss.rs`**: Windows Multimedia Class Scheduler Service (MMCSS) を利用して、リアルタイムオーディオ処理の優先度を向上させるための機能を提供します。
- **`src/opm.rs`**: C言語で実装されたNuked-OPMエミュレータをRustから呼び出すためのForeign Function Interface (FFI) ラッパーと、関連するYM2151固有のロジック。
- **`src/opm_ffi.rs`**: C言語の`opm.h`ファイルで定義された関数やデータ構造をRust側から利用するためのFFIバインディングを具体的に記述します。
- **`src/player.rs`**: オーディオ再生の全体的なフローと状態を管理する高レベルなプレイヤーロジック。
- **`src/resampler.rs`**: YM2151エミュレータの内部サンプリングレートとオーディオ出力デバイスのサンプリングレート間の変換を行うためのリサンプリングアルゴリズム。
- **`src/scheduler.rs`**: YM2151レジスタイベントのタイミングを正確にスケジュールし、オーディオフレームに同期して実行するロジック。
- **`src/server/command_handler.rs`**: サーバーがクライアントから受信したコマンドを解釈し、適切な処理を実行するロジック。
- **`src/server/connection.rs`**: クライアントからの接続を受け入れ、確立された接続を管理するサーバー側のロジック。
- **`src/server/mod.rs`**: `src/server`モジュールのエントリポイント。サーバー関連のサブモジュールを公開します。
- **`src/server/playback.rs`**: サーバー側での実際のYM2151イベント再生処理を管理します。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中か、インタラクティブモードか、再生中のイベントデータなど）を管理するデータ構造とロジック。
- **`src/wav_writer.rs`**: オーディオデータをWAVファイル形式で書き出すためのユーティリティ機能。

## 関数詳細説明
- **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**
  - **役割**: クライアントがサーバーに接続する前に、サーバーが起動していることを確認し、もし起動していなければ自動的にインストールしてバックグラウンドで起動します。その後、サーバーがコマンドを受け付けられる状態になるまで待機します。
  - **引数**: `app_name` - 現在のアプリケーション名を指定し、サーバーのインストール時に利用されます。
  - **戻り値**: サーバーの準備が完了した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`client::send_json(json_data: &str) -> anyhow::Result<()>`**
  - **役割**: 非インタラクティブモードで、YM2151レジスタイベントログを含むJSON文字列をサーバーに送信し、新しい演奏を開始させます。これにより、以前の演奏は停止し、新しいデータが再生されます。
  - **引数**: `json_data` - YM2151レジスタイベントのJSONデータを含む文字列。
  - **戻り値**: データ送信と再生指示が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`client::stop_playback() -> anyhow::Result<()>`**
  - **役割**: サーバーで現在行われているYM2151イベントの再生を停止し、音源を無音状態にします。
  - **引数**: なし
  - **戻り値**: 再生停止指示が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`client::shutdown_server() -> anyhow::Result<()>`**
  - **役割**: 実行中のYM2151再生サーバープロセスにシャットダウンを指示し、終了させます。
  - **引数**: なし
  - **戻り値**: シャットダウン指示が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`client::start_interactive() -> anyhow::Result<()>`**
  - **役割**: サーバーをインタラクティブモードに移行させ、連続したオーディオストリームの維持を開始します。これにより、クライアントは音響ギャップなしに複数のイベントを動的にスケジューリングできるようになります。
  - **引数**: なし
  - **戻り値**: インタラクティブモード開始指示が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**
  - **役割**: インタラクティブモードで、YM2151レジスタイベントログのJSONデータをサーバーに送信し、既存のオーディオストリームに無音ギャップなしでイベントをスケジューリングします。タイムスタンプは自動的に秒単位に変換されます。
  - **引数**: `json_data` - 再生するYM2151レジスタイベントのJSONデータを含む文字列。
  - **戻り値**: データ送信とスケジューリング指示が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`client::clear_schedule() -> anyhow::Result<()>`**
  - **役割**: インタラクティブモードにおいて、サーバーに現在スケジュールされている未処理のYM2151イベントをすべてキャンセルし、新しいイベントをすぐにスケジューリングできる状態にします。
  - **引数**: なし
  - **戻り値**: スケジュールクリア指示が成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`client::get_server_time() -> anyhow::Result<f64>`**
  - **役割**: サーバーの内部で管理されている現在の再生時刻（秒単位）を取得します。これはクライアントが正確なタイミングでイベントを送信するために利用できます。
  - **引数**: なし
  - **戻り値**: 成功した場合は現在のサーバー時刻を示す`f64`値、エラーが発生した場合は`anyhow::Error`を返します。

- **`server::run_server(config: ServerConfig) -> anyhow::Result<()>`**
  - **役割**: YM2151イベントログ再生サーバーのメインループを開始し、クライアントからの接続を受け入れ、プロトコルに従ってコマンドを処理し、オーディオ再生を管理します。
  - **引数**: `config` - サーバーの起動設定（例: verboseモードの有無、リサンプリング設定など）。
  - **戻り値**: サーバーが正常に終了した場合は`Ok(())`、エラーが発生した場合は`anyhow::Error`を返します。

- **`audio::player::Player::play_events(events: Vec<Ym2151Event>)`**
  - **役割**: YM2151レジスタイベントのリストを受け取り、オーディオ生成パイプラインを通じてNuked-OPMエミュレータを制御し、実際の音声を生成・再生します。
  - **引数**: `events` - 再生対象となるYM2151レジスタイベントのベクター。
  - **戻り値**: なし

## 関数呼び出し階層ツリー
```
main (アプリケーション起動)
├── client_mode_entry (クライアントモードの場合)
│   ├── client::ensure_server_ready
│   │   └── (内部的に `cargo install` や サーバープロセス起動コマンドを呼び出す可能性)
│   ├── client::send_json
│   │   └── ipc::pipe_windows::send_command (抽象化)
│   ├── client::stop_playback
│   │   └── ipc::pipe_windows::send_command (抽象化)
│   ├── client::shutdown_server
│   │   └── ipc::pipe_windows::send_command (抽象化)
│   ├── client::start_interactive
│   │   └── ipc::pipe_windows::send_command (抽象化)
│   ├── client::play_json_interactive
│   │   ├── src::client::json::parse_json (JSONパース)
│   │   └── ipc::pipe_windows::send_command (抽象化)
│   ├── client::clear_schedule
│   │   └── ipc::pipe_windows::send_command (抽象化)
│   └── client::get_server_time
│       └── ipc::pipe_windows::send_query (抽象化)
└── server_mode_entry (サーバーモードの場合)
    └── server::run_server
        ├── ipc::pipe_windows::NamedPipeServer::new (パイプサーバー作成)
        └── server::connection::handle_client_connection (クライアント接続処理ループ)
            ├── ipc::pipe_windows::PipeReader::read_message (コマンド受信)
            └── server::command_handler::handle_command
                ├── server::playback::start_playback
                │   └── audio::player::Player::play_events
                │       ├── audio::scheduler::schedule_event (イベントスケジュール)
                │       └── audio::generator::generate_samples
                │           └── opm::ym2151_write_reg (C言語FFIを通じてNuked-OPMを制御)
                │           └── audio::stream::write_audio_data (オーディオデバイスへ出力)
                ├── server::playback::stop_playback
                └── server::state::update_state (サーバー状態更新)
                └── ipc::pipe_windows::PipeWriter::write_message (応答送信)

---
Generated at: 2025-12-22 07:02:14 JST
