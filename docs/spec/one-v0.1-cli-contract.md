# One v0.1 — CLI Contract

**Status:** Approved normative contract

## 1. Purpose

The One CLI is the stable command-line entry point for project creation, checking, running, testing, building, packaging, version reporting, diagnostics, and help.

## 2. Command vocabulary

The primary user-facing vocabulary is conversational Indonesian. Technical aliases remain supported for compatibility and scripting.

| Primary command | Technical alias | Purpose |
|---|---|---|
| `one buat <folder>` | `one new <folder>` | create a project skeleton |
| `one periksa <file.one>` | `one check <file.one>` | validate source without executing it |
| `one jalankan <file.one>` | `one run <file.one>` | execute through the reference interpreter |
| `one test <folder>` | — | validate and execute the project's main program through the reference interpreter |
| `one bangun <file.one>` | `one build <file.one>` | lower source and write a `.oneir` artifact |
| `one package <folder>` | — | validate a project and write a `.onepkg` package artifact |
| `one lihat struktur <file.one>` | `one ast <file.one>` | print the parsed AST |
| `one version` | — | report tool/language version |
| `one help` | — | explain supported commands |

Command recognition is exact and bounded. The CLI does not interpret arbitrary natural language.

## 3. Exit status

The CLI exposes deterministic machine-observable success/failure status:

- `0` — requested command completed successfully;
- `1` — source, semantic, runtime, or build-domain failure;
- `2` — command-line usage or tool/environment failure.

Stable diagnostic codes identify the specific failure; prose is localized and is not the machine contract.

## 4. Output modes

Human-readable output is localized. A future machine-readable mode must provide stable diagnostic codes, severity, source spans, and structured fields without requiring prose parsing.

## 5. Project discovery

Commands operate on an explicit project root or an explicit source file. Project commands resolve `src/main.one` and `one.toml` deterministically. The CLI must not search arbitrary parent/sibling directories unpredictably.

## 6. Capability policy

Effectful execution loads an explicit runtime capability policy. The CLI must not silently grant host capabilities merely because a program requests an effect.

## 7. Offline behavior

Source checking, parsing, semantic analysis, tests that use deterministic local fixtures, and package/build operations that require no external resource must work offline. Commands requiring network effects must report that dependency explicitly.

## 8. Determinism

Command behavior must not depend on terminal history, shell aliases, editor state, filesystem enumeration order, or LLM output.

## 9. Project scaffold

`one buat <folder>` and `one new <folder>` create:

```text
<folder>/
  one.toml
  src/
    main.one
```

The command refuses to overwrite an existing folder.

## 10. Package boundary

`one package` produces a `.onepkg` artifact containing the validated project manifest and deterministic v0.1 intermediate representation/debug representation. It is not yet a platform installer or signed distribution package.

## 11. Acceptance criteria

- stable primary and compatibility command names;
- clear separation between check/run/test/build/package;
- deterministic exit status;
- structured diagnostics;
- explicit capability policy;
- offline validation for source-only operations;
- Windows/Linux support through the platform abstraction contract;
- no LLM or network dependency for command interpretation or language semantics.
