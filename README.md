# One

**One** is a human-oriented, deterministic, general-purpose programming language designed to let ordinary users build software from small scripts to large projects.

## Project context — read this first when starting a new session

The repository is the persistent source of accepted One project context. A new development session must read these contracts before making changes:

1. [`AGENTS.md`](AGENTS.md) — agent operating contract and required reading order.
2. [`docs/ONE_CONSTITUTION.md`](docs/ONE_CONSTITUTION.md) — vision, mission, architecture invariants, portability, security, localization, and global direction.
4. [`docs/REQUIREMENTS.md`](docs/REQUIREMENTS.md) — accepted requirements baseline.
5. [`docs/DECISIONS.md`](docs/DECISIONS.md) — accepted architectural decisions.
6. [`docs/DEVELOPMENT_PROTOCOL.md`](docs/DEVELOPMENT_PROTOCOL.md) — execution, verification, bug, and evidence rules.
7. Read the domain contract relevant to the task: [`ARCHITECTURE.md`](docs/ARCHITECTURE.md), [`COMPATIBILITY.md`](docs/COMPATIBILITY.md), [`SECURITY.md`](docs/SECURITY.md), [`LOCALIZATION.md`](docs/LOCALIZATION.md), or [`INTEROPERABILITY.md`](docs/INTEROPERABILITY.md).
8. Check current verification evidence before claiming support or completion.

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

## Repository health and guarded repair tooling

The repository-health scanner is **developer tooling**, not part of One's source-language grammar or CLI. It must not change how ordinary users write One programs.

- Scanner and tests: `tools/one_health.py`, `tools/test_one_health.py`
- Exact-SHA GitHub Actions evidence collector: `tools/collect_ci_health.py`, `tools/test_collect_ci_health.py`
- Reviewed detector/repair quality metrics: `tools/health_quality.py`, `tools/test_health_quality.py`, with the [quality ledger guide](docs/verification/health-quality-ledger.md)
- Usage and detector limitations: [repository health guide](docs/verification/repository-health.md) and [detector governance](docs/verification/health-detectors.md)

Run the read-only checks with Python 3.11+:

```sh
python tools/one_health.py
python -m unittest discover -s tools -p "test_one_health.py" -v
python -m unittest discover -s tools -p "test_collect_ci_health.py" -v
```

The current implementation covers repository paths and text integrity, Python/TOML/JSON syntax, Markdown links, heading anchors and fences, targeted workflow references and overlap hints, normative One contract drift, baseline triage, exact-SHA Actions run/job/step/artifact metadata with cautious failure correlations, and read-only security heuristics for known credential patterns, suspicious Python dynamic execution/shell usage, and risky GitHub Actions configuration. JSON and Markdown reports expose detector execution status and findings per detector; these are coverage indicators, not accuracy or repair-success guarantees. Its safe-repair allowlist currently only adds a missing final newline to an explicitly selected Markdown file after a preview and guarded branch/clean-tree checks. It redacts suspected secret values from findings and does not claim complete detection: obfuscation, novel payloads, unknown credential formats, runtime behavior, and false positives require layered controls and human review. It does not autonomously edit One source, approve findings, merge pull requests, or release production artifacts. Full YAML validation, live log interpretation/root-cause analysis, broader repair classes, and manual detector-quality adjudication remain evolving work. Use the quality ledger to record reviewed false positives/misses and verified repair outcomes; empty or unreviewed records are not counted as success.

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

The last production-implementation revision independently verified is `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`. Exact-SHA verification run [`39554661451`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/39554661451) checked out that revision and passed on Ubuntu and Windows Server 2025. Later commits added unit tests to `crates/one-core/src/stdlib.rs` (commit `ce8681d9e8aa6794bce284e39737f7d0af60eb05`) and removed a redundant test import (current `main` HEAD `d35db95153f6ddd79885494c72f5aee16c899751`). The production implementation was not changed by those two commits, but those added/changed tests have not been demonstrated by run `39554661451`; therefore current `main` HEAD is not exact-SHA verified by that run. Coverage of the verified implementation revision included:

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

## Documentation

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

The private GitHub repository remains the source of truth. Verification mirrors and local environments are verification infrastructure only. Never generalize evidence from one revision or platform to another without proof.


## Engineering execution and CI authority

All repository changes are made against GitHub branches; GitHub is the single source of truth. CircleCI is the canonical CI provider for this source repository and replaces GitHub Actions as the default CI path. Termux is limited to read-only, post-CircleCI verification of an already-proven exact SHA; it is not an implementation or repair environment. See [CI execution model](docs/CI_EXECUTION_MODEL.md).
