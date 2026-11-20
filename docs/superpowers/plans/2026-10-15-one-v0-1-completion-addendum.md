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

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution is now explicitly unavailable on Android and macOS until equivalent non-escapable containment exists; `ProcessAdapter` advertises only Linux and Windows.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.

## Android process containment extension — 2026-11-20

- Code candidate `58ae7831c7d17e60e53fa6a933dc2f170196c585` enables the same inherited Linux seccomp process-group filter on Android as on Linux and advertises `ProcessAdapter` on Android. It denies session/group changes and namespace escapes; filter setup failure is fail-closed.
- macOS process execution remains unavailable until equivalent containment exists. Windows Job Object behavior is unchanged.
- This Android extension is NOT BUILT OR TESTED. Android seccomp policy compatibility and the three adversarial process tests still require execution on an actual Android/Termux target. Issue #105 remains OPEN / NOT ACCEPTED.

## Android process containment regression coverage — 2026-11-20

- Latest code-bearing candidate: `418d9642778d975440b81df2bdb5e1bf9aa1d8a5` (NOT BUILT/TESTED).
- The inherited seccomp pre-exec filter is now installed on both Linux and Android. ProcessAdapter advertises Linux, Android, and Windows; macOS remains fail-closed. Android uses the same `setsid`/`setpgid`/namespace restrictions and fails closed if the child cannot install the filter.
- The three adversarial `setsid` regression tests and bounded-reader test are now target-gated for Linux and Android, so they can run in Termux as well as Linux CI. No Android/Termux execution evidence is claimed.
- Issue #105 remains OPEN / NOT ACCEPTED pending exact-SHA build and adversarial execution on Linux and a real Android/Termux target, plus Windows Job Object evidence reconciliation.

## IMPLEMENTATION CHECKPOINT — 2026-11-21 (supersedes older platform-status bullets)

Code-bearing source/test candidate: `4c46a41fe45baa7627557a2a25b9c71812840479`. PR #113 was OPEN / DRAFT / UNMERGED at this exact SHA before documentation synchronization. This candidate has NOT been built or tested; no previous exact-SHA result transfers.

### Issue #104 — filesystem implementation
- Linux: retained root handle, `openat2` no-follow/beneath/no-cross-device reads; atomic direct-child replacement; nested writes use the detached-mount + `mount_setattr(MOUNT_ATTR_NOSYMFOLLOW)` + `openat2` path and fail closed when the kernel/security profile cannot provide those primitives.
- Windows: existing native handle-relative/reparse-point backend and retained-parent rename protection remain unchanged.
- Android: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, mandatory `statx(STATX_MNT_ID)` equality, regular-file/single-link checks, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- macOS: direct-child reads and atomic replacement use retained root `dirfd`, `O_NOFOLLOW`, regular-file/single-link checks, `fstatfs` filesystem-ID plus mount-point comparison, exclusive temporary sibling, file sync, root-directory sync, and `renameat`. Nested paths remain fail-closed.
- Android/macOS hard-link side-effect regressions were added. Existing Android/macOS direct-child, symlink, and root-symlink regressions remain in source. These tests are NOT RUN.
- Issue #104 remains OPEN / NOT ACCEPTED until the fixed acceptance criteria have exact-SHA Unix and Windows execution evidence, including race/side-effect proof.

### Issue #105 — process containment implementation
- Linux and Android subprocess startup install an inherited seccomp filter before `exec`; it denies `setsid`, `setpgid`, `unshare`, `setns`, namespace-bearing `clone` flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` for compatible fallback. Filter setup failure is fail-closed.
- Windows Job Object containment is unchanged. macOS process execution remains fail-closed until equivalent non-escapable containment exists.
- Three adversarial `setsid` regressions cover normal completion, timeout, and output overflow on Linux/Android; they are NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED pending Linux + real Android/Termux execution and reconciliation of Windows Job Object evidence.

### Execution priority
- Continue production implementation against the existing v0.1 contract and release blockers. Add regression coverage with each code change, but do not launch broad matrix/audit cycles until the implementation scope is complete.
- Preserve B-001's prior exact-SHA success; do not repeat its investigation without a new relevant provider failure.
- Preserve `main`, protected verifier pins, fixed ledger weights, DoD, acceptance criteria, and release authorization boundaries.


## Implementation continuation refresh — 2026-11-21

Latest code-bearing candidate and PR #113 HEAD at this checkpoint: `fd1ccc5ca56f48a306ac8f8c551189f46b8fe44c` (OPEN / DRAFT / UNMERGED; NOT BUILT/TESTED). This supersedes the previous 2026-11-21 candidate pointer.

Issue #105 regression coverage now uses ignored helper tests in the same Rust test binary. The helper forks a descendant and calls `setsid(2)` directly, avoiding dependence on an external `setsid` utility on Android/Termux. The parent tests cover normal completion, timeout, and output overflow, record any escaped PID, and clean up the escaped group before reporting failure. The tests are NOT RUN.

Issue #104 implementation changes from the previous checkpoint remain: Linux guarded detached-mount nested writer; Android direct-child `statx(STATX_MNT_ID)` backend; macOS direct-child `fstatfs` filesystem-ID/mount-point backend; exclusive temp replacement with file and directory sync; hard-link regressions. Windows native backend remains unchanged. Nested Android/macOS operations fail closed.

Issues #104 and #105 remain OPEN / NOT ACCEPTED. No exact-SHA build/test or broad verification is claimed. Continue implementation-first and preserve all release/security gates.

## Process-group cleanup hardening — 2026-11-21

Latest code-bearing source/test candidate: `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` (NOT BUILT/TESTED). Linux/Android process supervision now observes direct-child exit using `waitid(P_PID, WEXITED | WNOHANG | WNOWAIT)` rather than reaping via `try_wait()` before process-group cleanup. The child remains a zombie (its PID/PGID cannot be recycled) while the wrapper sends SIGKILL to the invocation process group; only then does `Child::wait()` reap it. The redundant post-reap group kill was removed. All timeout, output-overflow, wait-error and normal-completion paths terminate the group before reaping.

Process escape regressions use in-binary ignored helpers with direct `fork` + `setsid(2)` syscalls, record any escaped PID and clean up before failing. Three paths: normal completion, timeout, output overflow. NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED.

## Implementation-first source correction — subprocess test-module duplicate helper — 2026-11-21

- Candidate source commit `0cceeb447fbe2ae59b8e3da78dee3ce8d9959830` removes a duplicate `cleanup_test_escaped_group` function definition from `crates/one-core/src/interop.rs` test module. Source readback confirms one definition remains.
- This addresses a source-visible Rust duplicate-definition compile blocker; it is NOT a compile result. No build/test/CI/verifier/platform matrix was run on this candidate.
- Release contract reconciliation commit `da5b007fe1dcfc7caa3c14c69b500866d5edbb75` corrects stale platform descriptions: filesystem adapters exist for Linux/Windows/Android/macOS (Android/macOS nested paths fail closed; Linux nested writes require detached-mount support); process adapters exist for Linux/Android/Windows (macOS fail-closed). These are implementation boundaries, not acceptance/support claims.
- Issue #104 and #105 remain OPEN / NOT ACCEPTED. Keep the fixed ledger, DoD, verifier pins, and release gates unchanged.
