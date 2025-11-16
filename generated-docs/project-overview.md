Last updated: 2025-11-17

# Project Overview

## プロジェクト概要
- YM2151（OPM）レジスタイベントログを受け取り、リアルタイムで音楽を再生するWindows専用のアプリケーションです。
- スタンドアロンで利用できるほか、サーバーとして常駐し、クライアントから再生制御やWAVファイル出力が可能です。
- 他の音楽作成・編集ツールにライブラリとして組み込み、よりインタラクティブな演奏体験を提供することを目指しています。

## 技術スタック
- フロントエンド: CLIベースまたはライブラリ利用が主であり、直接的なユーザーインターフェースとしてのフロントエンド技術は使用していません。
- 音楽・オーディオ:
    - YM2151 (OPM) チップエミュレーション: ヤマハFM音源チップYM2151の動作をソフトウェアで再現し、レジスタイベントログから音源を生成します。
    - Nuked-OPM: YM2151エミュレーションの中核として利用されているC言語製の音源ライブラリです。
    - リアルタイムオーディオ再生: WindowsのオーディオAPIを利用して、生成された音源を低遅延で再生します。
    - WAVファイル出力: 生成したオーディオデータを標準的なWAV形式で保存する機能を提供します。
- 開発ツール:
    - Rust: プロジェクトの主要な開発言語として使用されており、安全性とパフォーマンスに優れています。
    - cargo: Rustの公式ビルドシステムおよびパッケージマネージャーで、依存関係の管理とプロジェクトのビルドに利用されます。
    - zig cc: C言語で書かれたNuked-OPMライブラリをRustプロジェクトに組み込むためのCコンパイラとして使用されます。
    - anyhow: Rustにおける柔軟なエラーハンドリングを支援するクレートです。
    - serde / serde_json: JSONデータのシリアライズ（構造体からJSONへの変換）およびデシリアライズ（JSONから構造体への変換）に使用されます。
    - clap: コマンドラインインターフェース (CLI) の引数を簡単に定義し、パースするために使用されます。
    - log / env_logger: アプリケーションのログ出力機能を提供し、デバッグや実行状況の把握を助けます。
- テスト: Rustの標準テストフレームワークと、`assert_cmd`、`predicates`といったCLIアプリケーションのテストを補助するクレートが利用されています。
- ビルドツール: Rustプロジェクトのビルドには`cargo`が、C言語部分のコンパイルには`zig cc`が使われています。
- 言語機能:
    - Rust: 高い安全性、メモリ効率、並行性を実現するための現代的な言語機能が活用されています。
    - C言語: YM2151エミュレーションライブラリ`Nuked-OPM`の実装にC言語が用いられています。
- 自動化・CI/CD: `setup_ci_environment.sh`スクリプトが存在し、継続的インテグレーション（CI）環境のセットアップを自動化するのに役立ちます。
- 開発標準: `.editorconfig`ファイルによって、異なる開発環境間でのコードスタイルの一貫性が保たれています。

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
├── examples/
│   ├── test_client_non_verbose.rs
│   ├── test_client_verbose.rs
│   ├── test_logging_non_verbose.rs
│   └── test_logging_verbose.rs
├── generated-docs/
│   └── development-status-generated-prompt.md
├── issue-notes/
│   ├── 34.md
│   ├── 36.md
│   ├── 38.md
│   ├── 40.md
│   ├── 42.md
│   ├── 44.md
│   ├── 46.md
│   ├── 48.md
│   ├── 50.md
│   ├── 52.md
│   └── 54.md
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
│   ├── server.rs
│   └── wav_writer.rs
└── tests/
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
    ├── ipc_pipe_test.rs
    ├── logging_test.rs
    ├── phase3_test.rs
    ├── phase4_test.rs
    ├── phase5_test.rs
    ├── phase6_cli_test.rs
    ├── server_basic_test.rs
    ├── server_windows_fix_test.rs
    ├── tail_generation_test.rs
    └── test_utils.rs
