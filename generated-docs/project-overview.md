Last updated: 2025-12-26

# Project Overview

## プロジェクト概要
- YM2151（OPM）音源チップのレジスタイベントログをリアルタイムで再生するRust製のサーバー・クライアントシステムです。
- サーバーはバックグラウンドで常駐し、クライアントからの命令でJSON形式の音楽データを演奏・制御します。
- 非インタラクティブモードでの単発再生と、無音ギャップなしで動的な音響制御が可能なインタラクティブモードを提供します。

## 技術スタック
- フロントエンド: このプロジェクトには専用のGUIフロントエンドは含まれず、主にコマンドライン操作またはライブラリとして他のアプリケーションに組み込むことを想定しています。
- 音楽・オーディオ: YM2151 (OPM) 音源のエミュレーション (`Nuked-OPM` ライブラリを使用)、WAVファイル出力、リアルタイムオーディオ処理、リサンプリング技術。
- 開発ツール: `Rust` (バージョン1.70以降), `cargo` (ビルド、テスト、依存関係管理), `rust-script` (開発用スクリプト実行)。
- テスト: `cargo test`コマンドで実行される単体テストおよび統合テスト。
- ビルドツール: `cargo` (Rustプロジェクトの標準ビルドシステム), `build.rs` (C言語ライブラリとの連携など、カスタムビルドロジック用)。
- 言語機能: `Rust` (安全性とパフォーマンスを重視したシステムプログラミング言語)。
- 自動化・CI/CD: `setup_ci_environment.sh` (CI環境のセットアップスクリプト), `GitHub Copilot Coding Agent` (開発プロセスでTDDを支援)。
- 開発標準: `.editorconfig` (コードスタイルとフォーマットの統一)。

