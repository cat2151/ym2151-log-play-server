Last updated: 2025-11-21

# Project Overview

## プロジェクト概要
- YM2151 (OPM) 音源チップのレジスタイベントログをリアルタイムで再生するシステムです。
- サーバー・クライアント方式を採用し、他のアプリケーションからの柔軟な音楽再生と制御を可能にします。
- Windows専用として設計されており、インタラクティブな音楽制作やMMLプレイヤーへの組み込みを想定しています。

## 技術スタック
- フロントエンド: プログラムからの制御を可能にするクライアントライブラリとコマンドラインインターフェース（CLI）を提供します。
- 音楽・オーディオ: YM2151 (OPM) 音源チップのエミュレーション（Nuked-OPM使用）、リアルタイムオーディオ再生、JSON形式の音楽データ解析、WAVファイル出力、リサンプリング機能。
- 開発ツール: Rust言語、C言語部分をコンパイルするためのZig ccコンパイラ。
- テスト: Rustの標準テストフレームワークと、機能ごとの豊富な統合テスト。
- ビルドツール: Rustのパッケージマネージャー兼ビルドシステムであるCargo。
- 言語機能: Rustのモダンなシステムプログラミング機能（所有権、借用、非同期処理など）、C言語での低レベル処理。
- 自動化・CI/CD: （特に言及なし、開発者向け情報のため除外）
- 開発標準: `.editorconfig`によるプロジェクト全体でのコードスタイル統一。

