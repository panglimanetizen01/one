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

On Unix targets (including Linux, Android, and macOS), filesystem adapter opens resolve parent directories relative to already-open directory descriptors and apply O_NOFOLLOW to each path component and the final target. Read and write targets are checked as regular files after opening; read/write payloads remain capped at 16 MiB. Windows adapter opens reject reparse points, compare lexical and canonical paths, and verify the final path of the opened handle; a newly created file that fails handle verification is marked for deletion through that handle. Human-core module reads additionally verify the final handle remains under the canonical project root. Regression tests cover Unix symlinked parent directories, final-component symlinks, FIFO/non-regular inputs, and Windows directory-junction escapes.

These checks reduce the check-then-open race for the implemented Unix and Windows paths, but they do **not** constitute a complete filesystem sandbox: adapter capabilities still authorize explicit target paths rather than a separately configured project root, and other non-Unix/non-Windows targets retain a standard-library fallback. Do not claim that an arbitrary target path is confined to a project root merely because symlinks are rejected.

## Bounded database query execution

The SQLite query adapter accepts read-only `SELECT` requests and applies layered resource limits: at most 100,000 rows, 16 MiB of rendered UTF-8 output, 1 MiB per text value, 32 result columns, 1 MiB SQL text, 256 bound parameters, a 100,000-opcode SQLite VM limit, and SQLite limits on expression depth, compound selects, function arguments, attached databases, and LIKE patterns. A 2-second progress-handler deadline and 250 ms database-lock timeout bound query execution/waiting. Blob values are rendered as hexadecimal only when the encoded value fits within the output budget. Resource-limit and SQLite size/interruption errors map to the typed `ResourceLimitExceeded` adapter error; partial results are not returned as success. These controls do not establish a complete runtime-wide quota policy or a project-root sandbox.

## Signed package provenance

The CLI exposes Ed25519 signing and verification for .onepkg artifacts. V2 signing creates a .onepkg.provenance sidecar that authenticates the publisher ID, artifact SHA-256 and validated package format, deterministic source-tree digest, clean Git revision, ONE CLI version, builder OS/architecture, Rust/Cargo versions, and normative empty-dependency model in one signed message. A V2 sidecar is accepted as source-bound build provenance only when its signature verifies against a separately configured publisher trust store and the source root is rechecked. V1 artifact-only sidecars remain verifiable for compatibility, but they do not satisfy source-bound provenance acceptance. A checksum alone is never treated as authentication.

Private key seed files must be stored outside the repository and must not be committed. The trust store must be provisioned through an independently trusted channel; including a public key only in the sidecar is not sufficient to trust a publisher. Signing requires a clean, Git-tracked source tree and fails closed on symlinks, unsupported dependency declarations, dirty/untracked inputs, and oversized source files. The v0.1 package contract intentionally has an empty dependency set; external registry and version-conflict resolution are not implied or claimed.

Signing and verification apply bounded reads before parsing or hashing: package artifacts are limited to 64 MiB, private-key seed files to 256 bytes, provenance sidecars to 16 KiB, publisher trust stores to 1 MiB, source trees to 64 MiB, individual source files to 16 MiB, and source file count to 10,000. Oversized inputs are rejected before complete allocation. These limits do not replace trust-store authenticity or filesystem confinement.

## Explicit filesystem root scope

The CLI accepts `--root <directory>` together with `--izinkan filesystem.read` or `--izinkan filesystem.write` and an explicit `--target <path>`. The policy grants access only when the target is lexically contained under the configured root by path components; parent-directory components and absolute/relative scope mismatches are denied. The existing adapter-level no-follow / reparse-point checks still apply when opening the target.

Example:

```sh
one jalankan program.one --izinkan filesystem.read --root ./data --target ./data/input.txt
```

Without `--root`, the existing exact-target scope remains unchanged. This is an additional capability-policy boundary, not a general operating-system sandbox: callers must still use a trusted root, preserve adapter no-follow protections, and keep the host environment and mount configuration under control. It does not by itself provide a runtime-wide memory or open-resource quota.
