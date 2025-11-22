Last updated: 2025-11-23

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- クライアントからは、JSON形式の音楽データを送信して演奏を制御でき、非インタラクティブモードとリアルタイム性の高いインタラクティブモードをサポートします。
- 外部アプリケーションにライブラリとして組み込むことで、バックグラウンドでの安定した音楽再生機能を提供します。

## 技術スタック
- フロントエンド: 
    - 本プロジェクトはヘッドレス（CLIベース）であるため、直接的なフロントエンド技術は使用していません。他のプロジェクトにライブラリとして組み込むことで、それらのプロジェクトがフロントエンドを提供します。
- 音楽・オーディオ: 
    - **YM2151 (OPM) エミュレーション**: YM2151 FM音源チップの動作をソフトウェアでシミュレートするための「Nuked-OPM」ライブラリを使用しています。
    - **リアルタイムオーディオ再生**: YM2151のレジスタイベントを元に生成された音源を、リアルタイムでOSのオーディオデバイスを通じて再生します。
    - **WAVファイル出力**: デバッグや記録のために、生成されたオーディオをWAV形式でファイルに出力する機能を持っています。
    - **オーディオリサンプリング**: 異なるサンプルレート間で音声を変換するためのリサンプリング機能が組み込まれています。
- 開発ツール: 
    - **Rust 1.70以降**: プロジェクトの主要な実装言語であるRustのコンパイラとエコシステムです。
    - **Cargo**: Rustプロジェクトのビルド、依存関係管理、テスト実行に使用される標準ツールです。
    - **zig cc**: C言語で書かれたNuked-OPMライブラリをコンパイルするためのCコンパイラです。
- テスト: 
    - **`cargo test`**: Rustの標準的なテストフレームワークを使用し、単体テスト、統合テストが実装されています。
- ビルドツール: 
    - **Cargo**: Rustのプロジェクトビルドを管理します。
    - **`build.rs`**: Cargoのカスタムビルドスクリプトで、C言語で書かれたNuked-OPMライブラリのコンパイルなどを処理します。
- 言語機能: 
    - **Rust**: 高い安全性とパフォーマンスを特徴とするシステムプログラミング言語。`anyhow`などのクレートを用いて堅牢なエラーハンドリングを実現しています。
    - **C**: YM2151エミュレーションライブラリ「Nuked-OPM」の実装に使用されています。
- 自動化・CI/CD: 
    - **`setup_ci_environment.sh`**: CI環境をセットアップするためのシェルスクリプトです。
    - **`install-ym2151-tools.rs`**: 関連ツールの一括インストールを自動化するRustスクリプトです。
- 開発標準: 
    - **`.editorconfig`**: コーディングスタイルをプロジェクト全体で統一するための設定ファイルです。

