# ym2151-log-play-server

YM2151（OPM）レジスタイベントログを受け取り、リアルタイム再生を行うサーバー

## 対象プラットフォーム

- Windows専用
- Linux専用codeの禁止
    - 当projectにおいてはハルシネーションの増大が認められたため、
        - Linux専用codeを禁止します

## 概要

このプロジェクトは、YM2151（OPM）音源チップのレジスタイベントログを再生するプログラムです。
スタンドアロンモードとサーバー・クライアントモードの両方で動作します。

### 主な機能

- ✅ JSONイベントログの読み込みと再生
- ✅ リアルタイム音声出力
- ✅ WAVファイル出力
- ✅ サーバー・クライアントモードによるリモート制御
- ✅ 名前付きパイプ（FIFO）によるプロセス間通信

## 使い方

### スタンドアロンモード（通常の再生）

JSONファイルを直接再生：

```bash
# ビルドして実行
cargo run --release output_ym2151.json

# または既にビルドされたバイナリを使用
./target/release/ym2151-log-play-server output_ym2151.json
```

### サーバー・クライアントモード

#### サーバーの起動

サーバーとして常駐し、JSONを演奏開始：

```bash
# サーバーモードで起動（output_ym2151.jsonを演奏）
cargo run --release -- --server output_ym2151.json

# または
./target/release/ym2151-log-play-server --server output_ym2151.json
```

サーバーは `/tmp/ym2151_server.pipe` に名前付きパイプを作成し、クライアントからの接続を待機します。

#### クライアントからの操作

別のターミナルから、クライアントモードで操作：

```bash
# 新しいJSONファイルを再生（演奏を切り替え）
cargo run --release -- --client test_input.json

# 演奏を停止（無音化）
cargo run --release -- --client --stop

# サーバーをシャットダウン
cargo run --release -- --server --shutdown
```

### コマンドライン引数一覧

```
使用方法:
  ym2151-log-play-server <json_log_file>           # スタンドアロンモード
  ym2151-log-play-server --server <json_log_file>  # サーバーモード
  ym2151-log-play-server --server --shutdown       # サーバーシャットダウン
  ym2151-log-play-server --client <json_log_file>  # 新規JSONを演奏
  ym2151-log-play-server --client --stop           # 演奏停止

オプション:
  --server <file>    サーバーとして常駐し、指定されたJSONを演奏
  --server --shutdown サーバーをシャットダウン
  --client <file>    サーバーに新しいJSONファイルの演奏を指示
  --client --stop    サーバーに演奏停止を指示

例:
  # スタンドアロンで再生
  ym2151-log-play-server output_ym2151.json

  # サーバー起動
  ym2151-log-play-server --server output_ym2151.json

  # 別のターミナルから: 演奏を切り替え
  ym2151-log-play-server --client test_input.json

  # 別のターミナルから: 演奏停止
  ym2151-log-play-server --client --stop

  # 別のターミナルから: サーバー終了
  ym2151-log-play-server --server --shutdown
```

### 使用例シナリオ

#### シナリオ1: 基本的な使用

```bash
# ターミナル1: サーバー起動
$ cargo run --release -- --server output_ym2151.json
サーバーを起動しました: /tmp/ym2151_server.pipe
output_ym2151.json (3 イベント) を読み込みました
演奏を開始しました...

# ターミナル2: クライアントから操作
$ cargo run --release -- --client test_input.json
✅ サーバーに PLAY コマンドを送信しました

$ cargo run --release -- --client --stop
✅ サーバーに STOP コマンドを送信しました

$ cargo run --release -- --server --shutdown
✅ サーバーに SHUTDOWN コマンドを送信しました
```

#### シナリオ2: 連続再生

```bash
# サーバー起動（ターミナル1）
$ cargo run --release -- --server music1.json

# 次々と曲を切り替え（ターミナル2）
$ cargo run --release -- --client music2.json
$ sleep 5
$ cargo run --release -- --client music3.json
$ sleep 5
$ cargo run --release -- --client music1.json
```

### リリースビルド

```bash
cargo build --release
./target/release/ym2151-log-play-server output_ym2151.json
./target/release/ym2151-log-play-server --server output_ym2151.json
./target/release/ym2151-log-play-server --client output_ym2151.json
./target/release/ym2151-log-play-server --client stop
./target/release/ym2151-log-play-server --client --shutdown
```

### テストの実行

```bash
cargo test
```

## ビルド要件

- Rust 1.70以降
- zig cc（Cコンパイラとして使用）

## ライセンス

MIT License

## 利用ライブラリ

- Nuked-OPM: LGPL 2.1
- その他のRustクレート: 各クレートのライセンスに従う
