# One v0.1 Release Contract

Status: **EXACT-SHA CROSS-PLATFORM VERIFIED BASELINE WITH EXPERIMENTAL ORCHESTRATION; RELEASE APPROVAL REMAINS A SEPARATE GATE**.

## Current migration candidate — PR #95 — 2026-11-16

- Candidate branch: `ci/circleci-migration-20261113`; exact source SHA: `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`; base `main`: `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux`: PASS; workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`; job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`.
- Independent Windows Server 2025 run [#28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749): PASS, 32/32 steps; verifier SHA `29df4c3f0cd4bf15fcba2d74106bf21d5fc07507`; native exit code `0`, output `Budi`.
- Windows artifact ZIP SHA-256: `7dd4435e41bc8f0660f454768711c0fc460e2ca0fe38f2dee1034c59c0f767a5`; executable SHA-256: `9004e87f0c9d0357ee0197a1bce2a26474cb714aa8d71d54990236c9266dcfeb`; manifest SHA-256: `d95ae44ee24c5b793aeef59fb006e48d4c5b3e62de4cd5012b2b5a769cbf92a4`.
- PR #95 remains OPEN / DRAFT / UNMERGED. `main` remains unchanged and still owns the legacy GitHub Actions workflows until an authorized merge. The opt-in CircleCI `release-check` has not been run. This candidate evidence is not release approval and does not establish 100% project completion.

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

## Hosted CI infrastructure interpretation — 2026-11-16

A hosted CI failure is not automatically a release-blocking product defect. The evidence must identify whether ONE checks actually executed. The historical Windows job #254 failed during MSYS2 first-run/keyring initialization before ONE checks and belongs to a superseded workflow; do not reintroduce that job solely to make its historical run green. Jobs #409, #424, and #426 have explicit no-credit messages and are PLAN-CREDIT-BLOCKED; jobs #407 and #408 did not demonstrate runner execution and retain unknown root cause. Independent Windows run #21 is positive evidence only for source SHA f85375e279257dcbb7d4ded9632800fcd1b78cc9.

Release claims remain tied to exact source SHA and platform-specific executable evidence. See [CircleCI incident record](../verification/circleci-infrastructure-incident-2026-11-16.md) and [CI verification evidence contract](../spec/one-v0.1-ci-verification-contract.md). No paid-plan upgrade is implied or authorized by an infrastructure incident.
