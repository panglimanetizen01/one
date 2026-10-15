# One v0.1 Release Contract

Status: **VERIFIED** for the canonical source lineage verified by independent Linux and Windows execution.

## Scope

One v0.1 is the Indonesian-first reference implementation. Its purpose is to establish a deterministic language core and a usable command-line execution path without requiring an LLM or another programming language at runtime.

## Supported in v0.1

- DATA: declaration and assignment.
- ACTION: display, list creation, list mutation, and greeting through explicit productions.
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

- Names are grammar-bounded and may contain multiple words. Structural words are interpreted according to the active production; the parser does not globally reserve them.
- The conversational loop form `selama masih ada ...` is not part of v0.1; only explicit boolean conditions are accepted.
- `build` currently emits a deterministic `.oneir` IR artifact; it is not a native executable compiler.
- `.onepkg` is a deterministic intermediate/debug package boundary; it is not a platform installer or signed distribution package.
- Native C/C++ FFI is future work.
- Python, Node.js, and Java/JVM integration is currently process-boundary interoperability, not native language bindings.
- Package registry, full dependency registry, IDE/editor tooling, GUI framework integration, and additional locale grammars are future work.

## Release acceptance checklist

The independently verified source execution used canonical implementation commit `d17f2829187ec28031a4b4a94dd0e0af282f66b8`.

- [x] Linux compilation succeeds.
- [x] Windows compilation succeeds.
- [x] Linux unit/integration/conformance tests succeed.
- [x] Windows unit/integration/conformance tests succeed.
- [x] CLI smoke tests succeed through the platform-specific offline verification scripts.
- [x] Capability/security tests succeed.
- [x] Interoperability tests preserve exit status/stdout/stderr and reject denied capabilities.
- [x] Documentation and grammar are aligned with the implemented multi-word name and explicit-action behavior.
- [x] Regression tests cover multi-word loop variables and function parameters.
- [x] No known failing test remains in the recorded source verification run.

## Evidence

Independent verification run: `38753437740` in `panglimanetizen01/one-windows-verification`.

- Linux job: `116485529987` — success.
- Windows Server 2025 job: `116485530381` — success.

Both jobs checked out the canonical private repository at the exact implementation commit above and completed contract checks, build, Clippy, all workspace tests, and the platform-specific offline verification script.

The temporary public repository is execution infrastructure only. The private canonical repository remains the source of truth.

The private canonical GitHub Actions environment remains separately recorded as blocked because its hosted jobs historically failed before usable workflow-step evidence. No paid GitHub upgrade was purchased or required.

If GitHub-hosted runners require an account upgrade, **do not purchase an upgrade merely to satisfy this contract**; use the portable offline verification path instead.

## Release labels

- **IMPLEMENTATION-COMPLETE:** all in-scope source/docs/tests are present in the canonical repository.
- **VERIFICATION-PENDING:** required execution evidence is incomplete.
- **VERIFIED:** all acceptance checklist items have actual recorded evidence.

Current state: **VERIFIED**.
