# One v0.1 Release Contract


## Current exact-SHA CI candidate checkpoint — 2026-11-16

- Canonical candidate: PR #95, branch `ci/circleci-migration-20261113`, exact source SHA `bf37ccd3af924c95594b8ff3226b720f71dfdbb7`; base `main` SHA `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux`: terminal SUCCESS for this exact SHA; status ID `59019926594`, description `Your tests passed on CircleCI!`; workflow `742c32f4-6257-4b50-8de2-052ba494756a`, job `c516b092-744c-42c7-9959-683715952beb`.
- Independent Windows Server 2025 native verification: [run #27](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43858874677), terminal SUCCESS; job `133270151783`, 32/32 steps succeeded. Manifest source SHA exactly matches this candidate; target `x86_64-pc-windows-gnu`; host Windows Server 2025 AMD64; native exit code `0`, output `Budi`.
- Windows artifact ID `14222217931`, 15,870 bytes; independently recomputed ZIP SHA-256 `9a95c4b824c2f8ae938ece2d984ded52604bb5d9db1a70fbd4ade87d1bc29a98`; executable SHA-256 `7b5e04217fd38eb91d7deb8b67c9c8e28604f47a75865d3f74cc56c25cdb7dbe`; PE MZ/signature, AMD64 `0x8664`, PE32+ `0x20B` inspected. Manifest SHA-256 `b9ccd1f8d9358cacb483718bc9d9d5c095bdf94b7ab5a765141a7a07061543e0`; verifier SHA `07f3fb4c488a90d2dfbecba08df39e3a1328f94f`.
- Both required candidate CI gates therefore report PASS on the same exact source SHA: CircleCI Linux and independent native Windows. Historical failures on earlier SHAs do not override this exact-SHA evidence.
- CircleCI config defines Linux-host `one-linux` by default and opt-in Linux `release-linux`; it does not claim a CircleCI Windows executor. Linux MinGW cross-compilation is not Windows-host execution.
- PR #95 remains OPEN / DRAFT / UNMERGED. Canonical `main` remains `d35db95153f6ddd79885494c72f5aee16c899751` and still contains the three legacy GitHub Actions workflows; migration is not active on main until an authorized merge.
- Opt-in CircleCI `release-check` (`run_release_checks=true`) has not been executed for this candidate. CI PASS is not release approval or proof that the entire ONE project is 100% complete. No merge, tag, or release is authorized.


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
