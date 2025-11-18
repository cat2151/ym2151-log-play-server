Last updated: 2025-11-19

# Project Overview

## プロジェクト概要
- YM2151 (OPM) 音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- JSON音楽データ再生やインタラクティブなレジスタ操作、WAVファイル出力に対応しています。
- 外部プログラムからライブラリとして簡単にサーバーを制御・利用でき、スムーズな音楽再生体験を提供します。

## 技術スタック
- フロントエンド: 該当なし（CLIツールおよびライブラリとして機能）
- 音楽・オーディオ:
    - **YM2151 (OPM)**: Yamaha製FM音源チップ。本プロジェクトは、このチップのレジスタ操作イベントをシミュレートし、音源を生成します。
    - **Nuked-OPM**: YM2151音源チップのエミュレータライブラリ（C言語実装）。実際のサウンド生成の中核を担います。
    - **JSON**: 音楽データをレジスタイベントのログ形式で表現するために使用されます。
    - **WAV**: 再生されるオーディオをデバッグ目的でファイルに保存する際の出力形式です。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語。高い安全性とパフォーマンスを提供します。
    - **Cargo**: Rustの公式パッケージマネージャー兼ビルドシステム。依存関係管理、ビルド、テスト、ドキュメント生成などを担当します。
    - **anyhow**: Rustのエラーハンドリングライブラリ。柔軟なエラー報告を可能にします。
    - **clap**: Rustのコマンドライン引数パーサー。CLIインターフェースを構築するために使用されます。
- テスト:
    - **`cargo test`**: Rustの組み込みテストフレームワーク。単体テストや統合テストの実行に使用されます。
- ビルドツール:
    - **zig cc**: Cコンパイラとして使用され、Nuked-OPMのCコードをビルドプロセスに統合するために利用されます。
- 言語機能: 該当なし（特定の言語機能の明記はなし）
- 自動化・CI/CD:
    - **setup_ci_environment.sh**: CI環境をセットアップするためのシェルスクリプト。
- 開発標準:
    - **.editorconfig**: 異なるエディタやIDE間でコードスタイルを統一するための設定ファイル。

## ファイル階層ツリー
```
.cargo/
  config.toml
.editorconfig
.gitignore
Cargo.lock
Cargo.toml
LICENSE
README.ja.md
README.md
_config.yml
build.rs
examples/
  interactive_demo.rs
  test_client_non_verbose.rs
  test_client_verbose.rs
  test_logging_non_verbose.rs
  test_logging_verbose.rs
generated-docs/
  development-status-generated-prompt.md
issue-notes/
  34.md
  36.md
  38.md
  40.md
  42.md
  44.md
  46.md
  48.md
  50.md
  52.md
  54.md
  56.md
  58.md
  60.md
  62.md
  64.md
  66.md
  68.md
  70.md
  72.md
  74.md
opm.c
opm.h
setup_ci_environment.sh
src/
  audio.rs
  client.rs
  debug_wav.rs
  events.rs
  ipc/
    mod.rs
    pipe_windows.rs
    protocol.rs
  lib.rs
  logging.rs
  main.rs
  opm.rs
  opm_ffi.rs
  player.rs
  resampler.rs
  scheduler.rs
  server.rs
  wav_writer.rs
tests/
  client_json_test.rs
  client_test.rs
  client_verbose_test.rs
  debug_wav_test.rs
  duration_test.rs
  ensure_server_ready_test.rs
  fixtures/
    complex.json
    simple.json
  integration_test.rs
  interactive_mode_test.rs
  ipc_pipe_test.rs
  logging_test.rs
  phase3_test.rs
  phase4_test.rs
  phase5_test.rs
  phase6_cli_test.rs
  server_basic_test.rs
  server_windows_fix_test.rs
  tail_generation_test.rs
  test_utils.rs
```

