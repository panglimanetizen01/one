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

Filesystem adapters use the canonical current working directory as their adapter-level project root. On Unix, parent components are opened relative to already-open directory descriptors with O_NOFOLLOW, and the final target is opened without following symlinks; targets are verified as regular files after opening. Linux uses STATX_MNT_ID and device identity to reject mount crossings, including same-device bind-mount crossings when the kernel exposes mount IDs, and fails closed if the mount ID cannot be obtained. Other Unix platforms use st_dev as a best-effort mount identity; that does not prove separation of same-device bind mounts. Windows rejects reparse points, normalizes extended-length and ordinary absolute paths before deriving a root-relative path, and verifies the final opened handle against the canonical root. Regular-file targets with more than one hard link are rejected on Unix and Windows before reads or truncation, including imported human-core modules opened by the compiler. Reads and runtime filesystem writes remain capped at 16 MiB. CLI `.oneir`, `.onepkg`, and module-lock outputs use the same root-confined, no-follow writer: the `target` directory must not be a symlink/reparse point, and final artifact targets must be regular single-link files. Tests cover symlinked parent/final components, hard-link aliases, a Linux /proc mount boundary, a concurrent parent-directory/symlink swap, and Windows directory-junction escape.

These checks enforce adapter-level confinement beneath the canonical current working directory on Unix and Windows, but they do **not** constitute a complete operating-system sandbox or a separately configurable adapter root. Linux mount-ID enforcement is stronger than the st_dev-only fallback on other Unix platforms; do not claim same-device bind-mount confinement on those platforms. Non-Unix/non-Windows targets fail closed for filesystem adapter reads/writes because a no-follow implementation is not available there.

## Bounded database query execution

The SQLite query adapter accepts read-only `SELECT` requests and applies layered resource limits: at most 100,000 rows, 16 MiB of rendered UTF-8 output, 1 MiB per text value, 32 result columns, 1 MiB SQL text, 256 bound parameters, a 100,000-opcode SQLite VM limit, and SQLite limits on expression depth, compound selects, function arguments, attached databases, and LIKE patterns. A 2-second progress-handler deadline and 250 ms database-lock timeout bound query execution/waiting. Blob values are rendered as hexadecimal only when the encoded value fits within the output budget. Resource-limit and SQLite size/interruption errors map to the typed `ResourceLimitExceeded` adapter error; partial results are not returned as success. These controls are per-query limits. They are now combined with a process-wide admission budget described below; neither mechanism accounts for memory retained by callers after a result is returned or all arbitrary interpreter heap allocations.

## Signed package provenance

The CLI exposes Ed25519 signing and verification for .onepkg artifacts. V2 signing creates a .onepkg.provenance sidecar that authenticates the publisher ID, artifact SHA-256 and validated package format, deterministic source-tree digest, clean Git revision, ONE CLI version, builder OS/architecture, Rust/Cargo versions, and normative empty-dependency model in one signed message. A V2 sidecar is accepted as source-bound build provenance only when its signature verifies against a separately configured publisher trust store and the source root is rechecked. V1 artifact-only sidecars remain verifiable for compatibility, but they do not satisfy source-bound provenance acceptance. A checksum alone is never treated as authentication.

Private key seed files must be stored outside the repository and must not be committed. The trust store must be provisioned through an independently trusted channel; including a public key only in the sidecar is not sufficient to trust a publisher. Signing requires a clean, Git-tracked source tree and fails closed on symlinks, unsupported dependency declarations, dirty/untracked inputs, and oversized source files. The v0.1 package contract intentionally has an empty dependency set; external registry and version-conflict resolution are not implied or claimed.

Signing and verification apply bounded reads before parsing or hashing: package artifacts are limited to 64 MiB, private-key seed files to 256 bytes, provenance sidecars to 16 KiB, publisher trust stores to 1 MiB, source trees to 64 MiB, individual source files to 16 MiB, and source file count to 10,000. Oversized inputs are rejected before complete allocation. These limits do not replace trust-store authenticity or filesystem confinement.

## Explicit filesystem root scope

The CLI accepts `--root <directory>` together with `--izinkan filesystem.read` or `--izinkan filesystem.write` and an explicit `--target <path>`. The policy grants access only when the target is lexically contained under the configured root by path components; parent-directory components and absolute/relative scope mismatches are denied. The adapter independently anchors filesystem opens beneath the canonical current working directory and applies descriptor-relative/no-follow or Windows handle/reparse checks.

Example:

```sh
one jalankan program.one --izinkan filesystem.read --root ./data --target ./data/input.txt
```

Without `--root`, the existing exact-target policy scope remains unchanged. The policy root and adapter working-directory root are separate defense-in-depth boundaries. Neither is a general operating-system sandbox, and the adapter root is not independently configurable. The runtime-wide active-operation and output limits below bound selected resource-heavy paths but do not account for every heap allocation or caller-retained result.


## Subprocess lifecycle and bounds

The process adapter applies a per-invocation wall-clock timeout, separate stdout/stderr byte limits, and an aggregate captured-output limit. It drains stdout and stderr concurrently to avoid pipe deadlocks. On Unix, the child is launched in a new process group and timeout/output-limit cleanup terminates that group. On Windows, `CreateProcessW` uses `PROC_THREAD_ATTRIBUTE_JOB_LIST` so the child enters a kill-on-close Job Object before its first instruction; timeout/output-limit cleanup terminates the job and its descendants. Handles and pipe endpoints are owned and closed on success and failure paths. A shared process-wide admission budget allows at most 8 concurrent resource-heavy operations total (subprocesses plus SQLite queries), with a stricter maximum of 4 concurrent SQLite queries; admission fails closed before process creation/query execution when a quota is exhausted. The interpreter also caps aggregate emitted output at 16 MiB per `execute` or event dispatch.

The regression suite covers Windows command-line quoting, empty and Unicode arguments, a Unicode/space-containing working directory, output overflow, timeout, immediate descendant-spawn stress, and descendant cleanup. The shared operation budget and per-execution output cap are runtime-wide admission/output bounds, not a complete heap quota: output retained by callers after an operation returns and arbitrary intermediate interpreter values are not fully accounted. These controls are not a general operating-system sandbox.
