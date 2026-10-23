# Human-vNext starter grammar review

**Contract reviewed:** `human-vnext-starter/0.1`  
**Review outcome:** ACCEPTED WITH EXPLICIT SCOPE BOUNDARIES  
**Review type:** repository-internal specification/implementation/conformance review performed for the 30% foundation milestone  
**External sign-off:** none claimed  
**Canonical implementation:** `crates/one-core/src/human_vnext.rs`  
**Conformance corpus:** `tests/human-vnext/valid/` and `tests/human-vnext/invalid/`

## Decision

Accept the implementation-bounded starter grammar as the deterministic contract for the explicitly selected experimental human-vNext parser. This decision does **not** accept the broader `one-human-vnext/0.1-draft`, does not make arbitrary Indonesian prose executable, and does not change the normative ONE v0.1 grammar, parser, AST, or default CLI behavior.

## Review checklist

| Concern | Review result | Evidence and boundary |
|---|---|---|
| Version identity and compatibility | PASS | The contract has its own identifier/version; only the explicit `jalankan-human-vnext` entry point selects it. |
| Deterministic parsing | PASS | Unsupported forms fail with stable `Hxxxx` classes; no LLM or fuzzy fallback participates in parsing. |
| Names and binding | PASS | Names are case-sensitive, are not Unicode-normalized, use the documented Unicode-alphanumeric/`_`/`-`/single-space class, and data/action declarations share a namespace. |
| Structure and indentation | PASS | Structural lines use the documented four-space levels; tabs and noncanonical structural indentation are rejected. |
| Text-block fidelity | PASS after the opaque-whitespace regression fix | Exactly the first eight leading ASCII spaces are structural. Further spaces and tabs are payload; LF/CRLF behavior, blank lines, empty blocks, relative indentation, trailing newlines, Unicode, and instruction-like payload text have executable coverage. |
| Semantic validation | PASS | Duplicate declarations/persistence and unresolved display references are rejected with stable codes and relevant declaration-line diagnostics. |
| Fixture coverage | PASS for the stated starter subset | The contract checker requires at least six valid and seventeen invalid human-vNext fixtures. This is representative coverage, not exhaustive coverage of a general-purpose language. |
| Executability claim | PASS with restriction | Parsing is separate from execution. The current runner accepts only the exact task-list vertical slice and rejects unsupported parsed declarations/actions before opening storage. |
| v0.1 isolation | PASS | The default v0.1 command does not auto-detect this grammar; its regression tests remain part of the same workspace test run. |

## Diagnostic contract

The starter grammar's emitted parser diagnostic classes are:

| Code | Rejection class |
|---|---|
| H0001 | Tab used in structural indentation |
| H0002 | Noncanonical structural indentation or invalid text-block prefix |
| H0003 | Empty source or missing application declaration |
| H0004 | Invalid application name |
| H0005 | Indented application declaration |
| H0006 | Orphaned nested instruction |
| H0007 | Invalid data name |
| H0008 | Missing or malformed text-block header |
| H0010 | Invalid action name |
| H0011 | Action body indentation is not exactly one four-space level |
| H0012 | Invalid name in a display instruction |
| H0013 | Unsupported action-body instruction |
| H0014 | Empty action body |
| H0015 | Invalid persistence collection name |
| H0016 | Unknown top-level instruction |
| H0017 | Duplicate data/action declaration name |
| H0018 | Display reference does not resolve to a data declaration |
| H0019 | Duplicate persistence declaration |

H0009 is not emitted by this starter version and is intentionally not assigned a meaning. Diagnostic codes and the rejection classes above are versioned behavior; incompatible changes require a new contract version or an explicit reviewed compatibility decision.

## Remaining boundaries

- This review is not a claim that the broader human-instruction vNext design is complete.
- The task-list runner is a dedicated vertical-slice interpreter, not canonical AST/IR integration or a general compiler/runtime.
- This review does not certify hosted GitHub Actions, Windows execution, Android native execution, or release readiness.
- Completion points remain gated by exact-SHA executable verification and the separate milestone ledger; this review alone earns no verified points.
