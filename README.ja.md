# ym2151-log-play-server

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

YM2151（OPM）レジスタイベントログを受け取り、リアルタイム再生を行うサーバー・クライアント。Rustで書かれています。

## 対象プラットフォーム

- Windows専用
- Linux専用codeの禁止
    - 当projectにおいてはハルシネーションの増大が認められたため、
        - Linux専用codeを禁止します

## 開発状況

ライブラリとして、`cat-play-mml`や`ym2151-tone-editor`に組み込んで使っています。

頻繁に破壊的変更があります。特にclient-serverプロトコルとserver動作モードについて。

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

## クライアント実装ガイド

このセクションでは、2つの主要なクライアント実装パターンについて説明します。

### パターン1: 非インタラクティブモード

非インタラクティブモードは、単発のJSONデータ送信に適したシンプルなモードです。
各JSON送信ごとに演奏が停止・再開されます。

#### 基本的な使用方法

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // サーバーの準備を確認（必要に応じて自動的にインストールと起動）
    client::ensure_server_ready("your-app-name")?;
    
    // JSONデータを送信（演奏開始）
    let json_data = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x00"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::send_json(json_data)?;
    
    // 必要に応じて演奏制御
    std::thread::sleep(std::time::Duration::from_secs(5));
    client::stop_playback()?;
    
    // 別のJSONを再生
    let json_data2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::send_json(json_data2)?;
    
    // 終了時にシャットダウン
    client::shutdown_server()?;
    
    Ok(())
}
```

#### 特徴
- **シンプル**: 各JSONは独立して処理
- **演奏の切り替え**: JSON送信ごとに前の演奏は自動停止
- **間隔あり**: JSON間に短い無音期間が発生する可能性
- **用途**: 楽曲の切り替えや、連続性を重視しない用途、WAV保存用（verboseモード）

### パターン2: インタラクティブモード

インタラクティブモードは、リアルタイムな音響制御に適した高度なモードです。
連続した音声ストリームを維持しながら、レジスタイベントを動的にスケジューリングできます。

#### 基本的な使用方法

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // サーバーの準備
    client::ensure_server_ready("your-app-name")?;
    
    // インタラクティブモード開始（連続音声ストリーム開始）
    client::start_interactive()?;
    
    // 複数のJSONを無音ギャップなしで送信
    let phrase1 = r#"{"event_count": 2, "events": [
        {"time": 0, "addr": "0x08", "data": "0x78"},
        {"time": 2797, "addr": "0x20", "data": "0xC7"}
    ]}"#;
    client::play_json_interactive(phrase1)?;
    
    // フレーズの途中で別のフレーズに切り替え（音響ギャップなし）
    client::clear_schedule()?; // 未来のイベントをキャンセル
    let phrase2 = r#"{"event_count": 1, "events": [
        {"time": 1000, "addr": "0x28", "data": "0x3E"}
    ]}"#;
    client::play_json_interactive(phrase2)?;
    
    // サーバー時刻の同期取得（Web Audioの currentTime 相当）
    let server_time = client::get_server_time()?;
    println!("現在のサーバー時刻: {:.6}秒", server_time);
    
    // インタラクティブモード終了
    client::stop_interactive()?;
    
    Ok(())
}
```

#### 高度な機能

**スケジュールクリア機能**
```rust
// フレーズ1を開始
client::play_json_interactive(phrase1_json)?;

// フレーズ1の途中でフレーズ2に無音ギャップなしで切り替え
client::clear_schedule()?; // まだ処理されていないイベントをクリア
client::play_json_interactive(phrase2_json)?; // 新しいフレーズを即座にスケジューリング
```

**サーバー時刻同期**
```rust
// 正確なタイミング制御のためのサーバー時刻取得
let current_time = client::get_server_time()?;
// Web Audioの currentTime プロパティと同等の機能
```

#### 特徴
- **連続性**: 音声ストリームが途切れない
- **リアルタイム制御**: イベントの動的スケジューリング
- **無音ギャップなし**: フレーズ間の切り替えが滑らか
- **時刻同期**: サーバーとの正確なタイミング制御
- **用途**: リアルタイム音楽制御、音色エディタ、ライブパフォーマンス

#### タイミング変換
インタラクティブモードでは、ym2151logフォーマット（サンプル単位、55930 Hz）のJSONを自動的にf64秒単位に変換してサーバーに送信します：

```rust
// 入力: サンプル単位（i64、55930 Hz）
let input_json = r#"{"event_count": 1, "events": [
    {"time": 2797, "addr": "0x08", "data": "0x00"}  // 2797サンプル = 約0.05秒
]}"#;

// 内部で自動変換: f64秒単位
// {"time": 0.050027, ...} としてサーバーに送信
client::play_json_interactive(input_json)?;
```

### サーバー・クライアントモード

#### サーバーの起動

サーバーとして常駐し、待機状態で起動：

```bash
# 通常モード（ログファイルのみ）
cargo run --release -- server

# verbose モード（詳細ログとWAV出力）
cargo run --release -- server --verbose

# 低品位リサンプリングモード（比較用）
cargo run --release -- server --low-quality-resampling

# verbose + 低品位リサンプリング
cargo run --release -- server --verbose --low-quality-resampling
```

