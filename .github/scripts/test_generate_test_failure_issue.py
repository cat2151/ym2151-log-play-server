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
            failed_tests_list="- test_server_1\n- test_server_2",
            error_details="### test_server_1\n**Error**: Connection failed\n\n### test_server_2\n**Error**: Timeout",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123def456",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
        )
        
        # Check that key sections are present
        self.assertIn("## 失敗したテスト", result)
        self.assertIn("test_server_1", result)
        self.assertIn("test_server_2", result)
        self.assertIn("**ステータス**: 失敗", result)
        self.assertIn("### テストサマリー", result)
        self.assertIn("- **総テスト数**: 10", result)
        self.assertIn("- **成功**: 8", result)
        self.assertIn("- **失敗**: 2", result)
        self.assertIn("- **タイムアウト**: 0", result)
        self.assertIn("### 詳細", result)
        self.assertIn("- Workflow: Windows CI", result)
        self.assertIn("- Job: build-windows", result)
        self.assertIn("- Run: https://github.com/cat2151/ym2151-log-play-server/actions/runs/123456", result)
        self.assertIn("- Commit: abc123def456", result)
        self.assertIn("- Ref: refs/heads/main", result)
        self.assertIn("<details>", result)
        self.assertIn("詳細なエラーメッセージ", result)
        self.assertIn("**アーティファクト**", result)
    
    def test_timeout_status(self):
        """Test timeout status."""
        result = generate_issue_body(
            status_ja="タイムアウトによりキャンセル",
            total_tests="5",
            passed="3",
            failed="0",
            timed_out="2",
            failed_tests_list="- test_timeout_1 (タイムアウト)\n- test_timeout_2 (タイムアウト)",
            error_details="### test_timeout_1 (タイムアウト)\n**Error**: Timed out",
            workflow="Windows CI",
            job="build-windows",
            run_id="789012",
            run_attempt="2",
            ref="refs/heads/feature/test",
            commit="def456abc789",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
        )
        
        self.assertIn("**ステータス**: タイムアウトによりキャンセル", result)
        self.assertIn("- **タイムアウト**: 2", result)
        self.assertIn("test_timeout_1 (タイムアウト)", result)
        self.assertIn("test_timeout_2 (タイムアウト)", result)
    
    def test_with_empty_error_details(self):
        """Test with empty error details (should show minimal info)."""
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_list="- test_fail",
            error_details="",
            workflow="Windows CI",
            job="build-windows",
            run_id="111222",
            run_attempt="1",
            ref="refs/heads/main",
            commit="111222333",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
        )
        
        self.assertIn("- test_fail", result)
        # Should not have details section when error_details is empty
        self.assertNotIn("<details>", result)
    
    def test_with_whitespace_error_details(self):
        """Test with whitespace-only error details."""
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_list="- test_fail",
            error_details="   \n  \n ",
            workflow="Windows CI",
            job="build-windows",
            run_id="111222",
            run_attempt="1",
            ref="refs/heads/main",
            commit="111222333",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
        )
        
        # Should not have details section when error_details is only whitespace
        self.assertNotIn("<details>", result)


class TestGenerateIssueBodyWithTranslation(unittest.TestCase):
    """Test cases for issue body generation with Gemini translation."""
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_with_gemini_translation_success(self, mock_translate):
        """Test issue body generation with successful Gemini translation."""
        mock_translate.return_value = "日本語訳されたエラーメッセージ"
        
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_list="- test_fail",
            error_details="### test_fail\n**Error**: Test failed",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
        )
        
        # Check that Gemini translation is included
        self.assertIn("## 🤖 エラーメッセージの日本語訳（AI生成）", result)
        self.assertIn("日本語訳されたエラーメッセージ", result)
        
        # Verify the mock was called with error_details
        mock_translate.assert_called_once()
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_with_gemini_translation_failure(self, mock_translate):
        """Test issue body generation when Gemini translation fails."""
        mock_translate.return_value = None
        
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_list="- test_fail",
            error_details="### test_fail\n**Error**: Test failed",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
        )
        
        # Check that Gemini translation section is NOT included when translation fails
        self.assertNotIn("## 🤖 エラーメッセージの日本語訳（AI生成）", result)
        # But the regular content should still be there
        self.assertIn("## 失敗したテスト", result)
        self.assertIn("- test_fail", result)
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_without_error_details(self, mock_translate):
        """Test issue body generation without error details."""
        result = generate_issue_body(
            status_ja="失敗",
            total_tests="10",
            passed="9",
            failed="1",
            timed_out="0",
            failed_tests_list="- test_fail",
            error_details="",
            workflow="Windows CI",
            job="build-windows",
            run_id="123456",
            run_attempt="1",
            ref="refs/heads/main",
            commit="abc123",
            server_url="https://github.com",
            repository="cat2151/ym2151-log-play-server",
        )
        
        # Translation should not be attempted when there's no error details
        mock_translate.assert_not_called()


