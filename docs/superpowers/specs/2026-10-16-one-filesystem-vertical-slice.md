# One — Filesystem Read Vertical Slice

**Status:** Design approved; implementation not started.
**Date:** 2026-10-16
**Scope:** First filesystem capability exposed directly from One source through the existing governed orchestration runtime.

## Goal

Add one complete, evidence-backed filesystem read operation to One without creating a filesystem-specific execution path that bypasses the existing semantic, capability, governance, adapter, normalization, audit, and CLI boundaries.

## Boundary

The vertical slice is:

```text
One source
→ lexer/parser
→ AST/semantic operation
→ capability resolution
→ governance/policy
→ target scope
→ filesystem adapter
→ normalized result
→ interpreter
→ CLI output
```

The first operation is filesystem file read. It is intentionally narrow; directory mutation, writes, deletion, permissions changes, process execution, and arbitrary shell commands are outside this slice.

## Capability contract

The operation requires a dedicated filesystem-read capability with an explicit target scope. A request without that capability is denied before adapter invocation. A target outside the granted scope is denied before adapter invocation.

The adapter must declare its capability and platform compatibility. Registry absence must produce an unavailable-adapter error rather than a fake result. Backend I/O failure must remain a typed adapter failure.

## Source-level contract

Filesystem read must be expressible from One source rather than only through Rust APIs. The source operation must resolve to a typed semantic request consumed by the generic orchestration lifecycle.

The exact user-facing wording must follow the existing Indonesian-oriented language conventions and the repository's parser/semantic contracts. No natural-language wording may grant authority by itself.

## Generic orchestration requirement

The filesystem slice must reuse the existing orchestrator lifecycle:

1. validate request;
2. evaluate policy and scope;
3. select a compatible adapter deterministically;
4. invoke the adapter;
5. normalize result/error;
6. emit audit evidence;
7. return a typed One result or deterministic diagnostic.

If existing IR or interpreter structures are proven DNS-specific by failing tests, refactor only the minimum abstraction required to represent the filesystem semantic operation without duplicating the orchestration lifecycle.

## Required behavior

The implementation must have executable evidence for:

1. successful read;
2. denial without filesystem-read capability;
3. out-of-scope target denial;
4. unavailable adapter;
5. adapter/backend failure;
6. deterministic result/error normalization;
7. audit evidence for policy and execution state;
8. source-level end-to-end CLI execution.

The denied and out-of-scope cases must prove the adapter is not invoked.

## Determinism

Equivalent inputs and equivalent controlled backend conditions must produce deterministic semantic result/error representations. Raw platform-specific error text must not become the semantic contract where a stable classification is available.

## Security and authority boundary

The filesystem path is data, not authority. User wording, aliases, localization, generated source, or adapter identity must not bypass capability or scope checks. The adapter receives only the target and operation data authorized by the governance layer.

## Verification gate

Before this capability can be called Verified, the repository must pass the relevant unit/integration tests, Clippy, workspace tests, and an exact-SHA Linux/Windows verification through the existing independent verification path. Documentation must describe the capability as experimental/in development until that evidence exists.

## Non-goals

This slice does not claim general filesystem support, filesystem write support, arbitrary OS command execution, shell access, or support for every platform filesystem behavior.

## Success criteria

Filesystem read is complete only when it is specified, implemented, tested, independently verified on the supported verification matrix, and documented without overstating its status. The implementation must preserve the existing DNS behavior and generic governance guarantees.

## Extension: explicit file deletion effect — 2026-11-21

A file-delete operation is a distinct destructive effect, not an alias for write or general filesystem access.

- Semantic ID: `filesystem.delete`.
- Runtime capability: `Capability::FilesystemDelete`.
- Effect class: `EffectClass::Delete`.
- Adapter: `filesystem.file.delete`.
- Source form: `hapus berkas "path"`.
- The coarse `filesystem` grant and the read/write grants do not imply delete.
- The host must inject the same trusted root authority used by the other filesystem adapters; policy scope is evaluated before adapter invocation.
- Current adapter metadata advertises Windows only. Other platforms stay unavailable/fail-closed until their native primitive meets the root, object-identity, hard-link, symlink/mount, and race requirements.

The native Windows delete implementation passed its own exact-SHA CI slice, but the newly added language bridge remains unverified until its own build/test run completes. The feature is therefore implemented-in-source, not yet promoted to Verified.

## Exact-SHA verification update — 2026-11-21

The filesystem.delete language bridge passed One CI run #44871756593 on source/test SHA 0fa43eb0be53c8e79424597ece055147872b35c4. Ubuntu Build/Clippy/Tests passed (181/0/3 ignored); Windows Server 2025 Build/Clippy/Tests passed (167/0). The Windows explicit-grant interpreter path and actual junction escape sentinel-preservation regression passed. The adapter is Verified for Windows only; other platforms remain unavailable. Issue #104 remains OPEN / NOT ACCEPTED.
