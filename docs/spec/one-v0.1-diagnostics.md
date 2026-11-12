# One v0.1 — Diagnostics Contract

**Status:** Normative cross-layer contract for v0.1 review

Every diagnostic has a stable error code, severity, human-readable message, and source span when a source location exists. Notes and suggestions are optional. Human-facing wording is localizable; diagnostic identity and structured parameters are not.

## Required behavior

- Never silently modify source.
- Prefer Indonesian explanations for the initial locale.
- State what failed and why.
- Point to the source location when possible.
- For unresolved names, explain that the name must be created or clarified.
- For ambiguous or unsupported syntax, reject the source instead of selecting an interpretation.
- Runtime errors remain distinct from parse and semantic errors.
- Diagnostics must be deterministic for identical source, language version, localization contract, and semantic inputs.
- Related spans identify declarations/conflicts when relevant.

## Stable error families

Existing v0.1 implementation codes remain stable. New contracts use these semantic families for future additions:

- `ELEXxxx`: lexical
- `EPARSExxx`: parser/grammar
- `EAMBxxx`: ambiguity
- `ENAMExxx`: name/scope
- `ETYPExxx`: type
- `ESEMxxx`: semantic
- `ECAPxxx`: capability/effect
- `EIRxxx`: invalid CSIR
- `ERUNxxx`: runtime
- `EPLATxxx`: platform
- `ETOOLxxx`: tooling/configuration

The existing numeric codes below remain valid for the current reference implementation and are not silently renumbered.

## Existing v0.1 stable codes

### Lexical

- `E0001` — tab indentation.
- `E0002` — indentation is not a multiple of four spaces.
- `E0003` — block indentation increased by more than one level.
- `E0004` — dedentation returns to an unseen level.
- `E0005` — unsupported string escape.
- `E0006` — unterminated string.
- `E0007` — unsupported character.

### Parser

- `E1001` — missing statement newline.
- `E1002` — required block indentation missing.
- `E1003` — required dedent missing.
- `E1004` — orphan dedent.
- `E1005`–`E1007` — malformed `untuk setiap` loop.
- `E1008`–`E1010` — malformed function declaration.
- `E1011` — event has no explicit trigger.
- `E1012` — empty block.
- `E1013` — missing expression.
- `E1014` — unsupported expression phrase/operator.
- `E1015` — invalid number literal.
- `E1016` — token cannot begin an expression.
- `E1017`–`E1018` — malformed list mutation action.

### Semantic

- `E2001` — duplicate declaration in the current scope.
- `E2002` — assignment to an unknown name.
- `E2003` — incompatible assignment type.
- `E2004`–`E2009` — invalid action/list arguments.
- `E2010` — non-boolean conditional.
- `E2011` — non-list `untuk setiap` source.
- `E2012` — non-boolean `selama` condition.
- `E2013` — duplicate function.
- `E2014` — unknown identifier.
- `E2015` — arithmetic type error.
- `E2016` — incompatible comparison types.
- `E2017` — unknown function.
- `E2018` — wrong function arity.

### Runtime

- `R3001` — event not registered.
- `R3002` — execution step limit exceeded.
- `R3003` — assignment target missing at runtime.
- `R3004` — runtime condition is not boolean.
- `R3005` — `untuk setiap` value is not a list.
- `R3006`–`R3013` — action/list runtime errors.
- `R3014` — process capability denied.
- `R3015` — unsupported runtime action.
- `R3016` — runtime identifier missing.
- `R3017` — division by zero.
- `R3018` — incompatible runtime operation.
- `R3019` — runtime function missing.
- `R3020` — runtime function arity mismatch.
- `R3021` — event dispatch attempted before the program was executed.
- `R3022` — duplicate list declaration in the current scope.
- `R3023` — integer arithmetic overflow; checked arithmetic returns a deterministic runtime error.

### Capability/orchestration runtime

- `N3200` — orchestration target is not text.
- `N3201` — filesystem content or SQL statement is not text.
- `N3202` — database query parameter is not text.
- `N3205` — orchestration is not configured for this interpreter.
- `N3206` — adapter result could not be normalized.
- `N3207` — adapter returned bytes that are not valid UTF-8 for a text result.
- `N3208` — orchestration request is invalid.
- `N3209` — capability policy denied the request.
- `N3210` — orchestration request requires approval.
- `N3211` — no compatible adapter is available.
- `N3212` — adapter invocation failed.
- `N3213` — adapter returned an unsuccessful execution result.

## Machine-readable structure

Conceptual form:

```text
Diagnostic {
  code,
  severity,
  primary_span?,
  related_spans[],
  message_key,
  parameters,
  notes[],
  hint?
}
```

Serialization is implementation-defined until the CLI/tooling contract fixes it.

## No hidden repair

Diagnostics must never imply that the compiler silently changed source, reordered declarations, selected a fuzzy name, inserted an implicit conversion, granted a capability, or recovered from ambiguity unless an explicit later contract defines that behavior.

## Acceptance criteria

- stable codes are preserved;
- spans are preserved from AST/semantic provenance;
- ambiguity is rejected;
- diagnostics are deterministic;
- localization does not alter diagnostic identity;
- runtime and capability errors remain distinguishable;
- machine-readable structure is sufficient for CLI/editor tooling.