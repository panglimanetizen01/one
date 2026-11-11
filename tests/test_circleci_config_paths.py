"""Guard CircleCI's direct Python paths and test-discovery configuration."""
from __future__ import annotations

import re
import unittest
from pathlib import Path

from tools.ci_unittest import load_suite

ROOT = Path(__file__).resolve().parents[1]
CONFIG = ROOT / ".circleci" / "config.yml"
PYTHON_SCRIPT_RE = re.compile(
    r"(?m)^\s*python3\s+(tools/[A-Za-z0-9_./-]+)(?=\s|$)"
)
DISCOVERY_RE = re.compile(
    r"(?m)^\s*python3\s+tools/ci_unittest.py\s+"
    r"--start-directory\s+([^\s]+)\s+--pattern\s+([^\s]+)"
)


class CircleCIConfigPathTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.config = CONFIG.read_text(encoding="utf-8")

    def test_every_direct_tools_script_invocation_exists(self) -> None:
        paths = PYTHON_SCRIPT_RE.findall(self.config)
        self.assertTrue(paths, "expected direct Python tools/ invocations")
        for relative in paths:
            with self.subTest(path=relative):
                self.assertTrue((ROOT / relative).is_file(), f"missing CircleCI script: {relative}")

    def test_every_discovery_pattern_matches_a_nonempty_suite(self) -> None:
        discoveries = DISCOVERY_RE.findall(self.config)
        self.assertTrue(discoveries, "expected guarded unittest discovery commands")
        for start, pattern in discoveries:
            with self.subTest(start=start, pattern=pattern):
                suite = load_suite(start, pattern)
                self.assertGreater(
                    suite.countTestCases(), 0,
                    f"zero tests discovered for {start}/{pattern}",
                )


if __name__ == "__main__":
    unittest.main()
