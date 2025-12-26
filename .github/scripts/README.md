# GitHub Scripts

このディレクトリには、GitHub Actions ワークフロー内で使用されるスクリプトが含まれています。

## Scripts

### parse_nextest_junit.py

nextest の JUnit XML 出力を解析するスクリプト。

**用途**: `build_windows.yml` ワークフローで nextest が生成した JUnit XML ファイルを解析し、テスト統計情報と失敗したテストのカテゴリ別リストを抽出します。

**機能**:
- JUnit XML からテスト統計情報を抽出 (合計、成功、失敗、タイムアウト)
- 失敗したテストをカテゴリ別に分類
- タイムアウトの検出（failure message から判定）

**使い方**:

```bash
python3 parse_nextest_junit.py --junit-file target/nextest/default/junit.xml --output-format both
```

**出力形式**:
- `stats`: 統計情報のみ (`key=value` 形式)
- `categorized`: カテゴリ別失敗テストリストのみ (Markdown 形式)
- `both`: 統計情報とカテゴリ別リストの両方（`---` で区切られる）

**テスト**:
```bash
cd .github/scripts
python3 -m unittest test_parse_nextest_junit.py -v
```

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

**使い方**:

```bash
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

**テスト**:
```bash
cd .github/scripts
python3 -m unittest test_generate_test_failure_issue.py -v
```

## CI Workflow Integration

### build_windows.yml の動作フロー

1. **テスト実行**: `cargo nextest run` が JUnit XML を `target/nextest/default/junit.xml` に出力
2. **結果解析**: `parse_nextest_junit.py` が JUnit XML を解析して統計情報と失敗テストリストを抽出
3. **Issue 生成**: `generate_test_failure_issue.py` が解析結果から GitHub Issue の本文を生成
4. **Issue 作成**: GitHub Actions が Issue を自動作成

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
