Last updated: 2026-02-12

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタイベントログを、リアルタイムで再生するWindows専用のシステムです。
- サーバーとクライアントの分散アーキテクチャで構成され、JSON形式の音楽データを柔軟に制御できます。
- プログラムからのライブラリ利用やCLI操作に対応し、音楽制作や編集の体験を向上させます。

## 技術スタック
- フロントエンド: CLI (コマンドラインインターフェース) を介した操作、または他のRustアプリケーションからのライブラリ利用を想定。直接的なGUIフロントエンドは提供していません。
- 音楽・オーディオ:
    - Rust Audio Libraries: リアルタイムオーディオ処理のためのライブラリが内部で使用されています。
    - Nuked-OPM: YM2151 (OPM) 音源チップのレジスタイベントをエミュレートし、オーディオサンプルを生成するC言語ライブラリ。
    - Windows Multimedia Class Scheduler Service (MMCSS): Windows環境において、オーディオ処理の優先度を向上させるために活用されます。
- 開発ツール:
    - Rust: プロジェクトの主要なシステムプログラミング言語。
    - Cargo: Rustの公式ビルドシステムおよびパッケージマネージャー。
    - Visual Studio Code: 推奨される開発環境。
- テスト:
    - Cargo test: Rust標準のテストフレームワークを用いて、ユニットテストおよび結合テストが記述されています。
    - Nextest: 高速なテスト実行を可能にするRustのテストランナー。
- ビルドツール:
    - Cargo: プロジェクトのビルド、依存関係管理、テスト実行を担います。
    - `build.rs`: Rustのカスタムビルドスクリプトで、主にC言語で書かれたNuked-OPMのビルドをRustプロジェクトに統合するために使用されます。
- 言語機能:
    - Foreign Function Interface (FFI): C言語で実装されたNuked-OPMライブラリとRustコードを連携させるために使用されます。
    - JSON (データフォーマット): YM2151のレジスタイベントログデータを表現するための標準データ形式として使用されます。
    - anyhow: Rustにおける柔軟なエラーハンドリングを簡素化するためのクレート。
- 自動化・CI/CD:
    - Rust Script: 開発環境のセットアップや関連ツールのインストールを自動化するために使用されるスクリプト実行ツール。
- 開発標準:
    - EditorConfig: 複数のエディタやIDE間でコードスタイルの一貫性を保つための設定ファイル。
    - Git: ソースコードのバージョン管理システム。

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
📄 call_opm_clock_64times.c
📁 generated-docs/
🌐 googled947dc864c270e07.html
📄 install-ym2151-tools.rs
📁 issue-notes/ (開発者向け情報のため省略)
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
  📁 tests/ (開発者向け情報のため詳細省略)
  📄 wav_writer.rs
