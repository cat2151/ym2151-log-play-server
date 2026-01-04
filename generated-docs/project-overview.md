Last updated: 2026-01-05

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するサーバー・クライアントシステムです。
- Rustで開発されており、JSON音楽データを効率的に処理し、低遅延での演奏やWAVファイル出力に対応しています。
- 名前付きパイプによるプロセス間通信を利用し、プログラムからのシームレスな制御やインタラクティブな演奏切り替えを可能にします。

## 技術スタック
- フロントエンド: プロジェクト情報に直接記載なし。（クライアントSDKとしての利用が主であり、グラフィカルなUIを持つフロントエンドは本プロジェクトのスコープ外です。）
- 音楽・オーディオ:
    - **YM2151 (OPM)**: プロジェクトが対象とするヤマハ製FM音源チップ。
    - **Nuked-OPM**: YM2151音源のレジスタ操作をエミュレートし、音響を生成するためのC言語ライブラリ。FFIを介してRustから利用。
    - **WAVファイル出力**: 再生中の音源をWAV形式で保存する機能。
- 開発ツール:
    - **Rust**: プロジェクトの主要開発言語（バージョン1.70以降）。安全性、並行性、パフォーマンスに優れる。
    - **Cargo**: Rustの標準ビルドシステムおよびパッケージマネージャー。依存関係管理、ビルド、テスト実行を担う。
    - **rust-script**: Rustスクリプトを簡単に実行するためのツール。
    - **Visual Studio Code (VS Code)**: `.vscode/`ディレクトリに開発環境設定が格納されており、開発に利用されている。
    - **CodeQL**: 静的解析ツール。関連設定ファイルが存在する。
- テスト:
    - **Cargo Test**: Rust標準のテストフレームワーク。単体テスト、結合テストの実行に利用。
    - **Nextest**: 高速なRustテストランナー。
- ビルドツール:
    - **Cargo**: Rustプロジェクトのビルド、コンパイル、依存関係解決を行う。
    - **build.rs**: カスタムビルドロジック（C言語ライブラリのコンパイルなど）を記述するためのRustスクリプト。
- 言語機能:
    - **Rust 1.70以降**: 最新のRust言語機能とパフォーマンス最適化を利用。
    - **FFI (Foreign Function Interface)**: C言語で書かれたNuked-OPMライブラリをRustから安全に呼び出すためのメカニズム。
- 自動化・CI/CD:
    - **setup_ci_environment.sh**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプト。
    - **GitHub Linux Runner**: 開発プロセスにおいてCI/CD環境として利用が言及されている。
