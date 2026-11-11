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
import re
import subprocess
import tempfile
import sys
import time
from pathlib import Path
from typing import Any, Sequence

ROOT = Path(__file__).resolve().parents[1]

CHECKS: list[tuple[str, Sequence[str]]] = [
    ("Contract artifacts", (sys.executable, "tools/check_contract.py")),
    ("Build workspace", ("cargo", "--locked", "build", "--workspace", "--all-targets")),
    ("Clippy", ("cargo", "clippy", "--locked", "--workspace", "--all-targets", "--", "-D", "warnings")),
    ("Workspace tests", ("cargo", "--locked", "test", "--workspace", "--all-targets")),
    (
        "Deterministic .oneir/.onepkg artifacts",
        (
            "cargo", "--locked", "test", "-p", "one-cli", "--test", "cli",
            "build_and_package_artifacts_are_byte_deterministic",
        ),
    ),
    ("DNS success/failure classification", ("cargo", "--locked", "test", "-p", "one-core", "--all-targets", "dns_adapter")),
    ("Orchestration policy paths", ("cargo", "--locked", "test", "-p", "one-core", "--all-targets", "orchestrator")),
    ("Database vertical slice", ("cargo", "--locked", "test", "-p", "one-core", "--test", "database_vertical_slice")),
    ("HTTP GET vertical slice", ("cargo", "--locked", "test", "-p", "one-core", "--test", "http_get_vertical_slice")),
    ("HTTP GET CLI vertical slice", ("cargo", "--locked", "test", "-p", "one-cli", "--test", "http_get_orchestration")),
]


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")



def collect_git_metadata() -> dict[str, Any]:
    metadata: dict[str, Any] = {
        "status": "UNAVAILABLE",
        "commit_sha": None,
        "branch": None,
        "working_tree_clean": None,
        "dirty_path_count": None,
        "dirty_entries": [],
        "dirty_entries_truncated": False,
        "error": None,
    }

    def run_git(*args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["git", *args],
            cwd=ROOT,
            check=False,
            capture_output=True,
            text=True,
            timeout=10,
        )

    try:
        commit = run_git("rev-parse", "HEAD")
        if commit.returncode != 0:
            metadata["error"] = commit.stderr.strip() or f"git rev-parse HEAD exited {commit.returncode}"
            return metadata

        commit_sha = commit.stdout.strip()
        if not re.fullmatch(r"[0-9a-f]{40}", commit_sha):
            metadata["error"] = f"git rev-parse HEAD returned an invalid commit SHA: {commit_sha!r}"
            return metadata

        branch = run_git("branch", "--show-current")
        status = run_git("status", "--porcelain", "--untracked-files=all")
        if branch.returncode != 0 or status.returncode != 0:
            errors = [
                result.stderr.strip() or f"git metadata command exited {result.returncode}"
                for result in (branch, status)
                if result.returncode != 0
            ]
            metadata.update(
                status="PARTIAL",
                commit_sha=commit_sha,
                branch=branch.stdout.strip() or None if branch.returncode == 0 else None,
                error="; ".join(errors),
            )
            return metadata

        dirty_paths = [line for line in status.stdout.splitlines() if line]
        metadata.update(
            status="VERIFIED",
            commit_sha=commit_sha,
            branch=branch.stdout.strip() or None,
            working_tree_clean=not dirty_paths,
            dirty_path_count=len(dirty_paths),
            dirty_entries=dirty_paths[:100],
            dirty_entries_truncated=len(dirty_paths) > 100,
        )
        return metadata
    except (OSError, subprocess.TimeoutExpired) as exc:
        metadata["error"] = str(exc)
        return metadata


SECRET_ENV_PATTERN = re.compile(
    r"(?:TOKEN|SECRET|PASSWORD|PASSWD|CREDENTIAL|PRIVATE_KEY|API_KEY|AUTHORIZATION)",
    re.IGNORECASE,
)


def redact_text(value: Any, env: dict[str, str]) -> str:
    if value is None:
        return ""
    if isinstance(value, bytes):
        text_value = value.decode("utf-8", errors="replace")
    else:
        text_value = str(value)
    secrets = sorted(
        {
            secret
            for key, secret in env.items()
            if SECRET_ENV_PATTERN.search(key) and isinstance(secret, str) and secret
        },
        key=len,
        reverse=True,
    )
    for secret in secrets:
        text_value = text_value.replace(secret, "[REDACTED]")
    return text_value


def emit_output(value: Any, env: dict[str, str], *, stream: Any | None = None) -> None:
    output = redact_text(value, env)
    if output:
        destination = stream if stream is not None else sys.stdout
        print(output, end="" if output.endswith("\n") else "\n", file=destination, flush=True)


