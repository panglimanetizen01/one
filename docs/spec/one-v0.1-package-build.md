# One v0.1 — Package and Build Contract

**Status:** Draft normative contract for review

## 1. Purpose

The package/build layer turns a declared One project and its dependency graph into reproducible build inputs and artifacts. It does not redefine language semantics.

## 2. Project identity

A project has a stable manifest declaring language version, localization, entry module, dependencies, build targets, and required effect/capability policy metadata.

## 3. Dependency resolution

Dependencies resolve from explicit declarations and a lock state where reproducibility is required. Implicit network discovery, local filesystem order, or mutable latest-version selection is not semantic input.

## 4. Build graph

The build graph contains source modules, dependencies, generated intermediate artifacts, CSIR/backend steps, and final artifacts. Each node declares inputs and outputs. Unrelated nodes may build in parallel without changing results.

## 5. Reproducibility

Given identical source, manifest, dependency lock state, toolchain version, target, and declared build configuration, a deterministic build must produce equivalent semantic artifacts. Timestamps and machine paths must not silently alter semantic identity.

## 6. Capability separation

Build-time capabilities and program runtime capabilities are distinct. A build tool may need filesystem/network access according to build policy; those grants do not automatically become runtime capabilities of the produced program.

## 7. Targets

A project may declare Windows, Linux, or other supported targets. Target-specific packaging metadata is separate from common semantic source.

## 8. Failure behavior

Missing dependency, invalid manifest, unsupported target, failed compilation, or denied build effect must stop the affected build step with a structured diagnostic. The build system must not silently substitute another dependency or target.

## 9. Cache correctness

Caches are keyed by complete declared semantic inputs. Cache hits must not depend on hidden environment state. Invalid or stale cache entries must be ignored rather than treated as authoritative.

## 10. Artifact provenance

Final artifacts retain enough provenance to identify language version, toolchain version, target, and source/dependency lock state. Exact binary reproducibility is a future target unless the backend contract specifies it.

## 11. Acceptance criteria

- explicit project manifest;
- deterministic dependency graph;
- separated build/runtime capabilities;
- Windows/Linux targets supported by explicit metadata;
- reproducible semantic build inputs;
- structured failures;
- no hidden dependency or target substitution.