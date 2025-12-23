Last updated: 2025-12-24

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- サーバーはバックグラウンドで常駐し、クライアントからの命令で演奏の開始・停止・切り替えを柔軟に制御します。
- 音色エディタやMMLプレイヤーなど、他のアプリケーションとの連携を目的としたWindows専用の基盤を提供します。

## 技術スタック
- フロントエンド: クライアントライブラリ（Rust API）およびコマンドラインインターフェース（CLI）を通じて、外部アプリケーションからのプログラム的制御や直接的な操作を可能にします。
- 音楽・オーディオ: Nuked-OPMを基盤としたYM2151 (OPM) ハードウェアエミュレーション、高精度なリアルタイムオーディオ再生、およびデバッグ目的のWAVファイル出力機能を備えています。
- 開発ツール: Rustの公式ビルドシステムであるCargoがプロジェクトのビルドと依存関係管理に使用され、`rust-script`は開発補助スクリプトの実行に利用されます。
- テスト: Rustに組み込まれたテストフレームワーク（`cargo test`）を使用し、コードの機能と信頼性を継続的に検証しています。
- ビルドツール: Rustの標準ツール `Cargo` を用いて、プロジェクトのコンパイル、テスト、実行を効率的に管理します。
- 言語機能: Rust言語の安全性、並行処理、および高性能な機能が活用されています。また、C言語で記述されたNuked-OPMエミュレータとの連携にはFFI（外部関数インターフェース）が利用されています。
- 自動化・CI/CD: `setup_ci_environment.sh`スクリプトは継続的インテグレーション（CI）環境の構築に使用され、開発ワークフローの自動化を支援します。
- 開発標準: `.editorconfig` ファイルにより、様々なエディタやIDE間でのコードスタイルの統一を図り、`.gitignore` でバージョン管理から不要なファイルを除外しています。

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
    └── wav_writer.rs
