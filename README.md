# ym2151-log-play-server

YM2151（OPM）レジスタイベントログを受け取り、リアルタイム再生を行うサーバー

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
cargo run --release sample_events.json

# または既にビルドされたバイナリを使用
./target/release/ym2151-log-player-rust sample_events.json
```

### サーバー・クライアントモード

#### サーバーの起動

サーバーとして常駐し、JSONを演奏開始：

```bash
# サーバーモードで起動（sample_events.jsonを演奏）
cargo run --release -- --server sample_events.json

# または
./target/release/ym2151-log-player-rust --server sample_events.json
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
  ym2151-log-player-rust <json_log_file>           # スタンドアロンモード
  ym2151-log-player-rust --server <json_log_file>  # サーバーモード
  ym2151-log-player-rust --server --shutdown       # サーバーシャットダウン
  ym2151-log-player-rust --client <json_log_file>  # 新規JSONを演奏
  ym2151-log-player-rust --client --stop           # 演奏停止

オプション:
  --server <file>    サーバーとして常駐し、指定されたJSONを演奏
  --server --shutdown サーバーをシャットダウン
  --client <file>    サーバーに新しいJSONファイルの演奏を指示
  --client --stop    サーバーに演奏停止を指示

例:
  # スタンドアロンで再生
  ym2151-log-player-rust sample_events.json

  # サーバー起動
  ym2151-log-player-rust --server sample_events.json

  # 別のターミナルから: 演奏を切り替え
  ym2151-log-player-rust --client test_input.json

  # 別のターミナルから: 演奏停止
  ym2151-log-player-rust --client --stop

  # 別のターミナルから: サーバー終了
  ym2151-log-player-rust --server --shutdown
```

### 使用例シナリオ

#### シナリオ1: 基本的な使用

```bash
# ターミナル1: サーバー起動
$ cargo run --release -- --server sample_events.json
サーバーを起動しました: /tmp/ym2151_server.pipe
sample_events.json (3 イベント) を読み込みました
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

### 注意事項

⚠️ **暫定実装について**

サーバー・クライアント機能は検証用の暫定仕様です。将来的に破壊的変更が行われる可能性があります。

- 現在の実装はシンプルさを優先
- エラーハンドリングは最小限
- Unix/Linux環境のみサポート（Windowsは未実装）

### CI/ヘッドレス環境での実行

音声デバイスが利用できない環境（CI/ヘッドレス環境）では、ALSA設定ファイルを使用して音声出力をファイルにリダイレクトできます：

```bash
# ALSA設定ファイルを作成
cat <<'EOF' > ~/.asoundrc
pcm.!default {
  type file
  slave.pcm "null"
  file "/tmp/alsa_capture.wav"
  format "wav"
}
EOF

# 通常通りプログラムを実行
cargo run --release sample_events.json
```

この設定により、音声デバイスなしでもプログラムが正常に動作します。
音声出力は `/tmp/alsa_capture.wav` に保存され、同時に `output.wav` も生成されます。

## トラブルシューティング

### サーバー・クライアント関連

#### クライアントが "Failed to connect to server" エラーを出す

**原因:** サーバーが起動していない、または名前付きパイプが作成されていない

**解決策:**
1. サーバーが起動しているか確認：`ps aux | grep ym2151`
2. 名前付きパイプが存在するか確認：`ls -l /tmp/ym2151_server.pipe`
3. サーバーを起動してから再度クライアントを実行

#### サーバーが応答しない

**原因:** 名前付きパイプがブロックされている、または古いパイプが残っている

**解決策:**
1. サーバーを終了
2. 古いパイプを削除：`rm /tmp/ym2151_server.pipe`
3. サーバーを再起動

#### 複数のサーバーを起動してしまった

**原因:** 既存のサーバーが動作中に新しいサーバーを起動

**解決策:**
1. 全てのサーバープロセスを確認：`ps aux | grep ym2151`
2. プロセスを終了：`kill <PID>` または `pkill ym2151`
3. パイプをクリーンアップ：`rm /tmp/ym2151_server.pipe`

### 音声関連

#### "Unknown PCM default" エラー（ALSA）

**原因:** 音声デバイスが利用できない（CI/ヘッドレス環境など）

**解決策:** 上記の「CI/ヘッドレス環境での実行」を参照

#### 音が出ない

**確認事項:**
1. 音量設定を確認
2. `output.wav` ファイルが生成されているか確認
3. WAVファイルを別のプレイヤーで再生して確認
4. サーバーモードの場合、演奏が停止状態になっていないか確認

### ビルド関連

#### "alsa-sys was not found" エラー

**原因:** ALSA開発ライブラリがインストールされていない

**解決策:**
```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel
```

### コマンドライン引数

```
使用方法:
  ym2151-log-player-rust <json_log_file>

例:
  ym2151-log-player-rust sample_events.json
  ym2151-log-player-rust events.json
```

### JSONイベントログファイル形式

```json
{
  "event_count": 100,
  "events": [
    {"time": 0, "addr": "0x08", "data": "0x00"},
    {"time": 2, "addr": "0x20", "data": "0xC7"}
  ]
}
```

- `event_count`: イベント総数
- `events`: イベント配列
  - `time`: サンプル時刻（絶対時刻）
  - `addr`: YM2151レジスタアドレス（16進数文字列）
  - `data`: レジスタに書き込むデータ（16進数文字列）
  - `is_data`: （オプション、読み込み時は無視されます）

**注意:** プログラムは入力イベントを自動的に2段階（アドレス書き込み→データ書き込み）に分割し、必要な遅延を挿入します。

### ビルド要件

**注意:** リアルタイムオーディオ再生（デフォルト）には音声出力デバイスが必要です。
Linux環境では、ALSA開発ライブラリのインストールが必要です：

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel
```

### リリースビルド

```bash
cargo build --release
./target/release/ym2151-log-player-rust sample_events.json
```

### テストの実行

```bash
# 標準テスト（realtime-audio機能込み）
cargo test
```

## ビルド要件

- Rust 1.70以降
- zig cc（Cコンパイラとして使用）
- （オプション）ALSA開発ライブラリ（Linux環境でrealtime-audio機能を使用する場合）

**詳細なビルド手順は [BUILD.md](BUILD.md) を参照してください。**

その他の詳細は [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) を参照してください。

## ライセンス

MIT License

## 利用ライブラリ

- Nuked-OPM: LGPL 2.1
- その他のRustクレート: 各クレートのライセンスに従う
