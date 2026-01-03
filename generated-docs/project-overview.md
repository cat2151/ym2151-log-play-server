Last updated: 2026-01-04

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタイベントログをリアルタイムで高精度に再生するシステムです。
- サーバー・クライアント方式を採用し、バックグラウンドでの常駐再生と、柔軟な外部制御を実現します。
- 音色エディタやMMLプレーヤーなど、他のアプリケーションへの組み込みを主な用途としています。

## 技術スタック
- フロントエンド: CLI/ライブラリインターフェース (直接的なGUIは持たず、他のアプリケーションからの制御を前提としています)
- 音楽・オーディオ: Nuked-OPM (YM2151/OPMエミュレータ)、リアルタイムオーディオ処理、WAVファイル出力、オーディオリサンプリング
- 開発ツール: Rust (プログラミング言語、コンパイラ、Cargoパッケージマネージャー)、rust-script (開発用スクリプト実行)
- テスト: Cargo test (Rust標準テストフレームワーク)、nextest (テストランナー設定)
- ビルドツール: Cargo (Rustプロジェクトのビルドおよび依存関係管理)
- 言語機能: Rust (メモリ安全性、並行処理、高性能なシステムプログラミング)
- 自動化・CI/CD: setup_ci_environment.sh (継続的インテグレーション環境セットアップスクリプト)
- 開発標準: .editorconfig (エディタ設定によるコードスタイル統一)

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
📁 issue-notes/
  📖 100.md
  📖 101.md
  ... (多数のissueノート)
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
- **Cargo.toml**: Rustプロジェクトのメタデータ、依存関係、およびビルド設定を定義するファイル。
- **Cargo.lock**: ビルド時に実際に使用される全ての依存クレートの正確なバージョンとチェックサムを記録し、ビルドの再現性を保証します。
- **LICENSE**: 本プロジェクトのライセンス情報 (MIT License) を含みます。
- **README.ja.md, README.md**: プロジェクトの目的、機能、使い方、開発状況などを示す詳細なドキュメント（日本語版と英語版）。
- **_config.yml**: GitHub Pagesなどの静的サイトジェネレーターの設定ファイル。ドキュメントサイト生成に使用されます。
- **build.rs**: Rustのビルドスクリプト。主にC言語のライブラリ（Nuked-OPM）をRustにリンクするためのFFIバインディング生成に利用されます。
- **install-ym2151-tools.rs**: `rust-script`で実行可能な、開発に必要な関連ツールをインストールするためのスクリプト。
- **opm.c, opm.h**: YM2151（OPM）音源チップの動作をエミュレートする「Nuked-OPM」ライブラリのC言語ソースコードおよびヘッダーファイル。本プロジェクトの音源生成の中核を担います。
- **output_ym2151.json**: YM2151レジスタイベントログのJSON形式のサンプルデータ。クライアントからの入力例として使われます。
- **setup_ci_environment.sh**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。
- **src/main.rs**: アプリケーションのエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントのどちらとして動作するかを決定します。
- **src/lib.rs**: このプロジェクトがライブラリ（クレート）として他のRustプロジェクトから利用される際の公開インターフェースを定義します。
- **src/audio_config.rs**: オーディオ再生に関するグローバルな設定（サンプリングレート、バッファサイズなど）を管理します。
- **src/events.rs**: YM2151レジスタイベントのデータ構造（時間、アドレス、データなど）およびそのJSONパースロジックを定義します。
- **src/logging.rs**: アプリケーション全体のログ出力設定と処理を管理し、デバッグや状態監視をサポートします。
- **src/mmcss.rs**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用し、リアルタイムオーディオ処理の優先度をシステム上で高めるためのロジックを提供します。
- **src/opm.rs**: Nuked-OPMエミュレータ（C言語）をRustから安全に操作するためのAPIと、YM2151レジスタへの書き込みロジックを実装します。
- **src/opm_ffi.rs**: C言語で書かれたNuked-OPMライブラリとRust間のForeign Function Interface (FFI) バインディングを定義します。
- **src/player.rs**: YM2151レジスタイベントのシーケンスを解釈し、Nuked-OPMエミュレータを駆動して音声を生成する高レベルなプレイヤーロジックを提供します。
- **src/resampler.rs**: 生成されたオーディオデータを指定されたサンプリングレートに変換するリサンプリングアルゴリズムを実装します。
- **src/scheduler.rs**: YM2151レジスタイベントを時間軸に沿ってソートし、正確なタイミングで再生エンジンに供給する役割を担います。
- **src/debug_wav.rs**: デバッグ目的で生成されたオーディオデータをWAVファイルとしてディスクに書き込む機能を提供します。
- **src/audio/buffers.rs**: オーディオ再生に必要なバッファの管理と操作に関するユーティリティ関数を提供します。
- **src/audio/commands.rs**: オーディオ再生システム内部で使用されるコマンド（例: 再生、停止、一時停止）のデータ構造と処理を定義します。
- **src/audio/generator.rs**: YM2151エミュレータから得られたレジスタ状態に基づき、実際のオーディオサンプルデータを生成する役割を担います。
- **src/audio/mod.rs**: `src/audio`ディレクトリ内のサブモジュールをまとめ、外部に公開するインターフェースを定義します。
- **src/audio/player.rs**: オーディオストリームの開始、停止、およびオーディオデバイスへのデータ供給を抽象化する高レベルなプレイヤーロジックを管理します。
- **src/audio/scheduler.rs**: リアルタイムでオーディオイベントを正確なタイミングでスケジューリングし、音の途切れがないように制御します。
- **src/audio/stream.rs**: 実際のオーディオデバイスとのインターフェース層を提供し、オーディオデータのストリーミング処理を管理します。
- **src/client/config.rs**: クライアントがサーバーに接続するための設定（パイプ名など）や、クライアント固有の挙動に関する設定を管理します。
- **src/client/core.rs**: クライアント機能の主要なビジネスロジックを実装します。サーバーへのコマンド送信、レスポンス処理などが含まれます。
- **src/client/interactive.rs**: インタラクティブモードにおけるクライアント操作（連続再生、リアルタイムでのイベントスケジューリング、時刻同期など）に特化した機能を提供します。
- **src/client/json.rs**: クライアントとサーバー間でやり取りされるJSONデータ（YM2151イベントログなど）の構造と、シリアライズ/デシリアライズ処理を定義します。
- **src/client/mod.rs**: `src/client`ディレクトリ内のサブモジュールをまとめ、クライアント関連の公開APIを定義します。
- **src/client/server.rs**: クライアントプロセスがサーバープロセスを（必要に応じて）自動で起動・終了させるロジックや、サーバーの状態を確認する機能を提供します。
- **src/ipc/mod.rs**: プロセス間通信 (IPC) 機能の共通インターフェースと、サブモジュールの定義をまとめます。
- **src/ipc/pipe_windows.rs**: Windows固有の機能である名前付きパイプを使用したプロセス間通信の実装を提供します。
- **src/ipc/protocol.rs**: クライアントとサーバー間で交換されるメッセージの形式やコマンドセットなど、通信プロトコルの定義を管理します。
- **src/ipc/windows/mod.rs**: Windowsプラットフォームに特化したIPC関連のサブモジュールをまとめます。
- **src/ipc/windows/pipe_factory.rs**: Windowsの名前付きパイプのインスタンスを生成・管理するためのファクトリパターンを実装します。
- **src/ipc/windows/pipe_handle.rs**: Windowsのパイプハンドルを安全かつ効率的に扱うためのラッパーを提供し、リソースリークを防ぎます。
- **src/ipc/windows/pipe_reader.rs**: Windowsの名前付きパイプからデータを受信するための非同期読み取りロジックを実装します。
- **src/ipc/windows/pipe_writer.rs**: Windowsの名前付きパイプへデータを送信するための非同期書き込みロジックを実装します。
- **src/ipc/windows/test_logging.rs**: Windowsパイプのテスト時に利用されるロギングユーティリティ。
- **src/server/command_handler.rs**: クライアントから受信したコマンドを解析し、サーバーの適切な機能（再生開始、停止、シャットダウンなど）にルーティングする役割を担います。
- **src/server/connection.rs**: サーバー側でクライアントからの接続を管理し、データ送受信のためのコネクションハンドリングロジックを提供します。
- **src/server/mod.rs**: `src/server`ディレクトリ内のサブモジュールをまとめ、サーバー関連の公開インターフェースを定義します。
- **src/server/playback.rs**: サーバー内でのYM2151ログの再生制御、状態管理、オーディオデータフローの管理など、再生に関する主要ロジックを実装します。
- **src/server/state.rs**: サーバーの現在の動作状態（アイドル、再生中、インタラクティブモードなど）を管理し、異なるクライアントコマンド間で一貫した動作を保証します。
- **src/wav_writer.rs**: オーディオサンプルデータを標準のWAVファイル形式で保存するための機能を提供します。

