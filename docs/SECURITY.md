# One Security Contract

## Security position

One is general-purpose and can be used for legitimate security engineering, defensive automation, vulnerability research, authorized penetration testing, system hardening, forensic tooling, and network analysis.

Security capability does not require a separate language. It requires explicit runtime governance.

## Authority model

```text
human-readable source
 -> formal semantics
 -> typed capability request
 -> scope
 -> policy
 -> adapter
 -> execution
 -> audit
```

Source wording alone never grants privilege.

## Required controls for sensitive operations

Where applicable, a capability contract must define:

- semantic identity;
- runtime capability;
- effect class;
- target/resource scope;
- privilege requirements;
- platform/backend availability;
- adapter identity;
- input/output schema;
- audit requirements;
- determinism/side-effect expectations;
- verification boundary.

## No privilege escalation through syntax

The following must not bypass security controls:

- localized wording;
- aliases;
- macros;
- generated source;
- libraries;
- adapters;
- external tool substitution;
- AI-generated code.

## Security tooling interoperability

One may interoperate with operating-system APIs, network libraries, native libraries, external security tools, and analysis frameworks through explicit interfaces. Such integration must preserve scope and authorization boundaries.

## Auditability

Security-sensitive execution should record observed runtime facts such as capability, target/resource, adapter, policy decision, execution result, and relevant diagnostics without inventing unverifiable facts.

## Safe project principle

The project should not artificially cripple the language merely because it can express security tooling. Instead, it should make authorization, scope, and execution boundaries explicit and testable.

## Filesystem path resolution and platform boundary

On Linux and Android, filesystem adapter opens now resolve each parent directory relative to an already-open directory descriptor and apply O_NOFOLLOW to each path component and the final target. Read and write targets are checked as regular files after opening; read/write payloads remain capped at 16 MiB. Regression tests cover symlinked parent directories, final-component symlinks, and ensuring a rejected write symlink does not modify its referent.

This closes the check-then-open symlink race for these Linux/Android adapter paths, but it is **not** a complete filesystem sandbox: operations still use the process working directory for relative paths, the capability contract does not yet define a root confinement directory, and non-Linux/Android targets currently use standard-library opens after static validation rather than equivalent descriptor-relative traversal. Do not describe Windows or macOS filesystem operations as race-resistant until their corresponding primitives and tests exist.

## Signed package provenance

The CLI exposes Ed25519 signing and verification for .onepkg artifacts. V2 signing creates a .onepkg.provenance sidecar that authenticates the publisher ID, artifact SHA-256, deterministic source-tree digest, clean Git revision, ONE CLI version, builder OS/architecture, Rust/Cargo versions, and normative empty-dependency model in one signed message. A V2 sidecar is accepted as source-bound build provenance only when its signature verifies against a separately configured publisher trust store and the source root is rechecked. V1 artifact-only sidecars remain verifiable for compatibility, but they do not satisfy source-bound provenance acceptance. A checksum alone is never treated as authentication.

Private key seed files must be stored outside the repository and must not be committed. The trust store must be provisioned through an independently trusted channel; including a public key only in the sidecar is not sufficient to trust a publisher. Signing requires a clean, Git-tracked source tree and fails closed on symlinks, unsupported dependency declarations, dirty/untracked inputs, and oversized source files. The v0.1 package contract intentionally has an empty dependency set; external registry and version-conflict resolution are not implied or claimed.

Signing and verification apply bounded reads before parsing or hashing: package artifacts are limited to 64 MiB, private-key seed files to 256 bytes, provenance sidecars to 16 KiB, publisher trust stores to 1 MiB, source trees to 64 MiB, individual source files to 16 MiB, and source file count to 10,000. Oversized inputs are rejected before complete allocation. These limits do not replace trust-store authenticity or filesystem confinement.
