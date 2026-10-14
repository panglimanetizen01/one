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

The canonical repository contains the first executable language core:

```text
Localized source
    -> Lexer
    -> AST parser
    -> Semantic analyzer
    -> One Semantic IR
    -> Reference interpreter
    -> CLI
```

The v0.1 CLI provides:

```text
one new <folder>
one check <file.one>
one run <file.one>
one build <file.one>
one ast <file.one>
```

The implementation includes the six core constructs, structured diagnostics, a capability model, explicit process interoperability, project scaffolding, and conformance/golden fixtures.

`one build` currently emits a deterministic `.oneir` IR/debug artifact. It is not yet a native executable compiler.

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
- `docs/spec/one-v0.1-lexical.md`
- `docs/spec/one-v0.1-grammar.md`
- `docs/spec/one-v0.1-ast.md`
- `docs/testing/conformance.md`
- `docs/release/one-v0.1.md`
- `docs/verification/one-v0.1-evidence-matrix.md`

## Verification state

The source, contracts, fixtures, and offline verification paths are maintained in the canonical repository. GitHub-hosted CI has a documented runner blocker: recent Linux and Windows jobs failed before usable step/log evidence was available. That means the project must **not** claim CI-green or fully verified status until actual execution evidence exists.

If hosted GitHub runners require an account upgrade, do not purchase one merely for this project. Use `tools/verify_offline.sh` on Linux or `tools/verify_offline.ps1` on Windows, record the exact toolchain/commit/results, and keep hosted-CI status explicitly marked as blocked.

## Canonical source

This GitHub repository is the canonical source of truth for One. Local copies are verification/validation environments and are not the primary source of changes.
