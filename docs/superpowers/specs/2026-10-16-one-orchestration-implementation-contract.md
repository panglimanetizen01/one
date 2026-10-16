# One Orchestration Implementation Contract

Status: Implemented on `feature/one-orchestration-native`; final exact-SHA cross-platform verification remains a release gate.

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

## Evidence rule

The implementation status of each component is not equivalent to Verified or Supported. Final release status requires exact-SHA Linux and Windows verification through the independent verification repository and its build, Clippy, test, real-network, offline, and contract stages.
