Last updated: 2025-11-16


# プロジェクト概要生成プロンプト（来訪者向け）

## 生成するもの：
- projectを3行で要約する
- プロジェクトで使用されている技術スタックをカテゴリ別に整理して説明する
- プロジェクト全体のファイル階層ツリー（ディレクトリ構造を図解）
- プロジェクト全体のファイルそれぞれの説明
- プロジェクト全体の関数それぞれの説明
- プロジェクト全体の関数の呼び出し階層ツリー

## 生成しないもの：
- Issues情報（開発者向け情報のため）
- 次の一手候補（開発者向け情報のため）
- ハルシネーションしそうなもの（例、存在しない機能や計画を勝手に妄想する等）

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Project Overview

## プロジェクト概要
[以下の形式で3行でプロジェクトを要約]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 技術スタック
[使用している技術をカテゴリ別に整理して説明]
- フロントエンド: [フロントエンド技術とその説明]
- 音楽・オーディオ: [音楽・オーディオ関連技術とその説明]
- 開発ツール: [開発支援ツールとその説明]
- テスト: [テスト関連技術とその説明]
- ビルドツール: [ビルド・パース関連技術とその説明]
- 言語機能: [言語仕様・機能とその説明]
- 自動化・CI/CD: [自動化・継続的統合関連技術とその説明]
- 開発標準: [コード品質・統一ルール関連技術とその説明]

## ファイル階層ツリー
```
[プロジェクトのディレクトリ構造をツリー形式で表現]
```

## ファイル詳細説明
[各ファイルの役割と機能を詳細に説明]

## 関数詳細説明
[各関数の役割、引数、戻り値、機能を詳細に説明]

## 関数呼び出し階層ツリー
```
[関数間の呼び出し関係をツリー形式で表現]
```
```


以下のプロジェクト情報を参考にして要約を生成してください：

## プロジェクト情報
名前: 
説明: # ym2151-log-play-server

YM2151（OPM）レジスタイベントログを受け取り、リアルタイム再生を行うサーバー・クライアント

## 対象プラットフォーム

- Windows専用
- Linux専用codeの禁止
    - 当projectにおいてはハルシネーションの増大が認められたため、
        - Linux専用codeを禁止します

## 概要

このプロジェクトは、YM2151（OPM）音源チップのレジスタイベントログを再生するプログラムです。
スタンドアロンモードとサーバー・クライアントモードの両方で動作します。

### 主な機能

- JSON音楽データをリアルタイム演奏
- WAVファイル出力
- サーバーとして常駐し、バックグラウンドでリアルタイム演奏を続ける
- クライアントから制御し、素早く別の演奏に切り替え
- サーバー・クライアント通信に名前付きパイプを利用

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

サーバーとして常駐し、待機状態で起動：

```bash
cargo run --release -- --server
```

#### クライアントからの操作

別のターミナルから、クライアントモードで操作：

```bash
# 新しいJSONファイルを再生（演奏を切り替え）
cargo run --release -- --client test_input.json

# 演奏を停止（無音化）
cargo run --release -- --client --stop

# サーバーをシャットダウン
cargo run --release -- --client --shutdown
```

### コマンドライン引数一覧

```
使用方法:
  ym2151-log-play-server <json_log_file>           # スタンドアロンモード
  ym2151-log-play-server --server                  # サーバーモード
  ym2151-log-play-server --client <json_log_file>  # 新規JSONを演奏
  ym2151-log-play-server --client --stop           # 演奏停止
  ym2151-log-play-server --client --shutdown       # サーバーシャットダウン

オプション:
  --server           サーバーとして待機状態で起動
  --client <file>    サーバーに新しいJSONファイルの演奏を指示
  --client --stop    サーバーに演奏停止を指示
  --client --shutdown サーバーにシャットダウンを指示

例:
  # スタンドアロンで再生
  ym2151-log-play-server output_ym2151.json

  # サーバー起動
  ym2151-log-play-server --server

  # 別のターミナルから: 演奏を切り替え
  ym2151-log-play-server --client test_input.json

  # 別のターミナルから: 演奏停止
  ym2151-log-play-server --client --stop

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


依存関係:
{}

