"""Unit tests for the local CI runner's command result handling."""
from __future__ import annotations

import argparse
import subprocess
import unittest
from pathlib import Path
from unittest.mock import patch

from tools.ci_local import CHECKS, collect_git_metadata, make_report, run_step


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




class CargoLockTests(unittest.TestCase):
    def test_every_cargo_check_uses_locked_mode(self):
        cargo_checks = [(name, command) for name, command in CHECKS if command[0] == "cargo"]
        self.assertGreaterEqual(len(cargo_checks), 8)
        for name, command in cargo_checks:
            self.assertIn("--locked", command, f"{name} must not mutate Cargo.lock")


class GitMetadataTests(unittest.TestCase):
    @patch("tools.ci_local.subprocess.run")
    def test_captures_exact_commit_branch_and_clean_worktree(self, run):
        run.side_effect = [
            subprocess.CompletedProcess(["git"], 0, stdout="a" * 40 + "\n", stderr=""),
            subprocess.CompletedProcess(["git"], 0, stdout="tooling/python-local-ci\n", stderr=""),
            subprocess.CompletedProcess(["git"], 0, stdout="", stderr=""),
        ]
        metadata = collect_git_metadata()
        self.assertEqual(metadata["status"], "VERIFIED")
        self.assertEqual(metadata["commit_sha"], "a" * 40)
        self.assertEqual(metadata["branch"], "tooling/python-local-ci")
        self.assertTrue(metadata["working_tree_clean"])
        self.assertEqual(metadata["dirty_path_count"], 0)
        self.assertIsNone(metadata["error"])

    @patch("tools.ci_local.subprocess.run")
    def test_records_dirty_worktree_without_hiding_commit_sha(self, run):
        run.side_effect = [
            subprocess.CompletedProcess(["git"], 0, stdout="b" * 40 + "\n", stderr=""),
            subprocess.CompletedProcess(["git"], 0, stdout="", stderr=""),
            subprocess.CompletedProcess(["git"], 0, stdout=" M tools/ci_local.py\n?? artifacts/report.json\n", stderr=""),
        ]
        metadata = collect_git_metadata()
        self.assertEqual(metadata["status"], "VERIFIED")
        self.assertEqual(metadata["commit_sha"], "b" * 40)
        self.assertFalse(metadata["working_tree_clean"])
        self.assertEqual(metadata["dirty_path_count"], 2)

    @patch("tools.ci_local.subprocess.run")
    def test_non_git_directory_is_explicitly_unavailable(self, run):
        run.return_value = subprocess.CompletedProcess(
            ["git"], 128, stdout="", stderr="fatal: not a git repository"
        )
        metadata = collect_git_metadata()
        self.assertEqual(metadata["status"], "UNAVAILABLE")
        self.assertIsNone(metadata["commit_sha"])
        self.assertIn("not a git repository", metadata["error"])

    @patch("tools.ci_local.collect_git_metadata")
    def test_json_report_contains_source_revision(self, collect):
        expected = {
            "status": "VERIFIED",
            "commit_sha": "c" * 40,
            "branch": "main",
            "working_tree_clean": True,
            "dirty_path_count": 0,
            "error": None,
        }
        collect.return_value = expected
        report = make_report(
            "2026-11-11T00:00:00+00:00",
            [],
            argparse.Namespace(offline=False, timeout=30),
            0,
        )
        self.assertEqual(report["source_revision"], expected)


class CircleCIIntegrationTests(unittest.TestCase):
    def test_circleci_runs_local_runner_and_enforces_exact_sha_report(self):
        root = Path(__file__).resolve().parents[1]
        config = (root / ".circleci" / "config.yml").read_text(encoding="utf-8")
        self.assertIn("python3 tools/ci_local.py --report ci-local-report.json", config)
        self.assertIn('s["commit_sha"]==os.environ["CIRCLE_SHA1"]', config)
        self.assertIn('s["working_tree_clean"] is True', config)
        self.assertIn("ci-local-report.json", config)

if __name__ == "__main__":
    unittest.main()