## 関数詳細説明
- **`main()`**:
  - 役割: プロジェクトのエントリポイント。コマンドライン引数を解析し、アプリケーションをサーバーモードまたはクライアントモードとして起動します。
  - 引数: なし (コマンドライン引数は内部で処理)
  - 戻り値: `anyhow::Result<()>` (処理の成功/失敗を示す結果型)
- **`client::ensure_server_ready(app_name: &str)`**:
  - 役割: サーバープロセスがバックグラウンドで起動し、クライアントからのコマンドを受け付けられる状態であることを保証します。必要に応じてサーバーのインストールと起動を自動的に行います。
  - 引数: `app_name: &str` - サーバーの名前 (例: "cat-play-mml")。
  - 戻り値: `anyhow::Result<()>`
- **`client::send_json(json_data: &str)`**:
  - 役割: 非インタラクティブモードで、YM2151レジスタイベントを含むJSON文字列をサーバーに送信し、そのデータを再生するよう指示します。前回の再生は自動的に停止されます。
  - 引数: `json_data: &str` - YM2151レジスタイベントを含むJSON形式の文字列。
  - 戻り値: `anyhow::Result<()>`
- **`client::stop_playback()`**:
  - 役割: サーバーに対して、現在行われているYM2151の再生を停止するよう指示します。
  - 引数: なし
  - 戻り値: `anyhow::Result<()>`
