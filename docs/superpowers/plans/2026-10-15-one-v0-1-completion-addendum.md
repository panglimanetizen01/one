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


## CI provider migration addendum — 2026-11-13

The original task ledger records the historical GitHub Actions CI implementation. A later explicit project directive selects CircleCI as the canonical CI execution provider without changing the product scope or reopening the completed standalone `ci-local` extraction.

- Added `.circleci/config.yml` with default Linux/Windows product checks, opt-in release checks, and opt-in Windows native verification.
- Added `docs/CI.md` and updated the contributor guide, root README, documentation map, decision record (ADR-0017), implementation status, evidence matrix, release contract, and verification protocol.
- Updated `tools/check_contract.py` so the contract-artifact gate requires the CircleCI config rather than requiring GitHub Actions workflow files as permanent artifacts. It also checks for required job and product-gate markers.
- Forensic tree inspection confirmed that canonical `main` has no committed `Cargo.lock`; the CircleCI cache key was corrected to hash existing workspace manifests, and `--locked` was removed from the native verification commands. The contract checker now guards against reintroducing `--locked` before a lockfile exists.
- Legacy GitHub Actions workflows are intentionally retained unchanged until CircleCI demonstrates equivalent green checks on the same exact SHA.
- CircleCI VCS connectivity is evidenced by actual Linux job creation, but the first two observed Linux jobs failed and their logs are not accessible in the current integration. The root cause remains UNKNOWN; no CircleCI PASS is claimed. The Windows job failed on SHA `9776d7bdab72397cb533b052f7d5dfa2c0bad106` (failure-report pipeline `f84b18b4-80e2-4c21-8a04-ff36deacba81`); its root cause remains unknown because logs are inaccessible. The latest Linux/Windows retry is pending on SHA `12d4006ecc239e9ae3fb6b3fc191a1d50dd17317` under workflow `5f028c06-b3ff-4caf-9f96-b9b422640152`.


### Windows toolchain provisioning correction — 2026-11-13

CircleCI's published Windows Server 2025 image inventory does not list Rust/Cargo. The Windows jobs now install Rust stable through rustup and explicitly add the user Cargo bin directory to PATH in every Cargo-bearing PowerShell step. This addresses a guaranteed missing prerequisite; the earlier failed job's exact root cause remains unknown without its log. The next CircleCI run must prove the correction.


### Dual-platform CircleCI failure — 2026-11-13

The corrected config ran on SHA `e44d16b2b5f4ef50c33674ccf711bd11b410effa`; both Linux and Windows jobs failed under workflow `e5a0662d-023a-4e3d-9174-62a7aa55329a`, failure-report pipeline `ac6f1f05-1bbf-4d26-80ea-bf498ba29af3`. The exact cause is unknown without logs. No speculative config changes are authorized by this evidence; retrieve the failure report and fix the specific failed step.
