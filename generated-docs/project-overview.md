Last updated: 2025-11-09

# Project Overview

## プロジェクト概要
- YM2151 (OPM) 音源チップのレジスタイベントログをリアルタイムで再生するシステムです。
- スタンドアロンでのWAVファイル出力に加え、サーバー・クライアント構成で音楽データ演奏をバックグラウンドで実行します。
- Windowsプラットフォーム向けに特化しており、名前付きパイプによる効率的な制御を実現しています。

## 技術スタック
- フロントエンド: **Tone.js** (Web Audio API音声ライブラリを利用し、音声処理を簡素化), **Web Audio API** (ブラウザの音声処理技術、Tone.jsを通じて間接的に利用される可能性)
- 音楽・オーディオ: **MML (Music Macro Language)** (音楽記法を解析し、イベントデータに変換するパーサー)
- 開発ツール: **Node.js runtime** (JavaScript実行環境、ビルドスクリプトやCI/CDプロセスの一部で利用される可能性)
- テスト: *(特定のテストフレームワークは明示されていませんが、Rustの標準テスト機能が利用されています)*
- ビルドツール: *(明示されていませんが、RustプロジェクトのためCargoが利用されます)*
- 言語機能: *(明示されていませんが、Rust言語が利用されています)*
- 自動化・CI/CD: **GitHub Actions** (プロジェクト要約自動生成、Issue自動管理、README多言語翻訳、i18n automationといったCI/CDワークフローを自動化)
- 開発標準: **EditorConfig** (異なるエディタやIDE間でコードスタイルの一貫性を保つための設定)

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
├── generated-docs/
│   └── development-status-generated-prompt.md
├── opm.c
├── opm.h
├── setup_ci_environment.sh
├── src/
│   ├── audio.rs
│   ├── client.rs
│   ├── events.rs
│   ├── ipc/
│   │   ├── mod.rs
│   │   ├── pipe_windows.rs
│   │   └── protocol.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── opm.rs
│   ├── opm_ffi.rs
│   ├── player.rs
│   ├── resampler.rs
│   ├── server.rs
│   └── wav_writer.rs
└── tests/
    ├── client_test.rs
    ├── duration_test.rs
    ├── fixtures/
    │   ├── complex.json
    │   └── simple.json
    ├── integration_test.rs
    ├── ipc_pipe_test.rs
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
-   **`.cargo/config.toml`**: RustのビルドツールであるCargoの設定ファイルです。ビルド時の挙動やプロファイルをカスタマイズします。
-   **`.editorconfig`**: さまざまなエディタやIDE間で一貫したコーディングスタイル（インデント、改行コードなど）を維持するための設定ファイルです。
-   **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定します。
-   **`Cargo.lock`**: プロジェクトが依存するすべてのライブラリとその正確なバージョンを記録し、再現可能なビルドを保証します。
-   **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
-   **`LICENSE`**: プロジェクトのライセンス情報が記載されています。
-   **`README.ja.md` / `README.md`**: プロジェクトの概要、使い方、機能などを説明するドキュメント（日本語版と英語版）です。
-   **`_config.yml`**: GitHub ActionsなどのCI/CDワークフローや、ドキュメント生成などの設定に使用される汎用的な設定ファイルです。
-   **`build.rs`**: Rustのカスタムビルドスクリプトです。C言語のソースファイル（`opm.c`など）のコンパイルとリンクを設定するために使用されます。
-   **`generated-docs/development-status-generated-prompt.md`**: 自動生成されたドキュメントの一部で、開発ステータスに関する情報が記述されている可能性があります。
-   **`opm.c` / `opm.h`**: YM2151 (OPM) 音源チップのエミュレーションロジックをC言語で実装したソースファイルとヘッダファイルです。
-   **`setup_ci_environment.sh`**: CI (継続的インテグレーション) 環境をセットアップするために実行されるシェルスクリプトです。
-   **`src/audio.rs`**: システムのオーディオ出力デバイスを管理し、生成された音声データをリアルタイムで再生するためのインターフェースとロジックを定義します。
-   **`src/client.rs`**: サーバーとして常駐する本体に対して、名前付きパイプを通じて接続し、音楽データの送信や再生制御コマンドを送るクライアント側のロジックを実装します。
-   **`src/events.rs`**: YM2151音源チップのレジスタ操作イベントを表現するデータ構造と、それらを処理するためのヘルパー関数を定義します。
-   **`src/ipc/`**: プロセス間通信（IPC）に関連するモジュール群です。
    -   **`src/ipc/mod.rs`**: `ipc`モジュールのエントリポイントです。
    -   **`src/ipc/pipe_windows.rs`**: WindowsのOS機能である名前付きパイプを利用したプロセス間通信の実装を提供します。
    -   **`src/ipc/protocol.rs`**: サーバーとクライアント間で交換されるメッセージやコマンドのデータ構造、つまり通信プロトコルを定義します。
