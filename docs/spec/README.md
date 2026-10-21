# One v0.1 Specification Index

This index records the dependency order and review state of the language specification. **A file existing in the repository does not mean its contract has been approved or implemented.** Each gate must be explicitly reviewed against its dependencies and acceptance tests.

## Dependency-ordered gates

1. [Language Invariants](./one-v0.1-language-invariants.md) — accepted normative foundation constraints; changes require explicit review.
2. [Lexical Specification](./one-v0.1-lexical.md) — normative for the initial Indonesian v0.1 implementation; implementation conformance is tracked separately.
3. [Concrete Grammar](./one-v0.1-grammar.md) — normative for the initial Indonesian v0.1 implementation; implementation conformance is tracked separately.
4. [AST Contract](./one-v0.1-ast.md) — accepted for the v0.1 implementation baseline; its documented source-span limitations are normative.
5. Name resolution and scope — must preserve grammar-defined name boundaries and ambiguity rejection.
6. Type system.
7. Semantic model.
8. Common One IR.
9. Reference interpreter.
10. Conformance and golden suite.
11. Compiler/backend.
12. Runtime and effect/capability integration.
13. CLI and developer tooling.
14. Standard library.
15. Module, package, and build systems.
16. Windows platform layer.
17. Linux platform layer.
18. Interoperability layer.
19. Release verification.

## Cross-cutting contracts

These concerns must be designed and checked alongside the stages they constrain; they are not deferred until the end:

- **Localization:** [Localization overview](./one-v0.1-localization.md) and [detailed localization contract](./one-v0.1-localization-contract.md). The overview is architectural guidance; the detailed contract remains a draft until explicitly approved. Locale-specific grammar must converge on the shared semantic model.
- **Diagnostics:** stable codes, source spans, and deterministic failure behavior.
- **Effects/capabilities:** privileged operations require explicit authorization.
- **Platform abstraction:** platform differences must not silently alter platform-independent semantics.
- **Conformance:** fixtures are acceptance evidence, not substitutes for specification review.

## Review policy

- Do not mark a gate complete solely because its file exists.
- Do not weaken a contract to match an implementation without an explicit specification decision.
- When implementation and contract disagree, record the mismatch, add a failing regression test where practical, and change the implementation to satisfy the approved contract.
- If two contracts conflict, stop implementation of the affected area until the conflict is resolved and recorded.
- A status such as Verified or Supported requires executable evidence tied to a specific commit and environment.

The exact repository directory layout remains subject to architecture review. Specification status and implementation-conformance status are separate: an accepted or normative contract does not, by itself, prove that every required behavior is implemented or verified.
