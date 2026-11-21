# One

**One** is a human-oriented, deterministic, general-purpose programming language designed to let ordinary users build software from small scripts to large projects.

## Project context — read this first when starting a new session

The repository is the persistent source of accepted One project context. A new development session must read these contracts before making changes:

1. [`AGENTS.md`](AGENTS.md) — agent operating contract and required reading order.
2. [`docs/ONE_CONSTITUTION.md`](docs/ONE_CONSTITUTION.md) — vision, mission, architecture invariants, portability, security, localization, and global direction.
3. [`docs/REQUIREMENTS.md`](docs/REQUIREMENTS.md) — accepted requirements baseline.
4. [`docs/DECISIONS.md`](docs/DECISIONS.md) — accepted architectural decisions.
5. [`docs/DEVELOPMENT_PROTOCOL.md`](docs/DEVELOPMENT_PROTOCOL.md) — execution, verification, bug, and evidence rules.
6. Read the domain contract relevant to the task: [`ARCHITECTURE.md`](docs/ARCHITECTURE.md), [`COMPATIBILITY.md`](docs/COMPATIBILITY.md), [`SECURITY.md`](docs/SECURITY.md), [`LOCALIZATION.md`](docs/LOCALIZATION.md), or [`INTEROPERABILITY.md`](docs/INTEROPERABILITY.md).
7. Check current verification evidence before claiming support or completion.

This repository contract exists so project intent does not depend on a previous ChatGPT session.

## Foundation

- Human-oriented conversational surface syntax
- Formal deterministic grammar
- Minimal required punctuation
- Indonesian as the initial language with localization designed in from the start
- Common semantic model / IR shared across locales
- Core constructs: DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT
- Ambiguity is rejected rather than guessed
- Human-readable diagnostics
- Windows and Linux as first-class platforms
- LLM-independent compiler/runtime semantics
- Cross-distro, cross-terminal, cross-editor, and cross-environment portability as architectural goals
- Deep interoperability with existing languages and technologies

## v0.1 implementation

```text
Localized source
    -> Lexer
    -> AST parser
    -> Semantic analyzer
    -> One Semantic IR
    -> Reference interpreter
    -> CLI
```

CLI primary commands:

```text
one buat <folder>              # alias: one new
one periksa <file.one>         # alias: one check
one jalankan <file.one>        # alias: one run
one test <folder>
one bangun <file.one>          # alias: one build
one package <folder>
one lihat struktur <file.one> # alias: one ast
one version
one help
```

The implementation includes the six core constructs, structured diagnostics, capability model, explicit process interoperability, project scaffolding, strict v0.1 manifest validation, deterministic `.oneir`/`.onepkg` artifacts, and conformance/golden fixtures.

## Native backend development boundary

The native backend implementation from PR #64 is present on canonical `main` from merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`. It uses the same One Semantic IR and adds hosted native targets without target-specific semantic forks:

```text
One Source
 -> Parser
 -> Analyzer
 -> One Semantic IR
 -> Target Backend
 -> Toolchain
 -> Native Artifact
```

The last code-bearing canonical `main` revision independently verified is `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`. Exact-SHA verification run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451) checked out that source revision and passed on Ubuntu and Windows Server 2025. The subsequent evidence-documentation sync changes documentation only, not source or tests. Coverage included:

- contract artifacts and specification/implementation checks;
- build, AST contract regression tests, Clippy, and workspace tests;
- real-network runtime tests;
- platform-appropriate offline verification;
- Linux and Windows cross-toolchain provisioning and native-backend test coverage.

The canonical repository's hosted CI runs `39372356906` and `39381125667` failed before executable steps (`steps: []`). Those runs are infrastructure evidence, not source-test results; the independent exact-SHA matrix is the positive executable evidence for the last code-bearing source revision.

Target-specific claims remain bounded to their actual evidence: the recorded AArch64 execution is for its stated AArch64/Linux host and earlier implementation SHA; x86_64/Windows artifact and test evidence must not be generalized into an unrecorded execution claim for another target. See `docs/verification/native-backends-2026-10-19.md` and the evidence matrix for details.

## One Network foundation

Verified native runtime slices include DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits.

## Experimental language-level orchestration

DNS, filesystem read, and the limited process-execution source-level slices are experimentally available within their recorded evidence boundaries.

```text
One source: cari alamat "127.0.0.1"
 -> parser -> semantic validation -> typed orchestration IR
 -> target-scoped governance -> network.dns.resolve -> CLI output

One source: baca berkas "data/contoh.txt"
 -> parser -> semantic validation -> typed orchestration IR
 -> target-scoped governance -> filesystem.read -> CLI output

One source: jalankan proses "rustc"
 -> parser -> semantic validation -> typed orchestration IR
 -> target-scoped governance -> process.execute -> CLI output
```

The process slice has exact-SHA Ubuntu + Windows evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800` and is limited to one explicitly targeted program without arguments.

## Filesystem write experimental boundary

The source-level filesystem-write slice is executable:

```text
tulis ke berkas "data/hasil.txt" dengan "halo dari One"
 -> parser
 -> semantic validation
 -> typed orchestration IR
 -> target-scoped governance
 -> filesystem.write
 -> CLI output
