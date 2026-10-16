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
- Complete project lifecycle is an architectural goal
- Deep interoperability with existing ecosystems is an architectural goal
- LLM-independent compiler/runtime semantics
- Progressive complexity from beginner programs to large systems

## Untuk pengguna baru

Mulai dari **Panduan Pengguna One** di `docs/panduan-pengguna/README.md`. Panduan tersebut membedakan fitur yang sudah tersedia dari fitur yang masih dikembangkan.

## v0.1 implementation

The canonical repository contains the executable language core:

```text
Localized source
    -> Lexer
    -> AST parser
    -> Semantic analyzer
    -> One Semantic IR
    -> Reference interpreter
    -> CLI
```

The v0.1 CLI uses Indonesian conversational commands as its primary interface while retaining technical aliases:

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

The implementation includes the six core constructs, structured diagnostics, a capability model, explicit process interoperability, project scaffolding, strict v0.1 manifest validation, deterministic `.oneir`/`.onepkg` artifacts, and conformance/golden fixtures.

`one bangun` emits a deterministic `.oneir` IR/debug artifact. `one package` emits a deterministic `.onepkg` artifact. Neither is yet a native executable compiler or platform installer.

## One Network foundation

The first One-owned network vertical slice is implemented as a typed semantic model with explicit address, transport, security, protocol-family, target, timeout, connection, and connect/send/receive operation types. Network authority is granular and capability-aware.

Verified native runtime slices include DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within their explicit runtime limits. Registry entries do not by themselves imply runtime support.

## General-purpose orchestration foundation

One now has a verified runtime foundation for general-purpose human-language tool orchestration:

```text
semantic capability request
    -> scope validation
    -> execution governance
    -> compatible typed adapter
    -> normalized result/evidence
    -> audit record
```

The first real adapter is `network.dns.resolve`. Indonesian/English semantic-resolution fixtures, governance tests, adapter tests, result normalization, audit behavior, and real DNS runtime evidence are present.

**Important boundary:** this is a verified runtime-library foundation, not yet a claim that arbitrary One source code can invoke orchestration end-to-end. The next implementation target is:

```text
One source
 -> parser
 -> semantic operation
 -> capability resolution
 -> governance
 -> adapter
 -> normalized result
 -> One value/diagnostic
 -> CLI/runtime
```

## Documentation

User-facing documentation:

- `docs/panduan-pengguna/README.md`
- `docs/panduan-pengguna/status-fitur.md`
- `docs/README.md`

Normative engineering documents:

- `docs/superpowers/specs/2026-10-15-one-language-foundation-design.md`
- `docs/superpowers/specs/2026-10-15-one-end-to-end-network-platform-design.md`
- `docs/superpowers/specs/2026-10-16-one-general-purpose-human-language-tool-orchestration-and-governance.md`
- `docs/superpowers/plans/2026-10-16-one-human-language-tool-orchestration-and-governance.md`
- `docs/superpowers/specs/2026-10-16-one-orchestration-implementation-contract.md`
- `docs/spec/one-v0.1-language-invariants.md`
- `docs/spec/one-v0.1-lexical.md`
- `docs/spec/one-v0.1-grammar.md`
- `docs/spec/one-v0.1-cli-contract.md`
- `docs/spec/one-network-foundation.md`
- `docs/spec/one-v0.1-conformance-suite.md`
- `docs/release/one-v0.1.md`
- `docs/verification/one-v0.1-verification.md`
- `docs/verification/one-v0.1-evidence-matrix.md`

## Verification state

Canonical source SHA `be2e65852e9a4be833e659524a12c6359769665b` passed independent Ubuntu and Windows Server 2025 verification run `38871435579`, including build, Clippy, workspace tests, real-network runtime tests, contract-artifact checks, offline verification, and deterministic `.oneir`/`.onepkg` checks.

A later canonical documentation commit must receive fresh exact-SHA verification before it is called verified. The private canonical GitHub repository remains the source of truth; the public verification mirror is verification infrastructure only.

No paid GitHub upgrade is required for this workflow.
