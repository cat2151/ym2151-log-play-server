Last updated: 2025-11-20

# Project Overview

## プロジェクト概要
- YM2151 (OPM) 音源チップのレジスタイベントログを高精度でリアルタイム再生するシステムです。
- サーバーとクライアントで構成されており、Windows環境でのスムーズな音楽演奏と制御を実現します。
- 外部アプリケーションからの統合を容易にするライブラリ機能と、低遅延なインタラクティブ演奏モードを提供します。

## 技術スタック
- フロントエンド: Rust (クライアントライブラリとして他のRustアプリケーションから呼び出され、サーバーを制御します。)
- 音楽・オーディオ:
    - Nuked-OPM: YM2151 (OPM) FM音源チップのエミュレーションコア（C言語実装）。
    - rodio: クロスプラットフォーム対応のオーディオ再生ライブラリで、サーバーが音声を出力するために利用されます。
    - hound: WAV形式のオーディオファイルを書き出すためのライブラリ。verboseモードでのデバッグ出力に利用されます。
    - resampler: オーディオデータのリサンプリング処理を行うためのライブラリ。
- 開発ツール:
    - Rust: プロジェクトの主要なプログラミング言語。高い安全性とパフォーマンスを提供します。
    - Zig cc: Nuked-OPMのC言語コードをコンパイルするために使用されるCコンパイラ。
- テスト:
    - cargo test: Rustの標準テストフレームワークで、プロジェクトの様々な機能のテストに使用されます。
    - anyhow: エラーハンドリングを簡潔かつ強力に行うためのライブラリ。
- ビルドツール:
    - Cargo: Rustプロジェクトのビルド、依存関係管理、テスト実行などを行う標準ツール。
    - build.rs: C言語で書かれたNuked-OPMライブラリをRustプロジェクトに組み込むためのカスタムビルドスクリプト。
- 言語機能:
    - serde: Rustのデータ構造をシリアライズ（構造体をデータ形式に変換）およびデシリアライズ（データ形式を構造体に変換）するためのフレームワーク。
    - serde_json: JSONデータをSerdeを使って効率的に処理するためのライブラリ。YM2151イベントログの送受信に使用されます。
    - clap: コマンドライン引数を解析し、ユーザーフレンドリーなCLIインターフェースを構築するためのライブラリ。
- 自動化・CI/CD:
    - setup_ci_environment.sh: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。
- 開発標準:
    - .editorconfig: 複数のエディタやIDE間でコーディングスタイルを統一するための設定ファイル。
    - .gitignore: Gitが追跡しないファイルやディレクトリを指定するためのファイル。

## ファイル階層ツリー
```
.
├── .cargo/
│   └── config.toml
├── .editorconfig
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── INTERACTIVE_MODE_ANALYSIS.md
├── ISSUE_86_SUMMARY.md
├── LICENSE
├── README.ja.md
├── README.md
├── RUST_AGENTIC_CODING_BEST_PRACTICES.md
├── _config.yml
├── build.rs
├── examples/
│   ├── clear_schedule_demo.rs
│   ├── interactive_demo.rs
│   ├── play_json_interactive_demo.rs
│   ├── test_client_non_verbose.rs
│   ├── test_client_verbose.rs
│   ├── test_logging_non_verbose.rs
│   └── test_logging_verbose.rs
├── generated-docs/
│   └── development-status-generated-prompt.md
├── install-ym2151-tools.rs
├── issue-notes/
│   ├── 34.md
│   ├── ... (他のIssueノートファイル)
│   └── 86.md
├── opm.c
├── opm.h
├── setup_ci_environment.sh
├── src/
│   ├── audio.rs
│   ├── client.rs
│   ├── debug_wav.rs
│   ├── events.rs
│   ├── ipc/
│   │   ├── mod.rs
│   │   ├── pipe_windows.rs
│   │   └── protocol.rs
│   ├── lib.rs
│   ├── logging.rs
│   ├── main.rs
│   ├── opm.rs
│   ├── opm_ffi.rs
│   ├── player.rs
│   ├── resampler.rs
│   ├── scheduler.rs
│   ├── server.rs
│   ├── tests/
│   │   ├── audio_tests.rs
│   │   ├── client_tests.rs
│   │   ├── debug_wav_tests.rs
│   │   ├── events_tests.rs
│   │   ├── ipc_pipe_windows_tests.rs
│   │   ├── ipc_protocol_tests.rs
│   │   ├── logging_tests.rs
│   │   ├── mod.rs
│   │   ├── opm_ffi_tests.rs
│   │   ├── opm_tests.rs
│   │   ├── resampler_tests.rs
│   │   ├── scheduler_tests.rs
│   │   ├── server_tests.rs
│   │   └── wav_writer_tests.rs
│   └── wav_writer.rs
└── tests/
    ├── clear_schedule_test.rs
    ├── client_json_test.rs
    ├── client_test.rs
    ├── client_verbose_test.rs
    ├── debug_wav_test.rs
    ├── duration_test.rs
    ├── ensure_server_ready_test.rs
    ├── fixtures/
    │   ├── complex.json
    │   └── simple.json
    ├── integration_test.rs
    ├── interactive_mode_test.rs
    ├── ipc_pipe_test.rs
    ├── logging_test.rs
    ├── phase3_test.rs
    ├── phase4_test.rs
    ├── phase5_test.rs
    ├── phase6_cli_test.rs
    ├── play_json_interactive_test.rs
    ├── server_basic_test.rs
    ├── server_windows_fix_test.rs
    ├── tail_generation_test.rs
    └── test_utils.rs
```

