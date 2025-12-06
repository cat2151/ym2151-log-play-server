Last updated: 2025-12-07

# Project Overview

## プロジェクト概要
- YM2151 (OPM) 音源チップのレジスタイベントログをリアルタイムで再生するシステムです。
- サーバー・クライアントアーキテクチャを採用し、柔軟な音楽データ再生と制御を可能にします。
- 演奏中に次の操作を受け付けるインタラクティブな音楽体験を提供し、音色エディタなどとの連携も容易です。

## 技術スタック
- フロントエンド: このプロジェクトの主要機能はバックエンドのオーディオ再生とサーバー制御に特化しており、専用のフロントエンド技術は使用されていません。クライアントとのインタラクションは主にプログラム的またはコマンドラインインターフェースを通じて行われます。
- 音楽・オーディオ:
    - YM2151 (OPM): 山葉YM2151音源チップ（OPM）のエミュレーションを核としています。
    - Nuked-OPM: YM2151音源のエミュレーションに用いられるC言語ライブラリをRustから利用しています（FFI経由）。
    - リアルタイムオーディオ再生: Windowsプラットフォームに特化し、低レイテンシでのオーディオ出力を行います。
    - WAVファイル出力: デバッグや記録のために、再生音声をWAV形式で出力する機能を備えています。
    - リサンプリング: オーディオデータのサンプリングレート変換処理を行います。
- 開発ツール:
    - Rust: プロジェクトの主要な開発言語として使用されており、高いパフォーマンスと安全性を実現しています。
    - Cargo: Rustのビルドシステムとパッケージマネージャー。依存関係の管理、ビルド、テスト、実行を統合的に行います。
    - anyhow: Rustで柔軟なエラーハンドリングを可能にするクレートです。
    - serde / serde_json: JSONデータのシリアライズおよびデシリアライズに使用されるRustクレートです。
- テスト:
    - Cargo test: Rustの標準テストフレームワークを利用し、ユニットテストおよび統合テストが記述されています。テスト駆動開発（TDD）が積極的に採用されています。
- ビルドツール:
    - Cargo: Rustプロジェクトのビルドプロセス全体を管理します。
    - build.rs: Rustの標準ビルドスクリプトで、主にC言語ライブラリ (Nuked-OPM) とのFFIバインディング生成に使用されます。
- 言語機能:
    - Rust: メモリ安全性、並行処理、パフォーマンスに重点を置いたシステムプログラミング言語の特徴を活かしています。
- 自動化・CI/CD:
    - setup_ci_environment.sh: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプトです。
    - rust-script: Rustスクリプトを直接実行するためのツールであり、関連ツールのインストールスクリプトに利用されています。
- 開発標準:
    - .editorconfig: 複数のエディタやIDE間でコードスタイルの一貫性を保つための設定ファイルです。
    - .gitignore: Gitのバージョン管理から除外するファイルやディレクトリを指定します。
    - MIT License / LGPL 2.1: プロジェクト本体はMITライセンス、使用しているNuked-OPMライブラリはLGPL 2.1ライセンスです。

