# GitHub Scripts

このディレクトリには、GitHub Actions ワークフロー内で使用されるスクリプトが含まれています。
**汎用性**: これらのスクリプトは、cargo-nextestを使用する任意のWindows Rustプロジェクトで利用できます。

## Scripts

### parse_nextest_junit.py

nextest の JUnit XML 出力を解析する汎用スクリプト。

**用途**: `build_windows.yml` ワークフローで nextest が生成した JUnit XML ファイルを解析し、テスト統計情報と失敗したテストの詳細を抽出します。

**機能**:
- JUnit XML からテスト統計情報を抽出 (合計、成功、失敗、タイムアウト)
- 失敗したテストのシンプルなリストを生成（Agent作業用）
- 各失敗テストの詳細エラーメッセージを抽出（日本語訳用）
- タイムアウトの検出（failure message から判定）
- **アプリケーション固有の知識不要** - 汎用的に動作

**使い方**:

```bash
python3 parse_nextest_junit.py --junit-file target/nextest/default/junit.xml --output-format all
```

**出力形式**:
- `stats`: 統計情報のみ (`key=value` 形式)
- `list`: 失敗テストのシンプルリストのみ
- `errors`: 詳細エラーメッセージ付き失敗テスト
- `all`: すべて（`---FAILED_TESTS---` と `---ERROR_DETAILS---` で区切られる）

**出力例**:
```
total_tests=10
passed=7
failed=3
timed_out=1
---FAILED_TESTS---
- test::module::test_name
- test::other::test_timeout (タイムアウト)
---ERROR_DETAILS---
### test::module::test_name
**Error**: Connection refused
```
thread panicked at 'assertion failed'
```
```

**テスト**:
```bash
cd .github/scripts
python3 -m unittest test_parse_nextest_junit.py -v
```

### generate_test_failure_issue.py

テスト失敗時に GitHub Issue の本文を生成する汎用スクリプト。

**用途**: `build_windows.yml` ワークフローでテストが失敗またはタイムアウトした時に、詳細な情報を含む Issue を自動生成します。

**機能**:
- **Issue冒頭にシンプルな失敗テストリスト配置** - Agentが作業しやすい
- **Gemini API による日本語訳** - ユーザーの認知負荷削減（環境変数 `GEMINI_API_KEY` が設定されている場合）
- **各テストの詳細エラーメッセージ** - 折りたたみ表示で見やすく
- テスト統計情報とワークフロー詳細
- **アプリケーション固有の知識不要** - 任意のRustプロジェクトで利用可能

**Issue構造**:
1. 🤖 エラーメッセージの日本語訳（AI生成）← ユーザー向け
2. 失敗したテスト（シンプルリスト）← Agent向け
3. テストサマリー統計
4. ワークフロー詳細情報
5. <details>詳細なエラーメッセージ</details>

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
  --failed-tests-list-file "/tmp/failed_tests.txt" \
  --error-details-file "/tmp/error_details.txt"
```

**テスト**:
```bash
cd .github/scripts
python3 -m unittest test_generate_test_failure_issue.py -v
```

## CI Workflow Integration

### build_windows.yml の動作フロー

1. **テスト実行**: `cargo nextest run` が JUnit XML を `target/nextest/default/junit.xml` に出力
2. **結果解析**: `parse_nextest_junit.py` が JUnit XML を解析して：
   - 統計情報を抽出
   - シンプルな失敗テストリストを生成
   - 各テストの詳細エラーメッセージを抽出
3. **Issue 生成**: `generate_test_failure_issue.py` が：
   - 詳細エラーをGemini APIで日本語訳（オプション）
   - シンプルなテストリストを冒頭に配置
   - 構造化されたIssue本文を生成
4. **Issue 作成**: GitHub Actions が Issue を自動作成

## 汎用性と再利用

これらのスクリプトは以下のプロジェクトで再利用できます：

- ✅ Windows環境のRustプロジェクト
- ✅ cargo-nextestを使用するプロジェクト
- ✅ JUnit XML出力を生成する任意のテストフレームワーク

**アプリケーション固有の設定不要** - すぐに使えます。

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
- **汎用性を保つ** - アプリケーション固有のロジックを避ける

### テスト

全てのスクリプトにはユニットテストを含める必要があります:

```bash
# 特定のテストを実行
python3 -m unittest test_script_name.py -v

# 全てのテストを実行
python3 -m unittest discover -s .github/scripts -p "test_*.py" -v
```