## ファイル詳細説明
- **`.cargo/config.toml`**: Cargoのビルド設定やエイリアスなどを定義するファイル。
- **`.editorconfig`**: 異なるテキストエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイル。
- **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定するファイル。
- **`Cargo.lock`**: プロジェクトの依存関係ツリーと、使用されている各クレートの正確なバージョンを記録するファイル。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（名前、バージョン、作者など）と依存クレートを定義するファイル。
- **`INTERACTIVE_MODE_ANALYSIS.md`**: インタラクティブモードに関する分析や考察が記述されたドキュメント。
- **`ISSUE_86_SUMMARY.md`**: 特定の課題（Issue #86）に関する要約や経緯が記述されたドキュメント。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記載したファイル。
- **`README.ja.md`**: プロジェクトの日本語版概要と使い方を説明するファイル。
- **`README.md`**: プロジェクトの英語版概要と使い方を説明するファイル。
- **`RUST_AGENTIC_CODING_BEST_PRACTICES.md`**: Rustでのエージェントコーディングにおけるベストプラクティスをまとめたドキュメント。
- **`_config.yml`**: GitHub Pagesなどの設定ファイル（プロジェクト情報からは具体的な用途不明）。
- **`build.rs`**: Rustのビルドプロセス中に実行されるカスタムビルドスクリプト。Nuked-OPMのようなC言語ライブラリのコンパイルとリンクを設定するために使用されます。
- **`examples/`**: プロジェクトの様々な機能の具体的な使用例を示すRustソースファイル群。
    - `clear_schedule_demo.rs`: スケジュールクリア機能のデモ。
    - `interactive_demo.rs`: インタラクティブモードでのレジスタ書き込みのデモ。
    - `play_json_interactive_demo.rs`: JSONデータを使ってインタラクティブモードで再生するデモ。
    - `test_client_non_verbose.rs`: 非冗長モードのクライアント動作テスト。
    - `test_client_verbose.rs`: 冗長モードのクライアント動作テスト。
    - `test_logging_non_verbose.rs`: 非冗長ロギングのテスト。
    - `test_logging_verbose.rs`: 冗長ロギングのテスト。
