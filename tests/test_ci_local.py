"""Unit tests for the local CI runner's command result handling."""
from __future__ import annotations

import argparse
import subprocess
import unittest
from pathlib import Path
from contextlib import redirect_stderr, redirect_stdout
import io
import tempfile
import json
from unittest.mock import patch

from tools.ci_local import CHECKS, collect_git_metadata, make_report, run_step
from tools.ci_local_evidence import EvidenceValidationError, create_manifest, verify_manifest


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




    def test_python_bytecode_artifacts_are_ignored(self):
        root = Path(__file__).resolve().parents[1]
        for candidate in ("__pycache__/test_ci_local.cpython-312.pyc", "tools/ci_local.pyc"):
            result = subprocess.run(
                ["git", "check-ignore", "--no-index", candidate],
                cwd=root, check=False, capture_output=True, text=True,
            )
            self.assertEqual(result.returncode, 0, f"{candidate} must be ignored")

    def test_workspace_lockfile_is_tracked_and_not_ignored(self):
        root = Path(__file__).resolve().parents[1]
        lockfile = root / "Cargo.lock"
        self.assertTrue(lockfile.is_file(), "Cargo.lock must exist in a clean checkout")
        tracked = subprocess.run(
            ["git", "ls-files", "--error-unmatch", "Cargo.lock"],
            cwd=root, check=False, capture_output=True, text=True,
        )
        self.assertEqual(tracked.returncode, 0, tracked.stderr)
        ignored = subprocess.run(
            ["git", "check-ignore", "--no-index", "Cargo.lock"],
            cwd=root, check=False, capture_output=True, text=True,
        )
        self.assertNotEqual(ignored.returncode, 0, "Cargo.lock must not be ignored")

    def test_clippy_places_locked_after_external_subcommand(self):
        clippy = next(command for name, command in CHECKS if name == "Clippy")
        self.assertEqual(clippy[:3], ("cargo", "clippy", "--locked"))


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
        self.assertEqual(len(metadata["dirty_entries"]), 2)

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

    def test_report_preserves_before_and_after_worktree_snapshots(self):
        before = {"status": "VERIFIED", "commit_sha": "a" * 40, "working_tree_clean": True}
        after = {"status": "VERIFIED", "commit_sha": "a" * 40, "working_tree_clean": False}
        report = make_report(
            "2026-11-11T00:00:00+00:00",
            [],
            argparse.Namespace(offline=False, timeout=30),
            0,
            before,
            after,
        )
        self.assertEqual(report["schema_version"], 2)
        self.assertEqual(report["source_revision"], before)
        self.assertEqual(report["worktree_after"], after)

    def test_run_step_redacts_secret_from_command_stdout_and_stderr(self):
        secret = "unit-test-secret-value"
        env = {"TEST_TOKEN": secret}
        completed = subprocess.CompletedProcess(
            ["echo", secret], 0,
            stdout=f"stdout={secret}\\n",
            stderr=f"stderr={secret}\\n",
        )
        stdout = io.StringIO()
        stderr = io.StringIO()
        with patch("tools.ci_local.subprocess.run", return_value=completed) as run:
            with redirect_stdout(stdout), redirect_stderr(stderr):
                result = run_step("redaction", ["echo", secret], env, 10)
        self.assertNotIn(secret, stdout.getvalue())
        self.assertNotIn(secret, stderr.getvalue())
        self.assertIn("[REDACTED]", stdout.getvalue())
        self.assertIn("[REDACTED]", stderr.getvalue())
        self.assertEqual(result["command"], ["echo", "[REDACTED]"])
        self.assertTrue(run.call_args.kwargs["capture_output"])

    def test_write_report_is_atomic_and_leaves_no_temporary_file(self):
        with tempfile.TemporaryDirectory() as directory:
            destination = Path(directory) / "report.json"
            payload = {"schema_version": 2, "status": "PASS"}
            with patch("tools.ci_local.ROOT", Path(directory)):
                from tools.ci_local import write_report
                write_report(Path("report.json"), payload)
            self.assertEqual(json.loads(destination.read_text(encoding="utf-8")), payload)
            self.assertEqual(list(Path(directory).glob(".report.json.*.tmp")), [])