def run_step(
    name: str,
    command: Sequence[str],
    env: dict[str, str],
    timeout_seconds: float | None,
) -> dict[str, Any]:
    safe_command = [redact_text(part, env) for part in command]
    print(
        f"\n{'=' * 72}\nSTEP: {name}\nCOMMAND: {' '.join(safe_command)}\n{'=' * 72}",
        flush=True,
    )
    started = time.monotonic()
    record: dict[str, Any] = {
        "name": name,
        "command": safe_command,
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
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
        )
        emit_output(result.stdout, env)
        emit_output(result.stderr, env, stream=sys.stderr)
        record["exit_code"] = result.returncode
        record["status"] = "PASS" if result.returncode == 0 else f"FAIL (exit {result.returncode})"
    except subprocess.TimeoutExpired as exc:
        emit_output(exc.stdout, env)
        emit_output(exc.stderr, env, stream=sys.stderr)
        record["status"] = f"TIMEOUT (>{timeout_seconds:g}s)"
        record["error"] = "Command exceeded the configured timeout"
    except FileNotFoundError as exc:
        record["status"] = f"UNAVAILABLE ({exc.filename} not found)"
        record["error"] = redact_text(str(exc), env)
    except OSError as exc:
        record["status"] = "FAIL (could not start command)"
        record["error"] = redact_text(str(exc), env)

    record["elapsed_seconds"] = round(time.monotonic() - started, 3)
    print(f"RESULT: {record['status']} [{record['elapsed_seconds']:.1f}s]", flush=True)
    return record


def write_report(path: Path, report: dict[str, Any]) -> None:
    destination = path if path.is_absolute() else ROOT / path
    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary_path: Path | None = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="w",
            encoding="utf-8",
            dir=destination.parent,
            prefix=f".{destination.name}.",
            suffix=".tmp",
            delete=False,
        ) as temporary:
            temporary_path = Path(temporary.name)
            temporary.write(json.dumps(report, indent=2) + "\n")
            temporary.flush()
            os.fsync(temporary.fileno())
        os.replace(temporary_path, destination)
        temporary_path = None
    finally:
        if temporary_path is not None:
            try:
                temporary_path.unlink(missing_ok=True)
            except OSError:
                pass
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
    source_revision = collect_git_metadata()
    if source_revision["commit_sha"]:
        print(f"Source SHA: {source_revision['commit_sha']}")
        print(f"Git branch: {source_revision['branch'] or '(detached HEAD)'}")
        if source_revision["status"] == "VERIFIED":
            tree_state = "clean" if source_revision["working_tree_clean"] else f"dirty ({source_revision['dirty_path_count']} paths)"
            print(f"Git worktree: {tree_state}")
        else:
            print(f"Git worktree: {source_revision['status']} — {source_revision['error']}")
    else:
        print(f"Source revision: {source_revision['status']} — {source_revision['error']}")
    print(f"Per-command timeout: {'disabled' if timeout_seconds is None else f'{timeout_seconds:g}s'}")

    steps: list[dict[str, Any]] = []
    for name, command in (
        ("Rust compiler", ("rustc", "--version")),
        ("Cargo", ("cargo", "--version")),
    ):
        step = run_step(name, command, env, timeout_seconds)
        steps.append(step)
        if step["status"] != "PASS":
            exit_code = 2
            worktree_after = collect_git_metadata()
            report = make_report(started_at, steps, args, exit_code, source_revision, worktree_after)
            try:
                write_report(args.report, report)
            except OSError as exc:
                print(f"Could not write JSON report: {exc}", file=sys.stderr)
                return 1
            print("\nFINAL STATUS: FAIL — required Rust toolchain is unavailable.")
            return exit_code

    for name, command in CHECKS:
        steps.append(run_step(name, command, env, timeout_seconds))

    passed = sum(step["status"] == "PASS" for step in steps)
    failed = len(steps) - passed
    exit_code = 0 if failed == 0 else 1
    worktree_after = collect_git_metadata()
    report = make_report(started_at, steps, args, exit_code, source_revision, worktree_after)

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
    source_revision: dict[str, Any] | None = None,
    worktree_after: dict[str, Any] | None = None,
) -> dict[str, Any]:
    passed = sum(step["status"] == "PASS" for step in steps)
    revision = source_revision if source_revision is not None else collect_git_metadata()
    final_worktree = worktree_after if worktree_after is not None else collect_git_metadata()
    return {
        "schema_version": 2,
        "repository": str(ROOT),
        "source_revision": revision,
        "worktree_after": final_worktree,
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
            "This report describes checks run on this host only. source_revision records "
            "the Git commit and worktree state before checks; worktree_after records the "
            "state after checks and before report output. It does not prove execution on "
            "other operating systems or target architectures."
        ),
    }


if __name__ == "__main__":
    raise SystemExit(main())
