# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151（OPM）レジスタイベントログを受け取り、リアルタイム再生を行うサーバー・クライアント

## 対象プラットフォーム

- Windows専用
- Linux専用codeの禁止
    - 当projectにおいてはハルシネーションの増大が認められたため、
        - Linux専用codeを禁止します

## 状況

ライブラリとして、`cat-play-mml`や`ym2151-tone-editor`に組み込んで使っています。

## 概要

このプロジェクトは、YM2151（OPM）音源チップのレジスタイベントログを再生するプログラムです。
サーバー・クライアントモードで動作します。

### 主な機能

- JSON音楽データをリアルタイム演奏
- WAVファイル出力（verbose時）
- サーバーとして常駐し、バックグラウンドでリアルタイム演奏を続ける
- クライアントから制御し、素早く別の演奏に切り替え
- サーバー・クライアント通信に名前付きパイプを利用

## 使い方

### ライブラリとしての使用（プログラマティック制御）

このライブラリをプログラムから使用する場合の推奨パターン：

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // サーバーの準備を確認（必要に応じて自動的にインストールと起動）
    client::ensure_server_ready("cat-play-mml")?;
    
    // JSONデータを送信
    let json_data = r#"{"event_count": 2, "events": [...]}"#;
    client::send_json(json_data)?;
    
    // 再生制御
    client::stop_playback()?;
    
    // 終了時にシャットダウン
    client::shutdown_server()?;
    
    Ok(())
}
```

`ensure_server_ready()` 関数は以下のことを自動的に行い、シームレスな開発体験を提供します：
1. サーバーが既に起動しているか確認
2. PATHにサーバーアプリケーションが見つからない場合、cargo経由でインストール
3. サーバーをバックグラウンドモードで起動
4. サーバーがコマンドを受け付けられる状態になるまで待機

これにより、ライブラリユーザーがサーバーのライフサイクルを手動で管理する必要がなくなります。

### サーバー・クライアントモード

#### サーバーの起動

サーバーとして常駐し、待機状態で起動：

```bash
# 通常モード（ログファイルのみ）
cargo run --release -- server

# verbose モード（詳細ログとWAV出力）
cargo run --release -- server --verbose
```

#### クライアントからの操作

別のターミナルから、クライアントモードで操作：

```bash
# 新しいJSONファイルを再生（演奏を切り替え）
cargo run --release -- client test_input.json

# 演奏を停止（無音化）
cargo run --release -- client --stop

# サーバーをシャットダウン
cargo run --release -- client --shutdown
```

### コマンドライン引数一覧

```
使用方法:
  ym2151-log-play-server server [--verbose]         # サーバーモード
  ym2151-log-play-server client <json_log_file>     # 新規JSONを演奏
  ym2151-log-play-server client --stop              # 演奏停止
  ym2151-log-play-server client --shutdown          # サーバーシャットダウン

オプション:
  server           サーバーとして待機状態で起動
  server --verbose サーバーを詳細ログモードで起動（WAVファイルを出力）
  client <file>    サーバーに新しいJSONファイルの演奏を指示
  client --stop    サーバーに演奏停止を指示
  client --shutdown サーバーにシャットダウンを指示

例:
  # サーバー起動
  ym2151-log-play-server server

  # サーバー起動（verbose、WAV出力あり）
  ym2151-log-play-server server --verbose

  # 別のターミナルから: 演奏を切り替え
  ym2151-log-play-server client test_input.json

  # 別のターミナルから: 演奏停止
  ym2151-log-play-server client --stop

  # 別のターミナルから: サーバー終了
  ym2151-log-play-server --client --shutdown
```

### 使用例シナリオ

#### シナリオ1: 基本的な使用

```bash
# ターミナル1: サーバー起動
$ cargo run --release -- --server
サーバーを起動しました: /tmp/ym2151-log-play-server.pipe
サーバーが起動しました。クライアントからの接続を待機中...

# ターミナル2: クライアントから操作
$ cargo run --release -- --client test_input.json
✅ サーバーに PLAY コマンドを送信しました

$ cargo run --release -- --client --stop
✅ サーバーに STOP コマンドを送信しました

$ cargo run --release -- --client --shutdown
✅ サーバーに SHUTDOWN コマンドを送信しました
```

#### シナリオ2: 連続再生

```bash
# サーバー起動（ターミナル1）
$ cargo run --release -- --server

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
./target/release/ym2151-log-play-server --server
./target/release/ym2151-log-play-server --client output_ym2151.json
./target/release/ym2151-log-play-server --client --stop
./target/release/ym2151-log-play-server --client --shutdown
```

### テストの実行

```bash
cargo test
```

## ビルド要件

- Rust 1.70以降
- zig cc（Cコンパイラとして使用）

## 今後の展望
- 現状は落ち着いている認識
- 必要なものが見つかり次第実装

## プロジェクトが目指すもの
- モチベ：
  - これまでの課題：
    - 演奏終了まで次のコマンドが入力できない
  - 対策：
    - サーバとして常駐し、クライアントから制御する
  - 用途：
    - MSXのPLAY文のように、演奏しながら次のコマンドを入力できる体験を提供
    - 音色エディタ、フレーズエディタから、
      - クライアントとしてクレートを利用
    - playerにクレートを組み込み、サーバ兼クライアントにする
      - 初回は自分の複製をバックグラウンドでサーバとして起動して演奏開始し、自分は終了
        - ※明示的にサーバとして使う場合と異なり、printのかわりにlogに文言を出力する構想、logあったほうが把握しやすい
      - サーバ起動したあとは、クライアントとしてサーバにJSONを投げて、自分は終了
- シンプルでミニマム。より大規模なものを作るときに参考にしやすい用
- もし鳴らなくなったら、できるだけ優先して鳴るよう行動するつもり

## プロジェクトの意図
- なぜこのようなモジュール分割をしたか？
  - ここより上のレイヤー（MML入力からlog生成まで）を、GitHub Linux RunnerでGitHub Copilot Coding AgentがTDDできるようにするため。
  - このレイヤー（Windowsリアルタイム演奏と、Windowsクライアント・サーバー）は、GitHub Linux RunnerでGitHub Copilot Coding AgentがTDDできず、かわりにWindows localのagentによるTDDが必要なので、やや作業負荷が高い。
  - なので、作業負荷の高いこのレイヤーだけを切り分けて、ほかのレイヤーを効率的に開発できるようにするため。

## スコープ外
- 高度な機能
- 既存曲の再現

## 開発方法
- WindowsでagentにTDD
- このプロジェクトに限ってはLinux禁止
  - なぜなら、
    - 序盤で、実質Linux専用のcodeが生成された
      - Windows版の土台には役立ったかも
    - Unix/Linux/Windows分岐、realtime-audio有無の分岐、ほか分岐、それらに付随する大量のコメント、
      - でcode肥大してハルシネーションの温床となった
      - 低品質codeになり、ムダなallow deadcode、testのignored、重複test、ムダなcfg windows分岐なども多かった
      - ハルシネーション多発し、バグ修正や、Windows版の機能実装ができなくなった
    - このプロジェクトならWindowsでのagentのTDDがよく機能することが判明した
      - 上記のハルシネーションやムダも、TDDを利用した堅牢なリファクタリングで解決できた

## ライセンス

MIT License

## 利用ライブラリ

- Nuked-OPM: LGPL 2.1
- その他のRustクレート: 各クレートのライセンスに従う