## ファイル階層ツリー
```
📁 .config/
  📄 nextest.toml
📄 .editorconfig
📄 .gitignore
📁 .vscode/
  📄 extensions.json
  📄 settings.json
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
    📄 player_tests.rs
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
- **`.config/nextest.toml`**: `nextest`という高速なテストランナーの設定を記述するファイルです。
- **`.editorconfig`**: さまざまなエディタやIDE間で、インデントスタイル、文字コードなどの基本的なコーディングスタイルを統一するための設定ファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリのパターンを指定するファイルです。ビルド生成物や一時ファイルなどが含まれます。
- **`.vscode/extensions.json`**: Visual Studio Codeでこのプロジェクトを開発する際に推奨される拡張機能のリストです。
- **`.vscode/settings.json`**: Visual Studio Codeでこのプロジェクトを開いた際のワークスペース固有の設定を記述するファイルです。
- **`Cargo.lock`**: RustのパッケージマネージャーであるCargoが、プロジェクトのビルド時に実際に使用した依存関係の正確なバージョンを記録するファイルです。
- **`Cargo.toml`**: Rustプロジェクトの「マニフェスト」ファイルで、プロジェクト名、バージョン、著者、および依存関係（クレート）を定義します。
- **`LICENSE`**: このプロジェクトがMITライセンスで提供されていることを示します。
- **`README.ja.md`**: プロジェクトの日本語による概要、使い方、機能などを説明する主要なドキュメントファイルです。
- **`README.md`**: プロジェクトの英語による概要、使い方、機能などを説明する主要なドキュメントファイルです。
- **`_config.yml`**: 通常はJekyllなどの静的サイトジェネレーターで使用される設定ファイルですが、このプロジェクトではドキュメント生成やその他の設定に利用されている可能性があります。
- **`build.rs`**: Rustプロジェクトのビルドプロセス中に実行されるカスタムビルドスクリプトです。C言語で書かれた`Nuked-OPM`ライブラリのバインディング生成などに利用されます。
- **`generated-docs/development-status-generated-prompt.md`**: 自動生成された開発状況に関するドキュメントが格納されるファイルです。
- **`install-ym2151-tools.rs`**: `rust-script`を用いて、YM2151関連の開発ツールを一括でインストールするためのスクリプトです。
- **`opm.c`, `opm.h`**: YM2151音源チップをエミュレートする`Nuked-OPM`ライブラリのC言語ソースファイルとヘッダーファイルです。実際の音源生成ロジックを含みます。
- **`output_ym2151.json`**: YM2151のレジスタ操作イベントをJSON形式で記述したサンプルデータファイルです。再生テストやデモに使用されます。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプトです。
- **`src/audio/buffers.rs`**: オーディオデータの一時的な保管や処理に用いられるバッファリングメカニズムを定義します。
- **`src/audio/commands.rs`**: オーディオ再生システム内部でやり取りされるコマンド（再生開始、停止など）を定義します。
- **`src/audio/generator.rs`**: YM2151エミュレータから出力されるレジスタデータに基づき、実際の音声波形データを生成するロジックを含みます。
- **`src/audio/mod.rs`**: `audio`モジュールのルートファイルであり、関連するオーディオ処理コンポーネントをまとめます。
- **`src/audio/player.rs`**: オーディオ再生の主要な制御ロジックを実装し、生成器やスケジューラーと連携して音声をリアルタイムで出力します。
- **`src/audio/scheduler.rs`**: YM2151イベントが適切なタイミングでオーディオジェネレーターに渡されるように、イベントのスケジュールを管理します。
- **`src/audio/stream.rs`**: オーディオデバイスへの出力ストリーム管理や、音声データフローの制御を行います。
- **`src/audio_config.rs`**: オーディオデバイスのサンプリングレートやバッファサイズなど、オーディオ関連の設定を定義する構造体や定数を含みます。
- **`src/client/config.rs`**: クライアントアプリケーションの挙動を制御するための設定オプションを定義します。
- **`src/client/core.rs`**: クライアントの基本的な機能、サーバーとの接続確立、コマンド送信といったコアロジックを実装します。
- **`src/client/interactive.rs`**: リアルタイムで音響制御を行うインタラクティブモードに特化したクライアント側の機能を提供します。
- **`src/client/json.rs`**: クライアントがサーバーに送信するYM2151イベントデータなどのJSON形式のメッセージをシリアライズ・デシリアライズする機能を提供します。
- **`src/client/mod.rs`**: `client`モジュールのルートファイルであり、クライアント関連のコンポーネントをまとめます。
- **`src/client/server.rs`**: クライアントがサーバーを起動、停止、シャットダウンするといったライフサイクル管理を行うための関数を提供します。
- **`src/debug_wav.rs`**: デバッグ目的で、生成されたオーディオデータをWAVファイルとしてディスクに書き出す機能を提供します。
- **`src/demo_client_interactive.rs`**: インタラクティブモードでのクライアントの動作をデモンストレーションするためのサンプルコードです。
- **`src/demo_server_interactive.rs`**: インタラクティブモードでのサーバーの動作をデモンストレーションするためのサンプルコードです。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードでのサーバーの動作をデモンストレーションするためのサンプルコードです。
- **`src/events.rs`**: YM2151レジスタへの書き込みイベントを表すデータ構造を定義します。タイムスタンプとレジスタアドレス・データを含みます。
- **`src/ipc/mod.rs`**: `ipc`（プロセス間通信）モジュールのルートファイルであり、サーバーとクライアント間の通信手段を抽象化します。
- **`src/ipc/pipe_windows.rs`**: Windowsオペレーティングシステム固有の名前付きパイプを利用したプロセス間通信の実装を提供します。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間で交換されるメッセージの構造と、通信規約を定義します。
- **`src/ipc/windows/mod.rs`**: Windows版IPCのサブモジュールをまとめます。
- **`src/ipc/windows/pipe_factory.rs`**: Windowsの名前付きパイプを作成および設定するためのユーティリティ関数を提供します。
- **`src/ipc/windows/pipe_handle.rs`**: Windows名前付きパイプのハンドルを安全に管理するためのラッパーを提供します。
- **`src/ipc/windows/pipe_reader.rs`**: Windows名前付きパイプからデータを受信する機能を提供します。
- **`src/ipc/windows/pipe_writer.rs`**: Windows名前付きパイプへデータを送信する機能を提供します。
- **`src/ipc/windows/test_logging.rs`**: WindowsのIPCパイプに関連するテストやデバッグのためのロギング機能を提供します。
- **`src/lib.rs`**: このRustクレートのライブラリ部分のエントリポイントです。他のRustプロジェクトからこの機能をライブラリとして利用する際に参照されます。
- **`src/logging.rs`**: アプリケーション全体のログ出力機能（デバッグ、情報、エラーなど）を実装します。
- **`src/main.rs`**: プロジェクトの実行可能バイナリのエントリポイントです。サーバーモードまたはクライアントモードのどちらとして起動するかを解析し、それぞれのロジックを実行します。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用して、リアルタイムオーディオ処理の優先度を適切に設定し、音の途切れを防ぎます。
- **`src/opm.rs`**: C言語の`Nuked-OPM`エミュレータをRustから利用するための高レベルなインターフェースやラッパーを提供します。
- **`src/opm_ffi.rs`**: `opm.c`と`opm.h`で定義されたC関数をRustから呼び出すためのForeign Function Interface (FFI) バインディングを定義します。
- **`src/player.rs`**: YM2151のレジスタイベントを解釈し、オーディオジェネレーターを通じて音源を再生する主要なプレイヤーロジックをカプセル化します。
- **`src/resampler.rs`**: 生成されたオーディオサンプルのサンプリングレートを変換（リサンプリング）する機能を提供します。
- **`src/scheduler.rs`**: 音楽イベントを時間に基づいてスケジューリングし、指定されたタイミングで再生システムに送る役割を担います。
- **`src/server/command_handler.rs`**: クライアントから受信したコマンドを解析し、サーバーの内部状態や再生処理を適切に更新するロジックを実装します。
- **`src/server/connection.rs`**: サーバーがクライアントからの接続を受け入れ、管理するためのロジックを実装します。
- **`src/server/mod.rs`**: `server`モジュールのルートファイルであり、サーバー側の主要なコンポーネントをまとめます。
- **`src/server/playback.rs`**: サーバーがオーディオ再生を行う際の主要な制御フローと状態管理を扱います。
- **`src/server/state.rs`**: サーバーの現在の再生状態、設定、その他の内部データを一元的に管理する構造体を定義します。
- **`src/tests/` (ディレクトリ内の各ファイル)**: プロジェクト内の各モジュールや機能に対する自動テストコードが格納されています。これらはコードの品質と安定性を保証するために利用されます。
- **`src/wav_writer.rs`**: 処理されたオーディオデータを標準的なWAVファイル形式で出力する機能を提供します。

## 関数詳細説明
このプロジェクトで主に公開されているクライアント側の関数について、その役割、引数、戻り値を以下に示します。

-   **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**
    -   役割: YM2151再生サーバーが利用可能であることを確認します。サーバーが起動していない場合、自動的にインストールし、バックグラウンドで起動します。
    -   引数: `app_name` - サーバーアプリケーションを特定するための名前（例: "cat-play-mml"）。
    -   戻り値: `anyhow::Result<()>` - サーバーの準備が成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

-   **`client::send_json(json_data: &str) -> anyhow::Result<()>`**
    -   役割: 非インタラクティブモードで、YM2151のレジスタイベントログを含むJSONデータをサーバーに送信し、そのデータを再生するよう指示します。
    -   引数: `json_data` - YM2151レジスタイベントが記述されたJSON文字列。
    -   戻り値: `anyhow::Result<()>` - データ送信と再生指示が成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

-   **`client::stop_playback() -> anyhow::Result<()>`**
    -   役割: サーバーに対して、現在行っているオーディオ再生を直ちに停止するよう指示します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` - 停止指示が成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

