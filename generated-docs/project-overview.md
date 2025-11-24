Last updated: 2025-11-25

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- JSON形式の音楽データを扱い、Windowsプラットフォーム上で高精度なオーディオ再生とWAVファイル出力に対応します。
- 名前付きパイプによるサーバー・クライアント通信で、外部アプリケーションからの動的な演奏制御や切り替えが可能です。

## 技術スタック
- フロントエンド: このプロジェクト自体は直接的なGUIを持たず、クライアントライブラリやCLIとして機能します。UIは外部の利用プロジェクトによって提供されます。
- 音楽・オーディオ: Nuked-OPM (YM2151エミュレーション用Cライブラリ。高精度なFM音源チップYM2151のレジスタ操作をシミュレートし、オーディオデータを生成します。)
- 開発ツール: zig cc (Cコードコンパイル用。Nuked-OPMライブラリのビルドに使用されます。)
- テスト: Rustの標準テストフレームワーク (cargo testコマンドで実行されるユニットテストおよび統合テスト)。
- ビルドツール: Cargo (Rustのパッケージマネージャー兼ビルドシステム。プロジェクトの依存関係管理、ビルド、テスト、実行を担います。)
- 言語機能: Rust 1.70以降 (メモリ安全性、並行性、パフォーマンスに優れたシステムプログラミング言語。)
- 自動化・CI/CD: setup_ci_environment.sh (CI環境設定スクリプト。特定のCI/CDサービスに依存せず、環境構築の自動化に使用されます。)
- 開発標準: EditorConfig (.editorconfigファイルにより、異なるエディタやIDE間でコードのスタイルとフォーマットの一貫性を保ちます。)

