# One Repository Health Detector Governance

This file governs the repository-health scanner independently of One's language grammar and the fixed-weight completion ledger.

## Detector lifecycle

Every detector must have:
1. A stable `detector_id` and a rule ID that describes one observable condition.
2. At least one positive fixture and, where applicable, one negative fixture proving intentional non-detection.
3. Evidence that can be reproduced from the checked-out repository or attached CI metadata.
4. A severity and confidence calibrated to what the detector actually proves.
5. A recommendation that does not silently rewrite source, contracts, security policy, or release state.
6. A limitation entry whenever the detector is partial, heuristic, or environment-dependent.

A new detector starts as informational or medium severity unless it can demonstrate a deterministic, high-impact failure. Increasing severity requires a regression fixture and a short rationale in this file or the pull request.

## Current detector catalog

| Detector | What it checks | Known limitation |
|---|---|---|
| `revision-integrity` | Expected SHA shape and checked-out HEAD | Does not establish who authorized the revision |
| `syntax` | Python AST, TOML, JSON parsing | Does not validate all language/tool semantics |
| `markdown-links` | Broken relative paths, links escaping repository root, and stale same-page/cross-file Markdown heading or explicit HTML anchors | Markdown extensions, reference-style links, and generated routes are not fully parsed; heading slugging is a conservative approximation |
| `markdown-structure` | Fenced-code closure | Not a complete CommonMark validator |
| `path-integrity` | File and directory symlinks resolving outside repository; directory symlinks are surfaced but never traversed | Does not validate every runtime path or filesystem race; behavior depends on platform permissions and filesystem semantics |
| `file-integrity` | UTF-8 readability of scanned text files | Does not prove binary/artifact integrity |
| `workflow-integrity` | Duplicate CI config names, missing local actions and targeted script paths in GitHub Actions YAML and `.circleci/config.yml` | Structural heuristics only; not a full GitHub Actions or CircleCI schema validator |
| `workflow-overlap` | Identical single-line `run:` commands in multiple workflow files | Low-confidence review hint; intentional duplication is common |
| `evidence-freshness` | Declared source SHA versus checkout SHA in JSON evidence | Only applies when both fields are present |
| `contract-drift` | Required core CLI vocabulary plus high-value invariants in normative grammar, CLI, diagnostics, package/build, and platform contracts | Text presence does not prove implementation; a contract refactor may require updating detector fixtures |
| `ci-evidence` | Exact-SHA Actions run/job/step/artifact metadata plus surfaced findings for failed runs without job metadata, failed jobs without visible steps, and failed steps needing log review | Does not download or interpret live logs; API outages remain blocked/partial, not source failures |
| `artifact-integrity` | Expired CI artifacts, repeated artifact names within one run, missing artifact digests, and no-artifact observations in completed runs | Does not download artifact contents or prove their bytes match a digest; missing digest is informational, not a pass |
| `security-secrets` | Known high-signal private-key, GitHub/AWS/Slack, Google API, Stripe live-secret, npm/PyPI token patterns, and likely hardcoded credentials | Heuristic coverage only; secret values are redacted from reports; unknown formats, encoded secrets, and false positives remain possible |
| `security-code` | Python dynamic execution and shell usage; AST correlation for decoded/deserialized payload execution, downloaded-data execution, and credential/environment-to-network egress | Correlation is limited to expression trees and simple local assignments, not general taint analysis; a narrow `api.github.com` Authorization-client shape is excluded only when structurally rooted in that constant; other destinations remain findings; code is never executed |
| `security-javascript` | JavaScript/TypeScript obfuscated dynamic execution, network-plus-execution correlation, sensitive-source/network-sink correlation, and Node `child_process.exec/execSync` use | Regex-based heuristics; no JavaScript parser, complete data-flow analysis, package lifecycle analysis, or runtime sandbox; findings require review |
| `security-workflow` | Network-to-shell pipelines, broad write permissions, `pull_request_target`, and obviously mutable refs such as `main`/`master`/`latest` | Targeted checks only; not a full workflow security audit or YAML semantic analysis | 
| `security-supply-chain` | Non-commit-pinned GitHub Actions and Git dependency refs that name mutable branches | Tags can be intentionally used but are mutable; pinning does not independently establish upstream trust or provenance |
| `security-prompt-injection` | Agent-instruction surfaces that co-locate an instruction-override attempt with a request to reveal/send secrets | Only recognized agent-instruction paths/names and correlated phrases are checked; semantic obfuscation, encoded content, and other agent file conventions may evade detection; ordinary documentation is intentionally not treated as an agent prompt |
| `one-source-contract` | Expected accept/reject results for normative v0.1 fixtures via `one periksa` | Runs only when explicitly enabled and CLI is built; never executes source |

