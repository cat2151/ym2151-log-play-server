# GitHub Scripts

このディレクトリには、GitHub Actions ワークフロー内で使用されるスクリプトが含まれています。

## Scripts

### generate_test_failure_issue.py

テスト失敗時に GitHub Issue の本文を生成するスクリプト。

**用途**: `build_windows.yml` ワークフローでテストが失敗またはタイムアウトした時に、詳細な情報を含む Issue を自動生成します。

**機能**:
- テスト失敗の詳細情報を構造化して表示
- Gemini API を使用してエラーメッセージを日本語に翻訳（環境変数 `GEMINI_API_KEY` が設定されている場合）
- AIによる日本語訳をissue先頭に配置し、ユーザーの認知負荷を低減
- 指数バックオフによるリトライ機能でAPI呼び出しの信頼性を向上（初回60秒、最大2時間、最大8回試行）

**環境変数**:
- `GEMINI_API_KEY`: Gemini API キー（オプション。設定されている場合、エラーメッセージの日本語翻訳が有効になります）
- `FAILED_TESTS_CATEGORIZED`: 失敗したテストのカテゴリ別リスト（マークダウン形式）。後方互換性のため残されていますが、一時ファイルの使用を推奨します
- `ERROR_LOG`: 詳細なエラーログ。後方互換性のため残されていますが、一時ファイルの使用を推奨します

**使い方**:

方法1: 一時ファイルを使用（**推奨**：大きなログデータを扱う場合）
```bash
# 一時ファイルに複数行データを書き込む
echo "#### Server Tests (2件)
- test_server_1
- test_server_2" > /tmp/failed_tests.txt

echo "Error: test failed
Stack trace: at function()" > /tmp/error_log.txt

export GEMINI_API_KEY="your-api-key-here"

python3 generate_test_failure_issue.py \
  --status-ja "失敗" \
  --total-tests "10" \
  --passed "8" \
  --failed "2" \
  --timed-out "0" \
  --workflow "Windows CI" \
  --job "build-windows" \
  --run-id "123456" \
  --run-attempt "1" \
  --ref "refs/heads/main" \
  --commit "abc123" \
  --server-url "https://github.com" \
  --repository "owner/repo" \
  --failed-tests-categorized-file "/tmp/failed_tests.txt" \
  --error-log-file "/tmp/error_log.txt"
```

方法2: コマンドライン引数を使用（シンプルなケース向け）
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
  --error-log "Optional error log text"
```

方法3: 環境変数を使用（後方互換性のため残されています）
```bash
export GEMINI_API_KEY="your-api-key-here"
export FAILED_TESTS_CATEGORIZED="#### Server Tests (2件)
- test_server_1
- test_server_2"
export ERROR_LOG="Error: test failed
Stack trace: at function()"

python3 generate_test_failure_issue.py \
  --status-ja "失敗" \
  --total-tests "10" \
  --passed "8" \
  --failed "2" \
  --timed-out "0" \
  --workflow "Windows CI" \
  --job "build-windows" \
  --run-id "123456" \
  --run-attempt "1" \
  --ref "refs/heads/main" \
  --commit "abc123" \
  --server-url "https://github.com" \
  --repository "owner/repo"
```

**優先順位**:
1. 一時ファイル（`--failed-tests-categorized-file`, `--error-log-file`）
2. コマンドライン引数（`--failed-tests-categorized`, `--error-log`）
3. 環境変数（`FAILED_TESTS_CATEGORIZED`, `ERROR_LOG`）

**注意**: 
- 大きなログデータを扱う場合は、一時ファイルの使用を推奨します（環境変数にはサイズ制限があります）
- PowerShellでは複数行文字列のコマンドライン引数渡しで問題が発生する可能性があるため、一時ファイルの使用を推奨します

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
