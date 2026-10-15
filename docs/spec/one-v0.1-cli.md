# One v0.1 CLI Contract

## Commands

The primary command vocabulary is conversational Indonesian. It is intentionally phrased like a direct everyday instruction from a supervisor to a coworker.

| Conversational command | Technical alias | Purpose | Success | Failure |
|---|---|---|---:|---:|
| `one buat <folder>` | `one new <folder>` | create a new project scaffold | `0` | `2` |
| `one periksa <file.one>` | `one check <file.one>` | parse and semantic-check source | `0` | `1` |
| `one jalankan <file.one>` | `one run <file.one>` | execute through the reference interpreter | `0` | `1` |
| `one bangun <file.one>` | `one build <file.one>` | lower source and write a `.oneir` IR/debug artifact | `0` | `1` or `2` |
| `one lihat struktur <file.one>` | `one ast <file.one>` | print the parsed AST for inspection | `0` | `1` |

The conversational vocabulary is the documented primary interface. Technical aliases remain supported for compatibility and scripting.

## Everyday-language rule

- Prefer Indonesian everyday verbs over programming jargon in user-facing command help.
- A conversational command must preserve the behavior, exit status, and output of its technical alias.
- `lihat struktur` is intentionally two words because it reads naturally as an instruction while remaining unambiguous.
- The CLI does not interpret arbitrary natural language. Only the bounded command vocabulary defined by this contract is accepted.

## Determinism

- Commands never invoke an LLM.
- `check`, `run`, `build`, and `ast` do not rewrite the source file.
- `build` output is derived from the parsed/typed program and is not a native executable in v0.1.
- Diagnostic error codes are stable within the v0.1 contract.

## Project scaffold

`one buat <folder>` and its technical alias create:

```text
<folder>/
  one.toml
  src/
    main.one
```

The command refuses to overwrite an existing folder.

## Verification

CLI integration tests live under `crates/one-cli/tests/`. The conversational command tests verify help visibility, project creation, and behavior equivalence with the technical aliases. The portable offline verifier additionally exercises the CLI in a clean temporary project.
