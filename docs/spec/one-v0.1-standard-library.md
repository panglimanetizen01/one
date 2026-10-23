# One v0.1 — Standard Library Boundary

**Status:** Approved normative contract

## 1. Purpose

The standard library provides reusable One-level APIs without becoming a hidden runtime or capability bypass.

## 2. Layering

Standard library modules are built on the language semantic model, CSIR/runtime operations, and explicit capability effects. They do not replace the compiler, parser, or runtime.

## 3. Initial domains

The v0.1 boundary reserves domains for:

- text;
- collections;
- math/basic numeric operations;
- console;
- filesystem;
- process;
- networking;
- time;
- randomness;
- testing utilities.

Only domains whose semantic/effect contracts are already defined may be implemented as stable v0.1 APIs.

## 4. Current deterministic host-helper implementation

The `one_core::stdlib` Rust module currently exposes the following pure helper operations for runtime integration and tests:

- `display_value`: stable display representation of a runtime value.
- `text_length`, `text_contains`, `text_trim`, `text_replace`, `text_split`, and `text_to_integer`: deterministic text operations. Length counts Unicode scalar values, not grapheme clusters; split/replace use literal separators/needles; empty split separators and invalid input types return `None`.
- `list_length` and `list_get`: checked list access with zero-based indexes; negative and out-of-range indexes fail.
- `map_get`, `map_set`, and `map_from_entries`: text-keyed mappings backed by ordered keys for deterministic display.
- `number_abs`, `number_divide`, `number_min`, and `number_max`: checked integer helpers; overflow and division by zero are rejected.

These are currently public Rust library helpers with module unit tests. **They are not yet all callable as built-ins from `.one` source programs and do not, by themselves, satisfy the standard-library completion gate.** Runtime/source-level registration, structured language diagnostics for helper failures, and cross-backend conformance remain required before the corresponding API can be marked fully supported.

## 5. Capability enforcement

Effectful library calls retain their declared capability requirements. Wrapping `filesystem.read` in a library function cannot bypass or erase that requirement.

## 5. Determinism

Pure functions must be deterministic. Effectful functions explicitly identify external inputs/effects. Time and randomness use controlled providers in deterministic test mode.

## 6. Error model

Library failures use structured One diagnostics/runtime errors. Host exception types are not exposed as the stable API.

## 7. Versioning

Public library modules and functions have stable names and documented signatures. Breaking changes require a versioned language/library contract change.

## 8. Localization

Library semantic APIs are localization-independent. Surface names and documentation may be localized without changing their operation identity.

## 9. Acceptance criteria

- no capability bypass;
- no hidden global mutable state;
- deterministic pure operations;
- explicit effects;
- stable errors and signatures;
- compatibility with reference interpreter and compiler backends.
