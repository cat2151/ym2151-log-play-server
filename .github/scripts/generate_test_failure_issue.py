#!/usr/bin/env python3
"""
Generate issue body text for CI test failures.

This script generates the issue body for GitHub issues created when
Windows CI tests fail or time out.
"""

import argparse
import json
import os
import sys
import time
import urllib.request
import urllib.error
from typing import Optional


# Gemini API configuration
GEMINI_API_BASE_URL = "https://generativelanguage.googleapis.com/v1beta/models"
GEMINI_MODEL_NAME = "gemini-3-flash"


def translate_error_messages_with_gemini(error_details: str) -> Optional[str]:
    """
    Translate error messages to Japanese using Gemini API.
    
    Retrieves API key from GEMINI_API_KEY environment variable.
    Implements exponential backoff retry for transient API errors.
    
    Args:
        error_details: The error details text to translate (markdown formatted with test names and error messages)
    
    Returns:
        Translated text in Japanese, or None if error_details is empty
        
    Raises:
        ValueError: If API key is not available or empty
        Exception: For other non-API errors that should be detected early
    """
    api_key = os.getenv("GEMINI_API_KEY")
    if not api_key or not api_key.strip():
        raise ValueError("GEMINI_API_KEY environment variable is not set or empty. Translation cannot proceed without API key.")
    
    if not error_details or not error_details.strip():
        return None
    
    # Prepare the API request
    url = f"{GEMINI_API_BASE_URL}/{GEMINI_MODEL_NAME}:generateContent?key={api_key}"
    
    # Create the prompt for translation
    prompt = f"""以下は、Windowsビルド環境でのRustプロジェクトのテスト失敗情報です。
各テストのエラーメッセージを日本語に翻訳してください。
技術用語は適切に翻訳し、開発者が理解しやすいように要約してください。

失敗したテストとエラー:
```
{error_details}
```

日本語訳（各テストごとに失敗原因を簡潔に説明）:"""
    
    # Prepare request data
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
    
    # Exponential backoff retry configuration
    # Initial delay: 60 seconds (1 minute)
    # Max delay: 7200 seconds (2 hours)
    # Sequence: 60s -> 120s -> 240s -> 480s -> 960s -> 1920s -> 3840s -> 7200s (capped)
    max_retries = 8
    base_delay = 60.0  # seconds (1 minute)
    max_delay = 7200.0  # seconds (2 hours)
    
    for attempt in range(max_retries):
        try:
            # Make the API request
            req = urllib.request.Request(
                url,
                data=json.dumps(data).encode('utf-8'),
                headers={'Content-Type': 'application/json'}
            )
            
            with urllib.request.urlopen(req, timeout=60) as response:
                result = json.loads(response.read().decode('utf-8'))
                
                # Extract the translated text
                if 'candidates' in result and len(result['candidates']) > 0:
                    candidate = result['candidates'][0]
                    if 'content' in candidate and 'parts' in candidate['content']:
                        parts = candidate['content']['parts']
                        if len(parts) > 0 and 'text' in parts[0]:
                            return parts[0]['text'].strip()
            
            return None
        
        except (urllib.error.HTTPError, urllib.error.URLError) as e:
            # API-specific errors: retry with exponential backoff
            if attempt < max_retries - 1:
                delay = min(base_delay * (2 ** attempt), max_delay)
                print(f"Warning: Gemini API error (attempt {attempt + 1}/{max_retries}): {e}. Retrying in {delay}s...", file=sys.stderr)
                time.sleep(delay)
            else:
                # Max retries reached
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
    """
    Generate the issue body text for a test failure.
    
    Args:
        status_ja: Status in Japanese (e.g., "失敗" or "タイムアウトによりキャンセル")
        total_tests: Total number of tests run
        passed: Number of passed tests
        failed: Number of failed tests
        timed_out: Number of timed out tests
        failed_tests_list: Simple list of failed tests (markdown formatted)
        error_details: Detailed error messages for each failed test (markdown formatted)
        workflow: GitHub workflow name
        job: GitHub job name
        run_id: GitHub run ID
        run_attempt: GitHub run attempt number
        ref: GitHub ref (branch/tag)
        commit: GitHub commit SHA
        server_url: GitHub server URL
        repository: GitHub repository (owner/repo)
    
    Returns:
        The formatted issue body text
    """
    
    # Build the main sections
    sections = []
    
    # Try to translate error details using Gemini API for user cognitive load reduction
    # If API key is missing, ValueError will be raised and workflow will fail early
    if error_details:
        japanese_translation = translate_error_messages_with_gemini(error_details)
        if japanese_translation:
            sections.append("## 🤖 エラーメッセージの日本語訳（AI生成）")
            sections.append("")
            sections.append(japanese_translation)
            sections.append("")
            sections.append("---")
            sections.append("")
    
    # Header with simple failed tests list (for agent to easily work with)
    sections.append("## 失敗したテスト")
    sections.append("")
    sections.append(failed_tests_list)
    sections.append("")
    sections.append("---")
    sections.append("")
    
    # Status and statistics
    sections.append(f"**ステータス**: {status_ja}")
    sections.append("")
    sections.append("### テストサマリー")
    sections.append(f"- **総テスト数**: {total_tests}")
    sections.append(f"- **成功**: {passed}")
    sections.append(f"- **失敗**: {failed}")
    sections.append(f"- **タイムアウト**: {timed_out}")
    sections.append("")
    
    # Details
    sections.append("### 詳細")
    sections.append(f"- Workflow: {workflow}")
    sections.append(f"- Job: {job}")
    sections.append(f"- Run: {server_url}/{repository}/actions/runs/{run_id}")
    sections.append(f"- Commit: {commit}")
    sections.append(f"- Ref: {ref}")
    sections.append("")
    
    # Detailed error messages in collapsible section
    if error_details and error_details.strip():
        sections.append("<details>")
        sections.append("<summary>詳細なエラーメッセージ（クリックして展開）</summary>")
        sections.append("")
        sections.append(error_details)
        sections.append("")
        sections.append("</details>")
        sections.append("")
    
    # Artifacts
    sections.append("**アーティファクト**: 完全なログは上記のRunリンクから `test-logs` をダウンロード")
    
    return "\n".join(sections)


