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


## Project-root filesystem boundary — Issue #104

Filesystem effects are not a security sandbox merely because they pass through a capability adapter. Before any release can claim project-root confinement, the host must inject an immutable trusted root authority, all accepted operations must remain handle-relative, and symlink/reparse, mount, hard-link, traversal, race, side-effect, and size-limit tests must pass on the exact source revision.

The current implementation candidate is tracked in [Issue #104](https://github.com/panglimanetizen01/one/issues/104) and its [design record](security/issue-104-filesystem-root-confinement-design.md). PR #113 remains unmerged. Linux candidate writes are limited to direct children of the retained root; nested writes fail closed. Windows native atomic replacement is under exact-SHA verification. Other platform backends remain unsupported.

Do not advertise or rely on this capability as a released project sandbox until Issue #104 is ACCEPTED and the release contract is satisfied.


## CLI authority-root selection

The CLI must derive the filesystem authority root from the source/project context, never from the requested target. It prefers the nearest ancestor containing `one.toml`; otherwise it uses the working directory for sources beneath it or the canonical source parent for standalone sources outside it. The requested target is independently checked against that fixed root. Absolute targets outside the root are rejected for both read and write operations.

Regression tests cover outside-root read/write attempts and verify that outside data is neither disclosed nor modified. This remains a draft PR #113 candidate pending exact-SHA verification.