## ファイル階層ツリー
```
📁 ym2151-log-play-server/
  📁 .cargo/
    📄 config.toml
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
  📄 install-ym2151-tools.rs
  📁 issue-notes/
    📖 100.md
    📖 101.md
    📖 102.md
    📖 110.md
    📖 111.md
    📖 112.md
    📖 113.md
    📖 116.md
    📖 117.md
    📖 118.md
    📖 119.md
    📖 120.md
    📖 121.md
    📖 122.md
    📖 96.md
    📖 97.md
    📖 98.md
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
- **`.cargo/config.toml`**: Cargoのカスタム設定ファイル。ビルド時の挙動やリンカ設定などを指定することがあります。
- **`.editorconfig`**: コードエディタ間でインデントスタイル、文字コードなどのコーディング規約を統一するための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定するファイルです。
- **`.vscode/extensions.json`**: Visual Studio Codeのワークスペース推奨拡張機能のリスト。チーム開発で開発環境の統一に役立ちます。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定ファイル。特定のプロジェクトでのエディタの挙動やLintルールなどを定義します。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンとその依存ツリーを記録し、プロジェクトの再現可能なビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、著者、ライブラリの依存関係、ビルド設定などが記述されます。
- **`LICENSE`**: プロジェクトの利用条件を規定するライセンス情報ファイル（このプロジェクトではMIT Licenseが指定されています）。
- **`README.ja.md`**: プロジェクトの日本語での概要、機能、使い方、ビルド方法などを説明する主要なドキュメントファイルです。
- **`README.md`**: プロジェクトの英語での概要、機能、使い方、ビルド方法などを説明する主要なドキュメントファイルです。
- **`_config.yml`**: 通常、静的サイトジェネレーター（例: Jekyll）の設定ファイルとして使用されます。ドキュメントサイト生成などの用途かもしれません。
- **`build.rs`**: Rustプロジェクトのカスタムビルドスクリプト。主にC言語ライブラリ（Nuked-OPM）のコンパイルやリンクなどのビルド時処理を自動化するために使用されます。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成されたドキュメントや開発状況に関する情報が格納されるディレクトリ。
- **`install-ym2151-tools.rs`**: YM2151関連のツール群を一括でインストールするためのRustスクリプトファイル。
- **`issue-notes/`**: 開発中に発生した課題や検討事項を記録するためのノートを格納するディレクトリです。
- **`opm.c`**: YM2151（OPM）音源チップのエミュレーションを行うNuked-OPMライブラリのC言語ソースコードファイルです。
- **`opm.h`**: `opm.c`に対応するC言語ヘッダーファイル。Nuked-OPMライブラリの公開関数とデータ構造を定義します。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルデータが格納されたJSONファイル。テストやデモで使用されます。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするために使用されるシェルスクリプトです。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングと管理に関連するロジックを定義します。
- **`src/audio/commands.rs`**: オーディオ再生システム内で使用されるコマンド（例: 再生開始、停止）の定義を含みます。
- **`src/audio/generator.rs`**: YM2151エミュレータからの出力に基づいて、実際のオーディオサンプルを生成する機能を提供します。
- **`src/audio/mod.rs`**: `src/audio`モジュールのルートファイル。オーディオ関連のサブモジュールを公開します。
- **`src/audio/player.rs`**: オーディオ再生のコアロジックを実装し、オーディオストリームの開始・停止・データ供給を制御します。
- **`src/audio/scheduler.rs`**: YM2151レジスタイベントを正確なタイミングでエミュレータに供給するためのスケジューリング機構を実装します。
- **`src/audio/stream.rs`**: オーディオデバイスとのインターフェースを提供し、オーディオデータのリアルタイム出力ストリームを管理します。
- **`src/audio_config.rs`**: オーディオのサンプリングレート、チャンネル数、バッファサイズなど、オーディオ再生に関する設定を定義します。
- **`src/client/config.rs`**: クライアントアプリケーションの動作に必要な設定値（例: 名前付きパイプ名）を定義します。
- **`src/client/core.rs`**: クライアント機能の基盤となるロジック。サーバーとの通信やコマンド送信の抽象化を提供します。
- **`src/client/interactive.rs`**: 連続したオーディオストリームを維持しつつ、リアルタイムに演奏を制御する「インタラクティブモード」のクライアントサイド機能です。
- **`src/client/json.rs`**: JSON形式で記述されたYM2151レジスタイベントデータを解析し、プロジェクト内部のデータ構造に変換する機能を提供します。
- **`src/client/mod.rs`**: `src/client`モジュールのルートファイル。クライアント関連のサブモジュールを公開します。
- **`src/client/server.rs`**: クライアントからサーバーの起動、停止、状態確認といったライフサイクル管理を行うロジックを実装します。
- **`src/debug_wav.rs`**: デバッグ目的で、生成されたオーディオデータをWAVファイルとして出力する機能を提供します。
- **`src/demo_client_interactive.rs`**: インタラクティブモードクライアントの使用例を示すデモンストレーションコードです。
- **`src/demo_server_interactive.rs`**: インタラクティブモードサーバーの使用例を示すデモンストレーションコードです。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードサーバーの使用例を示すデモンストレーションコードです。
- **`src/events.rs`**: YM2151レジスタへの書き込みイベントのデータ構造（時間、アドレス、データなど）と、それらを処理するロジックを定義します。
- **`src/ipc/mod.rs`**: `src/ipc`モジュールのルートファイル。プロセス間通信（IPC）に関連するサブモジュールを公開します。
- **`src/ipc/pipe_windows.rs`**: Windowsの名前付きパイプを使用したプロセス間通信の実装を提供します。
- **`src/ipc/protocol.rs`**: クライアントとサーバー間でやり取りされるコマンドやメッセージのプロトコル（形式）を定義します。
- **`src/ipc/windows/mod.rs`**: Windows固有のIPC実装に関連するサブモジュールのルートファイルです。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを作成・初期化するためのファクトリパターンを実装します。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsの名前付きパイプのハンドルを安全に管理するためのラッパー構造体です。
- **`src/ipc/windows/pipe_reader.rs`**: Windowsの名前付きパイプからデータを非同期的に読み取る機能を提供します。
- **`src/ipc/windows/pipe_writer.rs`**: Windowsの名前付きパイプにデータを非同期的に書き込む機能を提供します。
- **`src/ipc/windows/test_logging.rs`**: Windows IPCテスト中に発生するイベントやデータを記録するためのロギングユーティリティです。
- **`src/lib.rs`**: このプロジェクトが提供するライブラリクレートのメインエントリーポイント。公開APIやモジュールの宣言を行います。
- **`src/logging.rs`**: アプリケーション全体のログ出力設定（ロガーの初期化、出力レベルなど）を定義します。
- **`src/main.rs`**: 実行可能バイナリのメインエントリーポイント。コマンドライン引数に応じてサーバーまたはクライアントとして動作を開始します。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用するための機能を提供し、オーディオ処理の優先度を向上させます。
- **`src/opm.rs`**: Nuked-OPM（C言語ライブラリ）をRustから利用するためのForeign Function Interface (FFI) ラッパーと、YM2151エミュレータのRust向けインターフェースを提供します。
- **`src/opm_ffi.rs`**: C言語で書かれたNuked-OPMライブラリの関数やデータ型をRustから安全に呼び出すためのFFIバインディングを定義します。
- **`src/player.rs`**: オーディオ再生の全体的なフローを管理する上位モジュール。YM2151エミュレータ、スケジューラ、オーディオストリームを連携させます。
- **`src/resampler.rs`**: オーディオデータのサンプリングレートを変換する機能（例: 高品質補間、低品質線形補間）を提供します。
- **`src/scheduler.rs`**: YM2151レジスタイベントをタイムスタンプに基づいて順序付けし、正確なタイミングで実行するためのスケジューリングロジックです。
- **`src/server/command_handler.rs`**: クライアントから受信したコマンドを解釈し、サーバーの内部状態や再生処理に反映させるロジックを実装します。
- **`src/server/connection.rs`**: クライアントとのプロセス間通信（IPC）接続の確立、維持、切断を管理します。
- **`src/server/mod.rs`**: `src/server`モジュールのルートファイル。サーバー関連のサブモジュールを公開します。
- **`src/server/playback.rs`**: サーバーサイドでのYM2151音楽データ再生処理の中核を担い、オーディオ生成と出力ストリームを管理します。
- **`src/server/state.rs`**: サーバーの現在の動作状態（再生モード、スケジューラの状態など）を保持し、管理する構造体です。
- **`src/wav_writer.rs`**: 生成されたオーディオデータを標準的なWAVファイル形式でディスクに書き込む機能を提供します。
- **`tests/audio/audio_playback_test.rs`**: オーディオ再生機能の動作を確認するテストスイート。
- **`tests/audio/audio_sound_test.rs`**: 生成される音響の品質や特性を検証するテストスイート。
- **`tests/audio/mod.rs`**: `tests/audio`モジュールのルートファイル。オーディオ関連のテストをまとめる。
- **`tests/clear_schedule_test.rs`**: インタラクティブモードにおけるスケジュールクリア機能の正確性を検証するテスト。
- **`tests/cli_integration_test.rs`**: コマンドラインインターフェース（CLI）の統合的な動作を検証するテスト。
- **`tests/client_json_test.rs`**: クライアントがJSON形式の音楽データをサーバーに送信し、正しく処理されるかを検証するテスト。
- **`tests/client_test.rs`**: クライアントの基本的な機能が正しく動作するかを検証するテストスイート。
- **`tests/client_verbose_test.rs`**: クライアントがverboseモードで詳細なログを出力するかどうかを検証するテスト。
- **`tests/debug_wav_test.rs`**: デバッグ目的でのWAVファイル出力機能が正しく動作するかを検証するテスト。
- **`tests/duration_test.rs`**: 音楽イベントの持続時間計算や時間管理が正確であるかを検証するテスト。
- **`tests/ensure_server_ready_test.rs`**: クライアントライブラリの`ensure_server_ready`関数がサーバーの準備と起動を正しく行うかを検証するテスト。
- **`tests/events_processing_test.rs`**: YM2151レジスタイベントの解析と処理ロジックの正確性を検証するテスト。
- **`tests/feature_demonstration_test.rs`**: 特定のプロジェクト機能が期待通りに動作することをデモンストレーション形式で示すテスト。
- **`tests/fixtures/complex.json`**: 複雑なYM2151レジスタイベントシーケンスを含むテスト用JSONフィクスチャ。
- **`tests/fixtures/simple.json`**: 単純なYM2151レジスタイベントシーケンスを含むテスト用JSONフィクスチャ。
- **`tests/integration_test.rs`**: プロジェクト全体としての各コンポーネントが連携して正しく動作するかを検証する統合テスト。
- **`tests/interactive/mod.rs`**: `tests/interactive`モジュールのルートファイル。インタラクティブモード関連のテストをまとめる。
- **`tests/interactive/mode_test.rs`**: インタラクティブモードの開始・終了やモード間の切り替えが正しく行われるかを検証するテスト。
- **`tests/interactive/play_json_test.rs`**: インタラクティブモードでJSONデータがスムーズに再生されるかを検証するテスト。
- **`tests/interactive/shared_mutex.rs`**: テスト内で共有されるミューテックスの正しい使用法と安全性に関するテストユーティリティ。
- **`tests/interactive/step_by_step_test.rs`**: インタラクティブモードの段階的な動作やイベント処理を詳細に検証するテスト。
- **`tests/interactive_tests.rs`**: インタラクティブモード全般の機能と挙動を検証するテストスイート。
- **`tests/ipc_pipe_test.rs`**: プロセス間通信（名前付きパイプ）の信頼性とパフォーマンスを検証するテスト。
- **`tests/logging_test.rs`**: プロジェクトのロギング機能が正しく設定され、期待通りのログを出力するかを検証するテスト。
- **`tests/server_basic_test.rs`**: サーバーの基本的な起動、待機、シャットダウンといった動作を検証するテスト。
- **`tests/server_integration_test.rs`**: サーバーとクライアントが連携した状態での、より複雑なシナリオを検証する統合テスト。
- **`tests/tail_generation_test.rs`**: 音源のリリースや減衰（テール）が正しく生成・処理されるかを検証するテスト。
- **`tests/test_util_server_mutex.rs`**: サーバーテストで使用される、特定の共有状態を保護するためのユーティリティミューテックス。

## 関数詳細説明
プロジェクト情報からは関数の具体的な役割、引数、戻り値、機能に関する詳細な情報は提供されていません。そのため、個々の関数の詳細な説明は生成できません。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした。

---
Generated at: 2025-11-25 07:02:37 JST