## ファイル詳細説明
-   `.cargo/config.toml`: Cargoのビルド設定やエイリアスなどを定義するファイル。
-   `.editorconfig`: コードのインデント、文字コード、改行コードなどのエディタ設定を統一するためのファイル。
-   `.gitignore`: Gitがバージョン管理の対象から外すファイルやディレクトリを指定するファイル。
-   `Cargo.lock`: `Cargo.toml`に記載された依存関係が実際に解決されたバージョンを正確に記録するファイル。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存クレートなどが記述されます。
-   `LICENSE`: プロジェクトのライセンス情報（MIT License）を記載したファイル。
-   `README.ja.md`, `README.md`: プロジェクトの概要、使い方、ビルド方法などを説明するドキュメント（日本語と英語）。
-   `_config.yml`: Jekyllなどの静的サイトジェネレーターの設定ファイルである可能性が高い。
-   `build.rs`: Rustプロジェクトのビルドスクリプト。Nuked-OPMのCコード (`opm.c`) をコンパイルし、Rustコードから利用できるようにバインドする処理を記述しています。
-   `examples/`: プロジェクトの機能の使用例を示す実行可能なコードを含むディレクトリ。
    -   `interactive_demo.rs`: インタラクティブモードでのYM2151レジスタリアルタイム書き込みデモ。
    -   `test_client_non_verbose.rs`, `test_client_verbose.rs`: クライアントの通常モードおよび詳細ログモードでの動作例。
    -   `test_logging_non_verbose.rs`, `test_logging_verbose.rs`: ロギング機能の動作確認用例。
-   `generated-docs/development-status-generated-prompt.md`: 自動生成されたドキュメントや開発状況に関するプロンプトの記録。
-   `issue-notes/`: 開発中の課題や設計上の決定に関するメモを記録するディレクトリ。
-   `opm.c`, `opm.h`: Nuked-OPMエミュレータのC言語ソースコードとヘッダファイル。YM2151音源のレジスタ操作をシミュレートし、オーディオデータを生成するコアロジックを含みます。
-   `setup_ci_environment.sh`: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。
-   `src/`: プロジェクトの主要なRustソースコードが格納されているディレクトリ。
    -   `audio.rs`: オーディオ出力デバイス（例: WASAPI on Windows）とのインターフェースを提供し、生成されたサウンドデータを再生するロジックを扱います。
    -   `client.rs`: サーバーに接続し、JSON音楽データの送信、再生停止、サーバーシャットダウン、インタラクティブモードでのレジスタ書き込みなどのコマンドを送信するクライアント側のAPIを提供します。
    -   `debug_wav.rs`: デバッグ目的で生成されたオーディオデータをWAVファイルとして保存するための機能を提供します。
    -   `events.rs`: YM2151のレジスタ操作イベントのデータ構造（アドレス、値、タイミング）と、それらをJSON形式でシリアライズ/デシリアライズする定義を含みます。
    -   `ipc/`: プロセス間通信（IPC）に関連するモジュールをまとめたディレクトリ。
        -   `mod.rs`: `ipc`モジュールを公開し、サブモジュールをまとめるファイル。
        -   `pipe_windows.rs`: Windowsの「名前付きパイプ」を使用してサーバーとクライアント間で通信を行う具体的な実装を含みます。
        -   `protocol.rs`: サーバーとクライアント間で交換されるコマンドやメッセージのデータ構造（プロトコル）を定義します。
    -   `lib.rs`: クレートのライブラリとしてのエントリポイント。他のプロジェクトから`ym2151-log-play-server`クレートを利用する際に公開されるAPIを定義します。
    -   `logging.rs`: アプリケーション全体のログ出力機能（例: ターミナル出力やファイルログ）を管理するモジュール。
    -   `main.rs`: プロジェクトの実行エントリポイント。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのどちらを起動するかを決定します。
    -   `opm.rs`: `opm.c`で定義されたNuked-OPMのC関数へのForeign Function Interface (FFI) バインディングを安全なRustインターフェースとして提供します。
    -   `opm_ffi.rs`: `opm.c`のFFIバインディングに関するより低レベルな定義やヘルパー関数が含まれる可能性があります。
    -   `player.rs`: YM2151のレジスタイベントログを再生するための主要なロジックを管理します。イベントのパース、スケジューリング、音源チップへの書き込みなどを行います。
    -   `resampler.rs`: 生成されたオーディオデータを異なるサンプリングレートに変換（リサンプリング）するための機能を提供します。
    -   `scheduler.rs`: YM2151レジスタイベントを時間ベースでスケジュールし、正確なタイミングで音源チップに書き込むためのメカニズムを管理します。
    -   `server.rs`: クライアントからのコマンドを受け取り、YM2151再生の開始/停止、インタラクティブモードの管理、WAVファイル出力などを実行するサーバー側のロジックを実装します。
    -   `wav_writer.rs`: オーディオデータをWAVファイル形式で書き出すためのユーティリティ機能。