- 開発標準:
    - **EditorConfig (.editorconfig)**: 異なるエディタやIDE間でコードの書式設定（インデントスタイル、文字コードなど）を統一するための設定。
    - **Git Ignore (.gitignore)**: Gitリポジトリでバージョン管理対象外とするファイルやディレクトリを指定。

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
├── issue-notes/ (※内容は開発者向けのため説明は省略)
│   ├── 100.md
│   ├── 101.md
│   ├── 102.md
│   ├── 110.md
│   ├── 111.md
│   ├── 112.md
│   ├── 113.md
│   ├── 116.md
│   ├── 117.md
│   ├── 118.md
│   ├── 119.md
│   ├── 120.md
│   ├── 121.md
│   ├── 122.md
│   ├── 123.md
│   ├── 124.md
│   ├── 128.md
│   ├── 130.md
│   ├── 132.md
│   ├── 134.md
│   ├── 138.md
│   ├── 141.md
│   ├── 143.md
│   ├── 146.md
│   ├── 148.md
│   ├── 150.md
│   ├── 152.md
│   ├── 154.md
│   ├── 156.md
│   ├── 158.md
│   ├── 161.md
│   ├── 165.md
│   ├── 167.md
│   ├── 169.md
│   ├── 173.md
│   ├── 178.md
│   ├── 96.md
│   ├── 97.md
│   ├── 98.md
│   └── 99.md
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
    │   ├── client_tests.rs
    │   ├── command_handler_tests.rs
    │   ├── debug_wav_tests.rs
    │   ├── demo_server_interactive_tests.rs
    │   ├── demo_server_non_interactive_tests.rs
    │   ├── events_tests.rs
    │   ├── ipc_pipe_windows_tests.rs
    │   ├── ipc_protocol_tests.rs
    │   ├── logging_tests.rs
    │   ├── mmcss_tests.rs
    │   ├── mod.rs
    │   ├── opm_ffi_tests.rs
    │   ├── opm_tests.rs
    │   ├── play_json_interactive_tests.rs
    │   ├── player_tests.rs
    │   ├── resampler_tests.rs
    │   ├── scheduler_tests.rs
    │   ├── server_tests.rs
    │   └── wav_writer_tests.rs
    └── wav_writer.rs
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
- **`.config/nextest.toml`**: 高速なRustテストランナーであるNextestの設定ファイルです。テストの実行方法やレポート出力に関する詳細を定義します。
- **`.editorconfig`**: 異なるテキストエディタやIDE間でコードの書式設定（インデントスタイル、文字コードなど）を統一するための設定ファイルです。プロジェクト全体のコードの一貫性を保ちます。
- **`.gitignore`**: Gitバージョン管理システムに対して、追跡すべきではないファイルやディレクトリ（ビルド生成物、一時ファイル、個人設定など）を指定するファイルです。
- **`.vscode/extensions.json`**: Visual Studio Codeを使用する開発者向けに推奨される拡張機能をリストアップします。
- **`.vscode/settings.json`**: Visual Studio Codeのワークスペース固有の設定を定義します。コード補完、フォーマット、リンティングなどの動作をカスタマイズします。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係が実際に解決されたバージョンとそのハッシュ値を記録するファイルです。これにより、異なる環境でもビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのビルドマニフェストファイルです。プロジェクト名、バージョン、著者、依存クレート、ビルド設定など、プロジェクトに関するメタデータを定義します。
- **`LICENSE`**: プロジェクトのライセンス（MIT License）が記載されています。ソフトウェアの利用、配布、変更に関する条件を定めます。
- **`README.ja.md`**: プロジェクトの日本語版の概要、主な機能、使い方、開発状況、ビルド要件などが詳細に記述されています。
- **`README.md`**: プロジェクトの英語版の概要、主な機能、使い方、開発状況、ビルド要件などが詳細に記述されています。
- **`_config.yml`**: Jekyllなどの静的サイトジェネレータで使用される設定ファイルである可能性が高いです。ドキュメント生成やウェブサイト構築に関連する設定を定義します。
- **`build.rs`**: Rustプロジェクトのビルド時にCargoによって実行されるカスタムビルドスクリプトです。C言語のソースファイル（`opm.c`など）のコンパイルやFFIバインディングの生成など、特定のビルド前処理を自動化するために使用されます。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発状況に関するドキュメントです。
- **`googled947dc864c270e07.html`**: Google Search Consoleなどのウェブマスターツールによるサイト所有権確認のために配置されるHTMLファイルです。
- **`install-ym2151-tools.rs`**: `rust-script`を使用して実行される、開発に必要な関連ツールを一括でインストールするためのスクリプトです。
- **`opm.c`**: YM2151音源エミュレータ「Nuked-OPM」のC言語ソースファイルです。FM音源のレジスタ操作をシミュレートし、デジタルオーディオ信号を生成する中核部分を担います。
- **`opm.h`**: `opm.c`に対応するC言語ヘッダファイルです。Nuked-OPMエミュレータの関数やデータ構造の宣言が含まれており、RustからFFIを介して利用されます。
- **`output_ym2151.json`**: YM2151レジスタイベントログのサンプルデータ、またはプログラムが生成するJSON形式の出力例です。プロジェクトの機能テストやデモに使用されます。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプトです。ビルドツールのインストールや環境変数の設定などを行います。
- **`src/audio/buffers.rs`**: オーディオデータのバッファリングを管理するためのロジックが含まれています。音源生成や再生に必要なデータの一時的な保持に使用されます。
- **`src/audio/commands.rs`**: オーディオ再生システム内で使用されるコマンド（例: 再生開始、停止、データ送信など）の定義を扱います。
- **`src/audio/generator.rs`**: YM2151音源のエミュレーションから実際のオーディオサンプルを生成するロジックを担当します。
- **`src/audio/mod.rs`**: `audio`モジュールを定義するファイルで、他の`audio`モジュール内のサブモジュールをまとめて公開します。
- **`src/audio/player.rs`**: 音声再生のコアロジックを実装しています。オーディオストリームへのデータ供給や再生状態の管理を行います。
- **`src/audio/scheduler.rs`**: YM2151レジスタイベントを時間軸に沿って正確にスケジューリングするロジックが含まれています。
- **`src/audio/stream.rs`**: システムのオーディオデバイスとのインターフェースや、オーディオストリームの管理に関するコードです。
- **`src/audio_config.rs`**: オーディオ再生に関するグローバルな設定（サンプリングレート、バッファサイズなど）を管理します。
- **`src/client/config.rs`**: クライアントアプリケーションの挙動を制御する設定項目を定義します。
- **`src/client/core.rs`**: クライアント機能の基盤となるロジックが含まれており、サーバーとの基本的な通信処理を担当します。
- **`src/client/interactive.rs`**: インタラクティブモードクライアントに特化したロジックを提供します。リアルタイムでの動的な音楽制御を可能にします。
- **`src/client/json.rs`**: クライアントがサーバーに送信するJSONデータの生成や、サーバーから受信するJSONデータの解析を扱います。
- **`src/client/mod.rs`**: `client`モジュールのルートファイルであり、外部アプリケーションがYM2151再生サーバーと連携するための公開APIを提供します。
- **`src/client/server.rs`**: クライアント側からサーバープロセス（インストール、起動、状態確認、シャットダウンなど）を管理するためのユーティリティ関数を提供します。
- **`src/debug_wav.rs`**: デバッグ目的で、生成されたオーディオデータをWAVファイルとして出力する機能を提供します。音源の正確性確認に役立ちます。
- **`src/demo_client_interactive.rs`**: インタラクティブクライアントのデモンストレーションコードが含まれています。
- **`src/demo_server_interactive.rs`**: インタラクティブサーバーのデモンストレーションコードが含まれています。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブサーバーのデモンストレーションコードが含まれています。
- **`src/events.rs`**: YM2151レジスタへの書き込みイベントを表現するデータ構造（時間、アドレス、データなど）と、それらを扱うロジックを定義します。
- **`src/ipc/mod.rs`**: `ipc`（プロセス間通信）モジュールのルートファイルです。クライアントとサーバー間の通信を抽象化します。
- **`src/ipc/pipe_windows.rs`**: Windowsオペレーティングシステムに特化した、名前付きパイプを使用したプロセス間通信の実装です。
- **`src/ipc/protocol.rs`**: クライアントとサーバー間でやり取りされるメッセージの形式や、通信規約を定義します。
- **`src/ipc/windows/mod.rs`**: Windows固有のIPC実装をグループ化するモジュールです。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを作成・初期化するための機能を提供します。
- **`src/ipc/windows/pipe_handle.rs`**: Windows APIのパイプハンドルをラップし、安全な操作を可能にします。
- **`src/ipc/windows/pipe_reader.rs`**: 名前付きパイプからデータを受信する（読み取る）ためのロジックを実装します。
- **`src/ipc/windows/pipe_writer.rs`**: 名前付きパイプにデータを送信する（書き込む）ためのロジックを実装します。
- **`src/ipc/windows/test_logging.rs`**: Windowsパイプのテスト時に使用される、ログ出力ユーティリティです。
- **`src/lib.rs`**: このプロジェクトがクレート（ライブラリ）として提供する機能のエントリポイントです。クライアントAPIなどの公開モジュールを定義します。
- **`src/logging.rs`**: アプリケーション全体で使用されるロギングシステム（イベントやエラーの記録）の実装です。
- **`src/main.rs`**: 実行可能ファイルのエントリポイントです。コマンドライン引数を解析し、サーバーモードまたはクライアントモードのどちらで起動するかを決定します。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用して、オーディオ処理の優先度を高めるための設定ロジックが含まれています。リアルタイム音声再生の安定性向上に寄与します。
- **`src/opm.rs`**: Nuked-OPMエミュレータ（C言語）のRustラッパーを提供します。YM2151のレジスタ操作をRustから抽象化して行えるようにします。
- **`src/opm_ffi.rs`**: C言語のNuked-OPMライブラリに対するRawなFFI（Foreign Function Interface）バインディングを定義します。`opm.c`および`opm.h`との直接的なインターフェースです。
- **`src/player.rs`**: 高レベルなオーディオプレイヤーロジックを扱います。オーディオストリームの管理、イベントスケジューリング、音源生成を連携させます。
- **`src/resampler.rs`**: 生成されたオーディオサンプルのサンプリングレートを変換するためのアルゴリズムと実装を提供します。異なるオーディオデバイスの要件に対応できます。
- **`src/scheduler.rs`**: YM2151のレジスタイベントを正確な時刻に実行するためのスケジューリング機能を提供します。
- **`src/server/command_handler.rs`**: サーバーがクライアントから受信したコマンドを解析し、適切な処理を実行するためのロジックを定義します。
- **`src/server/connection.rs`**: サーバー側でクライアントからのIPC接続（名前付きパイプ）を確立し、管理するロジックです。
- **`src/server/mod.rs`**: `server`モジュールを定義するファイルで、他の`server`モジュール内のサブモジュールをまとめて公開します。
- **`src/server/playback.rs`**: サーバー側でYM2151レジスタイベントの再生を制御し、オーディオストリームに送り出すロジックを実装します。
- **`src/server/state.rs`**: サーバーの現在の状態（再生中、停止中、インタラクティブモードなど）を管理し、共有データへの安全なアクセスを提供します。
- **`src/tests/`**: `src`ディレクトリ内の各モジュールに対する単体テストやモジュールテストが含まれています。
- **`src/wav_writer.rs`**: 生成されたオーディオデータをWAVファイル形式でディスクに書き込むための機能を提供します。
- **`tests/`**: プロジェクト全体の統合テストやエンドツーエンドテストが含まれるディレクトリです。クライアントとサーバー間の連携など、より高レベルな機能の検証を行います。
- **`tests/audio/`**: オーディオ関連機能のテストをまとめたサブディレクトリです。
- **`tests/fixtures/`**: テストで使用するサンプルデータ（例: JSONファイル）が格納されています。

