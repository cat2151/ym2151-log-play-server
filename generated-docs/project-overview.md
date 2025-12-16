Last updated: 2025-12-17

# Project Overview

## プロジェクト概要
- YM2151 (OPM) レジスタログをリアルタイムで再生するRust製サーバー・クライアントシステムです。
- JSON形式の音楽データを受け取り、高精度なオーディオ再生とWAVファイル出力が可能です。
- プログラマティックなAPIやコマンドラインツールを提供し、多様な外部アプリケーションとの連携を目的としています。

## 技術スタック
- フロントエンド: クライアントCLI (コマンドラインインターフェース) およびRustライブラリAPI (`client`モジュールが提供)
- 音楽・オーディオ: YM2151 (OPM) ハードウェアエミュレーション (Nuked-OPM CライブラリをFFI経由でRustから利用)、リアルタイムオーディオ再生 (MMCSS活用)、WAVファイル出力、オーディオリサンプリング機能
- 開発ツール: Rust (プログラミング言語)、Cargo (Rust標準のパッケージマネージャーおよびビルドシステム)、anyhow (エラーハンドリングライブラリ)、rust-script (ユーティリティスクリプト実行用)
- テスト: Rust標準テストフレームワーク (`cargo test`コマンドで実行される各種ユニット・統合テスト)
- ビルドツール: Cargo (Rustプロジェクトのビルド、依存関係管理、テスト実行)
- 言語機能: Rust (C言語とのForeign Function Interface (FFI) による連携、名前付きパイプによるプロセス間通信 (Windows専用)、並行処理によるリアルタイム性能確保、メモリ安全性)
- 自動化・CI/CD: `setup_ci_environment.sh` (継続的インテグレーション環境のセットアップスクリプト)
- 開発標準: .editorconfig (エディタ設定ファイルによりコードスタイルを統一)

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
- **`.editorconfig`**: コーディングスタイルを定義し、異なるエディタ間での一貫性を保つための設定ファイルです。
- **`.gitignore`**: Gitが追跡すべきではないファイルやディレクトリを指定するための設定ファイルです。
- **`.vscode/extensions.json`**: Visual Studio Codeのワークスペースで推奨される拡張機能のリストを定義します。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義します。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンを記録し、再現可能なビルドを保証するファイルです。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョン、著者、依存関係など）とビルド設定を定義するファイルです。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載しています。
- **`README.ja.md`**: プロジェクトの日本語による概要、使い方、開発状況、目的などを記述したドキュメントです。
- **`README.md`**: プロジェクトの英語による概要、使い方、開発状況、目的などを記述したドキュメントです。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレータで使用される設定ファイルです。
- **`build.rs`**: Rustプロジェクトのカスタムビルドスクリプトで、主にC言語のNuked-OPMライブラリをビルドし、Rustプロジェクトにリンクするために使用されます。
- **`generated-docs/development-status-generated-prompt.md`**: プロジェクトの生成されたドキュメントやプロンプトに関連するファイルです。
- **`googled947dc864c270e07.html`**: Google Search Consoleなどのサイト認証に使用されるHTMLファイルです。
- **`install-ym2151-tools.rs`**: Rustスクリプトで、開発に必要な関連ツール（`cat-play-mml`など）のインストールを自動化します。
- **`issue-notes/`**: 開発中のメモや検討事項が記録されているディレクトリです。
- **`opm.c`**: YM2151 (OPM) チップのエミュレーションロジックを実装したC言語のソースファイルです。Nuked-OPMライブラリの一部として機能します。
- **`opm.h`**: `opm.c`に対応するヘッダファイルで、C言語インターフェースの宣言を含みます。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルデータを含むJSONファイルです。
- **`setup_ci_environment.sh`**: CI/CD環境（例: GitHub Actions）でプロジェクトのビルドやテストを実行するためのセットアップシェルスクリプトです。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングを管理する機能を提供します。
- **`src/audio/commands.rs`**: オーディオ再生スレッドへのコマンド（再生開始、停止など）を定義し、伝達する仕組みを提供します。
- **`src/audio/generator.rs`**: YM2151音源エミュレータからの生の音響データを生成し、オーディオフレームに変換するロジックを含みます。
- **`src/audio/mod.rs`**: `audio`モジュールのエントリポイントで、サブモジュールを公開します。
- **`src/audio/player.rs`**: 実際のオーディオデバイスを介した音源の再生を制御し、オーディオストリームのライフサイクルを管理します。
- **`src/audio/scheduler.rs`**: タイムラインに沿ってYM2151イベントをオーディオ生成にスケジューリングする役割を担います。
- **`src/audio/stream.rs`**: OSのオーディオAPIと連携し、オーディオデータの連続的なストリーミングを処理します。
- **`src/audio_config.rs`**: オーディオのサンプルレートやチャンネル数などのグローバルな設定を定義します。
- **`src/client/config.rs`**: クライアントアプリケーションの挙動を制御するための設定オプションを定義します。
- **`src/client/core.rs`**: クライアントの基本的な操作（サーバーへの接続、コマンド送信など）をカプセル化する中核ロジックを含みます。
- **`src/client/interactive.rs`**: インタラクティブモードでのクライアント操作（連続再生、スケジュールクリア、サーバー時刻取得など）に特化した機能を提供します。
- **`src/client/json.rs`**: JSON形式の音楽データ（YM2151レジスタイベントログ）のパースとシリアライズを扱います。
- **`src/client/mod.rs`**: `client`モジュールのエントリポイントで、クライアント関連の機能を提供します。
- **`src/client/server.rs`**: クライアントからサーバーへコマンドを送信し、応答を受け取るための通信ロジックを扱います。
- **`src/debug_wav.rs`**: デバッグ目的で生成されたオーディオデータをWAVファイルとして保存する機能を提供します。
- **`src/demo_client_interactive.rs`**: インタラクティブモードクライアントの機能を示すデモコードです。
- **`src/demo_server_interactive.rs`**: インタラクティブモードサーバーの機能を示すデモコードです。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードサーバーの機能を示すデモコードです。
- **`src/events.rs`**: YM2151レジスタへの書き込みイベントのデータ構造（時間、アドレス、データ）とその処理ロジックを定義します。
- **`src/ipc/mod.rs`**: プロセス間通信（IPC）モジュールのエントリポイントです。
- **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを使用したプロセス間通信の実装を提供します。
- **`src/ipc/protocol.rs`**: クライアントとサーバー間でやり取りされるメッセージ（コマンドやデータ）のプロトコルを定義します。
- **`src/ipc/windows/mod.rs`**: Windows固有のパイプ関連機能のルートモジュールです。
- **`src/ipc/windows/pipe_factory.rs`**: 名前付きパイプのインスタンスを生成するファクトリパターンを実装します。
- **`src/ipc/windows/pipe_handle.rs`**: 名前付きパイプのOSレベルのハンドルを安全に管理するためのラッパーです。
- **`src/ipc/windows/pipe_reader.rs`**: 名前付きパイプからデータを非同期に読み込む機能を提供します。
- **`src/ipc/windows/pipe_writer.rs`**: 名前付きパイプへデータを非同期に書き込む機能を提供します。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプ関連テストのためのロギングユーティリティです。
- **`src/lib.rs`**: プロジェクトのライブラリクレートのエントリポイントで、公開API（`client`モジュールなど）を定義します。
- **`src/logging.rs`**: アプリケーション全体で使用されるロギングシステム（例: `log`クレート）の初期設定とユーティリティを提供します。
- **`src/main.rs`**: アプリケーションの実行可能ファイルのエントリポイントです。コマンドライン引数を解析し、サーバーまたはクライアントのどちらかのモードでプログラムを実行します。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用し、オーディオ再生の優先度を高めるための機能を提供します。
- **`src/opm.rs`**: Nuked-OPM CライブラリのYM2151エミュレータにアクセスするためのRustラッパー構造体とメソッドを定義します。
- **`src/opm_ffi.rs`**: C言語で実装されたNuked-OPMライブラリの関数をRustから呼び出すためのForeign Function Interface (FFI) 定義を含みます。
- **`src/player.rs`**: YM2151イベントログを解析し、それをNuked-OPMエミュレータに渡してオーディオを生成する中心的なロジックを含みます。
- **`src/resampler.rs`**: 異なるサンプルレート間でオーディオデータを変換するためのリサンプリングアルゴリズムを実装します。
- **`src/scheduler.rs`**: YM2151イベントを時間順に管理し、適切なタイミングでエミュレータに適用するためのスケジューリングロジックを提供します。
- **`src/server/command_handler.rs`**: クライアントから受信したコマンドを解釈し、サーバーの適切な処理ロジックにディスパッチします。
- **`src/server/connection.rs`**: クライアントからの新しい接続を待ち受け、確立された接続を管理する機能を提供します。
- **`src/server/mod.rs`**: `server`モジュールのエントリポイントで、サーバー関連の機能を提供します。
- **`src/server/playback.rs`**: サーバー側で実際のYM2151ログ再生処理（オーディオ出力含む）を管理します。
- **`src/server/state.rs`**: サーバーの現在の再生状態、モード（インタラクティブ/非インタラクティブ）、設定などを保持する構造体です。
- **`src/wav_writer.rs`**: 生成されたオーディオデータをWAV形式のファイルに書き込むためのユーティリティを提供します。
- **`tests/`**: プロジェクトの機能が正しく動作することを確認するためのユニットテストや統合テストが格納されているディレクトリです。

