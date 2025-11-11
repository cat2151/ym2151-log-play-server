Last updated: 2025-11-10

# Project Overview

## プロジェクト概要
- YM2151 (OPM) 音源チップのレジスタイベントログをリアルタイムで再生するWindows専用アプリケーションです。
- スタンドアロンモードに加え、サーバーとして常駐し、クライアントからの制御で複数のJSON音楽データの切り替え演奏が可能です。
- 生成したオーディオデータはWAVファイルとして出力することもでき、名前付きパイプによるプロセス間通信を利用しています。

## 技術スタック
- フロントエンド: コマンドラインインターフェース (CLI) ベースのため、特定のフロントエンドフレームワークは使用していません。
- 音楽・オーディオ: 
    - **YM2151 (OPM)**: エミュレート対象のヤマハ製FM音源チップ。
    - **Nuked-OPM (C言語ライブラリ)**: YM2151音源のエミュレーションコアとして使用されています（LGPL 2.1）。
    - **Rust クレート**: オーディオストリーム処理、WAVファイル書き出し、データ構造のシリアライズ・デシリアライズなどに利用されます。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語。
    - **Cargo**: Rustプロジェクトのビルドシステムおよびパッケージマネージャー。
    - **zig cc**: Cコンパイラとして使用され、Nuked-OPMライブラリのビルドに利用されます。
- テスト: 
    - **Cargo test**: Rustの組み込みテストフレームワークを使用し、単体テストおよび結合テストが記述されています。
    - **JSONフィクスチャ**: テスト用に準備されたJSON形式の音楽データファイルが使用されます。
- ビルドツール:
    - **Cargo**: プロジェクト全体のビルドと依存関係の管理を行います。
    - **build.rs**: C言語で書かれたNuked-OPMライブラリをRustプロジェクトに統合するためのビルドスクリプト。
- 言語機能:
    - **Rust言語機能全般**: 安全性、並行性、パフォーマンスに重点を置いたモダンなシステムプログラミング機能が活用されています。
- 自動化・CI/CD:
    - **setup_ci_environment.sh**: 継続的インテグレーション(CI)環境をセットアップするためのシェルスクリプトが存在します。
