# One v0.1 — Effect and Capability Model

**Status:** Draft normative contract for review

## 1. Purpose

This contract defines how One controls operations that cross the language/runtime boundary. It is the security boundary between pure semantic computation and host effects.

## 2. Core rule

No One program, library, interpreter, compiler-generated backend, or extension may perform a host effect unless that effect is represented by an explicit runtime operation and permitted by the active capability policy.

A capability grant is permission, not proof that the external operation will succeed.

## 3. Effect classes

Minimum v0.1 classes:

- `console.read`
- `console.write`
- `filesystem.read`
- `filesystem.write`
- `filesystem.delete`
- `process.spawn`
- `process.signal`
- `network.connect`
- `network.listen`
- `environment.read`
- `environment.write`
- `native.call`
- `clock.read`
- `random.read`
- `database.query`

Database query is currently defined as a read-only, parameterized SQLite `SELECT` boundary. Database mutation requires a separate capability/effect contract and is not implied by `database.query`.

The current language-level HTTP GET slice uses `network.connect` with an exact URL target and is limited to explicit `http://` HTTP/1.1 GET requests. It does not imply HTTPS, redirects, arbitrary methods, custom headers/bodies, cookies, or API-specific authentication.

UI, device, cryptography, package installation, and other domains require explicit capability definitions before becoming standard effects.

## 4. Capability identity

A capability is identified by a stable name plus optional scoped parameters. Examples:

```text
filesystem.read
filesystem.write:/project/data
network.connect:example.com:443
network.connect:http://127.0.0.1:8080/
process.spawn:one-tool
database.query::memory:
```

Scope parameters are constraints, not user-visible fuzzy matching.

## 5. Default policy

A program starts with no ambient host capability. Capabilities must be explicitly granted by the runtime/project policy.

Pure computation, local variable access, expression evaluation, and control flow do not require host capabilities.

## 6. Check point

Every effect follows:

```text
CSIR effect operation
    ↓
capability identity + parameters
    ↓
policy evaluation
    ↓
allow → host adapter
 deny  → structured capability error
```

There must be no alternate path around this gate.

## 7. Policy result

Policy evaluation produces exactly one of:

```text
Allowed(capability)
Denied(capability, reason)
InvalidCapabilityRequest(reason)
```

Policy evaluation is deterministic for identical policy, request, and runtime version.

## 8. Path and resource scope

A path-scoped capability must resolve the requested resource against the policy-defined root before access. String-prefix checks alone are insufficient for security-sensitive containment. Symlink, junction, mount, case-sensitivity, and platform-specific path semantics must be handled by the platform adapter.

The language semantic layer must not invent platform-specific path rules.

Database capabilities use typed `Resource` scope rather than `Target` scope. The requested database resource must exactly match the policy resource. The current SQLite adapter opens only the explicitly scoped database resource.

## 9. Network scope

Network capabilities may constrain protocol, host, port, address family, and listen/connect direction. The current language-level HTTP GET source operation constrains the full `http://` URL as an exact `Target` scope. DNS, proxying, redirects, and certificate verification are host-policy concerns and must not silently expand a grant.

The HTTP GET adapter currently accepts only explicit `http://` targets, performs HTTP/1.1 GET, and returns the response body under a UTF-8 normalization expectation. HTTPS and richer HTTP/API behavior require separate capability contracts.

## 10. Process scope

Process execution requires `process.spawn`. The capability may constrain executable identity, arguments, working directory, environment inheritance, and standard-stream routing. A denied process request must not start a process. The adapter bounds the direct child's execution to 10 seconds and captures at most 1 MiB per output stream. Unix cleanup kills the original process group and bounds reader shutdown with a 100 ms drain grace; output overflow or a pipe that does not become idle produces an error. Descendants that create a new session or process group may still survive; strong Unix process-tree containment is not claimed (Issue #105). Windows execution uses a Job Object assigned at process creation and terminates remaining job members during cleanup.

## 11. Native calls

Native FFI requires `native.call` plus the specific native binding contract. Native access is never implicitly enabled because a library is present on the machine.

## 12. Time and randomness

Clock and randomness are explicit effects because they can break reproducibility. Deterministic test/runtime modes may supply controlled clock/random sources. The runtime must not secretly substitute wall-clock time or ambient randomness when deterministic mode requires controlled sources.

## 13. Effect propagation

A function or module that can perform effects must expose its effect requirements through semantic/runtime metadata. A caller cannot erase an effect requirement merely by wrapping the operation in another function.

Library composition must preserve the transitive effect requirements needed by execution.

