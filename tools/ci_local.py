#!/usr/bin/env python3
"""Run One's portable repository CI checks on the current host.

This is a local verification runner, not a hosted CI service. Cross-platform
claims still require execution on the corresponding OS/target.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
import platform
import subprocess
import sys
import time
from pathlib import Path
from typing import Any, Sequence

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


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def run_step(
    name: str,
    command: Sequence[str],
    env: dict[str, str],
    timeout_seconds: float,
) -> dict[str, Any]:
    print(f"\n{'=' * 72}\nSTEP: {name}\nCOMMAND: {' '.join(command)}\n{'=' * 72}", flush=True)
    started = time.monotonic()
    record: dict[str, Any] = {
        "name": name,
        "command": list(command),
        "status": "FAIL",
        "elapsed_seconds": 0.0,
        "exit_code": None,
        "error": None,
    }
    try:
        result = subprocess.run(
            command,
            cwd=ROOT,
            env=env,
            check=False,
            timeout=timeout_seconds,
        )
        record["exit_code"] = result.returncode
        record["status"] = "PASS" if result.returncode == 0 else f"FAIL (exit {result.returncode})"
    except subprocess.TimeoutExpired:
        record["status"] = f"TIMEOUT (>{timeout_seconds:g}s)"
        record["error"] = f"Command exceeded the {timeout_seconds:g}-second timeout"
    except FileNotFoundError as exc:
        record["status"] = f"UNAVAILABLE ({exc.filename} not found)"
        record["error"] = str(exc)
    except OSError as exc:
        record["status"] = "FAIL (could not start command)"
        record["error"] = str(exc)

    record["elapsed_seconds"] = round(time.monotonic() - started, 3)
    print(f"RESULT: {record['status']} [{record['elapsed_seconds']:.1f}s]", flush=True)
    return record


def write_report(path: Path, report: dict[str, Any]) -> None:
    destination = path if path.is_absolute() else ROOT / path
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"JSON report: {destination}", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run the portable One CI checks on the current machine."
    )
    parser.add_argument(
        "--offline",
        action="store_true",
        help="set Cargo offline mode (fails if required dependencies are not cached)",
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=1800,
        metavar="SECONDS",
        help="maximum duration for each command (default: 1800; use 0 to disable)",
    )
    parser.add_argument(
        "--report",
        type=Path,
        default=Path("ci-local-report.json"),
        metavar="PATH",
        help="write a JSON report (default: ci-local-report.json, relative to repository root)",
    )
    args = parser.parse_args()
    if args.timeout < 0:
        parser.error("--timeout must be zero (disable) or a positive number")
    timeout_seconds = args.timeout or None

    env = os.environ.copy()
    if args.offline:
        env["CARGO_NET_OFFLINE"] = "true"

    started_at = utc_now()
    print("ONE LOCAL CI (Python runner)")
    print(f"Repository: {ROOT}")
    print(f"Host: {platform.platform()}")
    print(f"Python: {sys.version.split()[0]}")
    print("Scope: current host only; this run does not prove Windows/Android execution.")
    print(f"Per-command timeout: {'disabled' if timeout_seconds is None else f'{timeout_seconds:g}s'}")

    steps: list[dict[str, Any]] = []
    for name, command in (
        ("Rust compiler", ("rustc", "--version")),
        ("Cargo", ("cargo", "--version")),
    ):
        step = run_step(name, command, env, timeout_seconds) if timeout_seconds else run_step(name, command, env, 10**9)
        steps.append(step)
        if step["status"] != "PASS":
            exit_code = 2
            report = make_report(started_at, steps, args, exit_code)
            try:
                write_report(args.report, report)
            except OSError as exc:
                print(f"Could not write JSON report: {exc}", file=sys.stderr)
                return 1
            print("\nFINAL STATUS: FAIL — required Rust toolchain is unavailable.")
            return exit_code

    for name, command in CHECKS:
        steps.append(run_step(name, command, env, timeout_seconds or 10**9))

    passed = sum(step["status"] == "PASS" for step in steps)
    failed = len(steps) - passed
    exit_code = 0 if failed == 0 else 1
    report = make_report(started_at, steps, args, exit_code)

    print("\n" + "=" * 72)
    print("ONE LOCAL CI SUMMARY")
    print("=" * 72)
    for step in steps:
        print(f"{step['status']:28} {step['elapsed_seconds']:7.1f}s  {step['name']}")
    print(f"\nPassed: {passed}/{len(steps)}")
    print(f"Failed, timed out, or unavailable: {failed}/{len(steps)}")
    try:
        write_report(args.report, report)
    except OSError as exc:
        print(f"Could not write JSON report: {exc}", file=sys.stderr)
        return 1

    if exit_code:
        print("FINAL STATUS: FAIL — inspect the step results and JSON report.")
        return exit_code
    print("FINAL STATUS: PASS — all configured checks passed on this host.")
    print("This result does not substitute for separate Windows or Android target evidence.")
    return 0


def make_report(
    started_at: str,
    steps: list[dict[str, Any]],
    args: argparse.Namespace,
    exit_code: int,
) -> dict[str, Any]:
    passed = sum(step["status"] == "PASS" for step in steps)
    return {
        "schema_version": 1,
        "repository": str(ROOT),
        "host": platform.platform(),
        "python": sys.version.split()[0],
        "started_at": started_at,
        "finished_at": utc_now(),
        "offline": args.offline,
        "timeout_seconds": args.timeout,
        "status": "PASS" if exit_code == 0 else "FAIL",
        "exit_code": exit_code,
        "summary": {
            "total": len(steps),
            "passed": passed,
            "failed_or_unavailable": len(steps) - passed,
        },
        "steps": steps,
        "evidence_boundary": (
            "This report describes checks run on this host only. It does not prove "
            "execution on other operating systems or target architectures."
        ),
    }


if __name__ == "__main__":
    raise SystemExit(main())
