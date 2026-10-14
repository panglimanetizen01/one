# One v0.1 CLI Contract

## Commands

| Command | Purpose | Success | Failure |
|---|---|---:|---:|
| `one new <folder>` | create a new project scaffold | `0` | `2` |
| `one check <file.one>` | parse and semantic-check source | `0` | `1` |
| `one run <file.one>` | execute through the reference interpreter | `0` | `1` |
| `one build <file.one>` | lower source and write a `.oneir` IR/debug artifact | `0` | `1` or `2` |
| `one ast <file.one>` | print the parsed AST for inspection | `0` | `1` |

## Determinism

- Commands never invoke an LLM.
- `check`, `run`, `build`, and `ast` do not rewrite the source file.
- `build` output is derived from the parsed/typed program and is not a native executable in v0.1.
- Diagnostic error codes are stable within the v0.1 contract.

## Project scaffold

`one new <folder>` creates:

```text
<folder>/
  one.toml
  src/
    main.one
```

The command refuses to overwrite an existing folder.

## Verification

CLI integration tests live under `crates/one-cli/tests/`. The portable offline verifier additionally exercises `new`, `check`, `run`, `build`, and `ast` in a clean temporary project.
