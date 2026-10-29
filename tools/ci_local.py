#!/usr/bin/env python3
"""Run One's repository CI checks locally with a Python orchestration layer.

This is a local verification runner, not a hosted CI service. It runs the
repository's portable contract/build/lint/test commands on the current host.
Cross-platform claims still require execution on the corresponding OS/target.
"""
from __future__ import annotations

import argparse
import os
import platform
import subprocess
import sys
import time
from pathlib import Path
from typing import Sequence

ROOT = Path(__file__).resolve().parents[1]

CHECKS: list[tuple[str, Sequence[str]]] = [
    ("Contract artifacts", (sys.executable, "tools/check_contract.py")),
    ("Build workspace", ("cargo", "build", "--workspace", "--all-targets")),
    ("Clippy", ("cargo", "clippy", "--workspace", "--all-targets", "--", "-D", "warnings")),
    ("Workspace tests", ("cargo", "test", "--workspace", "--all-targets")),
    (
        "Deterministic .oneir/.onepkg artifacts",
        (
            "cargo", "test", "-p", "one-cli", "--test", "cli",
            "build_and_package_artifacts_are_byte_deterministic",
        ),
    ),
    ("DNS success/failure classification", ("cargo", "test", "-p", "one-core", "--all-targets", "dns_adapter")),
    ("Orchestration policy paths", ("cargo", "test", "-p", "one-core", "--all-targets", "orchestrator")),
    ("Database vertical slice", ("cargo", "test", "-p", "one-core", "--test", "database_vertical_slice")),
    ("HTTP GET vertical slice", ("cargo", "test", "-p", "one-core", "--test", "http_get_vertical_slice")),
    ("HTTP GET CLI vertical slice", ("cargo", "test", "-p", "one-cli", "--test", "http_get_orchestration")),
]


def run_step(name: str, command: Sequence[str], env: dict[str, str]) -> tuple[str, float]:
    print(f"\n{'=' * 72}\nSTEP: {name}\nCOMMAND: {' '.join(command)}\n{'=' * 72}", flush=True)
    started = time.monotonic()
    try:
        result = subprocess.run(command, cwd=ROOT, env=env, check=False)
        status = "PASS" if result.returncode == 0 else f"FAIL (exit {result.returncode})"
    except FileNotFoundError as exc:
        status = f"UNAVAILABLE ({exc.filename} not found)"
    except OSError as exc:
        status = f"FAIL (could not start command: {exc})"
    elapsed = time.monotonic() - started
    print(f"RESULT: {status} [{elapsed:.1f}s]", flush=True)
    return status, elapsed


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run the portable One CI checks on the current machine."
    )
    parser.add_argument(
        "--offline",
        action="store_true",
        help="set Cargo offline mode (fails if required dependencies are not cached)",
    )
    args = parser.parse_args()

    env = os.environ.copy()
    if args.offline:
        env["CARGO_NET_OFFLINE"] = "true"

    print("ONE LOCAL CI (Python runner)")
    print(f"Repository: {ROOT}")
    print(f"Host: {platform.platform()}")
    print(f"Python: {sys.version.split()[0]}")
    print("Scope: current host only; this run does not prove Windows/Android execution.")

    for name, command in (
        ("Rust compiler", ("rustc", "--version")),
        ("Cargo", ("cargo", "--version")),
    ):
        status, _ = run_step(name, command, env)
        if status != "PASS":
            print("\nFINAL STATUS: FAIL — required Rust toolchain is unavailable.")
            return 2

    results: list[tuple[str, str, float]] = []
    for name, command in CHECKS:
        status, elapsed = run_step(name, command, env)
        results.append((name, status, elapsed))

    print("\n" + "=" * 72)
    print("ONE LOCAL CI SUMMARY")
    print("=" * 72)
    for name, status, elapsed in results:
        print(f"{status:24} {elapsed:7.1f}s  {name}")

    passed = sum(status == "PASS" for _, status, _ in results)
    failed = len(results) - passed
    print(f"\nPassed: {passed}/{len(results)}")
    print(f"Failed or unavailable: {failed}/{len(results)}")
    if failed:
        print("FINAL STATUS: FAIL — inspect the first failing step above.")
        return 1
    print("FINAL STATUS: PASS — all configured checks passed on this host.")
    print("This result does not substitute for separate Windows or Android target evidence.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
