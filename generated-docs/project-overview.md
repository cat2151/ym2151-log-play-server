Last updated: 2025-11-22

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するWindows専用のサーバー・クライアントシステムです。
- クライアントはサーバーの再生を柔軟に制御でき、インタラクティブモードでは無音ギャップなしで動的な音響制御が可能です。
- 他の音楽ツール（音色エディタなど）にライブラリとして組み込まれることを想定し、スムーズな開発体験を提供します。

## 技術スタック
- フロントエンド: 該当なし (CLIアプリケーションのため)
- 音楽・オーディオ:
    - **YM2151 (OPM)**: プロジェクトの中心となるヤマハ製FM音源チップのエミュレーション対象。
    - **Nuked-OPM**: YM2151音源チップのエミュレーションに使用されるC言語ライブラリ (LGPL 2.1)。RustからFFI経由で利用。
    - **WAVファイル出力**: デバッグや詳細ログ出力時にWAV形式での音声保存機能を提供。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語 (バージョン 1.70以降)。
    - **Cargo**: Rustのビルドシステムおよびパッケージマネージャー。依存関係の管理、ビルド、テスト、実行を担う。
    - **zig cc**: C言語コード（Nuked-OPM）のコンパイルに使用されるCコンパイラ。
    - **rust-script**: Rustスクリプトを直接実行するためのツール。関連アプリの一括インストールに使用。
- テスト:
    - **Cargo test**: Rustの組み込みテストフレームワークを利用して単体テストおよび統合テストを実行。
- ビルドツール:
    - **Cargo**: Rustプロジェクトのビルドを管理。
    - **Zig cc**: C言語ライブラリのビルドに使用。
- 言語機能:
    - **Anyhow**: Rustで柔軟なエラーハンドリングを行うためのクレート。
    - **標準ライブラリ**: `std::thread`, `std::time::Duration` など、Rustの標準機能が幅広く利用されている。
- 自動化・CI/CD:
    - **setup_ci_environment.sh**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。
- 開発標準:
    - **.editorconfig**: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイル。

## ファイル階層ツリー
```
.
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
│   └── development-status-generated-prompt.md
├── install-ym2151-tools.rs
├── issue-notes/
│   ├── 100.md
│   └── ... (他のissueノート)
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
    ├── tests/
    │   ├── audio_tests.rs
    │   └── ... (他の内部テストファイル)
    └── wav_writer.rs
```