- 開発標準:
    - **.editorconfig**: 異なるエディタやIDE間で一貫したコーディングスタイルを維持するための設定ファイル。

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
  📖 development-status-generated-prompt.md
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
- **`.cargo/config.toml`**: Cargoのビルド設定ファイル。特定のコンパイラ（zig cc）の使用などを定義しています。
- **`.editorconfig`**: エディタのコードスタイル設定ファイル。インデントサイズや文字コードなどを統一します。
- **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定します。
- **`Cargo.lock`**: プロジェクトの依存関係の正確なバージョンを記録します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイル。プロジェクト名、バージョン、依存クレート、ビルド設定などが定義されています。
- **`LICENSE`**: プロジェクトのライセンス情報 (MIT License) を記載しています。
- **`README.ja.md`**, **`README.md`**: プロジェクトの概要、使い方、ビルド方法などを説明するドキュメント（日本語版と英語版）。
- **`_config.yml`**: プロジェクト全体の設定に関するYAMLファイル。
- **`build.rs`**: Rustのビルドスクリプト。Nuked-OPMのC言語ソースコードをコンパイルし、Rustから利用できるようにします。
- **`generated-docs/`**: 自動生成されたドキュメントを格納するためのディレクトリ。
- **`issue-notes/34.md`**: 特定の課題（Issue #34）に関する開発メモ。
- **`opm.c`**: Nuked-OPMライブラリのC言語ソースコード。YM2151音源のエミュレーションを行います。
- **`opm.h`**: `opm.c`に対応するヘッダファイル。C言語の関数やデータ構造の宣言が含まれます。
- **`setup_ci_environment.sh`**: 継続的インテグレーション (CI) 環境をセットアップするためのシェルスクリプト。
- **`src/audio.rs`**: オーディオデバイスへの出力に関連するロジックを実装します。サウンドカードへのアクセスやオーディオバッファの管理などを行います。
- **`src/client.rs`**: サーバーに対してコマンド（再生、停止、シャットダウンなど）を送信するクライアント側のロジックを定義します。
- **`src/events.rs`**: YM2151のレジスタ操作イベントのデータ構造を定義し、JSON形式の音楽データをパースする機能を提供します。
- **`src/ipc/mod.rs`**: プロセス間通信（IPC）モジュールのエントリポイント。
- **`src/ipc/pipe_windows.rs`**: Windowsの機能である名前付きパイプを使用して、サーバーとクライアント間の通信を実装します。
- **`src/ipc/protocol.rs`**: サーバーとクライアント間でやり取りされるコマンドやデータのプロトコル（形式）を定義します。
- **`src/lib.rs`**: プロジェクトのライブラリクレートのエントリポイント。共通の機能やデータ構造を定義します。
- **`src/main.rs`**: プログラムのエントリポイント。コマンドライン引数を解析し、スタンドアロン、サーバー、またはクライアントモードのいずれかでアプリケーションを起動します。
- **`src/opm.rs`**: C言語のNuked-OPMライブラリをRustから利用するための高レベルなバインディングとインターフェースを提供します。
- **`src/opm_ffi.rs`**: C言語のNuked-OPMライブラリとのFFI (Foreign Function Interface) 定義。Rustから直接Cの関数を呼び出すための型安全なインタフェースを生成します。
- **`src/player.rs`**: YM2151のイベントログを再生し、Nuked-OPMを通じてオーディオデータを生成するコア再生ロジックを管理します。
- **`src/resampler.rs`**: オーディオデータのサンプリングレートを変換するためのリサンプリング機能を提供します。
- **`src/server.rs`**: クライアントからの命令を受け取り、バックグラウンドでYM2151の演奏を管理するサーバー側のロジックを実装します。
- **`src/wav_writer.rs`**: 生成されたPCMオーディオデータを標準的なWAVファイル形式で出力する機能を提供します。
- **`tests/`**: プロジェクトのテストコードを格納するディレクトリ。
- **`tests/client_test.rs`**: クライアント機能のテスト。
- **`tests/duration_test.rs`**: 演奏時間の計算など、時間管理に関するテスト。
- **`tests/fixtures/complex.json`**: 複雑なYM2151イベントを含むテスト用JSONファイル。
- **`tests/fixtures/simple.json`**: シンプルなYM2151イベントを含むテスト用JSONファイル。
- **`tests/integration_test.rs`**: 統合テスト。各モジュールが連携して正しく動作するか検証します。
- **`tests/ipc_pipe_test.rs`**: 名前付きパイプによるIPCの機能テスト。
- **`tests/phaseX_test.rs`**: 開発フェーズごとの特定の機能やロジックを検証するテストファイル群。
- **`tests/server_basic_test.rs`**: サーバーの基本的な動作を検証するテスト。
- **`tests/server_windows_fix_test.rs`**: Windows環境でのサーバー機能に関する修正や特定の問題をテスト。
- **`tests/tail_generation_test.rs`**: オーディオの終端処理（音の余韻など）の生成に関するテスト。
- **`tests/test_utils.rs`**: テスト全体で共通して使用されるユーティリティ関数やヘルパーを定義します。

## 関数詳細説明
- **`main()` (src/main.rs)**:
    - 役割: アプリケーションのエントリポイント。
    - 引数: なし (ただし、内部でコマンドライン引数をパース)。
    - 戻り値: `Result<(), Box<dyn Error>>`。
    - 機能: コマンドライン引数を解析し、アプリケーションをスタンドアロンモード、サーバーモード、またはクライアントモードのいずれかで起動します。
- **`run_standalone_mode(json_log_file: &Path)` (src/main.rs)**:
    - 役割: JSON音楽ファイルを直接再生するスタンドアロンモードの処理を実行します。
    - 引数: `json_log_file` (再生するJSONファイルのパス)。
    - 戻り値: `Result<(), Box<dyn Error>>`。
    - 機能: 指定されたJSONファイルを読み込み、YM2151音源をリアルタイムで演奏します。
