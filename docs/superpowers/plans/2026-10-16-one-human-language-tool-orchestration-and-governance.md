# One Human-Language Tool Orchestration and Governance Implementation Plan

> **Current status:** Tasks 1–11, the language-level DNS vertical slice, and the language-level filesystem-read vertical slice are complete and independently verified at their recorded implementation revision. The next dependency-ordered boundary is broader language-level capability coverage.

**Goal:** Add a deterministic, general-purpose human-language tool-orchestration foundation to One so semantic operations can invoke typed external capabilities through governed adapters without making tool command syntax the language semantics.

**Architecture:** One source remains authoritative through the existing localized lexer → parser → AST → semantic analysis → CSIR/IR → runtime pipeline. Orchestration is a typed capability/runtime layer behind that semantic boundary. Natural-language wording does not grant authority; policy is evaluated over typed semantic requests.

**Evidence rule:** `Specified → Designed → Implemented → Verified → Supported`. Source presence is not execution evidence. Every later canonical SHA requires fresh exact-SHA verification.

## Completed foundation tasks

### Tasks 1–11 — COMPLETE

The typed orchestration domain model, semantic capability vocabulary, execution scopes/effects, deterministic governance, typed adapter contract, orchestration lifecycle, result normalization, audit records, DNS adapter, localization/conformance, cross-contract integration, and exact-SHA Linux/Windows verification are complete.

## Task 12 — Language-level DNS orchestration — COMPLETE / EXPERIMENTAL

The first source-level vertical slice is executable:

```text
One source
→ parser
→ semantic validation
→ typed orchestration IR
→ target-scoped governance
→ network.dns.resolve adapter
→ normalized result
→ CLI/runtime
```

Source form:

```text
cari alamat "127.0.0.1"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
```

The runtime denies orchestration without an explicit capability grant and rejects targets outside the granted scope. The CLI-facing permission boundary is implemented and tested.

## Task 13 — Language-level filesystem-read orchestration — COMPLETE / EXPERIMENTAL

The second source-level vertical slice is executable:

```text
One source
→ parser
→ semantic validation
→ typed orchestration IR
→ target-scoped governance
→ filesystem.read adapter
→ normalized UTF-8 text result
→ CLI/runtime
```

Source form:

```text
baca berkas "data/contoh.txt"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan filesystem.read --target data/contoh.txt
```

The runtime denies the operation without an explicit capability grant, rejects targets outside the granted scope, and has explicit backend failure/unavailable-adapter coverage.

Independent mirror run `38901366447` passed this source-to-runtime slice on Ubuntu and Windows Server 2025 against canonical source `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b`, including build, Clippy, workspace tests, contract checks, deterministic artifact checks, filesystem-read success/denial/out-of-scope/backend/unavailable-adapter coverage, and offline verification.

The feature is intentionally **Eksperimental**: only the declared filesystem-read boundary is exposed through this source-level path. The existence of this slice does not imply that broader filesystem or orchestration support is complete.

## Task 14 — Broader language-level capability coverage — IN DEVELOPMENT

Extend the same dependency chain without creating a second semantic authority:

```text
One source
→ parser
→ semantic operation
→ capability resolution
→ governance
→ adapter
→ normalized result
→ One value/diagnostic
→ CLI/runtime
```

Priority:

1. filesystem/storage beyond the verified read slice;
2. process/system services;
3. database;
4. HTTP/API;
5. build/package;
6. testing;
7. platform APIs;
8. native interoperability;
9. frontend/backend tooling;
10. desktop/mobile adapters;
11. localization.

For every capability family:

1. write a failing end-to-end source test;
2. reproduce the current gap;
3. implement the smallest semantic/IR/runtime bridge;
4. enforce explicit policy and scope;
5. exercise success, denial, out-of-scope, unavailable-adapter, and adapter-failure cases;
6. verify CLI/runtime behavior;
7. run Clippy and workspace tests;
8. run fresh exact-SHA Linux/Windows verification;
9. only then promote status.

## Status boundary

- `Implemented` means code exists.
- `Verified` means executable evidence exists for the exact source revision.
- `Eksperimental` means a verified slice exists but coverage/support is intentionally limited.
- `Didukung` is reserved for a stronger support claim.

The DNS and filesystem-read vertical slices are **Eksperimental**. Broader language-level orchestration remains **Dalam pengembangan**.