## 14. Capability inheritance

Child function calls inherit only the capabilities explicitly available to the executing runtime context. A function cannot manufacture a broader capability from an ordinary value or from another denied request. Where a policy explicitly supports coarse capability inheritance (for example, a `filesystem` grant covering `filesystem.read` and `filesystem.write`), parent-level denials and approval requirements apply to those inherited child requests as well. Explicit denial takes precedence over an inherited grant.

Process children do not automatically receive all parent capabilities; inheritance must be explicitly configured by the process policy.

## 15. Diagnostics

Minimum capability errors:

- `ECAP001` — capability denied;
- `ECAP002` — invalid capability request;
- `ECAP003` — resource outside granted scope;
- `ECAP004` — effect unavailable on this platform;
- `ECAP005` — effect failed after permission was granted.

A denial diagnostic must identify the requested capability, the relevant source/provenance span when available, and a safe remediation such as requesting an explicit grant. It must not claim that the external operation was attempted when policy denied it.

## 16. No ambient escalation

The following cannot grant new capability:

- name resolution;
- type inference;
- imports alone;
- environment variables;
- filesystem discovery;
- network responses;
- LLM output;
- source comments;
- user-provided strings interpreted as capability names.

Capability grants originate only from the runtime/project security policy and explicitly defined trusted host integration.

## 17. Determinism and auditability

Capability decisions must be reproducible from policy and request inputs. Runtime should expose structured effect events suitable for diagnostics, testing, and audit logs without leaking secrets.

## 18. Semantic boundary

The semantic analyzer may annotate required effects, but actual permission evaluation belongs to runtime/policy. The compiler may reject statically impossible capability requirements only when the rule is explicit and deterministic; it must not silently remove or downgrade an effect.

## 19. Acceptance criteria

- no ambient host capability;
- every host effect crosses the capability gate;
- denied effects have no external side effect;
- scoped filesystem/network/process/database permissions are enforced by adapters;
- native calls require explicit permission;
- time/randomness are controllable for deterministic tests;
- capability errors are stable and provenance-aware;
- reference interpreter and compiled backends enforce equivalent capability semantics.

## 20. Race-resistant project-root filesystem confinement (Issue #104)

A filesystem adapter MUST operate beneath an explicitly configured trusted project root. The trusted root MUST be established by the host/runtime configuration boundary and retained as an open directory handle; it MUST NOT be inferred from the requested target or from an untrusted source value.

The original capability request and policy scope MUST be evaluated unchanged before adapter invocation. Adapter confinement is defense in depth and MUST NOT replace or weaken capability-policy checks, inherited denials, or approval requirements.

All filesystem access MUST use handle-relative component traversal from the retained root. String-prefix checks, canonicalize-then-open, check-then-open, or a path-based fallback are not security boundaries and MUST NOT be used for accepted operations.

- Unix implementations MUST use directory-descriptor-relative traversal (for example, openat with O_DIRECTORY and O_NOFOLLOW), reject symlink components, verify mount identity (for example, Linux statx STATX_MNT_ID) as well as device identity to prevent mount crossing; st_dev alone is insufficient for same-device bind mounts, and validate the opened leaf handle as an allowed regular file.
- Windows implementations MUST use a genuinely handle-relative open primitive, reject reparse-point traversal, and verify volume/file identity. Canonical path inspection followed by a second path-based open is insufficient.
- Relative paths and supported absolute paths that identify an in-root target MUST preserve their documented behavior, but actual access MUST remain handle-relative. Parent traversal, out-of-root paths, unsupported device/UNC namespaces, and ambiguous forms MUST fail closed.
- The initial strict policy MUST reject hard-linked leaf files unless a separately reviewed policy and adversarial tests establish a safe alternative.
- Reads and writes MUST have explicit bounded sizes. Writes MUST use an exclusive temporary file in the already-open parent directory and an atomic handle-relative replacement protocol with cleanup on failure.
- If a platform cannot provide the required race-resistant handle-relative behavior, its filesystem adapter MUST fail closed and the feature MUST remain experimental on that platform.

Acceptance requires Unix and Windows adversarial tests on the same exact source SHA. Tests MUST prove real side effects: no outside read, creation, or modification; nonzero matches in the independent verifier; race tests against parent/leaf replacement; and unchanged policy-denial/approval precedence. Static path metadata checks alone are insufficient.

The design and implementation checkpoint is tracked in [Issue #104 filesystem root-confinement design](../security/issue-104-filesystem-root-confinement-design.md). This section defines requirements only; it is not evidence that the implementation exists or has passed verification.
