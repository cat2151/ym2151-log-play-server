#!/usr/bin/env python3

import argparse
import json
import os
import sys
import time
import urllib.request
import urllib.error
from typing import Optional

GEMINI_API_BASE_URL = "https://generativelanguage.googleapis.com/v1beta/models"
GEMINI_MODEL_NAME = "gemini-3-flash"


def translate_error_messages_with_gemini(error_details: str) -> Optional[str]:
    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key or not api_key.strip():
        raise ValueError("GEMINI_API_KEY environment variable is not set or empty")
    
    if not error_details or not error_details.strip():
        return None
    url = f"{GEMINI_API_BASE_URL}/{GEMINI_MODEL_NAME}:generateContent?key={api_key}"
    prompt = f"""以下は、Windowsビルド環境でのRustプロジェクトのテスト失敗情報です。
各テストのエラーメッセージを日本語に翻訳してください。
技術用語は適切に翻訳し、開発者が理解しやすいように要約してください。

失敗したテストとエラー:
```
{error_details}
```

日本語訳（各テストごとに失敗原因を簡潔に説明）:"""
    
    data = {
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 0.3,
            "maxOutputTokens": 2048
        }
    }
    
    max_retries = 8
    base_delay = 60.0
    max_delay = 7200.0
    for attempt in range(max_retries):
        try:
            req = urllib.request.Request(
                url,
                data=json.dumps(data).encode('utf-8'),
                headers={'Content-Type': 'application/json'}
            )
            
            with urllib.request.urlopen(req, timeout=60) as response:
                result = json.loads(response.read().decode('utf-8'))
                if 'candidates' in result and len(result['candidates']) > 0:
                    candidate = result['candidates'][0]
                    if 'content' in candidate and 'parts' in candidate['content']:
                        parts = candidate['content']['parts']
                        if len(parts) > 0 and 'text' in parts[0]:
                            return parts[0]['text'].strip()
            return None
        except (urllib.error.HTTPError, urllib.error.URLError) as e:
            if attempt < max_retries - 1:
                delay = min(base_delay * (2 ** attempt), max_delay)
                print(f"Warning: Gemini API error (attempt {attempt + 1}/{max_retries}): {e}. Retrying in {delay}s...", file=sys.stderr)
                time.sleep(delay)
            else:
                print(f"Error: Gemini API failed after {max_retries} attempts: {e}", file=sys.stderr)
                return None
    return None


def generate_issue_body(
    status_ja: str,
    total_tests: str,
    passed: str,
    failed: str,
    timed_out: str,
    failed_tests_list: str,
    error_details: str,
    workflow: str,
    job: str,
    run_id: str,
    run_attempt: str,
    ref: str,
    commit: str,
    server_url: str,
    repository: str,
) -> str:
    sections = []
    
    if error_details:
        japanese_translation = translate_error_messages_with_gemini(error_details)
        if japanese_translation:
            sections.append("## 🤖 エラーメッセージの日本語訳（AI生成）")
            sections.append("")
            sections.append(japanese_translation)
            sections.append("")
            sections.append("---")
            sections.append("")
    
    sections.append("## 失敗したテスト")
    sections.append("")
    sections.append(failed_tests_list)
    sections.append("")
    sections.append("---")
    sections.append("")
    
    sections.append(f"**ステータス**: {status_ja}")
    sections.append("")
    sections.append("### テストサマリー")
    sections.append(f"- **総テスト数**: {total_tests}")
    sections.append(f"- **成功**: {passed}")
    sections.append(f"- **失敗**: {failed}")
    sections.append(f"- **タイムアウト**: {timed_out}")
    sections.append("")
    
    sections.append("### 詳細")
    sections.append(f"- Workflow: {workflow}")
    sections.append(f"- Job: {job}")
    sections.append(f"- Run: {server_url}/{repository}/actions/runs/{run_id}")
    sections.append(f"- Commit: {commit}")
    sections.append(f"- Ref: {ref}")
    sections.append("")
    
    if error_details and error_details.strip():
        sections.append("<details>")
        sections.append("<summary>詳細なエラーメッセージ（クリックして展開）</summary>")
        sections.append("")
        sections.append(error_details)
        sections.append("")
        sections.append("</details>")
        sections.append("")
    
    sections.append("**アーティファクト**: 完全なログは上記のRunリンクから `test-logs` をダウンロード")
    
    return "\n".join(sections)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--status-ja", required=True)
    parser.add_argument("--total-tests", required=True)
    parser.add_argument("--passed", required=True)
    parser.add_argument("--failed", required=True)
    parser.add_argument("--timed-out", required=True)
    parser.add_argument("--failed-tests-list-file", required=True, help="Path to file containing failed tests list")
    parser.add_argument("--error-details-file", required=True, help="Path to file containing error details")
    parser.add_argument("--workflow", required=True)
    parser.add_argument("--job", required=True)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--run-attempt", required=True)
    parser.add_argument("--ref", required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--server-url", required=True)
    parser.add_argument("--repository", required=True)
    
    args = parser.parse_args()
    
    # Read large data from files to avoid command-line size limitations
    try:
        with open(args.failed_tests_list_file, 'r', encoding='utf-8') as f:
            failed_tests_list = f.read()
    except FileNotFoundError:
        print(f"Error: Failed tests list file not found: {args.failed_tests_list_file}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: Failed to read failed tests list file: {e}", file=sys.stderr)
        sys.exit(1)
    
    try:
        with open(args.error_details_file, 'r', encoding='utf-8') as f:
            error_details = f.read()
    except FileNotFoundError:
        print(f"Error: Error details file not found: {args.error_details_file}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: Failed to read error details file: {e}", file=sys.stderr)
        sys.exit(1)
    
    issue_body = generate_issue_body(
        status_ja=args.status_ja,
        total_tests=args.total_tests,
        passed=args.passed,
        failed=args.failed,
        timed_out=args.timed_out,
        failed_tests_list=failed_tests_list,
        error_details=error_details,
        workflow=args.workflow,
        job=args.job,
        run_id=args.run_id,
        run_attempt=args.run_attempt,
        ref=args.ref,
        commit=args.commit,
        server_url=args.server_url,
        repository=args.repository,
    )
    
    print(issue_body)
    return 0


if __name__ == "__main__":
    sys.exit(main())
