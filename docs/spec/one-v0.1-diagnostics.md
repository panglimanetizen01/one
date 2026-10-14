# One v0.1 Diagnostics Contract

Every diagnostic has a stable error code, severity, human-readable message, and source span when a source location exists. Notes and suggestions are optional.

## Required behavior

- Never silently modify source.
- Prefer Indonesian explanations for the initial locale.
- State what failed and why.
- Point to the source location when possible.
- For unresolved names, explain that the name must be created or clarified.
- For ambiguous or unsupported syntax, reject the source instead of selecting an interpretation.
- Runtime errors remain distinct from parse and semantic errors.

## Error families

- `E000x`: lexical errors
- `E100x`: parser/grammar errors
- `E200x`: semantic/type/name errors
- `R300x`: runtime/capability errors

## v0.1 stable codes in the reference implementation

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

Codes are part of the diagnostic contract and must not be reused for unrelated failures without a documented v0.1 contract change.