- **`client::shutdown_server()`**:
  - 役割: サーバープロセスを安全にシャットダウンするよう指示します。
  - 引数: なし
  - 戻り値: `anyhow::Result<()>`
- **`client::start_interactive()`**:
  - 役割: サーバーをインタラクティブモードに切り替えます。これにより、音声ストリームを維持したまま、複数のJSONイベントデータをスムーズに送信・切り替えできるようになります。
  - 引数: なし
  - 戻り値: `anyhow::Result<()>`
- **`client::play_json_interactive(json_data: &str)`**:
  - 役割: インタラクティブモードで、YM2151レジスタイベントを含むJSON文字列をサーバーに送信し、連続的な音声ストリーム内で再生をスケジューリングします。無音ギャップなしでの切り替えが可能です。
  - 引数: `json_data: &str` - YM2151レジスタイベントを含むJSON形式の文字列。
  - 戻り値: `anyhow::Result<()>`
- **`client::clear_schedule()`**:
  - 役割: インタラクティブモードにおいて、まだ処理されていない未来のYM2151レジスタイベントスケジュールをサーバー上でクリアします。これにより、リアルタイムでのフレーズ切り替えが容易になります。
  - 引数: なし
  - 戻り値: `anyhow::Result<()>`
- **`client::get_server_time()`**:
  - 役割: インタラクティブモードで、サーバーの現在の再生時刻をf64秒単位で取得します。Web Audioの`currentTime`に相当する機能です。
  - 引数: なし
  - 戻り値: `anyhow::Result<f64>` (現在のサーバー時刻を秒で返す)
