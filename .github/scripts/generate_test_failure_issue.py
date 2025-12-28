#!/usr/bin/env python3

# IMPORTANT: Gemini API translation is REQUIRED for this script.
# If the GEMINI_API_KEY is not set or the API call fails, the script MUST fail early.
# Fallback behavior (skipping translation) is strictly forbidden.
# This ensures that any configuration or API issues are immediately visible to the user.

import argparse
import json
import os
import sys
import time
import urllib.request
import urllib.error
from typing import Optional

# Force UTF-8 encoding for stdout to handle Unicode characters (emoji, Japanese text)
# on Windows consoles that default to cp1252 encoding
if sys.stdout.encoding != 'utf-8':
    sys.stdout.reconfigure(encoding='utf-8')
if sys.stderr.encoding != 'utf-8':
    sys.stderr.reconfigure(encoding='utf-8')

GEMINI_API_BASE_URL = "https://generativelanguage.googleapis.com/v1beta/models"
# GEMINI_MODEL_NAME = "gemini-3-pro-preview"
GEMINI_MODEL_NAME = "gemini-2.5-flash"

def translate_error_messages_with_gemini(error_details: str) -> Optional[str]:
    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key or not api_key.strip():
        raise ValueError("GEMINI_API_KEY environment variable is not set or empty")
    
    if not error_details or not error_details.strip():
        return None
    url = f"{GEMINI_API_BASE_URL}/{GEMINI_MODEL_NAME}:generateContent?key={api_key}"
    prompt = f"""以下のテスト失敗情報を日本語に翻訳してください。

失敗したテストとエラー:
```
{error_details}
```

日本語訳:"""
    
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
        except urllib.error.HTTPError as e:
            # For 4xx client errors, fail immediately as retrying won't help
            # These indicate problems with the request itself (wrong URL, authentication, etc.)
            if 400 <= e.code < 500:
                # Create a safe URL for logging (mask the API key)
                safe_url = f"{GEMINI_API_BASE_URL}/{GEMINI_MODEL_NAME}:generateContent?key=***"
                print(f"Error: Gemini API client error (HTTP {e.code}). Please check the configuration.", file=sys.stderr)
                print(f"URL: {safe_url}", file=sys.stderr)
                print(f"Model name: {GEMINI_MODEL_NAME}", file=sys.stderr)
                if e.code == 404:
                    print(f"Note: The model or endpoint was not found. Verify the model name is correct.", file=sys.stderr)
                elif e.code == 401 or e.code == 403:
                    print(f"Note: Authentication failed. Verify the GEMINI_API_KEY is correct.", file=sys.stderr)
                return None
            # For 5xx server errors, retry with exponential backoff
            if attempt < max_retries - 1:
                delay = min(base_delay * (2 ** attempt), max_delay)
                print(f"Warning: Gemini API error (attempt {attempt + 1}/{max_retries}): HTTP {e.code}. Retrying in {delay}s...", file=sys.stderr)
                time.sleep(delay)
            else:
                print(f"Error: Gemini API failed after {max_retries} attempts: HTTP {e.code}", file=sys.stderr)
                return None
        except urllib.error.URLError as e:
            # Network errors might be transient, so retry
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


def read_file_content(file_path: str, file_description: str) -> str:
    """Read content from a file with proper error handling.
    
    Args:
        file_path: Path to the file to read
        file_description: Description of the file for error messages
    
    Returns:
        Content of the file as string
    
    Raises:
        SystemExit: If file cannot be read
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return f.read()
    except FileNotFoundError:
        print(f"Error: {file_description} file not found: {file_path}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: Failed to read {file_description} file: {e}", file=sys.stderr)
        sys.exit(1)


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
    failed_tests_list = read_file_content(args.failed_tests_list_file, "failed tests list")
    error_details = read_file_content(args.error_details_file, "error details")
    
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
