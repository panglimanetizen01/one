# One v0.1 Diagnostics Contract

Every diagnostic has a stable error code, severity, human-readable message, and source span when a source location exists. Notes and suggestions are optional.

## Required behavior

- Never silently modify source.
- Prefer Indonesian explanations for the initial locale.
- State what failed and why.
- Point to the source location when possible.
- For unresolved names, explain that the name must be created or clarified.
- For ambiguous syntax, state the competing interpretations instead of selecting one.
- Runtime errors remain distinct from parse and semantic errors.

## Error families

- `E000x`: lexical errors
- `E100x`: parser/grammar errors
- `E200x`: semantic/type/name errors
- `R300x`: runtime/capability errors

Codes are part of the diagnostic contract and should not be reused for unrelated failures.