## Report schema compatibility

- Current schema: `one-health-report/1`; scanner version is reported separately.
- Scanner implementation version `0.1.2` includes JavaScript/TypeScript source traversal and plural-aware prompt-injection secret-action correlation; the report schema remains version 1 because field types and finding identity semantics are unchanged.
- Adding optional fields without changing existing field meaning is backward-compatible within schema version 1.
- Removing fields, changing field types, changing finding identity semantics, or changing baseline comparison meanings requires a new schema version and migration notes.
- Baselines with unknown schema versions, malformed finding records, missing/blank IDs, or duplicate finding IDs are rejected closed. Unreadable and invalid-UTF-8 baseline files return CLI exit code 2; never silently compare ambiguous or incompatible reports.
- Consumers must tolerate additional fields and must not interpret absence of a detector from `detectors_run` as a clean result.

## Suppression policy

There is no global suppression mechanism today. Do not add broad path or detector suppressions as a quick way to make CI green.

If suppression is introduced, every entry must include:
- exact rule ID and narrow path/pattern;
- reason and linked issue/PR;
- owner responsible for review;
- creation date and mandatory expiry date;
- a test proving the suppression does not hide a neighboring finding.

Expired suppressions must be reported as findings and must not be silently renewed. Security, revision identity, provenance, and release-gate findings must not be suppressible by repository-wide wildcard rules.

## Baselines and false-positive handling

Baseline comparison is a triage aid, not proof of repair and not a release/acceptance score. A missing finding is only marked resolved when the corresponding detector ran; otherwise it remains `not_checked`. Reviewers must verify detector coverage before closing a finding.

When a finding is a false positive:
1. Add a minimal regression fixture showing why the heuristic was wrong.
2. Prefer improving the detector or reducing confidence/severity over suppressing it.
3. Record the reason and changed behavior in the pull request.
4. Re-run all scanner tests and inspect the report on the exact resulting SHA.

Track detector regressions as defects in the scanner itself. Do not alter One grammar or normative fixtures merely to satisfy a health detector.

## Repair governance

Automated repairs must be deterministic, narrow, reversible, previewable, and independently testable. Repair code must refuse ambiguous paths, dirty checkouts, unexpected revisions, or unapproved branches. Each repair requires a dry-run proposal and a regression test that proves both the intended change and refusal conditions.

Repairs are never authorized to change normative grammar, accepted/rejected fixture classifications, fixed ledger weights, branch protection, workflow permissions, signing trust, provenance claims, or production release controls. Those changes require explicit human review and their own acceptance gates.

## Detector change log

- 2026-10-28 — Initial catalog and governance policy for schema v1, baseline triage, suppression expiry, and guarded repair.
- 2026-10-28 — Added scanner/runtime environment metadata, branch-scoped cached baseline comparison, and weekly scheduled repository-health scans. Independent scanner tests, CLI build, normative fixture validation, and cross-platform verification passed on source SHA `65d09e3f9ed8639ff0d22a948ada06c3def36c22`; subsequent source changes require a fresh exact-SHA run.
- 2026-10-28 — Added JavaScript/TypeScript file coverage and `security-javascript` rules for decoded dynamic execution, network-plus-execution correlation, environment/credential-to-egress correlation, and Node `child_process.exec/execSync`. Positive/negative regression fixtures were added; this is heuristic static analysis, not JavaScript parsing or a sandbox. Exact-SHA test execution is still required.