## ファイル階層ツリー
```
.editorconfig
.gitignore
.vscode/
  extensions.json
  settings.json
Cargo.lock
Cargo.toml
LICENSE
README.ja.md
README.md
_config.yml
build.rs
generated-docs/
googled947dc864c270e07.html
install-ym2151-tools.rs
issue-notes/
  100.md
  101.md
  102.md
  110.md
  111.md
  112.md
  113.md
  116.md
  117.md
  118.md
  119.md
  120.md
  121.md
  122.md
  96.md
  97.md
  98.md
  99.md
opm.c
opm.h
output_ym2151.json
setup_ci_environment.sh
src/
  audio/
    buffers.rs
    commands.rs
    generator.rs
    mod.rs
    player.rs
    scheduler.rs
    stream.rs
  audio_config.rs
  client/
    config.rs
    core.rs
    interactive.rs
    json.rs
    mod.rs
    server.rs
  debug_wav.rs
  demo_client_interactive.rs
  demo_server_interactive.rs
  demo_server_non_interactive.rs
  events.rs
  ipc/
    mod.rs
    pipe_windows.rs
    protocol.rs
    windows/
      mod.rs
      pipe_factory.rs
      pipe_handle.rs
      pipe_reader.rs
      pipe_writer.rs
      test_logging.rs
  lib.rs
  logging.rs
  main.rs
  mmcss.rs
  opm.rs
  opm_ffi.rs
  player.rs
  resampler.rs
  scheduler.rs
  server/
    command_handler.rs
    connection.rs
    mod.rs
    playback.rs
    state.rs
  tests/
    audio_tests.rs
    client_tests.rs
    debug_wav_tests.rs
    demo_server_interactive_tests.rs
    demo_server_non_interactive_tests.rs
    events_tests.rs
    ipc_pipe_windows_tests.rs
    ipc_protocol_tests.rs
    logging_tests.rs
    mmcss_tests.rs
    mod.rs
    opm_ffi_tests.rs
    opm_tests.rs
    play_json_interactive_tests.rs
    player_tests.rs
    resampler_tests.rs
    scheduler_tests.rs
    server_tests.rs
    wav_writer_tests.rs
  wav_writer.rs
tests/
  audio/
    audio_playback_test.rs
    audio_sound_test.rs
    mod.rs
  clear_schedule_test.rs
  cli_integration_test.rs
  client_json_test.rs
  client_test.rs
  client_verbose_test.rs
  debug_wav_test.rs
  duration_test.rs
  ensure_server_ready_test.rs
  events_processing_test.rs
  feature_demonstration_test.rs
  fixtures/
    complex.json
    simple.json
  integration_test.rs
  interactive/
    mod.rs
    mode_test.rs
    play_json_test.rs
    shared_mutex.rs
    step_by_step_test.rs
  interactive_tests.rs
  ipc_pipe_test.rs
  logging_test.rs
  server_basic_test.rs
  server_integration_test.rs
  tail_generation_test.rs
  test_util_server_mutex.rs
```

