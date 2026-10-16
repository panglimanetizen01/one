# One Orchestration Implementation Contract

Status: Verified on canonical `main` tree and merged into `main` at `8a8a3af449782b457ca1fc109b71fe9fdb006373`.

## Semantic boundary

One semantics are represented by stable typed identifiers. Localized surface labels resolve to the same `SemanticCapabilityId`; surface wording does not grant authority.

## Capability boundary

A `CapabilityRequest` carries the semantic operation, required capability, execution scope, and effect class. Validation rejects empty scopes, semantic/capability mismatches, and incompatible capability/effect pairs.

## Governance boundary

`ExecutionPolicy` evaluates a typed request in deterministic order:

1. invalid request → deny
2. explicit denial → deny
3. missing capability grant → deny
4. out-of-scope → deny
5. approval requirement → require approval
6. otherwise → allow

The policy is independent of locale or surface wording.

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

## Verification evidence

The implementation tree at exact head `8231b62d7b7d964b5bc3e270eb27a71eaae38187` was independently verified by `one-windows-verification` run `38855607871` on Ubuntu and Windows. The verified stages were build, Clippy, workspace tests, real-network runtime tests, offline verification, and contract-artifact checks. The subsequent canonical merge commit `8a8a3af449782b457ca1fc109b71fe9fdb006373` preserves the same tree SHA `ba2fc7f6af9b4c0e29b10d6313dfb71464f4e136`.

The canonical GitHub CI workflow has also been retried. Its jobs currently terminate before exposing any executable workflow step or log data; therefore no source-level root cause is asserted for that infrastructure-side CI result. The independent exact-tree cross-platform evidence above remains the executable verification record for this implementation.

## Evidence rule

`Verified` here means the implementation tree has executable cross-platform evidence matching the recorded tree. `Supported` remains platform/backend-specific and must only be asserted where the relevant runtime evidence exists.