```

## ファイル詳細説明
-   **Cargo.toml**: Rustプロジェクトのマニフェストファイル。プロジェクトのメタデータ、依存関係、およびビルド設定を定義します。
-   **Cargo.lock**: `Cargo.toml`に基づいて、プロジェクトの依存関係ツリーの正確なバージョンとハッシュを記録します。これにより、全ての環境で一貫したビルドが保証されます。
-   **LICENSE**: プロジェクトのライセンス情報（MIT License）を記載したファイルです。
-   **README.ja.md / README.md**: プロジェクトの日本語版と英語版の概要、機能、使い方、開発状況などを説明する主要なドキュメントファイルです。
-   **_config.yml**: Jekyllなどの静的サイトジェネレータで使用される設定ファイルで、ドキュメント生成やサイトの構成を定義します。
-   **build.rs**: Rustのビルドスクリプト。メインのビルドプロセスに加えて、C言語で書かれたNuked-OPMエミュレータのコンパイルなど、追加のビルドタスクを実行します。
-   **install-ym2151-tools.rs**: `rust-script`を使って実行できるスクリプトで、このプロジェクトの開発や利用に必要なツール群を一括でインストールするために用いられます。
-   **opm.c / opm.h**: YM2151音源のエミュレーションを行うNuked-OPMライブラリのC言語ソースファイルとヘッダファイルです。このプロジェクトの音源生成の根幹を担います。
-   **output_ym2151.json**: YM2151レジスタイベントログのサンプルデータ、またはプログラムが生成する出力ログの例を示すJSONファイルです。
-   **setup_ci_environment.sh**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。ビルドとテストが自動実行される環境を準備します。
-   **src/lib.rs**: このプロジェクトのライブラリクレートのエントリポイントです。公開されるモジュールを宣言し、外部からの利用を可能にします。
-   **src/main.rs**: 実行可能バイナリのエントリポイント。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのどちらかを起動します。
-   **src/audio/buffers.rs**: オーディオデータの効率的なバッファリングと管理を担うロジックが含まれています。
-   **src/audio/commands.rs**: オーディオ再生に関連する各種コマンド（再生、停止など）の定義と処理ロジックです。
-   **src/audio/generator.rs**: YM2151エミュレータから生成されたレジスタ状態に基づいて、実際のオーディオサンプルを生成する中心的なモジュールです。
-   **src/audio/player.rs**: オーディオストリームを管理し、生成されたオーディオデータをリアルタイムで出力する役割を担うプレイヤーロジックです。
-   **src/audio/scheduler.rs**: YM2151レジスタイベントを正確な時間にオーディオジェネレータに送るためのスケジューリングロジックです。
-   **src/audio/stream.rs**: 実際のオーディオデバイスとのインターフェースとなり、オーディオストリームの開始、停止、データ書き込みなどを扱います。
-   **src/audio_config.rs**: オーディオ再生の品質（サンプルレート、バッファサイズなど）に関する設定を定義し、管理します。
-   **src/client/config.rs**: クライアントアプリケーション固有の設定や構成を管理するモジュールです。
-   **src/client/core.rs**: クライアント側の基本的な操作ロジックや共通機能を提供します。
-   **src/client/interactive.rs**: インタラクティブモードにおけるクライアントの操作、例えばリアルタイムでのイベントスケジューリングやサーバー時刻の取得などを扱います。
-   **src/client/json.rs**: YM2151レジスタイベントを含むJSONデータのパース（解析）とシリアライズ（生成）を担当します。
-   **src/client/server.rs**: クライアントがサーバーアプリケーションと通信するための低レベルなメカニズムを提供します。
-   **src/debug_wav.rs**: デバッグ目的で、再生中のオーディオデータをWAVファイルとして出力する機能を提供します。
-   **src/demo_client_interactive.rs / src/demo_server_interactive.rs / src/demo_server_non_interactive.rs**: プロジェクトの主要機能（クライアントのインタラクティブモード、サーバーのインタラクティブ/非インタラクティブモード）の動作を示すデモンストレーションコードです。
-   **src/events.rs**: YM2151レジスタイベントのデータ構造定義と、それらのイベントを処理するためのユーティリティ関数を含みます。
-   **src/ipc/pipe_windows.rs**: Windowsの名前付きパイプを利用したプロセス間通信 (IPC) の具体的な実装を提供します。
-   **src/ipc/protocol.rs**: クライアントとサーバー間でやり取りされるコマンドやデータのフォーマット（プロトコル）を定義します。
-   **src/ipc/windows/pipe_factory.rs**: Windowsの名前付きパイプの生成と設定を行うためのファクトリパターンを実装します。
-   **src/ipc/windows/pipe_handle.rs**: Windowsのパイプハンドルを安全に管理するためのラッパー構造体です。
-   **src/ipc/windows/pipe_reader.rs**: 名前付きパイプからデータを受信するためのロジックを提供します。
-   **src/ipc/windows/pipe_writer.rs**: 名前付きパイプにデータを送信するためのロジックを提供します。
-   **src/logging.rs**: アプリケーション全体で使用されるロギングシステム（ログレベル設定、出力先など）を構成するモジュールです。
-   **src/mmcss.rs**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用し、リアルタイムオーディオ処理のスレッド優先度を高く設定するための機能を提供します。
-   **src/opm.rs**: RustからNuked-OPMエミュレータのC関数を呼び出すための高レベルな安全なAPIラッパーを提供します。
-   **src/opm_ffi.rs**: RustとNuked-OPMのC言語コード間で直接呼び出しを行うためのFFI（Foreign Function Interface）バインディングを定義します。
-   **src/player.rs**: オーディオプレイヤーの抽象インターフェースや共通ロジックを定義します。
-   **src/resampler.rs**: オーディオデータのサンプリングレートを変換（リサンプリング）するためのアルゴリズムと実装を提供します。
-   **src/scheduler.rs**: より汎用的なイベントスケジューリング機能を提供し、オーディオイベントのタイミング制御に利用されます。
-   **src/server/command_handler.rs**: クライアントから送信されたコマンドを解釈し、サーバーの適切な処理ロジックにディスパッチする役割を担います。
-   **src/server/connection.rs**: 複数のクライアントからの接続を管理し、それぞれの通信セッションを維持するためのロジックです。
-   **src/server/playback.rs**: サーバー側での実際のYM2151イベントの再生プロセスと、オーディオ出力へのルーティングを管理します。
-   **src/server/state.rs**: サーバーの現在の状態（再生中、停止中、インタラクティブモードなど）を保持し、状態遷移を管理するモジュールです。
-   **src/wav_writer.rs**: オーディオデータをWAVファイル形式で保存するためのユーティリティ機能を提供します。

## 関数詳細説明
-   **ym2151_log_play_server::client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>**:
    -   **役割**: YM2151再生サーバーが利用可能であることを確認し、必要に応じて自動的にインストールおよび起動します。
    -   **引数**: `app_name` - クライアントアプリケーションの名前。サーバーが既に起動しているかを確認する際の識別子として使用されます。
    -   **戻り値**: サーバーの準備が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: サーバーの実行状態をチェックし、PATHに存在しない場合はCargo経由でインストール。その後、サーバーをバックグラウンドモードで起動し、クライアントからのコマンドを受け付けられる状態になるまで待機します。

-   **ym2151_log_play_server::client::send_json(json_data: &str) -> anyhow::Result<()>**:
    -   **役割**: JSON形式のYM2151レジスタイベントログをサーバーに送信し、非インタラクティブモードでの再生を開始します。
    -   **引数**: `json_data` - 再生するYM2151イベントを含むJSON形式の文字列。
    -   **戻り値**: JSONデータの送信と再生指示が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: サーバーに対して指定されたJSONデータを再生するよう指示します。このモードでは、新しいJSONが送信されると既存の演奏は自動的に停止し、新しい演奏が開始されます。

-   **ym2151_log_play_server::client::stop_playback() -> anyhow::Result<()>**:
    -   **役割**: サーバーで現在行われているYM2151イベントの再生を停止し、無音状態にします。
    -   **引数**: なし。
    -   **戻り値**: 停止指示が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: サーバーに再生中のすべてのYM2151イベントの処理を中断させ、オーディオ出力を停止するよう指示します。

-   **ym2151_log_play_server::client::shutdown_server() -> anyhow::Result<()>**:
    -   **役割**: 起動中のYM2151再生サーバーアプリケーションを安全にシャットダウンします。
    -   **引数**: なし。
    -   **戻り値**: シャットダウン指示が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: サーバープロセスに終了コマンドを送信し、リソースを適切に解放してアプリケーションを終了させます。

-   **ym2151_log_play_server::client::start_interactive() -> anyhow::Result<()>**:
    -   **役割**: サーバーをインタラクティブモードに切り替え、連続した音声ストリームを途切れさせずに維持する準備をします。
    -   **引数**: なし。
    -   **戻り値**: モード切り替えが成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: リアルタイムでの音響制御を可能にするため、サーバーで音声ストリームを開始し、無音期間が発生しないようにします。

-   **ym2151_log_play_server::client::play_json_interactive(json_data: &str) -> anyhow::Result<()>**:
    -   **役割**: JSON形式のYM2151レジスタイベントログをインタラクティブモードのサーバーに送信し、既存の音声ストリームに無音ギャップなしでスケジュールします。
    -   **引数**: `json_data` - 再生するYM2151イベントを含むJSON文字列（内部でサンプル単位の時間が秒単位に自動変換されます）。
    -   **戻り値**: イベントのスケジュールが成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: 連続再生中に新しいイベントデータを動的に追加することで、滑らかな音楽の切り替えやリアルタイムな音色エディットなどを実現します。

-   **ym2151_log_play_server::client::clear_schedule() -> anyhow::Result<()>**:
    -   **役割**: インタラクティブモードでサーバーにスケジュールされている、まだ再生されていない未来のYM2151イベントをすべてキャンセルします。
    -   **引数**: なし。
    -   **戻り値**: スケジュールクリア指示が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: 現在再生中のフレーズを途中で中断し、無音ギャップなしで新しいフレーズに切り替えたい場合などに使用されます。

-   **ym2151_log_play_server::client::get_server_time() -> anyhow::Result<f64>**:
    -   **役割**: サーバーの現在の再生時刻を、秒単位の浮動小数点数として取得します（Web Audioの `currentTime` プロパティと同等の機能）。
    -   **引数**: なし。
    -   **戻り値**: 現在のサーバー時刻を `f64` 型で返すか、エラーが発生した場合は `anyhow::Result` を返します。
    -   **機能**: クライアントとサーバー間での正確なタイミング同期や、イベントの正確なスケジューリングに使用されます。

-   **main()**:
    -   **役割**: プロジェクトの主要なエントリポイントであり、コマンドライン引数に基づいてアプリケーションをサーバーモードまたはクライアントモードとして起動します。
    -   **引数**: なし（コマンドライン引数は実行時にOSから自動的に渡されます）。
    -   **戻り値**: 処理の結果を示す `anyhow::Result` を返します。
    -   **機能**: `server`引数でリアルタイム再生サーバーを、`client`引数でそのサーバーを制御するクライアントを起動します。これにより、単一のバイナリで両方の役割を果たすことができます。

## 関数呼び出し階層ツリー
```
main
├── (server mode execution path)
│   └── server::run_server (サーバーのメインループとクライアント接続処理)
│       ├── ipc::pipe_windows::create_named_pipe (名前付きパイプの作成)
│       └── server::command_handler::handle_command (クライアントからのコマンド処理)
│           ├── audio::player::start_playback (再生開始)
│           ├── audio::player::stop_playback (再生停止)
│           └── audio::scheduler::schedule_events (イベントのスケジューリング)
└── (client mode execution path)
    ├── client::ensure_server_ready (サーバーが起動しているか確認・起動)
    │   ├── client::server::is_server_running
    │   ├── client::server::install_server_if_needed
    │   └── client::server::launch_server_background
    ├── client::send_json (非インタラクティブJSON送信)
    │   └── ipc::protocol::send_command (JSONデータを含むコマンド送信)
    │       └── ipc::pipe_windows::write_to_pipe
    ├── client::stop_playback (再生停止コマンド送信)
    │   └── ipc::protocol::send_command (停止コマンド)
    ├── client::shutdown_server (サーバーシャットダウンコマンド送信)
    │   └── ipc::protocol::send_command (シャットダウンコマンド)
    ├── client::start_interactive (インタラクティブモード開始コマンド送信)
    │   └── ipc::protocol::send_command (インタラクティブ開始コマンド)
    ├── client::play_json_interactive (インタラクティブモードJSON送信)
    │   ├── events::ym2151_events_to_seconds (時間変換)
    │   └── ipc::protocol::send_command (JSONデータを含むコマンド送信)
    ├── client::clear_schedule (スケジュールクリアコマンド送信)
    │   └── ipc::protocol::send_command (スケジュールクリアコマンド)
    └── client::get_server_time (サーバー時刻取得コマンド送信)
        └── ipc::protocol::send_command (時刻取得コマンド)
            └── ipc::pipe_windows::read_from_pipe (結果受信)

---
Generated at: 2025-12-24 07:02:24 JST
