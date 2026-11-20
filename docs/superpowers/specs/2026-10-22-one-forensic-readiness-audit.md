# ONE Forensic Readiness Audit — Standalone Human-Oriented Language

**Audit date:** 2026-10-22  
**Scope:** Repository contracts on `main`, current v0.1 source architecture, native-backend evidence, and the human-instruction vNext prototype branch.  
**Status:** Evidence-based gap audit; not a declaration that ONE is complete.

## Executive finding

The intended product direction is already explicit in the repository's accepted constitution, requirements, architecture contract, decisions, and implementation plan. The project is not missing the ambition or the architectural principle. It has a real Rust workspace, a deterministic v0.1 frontend, a language-independent AST and Semantic IR, a reference interpreter, capability-governed adapters, SQLite adapter support, interoperability boundaries, and native backend work with bounded verification evidence.

However, the repository has **not yet demonstrated that the intended punctuation-light human-instruction language can independently build complete small-to-large projects from end to end**. The current default CLI is v0.1 and its examples still require quoted text and conventional constructs. The new human-oriented parser is an experimental isolated module, not the CLI's compilation frontend. It only accepts a handful of declarations and a display action. It has no semantic lowering into the existing IR, no executable action dispatch for its own AST, no task collection model, and no persistence implementation for its own syntax. The full task-list source remains an acceptance target rather than a working end-to-end fixture.

This is therefore an **implementation maturity gap**, not evidence that the original vision was absent.

## Evidence hierarchy and limits

Repository contracts reviewed on `main` include:

- `AGENTS.md`: source-of-truth hierarchy and DISCOVER → EXECUTE → VERIFY → FIX → REVERIFY → DOCUMENT → CONTINUE.
- `docs/ONE_CONSTITUTION.md`: human-oriented, formally deterministic general-purpose language; not an LLM wrapper.
- `docs/REQUIREMENTS.md`: general-purpose, localization, portability, standalone-project, and interoperability requirements.
- `docs/DECISIONS.md`: accepted semantic IR, deterministic semantics, capability governance, and portability decisions.
- `docs/ARCHITECTURE.md`, `docs/COMPATIBILITY.md`, `docs/INTEROPERABILITY.md`, and `docs/SECURITY.md`.
- `crates/one-core/src/{ast,semantic,ir,interpreter,adapter,database}.rs`.
- The existing native backend verification record, which explicitly bounds claims by exact source revision and target.

The experimental parser and its draft grammar were reviewed on branch `spec/human-instruction-grammar-vnext-20261022`. This audit does not treat proposed documents as normative and does not treat source presence as proof of execution.

## Findings by capability

| Capability | Finding | Evidence boundary |
|---|---|---|
| Human-instruction identity | **Specified / accepted** | Constitution and requirements explicitly require formal human-oriented source, not arbitrary prompts. |
| Deterministic compiler semantics | **Implemented for v0.1** | Existing lexer, parser, semantic analyzer, AST, IR, and interpreter are wired together in the default CLI. |
| No mandatory conventional structural punctuation in the intended surface | **Not yet demonstrated end-to-end** | v0.1 examples still use quoted strings; vNext grammar is draft and parser is isolated. |
| Complete vNext grammar | **Incomplete** | Prototype supports only project, text data, display action, and a persistence declaration form. |
| vNext semantic/IR lowering | **Missing** | Prototype-specific AST is not lowered into the canonical One Semantic IR. |
| vNext execution | **Missing** | The prototype is not connected to CLI or runtime. |
| Task-list create/list/complete operations | **Not implemented in vNext** | No demonstrated executable task collection or completion state transition in the human-instruction frontend. |
| Durable persistence across restart | **Available as a lower-level capability, not proven for vNext** | SQLite adapter exists, but the prototype persistence declaration does not invoke it or define data lifecycle. |
| Native compilation | **Implemented with bounded evidence** | Native backend verification record provides exact-SHA and target-specific claims; it does not prove vNext source compilation. |
| Interoperability | **Architecture and partial mechanisms exist** | Process, adapter, network and database boundaries exist; the vNext language surface is not yet connected to them comprehensively. |
| Cross-platform support | **Partially evidenced** | Recorded Linux/Windows checks apply to named commits and tests; do not generalize beyond the evidence matrix. |
| Large-project readiness | **Not yet demonstrated for the intended surface** | Module/package, dependency, type/effect, diagnostic and compatibility contracts must be verified as one integrated language experience. |
| AI-independent parsing and execution | **Implemented principle for v0.1; intended for vNext** | Existing pipeline is deterministic; vNext must be wired through the same deterministic authority boundary. |

## Concrete defects in the current vNext parser prototype

