# One v0.1 — Platform Abstraction Contract

**Status:** Draft normative contract for review

## 1. Purpose

The platform abstraction layer maps platform-specific services to the platform-neutral runtime/effect model. Windows and Linux are first-class targets; platform differences must not redefine One semantics.

## 2. Boundary

```text
Common Semantic IR
        ↓
Runtime + Capability Policy
        ↓
Platform Adapter
        ↓
Windows / Linux / future platform
```

Platform adapters may differ internally but expose the same defined semantic operations.

## 3. Required adapter domains

Minimum v0.1 interfaces:

- filesystem;
- process;
- networking;
- environment;
- clock;
- randomness;
- console/standard streams;
- native integration boundary.

## 4. Capability enforcement

Adapters receive only requests already approved by the runtime capability policy. An adapter must not expand a capability or infer permission from availability of a host API.

## 5. Path behavior

Adapters implement platform-correct canonicalization, containment, symlink/junction handling, and access failures. The semantic layer remains platform-neutral.

## 6. Process behavior

Adapters normalize executable invocation, arguments, environment, working directory, streams, exit status, and signal behavior into the defined One effect model. Unsupported operations produce structured platform errors.

## 7. Networking

Adapters normalize connect/listen requests and expose policy-relevant failures without changing the program's semantic meaning. DNS and TLS behavior are platform/service details constrained by capability policy.

## 8. External nondeterminism

Wall-clock time, randomness, process scheduling, filesystem enumeration, and network responses are external inputs. Deterministic test adapters must be able to replace them with controlled fixtures.

## 9. Platform availability

If a platform cannot provide a required operation, execution must return a stable `EPLATxxx`/runtime platform error. The runtime must not silently emulate a materially different operation.

## 10. Cross-platform conformance

The same CSIR fixture must produce equivalent defined semantic results on supported platforms. Platform-specific paths, executable names, and external resources belong in fixtures/configuration rather than in the common semantic contract.

## 11. Acceptance criteria

- Windows and Linux share one semantic runtime contract;
- capability checks precede host access;
- deterministic adapters exist for conformance tests;
- unsupported platform operations are explicit failures;
- no platform adapter reparses source or changes language semantics.