## 関数詳細説明
- **`ym2151_log_play_server::client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**
    - **役割**: YM2151再生サーバーが起動し、コマンドを受け付けられる状態にあることを保証します。必要に応じてサーバーのインストール、バックグラウンド起動、待機処理を自動で行います。
    - **引数**: `app_name` - サーバーを識別するためのアプリケーション名。
    - **戻り値**: 処理が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    - **機能**: ライブラリ利用者がサーバーのライフサイクルを意識せず、すぐに機能を利用開始できるようにします。
- **`ym2151_log_play_server::client::send_json(json_data: &str) -> anyhow::Result<()>`**
    - **役割**: JSON形式のYM2151レジスタイベントログデータをサーバーに送信し、再生を開始します（非インタラクティブモード）。既存の再生は停止され、新しいデータに切り替わります。
    - **引数**: `json_data` - 再生したいYM2151イベントを含むJSON文字列。
    - **戻り値**: 処理が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    - **機能**: 単発の楽曲再生や、WAV保存など、演奏の連続性よりも切り替えの明確さを重視するシナリオに適しています。
- **`ym2151_log_play_server::client::stop_playback() -> anyhow::Result<()>`**
    - **役割**: 現在サーバーで再生中のYM2151イベントログの再生を停止し、音源を無音状態にします。
    - **引数**: なし。
    - **戻り値**: 処理が成功した場合は `Ok(())`、エラーが発生した場合は `anywher::Result` を返します。
    - **機能**: 演奏の一時停止や、異なる楽曲に切り替える前処理として利用されます。
- **`ym2151_log_play_server::client::shutdown_server() -> anyhow::Result<()>`**
    - **役割**: バックグラウンドで動作しているYM2151再生サーバープロセスに、安全なシャットダウンを指示します。
    - **引数**: なし。
    - **戻り値**: 処理が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    - **機能**: アプリケーション終了時などにサーバーリソースを適切に解放するために使用されます。
- **`ym2151_log_play_server::client::start_interactive() -> anyhow::Result<()>`**
    - **役割**: サーバーをインタラクティブモードに移行させ、連続した音声ストリームの生成を開始します。これにより、クライアントは音響ギャップなしでイベントを動的にスケジューリングできるようになります。
    - **引数**: なし。
    - **戻り値**: 処理が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    - **機能**: リアルタイム音色エディタやライブパフォーマンスなど、連続的かつ動的な音響制御が求められる場面で利用されます。
- **`ym2151_log_play_server::client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**
    - **役割**: インタラクティブモードで、JSON形式のYM2151レジスタイベントデータをサーバーに送信し、現在の音声ストリームにシームレスにスケジュールします。イベントの時間情報は自動的に秒単位に変換されます。
    - **引数**: `json_data` - スケジュールしたいYM2151イベントを含むJSON文字列。
    - **戻り値**: 処理が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    - **機能**: リアルタイムでのフレーズ追加や音色変更など、音響の中断なしに動的に音楽を変化させます。
