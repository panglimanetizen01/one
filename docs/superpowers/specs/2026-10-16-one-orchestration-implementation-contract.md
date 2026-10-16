# One Orchestration Implementation Contract

Status: **Verified foundation boundary** on canonical `main` through independent exact-SHA cross-platform run `38903950612` against canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb`. Later source changes require fresh exact-SHA evidence.

## Semantic boundary

One semantics are represented by stable typed identifiers. Localized surface labels resolve to the same `SemanticCapabilityId`; surface wording does not grant authority.

## Capability boundary

A `CapabilityRequest` carries the semantic operation, required capability, execution scope, and effect class. Validation rejects empty scopes, semantic/capability mismatches, and incompatible capability/effect pairs.

## Governance boundary

`ExecutionPolicy` evaluates a typed request in deterministic order: invalid request → deny; explicit denial → deny; missing capability grant → deny; out-of-scope → deny; approval requirement → require approval; otherwise → allow. The policy is independent of locale or surface wording.

## Adapter boundary

`ToolAdapter` implementations expose metadata, declared capabilities, platform availability, typed arguments, and a typed invocation result. There is no free-form command field in `AdapterRequest`.

An adapter cannot execute a request unless its metadata declares the requested capability and the request itself is valid.

The process/system development slice adds `process.execute` as a typed adapter boundary. Its current implementation uses the explicit target scope as the executable program and does not accept additional arguments through the source-level operation.

## Orchestration boundary

`Orchestrator` performs request validation, governance evaluation, deterministic compatible-adapter selection, and only then adapter invocation. A denied request never reaches the adapter invocation boundary.

## Result and evidence boundary

External output is normalized through an explicit expectation. Invalid UTF-8 is a deterministic normalization error. Execution evidence preserves adapter identity and distinguishes observed success, observed failure, and uncertainty.

The process adapter currently returns its captured stdout as the primary normalized text output, falling back to stderr when stdout is empty. A failed backend invocation is surfaced as an adapter failure; a successful process result is still subject to the same normalization/evidence boundary.

## Audit boundary

Audit records distinguish policy decisions from attempted execution. A denied operation is not recorded as an attempted execution. An invocation is recorded as attempted even when its result is failure; uncertain outcomes remain uncertain.

## Localization/conformance

Current conformance fixtures cover Indonesian and English aliases for network connect/listen/discover and process execution. Materially different semantic candidates are rejected as ambiguous; repeated equivalent candidates are accepted.

The new source-level `jalankan proses` production is an explicit grammar operation, not arbitrary natural-language intent recognition.

## First real adapter

The first verified real adapter is `network.dns.resolve`. It uses the platform resolver directly, accepts a typed target scope, performs no shell/command-string substitution, and is exercised with a loopback target for deterministic integration evidence.

The process adapter is a later development boundary and must not be promoted to verified status until its own exact-SHA execution evidence exists.

## Exact-SHA verification evidence

Independent run `38903950612` in `panglimanetizen01/one-windows-verification` checked out canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` exactly. The recorded Ubuntu and Windows verification passed build, Clippy, workspace tests, real-network runtime tests, offline verification, contract-artifact checks, deterministic artifact tests, DNS success/failure classification, filesystem-read orchestration runtime, and orchestration runtime tests.

That run does **not** verify the later process/system source changes. A fresh exact-SHA run is required before the process slice can be labeled `Verified` or `Experimental`.

The independent mirror is verification infrastructure only; GitHub `main` remains the source of truth. The canonical hosted runner is not used as the sole evidence path.

## Evidence rule

`Verified` means the implementation has executable evidence matching the recorded exact source SHA. `Experimental` means a declared slice has that evidence but remains intentionally limited. `Supported` remains platform/backend-specific and must only be asserted where the relevant runtime evidence exists.
