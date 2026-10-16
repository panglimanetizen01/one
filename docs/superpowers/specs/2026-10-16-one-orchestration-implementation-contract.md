# One Orchestration Implementation Contract

Status: Verified on canonical `main` through independent exact-SHA cross-platform run `38896135846` against canonical source `e43b31fe9d7a5f62de4d3863b358aa28241fbf9c`.

## Semantic boundary

One semantics are represented by stable typed identifiers. Localized surface labels resolve to the same `SemanticCapabilityId`; surface wording does not grant authority.

## Capability boundary

A `CapabilityRequest` carries the semantic operation, required capability, execution scope, and effect class. Validation rejects empty scopes, semantic/capability mismatches, and incompatible capability/effect pairs.

## Governance boundary

`ExecutionPolicy` evaluates a typed request in deterministic order: invalid request → deny; explicit denial → deny; missing capability grant → deny; out-of-scope → deny; approval requirement → require approval; otherwise → allow. The policy is independent of locale or surface wording.

## Adapter boundary

`ToolAdapter` implementations expose metadata, declared capabilities, platform availability, typed arguments, and a typed invocation result. There is no free-form command field in `AdapterRequest`.

An adapter cannot execute a request unless its metadata declares the requested capability and the request itself is valid.

## Orchestration boundary

`Orchestrator` performs request validation, governance evaluation, deterministic compatible-adapter selection, and only then adapter invocation. A denied request never reaches the adapter invocation boundary.

## Result and evidence boundary

External output is normalized through an explicit expectation. Invalid UTF-8 is a deterministic normalization error. Execution evidence preserves adapter identity and distinguishes observed success, observed failure, and uncertainty.

## Audit boundary

Audit records distinguish policy decisions from attempted execution. A denied operation is not recorded as an attempted execution. An invocation is recorded as attempted even when its result is failure; uncertain outcomes remain uncertain.

## Localization/conformance

Current conformance fixtures cover Indonesian and English aliases for network connect/listen/discover and process execution. Materially different semantic candidates are rejected as ambiguous; repeated equivalent candidates are accepted.

## First real adapter

The first real adapter is `network.dns.resolve`. It uses the platform resolver directly, accepts a typed target scope, performs no shell/command-string substitution, and is exercised with a loopback target for deterministic integration evidence.

## Exact-SHA verification evidence

Independent run `38896135846` in `panglimanetizen01/one-windows-verification` checked out canonical source `e43b31fe9d7a5f62de4d3863b358aa28241fbf9c` exactly. Ubuntu job `116954563036` and Windows job `116956935967` passed build, Clippy, workspace tests, real-network runtime tests, offline verification, contract-artifact checks, deterministic artifact tests, DNS success/failure classification, and orchestration runtime tests.

The independent mirror is verification infrastructure only; GitHub `main` remains the source of truth. The canonical hosted runner is not used as the sole evidence path.

## Evidence rule

`Verified` means the implementation has executable evidence matching the recorded exact source SHA. `Supported` remains platform/backend-specific and must only be asserted where the relevant runtime evidence exists.