1. **Wrong symbol category accepted as a value.** The validation originally used one name set for both data declarations and action declarations, allowing a display reference to resolve to an action name. A targeted correction and regression test have now been committed on the prototype branch. CI and compilation still need to confirm it.
2. **Text block is not lossless.** The parser removes blank lines before parsing, so blank lines inside a text block are lost. It also strips each content line's entire indentation instead of a common block indentation amount, losing relative indentation. This contradicts the draft's intended text-block contract.
3. **Indentation validation is incomplete.** It checks for a tab only immediately after leading spaces, rather than rejecting a tab anywhere in the indentation prefix as one coherent rule.
4. **Diagnostics lose useful source location.** Duplicate names and unresolved display references currently report line 1 rather than the declaration/reference line.
5. **Name and statement boundaries are underspecified in code.** Names are accepted as nearly any nonempty string; reserved phrase boundaries, Unicode normalization, and ambiguity rules are not implemented as a comprehensive grammar.
6. **Persistence syntax is only a declaration.** Parsing “Simpan semua …” does not create storage, load prior state, handle failures, or persist anything.
7. **No lowering or runtime bridge.** The custom AST currently has no conversion to the canonical AST/IR and cannot run through the public CLI.
8. **CI is currently a blocking unknown.** The recorded workflow for the earlier head failed on both Ubuntu and Windows. Fetching its job logs returned a 404 from the log endpoint, so the exact root cause has not been established. The new parser fix must be judged by a fresh run, not assumed successful.

## Required architecture path

Keep v0.1 behavior stable. Treat the human-instruction surface as a separately versioned frontend that lowers into the shared semantic model and runtime. Do not fork the meaning of operations by locale or frontend.

```text
ONE human-instruction source
  -> lossless line/block scanner
  -> versioned deterministic grammar
  -> source-spanned ONE AST
  -> name, type, effect and capability analysis
  -> canonical One Semantic IR
  -> interpreter and/or native backend
  -> governed runtime adapters
  -> observable results and durable state
```

The parser must not contain the whole language as an unreviewed pile of phrase checks. Grammar productions, reserved phrases, name slots, types, effects, diagnostics, AST/IR mappings, and conformance fixtures must be explicit and versioned.

## Execution order and acceptance gates

### Gate 1 — Stabilize the prototype

- Retrieve actionable CI output or reproduce with a supported Rust environment.
- Fix text-block preservation: blank lines, common indentation, Unicode, CRLF, trailing newline, and command-like text.
- Fix indentation validation, declaration categories, duplicate-name locations, and reference spans.
- Add positive and negative fixtures for every supported production.
- Require Ubuntu and Windows CI to pass before proceeding.

### Gate 2 — Complete a minimal vertical slice

- Define a collection/list type and task record with explicit semantics.
- Define instructions for adding, listing, and marking a task complete.
- Define persistence lifecycle, location, error behavior, and capability policy.
- Lower every accepted instruction to canonical AST/IR or a deliberately versioned common semantic node.
- Wire version selection into the public CLI without silently changing v0.1.
- Prove a program can create tasks, list them, mark them complete, exit, restart, and observe the same persisted state.

### Gate 3 — Build the language, not a demo parser

Add formal, tested support for values/types, variables and state, functions/actions, conditions, loops, modules, collections, errors, events, and composition. Each construct must have parser fixtures, semantic/type rules, AST/IR mapping, interpreter behavior, and relevant native backend behavior. Unsupported forms must fail closed.

### Gate 4 — General-purpose standalone capability

Prioritize a standard library and package/module model; filesystem and storage; networking and HTTP; structured concurrency; serialization; UI/web/application interfaces; testing and debugging tools; build/package/release workflows; secure capabilities and auditing. The language must be usable without requiring users to write another language, while still allowing explicit FFI, process, protocol, generated-artifact, and native-library interoperability when useful.

### Gate 5 — Evidence and compatibility

- Run deterministic and negative conformance suites.
- Run build, tests, Clippy, and platform matrix on exact commit SHAs.
- Record platform and backend limits precisely.
- Verify source examples from beginner documentation against the exact compiler version.
- Promote the grammar to normative status only after review and passing acceptance gates.
- Do not merge draft PRs or change v0.1 semantics without explicit approval.

## Definition of done

The user-facing goal is met only when a user can write a punctuation-light ONE program using the documented human-instruction grammar; compile, run, test, package and debug it with the ONE toolchain; use standard libraries and core system capabilities; build an actual application without authoring another programming language; retain data across restarts; and rely on deterministic, documented diagnostics and capability enforcement. Interoperability with other languages remains supported but optional.

**Current conclusion:** ONE's original vision is recorded and its v0.1 foundation is substantial, but the full intended human-instruction, standalone end-to-end language is not yet complete. The immediate critical path is a correct, verified vNext parser and an executable task-list vertical slice, followed by systematic language and ecosystem completion.