## ファイル階層ツリー
📁 .cargo/
  📄 config.toml
📄 .editorconfig
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📁 _codeql_detected_source_root/
  📁 .cargo/
    📄 config.toml
  📄 .editorconfig
  📄 .gitignore
  📄 Cargo.lock
  📄 Cargo.toml
  📄 LICENSE
  📖 README.ja.md
  📖 README.md
  📁 _codeql_detected_source_root/
    📁 .cargo/
      📄 config.toml
    📄 .editorconfig
    📄 .gitignore
    📄 Cargo.lock
    📄 Cargo.toml
    📄 LICENSE
    📖 README.ja.md
    📖 README.md
    📁 _codeql_detected_source_root/
      📁 .cargo/
        📄 config.toml
      📄 .editorconfig
      📄 .gitignore
      📄 Cargo.lock
      📄 Cargo.toml
      📄 LICENSE
      📖 README.ja.md
      📖 README.md
      📁 _codeql_detected_source_root/
        📁 .cargo/
          📄 config.toml
        📄 .editorconfig
        📄 .gitignore
        📄 Cargo.lock
        📄 Cargo.toml
        📄 LICENSE
        📖 README.ja.md
        📖 README.md
        📁 _codeql_detected_source_root/
          📁 .cargo/
            📄 config.toml
          📄 .editorconfig
          📄 .gitignore
          📄 Cargo.lock
          📄 Cargo.toml
          📄 LICENSE
          📖 README.ja.md
          📖 README.md
          📁 _codeql_detected_source_root/
            📁 .cargo/
              📄 config.toml
            📄 .editorconfig
            📄 .gitignore
            📄 Cargo.lock
            📄 Cargo.toml
            📄 LICENSE
            📖 README.ja.md
            📖 README.md
            📁 _codeql_detected_source_root/
              📁 .cargo/
                📄 config.toml
              📄 .editorconfig
              📄 .gitignore
              📄 Cargo.lock
              📄 Cargo.toml
              📄 LICENSE
              📖 README.ja.md
              📖 README.md
              📁 _codeql_detected_source_root/
                📁 .cargo/
                  📄 config.toml
                📄 .editorconfig
                📄 .gitignore
                📄 Cargo.lock
                📄 Cargo.toml
                📄 LICENSE
                📖 README.ja.md
                📖 README.md
                📁 _codeql_detected_source_root/
                  📁 .cargo/
                    📄 config.toml
                  📄 .editorconfig
                  📄 .gitignore
                  📄 Cargo.lock
                  📄 Cargo.toml
                  📄 LICENSE
                  📖 README.ja.md
                  📖 README.md
                  📁 _codeql_detected_source_root/
                    📁 .cargo/
                      📄 config.toml
                    📄 .editorconfig
                    📄 .gitignore
                    📄 Cargo.lock
                    📄 Cargo.toml
                    📄 LICENSE
                    📖 README.ja.md
                    📖 README.md
                    📁 _codeql_detected_source_root/
                      📁 .cargo/
                        📄 config.toml
                      📄 .editorconfig
                      📄 .gitignore
                      📄 Cargo.lock
                      📄 Cargo.toml
                      📄 LICENSE
                      📖 README.ja.md
                      📖 README.md
                      📁 _codeql_detected_source_root/
                        📁 .cargo/
                          📄 config.toml
                        📄 .editorconfig
                        📄 .gitignore
                        📄 Cargo.lock
                        📄 Cargo.toml
                        📄 LICENSE
                        📖 README.ja.md
                        📖 README.md
                        📁 _codeql_detected_source_root/
                          📁 .cargo/
                            📄 config.toml
                          📄 .editorconfig
                          📄 .gitignore
                          📄 Cargo.lock
                          📄 Cargo.toml
                          📄 LICENSE
                          📖 README.ja.md
                          📖 README.md
                          📁 _codeql_detected_source_root/
                            📁 .cargo/
                              📄 config.toml
                            📄 .editorconfig
                            📄 .gitignore
                            📄 Cargo.lock
                            📄 Cargo.toml
                            📄 LICENSE
                            📖 README.ja.md
                            📖 README.md
                            📁 _codeql_detected_source_root/
                              📁 .cargo/
                                📄 config.toml
                              📄 .editorconfig
                              📄 .gitignore
                              📄 Cargo.lock
                              📄 Cargo.toml
                              📄 LICENSE
                              📖 README.ja.md
                              📖 README.md
                              📁 _codeql_detected_source_root/
                                📁 .cargo/
                                  📄 config.toml
                                📄 .editorconfig
                                📄 .gitignore
                                📄 Cargo.lock
                                📄 Cargo.toml
                                📄 LICENSE
                                📖 README.ja.md
                                📖 README.md
                                📁 _codeql_detected_source_root/
                                  📁 .cargo/
                                    📄 config.toml
                                  📄 .editorconfig
                                  📄 .gitignore
                                  📄 Cargo.lock
                                  📄 Cargo.toml
                                  📄 LICENSE
                                  📖 README.ja.md
                                  📖 README.md
                                  📁 _codeql_detected_source_root/
                                    📁 .cargo/
                                      📄 config.toml
                                    📄 .editorconfig
                                    📄 .gitignore
                                    📄 Cargo.lock
                                    📄 Cargo.toml
                                    📄 LICENSE
                                    📖 README.ja.md
                                    📖 README.md
                                    📁 _codeql_detected_source_root/
                                      📁 .cargo/
                                        📄 config.toml
                                      📄 .editorconfig
                                      📄 .gitignore
                                      📄 Cargo.lock
                                      📄 Cargo.toml
                                      📄 LICENSE
                                      📖 README.ja.md
                                      📖 README.md
                                      📁 _codeql_detected_source_root/
                                        📁 .cargo/
                                          📄 config.toml
                                        📄 .editorconfig
                                        📄 .gitignore
                                        📄 Cargo.lock
                                        📄 Cargo.toml
                                        📄 LICENSE
                                        📖 README.ja.md
                                        📖 README.md
                                        📁 _codeql_detected_source_root/
                                          📁 .cargo/
                                            📄 config.toml
                                          📄 .editorconfig
                                          📄 .gitignore
                                          📄 Cargo.lock
                                          📄 Cargo.toml
                                          📄 LICENSE
                                          📖 README.ja.md
                                          📖 README.md
                                          📁 _codeql_detected_source_root/
                                            📁 .cargo/
                                              📄 config.toml
                                            📄 .editorconfig
                                            📄 .gitignore
                                            📄 Cargo.lock
                                            📄 Cargo.toml
                                            📄 LICENSE
                                            📖 README.ja.md
                                            📖 README.md
                                            📁 _codeql_detected_source_root/
                                              📁 .cargo/
                                                📄 config.toml
                                              📄 .editorconfig
                                              📄 .gitignore
                                              📄 Cargo.lock
                                              📄 Cargo.toml
                                              📄 LICENSE
                                              📖 README.ja.md
                                              📖 README.md
                                              📁 _codeql_detected_source_root/
                                                📁 .cargo/
                                                  📄 config.toml
                                                📄 .editorconfig
                                                📄 .gitignore
                                                📄 Cargo.lock
                                                📄 Cargo.toml
                                                📄 LICENSE
                                                📖 README.ja.md
                                                📖 README.md
                                                📁 _codeql_detected_source_root/
                                                  📁 .cargo/
                                                    📄 config.toml
                                                  📄 .editorconfig
                                                  📄 .gitignore
                                                  📄 Cargo.lock
                                                  📄 Cargo.toml
                                                  📄 LICENSE
                                                  📖 README.ja.md
                                                  📖 README.md
                                                  📁 _codeql_detected_source_root/
                                                    📁 .cargo/
                                                      📄 config.toml
                                                    📄 .editorconfig
                                                    📄 .gitignore
                                                    📄 Cargo.lock
                                                    📄 Cargo.toml
                                                    📄 LICENSE
                                                    📖 README.ja.md
                                                    📖 README.md
                                                    📁 _codeql_detected_source_root/
                                                      📁 .cargo/
                                                        📄 config.toml
                                                      📄 .editorconfig
                                                      📄 .gitignore
                                                      📄 Cargo.lock
                                                      📄 Cargo.toml
                                                      📄 LICENSE
                                                      📖 README.ja.md
                                                      📖 README.md
                                                      📁 _codeql_detected_source_root/
                                                        📁 .cargo/
                                                          📄 config.toml
                                                        📄 .editorconfig
                                                        📄 .gitignore
                                                        📄 Cargo.lock
                                                        📄 Cargo.toml
                                                        📄 LICENSE
                                                        📖 README.ja.md
                                                        📖 README.md
                                                        📁 _codeql_detected_source_root/
                                                          📁 .cargo/
                                                            📄 config.toml
                                                          📄 .editorconfig
                                                          📄 .gitignore
                                                          📄 Cargo.lock
                                                          📄 Cargo.toml
                                                          📄 LICENSE
                                                          📖 README.ja.md
                                                          📖 README.md
                                                          📁 _codeql_detected_source_root/
                                                            📁 .cargo/
                                                              📄 config.toml
                                                            📄 .editorconfig
                                                            📄 .gitignore
                                                            📄 Cargo.lock
                                                            📄 Cargo.toml
                                                            📄 LICENSE
                                                            📖 README.ja.md
                                                            📖 README.md
                                                            📁 _codeql_detected_source_root/
                                                              📁 .cargo/
                                                                📄 config.toml
                                                              📄 .editorconfig
                                                              📄 .gitignore
                                                              📄 Cargo.lock
                                                              📄 Cargo.toml
                                                              📄 LICENSE
                                                              📖 README.ja.md
                                                              📖 README.md
                                                              📁 _codeql_detected_source_root/
                                                                📁 .cargo/
                                                                  📄 config.toml
                                                                📄 .editorconfig
                                                                📄 .gitignore
                                                                📄 Cargo.lock
                                                                📄 Cargo.toml
                                                                📄 LICENSE
                                                                📖 README.ja.md
                                                                📖 README.md
                                                                📁 _codeql_detected_source_root/
                                                                  📁 .cargo/
                                                                    📄 config.toml
                                                                  📄 .editorconfig
                                                                  📄 .gitignore
                                                                  📄 Cargo.lock
                                                                  📄 Cargo.toml
                                                                  📄 LICENSE
                                                                  📖 README.ja.md
                                                                  📖 README.md
                                                                  📁 _codeql_detected_source_root/
                                                                    📁 .cargo/
                                                                      📄 config.toml
                                                                    📄 .editorconfig
                                                                    📄 .gitignore
                                                                    📄 Cargo.lock
                                                                    📄 Cargo.toml
                                                                    📄 LICENSE
                                                                    📖 README.ja.md
                                                                    📖 README.md
                                                                    📁 _codeql_detected_source_root/
                                                                      📁 .cargo/
                                                                        📄 config.toml
                                                                      📄 .editorconfig
                                                                      📄 .gitignore
                                                                      📄 Cargo.lock
                                                                      📄 Cargo.toml
                                                                      📄 LICENSE
                                                                      📖 README.ja.md
                                                                      📖 README.md
                                                                      📁 _codeql_detected_source_root/
                                                                        📁 .cargo/
                                                                          📄 config.toml
                                                                        📄 .editorconfig
                                                                        📄 .gitignore
                                                                        📄 Cargo.lock
                                                                        📄 Cargo.toml
                                                                        📄 LICENSE
                                                                        📖 README.ja.md
                                                                        📖 README.md
                                                                        📁 _codeql_detected_source_root/
                                                                          📁 .cargo/
                                                                            📄 config.toml
                                                                          📄 .editorconfig
                                                                          📄 .gitignore
                                                                          📄 Cargo.lock
                                                                          📄 Cargo.toml
                                                                          📄 LICENSE
                                                                          📖 README.ja.md
                                                                          📖 README.md
                                                                          📁 _codeql_detected_source_root/
                                                                            📁 .cargo/
                                                                              📄 config.toml
                                                                            📄 .editorconfig
                                                                            📄 .gitignore
                                                                            📄 Cargo.lock
                                                                            📄 Cargo.toml
                                                                            📄 LICENSE
                                                                            📖 README.ja.md
                                                                            📖 README.md
                                                                            📁 _codeql_detected_source_root/
                                                                              📁 .cargo/
                                                                                📄 config.toml
                                                                              📄 .editorconfig
                                                                              📄 .gitignore
                                                                              📄 Cargo.lock
                                                                              📄 Cargo.toml
                                                                              📄 LICENSE
                                                                              📖 README.ja.md
                                                                              📖 README.md
                                                                              📁 _codeql_detected_source_root/
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
                                                                                  📖 36.md
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
                                                                              📄 _config.yml
                                                                              📄 build.rs
                                                                              📁 generated-docs/
                                                                              📁 issue-notes/
                                                                                📖 34.md
                                                                                📖 36.md
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
                                                                            📄 _config.yml
                                                                            📄 build.rs
                                                                            📁 generated-docs/
                                                                            📁 issue-notes/
                                                                              📖 34.md
                                                                              📖 36.md
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
                                                                          📄 _config.yml
                                                                          📄 build.rs
                                                                          📁 generated-docs/
                                                                          📁 issue-notes/
                                                                            📖 34.md
                                                                            📖 36.md
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
                                                                        📄 _config.yml
                                                                        📄 build.rs
                                                                        📁 generated-docs/
                                                                        📁 issue-notes/
                                                                          📖 34.md
                                                                          📖 36.md
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
                                                                      📄 _config.yml
                                                                      📄 build.rs
                                                                      📁 generated-docs/
                                                                      📁 issue-notes/
                                                                        📖 34.md
                                                                        📖 36.md
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
                                                                    📄 _config.yml
                                                                    📄 build.rs
                                                                    📁 generated-docs/
                                                                    📁 issue-notes/
                                                                      📖 34.md
                                                                      📖 36.md
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
                                                                  📄 _config.yml
                                                                  📄 build.rs
                                                                  📁 generated-docs/
                                                                  📁 issue-notes/
                                                                    📖 34.md
                                                                    📖 36.md
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
                                                                📄 _config.yml
                                                                📄 build.rs
                                                                📁 generated-docs/
                                                                📁 issue-notes/
                                                                  📖 34.md
                                                                  📖 36.md
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
                                                              📄 _config.yml
                                                              📄 build.rs
                                                              📁 generated-docs/
                                                              📁 issue-notes/
                                                                📖 34.md
                                                                📖 36.md
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
                                                            📄 _config.yml
                                                            📄 build.rs
                                                            📁 generated-docs/
                                                            📁 issue-notes/
                                                              📖 34.md
                                                              📖 36.md
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
                                                          📄 _config.yml
                                                          📄 build.rs
                                                          📁 generated-docs/
                                                          📁 issue-notes/
                                                            📖 34.md
                                                            📖 36.md
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
                                                        📄 _config.yml
                                                        📄 build.rs
                                                        📁 generated-docs/
                                                        📁 issue-notes/
                                                          📖 34.md
                                                          📖 36.md
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
                                                      📄 _config.yml
                                                      📄 build.rs
                                                      📁 generated-docs/
                                                      📁 issue-notes/
                                                        📖 34.md
                                                        📖 36.md
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
                                                    📄 _config.yml
                                                    📄 build.rs
                                                    📁 generated-docs/
                                                    📁 issue-notes/
                                                      📖 34.md
                                                      📖 36.md
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
                                                  📄 _config.yml
                                                  📄 build.rs
                                                  📁 generated-docs/
                                                  📁 issue-notes/
                                                    📖 34.md
                                                    📖 36.md
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
                                                📄 _config.yml
                                                📄 build.rs
                                                📁 generated-docs/
                                                📁 issue-notes/
                                                  📖 34.md
                                                  📖 36.md
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
                                              📄 _config.yml
                                              📄 build.rs
                                              📁 generated-docs/
                                              📁 issue-notes/
                                                📖 34.md
                                                📖 36.md
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
                                            📄 _config.yml
                                            📄 build.rs
                                            📁 generated-docs/
                                            📁 issue-notes/
                                              📖 34.md
                                              📖 36.md
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
                                          📄 _config.yml
                                          📄 build.rs
                                          📁 generated-docs/
                                          📁 issue-notes/
                                            📖 34.md
                                            📖 36.md
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
                                        📄 _config.yml
                                        📄 build.rs
                                        📁 generated-docs/
                                        📁 issue-notes/
                                          📖 34.md
                                          📖 36.md
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
                                      📄 _config.yml
                                      📄 build.rs
                                      📁 generated-docs/
                                      📁 issue-notes/
                                        📖 34.md
                                        📖 36.md
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
                                    📄 _config.yml
                                    📄 build.rs
                                    📁 generated-docs/
                                    📁 issue-notes/
                                      📖 34.md
                                      📖 36.md
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
                                  📄 _config.yml
                                  📄 build.rs
                                  📁 generated-docs/
                                  📁 issue-notes/
                                    📖 34.md
                                    📖 36.md
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
                                📄 _config.yml
                                📄 build.rs
                                📁 generated-docs/
                                📁 issue-notes/
                                  📖 34.md
                                  📖 36.md
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
                              📄 _config.yml
                              📄 build.rs
                              📁 generated-docs/
                              📁 issue-notes/
                                📖 34.md
                                📖 36.md
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
                            📄 _config.yml
                            📄 build.rs
                            📁 generated-docs/
                            📁 issue-notes/
                              📖 34.md
                              📖 36.md
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
                          📄 _config.yml
                          📄 build.rs
                          📁 generated-docs/
                          📁 issue-notes/
                            📖 34.md
                            📖 36.md
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
                        📄 _config.yml
                        📄 build.rs
                        📁 generated-docs/
                        📁 issue-notes/
                          📖 34.md
                          📖 36.md
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
                      📄 _config.yml
                      📄 build.rs
                      📁 generated-docs/
                      📁 issue-notes/
                        📖 34.md
                        📖 36.md
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
                    📄 _config.yml
                    📄 build.rs
                    📁 generated-docs/
                    📁 issue-notes/
                      📖 34.md
                      📖 36.md
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
                  📄 _config.yml
                  📄 build.rs
                  📁 generated-docs/
                  📁 issue-notes/
                    📖 34.md
                    📖 36.md
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
                📄 _config.yml
                📄 build.rs
                📁 generated-docs/
                📁 issue-notes/
                  📖 34.md
                  📖 36.md
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
              📄 _config.yml
              📄 build.rs
              📁 generated-docs/
              📁 issue-notes/
                📖 34.md
                📖 36.md
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
            📄 _config.yml
            📄 build.rs
            📁 generated-docs/
            📁 issue-notes/
              📖 34.md
              📖 36.md
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
          📄 _config.yml
          📄 build.rs
          📁 generated-docs/
          📁 issue-notes/
            📖 34.md
            📖 36.md
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
        📄 _config.yml
        📄 build.rs
        📁 generated-docs/
        📁 issue-notes/
          📖 34.md
          📖 36.md
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
      📄 _config.yml
      📄 build.rs
      📁 generated-docs/
      📁 issue-notes/
        📖 34.md
        📖 36.md
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
    📄 _config.yml
    📄 build.rs
    📁 generated-docs/
    📁 issue-notes/
      📖 34.md
      📖 36.md
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
  📄 _config.yml
  📄 build.rs
  📁 generated-docs/
  📁 issue-notes/
    📖 34.md
    📖 36.md
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
📄 _config.yml
📄 build.rs
📁 generated-docs/
📁 issue-notes/
  📖 34.md
  📖 36.md
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

## ファイル詳細分析


## 関数呼び出し階層
関数呼び出し階層を分析できませんでした

## プロジェクト構造（ファイル一覧）
README.ja.md
README.md
_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.ja.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/README.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/_codeql_detected_source_root/tests/fixtures/complex.json
_codeql_detected_source_root/issue-notes/34.md
_codeql_detected_source_root/tests/fixtures/complex.json
issue-notes/34.md
tests/fixtures/complex.json

上記の情報を基に、プロンプトで指定された形式でプロジェクト概要を生成してください。
特に以下の点を重視してください：
- 技術スタックは各カテゴリごとに整理して説明
- ファイル階層ツリーは提供された構造をそのまま使用
- ファイルの説明は各ファイルの実際の内容と機能に基づく
- 関数の説明は実際に検出された関数の役割に基づく
- 関数呼び出し階層は実際の呼び出し関係に基づく


---
Generated at: 2025-11-16 07:01:42 JST