- **`generated-docs/development-status-generated-prompt.md`**: プロジェクトの現在の開発状況に関する自動生成されたプロンプト情報。
- **`install-ym2151-tools.rs`**: 関連ツールを一括インストールするためのRustスクリプト。
- **`issue-notes/`**: 開発中に発生した特定の課題や問題に関するメモをまとめたディレクトリ。
- **`opm.c`**: Nuked-OPM（YM2151エミュレータ）のC言語による実装ソースファイル。
- **`opm.h`**: Nuked-OPMのC言語ヘッダーファイルで、RustからのFFI（Foreign Function Interface）で利用される関数や構造体の定義が含まれます。
- **`setup_ci_environment.sh`**: CI (継続的インテグレーション) 環境をセットアップするためのシェルスクリプト。
- **`src/`**: プロジェクトの主要なRustソースコードが配置されているディレクトリ。
    - **`audio.rs`**: オーディオデバイスの初期化、管理、および音声出力ストリームの処理ロジックをカプセル化しています。
    - **`client.rs`**: サーバーと通信し、演奏制御コマンド（JSONデータ送信、停止、シャットダウン、インタラクティブモード操作など）を実行するためのクライアント側ロジックを提供します。
    - **`debug_wav.rs`**: デバッグ目的で生成されたオーディオデータをWAVファイルとして出力する機能を提供します。
    - **`events.rs`**: YM2151音源のレジスタ設定イベントのデータ構造（時間、アドレス、データ）とそのパース（JSONからの変換など）を定義しています。
    - **`ipc/mod.rs`**: サーバーとクライアント間のプロセス間通信 (IPC) を管理するモジュールのエントリポイント。
    - **`ipc/pipe_windows.rs`**: Windowsの特定機能である名前付きパイプを使用して、サーバーとクライアント間の通信を実装します。
    - **`ipc/protocol.rs`**: サーバーとクライアント間で交換されるメッセージの形式（プロトコル）を定義します。
    - **`lib.rs`**: プロジェクトのライブラリクレートのエントリポイントで、外部アプリケーションから利用される公開API（clientモジュールなど）を定義します。
    - **`logging.rs`**: アプリケーション全体のロギング設定と機能を担当し、イベントやデバッグ情報の出力を行います。
    - **`main.rs`**: バイナリクレートのエントリポイント。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのどちらでアプリケーションを実行するかを決定します。
    - **`opm.rs`**: Nuked-OPMのC言語APIをRustから安全に呼び出すためのラッパーと高レベルなインターフェースを提供します。
    - **`opm_ffi.rs`**: C言語で書かれたNuked-OPMの関数とデータ構造に対するRustのFFI定義を含みます。
    - **`player.rs`**: YM2151イベント（レジスタ書き込み）をスケジュールし、実際に音源エミュレータに適用して音声を生成するコアロジックを担います。
    - **`resampler.rs`**: 生成されたYM2151のオーディオサンプルを指定された出力サンプリングレートに合わせてリサンプリングする機能を提供します。
    - **`scheduler.rs`**: 時間軸に基づいてYM2151イベントを正確にスケジューリングし、再生タイミングを管理します。
    - **`server.rs`**: サーバーアプリケーションの主要なロジックを実装。クライアントからのコマンドを受け取り、オーディオ再生を管理し、イベントスケジューリングを実行します。
    - **`src/tests/`**: `src` ディレクトリ内の各モジュールに対するユニットテストおよび統合テストコード。
    - **`wav_writer.rs`**: WAVファイルの基本的な書き込み機能を提供するユーティリティモジュール。
- **`tests/`**: プロジェクト全体の統合テストや、より複雑なシナリオのテストコードが配置されているディレクトリ。
    - `fixtures/`: テストに使用されるJSON形式のサンプルデータファイル。

## 関数詳細説明
- **`client::ensure_server_ready(server_name: &str) -> anyhow::Result<()>`**:
    - **役割**: YM2151再生サーバーが起動しているかを確認し、必要に応じて自動的にインストールおよびバックグラウンドで起動します。
    - **引数**: `server_name` (`&str`) - 起動を保証するサーバーアプリケーションの名前。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`。
- **`client::send_json(json_data: &str) -> anyhow::Result<()>`**:
    - **役割**: YM2151レジスタイベントを含むJSONデータをサーバーに送信し、その再生を指示します。
    - **引数**: `json_data` (`&str`) - 再生するYM2151イベントログを含むJSON文字列。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`。
- **`client::stop_playback() -> anyhow::Result<()>`**:
    - **役割**: 現在サーバーで再生中のYM2151音楽を停止させます。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`。
- **`client::shutdown_server() -> anyhow::Result<()>`**:
    - **役割**: 実行中のYM2151再生サーバープロセスを終了させます。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`。
- **`client::start_interactive() -> anyhow::Result<()>`**:
    - **役割**: サーバーをインタラクティブモードに移行させ、リアルタイムでのレジスタストリーミング再生を可能にします。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`。
- **`client::write_register(time: f64, addr: u8, data: u8) -> anyhow::Result<()>`**:
    - **役割**: インタラクティブモード中に、指定されたサーバー時刻にYM2151レジスタに値を書き込みます。
    - **引数**:
        - `time` (`f64`) - レジスタ書き込みをスケジュールするサーバー時刻（秒単位）。
        - `addr` (`u8`) - YM2151レジスタのアドレス。
        - `data` (`u8`) - レジスタに書き込むデータ。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`。
