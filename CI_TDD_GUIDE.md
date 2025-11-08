# CI/TDD Guide for Linux Runners

このガイドは、GitHub Actions Linux RunnerやヘッドレスLinux環境で、音声デバイスなしでym2151-log-play-serverを実行するための設定方法を説明します。

## セットアップ方法

### 1. ALSA開発ライブラリのインストール（ビルド時に必要）

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev
```

### 2. ALSA設定ファイルの作成（実行時に必要）

```bash
cat <<'EOF' > ~/.asoundrc
pcm.!default {
  type file
  slave.pcm "null"
  file "/tmp/alsa_capture.wav"
  format "wav"
}
EOF
```

または、付属のセットアップスクリプトを使用：

```bash
./setup_ci_environment.sh
```

## TDD（テスト駆動開発）での使用

ALSA設定が完了すれば、通常のローカル開発と同じようにテストを実行できます：

```bash
# ビルド
cargo build

# テスト実行
cargo test

# プログラム実行
cargo run sample_events.json
```

出力ファイル：
- `output.wav` - プログラムが直接生成するWAVファイル（441KB程度）
- `/tmp/alsa_capture.wav` - ALSAがキャプチャした音声ファイル（大きいサイズ）

## 仕組み

ALSA設定ファイル（`~/.asoundrc`）により：

1. プログラムは通常通り音声デバイスに出力しようとする
2. ALSAレイヤーがその出力を `/tmp/alsa_capture.wav` にリダイレクト
3. プログラムはエラーなく動作し、WAVファイルも生成される
4. 音声デバイスがなくても正常に動作する

## 利点

- ✅ コードがシンプル（`--no-audio` オプション不要）
- ✅ 単一の実行パス（条件分岐なし）
- ✅ CI環境でもローカル環境と同じコードが動作
- ✅ TDDが容易（テストコマンドに変更不要）

## トラブルシューティング

### エラー: "No output device available"

ALSA設定ファイルが正しく作成されていない可能性があります：

```bash
# 設定ファイルを確認
cat ~/.asoundrc

# セットアップスクリプトを再実行
./setup_ci_environment.sh
```

### ビルドエラー: "alsa-sys was not found"

ALSA開発ライブラリがインストールされていません：

```bash
sudo apt-get install -y libasound2-dev
```
