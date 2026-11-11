"""Tests for fail-closed unittest discovery."""
from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from tools.ci_unittest import load_suite


class DiscoveryGuardTests(unittest.TestCase):
    def test_empty_pattern_collects_zero_and_is_detectable(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            suite = load_suite(directory, "test_*.py")
            self.assertEqual(suite.countTestCases(), 0)

    def test_matching_test_file_collects_test_cases(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "test_sample.py").write_text(
                "import unittest\n"
                "class Sample(unittest.TestCase):\n"
                "    def test_ok(self):\n"
                "        self.assertTrue(True)\n",
                encoding="utf-8",
            )
            suite = load_suite(root, "test_*.py")
            self.assertEqual(suite.countTestCases(), 1)


if __name__ == "__main__":
    unittest.main()
