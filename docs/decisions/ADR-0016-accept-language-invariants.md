# ADR-0016: Accept the v0.1 Language Invariants as Normative Foundation Constraints

- **Status:** Accepted
- **Date:** 2026-10-21
- **Scope:** Language contract governance; does not certify implementation conformance

## Context

The approved foundation already establishes deterministic formal syntax, ambiguity rejection, a shared semantic model across localizations, explicit capability boundaries, cross-platform consistency, and no mandatory LLM dependency for language meaning. The language-invariants document expresses these constraints as LI-001 through LI-030.

The repository subsequently made the Indonesian lexical and concrete grammar contracts normative and accepted the AST contract for the implemented v0.1 baseline. Leaving the foundational invariant document marked as a draft while downstream contracts were treated as normative created an inconsistent specification gate.

## Decision

Accept `docs/spec/one-v0.1-language-invariants.md` as the normative foundation constraints for One v0.1.

This acceptance:
- locks the listed invariants as constraints on downstream contracts;
- does not promote every existing implementation behavior to normative status;
- does not imply that every invariant has already passed exhaustive conformance testing;
- leaves the detailed localization contract in draft status until separately reviewed;
- leaves source-span capabilities bounded by the accepted AST contract;
- requires implementation mismatches to be handled through regression tests and implementation fixes or an explicit compatibility decision.

The specification index is the source of truth for downstream gate ordering. Implementation status and executable verification remain separate evidence claims.

## Consequences

1. Lexical and grammar specifications are downstream normative contracts, not future unapproved gates.
2. The AST contract's dependency on approved language invariants is now consistent.
3. Implementation conformance must continue to be tracked by tests and exact-SHA evidence.
4. The known lexical-token-span versus AST-name-span boundary is stated explicitly; consumers must not infer a complete multi-word name range from the AST.

## Verification

The contract index, language-invariants status, lexical source-span wording, implementation status, and evidence matrix are reconciled in the forensic contract-reconciliation change. This ADR is a governance decision, not a claim that all target backends or all 30 invariants are exhaustively verified.