## ファイル階層ツリー
```
ym2151-log-play-server/
├── .cargo/
│   └── config.toml
├── .editorconfig
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.ja.md
├── README.md
├── _config.yml
├── build.rs
├── generated-docs/
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
- **`.cargo/config.toml`**: Cargoのビルド設定ファイル。Cコンパイラとして`zig cc`を使用する設定などが記述されています。
- **`.editorconfig`**: エディタのコードスタイル設定を統一するためのファイルです。
- **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定するファイルです。
- **`Cargo.lock`**: プロジェクトの依存関係の正確なバージョンを記録するファイルです。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存クレートなどが定義されています。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）が記述されています。
- **`README.ja.md` / `README.md`**: プロジェクトの概要、機能、使用方法などが記述された説明書（日本語版と英語版）。
- **`_config.yml`**: GitHub Pagesなどの設定ファイルである可能性がありますが、プロジェクト情報からは詳細不明です。
- **`build.rs`**: Rustのビルドスクリプト。C言語のNuked-OPMをビルドするために使用されます。
- **`generated-docs/`**: 生成されたドキュメントが格納される可能性のあるディレクトリです（現在は空）。
- **`install-ym2151-tools.rs`**: 関連ツールを一括インストールするためのRustスクリプトです。
- **`issue-notes/`**: 開発中の課題やメモが記述されたファイル群です。
- **`opm.c` / `opm.h`**: YM2151エミュレータ「Nuked-OPM」のC言語ソースファイルとヘッダファイルです。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルデータとして使用されるJSONファイルです。
- **`setup_ci_environment.sh`**: CI (継続的インテグレーション) 環境をセットアップするためのシェルスクリプトです。
- **`src/main.rs`**: バイナリクレートのエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントとしての動作を制御します。
- **`src/lib.rs`**: ライブラリクレートのエントリポイント。外部から呼び出される公開APIを定義しています。
- **`src/audio/buffers.rs`**: オーディオデータのバッファ管理に関連するロジックを扱います。
- **`src/audio/commands.rs`**: オーディオ再生に関連する内部コマンドの定義が含まれます。
- **`src/audio/generator.rs`**: YM2151からの音源生成ロジックを管理します。
- **`src/audio/mod.rs`**: `src/audio`モジュールのエントリポイントです。
- **`src/audio/player.rs`**: 実際のオーディオ再生処理の中核を担うロジックが含まれます。
- **`src/audio/scheduler.rs`**: オーディオイベントのタイミング調整やスケジューリングを行います。
- **`src/audio/stream.rs`**: オーディオストリームの管理と、OSのオーディオデバイスへのデータ送信を扱います。
- **`src/audio_config.rs`**: オーディオ再生に関する各種設定（サンプルレートなど）を定義します。
- **`src/client/config.rs`**: クライアントの設定情報やパラメータを管理します。
- **`src/client/core.rs`**: クライアントの基本的な操作ロジックを提供します。
- **`src/client/interactive.rs`**: インタラクティブモードクライアントの機能（リアルタイムイベント送信、スケジュールクリアなど）を実装します。
- **`src/client/json.rs`**: JSONデータのパースやシリアライズなど、JSON関連の処理を扱います。
- **`src/client/mod.rs`**: `src/client`モジュールのエントリポイント。クライアントAPIを集約しています。
- **`src/client/server.rs`**: クライアントがサーバーのライフサイクル（起動、停止、シャットダウン）を管理するためのユーティリティ関数を提供します。
- **`src/debug_wav.rs`**: デバッグ目的でWAVファイルを生成する機能を提供します。
- **`src/demo_client_interactive.rs`**: インタラクティブクライアントのデモコードです。
- **`src/demo_server_interactive.rs`**: インタラクティブモードサーバーのデモコードです。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードサーバーのデモコードです。
- **`src/events.rs`**: YM2151のレジスタイベントのデータ構造や処理ロジックを定義します。
- **`src/ipc/mod.rs`**: `src/ipc`モジュールのエントリポイント。プロセス間通信に関連するモジュールを集約しています。
- **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを使用したプロセス間通信の実装を提供します。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間でやり取りされるメッセージのプロトコルを定義します。
- **`src/ipc/windows/mod.rs`**: Windows固有のIPC関連モジュールを集約しています。
- **`src/ipc/windows/pipe_factory.rs`**: 名前付きパイプの生成を担当するファクトリパターンを実装します。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsのパイプハンドルを安全に扱うためのラッパーを提供します。
- **`src/ipc/windows/pipe_reader.rs`**: 名前付きパイプからのデータ読み込みを管理します。
- **`src/ipc/windows/pipe_writer.rs`**: 名前付きパイプへのデータ書き込みを管理します。
- **`src/ipc/windows/test_logging.rs`**: Windows IPCテスト用のロギング機能を提供します。
- **`src/logging.rs`**: アプリケーション全体のロギング設定と機能を提供します。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用し、オーディオ再生の優先度を上げるための機能を提供します。
- **`src/opm.rs`**: YM2151エミュレータ（Nuked-OPM）をRustから利用するための高レベルなラッパーです。
- **`src/opm_ffi.rs`**: C言語のNuked-OPMライブラリとRust間のForeign Function Interface (FFI) を定義します。
- **`src/player.rs`**: (`src/audio/player.rs`とは異なり、こちらはプロジェクトの全体的な「プレイヤー」機能、つまりサーバーまたはクライアントとしての再生ロジックを管理する可能性があります。プロジェクト情報からはより抽象的なプレイヤー機能と推測されます。)
- **`src/resampler.rs`**: 音声データのリサンプリング処理を実装します。
- **`src/scheduler.rs`**: (`src/audio/scheduler.rs`とは異なり、こちらは高レベルなイベントスケジューリングを担当する可能性があります。プロジェクト情報からはより抽象的なスケジューリング機能と推測されます。)
- **`src/server/command_handler.rs`**: クライアントから送信されたコマンドを処理するロジックが含まれます。
- **`src/server/connection.rs`**: クライアントとの接続を管理します。
- **`src/server/mod.rs`**: `src/server`モジュールのエントリポイント。サーバーサイドの機能を集約しています。
- **`src/server/playback.rs`**: サーバーでの音楽データ再生を管理します。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中か、停止中かなど）を管理します。
- **`src/tests/`**: ユニットテストやモジュールテストが格納されているディレクトリです。
- **`src/wav_writer.rs`**: WAVファイル形式でオーディオデータを書き出す機能を提供します。
- **`tests/`**: 統合テストやエンドツーエンドテストが格納されているディレクトリです。
- **`tests/fixtures/`**: テストで使用されるサンプルJSONデータ（`complex.json`, `simple.json`）が格納されています。

## 関数詳細説明
- **`client::ensure_server_ready(app_name: &str)`**:
    - **役割**: YM2151再生サーバーが利用可能な状態であることを保証します。
    - **引数**: `app_name: &str` - クライアントアプリケーションの名前。サーバーがインストールまたは起動される際に参照されます。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラーが返されます。
    - **機能**: サーバーが既に起動しているか確認し、見つからない場合はCargo経由でインストール・起動します。サーバーがコマンドを受け付けられる状態になるまで待機します。
- **`client::send_json(json_data: &str)`**:
    - **役割**: JSON形式のYM2151レジスタイベントデータをサーバーに送信し、再生を開始します。
    - **引数**: `json_data: &str` - 再生するYM2151イベントを含むJSON文字列です。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラーが返されます。
    - **機能**: 非インタラクティブモードで動作し、各JSON送信ごとに前の演奏を停止し、新しい演奏を開始します。
- **`client::stop_playback()`**:
    - **役割**: 現在サーバーで再生中のYM2151音楽を停止するよう指示します。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラーが返されます。
    - **機能**: サーバーに再生停止コマンドを送信し、音を無音化します。
- **`client::shutdown_server()`**:
    - **役割**: YM2151再生サーバープロセスを終了するよう指示します。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラーが返されます。
    - **機能**: サーバーにシャットダウンコマンドを送信し、サーバープロセスを安全に終了させます。
- **`client::start_interactive()`**:
    - **役割**: サーバーをインタラクティブモードに移行させ、連続的なオーディオストリームの再生を開始します。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラーが返されます。
    - **機能**: リアルタイムな音響制御を可能にするために、サーバーを連続音声ストリームモードに切り替えます。
- **`client::play_json_interactive(json_data: &str)`**:
    - **役割**: インタラクティブモードで、JSON形式のYM2151レジスタイベントをサーバーにスケジュールし、無音ギャップなしで再生に組み込みます。
    - **引数**: `json_data: &str` - スケジュールするYM2151イベントを含むJSON文字列です。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラーが返されます。
    - **機能**: YM2151ログデータをf64秒単位に自動変換し、サーバーのリアルタイムスケジュールにキューイングします。
- **`client::clear_schedule()`**:
    - **役割**: インタラクティブモードで、まだサーバーによって処理されていない未来のイベントスケジュールをクリアします。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、失敗した場合はエラーが返されます。
    - **機能**: リアルタイムで演奏内容を切り替える際に、不要な未来のイベントをキャンセルするために使用されます。
- **`client::get_server_time()`**:
    - **役割**: サーバーの現在の再生時刻を秒単位で取得します。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<f64>` - サーバーの現在の時刻（秒）が返されます。失敗した場合はエラーが返されます。
    - **機能**: クライアントがサーバーとのタイミングを正確に同期させるために使用できる、Web Audioの`currentTime`に相当する機能を提供します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2025-11-23 07:02:32 JST