## ファイル詳細説明
- **`.cargo/config.toml`**: Cargoのビルド設定やエイリアスなどを定義するファイル。
- **`.editorconfig`**: 異なるエディタやIDE間で、一貫したコーディングスタイルを維持するための設定ファイル。
- **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定するファイル。
- **`Cargo.lock`**: プロジェクトの依存関係ツリーと、各クレートの正確なバージョンを記録するファイル。再現性のあるビルドを保証する。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョン、著者など）と、外部依存関係、ビルド設定などを定義するマニフェストファイル。
- **`LICENSE`**: 本プロジェクトのライセンス情報（MIT License）を記載したファイル。
- **`README.ja.md` / `README.md`**: プロジェクトの概要、機能、使い方、ビルド方法などを説明するドキュメント（日本語版と英語版）。
- **`_config.yml`**: 通常、Jekyllなどの静的サイトジェネレーターの設定ファイルとして使用されるが、このプロジェクトでの具体的な用途は不明。
- **`build.rs`**: Rustプロジェクトのビルドプロセス中に実行されるビルドスクリプト。主にC/C++ライブラリをRustクレートにリンクするために使用される。このプロジェクトでは`opm.c`のコンパイルとリンクを担う。
- **`generated-docs/`**: 自動生成されたドキュメントを格納するディレクトリ。`development-status-generated-prompt.md`のようなファイルが含まれる。
- **`install-ym2151-tools.rs`**: `rust-script`を用いて、YM2151関連のツール群を一括でインストールするためのスクリプト。
- **`issue-notes/`**: プロジェクト開発中のメモや特定の問題に関するノートを格納するディレクトリ（内容は公開されない）。
- **`opm.c` / `opm.h`**: YM2151音源チップのエミュレーションロジックを含むC言語のソースファイルとヘッダファイル。Nuked-OPMエミュレータの実装。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルデータを含むJSONファイル。テストやデモンストレーションに使用。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするために使用されるシェルスクリプト。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングに関する構造体やロジックを定義します。
- **`src/audio/commands.rs`**: オーディオ再生システムへの内部コマンド（例: 再生開始、イベント追加）を定義します。
- **`src/audio/generator.rs`**: YM2151エミュレータからオーディオサンプルを生成するロジックを管理します。
- **`src/audio/mod.rs`**: `audio`モジュールのルートファイル。関連するサブモジュールをまとめて公開します。
- **`src/audio/player.rs`**: 実際のオーディオ再生プロセスを管理し、イベントスケジューラやオーディオストリームと連携します。
- **`src/audio/scheduler.rs`**: YM2151レジスタイベントのタイムライン管理と、正確なタイミングでのイベント実行をスケジューリングします。
- **`src/audio/stream.rs`**: OSのオーディオAPIと連携し、オーディオストリームの開始、停止、データ供給などを抽象化します。
- **`src/audio_config.rs`**: オーディオ再生に関する各種設定（サンプリングレート、バッファサイズなど）を定義します。
- **`src/client/config.rs`**: クライアント側の設定（例: サーバーとの通信に使用するパイプ名）を定義します。
- **`src/client/core.rs`**: クライアントのコア機能（サーバーへのコマンド送信など）を実装します。
- **`src/client/interactive.rs`**: インタラクティブモードでのサーバー操作に関するクライアントロジックを提供します。
- **`src/client/json.rs`**: JSON形式のYM2151イベントデータをパースしたり、関連する構造体（`Event`, `EventList`など）を定義したりします。
- **`src/client/mod.rs`**: `client`モジュールのルートファイル。クライアント関連の機能群をまとめます。
- **`src/client/server.rs`**: クライアントからサーバーの起動状態を確認し、必要に応じてインストールや起動を行うロジック（`ensure_server_ready`など）を提供します。
- **`src/debug_wav.rs`**: デバッグ目的で生成されたオーディオデータをWAVファイルとして保存する機能を提供します。
- **`src/demo_client_interactive.rs`**: インタラクティブモードで動作するクライアントのデモンストレーションコード。
- **`src/demo_server_interactive.rs`**: インタラクティブモードをサポートするサーバーのデモンストレーションコード。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードをサポートするサーバーのデモンストレーションコード。
- **`src/events.rs`**: YM2151レジスタイベントのデータ構造と、それらを処理するためのロジックを定義します。
- **`src/ipc/mod.rs`**: `ipc`（プロセス間通信）モジュールのルートファイル。関連するサブモジュールをまとめます。
- **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを使用したプロセス間通信の具体的な実装を提供します。
- **`src/ipc/protocol.rs`**: クライアントとサーバー間で送受信されるコマンドやデータのフォーマット（プロトコル）を定義します。
- **`src/ipc/windows/mod.rs`**: Windows固有のIPC実装に関連するサブモジュールをまとめます。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを作成するためのファクトリ機能を提供します。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsの名前付きパイプハンドルを安全に扱うためのラッパーを提供します。
- **`src/ipc/windows/pipe_reader.rs`**: Windowsの名前付きパイプからデータを読み取る機能を提供します。
- **`src/ipc/windows/pipe_writer.rs`**: Windowsの名前付きパイプにデータを書き込む機能を提供します。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプのテスト時に使用されるロギングユーティリティ。
- **`src/lib.rs`**: 本クレートのライブラリ部分のエントリポイント。他のモジュールを公開し、ライブラリとして使用される主要な機能を提供します。
- **`src/logging.rs`**: プロジェクト全体のロギング機能（ログメッセージの出力など）を実装します。
- **`src/main.rs`**: 実行可能ファイルのエントリポイント。コマンドライン引数を解析し、サーバーまたはクライアントのメインロジックを起動します。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用して、オーディオ処理スレッドの優先度を高め、リアルタイム性を確保する機能を提供します。
- **`src/opm.rs`**: RustからC言語で記述されたNuked-OPMエミュレータ（`opm.c`, `opm.h`）を呼び出すための外部関数インターフェース（FFI）バインディングを定義します。
- **`src/opm_ffi.rs`**: `opm.rs`と同様に、C言語のOPMエミュレータとのFFIに関連するコードや、生のバインディングを定義します。
- **`src/player.rs`**: オーディオ再生の全体的なフローを調整し、イベントのスケジューリング、オーディオ生成、ストリームへの出力などを統合します。
- **`src/resampler.rs`**: オーディオデータのサンプリングレートを変換（リサンプリング）するためのアルゴリズムやロジックを提供します。
- **`src/scheduler.rs`**: (おそらく`src/audio/scheduler.rs`と機能的に重複または連携しており、より高レベルなイベントスケジューリングを扱うか、旧バージョン互換性のため存在すると推測されます)
- **`src/server/command_handler.rs`**: クライアントから受信したコマンドを解析し、適切なサーバーアクション（再生開始、停止、設定変更など）をトリガーするロジックを実装します。
- **`src/server/connection.rs`**: クライアントとのIPC接続（名前付きパイプ）を確立し、維持するためのロジックを管理します。
- **`src/server/mod.rs`**: `server`モジュールのルートファイル。サーバー関連の機能群をまとめます。
- **`src/server/playback.rs`**: サーバー側でYM2151のオーディオ再生を管理する主要なロジックを実装します。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中か、インタラクティブモードか、設定など）を保持し、更新するためのロジックを定義します。
- **`src/tests/`**: Rustの内部テストファイルが格納されるディレクトリ。
- **`src/wav_writer.rs`**: 生のオーディオデータをWAVファイルフォーマットで書き出すためのユーティリティ機能を提供します。

