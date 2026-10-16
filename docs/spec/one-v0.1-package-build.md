# One v0.1 — Package and Build Contract

**Status:** Approved normative contract; v0.1 executable subset explicitly scoped below

## 1. Purpose

The package/build layer turns a declared One project and its dependency graph into reproducible build inputs and artifacts. It does not redefine language semantics.

## 2. Project identity

The package/build architecture reserves a stable manifest model for language version, localization, entry module, dependencies, build targets, and required effect/capability policy metadata. **The executable v0.1 subset intentionally implements the minimal manifest fields `nama` and `versi`; the remaining manifest dimensions are reserved for later executable contracts and must not be claimed as implemented.**

For the executable v0.1 subset, project entry is deterministically fixed to `src/main.one`, and the package command rejects missing or invalid `nama`/`versi` fields rather than guessing defaults.

## 3. Dependency resolution

The architecture requires dependencies to resolve from explicit declarations and a lock state where reproducibility is required. **The executable v0.1 subset has no external dependency graph or package registry; an empty dependency set is the only supported state.** Implicit network discovery, local filesystem order, or mutable latest-version selection is not semantic input.

## 4. Build graph

The architecture models source modules, dependencies, generated intermediate artifacts, CSIR/backend steps, and final artifacts. **The executable v0.1 subset builds one deterministic `src/main.one` entry source through the reference semantic pipeline into `.oneir`, and packages that validated IR into `.onepkg`.** Multi-module graphs and backend compilation remain future executable work.

## 5. Reproducibility

Given identical source, manifest, dependency lock state, toolchain version, target, and declared build configuration, a deterministic build must produce equivalent semantic artifacts. Timestamps and machine paths must not silently alter semantic identity. The executable v0.1 artifacts are deterministic textual representations; exact binary reproducibility remains outside the current subset.

## 6. Capability separation

Build-time capabilities and program runtime capabilities are distinct. A build tool may need filesystem/network access according to build policy; those grants do not automatically become runtime capabilities of the produced program. The v0.1 package/build path performs no implicit runtime capability grant.

## 7. Targets

The architecture permits Windows, Linux, or other supported targets through explicit metadata. **The executable v0.1 subset targets the host platform through the existing Windows/Linux platform contract and does not emit target-specific installers or native binaries.**

## 8. Failure behavior

Missing dependency, invalid manifest, unsupported target, failed compilation, or denied build effect must stop the affected build step with a structured diagnostic. The v0.1 subset implements invalid/missing manifest and source/build failures; unsupported target/dependency failures are represented by the explicit scope boundary because those inputs are not yet accepted.

## 9. Cache correctness

Caches are keyed by complete declared semantic inputs. Cache hits must not depend on hidden environment state. Invalid or stale cache entries must be ignored rather than treated as authoritative. The executable v0.1 subset does not use a build cache.

## 10. Artifact provenance

Final artifacts retain enough provenance to identify language version, toolchain version, target, and source/dependency lock state. **The executable v0.1 `.oneir` and `.onepkg` artifacts are intentionally debug/intermediate artifacts and do not yet satisfy the full provenance envelope; this is a documented limitation, not an unsupported claim.**

## 11. Acceptance criteria

For the architecture:

- explicit project manifest;
- deterministic dependency graph;
- separated build/runtime capabilities;
- Windows/Linux targets supported by explicit metadata;
- reproducible semantic build inputs;
- structured failures;
- no hidden dependency or target substitution.

For the executable v0.1 subset:

- explicit `nama` and `versi` manifest fields;
- deterministic fixed entry module `src/main.one`;
- empty external dependency set;
- deterministic `.oneir` and `.onepkg` textual artifacts;
- Windows/Linux host execution through the existing platform contract;
- structured source/semantic/build/manifest failures;
- no hidden dependency or target substitution.
