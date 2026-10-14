# One v0.1 Conformance Contract

## Purpose

The conformance suite is the executable derivative of the normative language contracts. It proves that accepted source parses/analyzes/lowers deterministically and that rejected source fails at the expected boundary.

## Fixture layout

```text
tests/conformance/valid/*.one
 tests/conformance/invalid/*.one
 tests/golden/*.snap
```

The Rust integration runner is located at `crates/one-core/tests/conformance.rs` because the repository root is a virtual Cargo workspace and root-level `tests/` files are not automatically compiled as integration tests.

## Required fixture classes

| Class | Required evidence |
|---|---|
| DATA | declaration and assignment |
| ACTION | display and list operations |
| CONDITION | true/false branches and comparison phrases |
| LOOP | `untuk setiap` and explicit `selama` condition |
| FUNCTION | parameters, explicit result, call |
| EVENT | registration and exact dispatch |
| LEXICAL | invalid character, bad indentation, malformed string |
| SYNTAX | missing expression/operator, malformed block |
| SEMANTIC | undefined name, duplicate declaration, type mismatch |
| SECURITY | denied process capability |
| INTEROP | preserved exit status/stdout/stderr where platform command is available |

## Determinism rules

1. A valid fixture must produce the same AST/IR structure on repeated runs.
2. An invalid fixture must fail with a stable diagnostic code, not merely a text-match coincidence.
3. Fixtures must not depend on wall-clock time, network access, random values, or an LLM.
4. Platform-specific fixtures must have a portable fallback or be explicitly marked platform-scoped.
5. The suite must never modify its source fixtures.

## Golden artifacts

Golden `.snap` files contain stable, intentionally human-readable representations of selected AST/IR contracts. They are reviewed as contract artifacts, not as opaque generated blobs. A golden change is a language-contract change and must be explained in the same commit.

## Release gate

The conformance suite is one required input to the v0.1 release gate. Passing the suite does not by itself prove Windows/Linux portability; those claims require platform execution evidence.