## Schema compatibility

- The scanner report uses schema version one-health-report/1. The detector_coverage object is an additive field; consumers should tolerate unknown fields and must not treat a missing field as a successful check.
- The CI collector uses one-ci-health-evidence/1. The correlations array is additive metadata; unknown or absent correlations are not evidence that a run is healthy.
- The reviewed quality ledger uses its own one-health-quality/1 schema and is never merged into the fixed-weight acceptance ledger.
- Any incompatible field rename, type change, or semantic change requires a schema version change, updated fixtures, and a migration note. Keep existing fields stable within a schema version.

## Detector quality and repair outcome measurement

Use [the health quality ledger guide](health-quality-ledger.md) and `tools/health_quality.py` to record reviewed true positives, false positives, confirmed misses, and verified repair outcomes. The tool reports observed sample shares and verified repair success rate; it does not infer correctness from missing data. A detector change should include positive/negative fixtures and a regression case for any confirmed incident. These metrics are operational quality evidence only and never modify the fixed-weight completion ledger or release gates.


## Security scanner regression contract (A–E)

- **A — Integrity:** exact checkout SHA, symlink/path boundaries, text readability, syntax, Markdown links, and CI/artifact metadata are separate findings. Missing metadata is blocked/unknown, never a pass.
- **B — Static security:** secret redaction, AST-correlated obfuscated execution, download-plus-execution, credential/environment-to-network egress, agent prompt-injection correlation, and workflow/dependency pinning use deterministic fixtures. The scanner does not execute repository code.
- **C — Correlation and triage:** multi-signal findings describe which indicators co-occur; they are leads for review, not proof of data flow, compromise, or root cause. Keep evidence values redacted.
- **D — Guarded repair:** repairs remain allowlisted, exact-path, clean-working-tree, approved-branch-only operations. Security findings never trigger automatic edits or credential rotation.
- **E — Quality corpus:** every detector change needs positive and negative regression cases. Human adjudication remains separate from synthetic fixtures; observed sample metrics are not statistically representative accuracy claims.

Known boundary: this remains a static repository scanner, not an OS sandbox, full taint engine, malware detonation service, complete YAML parser, dependency vulnerability database, or proof of secret non-exposure. Review findings safely and preserve the NOT-PASSED gate until exact-SHA execution evidence is available.


- 2026-10-28 — Exact-SHA regression evidence for source `8db21148436d313ebe148f8248267a75f7266ea0`: independent verifier run [40730508735](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40730508735) completed on Ubuntu and Windows. Each OS ran 59 scanner tests, 6 CI collector tests, 5 quality tests, built the ONE CLI, and self-scanned the checked-out ONE repository. Both reports matched `head_sha` and `expected_sha`; each reported 0 critical, 0 high, 11 medium, 27 informational findings, with 2 JSON artifacts ([Ubuntu](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40730508735), [Windows](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40730508735)). This is verified execution evidence for that SHA only. Any subsequent source commit makes it historical and requires revalidation before final-head acceptance.

## Python–Rust parity matrix (migration snapshot)

Reference revisions for this comparison: Python `tools/one_health.py` at PR #91 head `22b1c389491d132769eb4cd1e0abaa4bd42b753c`; native Rust `crates/one-cli/src/integrity.rs` at PR #89 head `242b88e4c8b6f2c2c2039f2bc95ebeab20ae96de`. This is a scope comparison, not a claim of semantic parity.