- **`client::get_server_time() -> anyhow::Result<f64>`**:
    - **役割**: サーバーの現在の内部時刻を秒単位で取得し、精密な同期スケジューリングを可能にします。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<f64>` - 処理が成功した場合はサーバー時刻（`f64`）、エラーが発生した場合は`Err`。
- **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**:
    - **役割**: インタラクティブモードが開始されている状態で、YM2151イベントを含むJSONデータを解析し、サーバーに連続的に送信します。
    - **引数**: `json_data` (`&str`) - 再生するYM2151イベントログを含むJSON文字列。
    - **戻り値**: `anyhow::Result<()>` - 処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`。
- **`server::run(config: ServerConfig) -> anyhow::Result<()>`**:
    - **役割**: サーバーのメインループを開始し、名前付きパイプを介してクライアントからのコマンドをリッスンし、オーディオ再生を管理します。
    - **引数**: `config` (`ServerConfig`) - サーバーの起動設定（verboseモード、リサンプリング品質など）。
    - **戻り値**: `anyhow::Result<()>` - サーバーが正常に終了した場合は`Ok(())`、実行中にエラーが発生した場合は`Err`。
- **`player::Player::play_event(event: Ym2151Event)`**:
    - **役割**: スケジュールされたYM2151イベント（レジスタ書き込み）を受け取り、内部のNuked-OPMエミュレータに適用し、サウンドを生成します。
    - **引数**: `event` (`Ym2151Event`) - 処理するYM2151レジスタイベント。
    - **戻り値**: なし。
- **`audio::AudioHandler::init() -> anyhow::Result<AudioHandler>`**:
    - **役割**: オーディオデバイスを初期化し、オーディオ再生ストリームを設定して開始します。
    - **引数**: なし。
    - **戻り値**: `anyhow::Result<AudioHandler>` - 初期化された`AudioHandler`インスタンス、またはエラー。
- **`resampler::Resampler::process_frame(&mut self, input_sample: f32) -> Option<f32>`**:
    - **役割**: 入力されたオーディオサンプルを指定された出力サンプリングレートに合わせてリサンプリングし、出力サンプルを生成します。
    - **引数**: `input_sample` (`f32`) - 入力オーディオサンプル。
    - **戻り値**: `Option<f32>` - リサンプリングされた出力サンプル（存在する場合）、または`None`。
- **`scheduler::Scheduler::add_event(&mut self, event: Ym2151Event)`**:
    - **役割**: YM2151イベントを内部のタイムラインに正確な時間で追加し、再生されるべき順序とタイミングを管理します。
    - **引数**: `event` (`Ym2151Event`) - スケジュールに追加するYM2151レジスタイベント。
    - **戻り値**: なし。

## 関数呼び出し階層ツリー
```
関数の呼び出し階層を自動で分析できませんでした。
しかし、主要な機能の流れは以下の通りです。

1.  **アプリケーション起動**:
    -   `main.rs` がコマンドライン引数 (`server` または `client`) を解析します。

2.  **サーバーモード (`ym2151-log-play-server server`)**:
    -   `server::run` がメインループを開始します。
    -   `audio::AudioHandler::init` でオーディオ出力が初期化されます。
    -   `ipc::pipe_windows` を通じてクライアントからのコマンドを待ち受けます。
    -   受信したコマンドに基づき、`player::Player` が `opm.rs` (Nuked-OPMラッパー) を介して `opm_ffi.rs` (CFFI) を呼び出し、YM2151エミュレータ (`opm.c`) を操作します。
    -   `scheduler::Scheduler` がYM2151イベントのタイミングを管理し、`player::Player::play_event` を呼び出します。
    -   YM2151エミュレータから生成された音声データは `resampler::Resampler` で処理され、`audio::AudioHandler` を通じて再生されます。
    -   `--verbose` オプションが指定された場合、`debug_wav.rs` と `wav_writer.rs` を使用してWAVファイルが出力されます。

3.  **クライアントモード (`ym2151-log-play-server client ...`) またはライブラリ利用**:
    -   `client::ensure_server_ready` がサーバーの起動と準備を保証します。
    -   `client::send_json` や `client::write_register` が `ipc::pipe_windows` を介してサーバーにコマンドやデータを送信します。
    -   `client::stop_playback` や `client::shutdown_server` がサーバーの制御コマンドを送信します。
    -   `client::start_interactive` や `client::play_json_interactive` はインタラクティブなイベント送信フローを開始します。

---
Generated at: 2025-11-20 07:02:31 JST
