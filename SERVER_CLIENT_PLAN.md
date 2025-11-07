# サーバー・クライアント機能 実装計画書

## 目次

- [概要](#概要)
- [要件定義](#要件定義)
- [アーキテクチャ設計](#アーキテクチャ設計)
- [実装設計](#実装設計)
- [段階的実装計画](#段階的実装計画)
- [テスト計画](#テスト計画)
- [依存関係](#依存関係)
- [想定される課題と対策](#想定される課題と対策)
- [スケジュール概算](#スケジュール概算)
- [成果物チェックリスト](#成果物チェックリスト)
- [参考資料](#参考資料)
- [まとめ](#まとめ)

## 概要

本計画書は、ym2151-log-play-server にサーバー・クライアント機能を追加するための実装計画を定義します。

### 目的

- YM2151 演奏を常駐サーバーとして実行
- クライアントから演奏制御（開始、停止、シャットダウン）
- 名前付きパイプによるプロセス間通信

### 重要な注意事項

> ⚠️ **この実装は検証用の暫定仕様です**
>
> 要件に明記されているように、この実装は「検証を素早く進めるため、仮です。あとから破壊的変更します」という前提があります。
> したがって、以下の方針で実装します：
> - シンプルな実装を優先
> - 拡張性よりも動作検証を重視
> - 最小限のエラーハンドリング
> - 将来の破壊的変更を前提とした設計

## 要件定義

### コマンドライン引数

```bash
# サーバーモード
--server <json_file>    # サーバーとして常駐し、指定JSONを演奏開始
--server --shutdown     # サーバーをシャットダウン

# クライアントモード
--client <json_file>    # サーバーに演奏停止 + 新規JSON演奏を指示
--client --stop         # サーバーに演奏停止を指示
```

### 機能仕様

#### 1. サーバーモード (`--server <json_file>`)

**動作:**
1. 指定されたJSONファイルを読み込み
2. 演奏を開始
3. 名前付きパイプを作成してクライアント接続を待機
4. クライアントからのメッセージを受信・処理
5. シャットダウンメッセージを受信するまで常駐

**処理するメッセージ:**
- `PLAY <json_path>`: 現在の演奏を停止し、新しいJSONを演奏
- `STOP`: 演奏を停止（無音化）
- `SHUTDOWN`: 演奏を停止してサーバーを終了

#### 2. サーバーシャットダウン (`--server --shutdown`)

**動作:**
1. 名前付きパイプに接続
2. `SHUTDOWN` メッセージを送信
3. サーバーの終了を待機

#### 3. クライアントモード (`--client <json_file>`)

**動作:**
1. 名前付きパイプに接続
2. `PLAY <json_path>` メッセージを送信
3. サーバーからの応答を受信

#### 4. 演奏停止 (`--client --stop`)

**動作:**
1. 名前付きパイプに接続
2. `STOP` メッセージを送信
3. サーバーからの応答を受信

## アーキテクチャ設計

### プロセス間通信方式

**選定: 名前付きパイプ (Named Pipe / FIFO)**

**理由:**
- 要件で明示的に指定されている
- OSネイティブなIPC機構
- Unix系とWindowsの両方で利用可能（実装は異なる）

**プラットフォーム別実装:**
- **Unix/Linux**: FIFO (`mkfifo`)
- **Windows**: Named Pipe (`CreateNamedPipe`)

### 名前付きパイプパス

```
Unix/Linux: /tmp/ym2151_server.pipe
Windows:    \\.\pipe\ym2151_server
```

### プロトコル仕様

#### メッセージフォーマット

テキストベースの単純なプロトコル（改行区切り）

```
<COMMAND> [<ARGUMENT>]\n
```

#### コマンド一覧

| コマンド | 引数 | 説明 |
|---------|------|------|
| PLAY | `<json_path>` | 演奏停止 + 新規JSON演奏開始 |
| STOP | なし | 演奏を停止 |
| SHUTDOWN | なし | サーバーを終了 |

#### レスポンス

```
OK\n              # 成功
ERROR <message>\n # エラー
```

#### 通信例

```
クライアント → サーバー: "PLAY /path/to/music.json\n"
サーバー → クライアント: "OK\n"

クライアント → サーバー: "STOP\n"
サーバー → クライアント: "OK\n"

クライアント → サーバー: "SHUTDOWN\n"
サーバー → クライアント: "OK\n"
[サーバー終了]
```

## 実装設計

### モジュール構成

```
src/
├── main.rs              # エントリーポイント（既存）
├── lib.rs               # ライブラリルート（既存）
├── server.rs            # 新規: サーバーモード実装
├── client.rs            # 新規: クライアントモード実装
├── ipc/                 # 新規: プロセス間通信モジュール
│   ├── mod.rs           # IPCモジュールルート
│   ├── protocol.rs      # プロトコル定義とパース
│   ├── pipe_unix.rs     # Unix名前付きパイプ実装
│   └── pipe_windows.rs  # Windows名前付きパイプ実装
├── player.rs            # 既存: 演奏エンジン
├── audio.rs             # 既存: オーディオ再生
└── ...                  # その他既存モジュール
```

### データ構造

#### プロトコル定義 (`src/ipc/protocol.rs`)

```rust
pub enum Command {
    Play(String),  // JSON path
    Stop,
    Shutdown,
}

pub enum Response {
    Ok,
    Error(String),
}

impl Command {
    pub fn parse(line: &str) -> Result<Self, String>;
    pub fn serialize(&self) -> String;
}

impl Response {
    pub fn serialize(&self) -> String;
}
```

#### サーバー状態 (`src/server.rs`)

```rust
enum ServerState {
    Playing,
    Stopped,
}

struct Server {
    state: Arc<Mutex<ServerState>>,
    audio_player: Arc<Mutex<Option<AudioPlayer>>>,
    shutdown_flag: Arc<AtomicBool>,
}
```

### スレッドモデル

#### サーバーモード

```
[メインスレッド]
  │
  ├─→ [音声再生スレッド]
  │     AudioPlayer が内部で管理
  │
  └─→ [IPC受信スレッド]
        名前付きパイプから接続を受け付け
        メッセージを受信・処理
```

**スレッド間通信:**
- `Arc<Mutex<>>`: 共有状態（演奏状態、AudioPlayer）
- `Arc<AtomicBool>`: シャットダウンフラグ

#### クライアントモード

```
[メインスレッド]
  名前付きパイプに接続
  メッセージ送信
  応答受信
  終了
```

### エラーハンドリング

#### サーバー側

- 名前付きパイプ作成失敗 → エラーメッセージを出力して終了
- JSONファイル読み込み失敗 → エラーレスポンスを返す
- 演奏中の予期しないエラー → ログ出力、演奏停止

#### クライアント側

- 名前付きパイプ接続失敗 → エラーメッセージ（サーバー未起動の可能性）
- タイムアウト → エラーメッセージ
- 不正なレスポンス → エラーメッセージ

### 既存コードへの影響

#### `src/main.rs` の変更

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    
    // コマンドライン引数解析
    if args.contains(&"--server".to_string()) {
        if args.contains(&"--shutdown".to_string()) {
            // サーバーシャットダウン（クライアントとして動作）
            client::shutdown_server();
        } else {
            // サーバーモード
            let json_path = extract_json_path(&args);
            server::run_server(json_path);
        }
    } else if args.contains(&"--client".to_string()) {
        if args.contains(&"--stop".to_string()) {
            // 停止指示
            client::stop_playback();
        } else {
            // 演奏指示
            let json_path = extract_json_path(&args);
            client::play_file(json_path);
        }
    } else {
        // 従来の動作（スタンドアロン演奏）
        // 既存のコードを維持
    }
}
```

#### `src/player.rs` の変更

**最小限の変更:**
- 演奏停止メソッドの追加: `pub fn stop(&mut self)`
- JSON再読み込みメソッドの追加: `pub fn load_new_json(&mut self, path: &str)`

#### `src/audio.rs` の変更

**最小限の変更:**
- 演奏停止メソッドの追加: `pub fn stop(&mut self)`
- 無音化の実装

## 段階的実装計画

### Phase 1: プロトコル定義とユーティリティ

**タスク:**
1. `src/ipc/mod.rs` 作成
2. `src/ipc/protocol.rs` 実装
   - `Command` enum定義
   - `Response` enum定義
   - パース・シリアライズ関数
3. ユニットテストでプロトコル検証

**成果物:**
- プロトコルパーサーとシリアライザー
- テストで動作確認

**期間:** 0.5日

### Phase 2: 名前付きパイプ抽象化

**タスク:**
1. `Cargo.toml` に `nix` クレートを追加
2. `src/ipc/pipe_unix.rs` 実装（Linux用）
   - `nix::unistd::mkfifo()` でFIFO作成
   - `std::fs::File` と `std::fs::OpenOptions` で読み書き
   - ノンブロッキングI/O対応
2. `src/ipc/pipe_windows.rs` スタブ作成（将来実装）
3. プラットフォーム切り替えロジック
4. 統合テスト

**成果物:**
- Unix向け名前付きパイプI/O
- Windows向けスタブ

**期間:** 1日

### Phase 3: クライアント機能実装

**タスク:**
1. `src/client.rs` 作成
2. 名前付きパイプ接続
3. メッセージ送信・応答受信
4. `play_file()` 関数実装
5. `stop_playback()` 関数実装
6. `shutdown_server()` 関数実装
7. エラーハンドリング

**成果物:**
- 動作するクライアント実装
- 手動テスト用コード

**期間:** 1日

### Phase 4: サーバー機能実装（基本）

**タスク:**
1. `src/server.rs` 作成
2. 名前付きパイプ作成・待機ループ
3. メッセージ受信・パース
4. 基本的な応答返信
5. マルチスレッド対応
   - IPC受信スレッド
   - 状態管理（Arc<Mutex<>>）

**成果物:**
- 基本的なサーバーフレームワーク
- メッセージ送受信の動作確認

**期間:** 1.5日

### Phase 5: 演奏制御統合

**タスク:**
1. `Player` に停止メソッド追加
2. `AudioPlayer` に停止メソッド追加
3. サーバーに演奏開始ロジック統合
4. `PLAY` コマンド実装
5. `STOP` コマンド実装
6. `SHUTDOWN` コマンド実装

**成果物:**
- 完全に機能するサーバー
- エンドツーエンドテスト

**期間:** 1.5日

### Phase 6: コマンドライン統合

**タスク:**
1. `main.rs` の引数解析更新
2. `--server` オプション実装
3. `--client` オプション実装
4. `--stop` オプション実装
5. `--shutdown` オプション実装
6. ヘルプメッセージ更新
7. エラーメッセージ改善

**成果物:**
- 完全統合されたCLI
- ドキュメント更新

**期間:** 1日

### Phase 7: テストと検証

**タスク:**
1. 統合テスト作成
2. エラーケーステスト
3. 複数クライアント同時接続テスト
4. ロングランテスト（メモリリーク検証）
5. ドキュメント作成
6. README更新

**成果物:**
- テストスイート
- ドキュメント

**期間:** 1日

### Phase 8: Windows対応（オプション）

**タスク:**
1. `src/ipc/pipe_windows.rs` 実装
2. Windows環境でのビルド・テスト
3. プラットフォーム固有の問題修正

**成果物:**
- Windows対応版

**期間:** 1.5日（オプション）

## テスト計画

### ユニットテスト

#### Protocol Tests (`src/ipc/protocol.rs`)

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_play_command() {
        let cmd = Command::parse("PLAY /path/to/file.json").unwrap();
        assert!(matches!(cmd, Command::Play(_)));
    }
    
    #[test]
    fn test_parse_stop_command() {
        let cmd = Command::parse("STOP").unwrap();
        assert!(matches!(cmd, Command::Stop));
    }
    
    #[test]
    fn test_serialize_response() {
        assert_eq!(Response::Ok.serialize(), "OK\n");
    }
}
```

### 統合テスト

#### サーバー・クライアント通信テスト

```rust
#[test]
fn test_server_client_play() {
    // 1. サーバーを別スレッドで起動
    // 2. クライアントから PLAY コマンド送信
    // 3. OK レスポンス確認
    // 4. サーバーシャットダウン
}

#[test]
fn test_server_client_stop() {
    // STOP コマンドのテスト
}

#[test]
fn test_server_shutdown() {
    // SHUTDOWN コマンドのテスト
}
```

### 手動テスト手順

#### 基本動作確認

```bash
# ターミナル1: サーバー起動
cargo run -- --server sample_events.json

# ターミナル2: 演奏停止
cargo run -- --client --stop

# ターミナル2: 別のファイルを演奏
cargo run -- --client test_input.json

# ターミナル2: サーバーシャットダウン
cargo run -- --server --shutdown
```

#### エラーケース確認

```bash
# サーバー未起動でクライアント実行
cargo run -- --client --stop
# → エラーメッセージ確認

# 存在しないファイル指定
cargo run -- --client nonexistent.json
# → エラーレスポンス確認
```

## 依存関係

### 新規追加が必要なクレート

**検討中:**
- `nix` (v0.27以降) - Unix名前付きパイプ（FIFO）作成用
  - ライセンス: MIT
  - 用途: `nix::unistd::mkfifo()` でFIFO作成
  - 代替: `libc` クレートで直接 `mkfifo()` システムコールを使用

**注記:**  
FIFO作成には `mkfifo()` システムコールが必要（Unixのみ）。標準ライブラリのみでの実装は不可能なため、`nix` または `libc` クレートの使用を推奨。

**理由:**
- 名前付きパイプ（FIFO）の作成には `nix` クレートまたは直接システムコール (`libc::mkfifo`) を使用
- 作成後のFIFOへの読み書きは `std::fs::File` と `std::fs::OpenOptions` で可能
- プロトコルは単純なテキスト形式（`String` で十分）
- スレッド管理は `std::thread`、同期は `std::sync`

### 既存依存関係の変更

なし

## 想定される課題と対策

### 課題1: 名前付きパイプの排他制御

**課題:** 複数クライアントが同時に接続する場合の競合

**対策:**
- 暫定実装では逐次処理（1接続ずつ処理）
- 将来的にはスレッドプール検討

### 課題2: 演奏中の停止処理

**課題:** AudioPlayerの安全な停止

**対策:**
- `AudioPlayer::stop()` メソッドで内部状態をクリア
- 無音データを出力するフラグを設定

### 課題3: プラットフォーム間の差異

**課題:** Unix と Windows で名前付きパイプの実装が大きく異なる

**対策:**
- 抽象化層（trait）で統一インターフェース提供
- プラットフォーム別実装を分離（`cfg` 属性使用）

### 課題4: エラーハンドリング

**課題:** 暫定実装のため詳細なエラー処理は不要だが、最低限は必要

**対策:**
- 致命的エラー: `panic!` または `std::process::exit(1)`
- 回復可能エラー: エラーレスポンス返信
- ログ出力: `eprintln!` で標準エラー出力

### 課題5: リソースクリーンアップ

**課題:** 名前付きパイプファイルの削除タイミング

**対策:**
- サーバー起動時: 既存パイプがあれば削除
- サーバー終了時: パイプファイルを削除（`Drop` trait実装）

## スケジュール概算

| Phase | タスク | 期間 |
|-------|--------|------|
| Phase 1 | プロトコル定義 | 0.5日 |
| Phase 2 | 名前付きパイプ抽象化 | 1日 |
| Phase 3 | クライアント実装 | 1日 |
| Phase 4 | サーバー基本実装 | 1.5日 |
| Phase 5 | 演奏制御統合 | 1.5日 |
| Phase 6 | CLI統合 | 1日 |
| Phase 7 | テスト・検証 | 1日 |
| **合計** | | **7.5日** |
| Phase 8 (オプション) | Windows対応 | 1.5日 |

**前提条件:**
- 1日 = 6-8時間の実装時間
- レビュー時間は含まない
- 既存コードの理解時間は含まない

## 成果物チェックリスト

### コード

- [ ] `src/ipc/mod.rs`
- [ ] `src/ipc/protocol.rs`
- [ ] `src/ipc/pipe_unix.rs`
- [ ] `src/ipc/pipe_windows.rs` (スタブ)
- [ ] `src/client.rs`
- [ ] `src/server.rs`
- [ ] `src/main.rs` 更新
- [ ] `src/player.rs` 更新（停止メソッド追加）
- [ ] `src/audio.rs` 更新（停止メソッド追加）

### テスト

- [ ] `src/ipc/protocol.rs` のユニットテスト
- [ ] `tests/server_client_test.rs` 統合テスト
- [ ] 手動テスト手順実行

### ドキュメント

- [ ] README.md 更新（新機能説明）
- [ ] 使用例の追加
- [ ] トラブルシューティングガイド

## 参考資料

### Unix Named Pipes

- `man 7 fifo`
- `mkfifo(3)`, `open(2)`, `read(2)`, `write(2)`

### Rust標準ライブラリ

- `std::fs::{File, OpenOptions, remove_file}` - 作成済みFIFOへの読み書き
- `std::sync::{Arc, Mutex, atomic::AtomicBool}`
- `std::thread`

### Unix システムコール / クレート

- `nix::unistd::mkfifo` または `libc::mkfifo` - FIFO作成（Unix）
- `nix::sys::stat::Mode` - パーミッション設定

### 既存実装参照

- `src/player.rs`: イベント処理ロジック
- `src/audio.rs`: オーディオ再生制御

## まとめ

本計画書では、ym2151-log-play-server にサーバー・クライアント機能を追加するための詳細な実装計画を策定しました。

### 重要ポイント

1. **暫定実装**: 検証目的のため、シンプルさを優先
2. **最小限の依存**: `nix` クレート（FIFO作成用）のみ追加
3. **段階的実装**: 8つのPhaseに分けて段階的に実装
4. **テスト重視**: 各Phaseでテストを実施

### 次のステップ

1. 本計画書のレビュー
2. Phase 1（プロトコル定義）から実装開始
3. 各Phase完了後にレビューと動作確認

---

**計画書バージョン:** 1.1  
**作成日:** 2025-11-07  
**最終更新:** 2025-11-07  
**ステータス:** レビュー完了