## 関数詳細説明
- **`client::ensure_server_ready(app_name: &str)`**
    - **役割**: YM2151再生サーバーが利用可能であることを確認し、必要であれば自動的にインストールしてバックグラウンドで起動します。
    - **引数**: `app_name: &str` - サーバーを起動するアプリケーションの名前（例: "cat-play-mml"）。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: サーバーの実行状態チェック、`cargo install`によるインストール、バックグラウンド起動、コマンド受付待機を行います。
- **`client::send_json(json_data: &str)`**
    - **役割**: 非インタラクティブモードで、YM2151レジスタイベントを含むJSONデータをサーバーに送信し、再生を開始します。
    - **引数**: `json_data: &str` - 再生するYM2151イベントログのJSON文字列。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: サーバーへのJSONデータ送信、既存の演奏の自動停止と新しい演奏の開始を制御します。
- **`client::stop_playback()`**
    - **役割**: 現在サーバーで再生中のYM2151音楽を停止し、無音状態にします。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: サーバーへ演奏停止コマンドを送信します。
- **`client::shutdown_server()`**
    - **役割**: 実行中のYM2151再生サーバープロセスを安全にシャットダウンします。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: サーバーへシャットダウンコマンドを送信し、プロセスを終了させます。
- **`client::start_interactive()`**
    - **役割**: サーバーをインタラクティブモードに移行させ、連続した音声ストリームの準備を開始します。このモードでは、イベントを動的にスケジューリングできます。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: サーバーへインタラクティブモード開始コマンドを送信し、シームレスなオーディオ再生のための準備を指示します。