-   `tests/`: プロジェクトのテストコードが格納されているディレクトリ。
    -   `fixtures/`: テストに使用されるデータファイル（例: JSON形式の音楽ログ）。
        -   `complex.json`: 複雑なテストケース用のJSONデータ。
        -   `simple.json`: シンプルなテストケース用のJSONデータ。
    -   `client_json_test.rs`, `client_test.rs`, `client_verbose_test.rs`: クライアント機能の様々な側面をテストするコード。
    -   `debug_wav_test.rs`: デバッグWAV出力機能のテスト。
    -   `duration_test.rs`: 再生時間の正確性を検証するテスト。
    -   `ensure_server_ready_test.rs`: サーバーの準備が自動的に行われる機能のテスト。
    -   `integration_test.rs`: サーバーとクライアント間の連携を含む、より大規模な統合テスト。
    -   `interactive_mode_test.rs`: インタラクティブモードでのリアルタイムレジスタ書き込み機能のテスト。
    -   `ipc_pipe_test.rs`: 名前付きパイプによるプロセス間通信機能のテスト。
    -   `logging_test.rs`: ロギング機能のテスト。
    -   `phaseX_test.rs`: 開発フェーズごとのテスト、または特定の機能セットのテスト。
    -   `server_basic_test.rs`, `server_windows_fix_test.rs`: サーバーの基本的な動作やWindows固有の修正に関するテスト。
    -   `tail_generation_test.rs`: 演奏終了後の無音期間生成に関するテスト。
    -   `test_utils.rs`: テストヘルパー関数や共通の設定が含まれるユーティリティファイル。

## 関数詳細説明
-   `client::ensure_server_ready(server_name: &str) -> anyhow::Result<()>`:
    -   **役割**: YM2151再生サーバーが利用可能な状態であることを確認します。必要に応じて、サーバーアプリケーションをPATHから探し、Cargo経由でインストールし、バックグラウンドで起動し、コマンドを受け付けられる状態になるまで待機します。
    -   **引数**: `server_name` - 起動するサーバーアプリケーションの名前（例: "ym2151-log-play-server"）。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `client::send_json(json_data: &str) -> anyhow::Result<()>`:
    -   **役割**: JSON形式のYM2151レジスタイベントログデータをサーバーに送信し、再生を開始するよう指示します。
    -   **引数**: `json_data` - 再生するJSON形式の音楽データ文字列。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `client::stop_playback() -> anyhow::Result<()>`:
    -   **役割**: 現在サーバーで再生中のYM2151音楽を停止するようサーバーに指示します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `client::shutdown_server() -> anyhow::Result<()>`:
    -   **役割**: 実行中のYM2151再生サーバープロセスを安全にシャットダウンするよう指示します。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `client::start_interactive() -> anyhow::Result<()>`:
    -   **役割**: サーバーをインタラクティブモードに移行させます。このモードでは、リアルタイムで個別のレジスタ書き込みコマンドを送信できます。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `client::write_register(delay_ms: u32, addr: u8, data: u8) -> anyhow::Result<()>`:
    -   **役割**: インタラクティブモードで、指定されたミリ秒後のタイミングでYM2151の特定のレジスタに値を書き込むようサーバーに指示します。
    -   **引数**: `delay_ms` - 現在からレジスタ書き込みまでの遅延時間（ミリ秒）。`addr` - 書き込むYM2151レジスタのアドレス。`data` - 書き込むレジスタ値。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `client::stop_interactive() -> anyhow::Result<()>`:
    -   **役割**: サーバーのインタラクティブモードを終了させます。
    -   **引数**: なし。
    -   **戻り値**: 成功した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `server::run_server(verbose: bool) -> anyhow::Result<()>`:
    -   **役割**: YM2151再生サーバーのメインループを開始します。クライアントからのIPCコマンド（再生、停止、シャットダウン、インタラクティブ操作など）を待ち受け、処理します。
    -   **引数**: `verbose` - 詳細ログ出力およびWAVファイル出力を行うかどうかを指定するフラグ。
    -   **戻り値**: サーバーが正常に終了した場合は`Ok(())`、エラーが発生した場合は`anyhow::Result`。