-   **`src/lib.rs`**: プロジェクトのライブラリクレートのエントリポイントで、他のモジュールや構造体を公開し、再利用可能な機能を提供します。
-   **`src/main.rs`**: アプリケーションのメインエントリポイントです。コマンドライン引数を解析し、スタンドアロン再生、サーバーモード起動、クライアント操作のいずれかを選択し、実行します。
-   **`src/opm.rs`**: C言語で記述されたYM2151エミュレータ（`opm.c`）をRustから安全に呼び出すためのラッパーとして機能します。
-   **`src/opm_ffi.rs`**: Rustの外部関数インターフェース (FFI) を利用し、C言語のYM2151エミュレータの関数やデータ構造をRust側で定義します。
-   **`src/player.rs`**: JSON形式の音楽データからYM2151レジスタイベントを読み込み、YM2151エミュレータにリアルタイムで送り、音声データを生成・再生する中核となるプレイヤーロジックを実装します。
-   **`src/resampler.rs`**: YM2151エミュレータから出力されるオーディオデータのサンプリングレートを、ターゲットとなるオーディオデバイスのサンプリングレートに合わせるためのリサンプリング処理を提供します。
-   **`src/server.rs`**: バックグラウンドで常駐し、クライアントからのリクエスト（音楽データのロード、再生開始/停止、状態取得など）を受け付け、YM2151のイベント再生を管理するサーバーロジックを実装します。
-   **`src/wav_writer.rs`**: YM2151エミュレータから生成されたオーディオデータを、WAV形式のファイルとして保存する機能を提供します。
-   **`tests/`**: プロジェクトのテストコードとテストデータが格納されています。
    -   **`tests/fixtures/`**: テストに使用されるJSON形式の音楽データサンプル（`complex.json`, `simple.json`）が格納されています。
    -   その他`client_test.rs`、`integration_test.rs`などは、各コンポーネントやシステム全体の挙動を検証するテストケースを記述したファイルです。

## 関数詳細説明
このプロジェクトでは、以下の主要な機能を持つ関数群が各ファイルに実装されていると推測されます。具体的な関数名やシグネチャは提供されていませんが、それぞれの役割に基づいて説明します。

-   **`main`関数 (src/main.rs)**: アプリケーションのエントリポイント。コマンドライン引数を解析し、アプリケーションをスタンドアロン再生モード、サーバーモード、またはクライアントモードのいずれかで起動する役割を担います。
-   **クライアント関連関数 (src/client.rs)**: サーバーとの接続を確立し、音楽データや制御コマンドを送信するための関数群（例: `connect_to_server`, `send_command`, `receive_response`など）。
-   **サーバー関連関数 (src/server.rs)**: クライアントからの接続リクエストを受け入れ、送られてきたコマンドを解析し、音楽再生を管理するための関数群（例: `start_server`, `handle_client_connection`, `process_command`など）。
-   **プレイヤー関連関数 (src/player.rs)**: JSON音楽データを読み込み、YM2151イベントシーケンスを処理して、リアルタイムでYM2151エミュレータにレジスタ値を送信し、オーディオデータを生成するコアロジックを担う関数群（例: `load_music_data`, `play_sequence`, `generate_audio_frame`など）。
-   **オーディオ出力関数 (src/audio.rs)**: システムのオーディオデバイスと連携し、YM2151エミュレータから生成された音声バッファをリアルタイムで出力するための関数群（例: `init_audio_stream`, `write_audio_buffer`など）。
-   **WAV書き出し関数 (src/wav_writer.rs)**: 生成されたオーディオデータをWAVファイル形式で保存するための関数群（例: `create_wav_file`, `write_wav_header`, `append_audio_data`など）。
-   **YM2151エミュレータFFI関数 (src/opm.rs, src/opm_ffi.rs)**: C言語で実装されたYM2151エミュレータの機能をRustから呼び出すための低レベルなインターフェース関数群（例: `opm_init`, `opm_write_register`, `opm_mix_stereo`など）。
-   **IPC通信関数 (src/ipc/pipe_windows.rs)**: Windowsの名前付きパイプを介してデータを送受信するための、プラットフォーム固有のIPC関連関数群（例: `create_named_pipe`, `connect_pipe`, `read_pipe`, `write_pipe`など）。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2025-11-09 07:02:06 JST
