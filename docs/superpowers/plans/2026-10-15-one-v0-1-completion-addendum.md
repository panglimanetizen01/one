# One v0.1 Completion Addendum

This addendum records the execution state of `docs/superpowers/plans/2026-10-15-one-v0-1-implementation.md` without rewriting its original task history.

## Task ledger

| Task | State | Canonical result |
|---|---|---|
| 1. Lexical/grammar contract | COMPLETE | lexical + concrete grammar specs + normative grammar example fixture |
| 2. Workspace/CI | COMPLETE | Cargo workspace + Linux/Windows workflow + contract-artifact gate |
| 3. Lexer | IMPLEMENTED | lexer/token modules + tests |
| 4. AST/parser | IMPLEMENTED | AST/parser modules + tests |
| 5. Names/scopes/types | IMPLEMENTED | semantic/value/type modules + tests |
| 6. Semantic IR | IMPLEMENTED | IR module + lowering tests |
| 7. Reference interpreter | IMPLEMENTED | runtime/interpreter + execution tests |
| 8. Diagnostics | IMPLEMENTED | structured diagnostic model + tests |
| 9. Capability/platform boundary | IMPLEMENTED | capability/platform modules + security tests |
| 10. CLI | IMPLEMENTED | conversational commands plus technical aliases |
| 11. Standard library/project support | IMPLEMENTED | stdlib + project scaffolding |
| 12. Process interoperability | IMPLEMENTED | explicit process boundary + tests |
| 13. Conformance/golden suite | COMPLETE AS ARTIFACTS | fixtures, runner, golden outputs, contract docs |
| 14. Documentation/security/release contract | AUDITED | contract closure checker strengthened; stale verification claims corrected |
| 15. Network native runtime slices | VERIFIED | DNS/TCP/UDP/TLS/HTTP/1.1/HTTP/2/QUIC/TFTP/NTP/FTP/SMTP/POP3/IMAP have executable runtime evidence |
| 16. Current full release verification | VERIFIED | canonical SHA `38dc98fcee49fa04c589d5a3c825b0d4006d6d61` passed independent Ubuntu/Windows run `38848143359` |

## Artifact closure

The normative Markdown contracts and their repository derivatives are indexed by `docs/README.md` and checked by `tools/check_contract.py`. The checker validates every concrete artifact referenced by the documentation index rather than relying only on a manually maintained subset.

The derivative set includes:

- formal foundation/specification and dependency-ordered plans;
- lexical, grammar, AST, localization, diagnostics, runtime, capability, module, package/build, standard-library, interop, CLI, and network contracts;
- conformance/release/verification documentation;
- valid/invalid One fixtures and golden outputs;
- executable conformance and CLI tests;
- network runtime acceptance tests;
- static contract-artifact consistency checker;
- independent Linux and Windows verification scripts;
- CI/release workflows;
- recorded independent verification evidence.

## Rulings

- **Ruling:** Do not purchase a GitHub upgrade solely to obtain hosted CI execution. Use the independent verification path when private hosted runners are unavailable.
- **Ruling:** The repository root remains a virtual Cargo workspace, so the executable conformance integration test lives under `crates/one-core/tests/conformance.rs`.
- **Ruling:** v0.1 does not silently accept multi-word user identifiers or the conversational `selama masih ada ...` loop form; both remain governed by the documented grammar boundary.
- **Ruling:** `one build` is an IR/debug artifact generator, not a native compiler, until a native backend contract and verification exist.
- **Ruling:** Protocol-registry status is evidence-driven. Inventory membership alone cannot advance a protocol to `Verified` or `Supported`.
- **Ruling:** FTP/SMTP/POP3/IMAP command fields reject CR/LF injection, and bounded framing/body limits are enforced before unsafe allocation or network side effects where applicable.

## Current release gate

Source, normative documentation, fixtures, executable tests, verification tooling, and network runtime slices are present. The current canonical release gate is **VERIFIED** for exact SHA `38dc98fcee49fa04c589d5a3c825b0d4006d6d61` by independent run `38848143359` on Ubuntu and Windows Server 2025. Any later canonical commit requires a new exact-SHA matrix.

## Release sprint overlay — 2026-11-20

This is a non-destructive execution overlay. It does not rewrite historical tasks, fixed acceptance criteria, ledger weights, or Definition of Done.

- The single prioritized work queue is [ONE_RELEASE_BLOCKER_REGISTER.md](../../verification/ONE_RELEASE_BLOCKER_REGISTER.md).
- The copy/paste entry point for new sessions is [MASTER_CONTINUATION_PROMPT.md](../../verification/MASTER_CONTINUATION_PROMPT.md).
- Execute in order: diagnose CircleCI pre-execution termination; close Issue #104 against unchanged criteria; reconcile remaining formal v0.1 gates; freeze one exact candidate SHA and run the integrated matrix; synchronize affected existing docs; prepare release artifacts only after approval.
- A job with `started_at=null` is not evidence of a test timeout or test failure. Current provider root cause remains UNKNOWN; see the blocker register and live checkpoint.
- This overlay changes prioritization only. It does not change task weights, acceptance rules, issue state, or release approval.

## Implementation-first release sprint override — 2026-11-20

This is a non-destructive execution-priority update. It does not modify the task ledger, fixed acceptance criteria, evidence weights, or DoD.

- Complete the real in-scope product implementation before initiating another broad verification cycle. Add/update regression tests with the implementation, but defer the integrated platform/security/release matrix until implementation scope is complete.
- Issue #104 is the current P0 production implementation blocker. The active candidate is PR #113; its latest source changes bind filesystem authority to host `cwd` and open the Linux root without following a final symlink. The candidate remains unbuilt/untested and unaccepted.
- B-001 classifier correction has exact-SHA CircleCI workflow/job success on `979973cd6fb6f76802e13dbbbe21530d68b4cf53`; preserve its historical root-cause uncertainty, but do not repeat the investigation without a new relevant failure.
- Preserve fail-closed behavior for Linux nested writes until a race-resistant parent-movement solution exists. Do not weaken the filesystem boundary to claim feature completeness.
- After Issue #104 implementation, continue any remaining v0.1 implementation gaps in the existing plan and release contract. Then reconcile the fixed ledger and run one integrated final matrix on a frozen exact SHA.

## Issue #104 implementation continuation — Android + macOS adapters — 2026-11-20

- Latest code-bearing candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814`; current PR #113 HEAD before this documentation update: `0bed293163aee8d398e1894811ef1ece17b78ff5`. PR remains OPEN / DRAFT / UNMERGED; no source build/test/device evidence is claimed.
- Added Android direct-child filesystem support using retained root handles, no-follow opens, mandatory `statx(STATX_MNT_ID)` checks, hard-link rejection, and atomic temp-file replacement. Android nested paths remain fail-closed.
- Added macOS direct-child filesystem support using retained root handles, no-follow opens, device-identity checks, hard-link rejection, and atomic temp-file replacement. macOS nested paths remain fail-closed.
- Added three target-gated regression tests per platform. They are implementation coverage only until compiled and executed on the respective targets.
- Linux nested writes remain fail-closed because the current safe parent-movement solution depends on privileged detached mount operations. Windows native handle-relative backend remains as previously implemented.
- These changes advance platform implementation but do NOT close Issue #104. Remaining work includes adversarial review of parent/root movement, platform-specific execution evidence, and any acceptance gaps in the fixed Issue #104 contract. Do not run the broad release matrix until implementation scope is complete.