-   `player::play_json(json_log: &str, wav_writer: Option<WavWriter>, ...)`:
    -   **役割**: 提供されたJSON形式のYM2151レジスタイベントログを解析し、オーディオ再生を開始します。必要に応じてWAVファイルへの出力も行います。
    -   **引数**: `json_log` - JSON形式のイベントログ文字列。`wav_writer` - WAVファイル出力用のオプションのライター。
    -   **戻り値**: 適切な再生制御構造体。
-   `opm::init()`, `opm::update(buffer: &mut [f32])`:
    -   **役割**: `opm::init()`はNuked-OPMエミュレータを初期化し、`opm::update()`は次のオーディオフレームを生成して指定されたバッファに書き込みます。
    -   **引数**: `update`はオーディオデータを書き込む浮動小数点数配列のバッファ。
    -   **戻り値**: `init`はなし、`update`もなし（バッファが直接変更される）。
-   `audio::start_audio_thread(sample_rate: u32, ...)`:
    -   **役割**: バックグラウンドでオーディオ再生スレッドを起動し、指定されたサンプリングレートでサウンド出力を管理します。`opm::update`を呼び出してオーディオフレームを継続的に生成します。
    -   **引数**: `sample_rate` - 使用するオーディオサンプリングレート。
    -   **戻り値**: オーディオスレッドの制御ハンドル。

## 関数呼び出し階層ツリー
```
main (ym2151-log-play-server CLI)
├─── server::run_server (サーバーモード起動時)
│    ├─── ipc::pipe_windows::create_server_pipe
│    ├─── loop (クライアントコマンド待ち受け)
│    │    ├─── ipc::pipe_windows::read_command
│    │    ├─── player::handle_command (受信したコマンドを処理)
│    │    │    ├─── Command::PlayJson
│    │    │    │    └─── player::play_json
│    │    │    │         ├─── events::parse_json_log
│    │    │    │         ├─── scheduler::start (イベントスケジューリング開始)
│    │    │    │         │    └─── audio::start_audio_thread (オーディオ出力スレッド起動)
│    │    │    │         │         └─── (オーディオコールバック内で) opm::update (繰り返し呼び出し、音源生成)
│    │    │    │         │              └─── opm::write_register (スケジューラーから指示されたレジスタ書き込み)
│    │    │    │         ├─── debug_wav::WavWriter::new (verboseモードの場合)
│    │    │    │         └─── resampler::Resampler::new
│    │    │    ├─── Command::Stop
│    │    │    │    └─── player::stop_playback
│    │    │    ├─── Command::StartInteractive
│    │    │    │    └─── player::start_interactive
│    │    │    ├─── Command::WriteRegister
│    │    │    │    └─── player::write_register
│    │    │    ├─── Command::StopInteractive
│    │    │    │    └─── player::stop_interactive
│    │    │    └─── Command::Shutdown (ループ終了)
│    └─── ipc::pipe_windows::close_server_pipe
│
└─── client::main_client (クライアントモード起動時、またはライブラリ利用時)
     ├─── client::ensure_server_ready
     │    ├─── (内部的に) cargo install (必要に応じて)
     │    ├─── (内部的に) Command::spawn (サーバープロセス起動)
     │    └─── ipc::pipe_windows::wait_for_server
     ├─── client::send_json
     │    └─── ipc::pipe_windows::send_command(Command::PlayJson)
     ├─── client::stop_playback
     │    └─── ipc::pipe_windows::send_command(Command::Stop)
     ├─── client::start_interactive
     │    └─── ipc::pipe_windows::send_command(Command::StartInteractive)
     ├─── client::write_register
     │    └─── ipc::pipe_windows::send_command(Command::WriteRegister)
     ├─── client::stop_interactive
     │    └─── ipc::pipe_windows::send_command(Command::StopInteractive)
     └─── client::shutdown_server
          └─── ipc::pipe_windows::send_command(Command::Shutdown)

---
Generated at: 2025-11-19 07:02:20 JST