-   **`client::shutdown_server() -> anyhow::Result<()>`**
    -   役割: サーバープロセスを終了させるよう指示します。これにより、バックグラウンドで実行中のサーバーが安全にシャットダウンされます。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` - シャットダウン指示が成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

-   **`client::start_interactive() -> anyhow::Result<()>`**
    -   役割: サーバーをインタラクティブモードに移行させ、連続的なオーディオストリームの準備を開始します。このモードでは、イベントを動的にスケジュールできます。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` - インタラクティブモードへの移行が成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

-   **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**
    -   役割: インタラクティブモードで、YM2151レジスタイベントログを含むJSONデータをサーバーの再生スケジュールに追加します。これにより、既存の再生と連続してイベントが処理されます。
    -   引数: `json_data` - YM2151レジスタイベントが記述されたJSON文字列。タイムスタンプは自動的に秒単位に変換されます。
    -   戻り値: `anyhow::Result<()>` - イベントのスケジューリングが成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

-   **`client::clear_schedule() -> anyhow::Result<()>`**
    -   役割: インタラクティブモードで、サーバーにキューイングされている（まだ再生されていない）すべてのYM2151イベントスケジュールをクリアします。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` - スケジュールクリアが成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

-   **`client::get_server_time() -> anyhow::Result<f64>`**
    -   役割: インタラクティブモードで、サーバーの現在のオーディオ再生時間（秒単位）を取得します。これは正確なタイミング制御に利用できます。
    -   引数: なし。
    -   戻り値: `anyhow::Result<f64>` - 現在のサーバー時刻（秒）が `Ok(f64)` として返されます。エラーが発生した場合は `Err` を返します。

-   **`client::stop_interactive() -> anyhow::Result<()>`**
    -   役割: インタラクティブモードを終了し、サーバーが連続的なオーディオストリームの処理を停止するようにします。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` - インタラクティブモードの終了が成功した場合は `Ok(())`、エラーが発生した場合は `Err` を返します。

## 関数呼び出し階層ツリー
```
提供された情報からは関数呼び出し階層ツリーを生成できませんでした。

---
Generated at: 2025-12-26 07:02:16 JST
