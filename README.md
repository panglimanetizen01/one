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

The first source-level orchestration vertical slice is real and independently verified:

```text
One source: cari alamat "127.0.0.1"
 -> parser
 -> semantic validation
 -> typed orchestration IR
 -> target-scoped governance
 -> network.dns.resolve
 -> normalized result
 -> CLI output
```

Run it with explicit permission and exact target scope:

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
```

Technical alias:

```text
one run main.one --allow network.discover --target 127.0.0.1
```

No explicit permission means denial. A different target is outside the granted scope. This is **Eksperimental**, not a claim that the full orchestration catalog is supported.

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

Canonical source `78fc1ecfe520bc35584e4fae36fb13ffa3226877` was independently verified in mirror run `38895405559` on Ubuntu and Windows Server 2025. The evidence includes exact-SHA checkout, contract checks, build, Clippy, workspace tests, real-network runtime tests, deterministic `.oneir`/`.onepkg` checks, DNS success/failure classification, orchestration runtime, and offline verification.

The private GitHub repository remains the source of truth. The public verification mirror is verification infrastructure only. No paid GitHub upgrade is required.
