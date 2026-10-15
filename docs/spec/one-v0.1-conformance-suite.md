# One v0.1 — Conformance Suite Contract

**Status:** Approved normative contract

## 1. Purpose

The conformance suite proves that lexer, parser, AST, semantic analysis, CSIR, reference interpreter, compiler backends, and platform adapters implement the same v0.1 contracts.

## 2. Test layers

Tests are grouped into:

1. lexical;
2. concrete grammar;
3. AST structure/spans;
4. name resolution/scope;
5. type checking;
6. semantic behavior;
7. CSIR validation;
8. reference interpreter;
9. effect/capability enforcement;
10. diagnostics;
11. cross-backend equivalence;
12. localization equivalence.

A failure is reported at the earliest violated contract boundary.

## 3. Fixture format

Each fixture identifies:

- language version;
- localization;
- source or CSIR input;
- expected accepted/rejected status;
- expected AST/semantic/IR result where the layer requires it;
- expected diagnostic code/span for rejection;
- expected runtime result/effects for executable fixtures.

Fixtures must be deterministic and self-contained.

## 4. Required lexical cases

Cover identifiers, multi-word names, literals, line boundaries, indentation, reserved structural words, malformed tokens, and lexical diagnostics.

## 5. Required parser cases

Cover all six semantic foundations: DATA, ACTION, CONDITION, LOOP, FUNCTION, EVENT; nested blocks; multi-word boundaries; expression precedence; invalid structure; and ambiguity rejection.

## 6. Required AST cases

Assert normalized node kinds, exact source spans, logical `Name` grouping, absence of unresolved ambiguity nodes, and deterministic repeated parsing.

## 7. Required name/scope cases

At minimum:

- declaration before use;
- use before declaration;
- nested lookup;
- shadowing;
- duplicate declarations;
- parameter scope;
- loop binding scope;
- unresolved reference;
- exact spelling;
- multi-word name resolution;
- deterministic ambiguity rejection.

## 8. Required type cases

Cover literal typing, assignment compatibility, list element types, arithmetic, comparison, Boolean conditions, function arguments/results, `Nothing`, invalid operations, and explicit conversion boundaries.

## 9. Required semantic cases

Cover declaration, assignment, display, list creation/mutation, conditions, loops, functions, events, evaluation order, state mutation, and defined errors.

## 10. Required CSIR cases

Every valid semantic construct must have a valid CSIR representation. Invalid combinations must be rejected before runtime. Provenance must survive lowering sufficiently for diagnostics and backend comparison.

## 11. Required runtime cases

Cover successful execution, structured runtime errors, function calls, recursion, control flow, mutation, resource limits, event delivery, and deterministic evaluation.

## 12. Required capability cases

For every effect class, test:

- allowed request;
- denied request;
- invalid request;
- out-of-scope resource;
- unavailable platform effect;
- external failure after permission.

A denied effect must have no external side effect.

## 13. Diagnostic conformance

Diagnostics must assert stable error code and primary span. Related spans are asserted when required by the contract. Message wording may evolve only if the stable diagnostic identity and semantic information remain intact.

## 14. Backend equivalence

The same validated CSIR fixture is executed by the reference interpreter and each backend. Compare semantic results, observable outputs, defined effects, and defined errors. Do not compare implementation-specific memory, timing, or allocation details.

## 15. Localization equivalence

Equivalent programs expressed through approved localization grammars must lower to equivalent semantic meaning and CSIR, while source spans and human-facing diagnostics remain localized appropriately.

## 16. Determinism

Running the same fixture repeatedly with the same declared inputs and runtime policy must produce the same result. Tests must not depend on filesystem ordering, wall-clock time, network state, process scheduling, hash iteration, or LLM output.

## 17. Negative-test rule

Every accepted construct should have at least one structurally invalid or semantically invalid neighboring case. Invalid programs must not be accepted through heuristic recovery.

## 18. Acceptance gate

A v0.1 implementation is conforming only when all mandatory fixtures pass at its supported layer and no known contract violation is masked by a later layer.

Conformance results must identify implementation version, language version, localization, fixture set version, and runtime/backend version.