## 関数詳細説明
- **`main()`**:
    - 役割: アプリケーションのエントリポイント。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのいずれかを起動します。
    - 引数: なし (Rustの標準 `main` 関数として、環境からコマンドライン引数を取得)
    - 戻り値: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**:
    - 役割: YM2151再生サーバーが起動していることを確認します。もし起動していない、またはPATHに見つからない場合は、自動的に`cargo install`でインストールし、バックグラウンドでサーバーを起動します。サーバーがコマンドを受け付けられる状態になるまで待機します。
    - 引数: `app_name` - このクライアントアプリケーションの名前。サーバーのインスタンスを識別するために使用されます。
    - 戻り値: `anyhow::Result<()>` - サーバーの準備が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::send_json(json_data: &str) -> anyhow::Result<()>`**:
    - 役割: 非インタラクティブモードで、YM2151レジスタイベントのJSONデータをサーバーに送信し、再生を開始します。新しいJSONが送信されると、以前の演奏は停止されます。
    - 引数: `json_data` - YM2151レジスタイベントを含むJSON形式の文字列。
    - 戻り値: `anyhow::Result<()>` - コマンド送信が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::stop_playback() -> anyhow::Result<()>`**:
    - 役割: サーバーに対して現在のYM2151の再生を直ちに停止するよう指示します。
    - 引数: なし
    - 戻り値: `anyhow::Result<()>` - コマンド送信が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::shutdown_server() -> anyhow::Result<()>`**:
    - 役割: サーバープロセスに安全にシャットダウンするよう指示します。
    - 引数: なし
    - 戻り値: `anyhow::Result<()>` - コマンド送信が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::start_interactive() -> anyhow::Result<()>`**:
    - 役割: サーバーをインタラクティブモードに移行させます。このモードでは連続的な音声ストリームが維持され、動的なイベントスケジューリングが可能になります。
    - 引数: なし
    - 戻り値: `anyhow::Result<()>` - モード切り替えコマンド送信が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**:
    - 役割: インタラクティブモード中にYM2151レジスタイベントのJSONデータをサーバーに送信します。既存の音声ストリームを中断することなく、イベントはサーバーのスケジュールに追加されます。
    - 引数: `json_data` - YM2151レジスタイベントを含むJSON形式の文字列。
    - 戻り値: `anyhow::Result<()>` - コマンド送信が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::clear_schedule() -> anyhow::Result<()>`**:
    - 役割: インタラクティブモードにおいて、サーバーでまだ処理されていない未来のYM2151イベントスケジュールをすべてクリアします。これにより、演奏を途中でスムーズに切り替えることができます。
    - 引数: なし
    - 戻り値: `anyhow::Result<()>` - コマンド送信が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。
- **`client::get_server_time() -> anyhow::Result<f64>`**:
    - 役割: サーバーの現在の再生時刻（秒単位）を取得します。クライアントがサーバーのタイミングと同期し、正確なリアルタイム制御を行うために使用されます。
    - 引数: なし
    - 戻り値: `anyhow::Result<f64>` - 成功した場合は現在のサーバー時刻 (f64)、エラーが発生した場合は`Err`を返します。
- **`server::run(options: ServerOptions) -> anyhow::Result<()>`** (推測):
    - 役割: サーバープロセスを起動し、指定されたオプション（verbose、低品位リサンプリングなど）に基づいて初期化を行います。その後、クライアントからの接続を待ち受け、受信したコマンドに応じてYM2151の再生を管理します。
    - 引数: `options` - サーバーの起動オプションを含む構造体。
    - 戻り値: `anyhow::Result<()>` - サーバーの実行が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します。

## 関数呼び出し階層ツリー
```
main (アプリケーションのエントリポイント)
├── server::run (サーバーモードの場合)
│   ├── server::connection::accept_connections (クライアント接続を待ち受け)
│   │   └── server::command_handler::handle_command (受信コマンドを処理)
│   │       ├── server::playback::start_playback (非インタラクティブ再生開始)
│   │       │   ├── audio::player::Player::play_json
│   │       │   │   ├── audio::scheduler::Scheduler::add_events
│   │       │   │   └── audio::stream::AudioStream::start
│   │       │   └── debug_wav::WavWriter::new (verboseモード時)
│   │       ├── server::playback::stop_playback (再生停止)
│   │       │   └── audio::stream::AudioStream::stop
│   │       ├── server::playback::start_interactive (インタラクティブモード開始)
│   │       │   └── audio::stream::AudioStream::start_interactive
│   │       ├── server::playback::play_json_interactive (インタラクティブモードでJSON再生)
│   │       │   └── audio::scheduler::Scheduler::add_events_interactive
│   │       ├── server::playback::clear_schedule (インタラクティブモードでスケジュールクリア)
│   │       │   └── audio::scheduler::Scheduler::clear
│   │       ├── server::playback::get_server_time (サーバー時刻取得)
│   │       │   └── audio::scheduler::Scheduler::get_current_time
│   │       └── server::state::update_state (サーバー状態更新)
│   └── mmcss::enable_mmcss (Windowsオーディオ優先度設定)
└── client::*functions* (クライアントモードまたはライブラリ利用の場合)
    ├── client::ensure_server_ready (サーバーの準備を確認・起動)
    │   ├── client::server::check_server_status
    │   └── client::server::start_background_server
    ├── client::send_json (非インタラクティブJSON送信)
    │   └── ipc::protocol::send_command_to_server
    ├── client::start_interactive (インタラクティブモード開始)
    │   └── ipc::protocol::send_command_to_server
    ├── client::play_json_interactive (インタラクティブJSON送信)
    │   └── ipc::protocol::send_command_to_server
    ├── client::clear_schedule (スケジュールクリア)
    │   └── ipc::protocol::send_command_to_server
    ├── client::get_server_time (サーバー時刻取得)
    │   └── ipc::protocol::send_command_to_server
    ├── client::stop_playback (再生停止)
    │   └── ipc::protocol::send_command_to_server
    └── client::shutdown_server (サーバーシャットダウン)
        └── ipc::protocol::send_command_to_server

(オーディオ処理コア)
audio::stream::AudioStream::callback (オーディオスレッドでOSから定期的に呼び出される)
└── audio::generator::Ym2151Generator::generate_samples
    └── opm::OpmChip::generate_samples
        └── opm_ffi::ym2151_render (C言語のNuked-OPMエミュレータを呼び出し)

---
Generated at: 2025-11-22 07:02:49 JST
