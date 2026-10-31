# Verification artifacts and workflows

Canonical project-owned verification workflows belong in this repository under `.github/workflows/` and are governed by the repository's normal review and branch-protection rules.

Independent verifier implementations, external runner evidence, and generated verification artifacts may live in the separate verifier repository when the verification contract calls for separation. External evidence must identify the exact canonical source commit SHA and must not be represented as canonical repository CI.

A workflow run, matching source pin, or artifact's presence is not a PASS by itself. Acceptance requires inspection of the actual job result, checkout SHA, steps/logs, and required artifacts for the documented scope. Historical-SHA evidence remains historical after the source SHA changes.

Exact-SHA scanner execution evidence: independent verifier run [40730508735](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40730508735) checked out ONE source `8db21148436d313ebe148f8248267a75f7266ea0` on Ubuntu and Windows. On each OS, 59 scanner tests, 6 collector tests, 5 quality tests, CLI build, and self-scan completed successfully; reports matched the expected checkout SHA and contained 0 critical/high, 11 medium, and 27 informational findings. This proves the documented scanner/test scope on that exact SHA, not final milestone acceptance. The canonical source workflow runs for that SHA failed before runner assignment with empty steps and no artifacts; root cause is unknown, so classify those runs as infrastructure/evidence failure rather than source-test results.
## Repository health and security scanner

Run the read-only integrity/security scanner and its regression corpus from the repository root:

~~~sh
python -m unittest discover -s tools -p "test_one_health.py" -v
python -m unittest discover -s tools -p "test_collect_ci_health.py" -v
python -m unittest discover -s tools -p "test_health_quality.py" -v
python tools/one_health.py --json-out /tmp/one-health-report.json --markdown-out /tmp/one-health-report.md
~~~

The scanner checks repository paths, Markdown links/fences, source/contract drift, selected Python and JavaScript/TypeScript security patterns, GitHub Actions references/permissions, CI evidence freshness, and artifact metadata. Its JavaScript/TypeScript rules are heuristic static indicators, not a parser, taint engine, malware verdict, or OS sandbox. Review findings before acting; do not execute suspicious repository content to validate a finding. The canonical `.github/workflows/repository-health.yml` workflow runs the regression corpus and scans the checked-out ONE repository. Acceptance still requires exact checkout SHA, executable steps/logs, exit codes, and expected artifacts; zero-step failures are blocked evidence, not PASS.
