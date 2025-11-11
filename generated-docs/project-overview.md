Last updated: 2025-11-12

# Project Overview

## プロジェクト概要
- YM2151音源チップのレジスタイベントログをリアルタイムで再生するWindows専用アプリケーションです。
- スタンドアロンモードでの音楽再生に加え、サーバーとして常駐しクライアントから演奏を制御できます。
- JSON形式の音楽データ再生、WAVファイル出力、名前付きパイプによるプロセス間通信に対応しています。

## 技術スタック
- フロントエンド: コマンドラインインターフェース (CLI) を通じて操作されます。
- 音楽・オーディオ: YM2151 (OPM) チップエミュレーション、Nuked-OPM (LGPL 2.1ライセンスのC言語ライブラリ) を使用し、リアルタイムでの音楽生成とWAVファイル出力に対応しています。
- 開発ツール: Rust (プログラミング言語) とCargo (Rustのビルドシステムおよびパッケージマネージャ) が主要な開発ツールです。C言語部分のコンパイルにはzig ccが使用されます。
- テスト: Rustの標準テストフレームワークであるCargo testを用いて、様々な機能のテストが記述されています。
- ビルドツール: Cargoがプロジェクトのビルドを管理し、`build.rs`スクリプトを通じてC言語コンポーネント（Nuked-OPM）のビルドも統合されています。
- 言語機能: 主にRust言語で記述されており、Nuked-OPMとの連携にはFFI (Foreign Function Interface) を利用しています。
- 自動化・CI/CD: `setup_ci_environment.sh`スクリプトが存在することから、継続的インテグレーション環境のセットアップと自動化が行われていることが示唆されます。
- 開発標準: `.editorconfig`ファイルにより、プロジェクト全体のコードスタイルが統一されています。

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
📁 issue-notes/
  📖 34.md
📄 opm.c
📄 opm.h
📄 setup_ci_environment.sh
📁 src/
  📄 audio.rs
  📄 client.rs
  📄 events.rs
  📁 ipc/
    📄 mod.rs
    📄 pipe_windows.rs
    📄 protocol.rs
  📄 lib.rs
  📄 main.rs
  📄 opm.rs
  📄 opm_ffi.rs
  📄 player.rs
  📄 resampler.rs
  📄 server.rs
  📄 wav_writer.rs
📁 tests/
  📄 client_test.rs
  📄 duration_test.rs
  📁 fixtures/
    📊 complex.json
    📊 simple.json
  📄 integration_test.rs
  📄 ipc_pipe_test.rs
  📄 phase3_test.rs
  📄 phase4_test.rs
  📄 phase5_test.rs
  📄 phase6_cli_test.rs
  📄 server_basic_test.rs
  📄 server_windows_fix_test.rs
  📄 tail_generation_test.rs
  📄 test_utils.rs
