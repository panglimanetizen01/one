#!/usr/bin/env python3
"""Create and verify exact-SHA Local CI report manifests."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import tempfile
from pathlib import Path
from typing import Any

SCHEMA_VERSION = 2
EXPECTED_STEP_COUNT = 12
EXPECTED_STEPS: tuple[tuple[str, tuple[str, ...] | None], ...] = (
    ("Rust compiler", ("rustc", "--version")),
    ("Cargo", ("cargo", "--version")),
    ("Contract artifacts", None),
    ("Build workspace", ("cargo", "--locked", "build", "--workspace", "--all-targets")),
    ("Clippy", ("cargo", "clippy", "--locked", "--workspace", "--all-targets", "--", "-D", "warnings")),
    ("Workspace tests", ("cargo", "--locked", "test", "--workspace", "--all-targets")),
    ("Deterministic .oneir/.onepkg artifacts", ("cargo", "--locked", "test", "-p", "one-cli", "--test", "cli", "build_and_package_artifacts_are_byte_deterministic")),
    ("DNS success/failure classification", ("cargo", "--locked", "test", "-p", "one-core", "--all-targets", "dns_adapter")),
    ("Orchestration policy paths", ("cargo", "--locked", "test", "-p", "one-core", "--all-targets", "orchestrator")),
    ("Database vertical slice", ("cargo", "--locked", "test", "-p", "one-core", "--test", "database_vertical_slice")),
    ("HTTP GET vertical slice", ("cargo", "--locked", "test", "-p", "one-core", "--test", "http_get_vertical_slice")),
    ("HTTP GET CLI vertical slice", ("cargo", "--locked", "test", "-p", "one-cli", "--test", "http_get_orchestration")),
)
EXPECTED_STEP_NAMES = tuple(name for name, _command in EXPECTED_STEPS)
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")


def validate_step_records(steps: list[Any]) -> None:
    actual_names = tuple(step.get("name") if isinstance(step, dict) else None for step in steps)
    if actual_names != EXPECTED_STEP_NAMES:
        raise EvidenceValidationError("report steps do not match the canonical Local CI check sequence")

    for (name, expected_command), step in zip(EXPECTED_STEPS, steps, strict=True):
        if step.get("status") != "PASS" or step.get("exit_code") != 0:
            raise EvidenceValidationError(f"Local CI step '{name}' did not record PASS with exit code 0")
        command = step.get("command")
        if not isinstance(command, list) or not command or not all(isinstance(part, str) for part in command):
            raise EvidenceValidationError(f"Local CI step '{name}' has an invalid command record")
        if expected_command is None:
            executable = command[0].replace("\\", "/").rsplit("/", 1)[-1].lower()
            if len(command) != 2 or not executable.startswith("python") or command[1] != "tools/check_contract.py":
                raise EvidenceValidationError("Contract artifacts step command does not match the canonical check")
        elif tuple(command) != expected_command:
            raise EvidenceValidationError(f"Local CI step '{name}' command does not match the canonical check")


class EvidenceValidationError(ValueError):
    """Raised when a Local CI report or evidence manifest is invalid."""


def validate_report_payload(payload: Any, expected_source_sha: str) -> None:
    if not isinstance(payload, dict):
        raise EvidenceValidationError("report root must be a JSON object")
    if payload.get("schema_version") != SCHEMA_VERSION:
        raise EvidenceValidationError("unsupported Local CI report schema")
    if not SHA_PATTERN.fullmatch(expected_source_sha):
        raise EvidenceValidationError("expected source SHA must be a full 40-character lowercase SHA")

    source = payload.get("source_revision")
    after = payload.get("worktree_after")
    if not isinstance(source, dict) or not isinstance(after, dict):
        raise EvidenceValidationError("report must contain source_revision and worktree_after objects")
    if source.get("status") != "VERIFIED" or source.get("commit_sha") != expected_source_sha:
        raise EvidenceValidationError("pre-run source revision does not match the expected source SHA")
    if source.get("working_tree_clean") is not True or source.get("dirty_path_count") != 0:
        raise EvidenceValidationError("pre-run source worktree is not clean")
    if after.get("status") != "VERIFIED" or after.get("commit_sha") != expected_source_sha:
        raise EvidenceValidationError("post-check worktree revision does not match the expected source SHA")
    if after.get("working_tree_clean") is not True or after.get("dirty_path_count") != 0:
        raise EvidenceValidationError("post-check source worktree is not clean")

    steps = payload.get("steps")
    if not isinstance(steps, list) or len(steps) != EXPECTED_STEP_COUNT:
        raise EvidenceValidationError(f"report must contain exactly {EXPECTED_STEP_COUNT} Local CI steps")
    validate_step_records(steps)
    passed = len(steps)
    failed = 0
    expected_summary = {
        "total": EXPECTED_STEP_COUNT,
        "passed": passed,
        "failed_or_unavailable": failed,
    }
    if payload.get("summary") != expected_summary:
        raise EvidenceValidationError("report summary does not match the individual step results")
    if passed != EXPECTED_STEP_COUNT:
        raise EvidenceValidationError("one or more Local CI steps did not PASS")
    if payload.get("status") != "PASS" or payload.get("exit_code") != 0:
        raise EvidenceValidationError("Local CI report status/exit code is not PASS/0")


def write_json_atomic(path: Path, payload: dict[str, Any]) -> None:
    destination = path
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
            temporary.write(json.dumps(payload, indent=2) + "\n")
            temporary.flush()
            os.fsync(temporary.fileno())
        os.replace(temporary_path, destination)
        temporary_path = None
    finally:
        if temporary_path is not None:
            temporary_path.unlink(missing_ok=True)


def create_manifest(
    report_path: Path,
    manifest_path: Path,
    expected_source_sha: str,
    *,
    verifier_sha: str | None = None,
    workflow: str | None = None,
    run_id: str | None = None,
    run_attempt: str | None = None,
    job: str | None = None,
    os_name: str | None = None,
    arch: str | None = None,
) -> dict[str, Any]:
    report_bytes = report_path.read_bytes() if report_path.is_file() else b""
    payload: Any = None
    errors: list[str] = []
    if not report_bytes:
        validation_status = "MISSING_REPORT"
        errors.append("report file is missing or empty")
    else:
        try:
            payload = json.loads(report_bytes)
        except (UnicodeDecodeError, json.JSONDecodeError) as exc:
            validation_status = "INVALID_JSON"
            errors.append(f"report JSON is invalid: {exc}")
        else:
            try:
                validate_report_payload(payload, expected_source_sha)
            except EvidenceValidationError as exc:
                validation_status = "FAIL"
                errors.append(str(exc))
            else:
                validation_status = "PASS"

    manifest: dict[str, Any] = {
        "schema_version": 1,
        "source_sha": expected_source_sha,
        "verifier_sha": verifier_sha,
        "workflow": workflow,
        "run_id": run_id,
        "run_attempt": run_attempt,
        "job": job,
        "os": os_name,
        "arch": arch,
        "report_file": report_path.as_posix(),
        "report_size_bytes": len(report_bytes),
        "report_sha256": hashlib.sha256(report_bytes).hexdigest() if report_bytes else None,
        "report_schema_version": payload.get("schema_version") if isinstance(payload, dict) else None,
        "report_status": payload.get("status") if isinstance(payload, dict) else None,
        "report_source_sha": payload.get("source_revision", {}).get("commit_sha")
        if isinstance(payload, dict) and isinstance(payload.get("source_revision"), dict)
        else None,
        "worktree_after_clean": payload.get("worktree_after", {}).get("working_tree_clean")
        if isinstance(payload, dict) and isinstance(payload.get("worktree_after"), dict)
        else None,
        "validation_status": validation_status,
        "validation_errors": errors,
    }
    write_json_atomic(manifest_path, manifest)
    return manifest


def verify_manifest(
    report_path: Path,
    manifest_path: Path,
    expected_source_sha: str,
    *,
    expected_verifier_sha: str | None = None,
) -> dict[str, Any]:
    if not report_path.is_file():
        raise EvidenceValidationError("report artifact is missing")
    if not manifest_path.is_file():
        raise EvidenceValidationError("report manifest is missing")
    report_bytes = report_path.read_bytes()
    try:
        manifest = json.loads(manifest_path.read_bytes())
        payload = json.loads(report_bytes)
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise EvidenceValidationError(f"report or manifest JSON is invalid: {exc}") from exc
    if not isinstance(manifest, dict):
        raise EvidenceValidationError("manifest root must be a JSON object")
    actual_digest = hashlib.sha256(report_bytes).hexdigest()
    if manifest.get("report_sha256") != actual_digest:
        raise EvidenceValidationError("report SHA-256 does not match the manifest")
    if manifest.get("report_size_bytes") != len(report_bytes):
        raise EvidenceValidationError("report byte size does not match the manifest")
    if manifest.get("source_sha") != expected_source_sha:
        raise EvidenceValidationError("manifest source SHA does not match the expected source SHA")
    if expected_verifier_sha is not None and manifest.get("verifier_sha") != expected_verifier_sha:
        raise EvidenceValidationError("manifest verifier SHA does not match the expected verifier SHA")
    if manifest.get("validation_status") != "PASS":
        raise EvidenceValidationError("manifest validation status is not PASS")
    validate_report_payload(payload, expected_source_sha)
    return manifest


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Create or verify exact-SHA Local CI evidence manifests.")
    subparsers = parser.add_subparsers(dest="command", required=True)
    for command in ("create", "verify"):
        child = subparsers.add_parser(command)
        child.add_argument("--report", type=Path, required=True)
        child.add_argument("--manifest", type=Path, required=True)
        child.add_argument("--expected-source-sha", required=True)
        if command == "create":
            child.add_argument("--verifier-sha")
            child.add_argument("--workflow")
            child.add_argument("--run-id")
            child.add_argument("--run-attempt")
            child.add_argument("--job")
            child.add_argument("--os", dest="os_name")
            child.add_argument("--arch")
        else:
            child.add_argument("--expected-verifier-sha")
    return parser


def main() -> int:
    args = build_parser().parse_args()
    try:
        if args.command == "create":
            manifest = create_manifest(
                args.report,
                args.manifest,
                args.expected_source_sha,
                verifier_sha=args.verifier_sha,
                workflow=args.workflow,
                run_id=args.run_id,
                run_attempt=args.run_attempt,
                job=args.job,
                os_name=args.os_name,
                arch=args.arch,
            )
            print(f"Manifest written: {args.manifest} (validation_status={manifest['validation_status']})")
            return 0
        verify_manifest(
            args.report,
            args.manifest,
            args.expected_source_sha,
            expected_verifier_sha=args.expected_verifier_sha,
        )
        print("Local CI report and manifest integrity PASS")
        return 0
    except (EvidenceValidationError, OSError) as exc:
        print(f"Local CI evidence validation failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
