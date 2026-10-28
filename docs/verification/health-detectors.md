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
| `markdown-links` | Broken relative links and links escaping repository root | Markdown extensions, reference-style links, and generated routes are not fully parsed |
| `markdown-structure` | Fenced-code closure | Not a complete CommonMark validator |
| `path-integrity` | Symlinks resolving outside repository | Does not validate every runtime path or filesystem race |
| `file-integrity` | UTF-8 readability of scanned text files | Does not prove binary/artifact integrity |
| `workflow-integrity` | Duplicate workflow names, missing local actions and targeted script paths | Not a full GitHub Actions YAML/schema validator |
| `workflow-overlap` | Identical single-line `run:` commands in multiple workflow files | Low-confidence review hint; intentional duplication is common |
| `evidence-freshness` | Declared source SHA versus checkout SHA in JSON evidence | Only applies when both fields are present |
| `contract-drift` | Required core CLI vocabulary plus high-value invariants in normative grammar, CLI, diagnostics, package/build, and platform contracts | Text presence does not prove implementation; a contract refactor may require updating detector fixtures |
| `ci-evidence` | Exact-SHA Actions run/job/step metadata plus saved-JSON classification for failed jobs with absent steps and failed steps needing log review | Does not download or interpret live logs; API outages remain blocked/partial, not source failures |
| `artifact-integrity` | Expired CI artifacts, repeated artifact names within one run, and unavailable artifact digests in saved collector evidence | Does not download artifact contents or prove their bytes match a digest; missing digest is informational, not a pass |
| `one-source-contract` | Expected accept/reject results for normative v0.1 fixtures via `one periksa` | Runs only when explicitly enabled and CLI is built; never executes source |

## Report schema compatibility

- Current schema: `one-health-report/1`; scanner version is reported separately.
- Adding optional fields without changing existing field meaning is backward-compatible within schema version 1.
- Removing fields, changing field types, changing finding identity semantics, or changing baseline comparison meanings requires a new schema version and migration notes.
- Baselines with unknown schema versions are rejected closed; never silently compare incompatible reports.
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