- **`client::play_json_interactive(json_data: &str)`**
    - **役割**: インタラクティブモードで、YM2151レジスタイベントを含むJSONデータをサーバーに送信し、現在の連続音声ストリームに無音ギャップなしでスケジューリングします。
    - **引数**: `json_data: &str` - 再生するYM2151イベントログのJSON文字列。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: サンプル単位で記述されたJSON内のイベント時間をf64秒単位に自動変換し、サーバーの再生スケジュールにイベントを追加します。
- **`client::clear_schedule()`**
    - **役割**: インタラクティブモードにおいて、まだ再生されていない未来のYM2151イベントスケジュールをサーバー上でクリアします。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: サーバーへスケジュールクリアコマンドを送信し、動的な演奏変更を可能にします。
- **`client::get_server_time() -> f64`**
    - **役割**: サーバーの内部オーディオ再生時刻（Web Audioの`currentTime`に相当）を秒単位で取得します。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<f64>` - 処理が成功した場合は現在のサーバー時刻（f64秒）、失敗した場合はエラー。
    - **機能**: サーバーへ時刻取得コマンドを送信し、正確なタイミング制御のための情報を返します。
- **`main()` (src/main.rs)**
    - **役割**: アプリケーションのメインエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントのどちらかのモードでプログラムを実行します。
    - **引数**: なし (実行環境からのコマンドライン引数を内部で利用)。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラー。
    - **機能**: コマンドライン引数のパース、ロギングの初期化、Windows Multimedia Class Scheduler Service (MMCSS) の設定（オーディオ優先度確保）、サーバーまたはクライアントモードのメイン処理への分岐と実行を担います。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-12-17 07:02:42 JST