class EvidenceManifestTests(unittest.TestCase):
    def valid_report(self, source_sha="a" * 40):
        return {
            "schema_version": 2,
            "source_revision": {
                "status": "VERIFIED", "commit_sha": source_sha,
                "working_tree_clean": True, "dirty_path_count": 0,
            },
            "worktree_after": {
                "status": "VERIFIED", "commit_sha": source_sha,
                "working_tree_clean": True, "dirty_path_count": 0,
            },
            "status": "PASS",
            "exit_code": 0,
            "summary": {"total": 12, "passed": 12, "failed_or_unavailable": 0},
            "steps": [{"name": f"step-{index}", "status": "PASS"} for index in range(12)],
        }

    def test_manifest_create_and_verify_roundtrip(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            report = root / "ci-local-report.json"
            manifest = root / "ci-local-manifest.json"
            report.write_text(json.dumps(self.valid_report()), encoding="utf-8")
            created = create_manifest(
                report, manifest, "a" * 40, verifier_sha="b" * 40,
                workflow="test", run_id="123", os_name="Linux", arch="X64",
            )
            self.assertEqual(created["validation_status"], "PASS")
            verified = verify_manifest(report, manifest, "a" * 40, expected_verifier_sha="b" * 40)
            self.assertEqual(verified["report_sha256"], created["report_sha256"])

    def test_missing_report_is_fail_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            manifest = root / "ci-local-manifest.json"
            created = create_manifest(root / "missing.json", manifest, "a" * 40)
            self.assertEqual(created["validation_status"], "MISSING_REPORT")
            with self.assertRaises(EvidenceValidationError):
                verify_manifest(root / "missing.json", manifest, "a" * 40)

    def test_corrupt_report_json_is_fail_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            report = root / "ci-local-report.json"
            manifest = root / "ci-local-manifest.json"
            report.write_text("{not-json", encoding="utf-8")
            created = create_manifest(report, manifest, "a" * 40)
            self.assertEqual(created["validation_status"], "INVALID_JSON")
            with self.assertRaises(EvidenceValidationError):
                verify_manifest(report, manifest, "a" * 40)

    def test_source_sha_mismatch_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            report = root / "ci-local-report.json"
            manifest = root / "ci-local-manifest.json"
            report.write_text(json.dumps(self.valid_report("a" * 40)), encoding="utf-8")
            created = create_manifest(report, manifest, "c" * 40)
            self.assertEqual(created["validation_status"], "FAIL")
            with self.assertRaises(EvidenceValidationError):
                verify_manifest(report, manifest, "c" * 40)

    def test_report_tampering_is_detected_by_manifest_digest(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            report = root / "ci-local-report.json"
            manifest = root / "ci-local-manifest.json"
            report.write_text(json.dumps(self.valid_report()), encoding="utf-8")
            create_manifest(report, manifest, "a" * 40)
            report.write_text(report.read_text(encoding="utf-8") + " ", encoding="utf-8")
            with self.assertRaisesRegex(EvidenceValidationError, "SHA-256"):
                verify_manifest(report, manifest, "a" * 40)

    def test_missing_manifest_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            report = root / "ci-local-report.json"
            report.write_text(json.dumps(self.valid_report()), encoding="utf-8")
            with self.assertRaisesRegex(EvidenceValidationError, "manifest is missing"):
                verify_manifest(report, root / "missing-manifest.json", "a" * 40)

    def test_report_summary_inconsistency_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            report = root / "ci-local-report.json"
            manifest = root / "ci-local-manifest.json"
            payload = self.valid_report()
            payload["summary"]["passed"] = 11
            report.write_text(json.dumps(payload), encoding="utf-8")
            created = create_manifest(report, manifest, "a" * 40)
            self.assertEqual(created["validation_status"], "FAIL")
            with self.assertRaises(EvidenceValidationError):
                verify_manifest(report, manifest, "a" * 40)


class CircleCIIntegrationTests(unittest.TestCase):
    def test_circleci_runs_local_runner_and_enforces_exact_sha_report(self):
        root = Path(__file__).resolve().parents[1]
        config = (root / ".circleci" / "config.yml").read_text(encoding="utf-8")
        self.assertIn("python3 tools/ci_local.py --report ci-local-report.json", config)
        self.assertIn("python3 tools/ci_local.py", config)
        self.assertIn("python3 tools/ci_local_evidence.py create", config)
        self.assertIn("python3 tools/ci_local_evidence.py verify", config)
        self.assertIn('--expected-source-sha "$CIRCLE_SHA1"', config)
        self.assertIn("ci-local-manifest.json", config)
        self.assertIn("ci-local-report.json", config)

if __name__ == "__main__":
    unittest.main()
