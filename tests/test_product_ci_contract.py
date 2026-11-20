from __future__ import annotations

import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WORKFLOW = ROOT / ".github" / "workflows" / "ci.yml"
GITIGNORE = ROOT / ".gitignore"


class ProductCiContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.config = WORKFLOW.read_text(encoding="utf-8")
        cls.gitignore = GITIGNORE.read_text(encoding="utf-8").splitlines()

    def test_every_cargo_check_uses_locked_mode(self) -> None:
        commands = re.findall(r"(?m)^\s*run:\s*cargo\s+([^\n]+)$", self.config)
        self.assertGreaterEqual(len(commands), 8, "expected direct product Cargo commands")
        for command in commands:
            with self.subTest(command=command):
                self.assertIn("--locked", command.split(), "Cargo checks must not mutate Cargo.lock")

    def test_python_bytecode_artifacts_are_ignored(self) -> None:
        self.assertIn("__pycache__/", self.gitignore)
        self.assertIn("*.py[cod]", self.gitignore)

    def test_workspace_lockfile_is_tracked_and_not_ignored(self) -> None:
        lockfile = ROOT / "Cargo.lock"
        self.assertTrue(lockfile.is_file(), "Cargo.lock must exist in a clean checkout")
        tracked = subprocess.run(
            ["git", "ls-files", "--error-unmatch", "Cargo.lock"],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(tracked.returncode, 0, tracked.stderr)
        ignored = subprocess.run(
            ["git", "check-ignore", "--no-index", "Cargo.lock"],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertNotEqual(ignored.returncode, 0, "Cargo.lock must not be ignored")

    def test_clippy_places_locked_after_external_subcommand(self) -> None:
        clippy = re.search(r"(?m)^\s*run:\s*cargo\s+clippy\s+([^\n]+)$", self.config)
        self.assertIsNotNone(clippy, "expected direct Cargo Clippy command")
        self.assertTrue(clippy.group(1).startswith("--locked "), "Clippy must receive --locked after the subcommand")


if __name__ == "__main__":
    unittest.main()
