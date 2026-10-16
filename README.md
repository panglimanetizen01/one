# One

**One** is a human-oriented, deterministic, general-purpose programming language designed to let ordinary users build software from small scripts to large projects.

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

## One Network foundation

Verified native runtime slices include DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit runtime limits.

## Experimental language-level orchestration

Two source-level orchestration vertical slices are currently real and independently verified at canonical implementation revision `f482c6b75505d9965dd33f4f25bb0cd04072cedb` through mirror run `38903950612`:

```text
One source: cari alamat "127.0.0.1"
 -> parser
 -> semantic validation
 -> typed orchestration IR
 -> target-scoped governance
 -> network.dns.resolve
 -> normalized result
 -> CLI output

One source: baca berkas "data/contoh.txt"
 -> parser
 -> semantic validation
 -> typed orchestration IR
 -> target-scoped governance
 -> filesystem.read
 -> normalized UTF-8 text result
 -> CLI output
```

Run them with explicit permission and exact target scope:

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
one jalankan main.one --izinkan filesystem.read --target data/contoh.txt
```

Technical aliases:

```text
one run main.one --allow network.discover --target 127.0.0.1
one run main.one --allow filesystem.read --target data/contoh.txt
```

No explicit permission means denial. A different target is outside the granted scope. These slices are **Eksperimental**, not a claim that the full orchestration catalog is supported.

## Process/System experimental boundary

A third source-level slice is implemented and independently verified, but intentionally limited in scope:

```text
One source: jalankan proses "rustc"
 -> parser
 -> semantic validation
 -> typed orchestration IR
 -> target-scoped governance
 -> process.execute
 -> normalized process output
 -> CLI output
```

Exact-SHA evidence: canonical source `9038ac75f197753ff1ab6af7478e63e4831903a2`, independent mirror run `38929072800`, with Ubuntu and Windows Server 2025 passing contract checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification.

Explicit permission surface:

```text
one jalankan main.one --izinkan process.execute --target rustc
```

The current implementation executes one explicitly targeted program without additional arguments and has TDD coverage for success, denial, and missing-program backend failure. This is **Eksperimental** within that boundary; it is not a claim of general process management, service management, shell scripting, or arbitrary command orchestration support.

## Documentation

- `docs/panduan-pengguna/README.md`
- `docs/panduan-pengguna/status-fitur.md`
- `docs/panduan-pengguna/perintah-cli.md`
- `docs/spec/one-v0.1-cli-contract.md`
- `docs/superpowers/specs/2026-10-16-one-general-purpose-human-language-tool-orchestration-and-governance.md`
- `docs/superpowers/plans/2026-10-16-one-human-language-tool-orchestration-and-governance.md`
- `docs/verification/one-v0.1-implementation-status.md`
- `docs/verification/one-v0.1-evidence-matrix.md`
- `docs/release/one-v0.1.md`

## Verification boundary

Process/System source revision `9038ac75f197753ff1ab6af7478e63e4831903a2` was independently verified in mirror run `38929072800` on Ubuntu and Windows Server 2025. The evidence includes exact-SHA checkout, contract checks, build, Clippy, workspace tests, real-network runtime tests, and platform-specific offline verification.

Canonical `main` is now `6d5a3ee7900a0705ebdcc9a4df4bfb3794ba7f6d`; this is the documentation checkpoint following merge commit `123df544f15d7eec8d74ca231b5e55ccebf4a7ee`. The documentation checkpoint was independently verified in mirror run `38929461255` on Ubuntu and Windows Server 2025.

The private GitHub repository remains the source of truth. The verification mirror is verification infrastructure only. No paid GitHub upgrade is required.