def _read_from_file(file_path: str) -> str:
    """
    Read error data from file generated by prior job.
    
    Args:
        file_path: Path to file containing error data from prior job
    
    Returns:
        The content from file
        
    Raises:
        FileNotFoundError: If file doesn't exist
        IOError: If file cannot be read
    """
    with open(file_path, 'r', encoding='utf-8') as f:
        return f.read()


def main():
    """
    Main entry point for the script.
    
    This script receives error data from prior GitHub Actions jobs via temporary files.
    The error data (simple failed test list and error details) are written to temporary
    files by the workflow and passed to this script as file path arguments.
    """
    parser = argparse.ArgumentParser(
        description="Generate issue body text for CI test failures. "
                    "Receives error data from prior job via temporary files."
    )
    
    parser.add_argument(
        "--status-ja",
        required=True,
        help="Status in Japanese (e.g., '失敗' or 'タイムアウトによりキャンセル')"
    )
    parser.add_argument(
        "--total-tests",
        required=True,
        help="Total number of tests run"
    )
    parser.add_argument(
        "--passed",
        required=True,
        help="Number of passed tests"
    )
    parser.add_argument(
        "--failed",
        required=True,
        help="Number of failed tests"
    )
    parser.add_argument(
        "--timed-out",
        required=True,
        help="Number of timed out tests"
    )
    parser.add_argument(
        "--failed-tests-list-file",
        required=True,
        help="Path to temporary file containing simple list of failed tests (generated by prior job)"
    )
    parser.add_argument(
        "--error-details-file",
        required=True,
        help="Path to temporary file containing detailed error messages (generated by prior job)"
    )
    parser.add_argument(
        "--workflow",
        required=True,
        help="GitHub workflow name"
    )
    parser.add_argument(
        "--job",
        required=True,
        help="GitHub job name"
    )
    parser.add_argument(
        "--run-id",
        required=True,
        help="GitHub run ID"
    )
    parser.add_argument(
        "--run-attempt",
        required=True,
        help="GitHub run attempt number"
    )
    parser.add_argument(
        "--ref",
        required=True,
        help="GitHub ref (branch/tag)"
    )
    parser.add_argument(
        "--commit",
        required=True,
        help="GitHub commit SHA"
    )
    parser.add_argument(
        "--server-url",
        required=True,
        help="GitHub server URL"
    )
    parser.add_argument(
        "--repository",
        required=True,
        help="GitHub repository (owner/repo)"
    )
    
    args = parser.parse_args()
    
    # Read error data from files generated by prior job
    failed_tests_list = _read_from_file(args.failed_tests_list_file)
    error_details = _read_from_file(args.error_details_file)
    
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
