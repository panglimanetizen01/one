# One v0.1 Release Contract

Status: **EXACT-SHA CROSS-PLATFORM VERIFIED BASELINE WITH EXPERIMENTAL ORCHESTRATION; RELEASE APPROVAL REMAINS A SEPARATE GATE**.

## Current forensic release gate — 2026-11-17

This checkpoint supersedes earlier historical candidate/head statements below where they conflict with live PR metadata. It does not authorize merge or release.

- Canonical `main`: `d35db95153f6ddd79885494c72f5aee16c899751`; repository visibility is PUBLIC. `main` still contains GitHub Actions workflows and has no `.circleci/config.yml`.
- PR #98–#101 each have exact-SHA GitHub Actions Ubuntu + Windows Server 2025 PASS on their own candidate SHAs: #98 `a83321bc41b3d0d28378841e29a6169abed8ac15` run #2018; #99 `8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9` run #2019; #100 `6375356c9c275ace5fd35cf4a81799a9546e1e1d` run #2020; #101 `cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f` run #2138. These fixes remain unmerged and their results do not combine into a single integrated source revision.
- PR #102 source `6c72d8e548bbbc28d85be39b8de427614989a038`: GitHub Actions run #2229 / `44086429416` PASS on Ubuntu and Windows Server 2025.
- PR #107 source `21e393157e52728d9510699ac6c9120987e3852c`: GitHub Actions run #2230 / `44088328030` PASS on Ubuntu and Windows Server 2025. It adds bounded Unix reader shutdown; it does not terminate escaped Unix descendants in production.
- PR #103 ledger source `041b359b8504551c2c1705093050d9c467368bac`: GitHub Actions run #2231 / `44089449162` PASS on Ubuntu and Windows Server 2025.
- PR #108 is the CircleCI migration candidate, based on PR #102 and carrying PR #107's verified bounded-reader implementation. It adds CircleCI config, updates contract checking/portable offline scripts for `--locked`, and removes the three repository GitHub Actions workflows on the candidate branch only. Its CircleCI schema/job execution has NOT been verified; no CircleCI status context is currently available.
- CircleCI contexts #669/#671/#673 are errors on branches without `.circleci/config.yml`; they are not source-test failures. Pipeline #643 on old PR #106 explicitly reported missing config. Do not generalize that exact error to other pipelines without their logs.
- Open security gates: Issue #104 (race-resistant filesystem root confinement), Issue #105 (strong Unix escaped-descendant containment), and Issue #82 (Android AArch64 runtime execution). Issue #81 remains open pending explicit reconciliation with the newer subprocess work.
- The Definition of Done remains 3 checked / 10 unchecked / 13 total. E6 remains **NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED**. No 100% completion, merge, or release claim is made.


## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Verified source revision and exact-SHA evidence boundary

The last code-bearing canonical source revision `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` passed independent exact-SHA verification run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451) on Ubuntu and Windows Server 2025.

The matrix passed contract-artifact checks, build, AST contract regression tests, Clippy, workspace tests, real-network runtime tests, and platform-appropriate offline verification. It also provisioned the Windows/Android cross-toolchains and Linux cross-compilers required by the native CLI integration coverage.

Canonical hosted `One CI` runs `39372356906` and `39381125667` failed before executable steps (`steps: []`). These are infrastructure/runner failures and do not establish a source failure; the independent exact-SHA matrix is the positive source-revision evidence.

The matrix verifies the recorded source revision on its Ubuntu and Windows hosts. Subsequent evidence-sync commits change documentation only, not source or tests. It does not automatically imply that every cross-compiled target artifact was executed on its own target architecture; target-specific execution claims remain bounded to their recorded evidence.

## Verified baseline