- **`src::server::command_handler::handle_command()` (仮称)**:
  - 役割: クライアントから名前付きパイプ経由で送られてきたコマンドを解析し、それに応じたサーバー内の処理（再生、停止、状態変更など）を実行します。
  - 引数: 受信したコマンドデータ、現在のサーバー状態など
  - 戻り値: 処理結果
- **`src::server::playback::start_playback()` (仮称)**:
  - 役割: YM2151レジスタイベントログの再生を開始します。オーディオストリームを初期化し、イベントスケジューラーとYM2151エミュレータを連携させます。
  - 引数: 再生するYM2151イベントデータ、再生モードなど
  - 戻り値: 処理結果
- **`src::audio::generator::generate_samples()` (仮称)**:
  - 役割: Nuked-OPMエミュレータを駆動し、YM2151のレジスタ設定に基づいて一定期間のオーディオサンプルデータを生成します。
  - 引数: YM2151エミュレータの状態、生成するサンプル数など
  - 戻り値: 生成されたオーディオサンプルデータ

## 関数呼び出し階層ツリー
提供された情報から具体的なコード内の関数呼び出し階層は分析できませんでした。
しかし、プロジェクトの機能説明に基づき、主要なクライアント操作とサーバー内部の概念的な連携フローを以下に示します。

```
[クライアントアプリケーション]
└── main (クライアントモード)
    ├── client::ensure_server_ready()
    │   ├── (サーバーが未起動の場合) サーバープロセスをバックグラウンドで起動
    │   └── サーバーとのIPC接続 (名前付きパイプ) を確立
    ├── (非インタラクティブモードでの再生) client::send_json(json_data)
    │   └── IPC通信によりJSONデータをサーバーへ送信
    ├── (インタラクティブモード開始) client::start_interactive()
    │   └── IPC通信によりサーバーへインタラクティブモード開始コマンドを送信
    ├── (インタラクティブモードでの再生) client::play_json_interactive(json_data)
    │   └── IPC通信によりJSONデータをサーバーへ送信 (リアルタイムスケジューリング)
    ├── (インタラクティブモードでのスケジュールクリア) client::clear_schedule()
    │   └── IPC通信によりサーバーへスケジュールクリアコマンドを送信
    ├── (インタラクティブモードでの時刻同期) client::get_server_time()
    │   └── IPC通信によりサーバーへ時刻取得コマンドを送信
    ├── client::stop_playback()
    │   └── IPC通信によりサーバーへ再生停止コマンドを送信
    └── client::shutdown_server()
        └── IPC通信によりサーバーへシャットダウンコマンドを送信

[サーバーアプリケーション]
└── main (サーバーモード)
    ├── IPC::pipe_windows::listen() (名前付きパイプでクライアント接続を待機)
    └── ループ: クライアントからのコマンドを処理
        ├── src::server::command_handler::handle_command() (受信コマンドの処理)
        │   ├── (再生コマンドの場合) src::server::playback::start_playback()
        │   │   ├── src::scheduler::schedule_events() (YM2151イベントをスケジュール)
        │   │   └── src::audio::player::start_stream() (オーディオストリーム開始)
        │   │       └── src::audio::stream::audio_callback() (オーディオデバイスからのコールバック)
        │   │           ├── src::audio::scheduler::get_next_event() (次のイベントを取得)
        │   │           ├── src::opm::synthesize() (YM2151エミュレータによる音源生成)
        │   │           ├── src::resampler::process() (サンプリングレート変換)
        │   │           └── (verboseモードの場合) src::debug_wav::write_samples() (WAVファイル出力)
        │   ├── (停止コマンドの場合) src::server::playback::stop_playback()
        │   └── (シャットダウンコマンドの場合) サーバープロセスを終了

---
Generated at: 2026-01-04 07:02:31 JST