class TestTranslateErrorMessages(unittest.TestCase):
    """Test cases for the translate_error_messages_with_gemini function."""
    
    def test_translate_with_no_api_key(self):
        """Test that translation returns None when API key is not in environment."""
        # Ensure GEMINI_API_KEY is not set
        original_key = os.environ.get('GEMINI_API_KEY')
        if 'GEMINI_API_KEY' in os.environ:
            del os.environ['GEMINI_API_KEY']
        
        try:
            result = translate_error_messages_with_gemini("Some error log")
            self.assertIsNone(result)
        finally:
            # Restore original key if it existed
            if original_key is not None:
                os.environ['GEMINI_API_KEY'] = original_key
    
    def test_translate_with_no_error_log(self):
        """Test that translation returns None when error log is empty."""
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        try:
            result = translate_error_messages_with_gemini("")
            self.assertIsNone(result)
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']
    
    def test_translate_with_whitespace_error_log(self):
        """Test that translation returns None when error log is whitespace only."""
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        try:
            result = translate_error_messages_with_gemini("   \n\t  ")
            self.assertIsNone(result)
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']
    
    @patch('generate_test_failure_issue.urllib.request.urlopen')
    def test_translate_success(self, mock_urlopen):
        """Test successful translation with Gemini API."""
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        # Mock successful API response
        mock_response = MagicMock()
        mock_response.read.return_value = b'{"candidates":[{"content":{"parts":[{"text":"\\u65e5\\u672c\\u8a9e\\u8a33"}]}}]}'
        mock_response.__enter__.return_value = mock_response
        mock_urlopen.return_value = mock_response
        
        try:
            result = translate_error_messages_with_gemini("Error message")
            self.assertEqual(result, "日本語訳")
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']
    
    @patch('generate_test_failure_issue.urllib.request.urlopen')
    def test_translate_malformed_response(self, mock_urlopen):
        """Test that translation handles malformed API responses."""
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        # Mock malformed API response
        mock_response = MagicMock()
        mock_response.read.return_value = b'{"invalid": "response"}'
        mock_response.__enter__.return_value = mock_response
        mock_urlopen.return_value = mock_response
        
        try:
            result = translate_error_messages_with_gemini("Error message")
            self.assertIsNone(result)
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']
    
    @patch('generate_test_failure_issue.urllib.request.urlopen')
    @patch('generate_test_failure_issue.time.sleep')
    def test_translate_api_error_with_retry(self, mock_sleep, mock_urlopen):
        """Test that translation retries with exponential backoff on API errors."""
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        # Mock API error (all retries fail)
        mock_urlopen.side_effect = urllib.error.URLError("Connection failed")
        
        try:
            result = translate_error_messages_with_gemini("Error message")
            self.assertIsNone(result)
            
            # Verify retries happened (8 attempts total, so 7 sleeps)
            self.assertEqual(mock_sleep.call_count, 7)
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']


class TestReadFromFile(unittest.TestCase):
    """Test cases for the _read_from_file function."""
    
    def test_read_from_file(self):
        """Test that content is read from file."""
        from generate_test_failure_issue import _read_from_file
        import tempfile
        
        with tempfile.NamedTemporaryFile(mode='w', delete=False, encoding='utf-8') as f:
            f.write("Test content\nLine 2")
            temp_path = f.name
        
        try:
            content = _read_from_file(temp_path)
            self.assertEqual(content, "Test content\nLine 2")
        finally:
            os.unlink(temp_path)
    
    def test_file_not_found_raises_error(self):
        """Test that FileNotFoundError is raised when file doesn't exist."""
        from generate_test_failure_issue import _read_from_file
        
        with self.assertRaises(FileNotFoundError):
            _read_from_file("/nonexistent/path/file.txt")


if __name__ == '__main__':
    unittest.main()
