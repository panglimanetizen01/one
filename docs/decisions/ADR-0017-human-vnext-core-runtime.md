# ADR-0017 — Human frontend integration, modules, mappings, and persistence

**Status:** Accepted for the experimental core/runtime slice; not a claim that the complete native vNext grammar is normative.

## Context

The vNext milestone requires ordinary programs to reach ONE's existing deterministic semantic and execution path, without creating a parallel evaluator or making AI part of program semantics. The slice also needs multi-file composition, a basic mapping collection, and a reusable storage primitive while preserving v0.1 compatibility.

## Decisions

1. `Buat program <nama>` is an explicit header for the human-core CLI. The body is passed through the canonical parser, semantic analyzer, One IR lowering, and reference interpreter. No vNext-only evaluator is introduced.
2. The opt-in command is `one jalankan-human-core <program.one>` (alias `one run-human-core`). Default v0.1 command routing does not auto-detect or reinterpret this format.
3. A project may use `Gunakan modul "relative.one"` immediately after its program header. A module begins with `Buat modul <nama>`. Only relative `.one` paths inside the project directory are accepted; traversal, absolute paths, external symlinks, duplicate imports, and import cycles are rejected.
4. In this slice, imported top-level declarations are visible project-wide and share one semantic namespace. Duplicate names are errors. Per-module private/export visibility and source-map preservation for imported-file diagnostics are deferred and must not be implied by documentation.
5. `Type::Map` and `Value::Map` use deterministic key ordering. Keys are text; values can be any supported ONE value. Duplicate-key insertion overwrites with the newest value; missing-key lookup/removal is an explicit runtime error.
6. `one_core::persistence::KeyValueStore` uses a versioned `ONE-KV-1` format, deterministic hex encoding, bounded entry/field/file sizes, corruption rejection, and temp-file replacement. The existing task-list application's older storage format is not silently migrated or replaced by this API.

## Compatibility consequences

- v0.1 syntax and command routing remain unchanged.
- The new frontend and module loader are opt-in and experimental.
- Unsupported constructs fail explicitly; examples only document tested behavior.
- A future change to module visibility, source mapping, persistence migration, or the native vNext grammar requires a separate compatibility decision and tests.