```

## ファイル詳細説明
-   **`.cargo/config.toml`**: Cargoのカスタム設定ファイル。ビルド設定やリンカ設定などが含まれる場合があります。
-   **`.editorconfig`**: コードエディタ間でインデントや文字コードなどのコーディングスタイルを統一するための設定ファイルです。
-   **`.gitignore`**: Gitによるバージョン管理の対象から除外するファイルやディレクトリを指定するファイルです。
-   **`Cargo.lock`**: プロジェクトの依存関係の正確なバージョンを記録し、再現性のあるビルドを保証します。
-   **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存クレート、ビルド設定などが定義されます。
-   **`LICENSE`**: 本プロジェクトがMITライセンスで提供されることを示すライセンス条項が記述されています。
-   **`README.ja.md` / `README.md`**: プロジェクトの目的、機能、使い方、ビルド方法などを説明する日本語版および英語版の概要ドキュメントです。
-   **`_config.yml`**: GitHub Pagesなどのサイト生成ツールで使用される設定ファイルである可能性があります。
-   **`build.rs`**: Rustプロジェクトのビルドスクリプト。C言語のNuked-OPMライブラリをRustから利用できるよう、コンパイルしてリンクする処理などを記述します。
-   **`examples/`**: プロジェクトの各種機能の使用方法を示すサンプルコードが格納されているディレクトリです。
    -   `test_client_non_verbose.rs`, `test_client_verbose.rs`, `test_logging_non_verbose.rs`, `test_logging_verbose.rs`: クライアント通信やロギングの動作を確認するための具体的な使用例です。
-   **`generated-docs/`**: 自動生成されたドキュメントやレポートが格納されるディレクトリです。
    -   `development-status-generated-prompt.md`: プロンプトから生成された開発ステータスに関するドキュメントと思われます。
-   **`issue-notes/`**: 開発中の課題や特定の機能に関するメモが記録されたMarkdownファイルの集まりです。
-   **`opm.c` / `opm.h`**: YM2151 (OPM) 音源チップのエミュレーションを行うC言語のソースファイルおよびヘッダーファイルです。Nuked-OPMライブラリの実装が含まれます。
-   **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするために使用されるシェルスクリプトです。ビルドに必要なツール（例: `zig cc`）のインストールなどが含まれる可能性があります。
-   **`src/`**: プロジェクトの主要なRustソースコードが格納されているディレクトリです。
    -   **`audio.rs`**: リアルタイムオーディオ再生に関連するロジックを管理し、OSのオーディオAPI (WindowsのWASAPIなど) とのインターフェースを提供します。
    -   **`client.rs`**: サーバー・クライアントモードにおけるクライアント側のロジックを実装します。サーバーへのコマンド送信や、サーバーの自動起動機能を提供します。
    -   **`debug_wav.rs`**: デバッグ目的でオーディオデータをWAVファイルとして出力する機能を提供します。
    -   **`events.rs`**: YM2151のレジスタイベントを表現するデータ構造や、JSON形式の音楽データをパースするロジックを定義します。
    -   **`ipc/`**: プロセス間通信（IPC）に関するモジュールが格納されています。
        -   **`mod.rs`**: `ipc`モジュールのルートファイルで、他のIPC関連モジュールを公開します。
        -   **`pipe_windows.rs`**: Windowsの名前付きパイプ（Named Pipe）を用いた具体的なIPC通信の実装を提供します。
        -   **`protocol.rs`**: クライアントとサーバー間でやり取りされるコマンドやメッセージのデータ構造（プロトコル）を定義します。
    -   **`lib.rs`**: 本プロジェクトがライブラリとして使用される際のエントリポイントとなるファイルです。`client`モジュールなどの公開インターフェースを提供します。
    -   **`logging.rs`**: アプリケーション全体のロギング設定や、ログメッセージのフォーマットなどに関する機能を提供します。
    -   **`main.rs`**: 実行可能バイナリのエントリポイント。コマンドライン引数を解析し、スタンドアロンモード、サーバーモード、クライアントモードのいずれかの処理を起動します。
    -   **`opm.rs`**: C言語で書かれた`Nuked-OPM`エミュレータへのRust FFI (Foreign Function Interface) ラッパーを提供し、RustコードからOPMエミュレーション機能を利用できるようにします。
    -   **`opm_ffi.rs`**: C言語のOPMエミュレータとのFFI定義を管理するファイル。`opm.rs`と同様に、Cコードとの連携を担います。
    -   **`player.rs`**: YM2151のレジスタイベントログを解釈し、Nuked-OPMエミュレータを駆動してオーディオサンプルを生成する、中心的な再生ロジックを実装します。
    -   **`resampler.rs`**: 生成されたオーディオサンプルのサンプリングレートを変換するリサンプリング機能を提供します。
    -   **`server.rs`**: サーバー・クライアントモードにおけるサーバー側のロジックを実装します。クライアントからのコマンドを受け取り、YM2151の再生を制御します。
    -   **`wav_writer.rs`**: オーディオデータをWAVファイル形式で効率的に書き出すための機能を提供します。
-   **`tests/`**: プロジェクトの単体テストや結合テストが格納されているディレクトリです。
    -   **`fixtures/`**: テストで使用されるJSON形式の音楽データ（`complex.json`, `simple.json`など）が格納されています。
    -   `client_json_test.rs`, `client_test.rs`, `client_verbose_test.rs`, `debug_wav_test.rs`, `duration_test.rs`, `ensure_server_ready_test.rs`, `integration_test.rs`, `ipc_pipe_test.rs`, `logging_test.rs`, `phase3_test.rs`, `phase4_test.rs`, `phase5_test.rs`, `phase6_cli_test.rs`, `server_basic_test.rs`, `server_windows_fix_test.rs`, `tail_generation_test.rs`, `test_utils.rs`: 各モジュールや機能に対する具体的なテストケースが記述されています。

## 関数詳細説明
-   **`main` (src/main.rs)**:
    -   役割: プログラムの起動時に最初に実行されるエントリポイントです。コマンドライン引数を解析し、アプリケーションの動作モード（スタンドアロン、サーバー、クライアント）を決定して、適切な処理を開始します。
    -   引数: なし（実行環境のコマンドライン引数を内部で処理します）。
    -   戻り値: `anyhow::Result<()>` (処理の成功またはエラーを示す結果)。
    -   機能: `clap`クレートを使用してコマンドライン引数をパースし、引数に基づいて`server::run_server`、`client`モジュールのコマンド送信関数、またはスタンドアロン再生ロジックを呼び出します。
-   **`client::ensure_server_ready` (src/client.rs)**:
    -   役割: YM2151再生サーバーが実行中であることを確認し、もし起動していなければ自動的にインストールしてバックグラウンドで起動します。
    -   引数: `app_name: &str` (サーバーアプリケーションの実行ファイル名)。
    -   戻り値: `anyhow::Result<()>` (サーバーの準備が完了したかどうか)。
    -   機能: サーバーのプロセスが既に存在するかをチェックし、存在しない場合は`cargo install`コマンドでインストールし、バックグラウンドでサーバーを起動します。サーバーがコマンドを受け付けられる状態になるまで待機します。
-   **`client::send_json` (src/client.rs)**:
    -   役割: 指定されたJSON形式の音楽データを、起動中のサーバーに送信して再生を指示します。
    -   引数: `json_data: &str` (YM2151レジスタイベントログを含むJSON文字列)。
    -   戻り値: `anyhow::Result<()>` (コマンド送信の成功またはエラー)。
    -   機能: サーバーとのIPCチャネル（名前付きパイプ）を通じて、`PLAY`コマンドとJSONデータをサーバーに送信します。
-   **`client::stop_playback` (src/client.rs)**:
    -   役割: サーバーに対して、現在再生中の音楽を停止するよう指示します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` (コマンド送信の成功またはエラー)。
    -   機能: サーバーとのIPCチャネルを通じて、`STOP`コマンドを送信し、音源の再生を停止させます。
