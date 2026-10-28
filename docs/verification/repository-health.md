# One Repository Health Scanner

Status: **implemented first increment; evolving**  
Tool: tools/one_health.py  
Regression tests: tools/test_one_health.py  
CI: .github/workflows/repository-health.yml

This tool checks the integrity of the repository around the One language. It is **not a One-language command** and does not change One syntax or grammar. One source programs remain governed by the normative grammar and CLI contracts.

## Run locally

Requires Python 3.11 or newer and a checkout of the repository.

~~~sh
python tools/one_health.py
python -m unittest discover -s tools -p "test_one_health.py" -v
python -m unittest discover -s tools -p "test_collect_ci_health.py" -v
python -m unittest discover -s tools -p "test_health_quality.py" -v
~~~

Create machine-readable and human-readable reports outside the tracked source tree when possible:

~~~sh
python tools/one_health.py --json-out /tmp/one-health.json --markdown-out /tmp/one-health.md
~~~

On Windows PowerShell, replace /tmp/... with a suitable temporary path, for example $env:TEMP\\one-health.json.

To assert that the checkout matches an exact commit:

~~~sh
python tools/one_health.py --expected-sha <40-character-git-sha>
~~~

CI sets GITHUB_SHA automatically. A mismatch is a critical finding and causes a non-zero exit.

## Current detector catalogue

| Detector | What it checks | Current boundary |
|---|---|---|
| Revision integrity | expected SHA versus git rev-parse HEAD | Checks exact identity only when an expected SHA is supplied |
| Syntax | Python AST, TOML and JSON parsing | TOML uses Python 3.11+ tomllib |
| Markdown | relative links, repository-root escapes, conservative heading/HTML anchors, unclosed fenced code blocks | Does not parse every Markdown dialect or every generated/reference-style link |
| Paths | symlinks resolving outside the repository | Does not follow symlink targets |
| Workflow integrity | duplicate workflow names, missing local actions, missing referenced scripts | Targeted checks, not a complete GitHub Actions schema validator |
| Workflow overlap | exact repeated single-line run commands across workflow files | Low-confidence review hint; repetition may be intentional |
| Evidence freshness | source SHA versus checked-out SHA in structured JSON evidence | Only for records that declare both values |
| Live CI metadata and correlations | exact-SHA Actions runs/jobs/steps/artifacts; surfaced findings for failed runs without jobs, failed jobs without visible steps, failed steps needing log review, and absent artifacts | Requires Actions read permission; logs are link-only; absence is not proof of a source failure or missing required artifact |
| Normative One contract drift | required grammar, CLI, diagnostics, package/build, and platform invariants | Text-presence checks only; conformance tests and human review remain authoritative |
| One source fixtures | runs one periksa against normative v0.1 valid/invalid fixtures when explicitly enabled | Requires a built CLI; never executes source |
| CLI documentation drift | approved core CLI vocabulary against the user-facing CLI guide | A focused consistency check; not proof the command is implemented |
| Artifact integrity hints | expired artifacts, duplicate names per run, missing digest metadata | Does not download/rehash artifact bytes; missing digest is a limitation, not a clean integrity result |

The scanner does not interpret free-form Indonesian as executable source. It does not claim that an example is valid One code simply because it is in Markdown. Normative contract checks detect disappearance of high-value invariants but are deliberately not a substitute for parser tests or human contract review. Language examples must be checked against the One grammar, implementation status, and conformance tests.

## Finding contract

Every finding includes a stable ID, detector/rule IDs, status, severity, confidence, path/line, evidence, and a recommended next action. The report schema is one-health-report/1. JSON and Markdown reports also expose detector status (ran, not applicable due to absent input, or not requested) and finding counts by detector. These counts are descriptive coverage metrics, not an accuracy or repair-success rate; confirmed false-positive, false-negative, and repair outcome tracking remains future work.

- **critical / high:** fail the scan because revision identity or a severe structural/syntax condition cannot be trusted.
- **medium:** actionable integrity issue; requires triage.
- **low / info:** review hint or coverage limitation.

The report is evidence about the checks that actually ran, not a root-cause declaration. Missing logs or unavailable services must remain unknown/blocked unless additional evidence establishes the cause. Findings are deduplicated and sorted to make repeated reports diff-friendly.

## Guarded repair (Tier 2)

The only automated repair currently allowlisted is adding a final newline to a Markdown file. First inspect the proposal:

~~~sh
python tools/one_health.py --propose-safe-fixes --repair-path docs/berkas-yang-dipilih.md --quiet
~~~

Applying it is deliberately constrained:

~~~sh
python tools/one_health.py --apply-safe-fixes --repair-path docs/berkas-yang-dipilih.md --quiet
~~~

The apply mode requires an explicit exact `--repair-path` for every file to change, rejects absolute/traversal paths and non-Markdown targets, and refuses to run unless the checkout is on a dedicated branch whose name starts with health/ or diagnostic/, the working tree is clean, and any supplied expected SHA matches HEAD. It does not fix broken links, rewrite One source, modify contracts, change workflow permissions, update acceptance weights, merge, publish, or release. Those actions require explicit review and their own verification.

After any repair, inspect the diff, rerun scanner tests and the scan, then run the required language and cross-platform tests for the resulting SHA. A result for the previous SHA does not validate a changed commit.

## Continuous development roadmap