| Capability | Python reference scanner | Native Rust scanner | Parity status / required follow-up |
|---|---|---|---|
| Exact expected SHA / checkout HEAD | Yes; JSON/CI evidence also checks SHA freshness | Yes; `--expected-sha` check | Partial: align JSON evidence schema and rule IDs |
| File and directory symlink escapes | Yes; surfaces directory symlinks and does not traverse them | Yes; enumerates symlinks without following them | Partial: rule IDs differ (`symlink-escapes-repository` vs `symlink-escapes-root`); align output contract and regression fixtures |
| Unreadable / invalid UTF-8 text | Emits a finding for unreadable scanned text | Emits `text-not-utf8` for selected extensions; directory enumeration errors abort scan | Gap: match per-file error handling and document fatal root/enumeration errors |
| Markdown links and anchors | Relative paths, root escapes, heading/HTML anchors and stale cross-file fragments (heuristic) | Broken relative-link checks only; skips external and fragment-only links | Gap: root-escape and anchor checks absent; parser coverage differs |
| Markdown structure | Unclosed fenced-code detection | Unclosed fenced-code detection | Partial: confirm equivalent fence edge cases with shared fixtures |
| Workflow integrity | Duplicate names, missing local actions/script paths, overlap hints and metadata checks | Missing workflow name and targeted missing local actions | Gap: add duplicate-name, script-path and overlap checks or explicitly scope out |
| Secret redaction | Known provider patterns, heuristic hardcoded secrets; redacts evidence in JSON/Markdown | Private-key/token prefixes and line-level secret heuristics; redacts finding evidence | Partial: provider-pattern coverage differs; add shared positive/negative fixtures and assert whole-report redaction |
| Python AST / JS/TS correlation | Python AST and local source/sink heuristics; JS/TS regex heuristics | No equivalent AST/source-sink correlation in current Rust implementation | Gap: implement equivalent rules or explicitly document Rust as narrower |
| Prompt injection / agent exfiltration | Dedicated prompt-injection and secret-exfiltration correlation | Not present in current Rust security detector | Gap: add tests and detector or declare unsupported scope |
| Supply-chain / dependency and workflow security | Dependency/workflow pinning and related correlation checks | Basic download-and-execute and `write-all` checks | Gap: narrower coverage; do not claim parity |
| CI run/job/step/artifact evidence | Dedicated collector/correlation, baseline triage and artifact freshness/digest observations | No equivalent live CI metadata collection in scanner module | Gap: remains Python-only unless a separate native contract is added |
| Baseline / finding lifecycle | Validated baseline; new/recurring/reopened/resolved classification | Not present in current Rust scanner module | Gap: Python-only feature; specify whether this remains intentionally out of scope |
| JSON and Markdown reports / CLI exit contract | Structured report, Markdown rendering, severity-based exit codes | Native CLI integration exists; report contract requires separate verification | Pending: compare schema, redaction, ordering, exit codes and error behavior with shared fixtures |
| Safe repair | Guarded, allowlisted Markdown newline repair only | Read-only scanner | Intentional divergence: Rust must remain read-only unless a separately reviewed repair contract is approved |

### Acceptance gate

Do not describe the scanners as parity-equivalent yet. For each shared capability, run the same positive and negative fixture corpus against both implementations and compare normalized `rule_id`, path, severity, confidence, redacted evidence, exit code and JSON schema. Mark Python-only capabilities explicitly rather than silently treating missing Rust findings as passes. Python regression suites, normative self-scan and Linux Rust build/Clippy/workspace tests passed on the exact revisions listed above in the independent verifier. Windows Rust native tests and Android AArch64 artifact/runtime checks remain pending; do not call native validation complete until those checks finish.


## CI provider boundary — CircleCI migration

The canonical CI provider for the migration candidate is CircleCI. The repository-health scanner includes `.circleci/config.yml` in generic workflow-integrity checks for duplicate configuration names and missing referenced scripts. GitHub Actions-specific security checks remain applicable only to legacy `.github/workflows/` files. A missing GitHub Actions workflow is not a CircleCI failure, and CircleCI config scanning does not claim full CircleCI schema validation.

The historical `tools/collect_ci_health.py` collector reads GitHub Actions API metadata only. It is not a CircleCI collector and must never be used to claim CircleCI run/job status. CircleCI pipeline/job evidence is recorded separately against the exact checkout SHA.
