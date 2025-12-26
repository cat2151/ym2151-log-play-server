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

from parse_nextest_junit import categorize_test, parse_junit_xml, format_categorized_output


class TestCategorizeTest(unittest.TestCase):
    """Test the categorize_test function."""
    
    def test_pipe_tests(self):
        """Test that pipe-related tests are categorized correctly."""
        self.assertEqual(categorize_test("pipe_test::basic"), "Pipe Tests")
        self.assertEqual(categorize_test("test_pipe_connection"), "Pipe Tests")
    
    def test_cli_integration_tests(self):
        """Test that CLI integration tests are categorized correctly."""
        self.assertEqual(categorize_test("cli_integration::test_basic"), "CLI Integration Tests")
    
    def test_client_tests(self):
        """Test that client tests are categorized correctly."""
        self.assertEqual(categorize_test("client_test::basic"), "Client Integration Tests")
        self.assertEqual(categorize_test("test_client_connect"), "Client Integration Tests")
    
    def test_interactive_tests(self):
        """Test that interactive tests are categorized correctly."""
        self.assertEqual(categorize_test("interactive::test_mode"), "Interactive Mode Tests")
    
    def test_server_tests(self):
        """Test that server tests are categorized correctly."""
        self.assertEqual(categorize_test("server_test::basic"), "Server Integration Tests")
        self.assertEqual(categorize_test("test_server_start"), "Server Integration Tests")
    
    def test_other_tests(self):
        """Test that other tests are categorized correctly."""
        self.assertEqual(categorize_test("audio::test_playback"), "その他")
        self.assertEqual(categorize_test("random_test"), "その他")


class TestParseJunitXml(unittest.TestCase):
    """Test the parse_junit_xml function."""
    
    def create_junit_xml(self, tests: int, failures: int, errors: int = 0, 
                         failed_test_names: list = None) -> str:
        """Helper to create a temporary JUnit XML file."""
        if failed_test_names is None:
            failed_test_names = [f"test_{i}" for i in range(failures)]
        
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
        for name in failed_test_names:
            testcase = ET.SubElement(testsuite, "testcase", {
                "name": name,
                "classname": "test_module"
            })
            ET.SubElement(testcase, "failure", {
                "message": "Test failed"
            })
        
        # Write to temp file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.xml', delete=False) as f:
            tree = ET.ElementTree(root)
            tree.write(f, encoding='unicode', xml_declaration=True)
            return f.name
    
    def test_basic_parsing(self):
        """Test basic JUnit XML parsing."""
        junit_file = self.create_junit_xml(10, 2, 0, ["server_test::basic", "client_test::connect"])
        
        try:
            stats, categorized = parse_junit_xml(junit_file)
            
            self.assertEqual(stats['total_tests'], '10')
            self.assertEqual(stats['passed'], '8')
            self.assertEqual(stats['failed'], '2')
            self.assertEqual(stats['timed_out'], '0')
            
            self.assertEqual(len(categorized['Server Integration Tests']), 1)
            self.assertEqual(len(categorized['Client Integration Tests']), 1)
        finally:
            Path(junit_file).unlink()
    
    def test_timeout_detection(self):
        """Test that timeouts are detected from failure messages."""
        root = ET.Element("testsuites")
        testsuite = ET.SubElement(root, "testsuite", {
            "name": "test-suite",
            "tests": "2",
            "failures": "2",
            "errors": "0",
            "skipped": "0"
        })
        
        # Test with timeout in failure message
        testcase1 = ET.SubElement(testsuite, "testcase", {
            "name": "test_timeout",
            "classname": "test_module"
        })
        ET.SubElement(testcase1, "failure", {
            "message": "Test timed out after 60 seconds"
        })
        
        # Regular failure
        testcase2 = ET.SubElement(testsuite, "testcase", {
            "name": "test_failure",
            "classname": "test_module"
        })
        ET.SubElement(testcase2, "failure", {
            "message": "Assertion failed"
        })
        
        with tempfile.NamedTemporaryFile(mode='w', suffix='.xml', delete=False) as f:
            tree = ET.ElementTree(root)
            tree.write(f, encoding='unicode', xml_declaration=True)
            junit_file = f.name
        
        try:
            stats, categorized = parse_junit_xml(junit_file)
            
            self.assertEqual(stats['total_tests'], '2')
            self.assertEqual(stats['passed'], '0')
            self.assertEqual(stats['failed'], '1')
            self.assertEqual(stats['timed_out'], '1')
        finally:
            Path(junit_file).unlink()


class TestFormatCategorizedOutput(unittest.TestCase):
    """Test the format_categorized_output function."""
    
    def test_formatting(self):
        """Test that categorized output is formatted correctly."""
        categorized = {
            "Server Integration Tests": ["server_test::basic", "server_test::connect"],
            "Client Integration Tests": ["client_test::basic"],
            "Pipe Tests": [],
            "CLI Integration Tests": [],
            "Interactive Mode Tests": [],
            "その他": []
        }
        
        output = format_categorized_output(categorized)
        
        self.assertIn("#### Server Integration Tests (2件)", output)
        self.assertIn("- server_test::basic", output)
        self.assertIn("- server_test::connect", output)
        self.assertIn("#### Client Integration Tests (1件)", output)
        self.assertIn("- client_test::basic", output)
        # Empty categories should not appear
        self.assertNotIn("#### Pipe Tests", output)


if __name__ == '__main__':
    unittest.main()
