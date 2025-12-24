# GitHub Scripts

このディレクトリには、GitHub Actions ワークフロー内で使用されるスクリプトが含まれています。

## Scripts

### generate_test_failure_issue.py

テスト失敗時に GitHub Issue の本文を生成するスクリプト。

**用途**: `build_windows.yml` ワークフローでテストが失敗またはタイムアウトした時に、詳細な情報を含む Issue を自動生成します。

**機能**:
- テスト失敗の詳細情報を構造化して表示
- Gemini API を使用してエラーメッセージを日本語に翻訳（オプション）
- AIによる日本語訳をissue先頭に配置し、ユーザーの認知負荷を低減

**使い方**:
```bash
python3 generate_test_failure_issue.py \
  --status-ja "失敗" \
  --total-tests "10" \
  --passed "8" \
  --failed "2" \
  --timed-out "0" \
  --failed-tests-categorized "#### Tests\n- test1\n- test2" \
  --workflow "Windows CI" \
  --job "build-windows" \
  --run-id "123456" \
  --run-attempt "1" \
  --ref "refs/heads/main" \
  --commit "abc123" \
  --server-url "https://github.com" \
  --repository "owner/repo" \
  --error-log "Optional error log text" \
  --gemini-api-key "Optional Gemini API key for translation"
```

**テスト**:
```bash
cd .github/scripts
python3 -m unittest test_generate_test_failure_issue.py -v
```

## Development

### 新しいスクリプトの追加

1. スクリプトファイルを作成 (`.py` 拡張子)
2. shebang を追加: `#!/usr/bin/env python3`
3. 実行可能にする: `chmod +x script_name.py`
4. 対応するテストファイルを作成: `test_script_name.py`
5. このREADMEを更新してスクリプトを文書化

### コーディング規約

- Python 3 標準ライブラリのみを使用 (可能な限り)
- 型ヒントを使用
- docstring を追加
- ユニットテストを作成
- 引数は argparse で処理

### テスト

全てのスクリプトにはユニットテストを含める必要があります:

```bash
# 特定のテストを実行
python3 -m unittest test_script_name.py -v

# 全てのテストを実行
python3 -m unittest discover -s .github/scripts -p "test_*.py" -v
```
