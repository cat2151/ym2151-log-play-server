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
import urllib.request
import urllib.error
from typing import Optional


def translate_error_messages_with_gemini(error_log: str, api_key: str) -> Optional[str]:
    """
    Translate error messages to Japanese using Gemini API.
    
    Args:
        error_log: The error log text to translate
        api_key: Gemini API key
    
    Returns:
        Translated text in Japanese, or None if translation fails
    """
    if not error_log or not error_log.strip():
        return None
    
    if not api_key or not api_key.strip():
        return None
    
    try:
        # Prepare the API request
        url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={api_key}"
        
        # Create the prompt for translation
        prompt = f"""以下は、Windowsビルド環境でのRustプロジェクトのテスト失敗ログです。
このエラーログを日本語に翻訳してください。
技術用語は適切に翻訳し、開発者が理解しやすいように要約してください。
エラーの主な原因と失敗したテストについて簡潔に説明してください。

エラーログ:
```
{error_log[:3000]}
```

日本語訳:"""
        
        # Prepare request data
        data = {
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }],
            "generationConfig": {
                "temperature": 0.3,
                "maxOutputTokens": 1000
            }
        }
        
        # Make the API request
        req = urllib.request.Request(
            url,
            data=json.dumps(data).encode('utf-8'),
            headers={'Content-Type': 'application/json'}
        )
        
        with urllib.request.urlopen(req, timeout=30) as response:
            result = json.loads(response.read().decode('utf-8'))
            
            # Extract the translated text
            if 'candidates' in result and len(result['candidates']) > 0:
                candidate = result['candidates'][0]
                if 'content' in candidate and 'parts' in candidate['content']:
                    parts = candidate['content']['parts']
                    if len(parts) > 0 and 'text' in parts[0]:
                        return parts[0]['text'].strip()
        
        return None
    
    except urllib.error.URLError as e:
        print(f"Warning: Failed to translate with Gemini API: {e}", file=sys.stderr)
        return None
    except json.JSONDecodeError as e:
        print(f"Warning: Failed to parse Gemini API response: {e}", file=sys.stderr)
        return None
    except Exception as e:
        print(f"Warning: Unexpected error during translation: {e}", file=sys.stderr)
        return None


def generate_issue_body(
    status_ja: str,
    total_tests: str,
    passed: str,
    failed: str,
    timed_out: str,
    failed_tests_categorized: str,
    workflow: str,
    job: str,
    run_id: str,
    run_attempt: str,
    ref: str,
    commit: str,
    server_url: str,
    repository: str,
    error_log: Optional[str] = None,
    gemini_api_key: Optional[str] = None,
) -> str:
    """
    Generate the issue body text for a test failure.
    
    Args:
        status_ja: Status in Japanese (e.g., "失敗" or "タイムアウトによりキャンセル")
        total_tests: Total number of tests run
        passed: Number of passed tests
        failed: Number of failed tests
        timed_out: Number of timed out tests
        failed_tests_categorized: Categorized list of failed tests (markdown formatted)
        workflow: GitHub workflow name
        job: GitHub job name
        run_id: GitHub run ID
        run_attempt: GitHub run attempt number
        ref: GitHub ref (branch/tag)
        commit: GitHub commit SHA
        server_url: GitHub server URL
        repository: GitHub repository (owner/repo)
        error_log: Optional detailed error log
        gemini_api_key: Optional Gemini API key for translation
    
    Returns:
        The formatted issue body text
    """
    
    # Build the main sections
    sections = []
    
    # If Gemini API key is provided, try to translate error messages
    if gemini_api_key and error_log:
        japanese_translation = translate_error_messages_with_gemini(error_log, gemini_api_key)
        if japanese_translation:
            sections.append("## 🤖 エラーメッセージの日本語訳（AI生成）")
            sections.append("")
            sections.append(japanese_translation)
            sections.append("")
            sections.append("---")
            sections.append("")
    
    # Header
    sections.append("Windows CI でビルドまたはテストに失敗しました。")
    sections.append("")
    sections.append(f"**ステータス**: {status_ja}")
    sections.append("")
    
    # Test Summary
    sections.append("## 失敗テストサマリー")
    sections.append("")
    sections.append(f"**総テスト数**: {total_tests}")
    sections.append(f"**成功**: {passed}")
    sections.append(f"**失敗**: {failed}")
    sections.append(f"**タイムアウト**: {timed_out}")
    sections.append("")
    
    # Failed Tests List
    sections.append("### 失敗したテスト一覧")
    sections.append(failed_tests_categorized)
    sections.append("")
    
    # Log Link
    sections.append("## ログへのリンク")
    sections.append(f"{server_url}/{repository}/actions/runs/{run_id}")
    sections.append("")
    
    # Details
    sections.append("## 詳細")
    sections.append(f"- Workflow: {workflow}")
    sections.append(f"- Job: {job}")
    sections.append(f"- Run ID: {run_id}")
    sections.append(f"- Run Attempt: {run_attempt}")
    sections.append(f"- Ref: {ref}")
    sections.append(f"- Commit: {commit}")
    sections.append("")
    
    # Detailed Error Log (if provided)
    if error_log and error_log.strip():
        sections.append("## 詳細なエラーログ")
        sections.append("<details>")
        sections.append("<summary>クリックして展開</summary>")
        sections.append("")
        sections.append("```")
        sections.append(error_log)
        sections.append("```")
        sections.append("")
        sections.append("</details>")
        sections.append("")
    
    # Artifacts
    sections.append("## アーティファクト")
    sections.append("完全なログは上記リンクの「Artifacts」セクションから `test-logs` をダウンロードしてください。")
    
    return "\n".join(sections)


def main():
    """Main entry point for the script."""
    parser = argparse.ArgumentParser(
        description="Generate issue body text for CI test failures"
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
        "--failed-tests-categorized",
        required=True,
        help="Categorized list of failed tests (markdown formatted)"
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
    parser.add_argument(
        "--error-log",
        default="",
        help="Optional detailed error log"
    )
    parser.add_argument(
        "--gemini-api-key",
        default="",
        help="Optional Gemini API key for translating error messages"
    )
    
    args = parser.parse_args()
    
    issue_body = generate_issue_body(
        status_ja=args.status_ja,
        total_tests=args.total_tests,
        passed=args.passed,
        failed=args.failed,
        timed_out=args.timed_out,
        failed_tests_categorized=args.failed_tests_categorized,
        workflow=args.workflow,
        job=args.job,
        run_id=args.run_id,
        run_attempt=args.run_attempt,
        ref=args.ref,
        commit=args.commit,
        server_url=args.server_url,
        repository=args.repository,
        error_log=args.error_log,
        gemini_api_key=args.gemini_api_key,
    )
    
    print(issue_body)
    return 0


if __name__ == "__main__":
    sys.exit(main())
