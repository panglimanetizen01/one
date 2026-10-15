# One v0.1 Release Contract

Status: **VERIFIED for canonical commit `69c4918e6845fdbbd79495b3fa2c59798831bce0`**.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Supported in v0.1

- DATA: declaration and assignment.
- ACTION: display, list creation, list mutation, greeting, and explicit standard-library actions.
- CONDITION: deterministic comparison phrases and `kalau tidak` branch.
- LOOP: `untuk setiap` and explicit boolean `selama` conditions.
- FUNCTION: parameters, explicit `hasilnya adalah` result, and calls using `dengan` / `dan`.
- EVENT: exact trigger registration and dispatch.
- Indonesian lexical and grammar contracts.
- Language-independent AST and Semantic IR.
- Reference interpreter.
- Structured diagnostics with source spans and stable error codes.
- Capability-aware process boundary.
- Cross-platform process API shape using argument vectors and preserved exit status/stdout/stderr.
- CLI commands `new`, `check`, `run`, `test`, `build`, `package`, `version`, and `ast`, with Indonesian conversational primary aliases.
- `one test` executes `src/main.one` and, when present, sorted deterministic `tests/conformance/valid/*.one` and `tests/conformance/invalid/*.one` fixtures.
- Deterministic `.oneir` and `.onepkg` textual artifacts.
- Strict v0.1 project manifest validation for `nama` and `versi`.
- Conformance fixtures and golden-output tests.
- Linux/Windows CI definition.

## Explicit v0.1 limits

- Identifiers are single lexical words. Multi-word identifiers are not silently reconstructed.
- The conversational loop form `selama masih ada ...` is not part of v0.1; only explicit boolean conditions are accepted.
- `build` currently emits a deterministic `.oneir` IR artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- Native C/C++ FFI is future work.
- Python, Node.js, and Java/JVM integration is currently process-boundary interoperability, not native language bindings.
- Package registry, full dependency registry, IDE/editor tooling, GUI framework integration, and additional locale grammars are future work.

## Release acceptance checklist

For canonical commit `69c4918e6845fdbbd79495b3fa2c59798831bce0`, every release condition has recorded evidence:

- [x] Linux compilation succeeds.
- [x] Windows compilation succeeds.
- [x] Linux unit/integration/conformance tests succeed.
- [x] Windows unit/integration/conformance tests succeed.
- [x] CLI smoke tests succeed through the platform-specific offline verification scripts.
- [x] Capability/security tests succeed.
- [x] Interoperability tests preserve exit status/stdout/stderr and reject denied capabilities.
- [x] Documentation matches the implementation and no contradictory grammar remains within the defined v0.1 scope.
- [x] No known failing test remains in the recorded verification runs.

## Evidence

The exact Linux and Windows execution evidence is recorded in `docs/verification/one-v0.1-linux-windows-2026-10-15.md`.

The verification used a temporary public GitHub repository solely as independent execution infrastructure. It checked out the private canonical repository at the exact commit above. The private canonical GitHub Actions environment remains historically blocked before usable steps; that status is not relabeled as green.

No GitHub paid upgrade was purchased or required.

## Release labels

- **IMPLEMENTATION-COMPLETE:** all in-scope source/docs/tests are present in the canonical repository.
- **VERIFICATION-PENDING:** required execution evidence is incomplete.
- **VERIFIED:** all acceptance checklist items have actual recorded evidence.

Current state: **VERIFIED** for the recorded canonical commit.