- **`start_server(json_log_file: &Path)` (src/server.rs経由)**:
    - 役割: YM2151ログ再生サーバーを起動し、バックグラウンドで常駐させます。
    - 引数: `json_log_file` (サーバー起動時に最初に再生するJSONファイルのパス)。
    - 戻り値: `Result<(), Box<dyn Error>>`。
    - 機能: 名前付きパイプを作成し、クライアントからのコマンドを待ち受けながら、指定されたJSON音楽の再生を開始します。
- **`send_client_command(command: Command, json_log_file: Option<&Path>)` (src/client.rs)**:
    - 役割: 起動中のサーバーに対して、再生、停止、シャットダウンなどのコマンドを送信します。
    - 引数: `command` (送信するコマンドの種類)、`json_log_file` (PLAYコマンドの場合に再生するJSONファイルのパス)。
    - 戻り値: `Result<(), Box<dyn Error>>`。
    - 機能: 名前付きパイプを通じてサーバーに命令を送り、演奏の制御を行います。
- **`play_json_data(player: &mut Player, events: Vec<YM2151Event>)` (src/player.rs)**:
    - 役割: パースされたYM2151イベントリストを基に、Nuked-OPMエミュレータを駆動し、オーディオデータを生成・再生します。
    - 引数: `player` (YM2151プレイヤーの状態管理オブジェクト)、`events` (再生するYM2151イベントのリスト)。
    - 戻り値: `Result<(), Box<dyn Error>>`。
    - 機能: イベントを時間順に処理し、対応するYM2151レジスタを操作して音源を鳴らします。
- **`write_wav_file(audio_data: &[f32], file_path: &Path)` (src/wav_writer.rs)**:
    - 役割: 生成されたオーディオサンプルをWAV形式でファイルに書き込みます。
    - 引数: `audio_data` (書き込むオーディオデータのスライス)、`file_path` (出力するWAVファイルのパス)。
    - 戻り値: `Result<(), Box<dyn Error>>`。
    - 機能: PCMオーディオデータから標準的なWAVヘッダを付加したファイルを生成します。
- **`init_opm()` (src/opm_ffi.rs / src/opm.rs)**:
    - 役割: Nuked-OPMエミュレータを初期化します。
    - 引数: なし。
    - 戻り値: YM2151エミュレータのインスタンスへのポインタまたはRustラッパーオブジェクト。
    - 機能: エミュレータの内部状態をセットアップし、レジスタをデフォルト値にリセットします。
- **`opm_write(instance, addr, data)` (src/opm_ffi.rs / src/opm.rs)**:
    - 役割: YM2151の特定のレジスタに値を書き込みます。
    - 引数: `instance` (YM2151エミュレータインスタンス)、`addr` (レジスタアドレス)、`data` (書き込むデータ)。
    - 戻り値: なし。
    - 機能: YM2151の音色、音量、周波数などの設定を変更します。
- **`opm_read(instance, addr)` (src/opm_ffi.rs / src/opm.rs)**:
    - 役割: YM2151の特定のレジスタから値を読み取ります。
    - 引数: `instance` (YM2151エミュレータインスタンス)、`addr` (レジスタアドレス)。
    - 戻り値: `u8` (レジスタから読み取ったデータ)。
    - 機能: YM2151のステータスや設定を読み取ります。
- **`opm_mix(instance, outputs, num_samples)` (src/opm_ffi.rs / src/opm.rs)**:
    - 役割: 指定されたサンプル数のオーディオデータを生成します。
    - 引数: `instance` (YM2151エミュレータインスタンス)、`outputs` (生成されたオーディオデータを格納するバッファ)、`num_samples` (生成するサンプル数)。
    - 戻り値: なし。
    - 機能: YM2151エミュレータをクロックを進め、FM合成によってオーディオ波形を計算し、出力バッファに書き込みます。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2025-11-10 07:02:18 JST
