#!/usr/bin/env python3
"""Run unittest discovery and fail closed when no tests are collected."""
from __future__ import annotations

import argparse
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))


def load_suite(start_directory: str | Path, pattern: str) -> unittest.TestSuite:
    start = Path(start_directory)
    if not start.is_absolute():
        start = ROOT / start
    start = start.resolve()
    if not start.is_dir():
        raise ValueError(f"test discovery directory does not exist: {start}")
    return unittest.defaultTestLoader.discover(str(start), pattern=pattern)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--start-directory", required=True)
    parser.add_argument("--pattern", required=True)
    args = parser.parse_args()
    try:
        suite = load_suite(args.start_directory, args.pattern)
    except (ImportError, OSError, ValueError) as exc:
        print(f"ERROR: unittest discovery failed: {exc}", file=sys.stderr)
        return 2
    count = suite.countTestCases()
    if count == 0:
        print(
            f"ERROR: unittest discovery collected zero tests "
            f"(start={args.start_directory!r}, pattern={args.pattern!r})",
            file=sys.stderr,
        )
        return 2
    print(f"Discovered {count} test case(s) for {args.pattern}", flush=True)
    result = unittest.TextTestRunner(verbosity=2).run(suite)
    return 0 if result.wasSuccessful() else 1


if __name__ == "__main__":
    raise SystemExit(main())
