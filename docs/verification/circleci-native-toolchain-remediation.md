# CircleCI Native Toolchain Remediation — Active Checkpoint

**Status:** CircleCI Linux verification PASS on exact SHA `e19d0129c6a3169a36e18a9ce7cd13f4a677d348`; full CircleCI coverage parity remains NOT VERIFIED.

## Scope and guardrails

- Repository: `panglimanetizen01/one`
- Branch: `fix/windows-temp-path-canonicalization`
- Existing PR: #94; keep open/draft/unmerged.
- Primary CI: CircleCI.
- Environment: user operates from Android/Termux. Termux is strictly read-only post-CircleCI verification: it may retrieve logs/artifacts through authenticated API calls and validate hashes/SHAs, but must never edit source/docs/config, repair code, commit, push, or administer CI/repository settings. The `capi` helper is read-only evidence retrieval only.
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

1. Resolve the live PR #94 branch HEAD from GitHub. This documentation update creates a new exact source SHA; do not use the predecessor's CircleCI PASS as proof for the new HEAD.
2. Inspect the CircleCI pipeline triggered by the new GitHub commit: workflow, job, every step, logs, artifacts, and exact checkout SHA. Fix the first deterministic failure on GitHub only; then validate the new SHA.
3. Maintain a coverage-parity matrix for the former source Actions scopes. The current CircleCI config contains only one Linux `linux-verify` job. Its Windows PE32+ and Android AArch64 compiler probes are not native Windows Server 2025 or Android API 35 runtime evidence.
4. Expand CircleCI coverage on GitHub in a staged, evidence-driven way until required OS/architecture/runtime/test/artifact/security/release scopes have exact-SHA jobs or an explicitly accepted independent-verifier alternative. Preserve existing Actions files until parity is proven and removal is explicitly authorized.
5. Reconcile the independent verifier canonical pin and evidence index only after the intended source candidate is stable. Run a fresh exact-SHA matrix and ledger gate after required evidence is terminal.
6. Keep merge, release, protected E2 dispatch, secret/trust-store mutation, branch-protection changes, and direct `main` edits outside scope unless explicitly authorized.

## No-repeat ledger

- Pipeline #22 diagnosis: COMPLETE for its observed missing Windows compiler / Android sysroot setup.
- Pipeline #23 parser diagnosis: COMPLETE.
- Pipeline #24 Windows target/declspec diagnosis: COMPLETE.
- Pipeline #25 Windows linker/runtime mismatch diagnosis: COMPLETE.
- MinGW GCC wrapper implementation: COMPLETE at commit `3d9d5a37bc37517a5f0a800690d2fbc57fc05400`; compiler-probe and Linux verification: PASS on exact source SHA `e19d012...` (CircleCI pipeline #34, job #19); native Windows/Android runtime: NOT VERIFIED.
- Android NDK r30 API 35 setup: IMPLEMENTED; execution verification on the latest config revision: NOT YET ESTABLISHED.
- Full Rust workspace and downstream CI steps on the latest revision: NOT RUN / NOT ESTABLISHED.
- Independent verifier pin reconciliation for the latest candidate: NOT DONE.

A scope marked complete above must not be repeated on the same unchanged revision unless new evidence invalidates it. A source/documentation commit invalidates prior exact-SHA PASS claims for the new HEAD, but it does not erase historical evidence.


## Exact CircleCI evidence — 2026-11-11

- PR #94 head at the time of the run: `e19d0129c6a3169a36e18a9ce7cd13f4a677d348`.
- Pipeline #34; workflow ID `9bf04f99-94b0-4bac-bd3f-749c8fb857d3`; workflow `one-linux-verification`; job #19 `linux-verify`; job ID `1ef51523-a767-4ccf-bae8-9f806dc13ca7`; conclusion SUCCESS.
- V1.1 job record reports `vcs_revision=e19d0129c6a3169a36e18a9ce7cd13f4a677d348` and `outcome=success`. Checkout log also recorded the expected commit.
- Successful steps included prerequisite/native toolchain setup, contract and user-doc validation, scanner regression tests, locked workspace build, repository health scan, warnings-denied Clippy, full workspace tests, deterministic package artifact test, database/HTTP vertical slices, and artifact uploads.
- Health JSON and Markdown artifacts downloaded with authenticated CircleCI API access. JSON SHA-256: `76b77bcc94e066ad8f2b270318ab5a12db76c73b3ea2106d4cf3bcd1c5664359`; Markdown SHA-256: `a69a2db6806a1bd4347211db5b31097154023c19fb3df6d955e3cea8724e1baf`.
- Health report: schema `one-health-report/2`, scanner `0.2.0`, expected SHA equals HEAD SHA, clean working tree, 27 low-confidence informational workflow-overlap findings, zero critical/high/medium/low.
- The health command uses `--quiet`, so its captured stdout event was empty; the downloaded JSON/Markdown artifacts are the report evidence.
- The job is Linux. It proves compiler probes (Windows PE32+ and Android AArch64) and the listed Linux checks, not native Windows or Android runtime acceptance. Full CircleCI parity remains NOT VERIFIED.
- Overall project gate remains NOT PASSED. Independent verifier pin reconciliation and the missing platform coverage remain open.
