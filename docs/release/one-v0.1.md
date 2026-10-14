# One v0.1 Release Contract

Status: **implementation contract complete; release verification remains evidence-gated**.

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
- CLI commands `new`, `check`, `run`, `build`, and `ast`.
- Conformance fixtures and golden-output tests.
- Linux/Windows CI definition.

## Explicit v0.1 limits

- Identifiers are single lexical words. Multi-word identifiers are not silently reconstructed.
- The conversational loop form `selama masih ada ...` is not part of v0.1; only explicit boolean conditions are accepted.
- `build` currently emits a deterministic `.oneir` IR/debug artifact; it is not a native executable compiler.
- Native C/C++ FFI is future work.
- Python, Node.js, and Java/JVM integration is currently process-boundary interoperability, not native language bindings.
- Package registry, IDE/editor tooling, GUI framework integration, and additional locale grammars are future work.

## Release acceptance checklist

A release may be labeled **verified** only when every item below has evidence:

- [ ] Linux compilation succeeds.
- [ ] Windows compilation succeeds.
- [ ] Linux unit/integration/conformance tests succeed.
- [ ] Windows unit/integration/conformance tests succeed.
- [ ] CLI smoke tests succeed on Linux and Windows.
- [ ] Capability/security tests succeed.
- [ ] Interoperability tests preserve exit status/stdout/stderr and reject denied capabilities.
- [ ] Documentation matches the implementation and no contradictory grammar remains.
- [ ] No known failing test remains.

If GitHub-hosted runners require an account upgrade or otherwise cannot execute, **do not purchase an upgrade merely to satisfy this contract**. Use the repository's portable offline verification scripts on independent Linux and Windows environments, record their exact evidence, and keep the GitHub-hosted CI status explicitly marked as blocked rather than falsely marking it green. Independent verification is evidence for execution correctness; it does not rewrite the historical CI result.

## Release labels

- **IMPLEMENTATION-COMPLETE:** all in-scope source/docs/tests are present in the canonical repository.
- **VERIFICATION-PENDING:** required execution evidence is incomplete.
- **VERIFIED:** all acceptance checklist items have actual recorded evidence.

Current state is `IMPLEMENTATION-COMPLETE + VERIFICATION-PENDING` until Linux and Windows execution evidence is recorded.
