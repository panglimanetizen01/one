# CircleCI Native Toolchain Remediation — Active Checkpoint

**Status:** IN PROGRESS — latest Windows MinGW GCC wrapper is implemented but not yet verified by a fresh CircleCI run.

## Scope and guardrails

- Repository: `panglimanetizen01/one`
- Branch: `fix/windows-temp-path-canonicalization`
- Existing PR: #94; keep open/draft/unmerged.
- Primary CI: CircleCI.
- Environment: user operates from Android/Termux. The official CircleCI CLI v1.14.0 was checksum-verified but crashes under Termux with `SIGSYS` in `faccessat2`; use the existing shell-local `capi` API helper instead.
- Never ask for the API token repeatedly, ask the user to paste it into chat, print it, or commit it. The helper exists only in the current Termux shell.
- No main edits, merge, release, protected E2 dispatch, secrets/trust-store mutation, or branch-protection changes are authorized.
- Never disable native target tests or weaken acceptance to manufacture a green build.

## Confirmed pipeline history

| Pipeline | Source revision | Job / first failure | Evidence and classification |
|---|---|---|---|
| #22 | `af33fccfa947bd88737920ca287201a013c6dc29` | Full Rust workspace tests; Windows compiler not found and Android compile used host headers | Native cross-toolchains were not provisioned correctly. |
| #23 | heredoc config attempt | Workflow was not created | CircleCI config parser interpreted shell heredoc syntax; configuration defect. |
| #24 | config with printf wrapper | Job #9, prerequisite/native toolchain install | MinGW headers were present, but Clang target/declspec mode was incompatible: “Only Win32 target is supported” and declspec diagnostics. |
| #25 | `b0dd4ad0808e561b4046ae137ecf8579b9302451` | Job #10, ID `311ef77e-f29d-4f04-93f6-24f9a9fe651d` | Windows linker failed with `cannot find -lgcc` and `cannot find -lgcc_eh`. Root cause: Clang driver was paired with MinGW GCC runtime/linker libraries incorrectly. Not a Rust source-test failure. |

## Implemented remediation

- Commit `be52f5bebf73a78e21e11c7387da0cc97ee507bc`: install Windows MinGW and Android NDK r30; pin NDK archive SHA-1; configure API 35 AArch64 compiler and preflight checks.
- Commit `9c5ccdc18549a79e89a81761387836d256e4c836`: remove shell heredoc from CircleCI config and generate wrapper with printf.
- Commit `b0dd4ad0808e561b4046ae137ecf8579b9302451`: add explicit Windows Clang target and declspec mode; Pipeline #25 showed this still failed at GCC runtime linking.
- Commit `3d9d5a37bc37517a5f0a800690d2fbc57fc05400`: change Windows wrapper to invoke `/usr/bin/x86_64-w64-mingw32-gcc`, stripping the backend's Clang-specific target argument first. This pairs the MinGW compiler driver with its runtime/linker search behavior.

## Documentation synchronization

The 2026-11-10 session updated these repository documents with the active checkpoint and no-repeat rules:
- `AGENTS.md`
- `docs/DEVELOPMENT_PROTOCOL.md`
- `docs/DECISIONS.md` (ADR-0018)
- `docs/verification/one-completion-ledger.md`
- `docs/verification/DEFINITION_OF_DONE.md`
- `docs/verification/one-milestone-100-session-checkpoint.md`
- `docs/verification/README.md`

These documentation commits are new exact source revisions. The latest live branch HEAD must be read from GitHub before any pipeline or verifier action; do not assume the last listed commit remains HEAD.

## Next actions — ordered, do not skip

1. Resolve current branch HEAD and inspect the CircleCI pipeline triggered by the latest commit (the configuration commit or documentation tip). If no pipeline exists, determine why before manually triggering one.
2. Inspect workflow status, job status, first failed step, full log, and artifacts. Record exact pipeline ID, workflow ID, job number/ID, source SHA, and checkout SHA.
3. Confirm the Windows preflight produces PE32+ x86_64 and Android preflight produces ELF AArch64. If Windows passes, proceed to Android evidence and then the full Rust workspace test step.
4. If the first failing step changes, diagnose that new root cause; patch only the minimal responsible config/code and retain the old failure record as historical.
5. Continue through every subsequent CI step: contract/docs validation, health scanner tests, locked build, health/normative checks, warnings-denied Clippy, workspace tests, deterministic artifacts, vertical slices, and stored reports.
6. Only when the workflow is terminal and every required step is successful may this CircleCI job be recorded PASS for its exact SHA. This does not itself promote E1–E6 ledger rows or authorize release.
7. Reconcile the independent verifier's canonical pin and evidence index only after the intended source candidate is stable. Run a fresh exact-SHA matrix and ledger gate after required evidence is terminal.

## No-repeat ledger

- Pipeline #22 diagnosis: COMPLETE for its observed missing Windows compiler / Android sysroot setup.
- Pipeline #23 parser diagnosis: COMPLETE.
- Pipeline #24 Windows target/declspec diagnosis: COMPLETE.
- Pipeline #25 Windows linker/runtime mismatch diagnosis: COMPLETE.
- MinGW GCC wrapper implementation: COMPLETE at commit `3d9d5a37bc37517a5f0a800690d2fbc57fc05400`; execution verification: NOT STARTED / PENDING fresh pipeline.
- Android NDK r30 API 35 setup: IMPLEMENTED; execution verification on the latest config revision: NOT YET ESTABLISHED.
- Full Rust workspace and downstream CI steps on the latest revision: NOT RUN / NOT ESTABLISHED.
- Independent verifier pin reconciliation for the latest candidate: NOT DONE.

A scope marked complete above must not be repeated on the same unchanged revision unless new evidence invalidates it. A source/documentation commit invalidates prior exact-SHA PASS claims for the new HEAD, but it does not erase historical evidence.
