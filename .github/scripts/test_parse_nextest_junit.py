#!/usr/bin/env python3
"""
Unit tests for parse_nextest_junit.py
"""

import sys
import unittest
from io import StringIO
from pathlib import Path
import tempfile
import xml.etree.ElementTree as ET

# Add parent directory to path to import the module
sys.path.insert(0, str(Path(__file__).parent))

from parse_nextest_junit import parse_junit_xml, format_failed_tests_list, format_failed_tests_with_errors


class TestParseJunitXml(unittest.TestCase):
    """Test the parse_junit_xml function."""
    
    def create_junit_xml(self, tests: int, failures: int, errors: int = 0, 
                         failed_test_data: list = None) -> str:
        """Helper to create a temporary JUnit XML file."""
        if failed_test_data is None:
            failed_test_data = [
                {
                    'name': f'test_{i}',
                    'classname': 'test_module',
                    'message': 'Test failed',
                    'details': 'Error details'
                }
                for i in range(failures)
            ]
        
        root = ET.Element("testsuites")
        testsuite = ET.SubElement(root, "testsuite", {
            "name": "test-suite",
            "tests": str(tests),
            "failures": str(failures),
            "errors": str(errors),
            "skipped": "0"
        })
        
        # Add passed tests
        passed_count = tests - failures - errors
        for i in range(passed_count):
            ET.SubElement(testsuite, "testcase", {
                "name": f"passed_test_{i}",
                "classname": "test_module"
            })
        
        # Add failed tests
        for test_data in failed_test_data:
            testcase = ET.SubElement(testsuite, "testcase", {
                "name": test_data['name'],
                "classname": test_data['classname']
            })
            failure = ET.SubElement(testcase, "failure", {
                "message": test_data['message']
            })
            failure.text = test_data['details']
        
        # Write to temp file with proper cleanup
        temp_file = tempfile.NamedTemporaryFile(mode='w', suffix='.xml', delete=False)
        try:
            tree = ET.ElementTree(root)
            tree.write(temp_file, encoding='unicode', xml_declaration=True)
            temp_file.close()
            return temp_file.name
        except:
            # Ensure file is closed and cleaned up on error
            temp_file.close()
            if Path(temp_file.name).exists():
                Path(temp_file.name).unlink()
            raise
    
    def test_basic_parsing(self):
        """Test basic JUnit XML parsing."""
        failed_test_data = [
            {
                'name': 'server_basic_test',
                'classname': 'tests::integration',
                'message': 'Connection refused',
                'details': 'Server not responding'
            },
            {
                'name': 'client_connect',
                'classname': 'tests::integration',
                'message': 'Failed to connect',
                'details': 'Network error'
            }
        ]
        junit_file = self.create_junit_xml(10, 2, 0, failed_test_data)
        
        try:
            stats, failed_tests = parse_junit_xml(junit_file)
            
            self.assertEqual(stats['total_tests'], '10')
            self.assertEqual(stats['passed'], '8')
            self.assertEqual(stats['failed'], '2')
            self.assertEqual(stats['timed_out'], '0')
            
            self.assertEqual(len(failed_tests), 2)
            self.assertEqual(failed_tests[0]['name'], 'tests::integration::server_basic_test')
            self.assertEqual(failed_tests[0]['message'], 'Connection refused')
            self.assertEqual(failed_tests[0]['details'], 'Server not responding')
            self.assertFalse(failed_tests[0]['is_timeout'])
        finally:
            Path(junit_file).unlink()
    
    def test_timeout_detection(self):
        """Test that timeouts are detected from failure messages."""
        failed_test_data = [
            {
                'name': 'test_timeout',
                'classname': 'test_module',
                'message': 'Test timed out after 60 seconds',
                'details': 'Timeout details'
            },
            {
                'name': 'test_failure',
                'classname': 'test_module',
                'message': 'Assertion failed',
                'details': 'Regular failure'
            }
        ]
        junit_file = self.create_junit_xml(2, 2, 0, failed_test_data)
        
        try:
            stats, failed_tests = parse_junit_xml(junit_file)
            
            # Both tests are counted as failures in JUnit XML
            # The timeout count is extracted separately from failure messages
            self.assertEqual(stats['total_tests'], '2')
            self.assertEqual(stats['passed'], '0')
            self.assertEqual(stats['failed'], '2')
            self.assertEqual(stats['timed_out'], '1')
            
            # Check timeout flag
            self.assertTrue(failed_tests[0]['is_timeout'])
            self.assertFalse(failed_tests[1]['is_timeout'])
        finally:
            Path(junit_file).unlink()


class TestFormatFailedTestsList(unittest.TestCase):
    """Test the format_failed_tests_list function."""
    
    def test_formatting(self):
        """Test that failed tests list is formatted correctly."""
        failed_tests = [
            {'name': 'test1', 'message': 'Error1', 'details': 'Details1', 'is_timeout': False},
            {'name': 'test2', 'message': 'Error2', 'details': 'Details2', 'is_timeout': True},
        ]
        
        output = format_failed_tests_list(failed_tests)
        
        self.assertIn("- test1", output)
        self.assertIn("- test2 (タイムアウト)", output)
    
    def test_empty_list(self):
        """Test that empty list returns empty string."""
        output = format_failed_tests_list([])
        self.assertEqual(output, "")


class TestFormatFailedTestsWithErrors(unittest.TestCase):
    """Test the format_failed_tests_with_errors function."""
    
    def test_formatting(self):
        """Test that failed tests with errors are formatted correctly."""
        failed_tests = [
            {
                'name': 'test1',
                'message': 'Connection failed',
                'details': 'Network error',
                'is_timeout': False
            },
            {
                'name': 'test2',
                'message': 'Timed out',
                'details': 'Exceeded limit',
                'is_timeout': True
            }
        ]
        
        output = format_failed_tests_with_errors(failed_tests)
        
        self.assertIn("### test1", output)
        self.assertIn("**Error**: Connection failed", output)
        self.assertIn("Network error", output)
        self.assertIn("### test2 (タイムアウト)", output)
        self.assertIn("**Error**: Timed out", output)
    
    def test_empty_list(self):
        """Test that empty list returns empty string."""
        output = format_failed_tests_with_errors([])
        self.assertEqual(output, "")


if __name__ == '__main__':
    unittest.main()
