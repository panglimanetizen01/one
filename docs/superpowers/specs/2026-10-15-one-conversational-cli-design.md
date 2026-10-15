# One Conversational CLI Design

**Status:** Design approved in conversation; written-spec review required before implementation.

## Intent

The One command-line interface should be understandable to ordinary users who are not programmers. Operating One should read like a short instruction from a manager to a colleague: direct, everyday Indonesian, and focused on the requested work rather than programming terminology.

## Design

The primary v0.1 command vocabulary is:

| User instruction | Operation |
|---|---|
| `one buat <folder>` | create a new One project |
| `one jalankan <file.one>` | check, lower, and execute a source file with the reference interpreter |
| `one periksa <file.one>` | parse and semantically validate a source file without executing it |
| `one bangun <file.one>` | produce the deterministic `.oneir` build/debug artifact |
| `one lihat struktur <file.one>` | print the parsed AST for inspection |

Examples:

```text
one buat toko
one jalankan toko.one
one periksa toko.one
one bangun toko.one
one lihat struktur toko.one
```

## Compatibility

The existing technical command vocabulary remains accepted as aliases in v0.1:

```text
one new <folder>
one run <file.one>
one check <file.one>
one build <file.one>
one ast <file.one>
```

The conversational vocabulary is the documented/default user-facing form. Aliases must resolve to exactly the same operation, exit-code behavior, determinism rules, and side effects.

## User-facing language

CLI status and error messages should also use ordinary Indonesian where practical. Messages must remain precise and must not hide diagnostic codes, source locations, or actionable information.

Preferred examples:

```text
One sedang menjalankan toko.one
Selesai. Tidak ada kesalahan.
```

For an invalid program:

```text
One menemukan masalah di baris 8.
Program belum dijalankan karena bagian ini perlu diperbaiki.
E....: <diagnostic message>
```

The exact wording of operational messages may be refined during implementation, but stable diagnostic codes and source locations remain part of the machine-verifiable contract.

## Constraints

- No LLM is required to interpret CLI commands.
- Command recognition remains deterministic.
- Existing file safety rules remain unchanged: source is never silently rewritten.
- `bangun` continues to produce `.oneir`; it does not imply native executable generation in v0.1.
- Existing exit codes remain unchanged.
- Windows and Linux must expose the same command vocabulary and semantics.
- The conversational syntax must not make ambiguous commands silently select an operation.

## Acceptance criteria

1. Every primary v0.1 CLI operation has an everyday Indonesian command.
2. Existing technical commands remain functional as compatibility aliases.
3. Both forms produce equivalent results and exit codes.
4. Help/usage output presents the conversational vocabulary first.
5. CLI tests cover the conversational commands and alias equivalence.
6. Offline Linux and Windows verification exercise the conversational commands.
7. Documentation, examples, and derivative CLI artifacts agree with this contract.

## Non-goals

This design does not change One's programming-language grammar, parser grammar, AST, runtime semantics, or project file format. It changes only the human-facing command vocabulary and associated CLI messaging.