## ファイル詳細説明
- **`.editorconfig`**: コードエディタの設定を定義し、プロジェクト全体で一貫したコーディングスタイル（インデント、改行など）を強制します。
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリ（ビルド生成物、一時ファイルなど）を指定します。
- **`.vscode/extensions.json`**: VS Codeでこのプロジェクトを開いた際に推奨される拡張機能をリストアップします。
- **`.vscode/settings.json`**: VS Codeのワークスペース固有の設定を定義し、開発環境を最適化します。
- **`Cargo.lock`**: Rustの依存関係解決の結果を正確に記録し、ビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョン）、依存関係、ビルド設定などを定義するマニフェストファイルです。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）が記述されています。
- **`README.ja.md`**: プロジェクトの目的、機能、使い方などを日本語で説明する主要なドキュメントです。
- **`README.md`**: プロジェクトの目的、機能、使い方などを英語で説明する主要なドキュメントです。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレータ（Jekyllなど）で使用される設定ファイルです。
- **`build.rs`**: Rustプロジェクトのビルドプロセス中に実行されるカスタムビルドスクリプトです。C言語ライブラリ（Nuked-OPM）のコンパイルとRustからのFfiバインディングの生成に使用されます。
- **`generated-docs/`**: 自動生成されたドキュメントやコードに関する情報を格納するディレクトリです。
- **`googled947dc864c270e07.html`**: Googleのサイト所有権確認ファイルで、プロジェクトのコードとは直接関係ありません。
- **`install-ym2151-tools.rs`**: 関連する開発ツールやユーティリティを一括してインストールするためのRustスクリプトです。
- **`issue-notes/`**: 開発中の課題、バグ、機能要求などのメモを記述したMarkdownファイルが格納されています（開発者向け情報）。
- **`opm.c`**, **`opm.h`**: YM2151音源チップのエミュレーションを行うC言語ライブラリ「Nuked-OPM」のソースファイルとヘッダーファイルです。
- **`output_ym2151.json`**: YM2151レジスタへの書き込みイベントをJSON形式で記述したサンプルデータファイルです。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするためのシェルスクリプトです。
- **`src/lib.rs`**: このRustクレートのライブラリ部分のルートファイルです。クライアントAPIやサーバーと共有されるロジックが定義されています。
- **`src/main.rs`**: プログラムの実行エントリポイントとなるファイルです。コマンドライン引数を解析し、サーバーまたはクライアントとしてのアプリケーションの動作を制御します。
- **`src/audio/buffers.rs`**: オーディオデータの一時的な保持に使用されるバッファの管理ロジックを実装しています。
- **`src/audio/commands.rs`**: オーディオ再生に関連する各種コマンド（再生開始、停止など）の定義を管理しています。
- **`src/audio/generator.rs`**: YM2151音源エミュレータ（Nuked-OPM）から実際のオーディオサンプルデータを生成するロジックが含まれています。
- **`src/audio/mod.rs`**: `src/audio`モジュールのルートファイルで、オーディオ関連のサブモジュールを公開します。
- **`src/audio/player.rs`**: オーディオ再生の全体的な制御フローと状態管理を担う主要なコンポーネメントです。
- **`src/audio/scheduler.rs`**: 音楽イベントを時間に基づいて正確にスケジューリングし、適切なタイミングで再生処理へ送る役割を果たします。
- **`src/audio/stream.rs`**: リアルタイムオーディオストリームの開始、停止、データ供給などの管理ロジックを提供します。
- **`src/audio_config.rs`**: オーディオ出力のグローバル設定（サンプリングレート、チャンネル数、バッファサイズなど）を定義します。
- **`src/client/config.rs`**: クライアントアプリケーションの挙動に関する設定やオプションを管理します。
- **`src/client/core.rs`**: クライアントがサーバーと通信し、基本的な再生制御を行うためのコア機能を提供します。
- **`src/client/interactive.rs`**: インタラクティブモードでのクライアント操作ロジックを実装し、リアルタイムでのイベントスケジューリングを可能にします。
- **`src/client/json.rs`**: JSON形式の音楽データをパースし、サーバーに送信する処理を担います。
- **`src/client/mod.rs`**: `src/client`モジュールのルートファイルで、クライアントが外部から利用するAPIを公開します。
- **`src/client/server.rs`**: クライアントからYM2151再生サーバーの起動や終了を管理するためのロジックを提供します。
- **`src/debug_wav.rs`**: デバッグ目的で、生成されたオーディオデータをWAVファイルとしてディスクに書き込む機能を提供します。
- **`src/demo_client_interactive.rs`**: インタラクティブモードでのクライアントの利用例を示すデモコードです。
- **`src/demo_server_interactive.rs`**: インタラクティブモードでのサーバーの動作を示すデモコードです。
- **`src/demo_server_non_interactive.rs`**: 非インタラクティブモードでのサーバーの動作を示すデモコードです。
- **`src/events.rs`**: YM2151レジスタへの書き込みイベントのデータ構造と、それらのイベントを処理するロジックを定義します。
- **`src/ipc/mod.rs`**: `src/ipc`モジュールのルートファイルで、プロセス間通信（IPC）に関連するサブモジュールを公開します。
- **`src/ipc/pipe_windows.rs`**: Windowsオペレーティングシステム特有の名前付きパイプを使用したIPCの実装を提供します。
- **`src/ipc/protocol.rs`**: クライアントとサーバー間でやり取りされるコマンドやメッセージのデータフォーマット（通信プロトコル）を定義します。
- **`src/ipc/windows/mod.rs`**: Windows固有のIPC実装をまとめるモジュールのルートファイルです。
- **`src/ipc/windows/pipe_factory.rs`**: 名前付きパイプの作成と初期化を行うファクトリパターンを実装しています。
- **`src/ipc/windows/pipe_handle.rs`**: Windowsのパイプハンドルを安全に管理するためのラッパー構造体です。
- **`src/ipc/windows/pipe_reader.rs`**: 名前付きパイプからのデータ読み込み処理を実装しています。
- **`src/ipc/windows/pipe_writer.rs`**: 名前付きパイプへのデータ書き込み処理を実装しています。
- **`src/ipc/windows/test_logging.rs`**: IPC関連のテスト時に使用されるロギングユーティリティを提供します。
- **`src/logging.rs`**: アプリケーション全体のロギング設定（ログレベル、出力先など）を構成します。
- **`src/mmcss.rs`**: WindowsのMultimedia Class Scheduler Service (MMCSS) を利用し、リアルタイムオーディオ処理の優先度を高く設定する機能を提供します。
- **`src/opm.rs`**: C言語で書かれたNuked-OPMライブラリをRustから扱いやすくするための高レベルなラッパーを提供します。
- **`src/opm_ffi.rs`**: RustとC言語のNuked-OPMライブラリ間のForeign Function Interface (FFI) バインディングを定義し、C関数をRustから呼び出せるようにします。
- **`src/player.rs`**: YM2151音源の再生に関する高レベルなロジックをカプセル化し、音源の初期化やイベント処理を管理します。
- **`src/resampler.rs`**: オーディオデータのサンプリングレートを変換するアルゴリズムと実装を提供します。
- **`src/scheduler.rs`**: イベントの登録、時間ベースのソート、実行を管理する汎用的なスケジューリングシステムです。
- **`src/server/command_handler.rs`**: クライアントから送信されたコマンドを解析し、それに応じたサーバー側の処理（再生、停止、シャットダウンなど）を実行します。
- **`src/server/connection.rs`**: サーバーとクライアント間の通信接続（名前付きパイプ）の確立と管理を担います。
- **`src/server/mod.rs`**: `src/server`モジュールのルートファイルで、サーバーの主要なロジックを公開します。
- **`src/server/playback.rs`**: サーバー上でのオーディオ再生状態（再生中、停止中、インタラクティブモードなど）とその遷移を管理します。
- **`src/server/state.rs`**: サーバー全体の現在の状態（設定、再生情報、スケジューラインスタンスなど）を保持するコンポーネントです。
- **`src/tests/`**: プロジェクト内の各モジュールのユニットテストが含まれるディレクトリです。
- **`src/wav_writer.rs`**: RAWオーディオデータを受け取り、標準的なWAVファイル形式でディスクに書き出す機能を提供します。
- **`tests/`**: プロジェクト全体の統合テストやエンドツーエンドテストが格納されるディレクトリです。
- **`tests/audio/`**: オーディオ関連機能のテストが含まれるディレクトリです。
    - **`tests/audio/audio_playback_test.rs`**: オーディオ再生システムの健全性を確認するテストです。
    - **`tests/audio/audio_sound_test.rs`**: 実際のサウンド出力が期待通りに行われるかを検証するテストです。
    - **`tests/audio/mod.rs`**: `tests/audio`モジュールのルートファイルです。
