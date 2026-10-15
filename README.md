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

The implementation includes the six core constructs, structured diagnostics, a capability model, explicit process interoperability, project scaffolding, package/build boundaries, and conformance/golden fixtures.

`one bangun` currently emits a deterministic `.oneir` IR/debug artifact. `one package` emits a validated `.onepkg` artifact. Neither is yet a native executable compiler or platform installer.

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

Start with `docs/README.md`, then read:

- `docs/superpowers/specs/2026-10-15-one-language-foundation-design.md`
- `docs/spec/one-v0.1-language-invariants.md`
- `docs/spec/one-v0.1-lexical.md`
- `docs/spec/one-v0.1-grammar.md`
- `docs/spec/one-v0.1-ast.md`
- `docs/spec/one-v0.1-cli-contract.md`
- `docs/spec/one-v0.1-conformance-suite.md`
- `docs/release/one-v0.1.md`
- `docs/verification/one-v0.1-evidence-matrix.md`

## Verification state

The source, contracts, fixtures, and offline verification paths are maintained in the canonical repository. GitHub-hosted CI has a documented runner blocker: recent Linux and Windows jobs failed before usable step/log evidence was available. The project therefore must **not** claim CI-green or fully verified status until actual execution evidence exists.

If hosted GitHub runners require an account upgrade, do not purchase one merely for this project. Use `tools/verify_offline.sh` on Linux or `tools/verify_offline.ps1` on Windows, record the exact toolchain/commit/results, and keep hosted-CI status explicitly marked as blocked.

## Canonical source

This GitHub repository is the canonical source of truth for One. Local copies are verification/validation environments and are not the primary source of changes.