### A — Evidence foundation
- [x] Deterministic report schema and stable finding IDs.
- [x] Optional exact-SHA checkout check.
- [x] JSON/Markdown output and source/checkout SHA mismatch detector.
- [x] Saved and live-collected CI metadata classification: no visible steps remains blocked/unknown, failed steps require log review; API outages remain blocked/partial.
- [x] Fixture tests for core failure classes.
- [x] Collect exact-SHA workflow run/job/step/artifact metadata directly from GitHub APIs; logs remain link-only and are not automatically interpreted.
- [x] Record scanner/runtime environment metadata and working-tree state in report.

### B — Static integrity and contracts
- [x] Python, TOML, JSON, Markdown-link, and fence checks.
- [x] Workflow local-action/script path checks.
- [x] Flag exact repeated one-line workflow commands as low-confidence overlap hints.
- [x] Basic CLI documentation vocabulary check.
- [ ] Add a full YAML parser/validator with an explicit pinned dependency.
- [ ] Validate One snippets against the normative parser or one periksa without executing them.
- [x] Add contract-specific checks for package/build artifacts, diagnostics identity/determinism, grammar ambiguity rejection, exact CLI recognition, and capability-boundary invariants.
- [ ] Add workflow overlap analysis with evidence that distinguishes duplication from intentional parallel testing.

### C — Correlation and triage
- [x] Stable IDs, severity, confidence, evidence, and next-step fields.
- [x] Branch-scoped cached baseline comparison with new/recurring/reopened/resolved/not-checked states and separate triage-priority inputs.
- [x] Detector coverage only counts when relevant input was actually present; absent CI evidence remains not-checked.
- [x] Persist a prior JSON report in a branch-scoped Actions cache and compare it on later runs; classify new, recurring, resolved, reopened, and not-checked findings.
- [x] Correlate failed runs with absent job metadata, failed jobs with no visible steps/runner, failed steps needing log review, and artifact absence from exact-SHA GitHub API metadata without guessing a root cause.
- [ ] Retrieve and securely retain failed-step logs where permissions and retention allow; currently the collector stores links and metadata only.
- [ ] Version and expose priority inputs separately from the fixed acceptance ledger.
- [ ] Measure false positives and detector coverage.

### D — Guarded repair
- [x] Dry-run proposal and one narrow allowlisted Markdown repair.
- [x] Exact-path allowlist, repository-relative path validation, and isolated-branch/clean-tree guards.
- [x] Regression fixtures prove the repair refuses missing path authorization, traversal, non-Markdown targets, and non-isolated branches.
- [x] Dry-run output includes a unified-diff preview for each selected repair.
- [ ] Add reviewed allowlist entries only when a repair is deterministic and reversible.
- [ ] Keep workflow, contract, security, provenance, and release changes human-reviewed.

### E — Continuous evolution
- [x] Version the report schema.
- [ ] Add a fixture for every shipped detector and regression-test every confirmed incident.
- [x] Add a weekly scheduled scan after the event-driven workflow passed the independent Linux/Windows scanner, build, and normative fixture checks.
- [x] Record schema compatibility and detector/suppression governance in `docs/verification/health-detectors.md`.
- [ ] Track repair success rate and blind spots quantitatively.

## Definition of done

This scanner is not yet a complete autonomous repair system. It becomes production-ready only after the remaining checklist items are implemented, tests run on the exact commit, findings are reviewed, and CI artifacts are retrievable. The current workflow tests the scanner on Ubuntu and Windows, compares with a branch-scoped cached baseline, runs weekly, and uploads its reports; this does not substitute for the existing One language, conformance, E1/E2/E6, or protected production provenance gates.


## One source fixture validation

The CI workflow builds the CLI once and calls the non-executing validation command for the normative v0.1 accepted and rejected conformance fixtures:

~~~sh
cargo build --locked -p one-cli
python tools/one_health.py --check-one-source --json-out one-health-report.json
~~~

This invokes only `one periksa`; it never invokes `one jalankan`. The experimental `tests/human-vnext` fixtures and `examples/human-vnext` programs are deliberately excluded because they have a separate, explicitly experimental grammar contract. A fixture whose actual parser acceptance differs from its expected valid/invalid class is reported as a high-severity contract mismatch.

## Detector quality and repair outcomes

Use the manual quality ledger when reviewers confirm a finding was correct, a false positive, or a missed issue, or when a guarded repair has been verified:

~~~sh
cp docs/verification/health-quality-ledger.example.json /tmp/health-quality.json
python tools/health_quality.py /tmp/health-quality.json --json-out /tmp/health-quality-report.json --markdown-out /tmp/health-quality-report.md
~~~

The ledger requires review evidence and unique IDs. It reports observed false-positive/miss shares and the success rate of verified terminal repairs. These metrics describe only manually reviewed samples and are not statistically representative accuracy guarantees. See [the quality ledger guide](health-quality-ledger.md).

## Historical comparison and triage priority

Use a prior JSON report as an explicit baseline:

~~~sh
python tools/one_health.py --baseline previous-report.json --json-out current-report.json --markdown-out current-report.md
~~~

Current findings are labeled `new`, `recurring`, or `reopened`; findings absent from the current completed scan are listed separately as `resolved` with a review caveat. The tool does not silently edit the fixed-weight One acceptance ledger. A triage priority score is provided alongside its inputs (impact, confidence, recurrence, and acceptance-blocker factor) so maintainers can review why a finding was ranked. It is an operational triage aid, not an acceptance score or proof of resolution.

Baseline comparison is currently opt-in and file-based; persistent cross-run history and automatic GitHub job/log ingestion remain future work.