## ファイル階層ツリー
```
📁 .cargo/
  📄 config.toml
📄 .editorconfig
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📄 _config.yml
📄 build.rs
📁 generated-docs/
📄 install-ym2151-tools.rs
📄 opm.c
📄 opm.h
📊 output_ym2151_f64seconds.json
📁 src/
  📄 audio.rs
  📄 audio_config.rs
  📁 client/
    📄 config.rs
    📄 core.rs
    📄 interactive.rs
    📄 json.rs
    📄 mod.rs
    📄 server.rs
  📄 debug_wav.rs
  📄 demo.rs
  📄 demo_server_interactive.rs
  📄 demo_server_non_interactive.rs
  📄 events.rs
  📁 ipc/
    📄 mod.rs
    📄 pipe_windows.rs
    📄 protocol.rs
  📄 lib.rs
  📄 logging.rs
  📄 main.rs
  📄 mmcss.rs
  📄 opm.rs
  📄 opm_ffi.rs
  📄 player.rs
  📄 resampler.rs
  📄 scheduler.rs
  📄 server.rs
  📁 tests/
    📄 audio_tests.rs
    📄 client_tests.rs
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
    📄 row_by_row_test.rs
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
-   `.cargo/config.toml`: RustのビルドツールCargoの設定ファイル。特にC/C++コンパイラとしてZig ccを使用する設定などが含まれます。
-   `.editorconfig`: さまざまなエディタで一貫したコーディングスタイルを維持するための設定ファイルです。
-   `.gitignore`: Gitによるバージョン管理から除外するファイルやディレクトリのパターンを定義します。
-   `Cargo.lock`: プロジェクトの依存関係の正確なバージョンを記録し、再現性のあるビルドを保証します。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存クレート、ビルド設定などを記述します。
-   `LICENSE`: プロジェクトがMITライセンスで提供されることを示します。
-   `README.ja.md`: プロジェクトの日本語による概要、機能、使用方法などを示した説明書です。
-   `README.md`: プロジェクトの英語による概要、機能、使用方法などを示した説明書です。
-   `_config.yml`: Jekyllなどの静的サイトジェネレータで使用される設定ファイル。
-   `build.rs`: Rustプロジェクトのビルドスクリプト。ビルド時に追加で実行されるロジック（例: C言語ライブラリのコンパイル）を記述します。
-   `generated-docs/`: 自動生成されたドキュメントを格納するためのディレクトリです。
-   `install-ym2151-tools.rs`: YM2151関連ツールを一括でインストールするためのRustスクリプトです。
-   `opm.c`: YM2151音源チップのエミュレーションを行うNuked-OPMのC言語ソースコードです。
-   `opm.h`: `opm.c`に対応するC言語ヘッダファイルで、外部からエミュレータ機能を利用するためのインターフェースを定義します。
-   `output_ym2151_f64seconds.json`: デモンストレーションやテストで使用されるYM2151レジスタイベントデータを含むJSONファイルです。
-   `src/audio.rs`: プロジェクトのオーディオ出力に関する主要なロジックを実装しています。オーディオストリームの開始・停止などを担当します。
-   `src/audio_config.rs`: オーディオのサンプリングレート、チャンネル数、バッファサイズなど、オーディオ関連の設定を定義します。
-   `src/client/config.rs`: クライアントアプリケーションの設定と、サーバーとの通信に必要なパラメータを管理します。
-   `src/client/core.rs`: クライアントのコア機能（サーバーへのコマンド送信、サーバーの起動確認など）を実装します。
-   `src/client/interactive.rs`: インタラクティブモードでのクライアント操作ロジックを扱います。リアルタイムでのイベントスケジューリングなどを提供します。
-   `src/client/json.rs`: JSON形式の音楽データをパースし、YM2151イベントに変換する処理を実装します。
-   `src/client/mod.rs`: `client`モジュールのルートファイルであり、クライアント関連のサブモジュールをまとめています。
-   `src/client/server.rs`: クライアント側からサーバーのライフサイクル（起動、停止、シャットダウン）を制御するためのロジックを提供します。
-   `src/debug_wav.rs`: デバッグ目的で、生成されたオーディオデータをWAVファイルとしてディスクに書き出す機能を提供します。
-   `src/demo.rs`: デモンストレーション用の共通ユーティリティ関数や設定を定義します。
-   `src/demo_server_interactive.rs`: インタラクティブモードのサーバー機能のデモンストレーションロジックを含みます。
-   `src/demo_server_non_interactive.rs`: 非インタラクティブモードのサーバー機能のデモンストレーションロジックを含みます。
-   `src/events.rs`: YM2151レジスタに対する書き込みイベントのデータ構造（時間、アドレス、データなど）と、それらを扱うロジックを定義します。
-   `src/ipc/mod.rs`: プロセス間通信（IPC）モジュールのルートファイルであり、IPC関連のサブモジュールをまとめています。
-   `src/ipc/pipe_windows.rs`: WindowsのNamed Pipe（名前付きパイプ）を利用したIPCの実装を提供します。
-   `src/ipc/protocol.rs`: サーバーとクライアント間でやり取りされるメッセージのフォーマットとプロトコルを定義します。
-   `src/lib.rs`: このプロジェクトをライブラリとして使用する際に公開されるAPIを定義するルートファイルです。
-   `src/logging.rs`: アプリケーションのロギング機能（ログ出力の初期化、フォーマットなど）を設定・実装します。
-   `src/main.rs`: 実行可能ファイルのメインエントリーポイント。コマンドライン引数を解析し、サーバーまたはクライアントのどちらかのモードを起動します。
-   `src/mmcss.rs`: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用するためのラッパー。オーディオ再生の優先度をシステムレベルで高めます。
-   `src/opm.rs`: C言語で実装されたNuked-OPMエミュレータをRustから呼び出すためのFFI（Foreign Function Interface）ラッパーです。
-   `src/opm_ffi.rs`: `opm.rs`よりも低レベルなC言語とのFFIバインディングを直接定義します。
-   `src/player.rs`: YM2151レジスタイベントを解釈し、Nuked-OPMエミュレータに供給して最終的なオーディオサンプルを生成する中心的なプレイヤーロジックを実装しています。
-   `src/resampler.rs`: オーディオデータのサンプリングレートを変換（リサンプリング）する機能を提供します。
-   `src/scheduler.rs`: YM2151イベントを時間に基づいて正確に再生するためのスケジューリングロジックを実装します。
-   `src/server.rs`: サーバーアプリケーションの主要なロジックを実装しています。クライアントからのコマンドを受け取り、オーディオ再生を管理します。
-   `src/tests/`: Rustの単体テストを格納するためのディレクトリです。
-   `src/wav_writer.rs`: オーディオデータを標準的なWAVファイル形式で書き出す機能を提供します。
-   `tests/`: 統合テストやエンドツーエンドテストを格納するためのディレクトリです。

## 関数詳細説明
-   `main::main()`:
    -   **役割**: プログラムの実行を開始するエントリーポイント。
    -   **引数**: なし（コマンドライン引数は内部でパース）。
    -   **戻り値**: `anyhow::Result<()>` (実行結果を示す)。
    -   **機能**: コマンドライン引数を解析し、`server`モードまたは`client`モードのどちらかの機能を起動します。
-   `client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`:
    -   **役割**: サーバーが動作していることを確認し、必要であれば自動的にサーバーをインストール・起動します。
    -   **引数**: `app_name: &str` - クライアントアプリケーションの名前。
    -   **戻り値**: `anyhow::Result<()>` (サーバーの準備が完了したかどうか)。
    -   **機能**: サーバープロセスの存在チェック、PATHからのアプリケーション検索、Cargo経由でのインストール、バックグラウンドでのサーバー起動、サーバーがコマンドを受け付ける状態になるまでの待機を行います。
-   `client::send_json(json_data: &str) -> anyhow::Result<()>`:
    -   **役割**: JSON形式のYM2151レジスタイベントデータをサーバーに送信し、再生を開始します（非インタラクティブモード）。
    -   **引数**: `json_data: &str` - 再生するJSON形式の音楽データ。
    -   **戻り値**: `anyhow::Result<()>` (データ送信の成否)。
    -   **機能**: サーバーとIPCで通信し、受け取ったJSONデータを再生キューに追加するよう指示します。
-   `client::start_interactive() -> anyhow::Result<()>`:
    -   **役割**: サーバーにインタラクティブモードの開始を指示します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` (モード切り替えの成否)。
    -   **機能**: 連続的な音声ストリームを維持し、リアルタイムでのイベントスケジューリングを可能にするためのモードをサーバーに設定します。
