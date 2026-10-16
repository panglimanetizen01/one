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

The source-level filesystem-write slice is now executable:

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

Latest exact-SHA executable evidence: `f4b4edc2e408928fb697fb355c3c69c88c62e09d`, verified on Termux/Android with contract check, build, Clippy `-D warnings`, targeted filesystem-write CLI test, all workspace tests, and offline verification. This does **not** establish Ubuntu/Windows support for this SHA. The boundary is one text payload to one explicitly targeted file with explicit `filesystem.write` permission.

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

Latest exact-SHA executable evidence for the current executable boundary is `f4b4edc2e408928fb697fb355c3c69c88c62e09d`, verified on Termux/Android. Contract, build, Clippy, targeted filesystem-write CLI, workspace tests, and offline verification all passed. Later documentation-only commits do not change that executable evidence boundary.

Latest cross-platform evidence remains `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800` on Ubuntu and Windows Server 2025. That evidence predates the filesystem-write CLI commits and must not be generalized to newer SHAs.

The private GitHub repository remains the source of truth. The verification mirror is verification infrastructure only. No paid GitHub upgrade is required.