- DATA, ACTION, CONDITION, LOOP, FUNCTION, and EVENT core language constructs.
- Indonesian lexical and grammar contracts.
- Language-independent AST and Semantic IR.
- Reference interpreter and structured diagnostics.
- Capability-aware process boundary and cross-platform process API shape.
- CLI commands `new`, `check`, `run`, `test`, `build`, `package`, `version`, and `ast`, with Indonesian conversational primary aliases.
- Deterministic `.oneir` and `.onepkg` textual artifacts.
- Strict v0.1 project manifest validation.
- Conformance fixtures and golden-output tests.
- Verified native network runtime slices: DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits.
- General-purpose orchestration runtime foundation: typed capability requests, deterministic governance, governed adapters, normalized results/evidence, audit records, localized semantic-resolution fixtures, and `network.dns.resolve`.
- Experimental source-level DNS orchestration through `cari alamat`.
- Experimental source-level filesystem-read orchestration through `baca berkas`.
- Experimental source-level process-execution orchestration through `jalankan proses`, limited to one explicitly targeted program without arguments and backed by the recorded cross-platform evidence.
- Experimental source-level filesystem-write orchestration through `tulis ke berkas`, limited to one text payload and one explicitly targeted file, with `filesystem.write` permission; source/core/CLI tests passed in the exact-SHA Ubuntu/Windows matrix `39554661451` for `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`, alongside historical Termux evidence.
- Experimental source-level database-query and HTTP-GET slices with historical Termux evidence and source/core/CLI tests passing in the exact-SHA Ubuntu/Windows matrix `39554661451` for `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`.

## Native backend development boundary — PR #64

The native backend contract is:

```text
One Source
→ Parser
→ Analyzer
→ One Semantic IR
→ Target Backend
→ Toolchain
→ Native Executable
```

Semantic IR remains target-independent.

### Linux AArch64

Target: `aarch64-unknown-linux-gnu`

Status on implementation SHA: **VERIFIED — AArch64/Linux host boundary**. A native ELF AArch64 artifact was built and executed successfully.

### Linux x86_64

Target: `x86_64-unknown-linux-gnu`

Status on implementation SHA: **BUILD TERBUKTI — execution pending**. The backend produces a real ELF x86-64 artifact using the verified cross-linker selection in the recorded environment. No x86_64 executor was available.

### Windows x86_64

Target: `x86_64-pc-windows-gnu`

Status on implementation SHA: **BUILD TERBUKTI — execution pending**. A PE32+ x86-64 artifact is proven. No Windows/Wine executor was available for the recorded environment.

The last code-bearing source revision has passed the exact-SHA build/test matrix. Individual native target execution claims remain bounded to the exact artifact and target executor recorded in the native-backend verification evidence.

## Explicit v0.1 limits

- The existing `build` command emits a deterministic `.oneir` IR/debug artifact; the native backend slice does not replace it with a general-purpose native compiler claim.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- The language-level orchestration catalog is not complete.
- Filesystem write is experimental only within the single-text-payload/single-target boundary and its currently verified Android/Termux platform boundary.
- Process execution is experimental only within the explicitly targeted single-program/no-arguments boundary.
- Database query remains a read-only parameterized SQLite SELECT slice; its tested source-level path has exact-SHA Ubuntu/Windows evidence at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and historical Termux evidence.
- HTTP GET remains limited to explicit `http://` HTTP/1.1 GET, exact target scope, and response-body normalization; its tested source-level path has exact-SHA Ubuntu/Windows evidence at `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` and historical Termux evidence.
- Native Linux x86_64 and Windows x86_64 execution are not yet verified for the merged `main` revision.
- Native C/C++ FFI, Python/Node/JVM native bindings, package registry, IDE/editor, GUI framework integration, additional locales, and additional protocol/capability inventory beyond the verified boundary remain future work.

## Release labels

- **IMPLEMENTATION-COMPLETE:** in-scope source/docs/tests are present.
- **VERIFIED:** exact source revision has executable evidence for the stated platform boundary.
- **EXPERIMENTAL:** the declared vertical slice is verified but intentionally limited in coverage or platform boundary.
- **BUILD TERBUKTI:** a native artifact was produced and structurally validated, but target execution evidence is missing.
- **SUPPORTED:** reserved for a stronger platform/support claim.

## Verification infrastructure rule

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution. **Do not purchase an upgrade** merely to obtain hosted runner execution; use the independent verification path when the canonical hosted runner is unavailable.
