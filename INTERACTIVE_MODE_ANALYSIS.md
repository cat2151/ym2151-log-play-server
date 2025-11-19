# インタラクティブモード vs 非インタラクティブモード 比較分析

## 概要

このドキュメントは、issue #86 で報告されたインタラクティブモードの問題を診断するための比較分析です。

## 非インタラクティブモード（PlayJson）の動作フロー

### 1. クライアント側 (client.rs)

```rust
send_json(json_data)
  ↓
Command::PlayJson { data: json_value }
  ↓
send_command(command)
  ↓
NamedPipe::connect_default()  // パイプ接続
  ↓
write_binary(&binary_data)     // コマンド送信
  ↓
read_binary_response()         // レスポンス受信
```

### 2. サーバー側 (server.rs)

```rust
Command::PlayJson { data }
  ↓
load_and_start_playback(&json_str, true)
  ↓
EventLog::from_json_str(data)  // JSONパース
  ↓
Player::new(log)               // 静的イベントログからプレーヤー作成
  ↓
AudioPlayer::new_with_quality(player, Some(log), quality)
  ↓
音声再生開始
```

## インタラクティブモードの動作フロー

### 1. 開始フロー

#### クライアント側
```rust
start_interactive()
  ↓
Command::StartInteractive
  ↓
send_command(command)
  ↓
NamedPipe::connect_default()  // パイプ接続
  ↓
write_binary(&binary_data)     // コマンド送信
  ↓
read_binary_response()         // レスポンス受信
```

#### サーバー側
```rust
Command::StartInteractive
  ↓
start_interactive_mode()
  ↓
Player::new_interactive()      // インタラクティブプレーヤー作成
  ↓
AudioPlayer::new_with_quality(player, None, quality)  // event_log = None
  ↓
音声ストリーミング開始（継続）
```

### 2. レジスタ書き込みフロー

#### クライアント側
```rust
write_register(time_offset_sec, addr, data)
  ↓
Command::WriteRegister { time_offset_sec, addr, data }
  ↓
send_command(command)
  ↓
NamedPipe::connect_default()  // 毎回新しいパイプ接続
  ↓
write_binary(&binary_data)     // コマンド送信
  ↓
read_binary_response()         // レスポンス受信
```

#### サーバー側
```rust
Command::WriteRegister { time_offset_sec, addr, data }
  ↓
state == Interactive をチェック
  ↓
current_time_sec = time_tracker.elapsed_sec()
  ↓
scheduled_samples = sec_to_samples(current_time_sec + time_offset_sec)
  ↓
player_ref.schedule_register_write(scheduled_samples, addr, data)
  ↓
イベントキューに追加
```

### 3. 停止フロー

#### クライアント側
```rust
stop_interactive()
  ↓
Command::StopInteractive
  ↓
send_command(command)
```

#### サーバー側
```rust
Command::StopInteractive
  ↓
audio_player.stop()
  ↓
state = Stopped
```

## 主な違いの分析

### 1. パイプ接続パターン

#### 非インタラクティブモード
- **1回のみのパイプ接続**: `send_json()` 呼び出し時に1回だけ接続
- JSONデータを送信したら接続を閉じる
- サーバー側は1つのコマンドを処理して次の接続を待つ

#### インタラクティブモード
- **複数回のパイプ接続**: 
  1. `start_interactive()` で1回目の接続
  2. 各 `write_register()` 呼び出しで新しい接続
  3. `stop_interactive()` で最後の接続
- 頻繁なパイプ接続/切断が発生

### 2. AudioPlayer の作成パラメータ

#### 非インタラクティブモード
```rust
AudioPlayer::new_with_quality(player, Some(log), quality)
```
- `event_log = Some(log)`: イベントログを渡す
- verbose時にWAVファイル出力される

#### インタラクティブモード
```rust
AudioPlayer::new_with_quality(player, None, quality)
```
- `event_log = None`: イベントログなし
- WAVファイル出力なし

### 3. Player の種類

#### 非インタラクティブモード
```rust
Player::new(log)
```
- 静的なイベントログから作成
- イベントは事前に全て定義済み
- `is_interactive() == false`

#### インタラクティブモード
```rust
Player::new_interactive()
```
- 空のイベントキューで開始
- イベントは動的に追加される
- `is_interactive() == true`
- `is_complete()` は常に `false`（終了しない）

## 考えられる問題の原因

### 1. パイプ接続の頻度
**症状**: パイプ接続失敗

**原因候補**:
- インタラクティブモードでは複数回の接続が必要
- サーバーが前の接続を適切にクローズしていない可能性
- Windows名前付きパイプのインスタンス制限

**確認方法**:
- verbose モードでパイプ接続/切断のログを確認
- サーバー側のパイプ作成/削除のタイミングを確認

### 2. タイミングの問題
**症状**: 無音（音が鳴らない）

**原因候補**:
- `time_offset_sec` の計算が不正確
- `time_tracker` のリセットタイミング
- サーバー時刻とクライアント時刻の同期問題
- レジスタ書き込みが過去のタイムスタンプでスケジュールされている

**確認方法**:
- verbose モードで時刻計算のログを確認
- `get_server_time()` の結果を確認
- スケジュールされたサンプル位置を確認

### 3. 音声ストリーミングの開始
**症状**: 無音（音が鳴らない）

**原因候補**:
- `AudioPlayer::new_with_quality(player, None, quality)` が音声デバイスを正しく初期化していない
- インタラクティブモードでは `event_log = None` のため、何らかの初期化処理がスキップされている可能性
- 音声ストリームは開始しているが、イベントキューが空のため無音

**確認方法**:
- AudioPlayer の作成が成功しているか確認
- 音声デバイスの初期化ログを確認
- イベントキューにイベントが追加されているか確認

### 4. examples vs tests の違い
**症状**: テストは成功するがdemoが失敗

**原因候補**:
- テストとdemoで異なるタイミング/待機パターンを使用している
- テストではサーバーを同一プロセス内で起動している可能性
- demoでは外部サーバープロセスへの接続が必要で、起動待機が不足している

**確認方法**:
- tests と examples のコードを比較
- サーバー起動の方法を確認
- `ensure_server_ready()` の使用有無を確認

## 推奨される診断手順

1. **verbose モードでサーバーを起動**
   ```
   cargo run --release -- server --verbose
   ```

2. **verbose モードでクライアントからインタラクティブモードをテスト**
   ```
   # 別のターミナルで
   cargo run --example interactive_demo
   ```

3. **ログを確認して以下を特定**:
   - パイプ接続の成功/失敗
   - StartInteractive コマンドの処理
   - AudioPlayer の作成成功
   - 音声ストリーミングの開始
   - WriteRegister コマンドの処理
   - レジスタ書き込みのスケジュール
   - サーバー時刻とスケジュール時刻

4. **非インタラクティブモードと比較**:
   - 同じverboseログを非インタラクティブモードで取得
   - パイプ接続パターンの違いを確認
   - AudioPlayer 初期化の違いを確認

## 次のステップ

このドキュメントで特定した問題候補に基づいて:
1. 詳細なデバッグログをコードに追加
2. テストコードを見直して実際の問題を検出できるようにする
3. examples/demo コードを修正して動作するようにする