#### クライアントからの操作

別のターミナルから、クライアントモードで操作：

```bash
# 新しいJSONファイルを再生（演奏を切り替え）
cargo run --release -- client output_ym2151.json

# 詳細モードで新しいJSONファイルを再生
cargo run --release -- client output_ym2151.json --verbose

# 演奏を停止（無音化）
cargo run --release -- client --stop

# サーバーをシャットダウン
cargo run --release -- client --shutdown
```

### コマンドライン引数一覧

```
使用方法:
  ym2151-log-play-server server [OPTIONS]           # サーバーモード
  ym2151-log-play-server client [OPTIONS] [FILE]    # クライアントモード

サーバーモード:
  server                    サーバーとして待機状態で起動
  server --verbose          詳細ログモードで起動（WAVファイルを出力）
  server --low-quality-resampling  低品位リサンプリングを使用（線形補間、比較用）

クライアントモード:
  client <json_file>        サーバーに新しいJSONファイルの演奏を指示
  client <json_file> --verbose  詳細な状態メッセージ付きで演奏を指示
  client --stop             サーバーに演奏停止を指示
  client --stop --verbose   詳細な状態メッセージ付きで演奏を停止
  client --shutdown         サーバーにシャットダウンを指示
  client --shutdown --verbose  詳細な状態メッセージ付きでサーバーをシャットダウン

例:
  # サーバー起動
  ym2151-log-play-server server

  # サーバー起動（verbose、WAV出力あり）
  ym2151-log-play-server server --verbose

  # サーバー起動（低品位リサンプリング）
  ym2151-log-play-server server --low-quality-resampling

  # 別のターミナルから: 演奏を切り替え
  ym2151-log-play-server client output_ym2151.json

  # 別のターミナルから: 詳細モードで演奏
  ym2151-log-play-server client output_ym2151.json --verbose

  # 別のターミナルから: 演奏停止
  ym2151-log-play-server client --stop

  # 別のターミナルから: サーバー終了
  ym2151-log-play-server client --shutdown
```

### 使用例シナリオ

#### シナリオ1: 基本的な使用

```bash
# ターミナル1: サーバー起動
$ cargo run --release -- server

# ターミナル2: クライアントから操作
$ cargo run --release -- client output_ym2151.json

$ cargo run --release -- client --stop

$ cargo run --release -- client --shutdown
```

#### シナリオ2: 連続再生

```bash
# サーバー起動（ターミナル1）
$ cargo run --release -- server

# 次々と曲を切り替え（ターミナル2）
$ cargo run --release -- client music2.json
$ Start-Sleep 5
$ cargo run --release -- client music3.json
$ Start-Sleep 5
$ cargo run --release -- client music1.json
```

### リリースビルド

```bash
cargo build --release
.\target\release\ym2151-log-play-server.exe server
.\target\release\ym2151-log-play-server.exe server --verbose
.\target\release\ym2151-log-play-server.exe client output_ym2151.json
.\target\release\ym2151-log-play-server.exe client --stop
.\target\release\ym2151-log-play-server.exe client --shutdown
```

### テストの実行

```bash
cargo test
```

## ビルド要件

- Rust 1.70以降

## 今後の展望
- 破壊的変更中
  - jsonフォーマットを変更予定
  - レジスタ書き込み後の規定cycle消費の仕様を簡素化して最終段で一括してかけるよう変更予定
- //現状は落ち着いている認識
- //必要なものが見つかり次第実装

## プロジェクトが目指すもの
- モチベ：
  - これまでの課題：
    - 演奏終了まで次のコマンドが入力できない（ym2151-log-player-rust）
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

## プロジェクトが目指さないもの（スコープ外）
- 高速化。開発しやすさを犠牲にし、スピードを追求。どんな環境でどんな高負荷をかけても音の途切れゼロ
- 高機能。開発しやすさを犠牲にし、あらゆる音楽データを入力し自動変換して演奏。複数YM2151を制御。MIDI入出力
- 高度な再現。開発しやすさを犠牲にし、あらゆるYM2151既存曲を完璧に再現して演奏

## プロジェクトの意図
- なぜこのようなモジュール分割をしたか？
  - ここより上のレイヤー（MML入力からlog生成まで）を、GitHub Linux RunnerでGitHub Copilot Coding AgentがTDDできるようにするため。
  - このレイヤー（Windowsリアルタイム演奏と、Windowsクライアント・サーバー）は、GitHub Linux RunnerでGitHub Copilot Coding AgentがTDDできず、かわりにWindows localのagentによるTDDが必要なので、やや作業負荷が高い。
  - なので、作業負荷の高いこのレイヤーだけを切り分けて、ほかのレイヤーを効率的に開発できるようにするため。

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
- 関連アプリの一括インストール
    - 用途、開発用に便利
    - 前提、`cargo install rust-script`しておくこと
```powershell
rust-script install-ym2151-tools.rs
```

## ライセンス

MIT License

## 利用ライブラリ

- Nuked-OPM: LGPL 2.1
- その他のRustクレート: 各クレートのライセンスに従う