-   `client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`:
    -   **役割**: インタラクティブモードでJSON形式のYM2151レジスタイベントデータをサーバーに送信し、現在の音声ストリームにスケジュールします。
    -   **引数**: `json_data: &str` - 再生するJSON形式の音楽データ。
    -   **戻り値**: `anyhow::Result<()>` (データ送信の成否)。
    -   **機能**: サンプル単位のイベント時間をf64秒単位に自動変換し、サーバーの現在の再生に影響を与えることなくイベントを動的に追加します。
-   `client::clear_schedule() -> anyhow::Result<()>`:
    -   **役割**: インタラクティブモードで、サーバーにスケジュールされている未来のイベントをすべてキャンセルします。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` (スケジュールのクリアの成否)。
    -   **機能**: 現在再生中のイベントには影響を与えず、今後のイベントキューをクリアすることで、急な曲の切り替えや停止を実現します。
-   `client::get_server_time() -> anyhow::Result<f64>`:
    -   **役割**: サーバーの現在の再生時刻を秒単位で取得します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<f64>` (現在のサーバー時刻、またはエラー)。
    -   **機能**: Web Audioの`currentTime`プロパティと同等の機能を提供し、正確なタイミング制御を可能にします。
-   `client::stop_playback() -> anyhow::Result<()>`:
    -   **役割**: サーバーに現在のYM2151音楽の再生を停止するよう指示します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` (停止指示の成否)。
    -   **機能**: 再生中の音源をミュートまたは停止し、無音状態にします。
-   `client::shutdown_server() -> anyhow::Result<()>`:
    -   **役割**: サーバープロセスを安全にシャットダウンするよう指示します。
    -   **引数**: なし。
    -   **戻り値**: `anyhow::Result<()>` (シャットダウン指示の成否)。
    -   **機能**: サーバーとのIPC接続を閉じ、サーバープロセスを終了させます。
-   `server::run_server(args: ServerArgs) -> anyhow::Result<()>`:
    -   **役割**: サーバーモードのメインループを実行します。
    -   **引数**: `args: ServerArgs` - サーバー起動時のオプション（例: `verbose`、`low-quality-resampling`）。
    -   **戻り値**: `anyhow::Result<()>` (サーバー実行の成否)。
    -   **機能**: クライアントからのコマンドを継続的にリッスンし、受信したデータに基づいてオーディオ再生を管理・制御します。
-   `audio::init_audio_stream(...) -> Result<cpal::Stream, ...>`:
    -   **役割**: オーディオ出力ストリームを初期化し、再生準備を整えます。
    -   **引数**: (オーディオデバイス、サンプリングレート、バッファサイズなどの設定)。
    -   **戻り値**: `Result<cpal::Stream, ...>` (初期化されたオーディオストリーム、またはエラー)。
    -   **機能**: CPALライブラリを使用してシステムオーディオデバイスと接続し、低遅延でのオーディオ出力ストリームを設定します。
-   `player::Player::new(...) -> Player`:
    -   **役割**: YM2151音源エミュレータを初期化し、YM2151レジスタイベントを処理するためのプレイヤーインスタンスを生成します。
    -   **引数**: (Nuked-OPMエミュレータの状態、オーディオ設定など)。
    -   **戻り値**: `Player` (初期化されたプレイヤーオブジェクト)。
    -   **機能**: 内部でNuked-OPMのエミュレータインスタンスを管理し、イベントに応じた音源の生成ロジックを提供します。
-   `scheduler::Scheduler::add_events(...)`:
    -   **役割**: 指定されたYM2151レジスタイベントを再生キューに追加し、適切なタイミングで処理されるようスケジュールします。
    -   **引数**: (YM2151イベントのリスト、開始時刻など)。
    -   **戻り値**: なし。
    -   **機能**: イベントをタイムスタンプに基づいてソート・管理し、オーディオフレーム生成時に適切なイベントがYM2151エミュレータに適用されるようにします。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-11-21 07:02:30 JST
