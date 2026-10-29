"""Unit tests for the local CI runner's command result handling."""
from __future__ import annotations

import subprocess
import unittest
from unittest.mock import patch

from tools.ci_local import run_step


class RunStepTests(unittest.TestCase):
    def test_successful_command_is_pass(self) -> None:
        with patch("tools.ci_local.subprocess.run", return_value=subprocess.CompletedProcess(["true"], 0)):
            result = run_step("sample", ["true"], {}, 10)
        self.assertEqual(result["status"], "PASS")
        self.assertEqual(result["exit_code"], 0)
        self.assertIsNone(result["error"])
        self.assertGreaterEqual(result["elapsed_seconds"], 0)

    def test_nonzero_command_is_failure(self) -> None:
        with patch("tools.ci_local.subprocess.run", return_value=subprocess.CompletedProcess(["false"], 7)):
            result = run_step("sample", ["false"], {}, 10)
        self.assertEqual(result["status"], "FAIL (exit 7)")
        self.assertEqual(result["exit_code"], 7)

    def test_timeout_is_not_reported_as_pass(self) -> None:
        with patch(
            "tools.ci_local.subprocess.run",
            side_effect=subprocess.TimeoutExpired(cmd=["slow"], timeout=1),
        ):
            result = run_step("sample", ["slow"], {}, 1)
        self.assertEqual(result["status"], "TIMEOUT (>1s)")
        self.assertIsNone(result["exit_code"])
        self.assertIn("timeout", result["error"].lower())

    def test_missing_command_is_unavailable(self) -> None:
        with patch(
            "tools.ci_local.subprocess.run",
            side_effect=FileNotFoundError(2, "not found", "missing-tool"),
        ):
            result = run_step("sample", ["missing-tool"], {}, 10)
        self.assertEqual(result["status"], "UNAVAILABLE (missing-tool not found)")
        self.assertIsNotNone(result["error"])


if __name__ == "__main__":
    unittest.main()
