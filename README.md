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

Mulai dari **[Panduan Pengguna One](docs/panduan-pengguna/README.md)**. Panduan tersebut menjelaskan langkah penggunaan dengan bahasa yang lebih sederhana dan secara eksplisit membedakan fitur yang sudah tersedia dari fitur yang masih dikembangkan.

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

The command vocabulary is bounded and deterministic; One does not interpret arbitrary natural-language instructions as CLI commands.

The implementation includes the six core constructs, structured diagnostics, a capability model, explicit process interoperability, project scaffolding, strict v0.1 manifest validation, deterministic `.oneir`/`.onepkg` artifacts, and conformance/golden fixtures. `one test` can also execute sorted project-local conformance fixtures when present.

The v0.1 grammar and parser support grammar-bounded multi-word names, explicit action productions including `sapa`, and production-scoped structural boundaries. Unsupported arbitrary first-word actions are rejected rather than interpreted implicitly.

`one bangun` currently emits a deterministic `.oneir` IR/debug artifact. `one package` emits a validated deterministic `.onepkg` artifact. Neither is yet a native executable compiler or platform installer.

## One Network foundation

The first One-owned network vertical slice is implemented and independently verified as a typed semantic model with explicit address, transport, security, protocol-family, target, timeout, connection, and connect/send/receive operation types. Network authority is granular (`NetworkConnect`, `NetworkListen`, `NetworkDiscover`, `NetworkAdmin`) and the existing coarse `Network` capability remains a compatibility grant without implying administrative authority.

The network foundation also provides deterministic typed network IR, a protocol compatibility registry, structured network diagnostics, and a reference-interpreter boundary that validates and checks capabilities without performing external network I/O. The registry does not make protocol names into language keywords and does not imply that real DNS/socket/HTTP/QUIC adapters are implemented.

Verified checkpoint: `d05eab5a103b63949e1dfaa9b97a479115c27a8c`. Independent Linux and Windows Server 2025 verification run: `38763329481`.

See `docs/spec/one-network-foundation.md` for the exact implemented boundary and explicit unsupported scope.

## Example

```text
nama adalah "Budi"
umur adalah 18

kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"
kalau tidak
    tampilkan "Belum boleh masuk"
```

## Documentation

User-facing documentation:

- `docs/panduan-pengguna/README.md` — beginner entry point and documentation rules.
- `docs/panduan-pengguna/status-fitur.md` — explicit capability status vocabulary.
- `docs/README.md` — complete engineering documentation map.

Normative engineering documents:

- `docs/superpowers/specs/2026-10-15-one-language-foundation-design.md`
- `docs/superpowers/specs/2026-10-15-one-end-to-end-network-platform-design.md`
- `docs/spec/one-v0.1-language-invariants.md`
- `docs/spec/one-v0.1-lexical.md`
- `docs/spec/one-v0.1-grammar.md`
- `docs/spec/one-v0.1-ast.md`
- `docs/spec/one-v0.1-cli-contract.md`
- `docs/spec/one-network-foundation.md`
- `docs/spec/one-v0.1-conformance-suite.md`
- `docs/release/one-v0.1.md`
- `docs/verification/one-v0.1-linux-windows-2026-10-15.md`

## Verification state

The v0.1 source, contracts, fixtures, README, changelog, and offline verification paths are maintained in the canonical repository. The current One Network checkpoint is independently verified on Linux and Windows Server 2025 through the temporary public verification mirror, without purchasing a GitHub upgrade.

The audit also closed the previously identified parser/contract mismatches: explicit action boundaries, the valid `sapa` action, multi-word loop/function names, and production-scoped structural-word boundaries are now covered by regression tests and aligned documentation.

Earlier private hosted-runner provisioning failures remain historical evidence and are not relabeled as green. The independent verification run is the evidence for the exact network checkpoint recorded above.

If GitHub-hosted runners require an account upgrade, do not purchase one merely for this project. Use `tools/verify_offline.sh` on Linux or `tools/verify_offline.ps1` on Windows and record exact execution evidence.

## Canonical source

This GitHub repository is the canonical source of truth for One. Local copies are verification/validation environments and are not the primary source of changes.
