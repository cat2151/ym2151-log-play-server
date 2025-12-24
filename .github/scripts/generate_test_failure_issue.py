#!/usr/bin/env python3
"""
Generate issue body text for CI test failures.

This script generates the issue body for GitHub issues created when
Windows CI tests fail or time out.
"""

import argparse
import sys
from typing import Optional


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
    
    Returns:
        The formatted issue body text
    """
    
    # Build the main sections
    sections = []
    
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
    )
    
    print(issue_body)
    return 0


if __name__ == "__main__":
    sys.exit(main())
