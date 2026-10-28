#!/usr/bin/env python3
"""Collect exact-SHA GitHub Actions run, job, step, and artifact metadata."""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.parse import urlencode
from urllib.request import Request, urlopen

API_ROOT = "https://api.github.com"
SCHEMA_VERSION = "one-ci-health-evidence/1"


def _request_json(url: str, token: str) -> dict:
    request = Request(url, headers={
        "Accept": "application/vnd.github+json",
        "Authorization": f"Bearer {token}",
        "X-GitHub-Api-Version": "2022-11-28",
        "User-Agent": "one-repository-health",
    })
    with urlopen(request, timeout=15) as response:
        return json.loads(response.read().decode("utf-8"))


def _git_head(root: Path) -> str | None:
    try:
        return subprocess.run(
            ["git", "-C", str(root), "rev-parse", "HEAD"],
            check=True, capture_output=True, text=True, timeout=5,
        ).stdout.strip()
    except (OSError, subprocess.SubprocessError):
        return None


def collect_ci_health(repo: str, source_sha: str, token: str, root: Path,
                      request_json=_request_json) -> dict:
    checked_out_sha = _git_head(root)
    result = {
        "schema_version": SCHEMA_VERSION,
        "collector": "tools/collect_ci_health.py",
        "repository": repo,
        "source_sha": source_sha,
        "checked_out_sha": checked_out_sha,
        "collector_status": "blocked",
        "runs": [],
        "jobs": [],
        "artifacts": [],
        "collection_errors": [],
        "limitations": [
            "GitHub API metadata only; logs are not downloaded or interpreted.",
            "Artifact presence is recorded, not treated as proof of artifact correctness.",
            "Only runs whose head_sha exactly matches source_sha are included.",
        ],
    }
    if not repo or "/" not in repo or not source_sha or len(source_sha) != 40:
        result["collection_errors"].append("invalid repository or source SHA input")
        return result
    if not token:
        result["collection_errors"].append("GITHUB_TOKEN is unavailable; live Actions evidence was not collected")
        return result
    if checked_out_sha != source_sha:
        result["collection_errors"].append(
            f"checked-out HEAD {checked_out_sha!r} does not match requested source SHA"
        )
        return result

    base = f"{API_ROOT}/repos/{repo}"
    try:
        query = urlencode({"head_sha": source_sha, "per_page": 100})
        response = request_json(f"{base}/actions/runs?{query}", token)
        runs = response.get("workflow_runs", [])
        if not isinstance(runs, list):
            raise ValueError("Actions run-list response did not contain a list")
        matching = [run for run in runs if isinstance(run, dict) and run.get("head_sha") == source_sha]
        result["runs"] = [{
            "id": run.get("id"),
            "name": run.get("name"),
            "path": run.get("path"),
            "event": run.get("event"),
            "status": run.get("status"),
            "conclusion": run.get("conclusion"),
            "head_sha": run.get("head_sha"),
            "html_url": run.get("html_url"),
        } for run in matching]
        for run in matching:
            run_id = run.get("id")
            if not isinstance(run_id, int):
                result["collection_errors"].append("Actions run had no integer ID")
                continue
            jobs_response = request_json(f"{base}/actions/runs/{run_id}/jobs?per_page=100", token)
            jobs = jobs_response.get("jobs", [])
            if not isinstance(jobs, list):
                result["collection_errors"].append(f"run {run_id}: jobs response was not a list")
                continue
            for job in jobs:
                if not isinstance(job, dict):
                    continue
                steps = job.get("steps") if isinstance(job.get("steps"), list) else []
                result["jobs"].append({
                    "run_id": run_id,
                    "run_name": run.get("name"),
                    "run_url": run.get("html_url"),
                    "name": job.get("name"),
                    "id": job.get("id"),
                    "status": job.get("status"),
                    "conclusion": job.get("conclusion"),
                    "runner_id": job.get("runner_id"),
                    "logs_url": job.get("html_url"),
                    "steps": [{
                        "name": step.get("name"),
                        "number": step.get("number"),
                        "status": step.get("status"),
                        "conclusion": step.get("conclusion"),
                    } for step in steps if isinstance(step, dict)],
                })
            artifacts_response = request_json(f"{base}/actions/runs/{run_id}/artifacts?per_page=100", token)
            artifacts = artifacts_response.get("artifacts", [])
            if not isinstance(artifacts, list):
                result["collection_errors"].append(f"run {run_id}: artifacts response was not a list")
                continue
            for artifact in artifacts:
                if isinstance(artifact, dict):
                    result["artifacts"].append({
                        "run_id": run_id,
                        "run_name": run.get("name"),
                        "name": artifact.get("name"),
                        "size_in_bytes": artifact.get("size_in_bytes"),
                        "expired": artifact.get("expired"),
                        "digest": artifact.get("digest"),
                        "created_at": artifact.get("created_at"),
                        "url": artifact.get("url"),
                    })
        result["collector_status"] = "complete" if not result["collection_errors"] else "partial"
    except (HTTPError, URLError, TimeoutError, OSError, ValueError, json.JSONDecodeError) as exc:
        result["collection_errors"].append(f"{type(exc).__name__}: {exc}")
        result["collector_status"] = "partial" if result["runs"] or result["jobs"] else "blocked"
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", default=os.environ.get("GITHUB_REPOSITORY", ""))
    parser.add_argument("--sha", default=os.environ.get("GITHUB_SHA", ""))
    parser.add_argument("--root", type=Path, default=Path("."))
    parser.add_argument("--output", type=Path, default=Path("ci-health-evidence.json"))
    args = parser.parse_args(argv)
    report = collect_ci_health(args.repo, args.sha, os.environ.get("GITHUB_TOKEN", ""), args.root.resolve())
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"CI evidence collection: {report['collector_status']} ({len(report['runs'])} runs, "
          f"{len(report['jobs'])} jobs, {len(report['artifacts'])} artifacts)")
    if report["collection_errors"]:
        print("; ".join(report["collection_errors"]), file=sys.stderr)
    # Collection outages are recorded as blocked evidence rather than misreported as source failures.
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
