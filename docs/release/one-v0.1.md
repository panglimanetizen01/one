# One v0.1 Release Contract


## Current exact-SHA CI candidate checkpoint — 2026-11-16

- Canonical candidate: PR #95, branch `ci/circleci-migration-20261113`, exact source SHA `e92d3a3003b68283d5f0fe1315dcb41dcc33bd62`; base `main` SHA `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux` is terminal SUCCESS on that exact source SHA (workflow `87ef4587-00f4-46a6-9837-01f9d760f9e9`, job `72e6485f-ae89-4302-b4b8-e084b5de60bd`). This is the Linux product job, not the opt-in release job.
- Independent Windows Server 2025 run [#22](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43852530420) is terminal SUCCESS for the same exact source SHA. Subsequent verifier-documentation validation runs [#23](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43853509916) and [#24](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43853556846) also completed SUCCESS; run #24 is the latest verifier-branch run.
- Latest Windows run #24: job `133254250473`; 32/32 steps succeeded; verifier merge-ref SHA `2b9d70894d7b166695a571311c67460d505b9c0f`; target `x86_64-pc-windows-gnu`; Windows Server 2025 AMD64; native exit code `0`; output `Budi`.
- Run #24 artifact ID `14220981534`; ZIP SHA-256 `e65a6925c67b325667cc6e029d00e1542601facdfb2474cf1766546619c24567`, independently recomputed and matched GitHub's artifact digest. Embedded executable SHA-256 `cf07f9179783eb90b8637192a6418cba25d42461504da9e5a76c159cd02fffeb`; MZ, PE signature, AMD64 `0x8664`, and PE32+ `0x20B` were independently inspected. Manifest source SHA equals `e92d3a3003b68283d5f0fe1315dcb41dcc33bd62` and records verifier SHA, run identity, host, binary digest, exit code, and output.
- Structural audit of the candidate tree: no `.github/workflows/` paths and no `ci-local` paths. The canonical CircleCI config contains only `one-linux` by default and an opt-in `release-linux` job; no CircleCI Windows executor is present.
- IMPORTANT: PR #95 is still OPEN / DRAFT / UNMERGED. The migration is not active on canonical `main` until an authorized merge. Current `main` remains `d35db95153f6ddd79885494c72f5aee16c899751` and still contains the three legacy GitHub Actions workflows. Do not claim the migration is live on main.
- The opt-in CircleCI `release-check` workflow (pipeline parameter `run_release_checks=true`) has not yet been executed for this candidate; release approval is separate. This candidate checkpoint is CI PASS, not final project/release completion.
- The only changes since the earlier Windows-tested source `f85375e279257dcbb7d4ded9632800fcd1b78cc9` are three documentation-only commits across eight Markdown files; no product source, tests, or CircleCI config changed in that interval. Exact-SHA evidence is nevertheless bound to `e92d3a3...`.

Status: **EXACT-SHA CROSS-PLATFORM VERIFIED BASELINE WITH EXPERIMENTAL ORCHESTRATION; RELEASE APPROVAL REMAINS A SEPARATE GATE**.

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


## CI provider ownership — 2026-11-14

CircleCI is the canonical CI execution provider for the ONE repository's Linux product and Linux release checks. The migration branch runs `one-linux` and, when requested, `release-linux`; it does not define a Windows CircleCI executor or Windows release job.

Windows Server 2025 product/native verification is performed by the independent `panglimanetizen01/one-windows-verification` repository. Its workflow pins the canonical ONE source by full commit SHA, asserts source and verifier identities, executes the Windows native artifact, and uploads a manifest plus binary. The Linux CircleCI result and Windows verifier result must both pass for the same exact source SHA before cross-platform verification is claimed.

The Linux CircleCI job retains MinGW-w64 cross-compilation regression coverage for the Windows target. This does not prove Windows-host execution. Current provider boundaries and evidence requirements are maintained in `docs/CI.md` and `docs/verification/windows-native-verification.md`.
