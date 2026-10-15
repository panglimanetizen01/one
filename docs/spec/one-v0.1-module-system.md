# One v0.1 — Module System

**Status:** Draft normative contract for review

## 1. Purpose

Modules provide explicit project-level namespaces and composition without changing the language's lexical name-resolution rules.

## 2. Module identity

A module has a stable project-relative identity defined by the project/module manifest contract. Filesystem paths are not semantic identifiers by themselves.

## 3. Exports

A module explicitly exports declarations. Non-exported declarations are private to the module and cannot be resolved by an importing module.

## 4. Imports

Imports create explicit bindings in the importing module. Importing a module must not execute arbitrary module code merely to discover its declarations.

## 5. Name resolution

Module resolution occurs before ordinary lexical resolution supplies imported declarations. Imported names enter a defined module scope and obey the same ambiguity/duplicate rules as local declarations.

## 6. Cycles

Module cycles are detected deterministically. v0.1 must reject cycles that cannot be represented by the defined initialization model rather than relying on load order.

## 7. Initialization

Module initialization order must be explicit and deterministic. A module must not observe an uninitialized exported value unless a later contract explicitly defines that state.

## 8. Capability boundary

Importing a module does not automatically grant its consumer new host capabilities. Effect requirements are declared separately and enforced by the runtime capability policy.

## 9. Package boundary

Module identity is language-level. Package/dependency resolution supplies modules but does not redefine their semantics.

## 10. Localization

Module names and declarations follow the active localization grammar, but module identity and export semantics are localization-independent.

## 11. Diagnostics

Duplicate imports, missing modules, missing exports, cyclic initialization, and ambiguous imported names must have stable diagnostics with relevant source spans.

## 12. Acceptance criteria

- explicit imports/exports;
- deterministic module graph;
- no implicit filesystem-wide discovery;
- no import-time capability escalation;
- deterministic cycle handling;
- compatibility with lexical name resolution and CSIR lowering.