```

Explicit permission:

```text
one jalankan main.one --izinkan filesystem.write --target data/hasil.txt
```

Technical alias:

```text
one run main.one --allow filesystem.write --target data/hasil.txt
```

The filesystem-write source/core/CLI tests also passed in the exact-SHA Ubuntu/Windows Server 2025 matrix for source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` (run [39554661451](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451)). This proves the tested source-level slice on those host environments, not universal filesystem portability or release readiness.

## Database query boundary

A SQLite-backed source-level database query path is executable-verified within its recorded Termux/Android boundary and its core/CLI test suites passed in the exact-SHA Ubuntu/Windows Server 2025 matrix for source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` (run [39554661451](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451)). It remains a read-only parameterized SELECT slice and is not a general database abstraction.

## HTTP GET boundary

An explicit HTTP GET source-level path is executable-verified within its recorded Termux/Android boundary and its core/CLI loopback test suites passed in the exact-SHA Ubuntu/Windows Server 2025 matrix for source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` (run [39554661451](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451)). The slice is bounded to explicit `http://` URLs, GET, exact target scope, and response-body normalization.


## Architecture navigation

- [ONE full architecture map](https://github.com/panglimanetizen01/one/blob/docs/system-architecture-diagram-20261115/docs/architecture/one-full-architecture-map.png) — visual navigation reference currently carried by draft PR #97.
- [Written architecture contract](docs/ARCHITECTURE.md) — normative semantic and platform boundaries. The diagram is navigation only and never overrides contracts, implementation, or exact-SHA evidence.

## CI execution provider

CircleCI is the intended canonical Linux CI provider, currently carried only by draft PR #113. The live exact-SHA readback is recorded in [the live 2026-11-20 checkpoint](docs/verification/live-checkpoint-2026-11-20.md): code-bearing source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` is SUCCESS, while current documentation HEAD `5daf3337b4bcc93fa3ee6072f9633915d5fea85b` is FAILURE (job #833; first reported step #101 CANCELED; provider root cause UNKNOWN). The source-to-HEAD range is documentation-only, so neither status is transferable to the other SHA. The separate `main` pipeline #399 is ERROR, with provider cause UNKNOWN. No CircleCI provider logs were inspected in this session.

The configuration is not integrated into canonical `main`, which remains `d35db95153f6ddd79885494c72f5aee16c899751` and has no `.circleci/config.yml`. No canonical CI-provider cutover is claimed.

Windows Server 2025 native execution remains independently verified in [one-windows-verification](https://github.com/panglimanetizen01/one-windows-verification). Linux-host MinGW cross-compilation does not replace Windows-host execution.

## Documentation
- [Live continuation checkpoint — 2026-11-20](docs/verification/live-checkpoint-2026-11-20.md) — exact source/documentation SHAs, CircleCI split status, independent verifier runs, platform limits, and next gates.
- [Forensic repository audit](docs/verification/forensic-audit-2026-11-17.md) — latest inventory, confirmed findings, exact-SHA evidence, and open gates.
- [Master continuation prompt](docs/verification/MASTER_CONTINUATION_PROMPT_2026-11-17.md) — mandatory read-first context and next-session execution order.

- `docs/panduan-pengguna/README.md`
- `docs/panduan-pengguna/status-fitur.md`
- `docs/panduan-pengguna/perintah-cli.md`
- `docs/spec/one-v0.1-cli-contract.md`
- `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md`
- `docs/verification/one-v0.1-implementation-status.md`
- `docs/verification/one-v0.1-evidence-matrix.md`
- `docs/verification/native-backends-2026-10-19.md`
- `docs/release/one-v0.1.md`

## Verification boundary

The GitHub repository is currently PUBLIC and remains the canonical source of truth. Repository visibility is owner-controlled and must not be changed as part of routine development. Verification mirrors and local environments are verification infrastructure only. Never generalize evidence from one revision or platform to another without proof.


## Project-root filesystem confinement status

The source-level `filesystem.write` slice described above is distinct from a security sandbox. The stronger project-root confinement capability is still experimental in draft PR #113 / Issue #104 and is **not part of canonical `main`**.

- Linux candidate writes support direct children and a guarded detached-mount nested path; nested operations fail closed when required kernel/security primitives are unavailable. Android/macOS have direct-child native candidates; their nested paths remain fail-closed.
- Windows candidate writes use native handle-relative temporary-file replacement. Protected E4 verification passed for the canonical source pin `722d662e7a81711ba3607d26571f4048a292b3ed` (verifier run [#44519577693](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577693)); the Windows native probe is limited to Windows Server 2025 build 26100.
- Protected E4 and Android API 35 ARM64 runtime evidence remain tied to source pin `722d662...`; Issue #82 is closed only for the Android runtime-evidence scope.
- Current PR #113 code candidate `c7f460a4e38be61de635ad5af9f7ff987a941f01` adds truthful filesystem adapter platform metadata and actual host-platform selection. E4 run #44585270928 passed the metadata regression on Linux and Windows; Linux distro run #44585282690 passed Ubuntu 24.04 and Debian 12 workspace tests, including the CLI regression.
- CircleCI on exact source `c7f460a...` is PENDING; last terminal SUCCESS was on documentation parent `26a34ef...` and does not transfer. The independent release gate has not passed.
- Other platform backends remain unsupported/fail-closed.

Do not rely on the current released `filesystem.write` slice to confine hostile paths or treat it as a project sandbox. Read the [user-facing feature status](docs/panduan-pengguna/status-fitur.md), [security contract](docs/SECURITY.md), and [Issue #104 design](docs/security/issue-104-filesystem-root-confinement-design.md) before enabling filesystem write in an untrusted environment.


### Host-selected filesystem root

The CLI derives filesystem authority exclusively from the host process current working directory (`cwd`), never from the source-file location or requested target. Relative and absolute targets outside that root are rejected. This is separate from the exact target capability scope and is covered by positive and negative CLI regression fixtures.

## Host-owned filesystem root implementation checkpoint — 2026-11-20

PR #113 implementation now derives filesystem authority exclusively from the host process working directory. Linux root acquisition uses `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC` and checks device/inode identity against the retained handle. Windows preserves the final root component, opens with native no-reparse flags, and checks volume/file identity. CLI positive/negative root-selection fixtures plus Linux symlink-root and Windows junction-root regressions have been added.

Latest source/test candidate: `ea376531e99f37b874d3d3a8edd4e8b4d656095f`; current PR #113 HEAD at last readback: `6f7d86842c80a81135227d8d62404e848ab5d8c3`. These changes have NOT been built or tested; no prior exact-SHA evidence transfers. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## Android host identity — 2026-11-20

- `crates/one-core/src/platform.rs` now exposes `Platform::Android` and returns it for `target_os = "android"` instead of collapsing Android into `Platform::Other` (`f8e46856803c9f45f54402e71d7e19853bdf46bc`).
- This is host identity only; it does NOT enable Android filesystem operations. The filesystem adapter remains fail-closed on Android until a safe resolver is available that avoids process-killing seccomp behavior and preserves root/symlink/mount/race guarantees.
- The platform unit test was updated but has NOT been run.

## Explicit desktop/mobile host identity — 2026-11-20

- `Platform` now identifies Windows, Linux, Android, and macOS explicitly rather than collapsing Android/macOS into `Other`; the platform unit test covers all four target cfgs (`5846eb70f2a462331873cc0023d1f1ab1106f86a`).
- This is platform identification only. It does not imply filesystem adapter support on Android/macOS; those adapters remain fail-closed pending platform-safe resolver implementations.
- The platform test has NOT been run.


## Latest filesystem implementation candidate — 2026-11-21

The current unmerged PR #113 code-bearing source is `72eae103bc3304b0961c66a0cf61f716ddfa9ac4`. Its host-owned filesystem root is the process working directory, never a source-file path, manifest lookup, or program-supplied target. Linux includes a guarded detached-mount nested writer; Windows retains native handle-relative operations; Android/macOS include direct-child native read/write candidates and fail closed for nested paths. The CLI fixture corrections are test-only commits `46c777e...` and `2600ec1...`. No tests/builds were run on this candidate. Issue #104 remains OPEN / NOT ACCEPTED, and canonical `main` is unchanged.

## Superseding Issue #104 / filesystem.delete checkpoint — 2026-11-21

PR #113 remains OPEN / DRAFT / UNMERGED. The current language-bridge source checkpoint is 0fa43eb0be53c8e79424597ece055147872b35c4; the current documentation-only PR descendant at this update is tracked in the live checkpoint. This supersedes the older PR HEAD and test-status snapshots above.

The Windows native file-delete backend is implemented and its native tests passed at exact source/test SHA 6f2d5e2d0b02d6aed6c692a8a7b7fa184a70fcb0 (One CI #44870781889). The subsequent ONE language bridge adds the explicit filesystem.delete capability, Delete effect, and source form:

    hapus berkas "target.txt"

Delete is not implied by general filesystem/read/write grants. The adapter requires host-injected FilesystemAuthority and is advertised on Windows only. Linux, Android, and macOS remain fail-closed for delete. At source bridge SHA 0fa43eb0be53c8e79424597ece055147872b35c4, Ubuntu Build/Clippy/Tests passed; Windows and CircleCI were still pending at last exact-SHA readback. Do not claim the language bridge verified until both are terminal. Issue #104 remains OPEN / NOT ACCEPTED; see the security design and single release blocker register.

## Exact-SHA filesystem.delete verification — 2026-11-21

The Windows-only filesystem.delete language bridge passed One CI run #44871756593 on exact source/test SHA 0fa43eb0be53c8e79424597ece055147872b35c4: Ubuntu Build/Clippy/Tests SUCCESS (181 passed / 0 failed / 3 ignored); Windows Server 2025 Build/Clippy/Tests SUCCESS (167 passed / 0 failed). CircleCI status context ci/circleci: one-linux is SUCCESS on the same SHA. The Windows explicit-grant path and actual junction escape sentinel-preservation test passed. Linux, Android, and macOS delete remain unavailable/fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.
