#!/usr/bin/env python3
"""
Unit tests for generate_test_failure_issue.py
"""

import os
import unittest
import urllib.error
from unittest.mock import patch, MagicMock
from generate_test_failure_issue import generate_issue_body, translate_error_messages_with_gemini


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


class TestHelperFunctions(unittest.TestCase):
    """Test cases for helper functions."""
    
    @patch.dict(os.environ, {"TEST_VAR": "env_value"})
    def test_get_value_with_env_fallback_uses_arg(self):
        """Test that non-empty argument value is returned."""
        from generate_test_failure_issue import _get_value_with_env_fallback
        result = _get_value_with_env_fallback("arg_value", "TEST_VAR")
        self.assertEqual(result, "arg_value")
    
    @patch.dict(os.environ, {"TEST_VAR": "env_value"})
    def test_get_value_with_env_fallback_uses_env(self):
        """Test that environment variable is used when argument is empty."""
        from generate_test_failure_issue import _get_value_with_env_fallback
        result = _get_value_with_env_fallback("", "TEST_VAR")
        self.assertEqual(result, "env_value")
    
    @patch.dict(os.environ, {"TEST_VAR": "env_value"})
    def test_get_value_with_env_fallback_whitespace(self):
        """Test that environment variable is used when argument is whitespace."""
        from generate_test_failure_issue import _get_value_with_env_fallback
        result = _get_value_with_env_fallback("  \n  ", "TEST_VAR")
        self.assertEqual(result, "env_value")
    
    @patch.dict(os.environ, {}, clear=True)
    def test_get_value_with_env_fallback_no_env(self):
        """Test that empty string is returned when both arg and env are empty."""
        from generate_test_failure_issue import _get_value_with_env_fallback
        result = _get_value_with_env_fallback("", "NONEXISTENT_VAR")
        self.assertEqual(result, "")


class TestTranslateErrorMessages(unittest.TestCase):
    """Test cases for the translate_error_messages_with_gemini function."""
    
    @patch.dict(os.environ, {}, clear=True)
    def test_translate_with_no_api_key(self):
        """Test that translation returns None when API key is not in environment."""
        error_log = "Error: test failed"
        result = translate_error_messages_with_gemini(error_log)
        self.assertIsNone(result)
    
    @patch.dict(os.environ, {"GEMINI_API_KEY": "fake-api-key"})
    def test_translate_with_no_error_log(self):
        """Test that translation returns None when error log is empty."""
        result = translate_error_messages_with_gemini("")
        self.assertIsNone(result)
    
    @patch.dict(os.environ, {"GEMINI_API_KEY": "fake-api-key"})
    def test_translate_with_whitespace_error_log(self):
        """Test that translation returns None when error log is whitespace only."""
        result = translate_error_messages_with_gemini("   \n  ")
        self.assertIsNone(result)
    
    @patch.dict(os.environ, {"GEMINI_API_KEY": "fake-api-key"})
    @patch('urllib.request.urlopen')
    def test_translate_success(self, mock_urlopen):
        """Test successful translation with Gemini API."""
        # Mock API response
        mock_response = MagicMock()
        mock_response.read.return_value = b'''
        {
            "candidates": [{
                "content": {
                    "parts": [{
                        "text": "\\u30c6\\u30b9\\u30c8\\u304c\\u5931\\u6557\\u3057\\u307e\\u3057\\u305f"
                    }]
                }
            }]
        }
        '''
        mock_response.__enter__ = MagicMock(return_value=mock_response)
        mock_response.__exit__ = MagicMock(return_value=False)
        mock_urlopen.return_value = mock_response
        
        error_log = "Error: test failed"
        result = translate_error_messages_with_gemini(error_log)
        
        self.assertIsNotNone(result)
        self.assertEqual(result, "テストが失敗しました")
    
    @patch.dict(os.environ, {"GEMINI_API_KEY": "fake-api-key"})
    @patch('urllib.request.urlopen')
    @patch('time.sleep')  # Mock sleep to speed up test
    def test_translate_api_error_with_retry(self, mock_sleep, mock_urlopen):
        """Test that translation retries with exponential backoff on API errors."""
        mock_urlopen.side_effect = urllib.error.URLError("Connection failed")
        
        error_log = "Error: test failed"
        result = translate_error_messages_with_gemini(error_log)
        
        # Should return None after max retries
        self.assertIsNone(result)
        # Should have attempted 8 times (updated from 5)
        self.assertEqual(mock_urlopen.call_count, 8)
        # Should have called sleep 7 times (between retries)
        self.assertEqual(mock_sleep.call_count, 7)
        # Verify exponential backoff with 60s base and 7200s max
        expected_delays = [60.0, 120.0, 240.0, 480.0, 960.0, 1920.0, 3840.0]
        actual_delays = [call[0][0] for call in mock_sleep.call_args_list]
        self.assertEqual(actual_delays, expected_delays)
    
    @patch.dict(os.environ, {"GEMINI_API_KEY": "fake-api-key"})
    @patch('urllib.request.urlopen')
    def test_translate_malformed_response(self, mock_urlopen):
        """Test that translation handles malformed API responses."""
        mock_response = MagicMock()
        mock_response.read.return_value = b'{"invalid": "response"}'
        mock_response.__enter__ = MagicMock(return_value=mock_response)
        mock_response.__exit__ = MagicMock(return_value=False)
        mock_urlopen.return_value = mock_response
        
        error_log = "Error: test failed"
        result = translate_error_messages_with_gemini(error_log)
        
        # Should return None when response doesn't have expected structure
        self.assertIsNone(result)


class TestGenerateIssueBodyWithTranslation(unittest.TestCase):
    """Test cases for generate_issue_body with Gemini translation."""
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_with_gemini_translation_success(self, mock_translate):
        """Test issue body generation with successful Gemini translation."""
        mock_translate.return_value = "テストが失敗しました。詳細なエラー情報が含まれます。"
        
        error_log = "Error: test failed\nStack trace: at function()"
        
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
        
        # Check that translation section is present
        self.assertIn("## 🤖 エラーメッセージの日本語訳（AI生成）", result)
        self.assertIn("テストが失敗しました。詳細なエラー情報が含まれます。", result)
        self.assertIn("---", result)
        
        # Check that translation was called with correct parameters
        mock_translate.assert_called_once_with(error_log)
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_with_gemini_translation_failure(self, mock_translate):
        """Test issue body generation when Gemini translation fails."""
        mock_translate.return_value = None
        
        error_log = "Error: test failed"
        
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
        
        # Check that translation section is NOT present when translation fails
        self.assertNotIn("## 🤖 エラーメッセージの日本語訳（AI生成）", result)
        
        # But the rest of the issue should still be generated
        self.assertIn("Windows CI でビルドまたはテストに失敗しました", result)
    
    def test_without_error_log(self):
        """Test issue body generation without error log."""
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
            error_log=None,
        )
        
        # Check that translation section is NOT present without error log
        self.assertNotIn("## 🤖 エラーメッセージの日本語訳（AI生成）", result)
        
        # But the rest of the issue should still be generated
        self.assertIn("Windows CI でビルドまたはテストに失敗しました", result)


if __name__ == "__main__":
    unittest.main()
