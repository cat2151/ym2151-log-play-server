Last updated: 2025-12-27

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- クライアントはJSONデータをサーバーに送信し、演奏の開始、停止、切り替え、WAV出力などを制御します。
- リアルタイム音楽制御、音色エディタ、ライブパフォーマンスなど、連続した音声ストリームが必要な用途に最適です。

## 技術スタック
- フロントエンド: 該当なし (CLIベースのクライアントであり、主にライブラリとして他のアプリケーションに組み込まれるため)
- 音楽・オーディオ: YM2151 (OPM) (音源チップのエミュレーション), Nuked-OPM (YM2151エミュレーションのコアライブラリ), WAV出力 (演奏の記録機能)
- 開発ツール: Rust (主要なプログラミング言語), Cargo (Rustのビルドシステムおよびパッケージマネージャー), rust-script (Rustスクリプトの実行), Visual Studio Code (.vscode設定による開発環境の標準化)
- テスト: Cargo test (Rust標準のテストフレームワーク), Nextest (高速なテストランナーの利用を示唆)
- ビルドツール: Cargo (Rustプロジェクトのビルドと依存関係管理)
- 言語機能: Rust (高性能で安全なシステムプログラミングを提供し、リアルタイム処理を実現)
- 自動化・CI/CD: setup_ci_environment.sh (CI環境構築スクリプト)
- 開発標準: .editorconfig (コーディングスタイルの統一), .vscode/settings.json (開発環境の統一設定)

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
├── generated-docs/
│   └── development-status-generated-prompt.md
├── googled947dc864c270e07.html
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
- `Cargo.toml`: Rustプロジェクトの依存関係、メタデータ、ビルド設定を定義するマニフェストファイルです。
- `Cargo.lock`: `Cargo.toml`に基づいて解決された、プロジェクトが使用するすべての依存ライブラリの正確なバージョンとハッシュを記録するファイルです。
- `LICENSE`: プロジェクトのライセンス情報（MIT License）が記載されたファイルです。
- `README.ja.md`, `README.md`: プロジェクトの概要、使い方、開発状況などを説明するマークダウン形式のドキュメントです（日本語と英語）。
- `_config.yml`: Jekyllなどの静的サイトジェネレーターの設定ファイルである可能性があり、ドキュメント生成に関連する設定を含みます。
- `build.rs`: Rustプロジェクトのビルド時にカスタムロジック（例: C言語ライブラリのコンパイル）を実行するためのスクリプトです。
- `generated-docs/`: プロジェクトのドキュメントが自動生成されて格納されるディレクトリです。
- `googled947dc864c270e07.html`: Googleサイト認証などのために使用されるHTMLファイルです。
- `install-ym2151-tools.rs`: `rust-script`を用いてYM2151関連ツールを一括インストールするためのRustスクリプトです。
- `opm.c`, `opm.h`: YM2151音源チップのエミュレーションを行うC言語のソースファイルとヘッダファイルで、Nuked-OPMライブラリのコア部分を構成します。
- `output_ym2151.json`: YM2151レジスタイベントログのJSON形式のサンプルデータファイルです。
- `setup_ci_environment.sh`: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプトです。
- `src/main.rs`: プロジェクトのエントリポイントとなるメインファイルで、サーバーまたはクライアントモードの起動ロジックを含みます。
- `src/lib.rs`: このプロジェクトがライブラリとして他のRustクレートから利用される際に公開されるAPI定義を含むモジュールです。
- `src/audio/`: オーディオ関連の機能をまとめたモジュール群です。
  - `buffers.rs`: オーディオデータのバッファリングと管理を行います。
  - `commands.rs`: オーディオ再生を制御するためのコマンドの定義が含まれます。
  - `generator.rs`: YM2151エミュレーターからオーディオ信号を生成するロジックを担当します。
  - `player.rs`: オーディオ再生の中核となるプレイヤーロジックを実装します。
  - `scheduler.rs`: YM2151イベントを正確な時間に基づいてスケジューリングする機能を提供します。
  - `stream.rs`: オーディオストリームの管理とシステムへの出力ロジックを扱います。
- `src/audio_config.rs`: オーディオ再生に関する各種設定（サンプリングレート、バッファサイズなど）を定義します。
- `src/client/`: クライアント側のアプリケーションロジックをまとめたモジュール群です。
  - `config.rs`: クライアントの設定（サーバーアドレスなど）を定義します。
  - `core.rs`: クライアントの中核となる制御機能を提供します。
  - `interactive.rs`: インタラクティブモードでのサーバー操作（リアルタイム演奏制御）に関連するロジックです。
  - `json.rs`: YM2151レジスタイベントのJSONデータのパースや生成を行います。
  - `server.rs`: クライアントからサーバーの起動確認、インストール、シャットダウンといったライフサイクルを管理するロジックです。
- `src/debug_wav.rs`: デバッグ目的で生成されたオーディオデータをWAVファイルとして出力する機能を提供します。
- `src/demo_client_interactive.rs`: インタラクティブモードで動作するクライアントのデモンストレーションコードです。
- `src/demo_server_interactive.rs`: インタラクティブモードで動作するサーバーのデモンストレーションコードです。
- `src/demo_server_non_interactive.rs`: 非インタラクティブモードで動作するサーバーのデモンストレーションコードです。
- `src/events.rs`: YM2151レジスタへの書き込みイベントのデータ構造と、それらを処理するロジックを定義します。
- `src/ipc/`: プロセス間通信（IPC）の実装をまとめたモジュール群です。
  - `pipe_windows.rs`: Windowsの名前付きパイプを利用したIPCの具体的な実装を提供します。
  - `protocol.rs`: クライアントとサーバー間でやり取りされるコマンドやデータの通信プロトコルを定義します。
  - `src/ipc/windows/`: Windows固有のパイプ関連APIをラップするモジュールです。
- `src/logging.rs`: アプリケーション全体のログ出力機能を提供します。
- `src/mmcss.rs`: Windows Multimedia Class Scheduler Service (MMCSS) を利用し、オーディオ処理スレッドの優先度を高めるための機能です。
- `src/opm.rs`: C言語で書かれたNuked-OPMライブラリをRustから利用するためのラッパーモジュールです。
- `src/opm_ffi.rs`: RustとC言語間でデータや関数をやり取りするための外部関数インターフェース（FFI）定義です。
- `src/player.rs`: YM2151イベントの再生制御と全体的な状態管理を行う高レベルなプレイヤーロジックです。
- `src/resampler.rs`: オーディオデータのサンプリングレートを変換するリサンプリング機能を提供します。
- `src/scheduler.rs`: YM2151レジスタイベントを正確なタイミングで実行するためのスケジューリングロジックです。
- `src/server/`: サーバー側のアプリケーションロジックをまとめたモジュール群です。
  - `command_handler.rs`: クライアントから受信したコマンドを解釈し、適切な処理を実行します。
  - `connection.rs`: クライアントとの接続確立やデータ送受信の管理を行います。
  - `playback.rs`: サーバー上での実際のオーディオ再生状態を管理します。
  - `state.rs`: サーバーの内部的な状態（再生状況、設定など）を管理します。
- `src/wav_writer.rs`: 生成されたオーディオデータをWAV形式のファイルとして書き出す機能です。
- `tests/`: プロジェクトの様々なテストコードを格納するディレクトリです。

## 関数詳細説明
プロジェクト情報に具体的な関数の詳細（シグネチャ、引数、戻り値など）が提供されていないため、個別の関数の詳細説明はできません。ただし、上記のファイル詳細説明で各ファイルが担当する主要な機能について説明しています。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-12-27 07:02:18 JST