```

## ファイル詳細説明
- **`.cargo/config.toml`**: Cargoのビルド設定やコンパイルオプション、エイリアスなどを定義するファイルです。Zig CCの利用設定などが記述されている可能性があります。
- **`.editorconfig`**: 異なるエディタやIDEを使用する開発者間で、インデントスタイル、文字コード、行末文字などのコードフォーマットを統一するための設定ファイルです。
- **`.gitignore`**: Gitバージョン管理システムが無視するファイルやディレクトリ（ビルド生成物、一時ファイルなど）を指定するファイルです。
- **`Cargo.lock`**: プロジェクトのビルド時に使用された依存クレートの正確なバージョンとハッシュ値が記録されており、再現性のあるビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルです。プロジェクト名、バージョン、著者情報、ライセンス、依存関係、ビルド設定などが定義されています。
- **`LICENSE`**: プロジェクトのライセンス情報（MIT License）を記述したファイルです。
- **`README.ja.md`**, **`README.md`**: プロジェクトの概要、主な機能、使い方、ビルド方法などを説明するドキュメントファイルです。日本語版と英語版があります。
- **`_config.yml`**: CI/CD (継続的インテグレーション/継続的デリバリー) パイプラインやCodeQLなどの外部ツールに関連する設定が記述されている可能性があります。
- **`build.rs`**: Cargoがビルドの前に実行するカスタムビルドスクリプトです。主にC言語で書かれたNuked-OPMライブラリのコンパイルと、RustからのFFIバインディング生成を担当します。
- **`generated-docs/`**: プロジェクトから自動生成されたドキュメントやコードなどが格納される可能性があるディレクトリです。
- **`issue-notes/34.md`**: 特定の課題（Issue #34）に関する調査結果や解決策のメモが記述されたファイルです。
- **`opm.c`**, **`opm.h`**: YM2151音源チップのエミュレータであるNuked-OPMライブラリのC言語ソースコードとそのヘッダファイルです。
- **`setup_ci_environment.sh`**: 継続的インテグレーション（CI）環境をセットアップするために使用されるシェルスクリプトです。
- **`src/audio.rs`**: オーディオデバイスへの出力インターフェースを管理し、リアルタイムオーディオ再生ストリームを処理するロジックが含まれています。
- **`src/client.rs`**: サーバーモードで実行中のアプリケーションに対して、再生、停止、シャットダウンなどのコマンドを名前付きパイプを通じて送信するクライアント側のロジックを実装しています。
- **`src/events.rs`**: YM2151レジスタへの書き込みイベントを表すデータ構造と、JSON形式のイベントログをパース（解析）する機能を提供します。
- **`src/ipc/mod.rs`**: プロセス間通信（IPC）に関連するサブモジュールをまとめるルートモジュールです。
- **`src/ipc/pipe_windows.rs`**: Windowsプラットフォーム専用の名前付きパイプを使用したプロセス間通信の具体的な実装を提供します。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間でやり取りされるメッセージの形式やコマンドの種類など、通信プロトコルを定義します。
- **`src/lib.rs`**: プロジェクトがライブラリとして機能する際に使用されるルートモジュールです。共通のデータ構造やヘルパー関数が含まれることがあります。
- **`src/main.rs`**: アプリケーションのエントリポイントです。コマンドライン引数を解析し、スタンドアロンモード、サーバーモード、クライアントモードのいずれかを起動する役割を担います。
- **`src/opm.rs`**: C言語で書かれたNuked-OPMライブラリの関数をRustから安全に呼び出すためのラッパーを提供し、FFI (Foreign Function Interface) を管理します。
- **`src/opm_ffi.rs`**: RustとC言語の間のFFIバインディング（結合コード）を定義するファイルで、通常は`bindgen`などのツールによって生成されます。
- **`src/player.rs`**: YM2151イベントログを読み込み、Nuked-OPMエミュレータを制御して実際のオーディオデータを生成し、再生キューに送る主要な再生ロジックを実装しています。
- **`src/resampler.rs`**: オーディオデータのサンプリングレートを異なるレートに変換するための機能を提供します。
- **`src/server.rs`**: クライアントからのコマンドを受信し、それに基づいてYM2151イベントの再生を制御するバックグラウンドサーバーのロジックを実装します。
- **`src/wav_writer.rs`**: 生成されたオーディオデータを標準的なWAVファイル形式で保存するための機能を提供します。
- **`tests/`**: プロジェクトのテストコードが格納されているディレクトリです。
    - **`tests/client_test.rs`**: クライアント機能の動作を検証するテストです。
    - **`tests/duration_test.rs`**: 音楽の再生時間やタイミングに関するテストです。
    - **`tests/fixtures/`**: 各種テストで使用されるサンプルデータ（JSON音楽データなど）が置かれています。
        - **`tests/fixtures/complex.json`**: 複雑な構造を持つJSON音楽データです。
        - **`tests/fixtures/simple.json`**: シンプルな構造を持つJSON音楽データです。
    - **`tests/integration_test.rs`**: プロジェクト全体の主要なコンポーネントが連携して正しく動作するかを検証する統合テストです。
    - **`tests/ipc_pipe_test.rs`**: 名前付きパイプによるプロセス間通信が正しく機能するかをテストします。
    - **`tests/phaseX_test.rs`**: 特定の開発フェーズや機能セットに焦点を当てたテストファイル群です（例: `phase3_test.rs`, `phase4_test.rs`など）。
    - **`tests/server_basic_test.rs`**: サーバーの基本的な起動、コマンド処理、停止などの動作を検証するテストです。
    - **`tests/server_windows_fix_test.rs`**: Windows環境に特有のサーバー関連のバグ修正や挙動を検証するテストです。
    - **`tests/tail_generation_test.rs`**: 音楽再生終了時のフェードアウトや後処理（テール）の生成に関するテストです。
    - **`tests/test_utils.rs`**: 各テストファイルで共通して使用されるユーティリティ関数やヘルパーロジックが定義されています。

## 関数詳細説明
- `main()`: アプリケーションのエントリポイント。コマンドライン引数を解析し、アプリケーションをスタンドアロンモード、サーバーモード、またはクライアントモードで起動します。
- `run_standalone_mode(json_file_path: &str)`: 指定されたJSONファイルからYM2151イベントログを読み込み、リアルタイムでオーディオ再生を行うか、またはWAVファイルとして出力します。
- `start_server_mode(json_file_path: &str)`: 名前付きパイプを作成し、クライアントからのコマンドを待ち受けながら、指定されたJSON音楽ファイルの再生を開始するサーバープロセスを起動します。
- `send_client_command(command: ClientCommand, file_path: Option<&str>)`: サーバーに接続し、指定されたコマンド（再生、停止、シャットダウン）と、必要に応じてJSONファイルパスを送信します。
- `play_ym2151_events(events: Vec<YM2151Event>, audio_output: AudioOutput)`: YM2151イベントのリストを受け取り、Nuked-OPMエミュレータを駆動してオーディオサンプルを生成し、指定されたオーディオ出力ストリームに書き込みます。
- `write_audio_to_wav(samples: &[f32], output_path: &str)`: 生成されたオーディオサンプルデータを受け取り、指定されたパスに標準のWAVファイル形式で保存します。
- `parse_json_events_from_file(file_path: &str) -> Result<Vec<YM2151Event>, Error>`: 指定されたJSONファイルからYM2151イベントのリストを読み込み、パースして返します。
- `ym2151_emulator_init()`: Nuked-OPMエミュレータを初期化し、内部状態をリセットします。
- `ym2151_emulator_render(register_address: u8, value: u8) -> Vec<f32>`: YM2151レジスタへの書き込みイベントをエミュレータに渡し、対応するオーディオサンプルチャンクを生成して返します。
- `create_named_pipe_server()`: Windowsの名前付きパイプサーバーインスタンスを作成し、クライアントからの接続を待機します。
- `connect_to_named_pipe_client()`: 既存の名前付きパイプサーバーにクライアントとして接続し、通信チャネルを確立します。
- `resample_audio_data(input_samples: &[f32], input_rate: u32, output_rate: u32) -> Vec<f32>`: 入力オーディオサンプルを指定された入力サンプリングレートから目的の出力サンプリングレートに変換します。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2025-11-12 07:02:42 JST