-   **`client::shutdown_server` (src/client.rs)**:
    -   役割: 起動中のサーバープロセスを安全にシャットダウンするよう指示します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` (コマンド送信の成功またはエラー)。
    -   機能: サーバーとのIPCチャネルを通じて、`SHUTDOWN`コマンドを送信し、サーバープロセスを終了させます。
-   **`server::run_server` (src/server.rs)**:
    -   役割: アプリケーションをサーバーモードで起動し、クライアントからのプロセス間通信（IPC）コマンドを継続的に監視・処理します。
    -   引数: なし。
    -   戻り値: `anyhow::Result<()>` (サーバー実行結果)。
    -   機能: Windowsの名前付きパイプを作成し、クライアントからの接続を待機します。`PLAY`, `STOP`, `SHUTDOWN`などのコマンドを受け取ると、`player`モジュールを介してYM2151の再生を制御します。
-   **`player::Player::new` (src/player.rs)**:
    -   役割: YM2151音源再生を管理する`Player`インスタンスを新しく作成し、初期化します。
    -   引数: (具体的な引数はソースコードによるが、おそらくオーディオ出力設定、サンプリングレートなどが含まれます)。
    -   戻り値: `Player` (初期化されたプレイヤーインスタンス)。
    -   機能: `Nuked-OPM`エミュレータの初期化、オーディオ出力ストリームの準備、リサンプラーの構成などを行います。
-   **`player::Player::play_json` (src/player.rs)**:
    -   役割: JSON形式のYM2151レジスタイベントログを解析し、リアルタイムで音源チップをエミュレートして音楽を再生します。
    -   引数: `json_data: &str` (再生するYM2151イベントを含むJSON文字列)。
    -   戻り値: `anyhow::Result<()>` (再生処理の成功またはエラー)。
    -   機能: `events`モジュールでJSONデータをパースし、`opm`モジュールを通じてYM2151エミュレータにレジスタコマンドを順番に送信します。`audio`モジュールを利用して生成されたオーディオサンプルをリアルタイムで出力し、必要に応じて`resampler`でサンプリングレートを調整します。
-   **`player::Player::stop` (src/player.rs)**:
    -   役割: 現在`Player`インスタンスが行っているYM2151の再生を直ちに停止し、音源を無音状態にします。
    -   引数: なし。
    -   戻り値: なし。
    -   機能: `Nuked-OPM`エミュレータの状態をリセットし、アクティブなオーディオストリームを停止またはミュートします。
-   **`audio::start_audio_stream` (src/audio.rs)**:
    -   役割: Windowsのオーディオデバイスに対し、リアルタイムでの音声出力ストリームを開始します。
    -   引数: (詳細な引数はソースコードによるが、オーディオデータを提供するコールバック関数、サンプリングレート、チャンネル数など)。
    -   戻り値: `anyhow::Result<AudioStreamHandle>` (オーディオストリームを制御するためのハンドル)。
    -   機能: OSのオーディオAPIと連携し、継続的にオーディオサンプルを要求し、スピーカーなどに出力するバックグラウンドプロセスを確立します。
-   **`opm::Opm::write_reg` (src/opm.rs)**:
    -   役割: YM2151 (OPM) チップエミュレータの特定のレジスタに値を書き込みます。
    -   引数: `reg_addr: u8` (YM2151レジスタのアドレス), `value: u8` (書き込む値)。
    -   戻り値: なし。
    -   機能: FFIを通じてC言語の`Nuked-OPM`エミュレータの内部関数を呼び出し、音源チップのサウンド生成パラメーターを更新します。
-   **`opm::Opm::mix` (src/opm.rs)**:
    -   役割: YM2151 (OPM) エミュレータからオーディオサンプルを生成し、指定されたバッファに書き込みます。
    -   引数: `buffer: &mut [f32]` (生成されたオーディオサンプルを格納するための浮動小数点数配列)。
    -   戻り値: なし。
    -   機能: FFIを通じてC言語の`Nuked-OPM`エミュレータの処理関数を呼び出し、現在の音源チップの状態に基づいて一定時間分のオーディオ波形データを計算し、バッファに充填します。
-   **`wav_writer::WavWriter::new` (src/wav_writer.rs)**:
    -   役割: 指定されたパスに新しいWAVファイルを作成し、オーディオデータを書き込むための`WavWriter`インスタンスを初期化します。
    -   引数: `path: &Path` (出力WAVファイルのパス), `sample_rate: u32` (サンプリングレート), `channels: u16` (チャンネル数)。
    -   戻り値: `anyhow::Result<WavWriter>` (WAVライターインスタンスの成功またはエラー)。
    -   機能: WAVファイルのヘッダー情報を設定し、ファイルストリームを開きます。
-   **`wav_writer::WavWriter::write_samples` (src/wav_writer.rs)**:
    -   役割: 浮動小数点形式のオーディオサンプルデータを、開いているWAVファイルに書き込みます。
    -   引数: `samples: &[f32]` (書き込むオーディオサンプルのスライス)。
    -   戻り値: `anyhow::Result<()>` (書き込み処理の成功またはエラー)。
    -   機能: 入力された浮動小数点サンプルをWAVファイル形式（通常は16-bit PCM）に変換し、ファイルに逐次書き込みます。

## 関数呼び出し階層ツリー
```
main (src/main.rs)
├── server::run_server (src/server.rs)  [if --server option is used]
│   ├── ipc::pipe_windows::create_named_pipe (src/ipc/pipe_windows.rs)
│   └── (Loop) Read client commands
│       ├── ipc::pipe_windows::read_command (src/ipc/pipe_windows.rs)
│       └── (Match command)
│           ├── player::Player::play_json (src/player.rs)  [on PLAY command]
│           │   ├── events::parse_json (src/events.rs)
│           │   ├── opm::Opm::write_reg (src/opm.rs) (repeatedly based on events)
│           │   ├── audio::start_audio_stream (src/audio.rs)
│           │   │   └── (OS Audio API calls)
│           │   ├── (Callback loop in audio stream)
│           │   │   ├── opm::Opm::mix (src/opm.rs)
│           │   │   └── resampler::resample (src/resampler.rs)
│           │   └── wav_writer::WavWriter::write_samples (src/wav_writer.rs) [if WAV output enabled]
│           ├── player::Player::stop (src/player.rs)  [on STOP command]
│           └── (Server exit) [on SHUTDOWN command]
├── client::send_command (Abstract command sending) [if --client option is used]
│   ├── client::ensure_server_ready (src/client.rs)  [optional, if server is not detected]
│   │   ├── (cargo install command)
│   │   ├── (std::process::Command::spawn)
│   │   └── ipc::pipe_windows::wait_for_server_ready (src/ipc/pipe_windows.rs)
│   ├── ipc::pipe_windows::connect_to_pipe (src/ipc/pipe_windows.rs)
│   └── ipc::pipe_windows::write_command (src/ipc/pipe_windows.rs)
│       ├── ipc::protocol::Command::PlayJson (JSON data)
│       ├── ipc::protocol::Command::Stop
│       └── ipc::protocol::Command::Shutdown
└── (Standalone Playback Logic) [if JSON file path is passed directly]
    ├── player::Player::new (src/player.rs)
    ├── player::Player::play_json (src/player.rs)
    └── (Optional WAV output)
        ├── wav_writer::WavWriter::new (src/wav_writer.rs)
        └── wav_writer::WavWriter::write_samples (src/wav_writer.rs) (repeatedly)

(Library Usage Flow for external crates)
External Crate's main()
└── ym2151_log_play_server::client::ensure_server_ready (src/client.rs)
    └── (Calls within ensure_server_ready as above)
└── ym2151_log_play_server::client::send_json (src/client.rs)
    └── (Calls within client::send_json as above, eventually ipc::pipe_windows::write_command)
└── ym2151_log_play_server::client::stop_playback (src/client.rs)
    └── (Calls within client::stop_playback as above, eventually ipc::pipe_windows::write_command)
└── ym2151_log_play_server::client::shutdown_server (src/client.rs)
    └── (Calls within client::shutdown_server as above, eventually ipc::pipe_windows::write_command)

---
Generated at: 2025-11-17 07:02:40 JST
