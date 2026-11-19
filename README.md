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

CircleCI is the intended canonical Linux CI provider. The draft PR #113 candidate now contains `.circleci/config.yml`, and CircleCI `ci/circleci: one-linux` reports terminal SUCCESS on exact PR HEAD `26a34efb6d0908bd02738a7b05090f96051a6dc7` (workflow `a7016ead-ed2d-467f-9f1b-4d20c60a966f`, job `6cc9778c-307c-48fc-9111-39c97d288393`). This is job-level exact-SHA evidence; provider step logs were not separately inspected. The configuration is not integrated into canonical `main`, which remains `d35db95153f6ddd79885494c72f5aee16c899751` and still has no `.circleci/config.yml`. Therefore no canonical CI-provider cutover is claimed.

Windows Server 2025 native execution remains independently verified in [one-windows-verification](https://github.com/panglimanetizen01/one-windows-verification). Linux-host MinGW cross-compilation does not replace Windows-host execution.

## Documentation
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

- Linux candidate writes are limited to direct children of the host-injected root; nested paths fail closed.
- Windows candidate writes use native handle-relative temporary-file replacement. Protected E4 verification passed for the canonical source pin `722d662e7a81711ba3607d26571f4048a292b3ed` (verifier run [#44519577693](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577693)); the Windows native probe is limited to Windows Server 2025 build 26100.
- Protected E4 and Android API 35 ARM64 runtime evidence remain tied to source pin `722d662...`; Issue #82 is closed only for the Android runtime-evidence scope.
- Current PR #113 code candidate `c7f460a4e38be61de635ad5af9f7ff987a941f01` adds truthful filesystem adapter platform metadata and actual host-platform selection. E4 run #44585270928 passed the metadata regression on Linux and Windows; Linux distro run #44585282690 passed Ubuntu 24.04 and Debian 12 workspace tests, including the CLI regression.
- CircleCI on exact source `c7f460a...` is PENDING; last terminal SUCCESS was on documentation parent `26a34ef...` and does not transfer. The independent release gate has not passed.
- Other platform backends remain unsupported/fail-closed.

Do not rely on the current released `filesystem.write` slice to confine hostile paths or treat it as a project sandbox. Read the [user-facing feature status](docs/panduan-pengguna/status-fitur.md), [security contract](docs/SECURITY.md), and [Issue #104 design](docs/security/issue-104-filesystem-root-confinement-design.md) before enabling filesystem write in an untrusted environment.


### Host-selected filesystem root

The CLI does not derive filesystem authority from the requested target. It selects the root from the source/project context (nearest ancestor with `one.toml`, otherwise working directory or standalone source parent), then rejects absolute targets outside that root. This is separate from the exact target capability scope and is covered by CLI regression tests.
