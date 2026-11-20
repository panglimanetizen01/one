#!/usr/bin/env python3
"""Fetch only the failed CircleCI job steps and their logs for ONE.

Uses CircleCI API v2 only to discover pipeline/workflow/job UUIDs, then uses
the current CircleCI API v3 for job metadata and per-step logs.
No third-party Python packages are required.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Sequence

DEFAULT_PROJECT_SLUG = "gh/panglimanetizen01/one"
API_ROOT = "https://circleci.com"
FAILED_OUTCOMES = {"failed", "timed_out", "timedout", "error"}
FAILED_JOB_STATUSES = {"failed", "timed_out", "timedout", "error"}
SECRET_PATTERNS = [
    (re.compile(r"(?i)(Bearer\s+)[A-Za-z0-9._~+/=-]+"), r"\1[REDACTED]"),
    (re.compile(r"(?i)\b(CIRCLECI_TOKEN|CIRCLE_TOKEN|GITHUB_TOKEN|GH_TOKEN)\s*=\s*\S+"), r"\1=[REDACTED]"),
    (re.compile(r"(?i)\b(token|password|passwd|secret|api[_-]?key)\s*([=:]\s*)[^\s,;]+"), r"\1\2[REDACTED]"),
    (re.compile(r"\bgh[pousr]_[A-Za-z0-9_]{20,}\b"), "[REDACTED_GITHUB_TOKEN]"),
    (re.compile(r"\bgithub_pat_[A-Za-z0-9_]{20,}\b"), "[REDACTED_GITHUB_TOKEN]"),
]


class DiagnosticError(RuntimeError):
    pass


def redact(text: str) -> str:
    for pattern, replacement in SECRET_PATTERNS:
        text = pattern.sub(replacement, text)
    return text


def extract_failed_steps(job_payload: Dict[str, Any]) -> List[Dict[str, Any]]:
    """Return failed steps with their parallel execution index."""
    data = job_payload.get("data") or {}
    attributes = data.get("attributes") or {}
    executions = attributes.get("parallel_executions") or []
    found: List[Dict[str, Any]] = []
    for execution_index, execution in enumerate(executions):
        for step in execution.get("steps") or []:
            outcome = str(step.get("outcome") or "").lower()
            if outcome in FAILED_OUTCOMES:
                found.append({
                    "execution": execution_index,
                    "num": step.get("num"),
                    "name": step.get("name") or "Unnamed step",
                    "outcome": outcome,
                })
    return found


def classify_failure(text: str) -> str:
    """Classify captured provider log text only, never report narration/metadata."""
    lowered = text.lower()
    if any(term in lowered for term in (
        "too long with no output", "context deadline exceeded",
        "timed out", "timed_out", "operation timed out",
    )) or re.search(r"\btimeout\b", lowered):
        return "TIMEOUT / NO-OUTPUT"
    if any(term in lowered for term in (
        "error[e", "could not compile", "clang: error",
        "gcc: error", "linker", "undefined reference",
    )):
        return "COMPILER / LINKER"
    if any(term in lowered for term in (
        "test result: failed", "test failed", "assertion failed",
        "panicked at", "failures:",
    )):
        return "TEST FAILURE"
    if any(term in lowered for term in (
        "401 unauthorized", "403 forbidden", "unauthorized",
        "authentication failed", "invalid token",
    )):
        return "AUTHENTICATION / PERMISSION"
    if any(term in lowered for term in (
        "apt-get", "sdkmanager", "failed to fetch",
        "temporary failure in name resolution", "could not resolve",
        "connection reset", "429 too many requests",
    )):
        return "DEPENDENCY / NETWORK / PROVISIONING"
    if any(term in lowered for term in (
        "config validation", "configuration is invalid", "yaml:",
        "unclosed tag", "unknown orb", "schema validation",
    )):
        return "CI CONFIGURATION"
    return "UNCLASSIFIED — inspect the evidence below"


def classify_job_evidence(
    step_outcomes: Sequence[str],
    started_at: Optional[str],
    log_text: str,
    start_time_known: bool = True,
) -> str:
    """Keep provider outcomes distinct from logs and explicitly null start time."""
    outcomes = {str(value or "").lower() for value in step_outcomes}
    if outcomes.intersection({"timed_out", "timedout"}):
        return "TIMEOUT / NO-OUTPUT"
    if not log_text.strip():
        if start_time_known and not started_at:
            return "PRE-EXECUTION TERMINATION / ROOT CAUSE UNKNOWN"
        if not start_time_known:
            return "NO-OUTPUT / START TIME UNKNOWN"
        return "NO-OUTPUT / ROOT CAUSE UNKNOWN"
    return classify_failure(log_text)


def request_bytes(url: str, headers: Dict[str, str], allow_missing: bool = False) -> bytes:
    request = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(request, timeout=90) as response:
            return response.read()
    except urllib.error.HTTPError as exc:
        if allow_missing and exc.code in (204, 404):
            return b""
        detail = exc.read().decode("utf-8", errors="replace")[:1200]
        raise DiagnosticError(
            "CircleCI API returned HTTP {} for {}: {}".format(
                exc.code, url, redact(detail)
            )
        ) from exc
    except urllib.error.URLError as exc:
        raise DiagnosticError("CircleCI API connection failed: {}".format(exc)) from exc


def get_json(url: str, headers: Dict[str, str]) -> Dict[str, Any]:
    raw = request_bytes(url, headers)
    try:
        parsed = json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise DiagnosticError("CircleCI API returned invalid JSON for " + url) from exc
    if not isinstance(parsed, dict):
        raise DiagnosticError("Unexpected CircleCI API response shape for " + url)
    return parsed


def v2_headers(token: str) -> Dict[str, str]:
    return {"Circle-Token": token, "Accept": "application/json"}


def v3_headers(token: str) -> Dict[str, str]:
    return {"Authorization": "Bearer " + token, "Accept": "application/json"}


def paged_v2_items(url: str, headers: Dict[str, str]) -> Iterable[Dict[str, Any]]:
    """Yield every page without truncation and reject pagination-token cycles."""
    page_url = url
    seen_tokens = set()
    while page_url:
        payload = get_json(page_url, headers)
        items = payload.get("items")
        if not isinstance(items, list):
            raise DiagnosticError(
                "CircleCI API pagination response has no valid 'items' array; "
                "discovery was stopped."
            )
        for item in items:
            if not isinstance(item, dict):
                raise DiagnosticError(
                    "CircleCI API pagination response contains a non-object item; "
                    "discovery was stopped."
                )
            yield item
        next_token = payload.get("next_page_token")
        if next_token is None:
            return
        if not isinstance(next_token, str) or not next_token:
            raise DiagnosticError(
                "CircleCI API pagination returned an invalid next_page_token; "
                "discovery was stopped."
            )
        if next_token in seen_tokens:
            raise DiagnosticError(
                "CircleCI API pagination repeated a next_page_token; "
                "discovery was stopped to prevent an infinite loop."
            )
        seen_tokens.add(next_token)
        separator = "&" if "?" in url else "?"
        page_url = url + separator + urllib.parse.urlencode({"page-token": next_token})


def git_value(*args: str) -> str:
    try:
        return subprocess.check_output(
            ["git", *args], stderr=subprocess.DEVNULL, text=True
        ).strip()
    except (OSError, subprocess.CalledProcessError) as exc:
        raise DiagnosticError(
            "Cannot determine the current Git revision. Run inside the ONE repository "
            "or pass --sha and --branch."
        ) from exc


def discover_failed_jobs(
    token: str, project_slug: str, branch: str, commit_sha: str
) -> List[Dict[str, Any]]:
    encoded_slug = urllib.parse.quote(project_slug, safe="/")
    query = urllib.parse.urlencode({"branch": branch})
    pipeline_url = "{}/api/v2/project/{}/pipeline?{}".format(
        API_ROOT, encoded_slug, query
    )
    pipelines = [
        item for item in paged_v2_items(pipeline_url, v2_headers(token))
        if ((item.get("vcs") or {}).get("revision") == commit_sha)
    ]
    if not pipelines:
        raise DiagnosticError(
            "No CircleCI pipeline was found for exact HEAD {} on branch {}. "
            "Refusing to diagnose a different commit. Pass --job-id UUID to inspect "
            "a known job explicitly.".format(commit_sha, branch)
        )
    pipelines.sort(key=lambda item: item.get("created_at") or "")
    failures: List[Dict[str, Any]] = []
    for pipeline in pipelines:
        pipeline_id = pipeline.get("id")
        if not pipeline_id:
            raise DiagnosticError("An exact-SHA pipeline has no UUID.")
        workflow_url = "{}/api/v2/pipeline/{}/workflow".format(API_ROOT, pipeline_id)
        workflows = list(paged_v2_items(workflow_url, v2_headers(token)))
        for workflow in workflows:
            workflow_id = workflow.get("id")
            if not workflow_id:
                continue
            jobs_url = "{}/api/v2/workflow/{}/job".format(API_ROOT, workflow_id)
            for job in paged_v2_items(jobs_url, v2_headers(token)):
                status = str(job.get("status") or "").lower()
                if status in FAILED_JOB_STATUSES and job.get("id"):
                    failures.append({
                        "id": job["id"],
                        "name": job.get("name") or "Unnamed job",
                        "job_number": job.get("job_number"),
                        "status": status,
                        "workflow": workflow.get("name") or workflow_id,
                        "pipeline_id": pipeline_id,
                        "pipeline_number": pipeline.get("number"),
                        "pipeline_status": pipeline.get("state") or pipeline.get("status"),
                        "pipeline_revision": (pipeline.get("vcs") or {}).get("revision"),
                        "workflow_id": workflow_id,
                        "workflow_status": workflow.get("status"),
                        "job_started_at": job.get("started_at"),
                        "job_start_time_known": "started_at" in job,
                        "commit_sha": commit_sha,
                        "branch": branch,
                    })
    if not failures:
        # An exact-SHA pipeline with no failed job is not a diagnostic/API error:
        # it may still be running, may have succeeded, or may have no jobs.
        # The caller must not report this as a build failure or as a PASS.
        return []
    return failures



def source_identity_line(job: Dict[str, Any]) -> str:
    """Make the source-SHA evidence boundary visible in every job report."""
    commit_sha = job.get("commit_sha")
    if commit_sha:
        return "SOURCE SHA: {}".format(commit_sha)
    return (
        "SOURCE SHA: UNVERIFIED — direct --job-id lookup bypasses exact-SHA "
        "pipeline discovery. Verify the workflow/pipeline revision in CircleCI "
        "before attributing these logs to a commit."
    )

def inspect_job(token: str, job: Dict[str, Any], max_lines: int) -> List[str]:
    report: List[str] = []
    job_id = str(job["id"])
    detail_url = "{}/api/v3/jobs/{}".format(API_ROOT, urllib.parse.quote(job_id))
    payload = get_json(detail_url, v3_headers(token))
    data = payload.get("data") or {}
    attributes = data.get("attributes") or {}
    job_name = attributes.get("name") or job.get("name") or "Unnamed job"
    job_outcome = attributes.get("outcome") or job.get("status") or "unknown"
    report.extend([
        "",
        "=" * 78,
        "JOB: {} | NUMBER: {} | OUTCOME: {}".format(
            job_name, job.get("job_number") or "n/a", job_outcome
        ),
        "JOB UUID: {}".format(job_id),
        "WORKFLOW: {}".format(job.get("workflow") or "explicit job lookup"),
        "PIPELINE: {} | UUID: {} | STATUS: {}".format(job.get("pipeline_number") or "n/a", job.get("pipeline_id") or "not resolved", job.get("pipeline_status") or "UNVERIFIED"),
        "PIPELINE REVISION: {}".format(job.get("pipeline_revision") or "UNVERIFIED"),
        "WORKFLOW UUID: {}".format(job.get("workflow_id") or "UNVERIFIED"),
        "WORKFLOW STATUS: {}".format(job.get("workflow_status") or "UNVERIFIED"),
        "JOB STARTED AT: {}".format(
            (job.get("job_started_at") or "null / not started")
            if job.get("pipeline_id") and job.get("job_start_time_known")
            else ("UNKNOWN" if job.get("pipeline_id") else "UNVERIFIED (direct job lookup)")
        ),
        "=" * 78,
    ])
    report.append(source_identity_line(job))
    steps = extract_failed_steps(payload)
    if not steps:
        report.append(
            "No step with failed/timed_out outcome was exposed by API v3. "
            "Showing all non-successful completed steps, if any."
        )
        executions = attributes.get("parallel_executions") or []
        for execution_index, execution in enumerate(executions):
            for step in execution.get("steps") or []:
                outcome = str(step.get("outcome") or "").lower()
                if outcome and outcome != "succeeded":
                    steps.append({
                        "execution": execution_index,
                        "num": step.get("num"),
                        "name": step.get("name") or "Unnamed step",
                        "outcome": outcome,
                    })
    if not steps:
        report.append("No failed step details are present in the job response.")
        return report

    log_evidence: List[str] = []
    for step in steps:
        number = step.get("num")
        if number is None:
            report.append("Cannot fetch logs: failed step has no step number.")
            continue
        execution = int(step["execution"])
        report.extend([
            "",
            "FAILED STEP #{} | EXECUTION {} | {} | {}".format(
                number, execution, step["outcome"].upper(), step["name"]
            ),
        ])
        params = urllib.parse.urlencode({
            "filter[step_num]": str(number),
            "filter[execution]": str(execution),
        })
        for stream in ("stdout", "stderr"):
            log_url = "{}/api/v3/jobs/{}/{}?{}".format(
                API_ROOT, urllib.parse.quote(job_id), stream, params
            )
            raw = request_bytes(log_url, v3_headers(token), allow_missing=True)
            text = redact(raw.decode("utf-8", errors="replace"))
            if text.strip():
                log_evidence.append(text)
            if not text.strip():
                report.append("--- {}: no output ---".format(stream.upper()))
                continue
            lines = text.splitlines()
            report.append("--- {}: last {} of {} lines ---".format(
                stream.upper(), min(max_lines, len(lines)), len(lines)
            ))
            report.extend(lines[-max_lines:])
    classification = classify_job_evidence(
        [step.get("outcome") for step in steps],
        attributes.get("started_at") or job.get("job_started_at"),
        "\n".join(log_evidence),
        "started_at" in attributes or job.get("job_start_time_known", False),
    )
    report.insert(0, "EVIDENCE CLASSIFICATION: " + classification)
    report.append("")
    report.append("Note: classification is a log-based clue, not an automatic root-cause verdict.")
    return report


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = argparse.ArgumentParser(
        description="Find failed CircleCI jobs for the current ONE commit and print only failed-step logs."
    )
    parser.add_argument("--job-id", help="Inspect a known CircleCI job UUID directly.")
    parser.add_argument("--sha", help="Exact Git commit SHA; defaults to git HEAD.")
    parser.add_argument("--branch", help="Git branch; defaults to the current branch.")
    parser.add_argument("--project-slug", default=os.environ.get(
        "CIRCLECI_PROJECT_SLUG", DEFAULT_PROJECT_SLUG
    ))
    parser.add_argument("--max-lines", type=int, default=100)
    parser.add_argument("--output", help="Also save the redacted report to this path.")
    args = parser.parse_args(argv)

    token = os.environ.get("CIRCLECI_TOKEN") or os.environ.get("CIRCLE_TOKEN")
    if not token:
        print(
            "ERROR: Set CIRCLECI_TOKEN in your local environment. "
            "Never paste the token into chat or commit it.",
            file=sys.stderr,
        )
        return 2
    if args.max_lines < 1 or args.max_lines > 1000:
        parser.error("--max-lines must be between 1 and 1000")

    try:
        if args.job_id:
            jobs = [{
                "id": args.job_id,
                "name": "Explicit job UUID",
                "status": "unknown",
                "workflow": "not resolved",
            }]
            commit_sha = None
            branch = None
        else:
            commit_sha = args.sha or git_value("rev-parse", "HEAD")
            branch = args.branch or git_value("branch", "--show-current")
            if not branch:
                raise DiagnosticError(
                    "Git is in detached-HEAD state. Pass --branch explicitly."
                )
            print("Project: {}".format(args.project_slug))
            print("Branch: {}".format(branch))
            print("Exact commit: {}".format(commit_sha))
            jobs = discover_failed_jobs(
                token, args.project_slug, branch, commit_sha
            )

        report = [
            "ONE / CIRCLECI TARGETED FAILURE DIAGNOSTIC",
            "Project: " + args.project_slug,
        ]
        if commit_sha:
            report.append("Exact commit: " + commit_sha)
            report.append("Branch: " + str(branch))
        report.append("Jobs selected for inspection: {}".format(len(jobs)))
        if not jobs and not args.job_id:
            report.append(
                "No failed or timed-out jobs were listed for the exact-SHA pipeline. "
                "This is NOT a PASS: the pipeline may still be running, may have "
                "succeeded, or may contain no jobs. Check workflow status in CircleCI."
            )
        for job in jobs:
            report.extend(inspect_job(token, job, args.max_lines))

        output = redact("\n".join(report).rstrip() + "\n")
        print(output, end="")
        if args.output:
            destination = Path(args.output).expanduser()
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(output, encoding="utf-8")
            print("\nSaved redacted report: {}".format(destination))
        return 0
    except DiagnosticError as exc:
        print("DIAGNOSTIC ERROR: " + redact(str(exc)), file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