- **`tests/clear_schedule_test.rs`**: インタラクティブモードでのスケジュールクリア機能の動作を検証するテストです。
- **`tests/cli_integration_test.rs`**: コマンドラインインターフェース（CLI）の動作を総合的に検証する統合テストです。
- **`tests/client_json_test.rs`**: クライアントがJSONデータをサーバーに正しく送信できるかをテストします。
- **`tests/client_test.rs`**: クライアントの基本的な機能（サーバーへの接続、コマンド送信など）を検証するテストです。
- **`tests/client_verbose_test.rs`**: クライアントの冗長（verbose）モードでのログ出力などを検証するテストです。
- **`tests/debug_wav_test.rs`**: デバッグWAV出力機能が正しく動作するかを確認するテストです。
- **`tests/duration_test.rs`**: プロジェクト内の時間計算や期間に関するロジックを検証するテストです。
- **`tests/ensure_server_ready_test.rs`**: サーバーの準備が自動的に行われる機能（インストール、起動、待機）を検証するテストです。
- **`tests/events_processing_test.rs`**: YM2151イベントのパース、スケジューリング、処理ロジックを検証するテストです。
- **`tests/feature_demonstration_test.rs`**: 主要な機能やシナリオが期待通りに動作することをデモンストレーションするテストです。
- **`tests/fixtures/`**: テストで使用するサンプルデータや設定ファイルが格納されています。
    - **`tests/fixtures/complex.json`**: 複雑なYM2151イベントを含むテスト用JSONデータです。
    - **`tests/fixtures/simple.json`**: シンプルなYM2151イベントを含むテスト用JSONデータです。
