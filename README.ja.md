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

### インタラクティブモード（リアルタイムレジスタストリーミング）

インタラクティブモードは、リアルタイムレジスタ書き込みによる連続的な音声ストリーミングを可能にします。トーンエディタなど、即座の音声フィードバックが必要で、再生の空白時間を避けたいアプリケーションに最適です。

#### 基本的なインタラクティブモード

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // サーバー準備確認
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // インタラクティブモード開始
    client::start_interactive()?;
    
    // タイミング指定してレジスタ書き込み（秒単位、f64）
    client::write_register(0.0, 0x08, 0x00)?;     // 即座: 全チャンネルキーオフ
    client::write_register(0.050, 0x28, 0x48)?;   // +50ms: 音程設定
    client::write_register(0.050, 0x08, 0x78)?;   // +50ms: チャンネル0キーオン
    client::write_register(0.500, 0x08, 0x00)?;   // +500ms: キーオフ
    
    // 精密な同期のためサーバー時刻を取得
    let server_time = client::get_server_time()?;
    println!("サーバー時刻: {:.6} 秒", server_time);
    
    // インタラクティブモード停止
    client::stop_interactive()?;
    
    Ok(())
}
```

#### JSONデータを使用したインタラクティブモード（便利関数）

すでにym2151log形式のJSONデータを持つクライアントアプリケーションのために、`play_json_interactive()` 便利関数は変換やタイミングロジックを手動で実装する必要性を排除します。この関数はJSONの解析とレジスタ書き込みのみを行い、インタラクティブモードのライフサイクルはユーザーが制御します：

```rust
use ym2151_log_play_server::client;

fn main() -> anyhow::Result<()> {
    // サーバー準備確認
    client::ensure_server_ready("ym2151-log-play-server")?;
    
    // インタラクティブモードを一度開始
    client::start_interactive()?;
    
    // 複数のJSONを停止せずに送信 - 音の途切れなし！
    let json1 = r#"{
        "event_count": 2,
        "events": [
            {"time": 0, "addr": "0x08", "data": "0x00"},
            {"time": 2797, "addr": "0x28", "data": "0x48"}
        ]
    }"#;
    client::play_json_interactive(json1)?;
    
    let json2 = r#"{
        "event_count": 1,
        "events": [
            {"time": 5594, "addr": "0x08", "data": "0x78"}
        ]
    }"#;
    client::play_json_interactive(json2)?;
    
    // 再生完了待機
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // 完了時にインタラクティブモード停止
    client::stop_interactive()?;
    
    Ok(())
}
```

**主な特徴：**
- **連続ストリーミング**: 音声が途切れず、パラメータ変更時の無音時間を排除
- **レイテンシ補正**: ジッタ補正のための50msバッファ（Web Audioスタイルのスケジューリング）
- **サンプル精度のタイミング**: Float64秒（Web Audio API互換）で1/55930秒（1サンプル）までの精度を提供
- **サーバー時刻同期**: `get_server_time()` でサーバーの時間座標系を取得し、精密なスケジューリングが可能
- **WAV出力なし**: ファイルI/Oオーバーヘッドなしでリアルタイム用に最適化
- **便利関数**: `play_json_interactive()` がインタラクティブモードのライフサイクル管理なしでJSONの解析と時間変換を処理

**メリット：**
- トーンエディタ（例：ym2151-tone-editor）で即座の音声フィードバック
- 再生中断なしでのスムーズなパラメータ変更
- 音の途切れなく複数のJSONを連続送信可能
- 静的イベントログ再生と比較して低レイテンシ
- クロスプラットフォームの一貫性のためWeb Audio互換の時間表現
- クライアントがインタラクティブモードの開始/停止を制御

完全な例は `examples/interactive_demo.rs` と `examples/play_json_interactive_demo.rs` を参照してください。

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
cargo run --release -- client test_input.json

# 詳細モードで新しいJSONファイルを再生
cargo run --release -- client test_input.json --verbose

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
  ym2151-log-play-server client test_input.json

  # 別のターミナルから: 詳細モードで演奏
  ym2151-log-play-server client test_input.json --verbose

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
サーバーを起動しました: \pipe\ym2151-log-play-server.pipe
サーバーが起動しました。クライアントからの接続を待機中...

# ターミナル2: クライアントから操作
$ cargo run --release -- client test_input.json
✅ サーバーに演奏コマンドを送信しました

$ cargo run --release -- client --stop
✅ サーバーに停止コマンドを送信しました

$ cargo run --release -- client --shutdown
✅ サーバーにシャットダウンコマンドを送信しました
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
- zig cc（Cコンパイラとして使用）

## 今後の展望
- 現状は落ち着いている認識
- 必要なものが見つかり次第実装

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
