#!/usr/bin/env python3

import argparse
import sys
import xml.etree.ElementTree as ET
from typing import Dict, List, Tuple


def parse_junit_xml(junit_file_path: str) -> Tuple[Dict[str, str], List[Dict[str, str]]]:
    try:
        tree = ET.parse(junit_file_path)
        root = tree.getroot()
    except FileNotFoundError:
        print(f"Error: JUnit XML file not found: {junit_file_path}", file=sys.stderr)
        sys.exit(1)
    except ET.ParseError as e:
        print(f"Error: Invalid XML format in {junit_file_path}: {e}", file=sys.stderr)
        sys.exit(1)
    except PermissionError:
        print(f"Error: Permission denied reading {junit_file_path}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: Failed to parse JUnit XML file {junit_file_path}: {e}", file=sys.stderr)
        sys.exit(1)
    testsuite = root.find('.//testsuite')
    if testsuite is None:
        testsuite = root
    
    total_tests = testsuite.get('tests', '0')
    failures = int(testsuite.get('failures', '0'))
    errors = int(testsuite.get('errors', '0'))
    skipped = int(testsuite.get('skipped', '0'))
    
    total = int(total_tests)
    failed = failures + errors
    passed = total - failed - skipped
    
    statistics = {
        'total_tests': str(total),
        'passed': str(passed),
        'failed': str(failed),
        'timed_out': '0'
    }
    
    failed_tests = []
    
    for testcase in root.findall('.//testcase'):
        failure = testcase.find('failure')
        error = testcase.find('error')
        
        if failure is not None or error is not None:
            test_name = testcase.get('name', 'unknown')
            classname = testcase.get('classname', '')
            
            if classname:
                full_name = f"{classname}::{test_name}"
            else:
                full_name = test_name
            
            failure_elem = failure if failure is not None else error
            failure_message = failure_elem.get('message', '')
            failure_details = failure_elem.text or ''
            
            is_timeout = 'timeout' in failure_message.lower() or 'timed out' in failure_message.lower()
            if is_timeout:
                statistics['timed_out'] = str(int(statistics['timed_out']) + 1)
            
            failed_tests.append({
                'name': full_name,
                'message': failure_message,
                'details': failure_details.strip(),
                'is_timeout': is_timeout
            })
    
    return statistics, failed_tests


def format_failed_tests_list(failed_tests: List[Dict[str, str]]) -> str:
    if not failed_tests:
        return ""
    
    lines = []
    for test in failed_tests:
        suffix = " (タイムアウト)" if test['is_timeout'] else ""
        lines.append(f"- {test['name']}{suffix}")
    
    return "\n".join(lines)


def format_failed_tests_with_errors(failed_tests: List[Dict[str, str]]) -> str:
    if not failed_tests:
        return ""
    
    lines = []
    for test in failed_tests:
        suffix = " (タイムアウト)" if test['is_timeout'] else ""
        lines.append(f"### {test['name']}{suffix}")
        lines.append("")
        if test['message']:
            lines.append(f"**Error**: {test['message']}")
            lines.append("")
        if test['details']:
            lines.append("```")
            lines.append(test['details'])
            lines.append("```")
            lines.append("")
    
    return "\n".join(lines)


def write_github_output(output_file: str, statistics: Dict[str, str], failed_tests: List[Dict[str, str]]) -> None:
    import tempfile
    import os
    
    # Write large data to temporary files to avoid command-line size limitations
    failed_tests_list_content = format_failed_tests_list(failed_tests)
    error_details_content = format_failed_tests_with_errors(failed_tests)
    
    # Create temporary files for large data with secure permissions
    failed_tests_list_fd, failed_tests_list_path = tempfile.mkstemp(suffix='.txt', prefix='failed_tests_list_', text=True)
    error_details_fd, error_details_path = tempfile.mkstemp(suffix='.txt', prefix='error_details_', text=True)
    
    try:
        # Write failed tests list to file
        with os.fdopen(failed_tests_list_fd, 'w', encoding='utf-8') as f:
            f.write(failed_tests_list_content)
        
        # Write error details to file
        with os.fdopen(error_details_fd, 'w', encoding='utf-8') as f:
            f.write(error_details_content)
        
        # Write paths to GITHUB_OUTPUT
        with open(output_file, 'a', encoding='utf-8') as f:
            f.write(f"total_tests={statistics['total_tests']}\n")
            f.write(f"passed={statistics['passed']}\n")
            f.write(f"failed={statistics['failed']}\n")
            f.write(f"timed_out={statistics['timed_out']}\n")
            f.write(f"failed_tests_list_file={failed_tests_list_path}\n")
            f.write(f"error_details_file={error_details_path}\n")
    except Exception:
        # Attempt to clean up temp files before re-raising exception
        try:
            os.unlink(failed_tests_list_path)
        except OSError as e:
            print(f"Warning: failed to delete temp file {failed_tests_list_path}: {e}", file=sys.stderr)
        try:
            os.unlink(error_details_path)
        except OSError as e:
            print(f"Warning: failed to delete temp file {error_details_path}: {e}", file=sys.stderr)
        raise


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--junit-file", required=True)
    parser.add_argument("--github-output", help="Path to GITHUB_OUTPUT file for direct writing")
    args = parser.parse_args()
    
    statistics, failed_tests = parse_junit_xml(args.junit_file)
    
    if args.github_output:
        write_github_output(args.github_output, statistics, failed_tests)
    else:
        print(f"total_tests={statistics['total_tests']}")
        print(f"passed={statistics['passed']}")
        print(f"failed={statistics['failed']}")
        print(f"timed_out={statistics['timed_out']}")
        print("---FAILED_TESTS---")
        print(format_failed_tests_list(failed_tests))
        print("---ERROR_DETAILS---")
        print(format_failed_tests_with_errors(failed_tests))
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