📁 tests/ (開発者向け情報のため詳細省略)
```

## ファイル詳細説明
-   **`.config/nextest.toml`**: RustのテストランナーであるNextestの設定ファイルで、テスト実行の挙動をカスタマイズするために使用されます。
-   **`.editorconfig`**: 異なるエディタやIDE間でコードのスタイル（インデント、改行コードなど）の一貫性を維持するための設定ファイルです。
-   **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定します。ビルド生成物や一時ファイルなどが含まれます。
-   **`.vscode/extensions.json`**: Visual Studio Codeのワークスペースで推奨される拡張機能をリストアップします。
-   **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義します。
-   **`Cargo.lock`**: Cargoがプロジェクトの依存関係を解決し、実際に使用したクレートの正確なバージョンとその依存ツリーを記録します。これにより、再現性のあるビルドが保証されます。
-   **`Cargo.toml`**: Rustプロジェクトのビルド設定、メタデータ（名前、バージョンなど）、および外部ライブラリ（クレート）の依存関係を記述するマニフェストファイルです。
-   **`LICENSE`**: 本プロジェクトのライセンス条項（MIT License）が記載されています。
-   **`README.ja.md`**: プロジェクトの目的、機能、使用方法などが日本語で説明されたドキュメントです。
-   **`README.md`**: プロジェクトの目的、機能、使用方法などが英語で説明されたドキュメントです。
-   **`_config.yml`**: おそらく静的サイトジェネレータ（例: Jekyll）などの設定ファイルで、ドキュメント生成プロセスに関連するものです。
-   **`build.rs`**: Rustプロジェクトのビルドスクリプトです。主にC言語で書かれたNuked-OPMライブラリをコンパイルし、Rustコードから利用可能にするための処理を記述します。
-   **`call_opm_clock_64times.c`**: Nuked-OPMエミュレータの内部クロックを効率的に複数回呼び出すためのC言語ソースファイルで、FFIを通じてRustから利用されます。
-   **`generated-docs/`**: プロジェクトのドキュメントが自動生成されて格納されるディレクトリです。
-   **`googled947dc864c270e07.html`**: Googleサービスによるサイト認証などに使用される静的なHTMLファイルです。
-   **`install-ym2151-tools.rs`**: `rust-script`を使って、このプロジェクトに関連する開発ツールや依存関係を一括でインストールするためのRustスクリプトです。
-   **`opm.c`**: YM2151音源チップのレジスタ操作をエミュレートし、音響データを生成するNuked-OPMエミュレータの主要なC言語ソースファイルです。
-   **`opm.h`**: `opm.c`で定義された関数やデータ構造の宣言が含まれるC言語のヘッダーファイルです。
-   **`output_ym2151.json`**: YM2151音源のレジスタイベントログデータを含むサンプルJSONファイルです。クライアントモードでの再生テストなどに利用されます。
-   **`setup_ci_environment.sh`**: CI（継続的インテグレーション）環境をセットアップするためのシェルスクリプトです。
-   **`src/audio/buffers.rs`**: オーディオデータのバッファリングメカニズムを実装しており、スムーズな音声再生を支援します。
-   **`src/audio/commands.rs`**: オーディオ再生システム内部で利用されるコマンド（再生開始、停止など）の定義を含みます。
-   **`src/audio/generator.rs`**: YM2151音源エミュレータからオーディオサンプルを生成するロジックをカプセル化しています。
-   **`src/audio/mod.rs`**: `audio`モジュール全体の公開インターフェースを定義し、サブモジュールをまとめています。
-   **`src/audio/player.rs`**: 実際のオーディオ再生を担当する中核的なロジックを含みます。
-   **`src/audio/scheduler.rs`**: YM2151のレジスタイベントを時間に基づいて正確にスケジューリングする機能を提供します。
-   **`src/audio/stream.rs`**: リアルタイムオーディオストリームの開始、停止、データ供給などの管理を行います。
-   **`src/audio_config.rs`**: オーディオのサンプリングレート、バッファサイズ、チャンネル数などのグローバルな設定を定義します。
-   **`src/client/config.rs`**: クライアントアプリケーションの動作に関する設定（サーバー接続情報など）を定義します。
-   **`src/client/core.rs`**: クライアントの基本的な機能（サーバーへの接続、コマンド送信など）を実装します。
-   **`src/client/interactive.rs`**: インタラクティブモードにおけるクライアントからの操作（連続再生、スケジュールクリアなど）を処理するロジックです。
-   **`src/client/json.rs`**: クライアントがサーバーに送信するJSON形式のデータを扱うためのユーティリティ関数を含みます。
-   **`src/client/mod.rs`**: `client`モジュール全体の公開インターフェースを定義し、サブモジュールをまとめています。
-   **`src/client/server.rs`**: クライアントがバックグラウンドでサーバープロセスを起動・管理するためのロジックを提供します。
-   **`src/debug_wav.rs`**: デバッグ目的で生成されたオーディオデータをWAVファイルとして出力するための機能です。
-   **`src/demo_client_interactive.rs`**: インタラクティブクライアントの動作を示すデモンストレーションコードです。
-   **`src/demo_server_interactive.rs`**: インタラクティブモードで動作するサーバーのデモンストレーションコードです。
-   **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードで動作するサーバーのデモンストレーションコードです。
-   **`src/events.rs`**: YM2151のレジスタ操作イベントのデータ構造と、それらを処理するためのロジックを定義します。
-   **`src/ipc/mod.rs`**: プロセス間通信（IPC）モジュール全体の公開インターフェースを定義し、サブモジュールをまとめています。
-   **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを利用したIPCの実装を提供し、サーバーとクライアント間の通信を確立します。
-   **`src/ipc/protocol.rs`**: サーバーとクライアント間で交換されるメッセージのフォーマットと種類を定義する通信プロトコルです。
-   **`src/ipc/windows/mod.rs`**: Windows固有のIPCパイプ関連モジュールをまとめています。
-   **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを生成するためのヘルパー関数群を提供します。
-   **`src/ipc/windows/pipe_handle.rs`**: Windowsパイプのハンドルを安全に管理するためのラッパー構造体を含みます。
-   **`src/ipc/windows/pipe_reader.rs`**: Windowsパイプからデータを非同期的に読み込むためのロジックを実装します。
-   **`src/ipc/windows/pipe_writer.rs`**: Windowsパイプへデータを非同期的に書き込むためのロジックを実装します。
-   **`src/ipc/windows/test_logging.rs`**: Windowsパイプのテスト時にログを記録するためのユーティリティです。
-   **`src/lib.rs`**: 本プロジェクトがライブラリクレートとして提供する公開モジュールと機能のエントリポイントです。
-   **`src/logging.rs`**: アプリケーション全体で使用されるロギングシステム（ログメッセージの出力など）を初期化・設定します。
-   **`src/main.rs`**: アプリケーションのメインエントリポイントです。コマンドライン引数を解析し、サーバーまたはクライアントとしての起動を決定します。
-   **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用して、オーディオ処理スレッドの優先度を高く設定し、低遅延再生を実現します。
-   **`src/opm.rs`**: RustコードからC言語で書かれたNuked-OPMエミュレータを安全に呼び出すためのRustラッパーを提供します。
-   **`src/opm_ffi.rs`**: Nuked-OPM (C言語) とRust間のForeign Function Interface (FFI) の定義を含み、相互運用を可能にします。
-   **`src/player.rs`**: YM2151レジスタイベントを解釈し、Nuked-OPMエミュレータを制御して音響データを生成する主要な音楽再生ロジックです。
-   **`src/resampler.rs`**: 生成されたオーディオサンプルを、指定された出力サンプリングレートに合わせてリサンプリングする機能を提供します。
-   **`src/scheduler.rs`**: 音楽イベントの再生タイミングを管理し、リアルタイムでの正確なイベント処理を保証します。
-   **`src/server/command_handler.rs`**: クライアントからサーバーに送信された各種コマンド（再生、停止、シャットダウンなど）を解析し、適切なサーバーアクションにディスパッチする役割を担います。
-   **`src/server/connection.rs`**: サーバー側でクライアントからのIPC接続を確立し、管理するためのロジックです。
-   **`src/server/mod.rs`**: `server`モジュール全体の公開インターフェースを定義し、サーバー関連のサブモジュールをまとめています。
-   **`src/server/playback.rs`**: サーバーが保持する再生状態（現在の曲、再生位置など）を管理し、音楽再生のライフサイクルを制御します。
-   **`src/server/state.rs`**: サーバー全体の共有状態（再生状態、設定など）を一元的に管理するための構造体やロジックです。
-   **`src/wav_writer.rs`**: リアルタイムで生成されるオーディオデータを標準的なWAVファイル形式で保存するための機能を提供します。

## 関数詳細説明
-   **`main()`**:
    *   役割: アプリケーションのエントリポイント。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのいずれかでプログラムを起動します。
    *   引数: なし（実行環境からコマンドライン引数を取得）。
    *   戻り値: 実行結果を示す `Result<(), anyhow::Error>`。
-   **`client::ensure_server_ready(app_name: &str)`**:
    *   役割: YM2151再生サーバーが利用可能であることを確認します。サーバーが起動していない場合、自動的にインストールとバックグラウンド起動を行い、コマンドを受け付けられる状態になるまで待機します。
    *   引数: `app_name` (サーバープロセスを識別するためのアプリケーション名)。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`client::send_json(json_data: &str)`**:
    *   役割: 非インタラクティブモードで、YM2151レジスタイベントログを含むJSONデータをサーバーに送信し、新しい演奏を開始します。以前の演奏は自動的に停止されます。
    *   引数: `json_data` (再生するJSON形式の音楽データ文字列)。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`client::stop_playback()`**:
    *   役割: サーバーで現在再生中の音楽を停止させ、無音状態にします。
    *   引数: なし。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`client::shutdown_server()`**:
    *   役割: 実行中のYM2151再生サーバープロセスにシャットダウンを指示し、終了させます。
    *   引数: なし。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`client::start_interactive()`**:
    *   役割: サーバーをインタラクティブモードに切り替えます。このモードでは、音声ストリームが継続され、イベントを動的にスケジュールできます。
    *   引数: なし。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`client::play_json_interactive(json_data: &str)`**:
    *   役割: インタラクティブモードで、YM2151レジスタイベントログを含むJSONデータをサーバーの再生スケジュールに追加します。音声ストリームを途切れさせることなく、リアルタイムにイベントを挿入できます。
    *   引数: `json_data` (スケジュールに追加するJSON形式の音楽データ文字列)。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`client::clear_schedule()`**:
    *   役割: インタラクティブモードで、まだ再生されていない未来のすべてのイベントスケジュールをサーバーからクリアします。
    *   引数: なし。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`client::get_server_time()`**:
    *   役割: サーバーの内部タイマーにおける現在の再生時刻（秒単位）を取得します。クライアントからの正確なタイミング制御に利用されます。
    *   引数: なし。
    *   戻り値: 現在のサーバー時刻 (f64秒) または処理の失敗を示す `anyhow::Result<f64>`。
-   **`audio::generator::generate_samples(opm_instance: &mut Opm, output_buffer: &mut [f32], event_queue: &mut EventQueue, current_time: &mut f64)`**:
    *   役割: Nuked-OPMエミュレータ (`opm_instance`) を駆動し、指定された期間のオーディオサンプルを生成して `output_buffer` に書き込みます。同時に、`event_queue` からイベントを取り出してYM2151のレジスタに適用します。
    *   引数: `opm_instance` (YM2151エミュレータインスタンス), `output_buffer` (生成されたサンプルを格納するバッファ), `event_queue` (処理すべきイベントのキュー), `current_time` (現在の再生時刻)。
    *   戻り値: なし。
-   **`server::command_handler::handle_command(command: ServerCommand, server_state: &Arc<ServerState>)`**:
    *   役割: クライアントから受信した `ServerCommand` を解釈し、サーバーの現在の状態 (`server_state`) に応じて適切なアクション（再生開始、停止、シャットダウンなど）を実行します。
    *   引数: `command` (処理するコマンド), `server_state` (サーバーの共有状態)。
    *   戻り値: 処理の成功/失敗を示す `anyhow::Result<()>`。
-   **`wav_writer::write_wav_header(writer: &mut impl Write, sample_rate: u32, num_channels: u16, bit_depth: u16, data_size: u32)`**:
    *   役割: WAVファイルのヘッダ情報を書き込みます。
    *   引数: `writer` (書き込み先), `sample_rate` (サンプルレート), `num_channels` (チャンネル数), `bit_depth` (ビット深度), `data_size` (データ部分のサイズ)。
    *   戻り値: 処理の成功/失敗を示す `std::io::Result<usize>`。

## 関数呼び出し階層ツリー
```
main()
 ├─ client_mode() (CLIでclient引数がある場合)
 │  ├─ client::ensure_server_ready()
 │  │  └─ client::server::install_and_start_server() (必要に応じて)
 │  │     └─ std::process::Command::spawn()
 │  ├─ client::send_json()
 │  │  └─ ipc::pipe_windows::send_command()
 │  │     └─ ipc::windows::pipe_writer::write_to_pipe()
 │  ├─ client::stop_playback()
 │  │  └─ ipc::pipe_windows::send_command()
 │  ├─ client::shutdown_server()
 │  │  └─ ipc::pipe_windows::send_command()
 │  ├─ client::start_interactive()
 │  │  └─ ipc::pipe_windows::send_command()
 │  ├─ client::play_json_interactive()
 │  │  └─ ipc::pipe_windows::send_command()
 │  ├─ client::clear_schedule()
 │  │  └─ ipc::pipe_windows::send_command()
 │  └─ client::get_server_time()
 │     └─ ipc::pipe_windows::send_command()
 │        └─ ipc::windows::pipe_reader::read_from_pipe()
 └─ server_mode() (CLIでserver引数がある場合)
    ├─ ipc::pipe_windows::create_server_pipe()
    ├─ server::connection::listen_for_clients()
    │  └─ server::command_handler::handle_command()
    │     ├─ server::playback::start_playback()
    │     │  ├─ audio::stream::start_audio_stream()
    │     │  │  ├─ audio::generator::generate_samples()
    │     │  │  │  ├─ opm::update() (Nuked-OPM FFI)
    │     │  │  │  └─ audio::scheduler::process_events()
    │     │  │  └─ resampler::resample()
    │     │  └─ debug_wav::start_wav_debug_output() (verboseモード時)
    │     │     └─ wav_writer::write_wav_header()
    │     ├─ server::playback::stop_playback()
    │     └─ server::state::update_state()
    └─ mmcss::enable_mmcss() (Windowsのみ)

---
Generated at: 2026-02-12 07:06:06 JST
