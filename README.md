# ym2151-log-play-server

YM2151（OPM）レジスタイベントログを受け取り、リアルタイム再生を行うサーバー、を目指して開発中です

## 状況

まだコピー元のままです。

これから実装します。

## 次のタスク

### 機能追加
- コマンドライン引数 `--server` `--client` `--stop` `--shutdown` を受け取れるようにします
- `--server sample_events.json` : サーバーとして常駐し、このjsonを演奏開始します。同時に名前付きパイプでクライアントからの接続を待機します
- `--server --shutdown` : サーバーを演奏stopし、シャットダウンします
- `--client sample_events.json` : sample_events.json を名前付きパイプでサーバーにわたします。サーバーは演奏stopし、このjsonを演奏します
- `--client --stop` : サーバーに名前付きパイプで接続して「演奏をstopしてください」というメッセージを送信します
- ※上記は検証を素早く進めるため、仮です。あとから破壊的変更します

## 移行は古い情報です

## 使い方

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
