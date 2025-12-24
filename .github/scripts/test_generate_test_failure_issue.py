#!/usr/bin/env python3
"""
Unit tests for generate_test_failure_issue.py
"""

import unittest
from generate_test_failure_issue import generate_issue_body


class TestGenerateIssueBody(unittest.TestCase):
    """Test cases for the generate_issue_body function."""
    
    def test_basic_failure(self):
        """Test basic failure case with minimal data."""
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="8",
            failed="2",
            timed_out="0",
            failed_tests_categorized="#### Server Tests (2件)\n- test_server_1\n- test_server_2",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123def456",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
            error_log=None,
        )
        
        # Check that key sections are present
        self.assertIn("Windows CI でビルドまたはテストに失敗しました", result)
        self.assertIn("**ステータス**: 失敗", result)
        self.assertIn("## 失敗テストサマリー", result)
        self.assertIn("**総テスト数**: 10", result)
        self.assertIn("**成功**: 8", result)
        self.assertIn("**失敗**: 2", result)
        self.assertIn("**タイムアウト**: 0", result)
        self.assertIn("### 失敗したテスト一覧", result)
        self.assertIn("#### Server Tests (2件)", result)
        self.assertIn("test_server_1", result)
        self.assertIn("test_server_2", result)
        self.assertIn("## ログへのリンク", result)
        self.assertIn("https://github.com/cat2151/ym2151-log-play-server/actions/runs/123456", result)
        self.assertIn("## 詳細", result)
        self.assertIn("- Workflow: Windows CI", result)
        self.assertIn("- Job: build-windows", result)
        self.assertIn("- Run ID: 123456", result)
        self.assertIn("- Run Attempt: 1", result)
        self.assertIn("- Ref: refs/heads/main", result)
        self.assertIn("- Commit: abc123def456", result)
        self.assertIn("## アーティファクト", result)
        self.assertIn("完全なログは上記リンクの「Artifacts」セクションから `test-logs` をダウンロードしてください", result)
        
        # Check that error log section is NOT present when error_log is None
        self.assertNotIn("## 詳細なエラーログ", result)
    
    def test_timeout_status(self):
        """Test timeout status."""
        result = generate_issue_body(
            status_ja="タイムアウトによりキャンセル",
            total_tests="5",
            passed="3",
            failed="0",
            timed_out="2",
            failed_tests_categorized="#### Timeout Tests (2件)\n- test_timeout_1\n- test_timeout_2",
            workflow="Windows CI",
            job="build-windows",
            run_id="789012",
            run_attempt="2",
            ref="refs/heads/feature/test",
            commit="def456abc789",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
            error_log=None,
        )
        
        self.assertIn("**ステータス**: タイムアウトによりキャンセル", result)
        self.assertIn("**タイムアウト**: 2", result)
        self.assertIn("test_timeout_1", result)
        self.assertIn("test_timeout_2", result)
    
    def test_with_error_log(self):
        """Test with detailed error log."""
        error_log = "Error: Test failed\nStack trace:\n  at function1()\n  at function2()"
        
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_categorized="#### Tests (1件)\n- test_fail",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
            error_log=error_log,
        )
        
        # Check that error log section IS present
        self.assertIn("## 詳細なエラーログ", result)
        self.assertIn("<details>", result)
        self.assertIn("<summary>クリックして展開</summary>", result)
        self.assertIn("```", result)
        self.assertIn(error_log, result)
        self.assertIn("</details>", result)
    
    def test_with_empty_error_log(self):
        """Test with empty error log (should not show error log section)."""
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_categorized="#### Tests (1件)\n- test_fail",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
            error_log="",
        )
        
        # Check that error log section is NOT present when error_log is empty
        self.assertNotIn("## 詳細なエラーログ", result)
    
    def test_with_whitespace_error_log(self):
        """Test with whitespace-only error log (should not show error log section)."""
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_categorized="#### Tests (1件)\n- test_fail",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
            error_log="   \n  \t  \n  ",
        )
        
        # Check that error log section is NOT present when error_log is whitespace only
        self.assertNotIn("## 詳細なエラーログ", result)
    
    def test_multiple_categories(self):
        """Test with multiple test categories."""
        categorized = """#### Server Tests (2件)
- server_test_1
- server_test_2

#### Client Tests (1件)
- client_test_1

#### その他 (1件)
- misc_test_1"""
        
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="6",
            failed="4",
            timed_out="0",
            failed_tests_categorized=categorized,
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
            error_log=None,
        )
        
        # Check that all categories are preserved
        self.assertIn("#### Server Tests (2件)", result)
        self.assertIn("server_test_1", result)
        self.assertIn("server_test_2", result)
        self.assertIn("#### Client Tests (1件)", result)
        self.assertIn("client_test_1", result)
        self.assertIn("#### その他 (1件)", result)
        self.assertIn("misc_test_1", result)
    
    def test_special_characters_in_test_names(self):
        """Test with special characters in test names."""
        categorized = "#### Tests (1件)\n- test::with::colons\n- test_with_underscores"
        
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="8",
            failed="2",
            timed_out="0",
            failed_tests_categorized=categorized,
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
            error_log=None,
        )
        
        # Check that test names with special characters are preserved
        self.assertIn("test::with::colons", result)
        self.assertIn("test_with_underscores", result)


if __name__ == "__main__":
    unittest.main()