- **`tests/integration_test.rs`**: アプリケーション全体の主要なコンポーネントが連携して動作することを検証する一般的な統合テストです。
- **`tests/interactive/`**: インタラクティブモード関連のテストが含まれるディレクトリです。
    - **`tests/interactive/mod.rs`**: `tests/interactive`モジュールのルートファイルです。
    - **`tests/interactive/mode_test.rs`**: インタラクティブモードの開始・停止とその状態遷移を検証するテストです。
    - **`tests/interactive/play_json_test.rs`**: インタラクティブモードでJSONデータを再生する機能を検証するテストです。
    - **`tests/interactive/shared_mutex.rs`**: 共有リソースへの排他アクセスを管理するミューテックスの動作をテストします。
    - **`tests/interactive/step_by_step_test.rs`**: インタラクティブな操作をステップバイステップで実行し、各段階での動作を検証するテストです。
- **`tests/interactive_tests.rs`**: インタラクティブモード全般の機能と挙動を検証するテストです。
- **`tests/ipc_pipe_test.rs`**: プロセス間通信（IPC）に用いる名前付きパイプの機能と信頼性を検証するテストです。
- **`tests/logging_test.rs`**: アプリケーションのロギング機能が正しくメッセージを出力するかを検証するテストです。
- **`tests/server_basic_test.rs`**: サーバーの基本的な起動、待機、シャットダウンなどの機能を検証するテストです。
- **`tests/server_integration_test.rs`**: サーバーがクライアントからの様々な要求に正しく応答し、オーディオ再生を適切に制御できるかを検証する統合テストです。
- **`tests/tail_generation_test.rs`**: 音声の再生終了時やイベントの終端処理が適切に行われるかを検証するテストです。
- **`tests/test_util_server_mutex.rs`**: テストユーティリティとして、サーバーの状態を保護するためのミューテックス関連のロジックを定義します。

## 関数詳細説明
- **`client::ensure_server_ready(app_name: &str) -> anyhow::Result<()>`**
    - **役割**: YM2151再生サーバーの準備状況を確認し、必要であれば自動的にインストール、バックグラウンド起動を行い、利用可能になるまで待機します。
    - **引数**: `app_name` - クライアントアプリケーションの名前。サーバーの識別やログ出力に使用される場合があります。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: クライアントがサーバーのライフサイクル管理を手動で行う手間を省き、シームレスな統合を可能にします。
- **`client::send_json(json_data: &str) -> anyhow::Result<()>`**
    - **役割**: YM2151レジスタのイベントログを含むJSONデータをサーバーに送信し、新しい音楽の再生を開始します（非インタラクティブモード）。
    - **引数**: `json_data` - YM2151レジスタへの書き込みイベントが記述されたJSON形式の文字列。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: 単発の音楽データの再生や、前回の演奏を停止して新しい演奏に切り替える際に使用されます。
- **`client::stop_playback() -> anyhow::Result<()>`**
    - **役割**: サーバーに対して、現在再生中のYM2151音楽を停止するよう指示します。
    - **引数**: なし。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: 強制的に再生を中断し、音源を無音状態にします。
- **`client::shutdown_server() -> anyhow::Result<()>`**
    - **役割**: YM2151再生サーバーを安全にシャットダウンするよう指示します。
    - **引数**: なし。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: サーバープロセスを終了させ、リソースを解放します。
- **`client::start_interactive() -> anyhow::Result<()>`**
    - **役割**: サーバーをインタラクティブモードに移行させ、連続した音声ストリームの生成を準備します。これにより、リアルタイムなイベントスケジューリングが可能になります。
    - **引数**: なし。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: リアルタイム音楽制御や音色エディタなど、低レイテンシでの動的な操作が必要なシナリオで利用されます。
