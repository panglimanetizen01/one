# One v0.1 — Specification Gate Workflow

**Status:** Draft governance record

## Canonical rule

GitHub is the canonical source of truth for One. Local and Termux environments are verification/validation environments only.

## Technical dependency order

1. Language Invariants
2. Lexical Specification
3. Concrete Grammar
4. AST Contract
5. Scope and name resolution
6. Type System
7. Semantic Model
8. Common Language IR
9. Reference Interpreter
10. Conformance and golden tests
11. Compiler/backend
12. Runtime
13. CLI and developer tooling
14. Standard Library
15. Package/dependency system
16. Windows platform layer
17. Linux platform layer
18. Interoperability layer
19. Localization architecture
20. Release verification

## Gate rule

A stage is not complete because implementation code exists or a simple example passes.

A stage is complete only when its specification, valid/invalid examples, ambiguity rules, edge cases, acceptance criteria, and appropriate conformance tests are defined and reviewed.

Implementation behavior cannot silently promote itself into the language contract.

## Conflict rule

If an existing implementation conflicts with an approved specification, stop and audit the conflict. Do not modify the specification merely to make an existing implementation pass.

## Local verification rule

Local/Termux verification begins only after the relevant canonical GitHub state is stable enough to justify independent verification. Local changes must not become the source of canonical changes.
