Last updated: 2025-12-23

# Project Overview

## プロジェクト概要
- YM2151 (OPM) チップのレジスタイベントログをリアルタイムで再生するシステムです。
- Rustで実装されたサーバー・クライアントアーキテクチャを持ち、Windows環境で動作します。
- 他のアプリケーションから音楽データを送信し、素早く演奏を切り替えられるリアルタイム音源サービスを提供します。

## 技術スタック
- フロントエンド: 
    - なし (本プロジェクトはバックエンドサービスであり、クライアントはライブラリまたはコマンドラインで動作)
- 音楽・オーディオ: 
    - **Nuked-OPM**: YM2151 (OPM) 音源チップのエミュレーションに利用されるライブラリ。
    - **リアルタイムオーディオ処理**: Rustのオーディオ関連クレート（具体的な名前は明示されていないが、低遅延再生を可能にするための機能）
- 開発ツール: 
    - **Rust 1.70+**: プロジェクトの主要なプログラミング言語と実行環境。
    - **Cargo**: Rustの公式ビルドシステムおよびパッケージマネージャー。
- テスト: 
    - **Cargo test**: Rustに組み込まれたテストフレームワークを利用し、ユニットテストおよび統合テストを実行。
    - **Nextest**: 高速なテスト実行を可能にするRustテストランナー。
- ビルドツール: 
    - **Cargo**: Rustプロジェクトのビルドと依存関係管理。
    - **build.rs**: Rustのビルドスクリプト機能を利用し、C言語コード (opm.c) のコンパイルなどを行う。
- 言語機能: 
    - **Rust**: 安全性、並行性、パフォーマンスを重視したシステムプログラミング言語。
    - **JSON**: 音楽データの記述およびクライアント・サーバー間の通信プロトコルとして利用。
- 自動化・CI/CD: 
    - **setup_ci_environment.sh**: CI (継続的インテグレーション) 環境のセットアップスクリプト。
- 開発標準: 
    - **.editorconfig**: コードのスタイルやフォーマットを統一するための設定ファイル。
    - **.gitignore**: Gitによるバージョン管理から除外するファイルを指定。

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
├── build.rs
├── generated-docs/
├── install-ym2151-tools.rs
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
│   │   └── ... (モジュール内部のテストファイル)
│   └── wav_writer.rs
└── tests/
    ├── audio/
    │   └── ... (オーディオ関連の統合テスト)
    ├── fixtures/
    │   ├── complex.json
    │   └── simple.json
    └── ... (その他の統合テスト、ユーティリティ)
