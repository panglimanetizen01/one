# Changelog

## Unreleased — security and filesystem integration

- Bind filesystem authority to the host process working directory instead of the `.one` source location or requested target.
- Open Linux project roots with directory/no-follow/close-on-exec flags and validate path identity against the retained root handle.
- Open Windows project roots without following a final reparse point; validate volume/file identity against the retained native handle.
- Add a CLI regression fixture for a source file outside the working directory accessing an in-root target.


## Unreleased — One v0.1

- Added formal lexical and concrete grammar contracts.
- Added Rust 2024 workspace and Linux/Windows CI matrix.
- Added deterministic lexer, AST, parser, semantic analysis, Semantic IR, reference interpreter, diagnostics, and capability model.
- Added cross-platform process interoperability and core standard-library primitives.
- Added deterministic project lifecycle, conversational Indonesian CLI commands plus technical aliases, manifest validation, `.oneir`/`.onepkg` artifacts, conformance fixtures, and golden outputs.
- Added Linux/Windows offline verification scripts and deterministic artifact checks so verification does not depend on purchasing hosted CI capacity.
- Added the One Network semantic/runtime foundation and verified native runtime slices for DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit limits.
- Added the general-purpose human-language orchestration foundation: typed semantic capability requests, scoped deterministic governance, typed adapters, deterministic orchestration, normalized evidence, audit records, Indonesian/English conformance fixtures, and the first real `network.dns.resolve` adapter.
- Added the first experimental language-level orchestration slice: `cari alamat` reaches the governed DNS adapter through typed IR and returns a normalized CLI result when `network.discover` is explicitly granted for an exact target.
- Added the second experimental language-level orchestration slice: `baca berkas` reaches the governed `filesystem.read` adapter through typed IR and returns normalized UTF-8 text when `filesystem.read` is explicitly granted for an exact target.
- Added policy-denial, out-of-scope, backend-failure, and unavailable-adapter coverage for the filesystem source-level orchestration path.
- Independently verified the filesystem source-level orchestration/CLI revision on Ubuntu and Windows Server 2025 through exact-SHA checkout, contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, offline verification, and deterministic artifact verification at canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` via mirror run `38903950612`.
- Refreshed the implementation-status, release, CLI contract, grammar, and user-guide documentation to distinguish the verified filesystem vertical slice from the still-in-development orchestration catalog.
- Added the language-level process-execution slice: `jalankan proses` → parser → semantic validation → typed orchestration IR → target-scoped governance → `process.execute` adapter → normalized output → CLI, with TDD coverage for success, denial, and backend failure.
- Independently verified the process-execution source/runtime slice at canonical source `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800` on Ubuntu and Windows Server 2025. The verified boundary is one explicitly targeted program without additional arguments and explicit `process.execute` permission.
- Added the filesystem-write source-level slice: `tulis ke berkas` → parser → semantic validation → typed orchestration IR → target-scoped governance → `filesystem.write` adapter → CLI.
- Added a CLI regression test proving `filesystem.write` permission reaches the adapter and writes the expected text to the exact target. The filesystem-write CLI verification was first recorded on Termux/Android and remains platform-bound to its recorded exact SHA.
- Added the database-query source/runtime slice: `kueri database` → parser → semantic validation → resource-scoped `database.query` capability → `database.sqlite.query` adapter → parameterized SQLite `SELECT` → deterministic UTF-8 normalization → CLI, with executable evidence recorded for its current Termux/Android boundary.
- Added the HTTP GET source/runtime slice: `ambil dari` → parser → semantic validation → target-scoped `network.connect` capability → `network.http.get` adapter → bounded HTTP/1.1 GET → deterministic response-body normalization → CLI, with executable evidence recorded for its current Termux/Android boundary.
- Added the native hosted backend contract and implementation slice for Linux AArch64, Linux x86_64, and Windows x86_64 on PR #64. Semantic IR remains target-independent and native code generation uses the existing deterministic backend/toolchain boundary.
- Native Linux AArch64 is verified on the exact PR #64 SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd` for an AArch64/Linux host boundary: ELF AArch64 artifact built and executed successfully.
- Native Linux x86_64 is build-proven on the same PR SHA: ELF x86-64 artifact built successfully using the verified cross-linker selection and the dedicated regression test passes. Target execution remains pending because no x86_64 executor was available.
- Native Windows x86_64 is build-proven on the same PR SHA: PE32+ x86-64 artifact proven. Target execution remains pending because no Windows/Wine executor was available.
- PR #64 was merged into canonical `main` at `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`. The pre-merge native evidence remains bounded to its recorded implementation SHA until exact-SHA post-merge verification is completed.
- The first post-merge `One CI` run `39371290853` failed on both matrix jobs before any executable step (`steps: []`); this is recorded as an infrastructure/runner failure, not as source-code failure evidence.
- Updated the native backend contract map, implementation-status inventory, evidence matrix, release boundary, user feature-status guide, and verification record to reflect the merged state and preserve exact-SHA evidence boundaries.

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution.

## Release execution controls — 2026-11-20

- Added one linked release blocker register and a compact cross-session master prompt, connected to the existing roadmap, evidence matrix, Mermaid architecture source, release contract, and documentation index.
- Recorded the exact CircleCI pre-execution mapping for PR #113 pipelines #1199 and #1198. Both jobs have `started_at=null`; root cause remains UNKNOWN and is not labeled a test timeout or source failure.

## Android filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec` (NOT BUILT OR TESTED).
- Android now has a native `FilesystemAuthority` root open and a handle-relative direct-child read/atomic-replace backend. Root and leaf are opened without following final symlinks; `statx(STATX_MNT_ID)` is required to reject mount crossings; reads reject non-regular and hard-linked files; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The backend deliberately avoids `openat2` because Android seccomp may terminate the process with `SIGSYS` for that syscall. It requires kernel `statx` mount-ID support; if unavailable it fails closed.
- Scope is intentionally limited to direct children of the trusted root. Nested Android paths return `UnsupportedPlatform`; no claim of full Android filesystem support or Issue #104 acceptance is made.
- Three Android-targeted regression tests cover direct-child read/write, leaf-symlink and nested-path denial, and final-component root-symlink rejection. They have NOT been run. Actual Termux/device execution is still required.
- Linux nested writes remain fail-closed; macOS filesystem adapter remains unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## macOS filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814` (NOT BUILT OR TESTED).
- macOS now has a native retained-root open and direct-child read/atomic-replace backend. Root and leaf operations use directory/file descriptors with `O_NOFOLLOW`; file and root device identity are compared; hard-linked and non-regular targets are rejected; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The macOS adapter is deliberately limited to direct children of the trusted root. Nested paths return `UnsupportedPlatform`. This implementation has no target-host execution evidence yet; its mount/device and race behavior still require platform-specific review.
- Three macOS-only regression tests cover direct-child read/write, symlink/nested-path denial, and final-component root-symlink rejection. They have NOT been run.
- Android direct-child backend remains statx-mount-ID dependent; Linux nested writes remain fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.

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
