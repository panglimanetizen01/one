# One v0.1 — CLI Contract

**Status:** Approved normative contract

## 1. Purpose

The One CLI is the stable command-line entry point for project creation, checking, running, testing, building, packaging, version reporting, diagnostics, help, and explicitly governed runtime capabilities.

## 2. Command vocabulary

The primary user-facing vocabulary is conversational Indonesian. Technical aliases remain supported for compatibility and scripting.

| Primary command | Technical alias | Purpose |
|---|---|---|
| `one buat <folder>` | `one new <folder>` | create a project skeleton |
| `one periksa <file.one>` | `one check <file.one>` | validate source without executing it |
| `one jalankan <file.one>` | `one run <file.one>` | execute through the reference interpreter |
| `one test <folder>` | — | execute the project's main program and any sorted project-local conformance fixtures through the reference interpreter |
| `one bangun <file.one>` | `one build <file.one>` | lower source and write a deterministic `.oneir` artifact |
| `one package <folder>` | — | validate a project and write a deterministic `.onepkg` package artifact |
| `one lihat struktur <file.one>` | `one ast <file.one>` | print the parsed AST |
| `one version` | — | report tool/language version |
| `one help` | — | explain supported commands |

Command recognition is exact and bounded. The CLI does not interpret arbitrary natural language.

## 3. Explicit orchestration permission

Effectful orchestration remains opt-in. The CLI must not silently grant capabilities.

The currently exposed experimental source-level orchestration permission surfaces are:

### DNS discovery

```text
one jalankan <file.one> --izinkan network.discover --target <alamat>
```

Technical alias:

```text
one run <file.one> --allow network.discover --target <alamat>
```

### Filesystem read

```text
one jalankan <file.one> --izinkan filesystem.read --target <path>
```

Technical alias:

```text
one run <file.one> --allow filesystem.read --target <path>
```

### Filesystem write

```text
one jalankan <file.one> --izinkan filesystem.write --target <path>
```

Technical alias:

```text
one run <file.one> --allow filesystem.write --target <path>
```

The `tulis ke berkas "<path>" dengan "<isi>"` source operation maps to semantic capability `filesystem.write`, effect `Write`, and the governed `filesystem.write` adapter. The current boundary accepts one text payload for one explicitly targeted file. No permission means denial and a different target is outside the granted scope. This slice is **EXPERIMENTAL** and currently has exact-SHA Termux/Android evidence only.

### Process execution — experimental boundary

```text
one jalankan <file.one> --izinkan process.execute --target <program>
```

Technical alias:

```text
one run <file.one> --allow process.execute --target <program>
```

The `jalankan proses "<program>"` source operation maps to semantic capability `process.execute`, effect `Execute`, and the governed `process.execute` adapter. The current implementation executes the selected program without additional arguments and normalizes its stdout (or stderr when stdout is empty) as UTF-8 text. This slice is **EXPERIMENTAL** within the one-program/no-arguments boundary and has exact-SHA Ubuntu + Windows evidence at `9038ac75f197753ff1ab6af7478e63e4831903a2` via `38929072800`.

Rules for current orchestration permission surfaces:

- the capability must be explicitly granted;
- `--target` is mandatory when an orchestration capability is granted;
- the runtime policy is scoped to exactly the supplied target;
- no orchestration capability is granted when these flags are absent;
- a source operation targeting a different value is denied by governance;
- unknown capabilities are rejected as CLI usage errors;
- these permission syntaxes are controlled adapter boundaries, not One language grammar;
- the filesystem-read slice currently reads UTF-8 text and does not constitute general filesystem support;
- the filesystem-write slice currently writes one text payload to one explicitly targeted file and does not constitute general filesystem management or sandbox/path-confinement support;
- the process-execution slice currently executes one explicitly targeted program without arguments and does not constitute general process-management support;
- DNS and filesystem-read are experimental within their recorded evidence boundaries;
- filesystem write is experimental and currently verified only on Termux/Android;
- process execution is experimental within its recorded cross-platform boundary.

## 4. Exit status

The CLI exposes deterministic machine-observable success/failure status:

- `0` — requested command completed successfully;
- `1` — source, semantic, runtime, or build-domain failure;
- `2` — command-line usage or tool/environment failure.

Stable diagnostic codes identify the specific failure; prose is localized and is not the machine contract.

## 5. Output modes

Human-readable output is localized. A future machine-readable mode must provide stable diagnostic codes, severity, source spans, and structured fields without requiring prose parsing.

## 6. Project discovery

Commands operate on an explicit project root or an explicit source file. Project commands resolve `src/main.one` and `one.toml` deterministically. The CLI must not search arbitrary parent/sibling directories unpredictably.

## 7. Capability policy

Effectful execution loads an explicit runtime capability policy. The CLI must not silently grant host capabilities merely because a program requests an effect.

## 8. Offline behavior

Source checking, parsing, semantic analysis, tests that use deterministic local fixtures, and package/build operations that require no external resource must work offline. Commands requiring network effects must report that dependency explicitly.

## 9. Determinism

Command behavior must not depend on terminal history, shell aliases, editor state, filesystem enumeration order, or LLM output. Fixture discovery is sorted explicitly before execution.

## 10. Project scaffold

`one buat <folder>` and `one new <folder>` create:

```text
<folder>/
  one.toml
  src/
    main.one
```

The command refuses to overwrite an existing folder.

## 11. Package boundary

`one package` produces a `.onepkg` artifact containing the validated project manifest and deterministic v0.1 intermediate representation/debug representation. It is not yet a platform installer or signed distribution package.

## 12. Acceptance criteria

- stable primary and compatibility command names;
- clear separation between check/run/test/build/package;
- deterministic exit status;
- structured diagnostics;
- explicit capability policy;
- exact target scoping for the current CLI orchestration permission surfaces;
- offline validation for source-only operations;
- Windows/Linux support through the platform abstraction contract;
- no LLM or network dependency for command interpretation or language semantics.
