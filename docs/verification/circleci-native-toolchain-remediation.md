# CircleCI Native Toolchain Remediation — Active Checkpoint


## Active CI remediation checkpoint — 2026-11-11 (CircleCI exact-SHA evidence)

- Source repository `panglimanetizen01/one`; PR #94 branch `fix/windows-temp-path-canonicalization`, OPEN / DRAFT / UNMERGED; base `milestone/100-ecosystem-20261023` at `d75b2846465ba53281198693750f2e6ba39e2e33`.
- Last inspected source revision with completed CircleCI evidence: `912c10fe86cee10294a5e09b96a99f00477d788a`. Workflow `f392e32f-564a-4756-a97c-87db69376cd0`, job #20 `linux-verify`, ID `a28b1024-b9f1-424c-b92f-05f275366f98`, SUCCESS. It checked out the exact SHA and reported success for prerequisite/native compiler setup, contract/user-doc checks, scanner regression tests, locked workspace/CLI build, health/normative source scan, warnings-denied Clippy, full workspace tests, deterministic package artifact test, database/HTTP vertical slices, and artifact upload.
- CircleCI #20 artifacts API listed `one-health-report.json` and `one-health-report.md`, but both metadata URLs returned HTTP 404. Payloads and hashes remain NOT VERIFIED; the 404 does not prove deletion/expiry.
- Subsequent source commit `15781298a11c26580545b90c398a98d79b572a9b` updates docs and adds a literal escaped-newline Markdown detector/regression tests. CircleCI status was PENDING at last GitHub read; it is not yet verified. The following docs-only follow-up commit also invalidates that pending result; always resolve the latest live branch HEAD before reading CI.
- Legacy source Actions runs on SHA `912c10f...` and `1578129...` completed FAILURE with `steps: null` and unavailable logs (`BlobNotFound`). Root cause UNKNOWN; do not call these source-test failures or PASS. CircleCI remains canonical; preserve Actions files while coverage parity is reviewed.
- CircleCI config still defines one Linux job. Windows PE32+ and Android AArch64 compiler probes do not prove Windows Server 2025 runtime or Android API 35 AArch64 ELF runtime. Both remain NOT VERIFIED for the active source candidate until dedicated exact-SHA runtime evidence exists.
- Independent verifier pin remains stale at `af33fccfa947bd88737920ca287201a013c6dc29`; verifier README has separately named `f21e8d...`. Synchronize only after source CircleCI proves the final source SHA; keep `LOCAL_CI_CANONICAL_COMMIT.txt` separate for its Local CI/PR #92 track.
- Acceptance remains DoD 3 checked / 10 unchecked / 13 total; 92/100 implementation provisional; 60/100 historical verified subtotal; final score UNKNOWN / NOT RECONCILED; gate NOT PASSED. No weight or checkbox changes without direct exact-SHA evidence.
- No-repeat: reuse CircleCI #20 only for SHA `912c10f...`; do not rerun it unchanged. Validate the final new source SHA once, inspect steps/logs/artifacts, then advance the verifier pin and run the required matrix. Do not dispatch protected E2 provenance or release without its existing authorization.
- Termux is read-only verification only. No direct `main` edit, merge, release, secrets/trust-store mutation, branch-protection change, or source Actions removal.

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
