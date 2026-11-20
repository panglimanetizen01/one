# One v0.1 — Package and Build Contract

**Status:** Approved normative contract; v0.1 executable subset explicitly scoped below

## 1. Purpose

The package/build layer turns a declared One project and its dependency graph into reproducible build inputs and artifacts. It does not redefine language semantics.

## 2. Project identity

The package/build architecture reserves a stable manifest model for language version, localization, entry module, dependencies, build targets, and required effect/capability policy metadata. For executable v0.1, both values must be non-empty, single-component ASCII filename-safe identifiers (letters, digits, `.`, `_`, `-`, and `+` only; no path separators, prefixes, control characters, or leading/trailing dots). The CLI rejects unsafe values before constructing artifact or module-lock paths.

**The executable v0.1 subset intentionally implements the minimal manifest fields `nama` and `versi`; the remaining manifest dimensions are reserved for later executable contracts and must not be claimed as implemented.**

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

The package artifact itself is deterministic content, not a self-authenticating release. The milestone-100 V2 sidecar supplies a separate authenticated source-bound build attestation when signed with a trusted publisher key; an artifact without a verified sidecar is not authenticated.

## 10A. Experimental human-core packaging extension (milestone 100)

The milestone-100 feature branch adds an explicitly detected project path for projects whose `src/main.one` first line is `Buat program <nama>`. The `build`, `test`, and `package` commands invoke `one_core::human_compiler::compile_file`, so relative `Gunakan modul` imports are resolved and compiled through the canonical semantic IR. Build emits a textual `.oneir`, test executes the program through the reference interpreter, and package emits the versioned artifact described below. Its deterministic artifact starts with `One human-core package v1` and records the manifest, the `human-core` frontend marker, and canonical IR. It also writes `target/<nama>-<versi>.one.lock`, embedding the same module lock in the package. When this lock exists, human-core `build` and `test` recompute the resolved module list and reject a stale lock with `ETOOL006`; rerun `one package` to refresh it. The lock format is `One module lock v2` and lists the root and resolved relative module paths in sorted order with SHA-256 content digests. These digests provide cryptographic content fingerprints for comparison against a trusted lock, but the lock is not signed and does not authenticate publisher identity or artifact provenance. A party able to replace both source modules and the lock can recompute the digests.

This is a separate experimental artifact format. It does not change the existing `One package v0.1` format for ordinary v0.1 source projects, and it does not imply external package dependency resolution, signed package identity, authenticated provenance, target-specific native artifacts, or the full provenance envelope. SHA-256 detects content mismatch only when the expected lock is trusted; the local lock is not a signature or standalone security boundary. The external registry and dependency graph/version-conflict handling are explicitly outside the executable v0.1 subset and are not blockers for the v0.1 package contract. Unsupported dependency declarations must continue to fail closed; the absence of a registry is not a failed v0.1 dependency-resolution test. The milestone E2 gate remains open for the release integration and authenticated build-provenance criteria that are explicitly required by the milestone, and those criteria must be verified independently.


## 10B. Build/package artifact output confinement

The CLI reads the package manifest, main source, conformance fixtures, and module-lock data through a root-confined no-follow reader; these inputs must be regular single-link files beneath the selected project root. The CLI writes `.oneir`, `.onepkg`, and module-lock artifacts only through a root-confined filesystem writer. The `target` directory must be a real directory beneath the project root; symlink/reparse-point directory redirection is rejected. Existing artifact targets must be regular files with exactly one hard link; symlink/reparse-point and hard-linked final targets are rejected before truncation. A failed confinement check must not modify or create an artifact outside the project root.

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

## 10B. Experimental Ed25519 package provenance sidecar (milestone 100)

The CLI now provides additive commands to sign and verify an existing package artifact without changing the .onepkg artifact format:

- `one sign-package <file.onepkg> --publisher <id> --private-key <seed-hex-file> --source-root <project-dir>` signs a V2 build attestation. It binds the validated artifact format and digest to the deterministic digest of `one.toml` and the project source tree, the full clean Git revision, ONE CLI version, builder OS/architecture, Rust/Cargo versions, and the normative `empty-v0.1` dependency model. The source root must be a clean Git-tracked project with no symlinks or unsupported dependency declarations. The key file contains a 32-byte Ed25519 seed encoded as 64 hexadecimal characters; keep it outside the repository and never commit it.
- `one verify-package <file.onepkg> --trusted-publishers <trust-store> [--source-root <project-dir>]` checks the artifact SHA-256, Ed25519 signature, and publisher-to-key binding. Supplying `--source-root` also recomputes and checks the source-tree digest and Git revision; without it, the source snapshot is not rechecked.
- The sidecar is `<file.onepkg>.provenance`. V2 records publisher ID, public key, artifact digest and validated artifact format, source digest/revision, CLI version, builder OS/architecture, Rust/Cargo versions, dependency model, and signature; all attestation fields are included in the signed message. Legacy V1 artifact-only sidecars remain verifiable but do not satisfy source-bound build-provenance acceptance. The trust store contains one `publisher-id=public-key-hex` entry per line; duplicate or malformed entries are rejected.
- SHA-256 is used as the signed content identifier, not as a substitute for the signature. Verification fails closed for changed bytes, unknown publishers, key mismatches, malformed sidecars, and invalid signatures.
- Signing and verification bound package reads to 64 MiB, private-key seed reads to 256 bytes, provenance sidecars to 16 KiB, publisher trust stores to 1 MiB, source trees to 64 MiB, individual source files to 16 MiB, and source file count to 10,000. Inputs beyond these limits are rejected before complete allocation.

The V2 sidecar implements source-bound authenticated provenance for the normative v0.1 empty-dependency package contract. It intentionally does not implement an external One package registry or dependency graph/version-conflict resolution: those are outside the executable v0.1 subset, and unsupported dependency declarations fail closed. Signing remains an explicit release step because private keys must not be embedded in `one package`; the release process must sign the final artifact and verify it against a trust store delivered through an independently trusted channel. Artifact-only V1 signatures are not sufficient for the milestone E2 source-provenance criterion.