```

## ファイル詳細説明
- **.config/nextest.toml**: Nextestテストランナーの設定ファイル。テストの並列実行や出力形式などを定義します。
- **.editorconfig**: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイル。
- **.gitignore**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定します。
- **.vscode/extensions.json**: VS Code向け推奨拡張機能のリスト。
- **.vscode/settings.json**: VS Codeのワークスペース設定ファイル。
- **Cargo.lock**: プロジェクトの正確な依存関係ツリーとバージョンを記録し、再現可能なビルドを保証します。
- **Cargo.toml**: Rustプロジェクトのマニフェストファイル。プロジェクトのメタデータ、依存関係、ビルド設定などを記述します。
- **LICENSE**: プロジェクトのライセンス情報 (MIT License)。
- **README.ja.md**: プロジェクトの日本語による概要、使い方、開発状況などを説明する主要なドキュメント。
- **README.md**: プロジェクトの英語による概要、使い方、開発状況などを説明する主要なドキュメント。
- **build.rs**: Rustプロジェクトのビルド時に実行されるカスタムビルドスクリプト。主にC言語で書かれた`opm.c`をRustから利用するためにコンパイルします。
- **generated-docs/**: 自動生成されたドキュメントを格納するディレクトリ。
- **install-ym2151-tools.rs**: 関連ツールを一括インストールするためのRustスクリプト。
- **opm.c**: YM2151 (OPM) 音源チップのエミュレーションを行うC言語ソースコード (Nuked-OPM由来)。
- **opm.h**: `opm.c`に対応するヘッダーファイル。
- **output_ym2151.json**: YM2151レジスタイベントログのJSONフォーマットの例を示すデータファイル。
- **setup_ci_environment.sh**: 継続的インテグレーション(CI)環境をセットアップするためのシェルスクリプト。
- **src/audio/**: リアルタイムオーディオ再生の中核を担うモジュール群。
    - `buffers.rs`: オーディオバッファの管理を担当します。
    - `commands.rs`: オーディオエンジンへのコマンド定義と処理を行います。
    - `generator.rs`: YM2151からの音源データを生成します。
    - `mod.rs`: `audio`モジュールのルートファイル。サブモジュールをまとめて公開します。
    - `player.rs`: オーディオ再生ロジック（実際の音を鳴らす部分）を実装します。
    - `scheduler.rs`: 音楽イベントを時間軸に沿ってスケジュールし、適切なタイミングで再生キューに送ります。
    - `stream.rs`: オーディオストリームの管理と低レベルなオーディオデバイスとの連携を行います。
- **src/audio_config.rs**: オーディオ関連の設定（サンプリングレート、バッファサイズなど）を定義します。
- **src/client/**: サーバーと通信し、演奏を制御するためのクライアント側ロジック。
    - `config.rs`: クライアント側の設定を管理します。
    - `core.rs`: クライアントの基本的な機能とサーバー通信の共通処理を実装します。
    - `interactive.rs`: 連続的な音声ストリームを維持しつつ、リアルタイムでのイベントスケジューリングを可能にするインタラクティブモードのクライアント機能を提供します。
    - `json.rs`: JSON形式の音楽データのパースやシリアライズを扱います。
    - `mod.rs`: `client`モジュールのルートファイル。サブモジュールをまとめて公開します。
    - `server.rs`: サーバーとの接続確立や状態確認、自動起動などの処理を行います。
- **src/debug_wav.rs**: デバッグ目的で生成されたオーディオデータをWAVファイルとして出力する機能を提供します。
- **src/demo_client_interactive.rs**: インタラクティブモードクライアントのデモンストレーションコード。
- **src/demo_server_interactive.rs**: インタラクティブモードサーバーのデモンストレーションコード。
- **src/demo_server_non_interactive.rs**: 非インタラクティブモードサーバーのデモンストレーションコード。
- **src/events.rs**: YM2151レジスタイベントのデータ構造とその処理を定義します。
- **src/ipc/**: サーバー・クライアント間のプロセス間通信 (IPC) を管理するモジュール。
    - `mod.rs`: `ipc`モジュールのルートファイル。サブモジュールをまとめて公開します。
    - `pipe_windows.rs`: Windowsの名前付きパイプを利用したIPCの実装。
    - `protocol.rs`: サーバー・クライアント間でやり取りされるメッセージのプロトコル定義。
    - `windows/**`: Windows固有のパイプ関連機能の実装。
        - `mod.rs`: `windows`モジュールのルートファイル。
        - `pipe_factory.rs`: 名前付きパイプの生成を担当します。
        - `pipe_handle.rs`: パイプハンドルを管理します。
        - `pipe_reader.rs`: パイプからのデータ読み込みを扱います。
        - `pipe_writer.rs`: パイプへのデータ書き込みを扱います。
        - `test_logging.rs`: テスト用途のロギング機能。
- **src/lib.rs**: プロジェクトのライブラリクレートのエントリポイント。
- **src/logging.rs**: アプリケーション全体のロギング機能を提供します。
- **src/main.rs**: プロジェクトの実行可能バイナリのエントリポイント。サーバーモードとクライアントモードのどちらで起動するかを制御します。
- **src/mmcss.rs**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用し、オーディオ再生の優先度を高めるための機能。
- **src/opm.rs**: C言語で書かれたNuked-OPMエミュレータ (`opm.c`) をRustから安全に呼び出すためのラッパーを提供します。
- **src/opm_ffi.rs**: RustとC言語の間にFuzzed Foreign Function Interface (FFI) を定義し、Nuked-OPMとの連携を可能にします。
- **src/player.rs**: オーディオデータを再生する高レベルなインターフェースを提供します。
- **src/resampler.rs**: 異なるサンプリングレート間でオーディオデータを変換するリサンプリング機能を提供します。
- **src/scheduler.rs**: 音楽イベントの再生タイミングを管理するスケジューラー。
- **src/server/**: YM2151レジスタイベントログの再生を処理するサーバー側のロジック。
    - `command_handler.rs`: クライアントから受け取ったコマンドを処理します。
    - `connection.rs`: クライアントとのIPC接続を管理します。
    - `mod.rs`: `server`モジュールのルートファイル。サブモジュールをまとめて公開します。
    - `playback.rs`: サーバー内部での実際のオーディオ再生状態を管理します。
    - `state.rs`: サーバーの状態（再生中か、停止中かなど）を管理します。
- **src/tests/**: `src`モジュール内部のユニットテストや統合テストを含むディレクトリ。
- **src/wav_writer.rs**: オーディオデータをWAVファイル形式で出力する機能。
- **tests/**: プロジェクト全体の統合テストやシナリオテストを含むディレクトリ。
    - `audio/**`: オーディオ再生に関する統合テスト。
    - `fixtures/**`: テストで使用されるサンプルデータ（JSONファイルなど）を格納します。

## 関数詳細説明
- **client::ensure_server_ready(app_name: &str)**
    - 役割: サーバーアプリケーションが起動していることを確認し、もし起動していなければ自動的にインストールしてバックグラウンドで起動します。
    - 引数: `app_name` - クライアントアプリケーションの名前（サーバーがどのアプリケーションによって起動されたかを識別するため）。
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合はOk、失敗した場合はエラー。
    - 機能: サーバーの存在チェック、Cargoによるインストール、バックグラウンド起動、コマンド受付待機を自動化し、シームレスなサーバー利用を提供します。

- **client::send_json(json_data: &str)**
    - 役割: 非インタラクティブモードで、YM2151レジスタイベントログを含むJSONデータをサーバーに送信し、その再生を開始します。
    - 引数: `json_data` - YM2151イベントログを記述したJSON文字列。
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合はOk、失敗した場合はエラー。
    - 機能: サーバーに対し、提供されたJSONデータに基づいて新しい演奏を開始するよう指示します。前の演奏は自動的に停止されます。

- **client::stop_playback()**
    - 役割: サーバーで現在行われているYM2151の演奏を停止します。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合はOk、失敗した場合はエラー。
    - 機能: サーバーに無音化するよう命令を送信します。

- **client::shutdown_server()**
    - 役割: バックグラウンドで実行中のYM2151再生サーバープロセスを終了させます。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合はOk、失敗した場合はエラー。
    - 機能: サーバーにシャットダウン命令を送信し、リソースを解放します。

- **client::start_interactive()**
    - 役割: サーバーとのインタラクティブモードでの通信を開始します。これにより、連続的な音声ストリームが確立され、リアルタイムでの動的な音楽制御が可能になります。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合はOk、失敗した場合はエラー。
    - 機能: サーバーをインタラクティブなイベントスケジューリングを受け入れる状態に切り替え、無音ギャップのない音楽制御を可能にします。

- **client::play_json_interactive(json_data: &str)**
    - 役割: インタラクティブモードで、YM2151レジスタイベントログを含むJSONデータをサーバーに送信し、連続音声ストリーム内で動的にイベントをスケジューリングして再生します。
    - 引数: `json_data` - YM2151イベントログを記述したJSON文字列。
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合はOk、失敗した場合はエラー。
    - 機能: サーバーの現在の再生状況に影響を与えずに、新しい音楽イベントを再生キューに追加します。

- **client::clear_schedule()**
    - 役割: インタラクティブモードにおいて、サーバーの再生キューからまだ処理されていない未来のイベントをすべてキャンセルします。
    - 引数: なし。
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合はOk、失敗した場合はエラー。
    - 機能: 急な演奏内容の変更や中断が必要な場合に、既存のスケジュールをクリアして新しいイベントをすぐに反映できるようにします。

- **client::get_server_time()**
    - 役割: サーバーが現在認識している再生時刻（秒単位）を取得します。Web Audioの`currentTime`プロパティに相当します。
    - 引数: なし。
    - 戻り値: `anyhow::Result<f64>` - 処理が成功した場合はサーバー時刻 (f64)、失敗した場合はエラー。
    - 機能: クライアントとサーバー間のタイミングを正確に同期させ、精密なリアルタイム音楽制御を支援します。

## 関数呼び出し階層ツリー
```
main (エントリーポイント)
├── client::ensure_server_ready (サーバーが起動していない場合)
│   ├── (内部でサーバープロセスを起動)
│   └── (サーバーがコマンドを受け付けるまで待機)
├── client::send_json (非インタラクティブモードでのデータ送信)
│   └── (IPCを通じてサーバーへJSONデータを送信)
├── client::stop_playback (再生停止)
│   └── (IPCを通じてサーバーへ停止コマンドを送信)
├── client::shutdown_server (サーバーシャットダウン)
│   └── (IPCを通じてサーバーへシャットダウンコマンドを送信)
├── client::start_interactive (インタラクティブモード開始)
│   └── (IPCを通じてサーバーへインタラクティブモード開始コマンドを送信)
├── client::play_json_interactive (インタラクティブモードでのデータ送信)
│   └── (IPCを通じてサーバーへJSONデータをスケジューリングコマンドとして送信)
├── client::clear_schedule (スケジュールのクリア)
│   └── (IPCを通じてサーバーへスケジュールクリアコマンドを送信)
└── client::get_server_time (サーバー時刻の取得)
    └── (IPCを通じてサーバーへ時刻要求コマンドを送信)

---
Generated at: 2025-12-23 07:02:22 JST
