#!/usr/bin/env python3

import os
import unittest
import urllib.error
import tempfile
from pathlib import Path
from unittest.mock import patch, MagicMock
from generate_test_failure_issue import generate_issue_body, translate_error_messages_with_gemini


class TestGenerateIssueBody(unittest.TestCase):
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_basic_failure(self, mock_translate):
        mock_translate.return_value = None
        
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
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_timeout_status(self, mock_translate):
        mock_translate.return_value = None
        
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
        self.assertNotIn("<details>", result)
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_with_whitespace_error_details(self, mock_translate):
        mock_translate.return_value = None
        
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
        
        self.assertNotIn("<details>", result)


class TestGenerateIssueBodyWithTranslation(unittest.TestCase):
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_with_gemini_translation_success(self, mock_translate):
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
        
        self.assertIn("## 🤖 エラーメッセージの日本語訳（AI生成）", result)
        self.assertIn("日本語訳されたエラーメッセージ", result)
        mock_translate.assert_called_once()
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_with_gemini_translation_failure(self, mock_translate):
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
        
        self.assertNotIn("## 🤖 エラーメッセージの日本語訳（AI生成）", result)
        self.assertIn("## 失敗したテスト", result)
        self.assertIn("- test_fail", result)
    
    @patch('generate_test_failure_issue.translate_error_messages_with_gemini')
    def test_without_error_details(self, mock_translate):
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
        
        mock_translate.assert_not_called()
    
    def test_with_missing_api_key_raises_error(self):
        original_key = os.environ.get('GEMINI_API_KEY')
        if 'GEMINI_API_KEY' in os.environ:
            del os.environ['GEMINI_API_KEY']
        
        try:
            with self.assertRaises(ValueError) as context:
                generate_issue_body(
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
            self.assertIn("GEMINI_API_KEY", str(context.exception))
        finally:
            if original_key is not None:
                os.environ['GEMINI_API_KEY'] = original_key


class TestTranslateErrorMessages(unittest.TestCase):
    def test_translate_with_no_api_key(self):
        original_key = os.environ.get('GEMINI_API_KEY')
        if 'GEMINI_API_KEY' in os.environ:
            del os.environ['GEMINI_API_KEY']
        
        try:
            with self.assertRaises(ValueError) as context:
                translate_error_messages_with_gemini("Some error log")
            self.assertIn("GEMINI_API_KEY", str(context.exception))
        finally:
            if original_key is not None:
                os.environ['GEMINI_API_KEY'] = original_key
    
    def test_translate_with_no_error_log(self):
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        try:
            result = translate_error_messages_with_gemini("")
            self.assertIsNone(result)
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']
    
    def test_translate_with_whitespace_error_log(self):
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        try:
            result = translate_error_messages_with_gemini("   \n\t  ")
            self.assertIsNone(result)
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']
    
    @patch('generate_test_failure_issue.urllib.request.urlopen')
    def test_translate_success(self, mock_urlopen):
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
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
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
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
        os.environ['GEMINI_API_KEY'] = "test-api-key"
        
        mock_urlopen.side_effect = urllib.error.URLError("Connection failed")
        
        try:
            result = translate_error_messages_with_gemini("Error message")
            self.assertIsNone(result)
            self.assertEqual(mock_sleep.call_count, 7)
        finally:
            if 'GEMINI_API_KEY' in os.environ:
                del os.environ['GEMINI_API_KEY']


if __name__ == '__main__':
    unittest.main()
