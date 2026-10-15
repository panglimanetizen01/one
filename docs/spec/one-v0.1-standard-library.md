# One v0.1 — Standard Library Boundary

**Status:** Draft normative contract for review

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

## 4. Capability enforcement

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