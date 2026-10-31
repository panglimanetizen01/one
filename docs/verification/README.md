# Verification artifacts and workflows

## Active exact-SHA recovery — 2026-10-31

This current-state block supersedes prior active summaries. The parent candidate before this documentation reconciliation was `8ddcfb8c752aee78e94b47d03402ab1f5fd3ea92`. This commit advances the source SHA; the live PR #80 head and independent verifier pin are the authoritative exact current SHA, avoiding a self-referential SHA inside this file.

- **Source/PR:** `panglimanetizen01/one`, branch `milestone/100-ecosystem-20261023`, PR #80 open/draft/unmerged; base `main` `d35db95153f6ddd79885494c72f5aee16c899751`.
- **Implemented delta:** source ancestry contains immutable action pins (commit `e58a68987fbdf0c6cb0498d8a5ed52fb5e73e439`) and the Check Run collector work (commits `b418b31ecd779c82972819604182a5585b4d8236`, `fecf9cfb88aa44cd599abde010362ea803ffc32b`, `297b169cb4cbb753b6a041f20a3f7eb22d5562d0`). The collector fetches exact-SHA Check Runs and up to 100 annotations per Check Run, redacts selected high-signal secret patterns, rejects missing/mismatched Check Run SHA values, and detects explicit billing/spending-limit denial. The scanner classifies this as an informational/high-confidence infrastructure/account blocker, not a source failure. The workflow grants read-only `checks: read`.
- **Regression inventory:** `tools/test_one_health.py` contains 62 test methods; `tools/test_collect_ci_health.py` contains 9. These are source counts only; no test execution is implied.
- **Latest canonical source runs on parent `8ddcfb8...`:** One CI [41217339585](https://github.com/panglimanetizen01/one/actions/runs/41217339585), Repository Health [41217339577](https://github.com/panglimanetizen01/one/actions/runs/41217339577), focused stdlib [41217339600](https://github.com/panglimanetizen01/one/actions/runs/41217339600), E1 [41217339578](https://github.com/panglimanetizen01/one/actions/runs/41217339578), E2 [41217339580](https://github.com/panglimanetizen01/one/actions/runs/41217339580), Linux distro [41217339576](https://github.com/panglimanetizen01/one/actions/runs/41217339576). All inspected jobs have `runner_id=0`, empty runner name, zero steps, and no artifacts. This is pre-run infrastructure/evidence failure; root cause UNKNOWN, not source failure and not PASS.
- **Parent `297b169...` Check Runs:** 15 exact-SHA Check Runs were returned; all had null output title/summary. No explicit billing block was established. The annotation API was not independently retrievable through the available connector. The collector changes have not yet run inside repository-health CI.
- **Verifier:** before this docs commit, `CANONICAL_COMMIT.txt` and `CANDIDATE_VERIFICATION.md` were synchronized to parent `8ddcfb8...`; verifier branch head was `769a239e07e65152f7828bd5adf61338e3852248`. After this commit, update both to the new live PR #80 head. Preserve separate `LOCAL_CI_CANONICAL_COMMIT.txt` at PR #92 SHA `62e391e699fe4ac5a18d6feb808568c192dd82e1`.
- **DoD/ledger:** 3 checked / 10 unchecked / 13 total. Implementation 92/100 provisional; verified E1 0/10, E2 0/8, E3 0/8, E4 0/6, E5 0/4, E6 0/4. Final score UNKNOWN / NOT RECONCILED; gate NOT PASSED. Fixed weights unchanged.
- **No-repeat/safety:** do not repeat the 132-file inventory or identical zero-step workflow retries. No merge, production release, protected E2 dispatch, secret/trust-store mutation, branch-protection bypass, or direct main edit. E2 release remains unauthorized without explicit approval.


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