- **`ym2151_log_play_server::client::clear_schedule() -> anyhow::Result<()>`**
    - **役割**: インタラクティブモードで、サーバーにスケジュールされている未再生のYM2151イベントをすべてキャンセルします。
    - **引数**: なし。
    - **戻り値**: 処理が成功した場合は `Ok(())`、エラーが発生した場合は `anyhow::Result` を返します。
    - **機能**: 楽曲の途中で異なるフレーズに切り替える際、既存のキューをクリアして新しいイベントを即座に反映させるために使用されます。
- **`ym2151_log_play_server::client::get_server_time() -> anyhow::Result<f64>`**
    - **役割**: サーバー内部で現在進行中の再生時刻を、秒単位の浮動小数点数（`f64`）で取得します。
    - **引数**: なし。
    - **戻り値**: 現在のサーバー時刻を表す `f64`、またはエラー発生時は `anyhow::Result` を返します。
    - **機能**: クライアントとサーバー間のタイミングを正確に同期させ、厳密なリアルタイム制御を実現するために利用されます（Web Audio APIの `currentTime` プロパティに相当します）。

## 関数呼び出し階層ツリー
```
main (ym2151-log-play-server CLI entry)
├── server (コマンドライン引数 "server" で起動)
│   └── server::connection::accept_connections()
│       └── server::command_handler::handle_command()
│           ├── audio::player::start_playback()
│           ├── audio::player::stop_playback()
│           ├── server::state::set_interactive_mode()
│           ├── server::state::get_server_time()
│           └── server::state::shutdown()
└── client (コマンドライン引数 "client" で起動, またはライブラリ利用)
    ├── client::core::send_command_to_server() (内部IPC通信層を介してサーバーへコマンド送信)
    │   ├── ipc::pipe_windows::send_data() (JSONデータ送信)
    │   └── ipc::pipe_windows::send_command() (制御コマンド送信)
    └── client::* (公開API関数)
        ├── client::ensure_server_ready()
        │   ├── client::server::is_server_running()
        │   ├── client::server::install_server()
        │   └── client::server::start_server_background()
        ├── client::send_json()
        ├── client::stop_playback()
        ├── client::shutdown_server()
        ├── client::start_interactive()
        ├── client::play_json_interactive()
        ├── client::clear_schedule()
        └── client::get_server_time()

---
Generated at: 2026-01-05 07:03:10 JST