- **`client::play_json_interactive(json_data: &str) -> anyhow::Result<()>`**
    - **役割**: インタラクティブモードでYM2151レジスタのイベントログを含むJSONデータをサーバーに送信し、既存の音声ストリームを中断せずにイベントをスケジュールに追加します。
    - **引数**: `json_data` - YM2151レジスタイベントが記述されたJSON形式の文字列。イベントの時間情報は秒単位に自動変換されます。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: 無音ギャップなしで、複数の音楽フレーズやイベントを連続して再生・切り替えを行うために使用されます。
- **`client::clear_schedule() -> anyhow::Result<()>`**
    - **役割**: インタラクティブモードにおいて、まだ再生されていない未来のイベントスケジュールをすべてクリアします。
    - **引数**: なし。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: リアルタイムで曲を切り替えたり、予期しないイベントを中断したりする際に、過去のイベントの影響を排除するために使用されます。
- **`client::get_server_time() -> anyhow::Result<f64>`**
    - **役割**: サーバーのオーディオエンジンが現在処理している時刻を秒単位（`f64`）で取得します。
    - **引数**: なし。
    - **戻り値**: サーバーの現在時刻（秒）が `Ok(f64)` で返されるか、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: クライアントがイベントを正確なタイミングでスケジューリングするために、サーバーと時刻を同期する際に利用されます（Web Audio APIの`currentTime`に相当）。
- **`client::stop_interactive() -> anyhow::Result<()>`**
    - **役割**: サーバーのインタラクティブモードを終了し、連続した音声ストリームの生成を停止します。
    - **引数**: なし。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: インタラクティブなセッションを安全に終了させ、通常の再生モードに戻すか、リソースを解放します。
- **`main() -> anyhow::Result<()>`**
    - **役割**: プロジェクトのメインエントリポイントであり、コマンドライン引数を解析してアプリケーションをサーバーモードまたはクライアントモードとして起動します。
    - **引数**: なし（コマンドライン引数は環境から取得されます）。
    - **戻り値**: 成功した場合は `Ok(())`、エラーが発生した場合は`anyhow::Result`型のエラー。
    - **機能**: アプリケーションの起動時の設定を行い、適切な動作モードへのディスパッチを管理します。

## 関数呼び出し階層ツリー
```
main()
├── Server Mode Execution
│   ├── server::mod::run_server()
│   │   ├── src::audio::stream::start_audio_stream()
│   │   ├── ipc::pipe_windows::create_named_pipe_server()
│   │   └── server::command_handler::handle_client_commands()
│   │       ├── server::playback::start_playback()
│   │       ├── server::playback::stop_playback()
│   │       ├── server::playback::start_interactive_mode()
│   │       ├── server::playback::schedule_events()
│   │       ├── server::playback::clear_event_schedule()
│   │       └── server::playback::get_current_time()
│   └── (Graceful shutdown logic)
└── Client Mode Execution
    ├── client::main_client_cli()
    │   ├── client::ensure_server_ready()
    │   │   ├── (Internal: Cargo install if server not found)
    │   │   └── (Internal: `std::process::Command::spawn` to start server)
    │   ├── client::send_json()
    │   │   └── ipc::pipe_windows::send_command()
    │   ├── client::stop_playback()
    │   │   └── ipc::pipe_windows::send_command()
    │   ├── client::shutdown_server()
    │   │   └── ipc::pipe_windows::send_command()
    │   ├── client::start_interactive()
    │   │   └── ipc::pipe_windows::send_command()
    │   ├── client::play_json_interactive()
    │   │   └── ipc::pipe_windows::send_command()
    │   ├── client::clear_schedule()
    │   │   └── ipc::pipe_windows::send_command()
    │   └── client::get_server_time()
    │       └── ipc::pipe_windows::send_command()
    └── (Error handling)

---
Generated at: 2025-12-07 07:02:48 JST
