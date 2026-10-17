# One Human-Language Tool Orchestration and Governance Implementation Plan

> **Current status:** Tasks 1–13 and the language-level process/system vertical slice have source implementations and recorded historical executable evidence at their respective revisions. The current branch extends the chain with a SQLite database-query vertical slice, but the current database revision is not yet executable-verified.

**Goal:** Add a deterministic, general-purpose human-language tool-orchestration foundation to One so semantic operations can invoke typed external capabilities through governed adapters without making tool command syntax the language semantics.

**Architecture:** One source remains authoritative through the existing localized lexer → parser → AST → semantic analysis → CSIR/IR → runtime pipeline. Orchestration is a typed capability/runtime layer behind that semantic boundary. Natural-language wording does not grant authority; policy is evaluated over typed semantic requests.

**Evidence rule:** `Specified → Designed → Implemented → Verified → Supported`. Source presence is not execution evidence. Every later canonical SHA requires fresh exact-SHA verification.

## Completed foundation tasks

### Tasks 1–11 — COMPLETE

The typed orchestration domain model, semantic capability vocabulary, execution scopes/effects, deterministic governance, typed adapter contract, orchestration lifecycle, result normalization, audit records, DNS adapter, localization/conformance, cross-contract integration, and exact-SHA Linux/Windows verification are complete at their recorded revisions.

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

Independent mirror run `38903950612` passed this source-to-runtime slice on Ubuntu and Windows Server 2025 against canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb`, including build, Clippy, workspace tests, contract checks, deterministic artifact checks, filesystem-read success/denial/out-of-scope/backend/unavailable-adapter coverage, and offline verification.

The feature is intentionally **Eksperimental**: only the declared filesystem-read boundary is exposed through this source-level path. The existence of this slice does not imply that broader filesystem or orchestration support is complete.

## Task 14 — Language-level process/system orchestration — IMPLEMENTED / HISTORICALLY VERIFIED / EXPERIMENTAL

The process vertical slice implements the same dependency chain for one explicitly targeted process:

```text
One source
→ parser
→ semantic operation
→ capability resolution
→ governance
→ process.execute adapter
→ normalized result/evidence
→ One result/diagnostic
→ CLI/runtime
```

Source form:

```text
jalankan proses "rustc"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan process.execute --target rustc
```

Current implementation boundary:

- one explicit target is treated as the executable program;
- no additional process arguments are accepted yet;
- the target must be explicitly granted by policy;
- missing capability is denied;
- target mismatch is denied by governance;
- missing executable is classified as adapter/backend failure;
- stdout is normalized as UTF-8 text, with stderr used when stdout is empty.

Historical exact-SHA cross-platform evidence exists at `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800`. That evidence does not verify later database changes on the current branch.

## Task 15 — Language-level database-query orchestration — IMPLEMENTED / UNVERIFIED

The current branch adds a real SQLite source-to-runtime slice:

```text
One source
→ parser
→ semantic operation
→ capability resolution
→ resource-scoped governance
→ database.sqlite.query adapter
→ parameterized SQLite SELECT
→ normalized result
→ CLI/runtime
```

Source form:

```text
kueri database ":memory:" dengan "SELECT ?1" dan "42"
```

Explicit CLI permission:

```text
one jalankan main.one --izinkan database.query --target :memory:
```

Implementation boundary:

- `Capability::DatabaseQuery` is explicit and has no ambient grant;
- `SemanticCapabilityId::DatabaseQuery` maps the source operation to the capability;
- the request uses `ExecutionScope::Resource` for the database resource;
- `database.sqlite.query` uses real SQLite through `rusqlite` with bundled SQLite;
- SQL parameters are passed separately from the SQL text;
- only `SELECT` statements are accepted by the current adapter;
- results are deterministically normalized as UTF-8 text;
- source/core tests cover success, default denial, out-of-scope denial, backend failure, and normalization;
- CLI tests cover success, default denial, and out-of-scope denial.

The source/test revision is `0aa4acdf3796235eac432138fec829b6508879e3`. It is **not VERIFIED**: hosted GitHub Actions run #550 for this revision failed both Ubuntu and Windows jobs with zero reported steps, and the available container cannot reach GitHub DNS. No support claim is made.

## Task 16 — Broader language-level capability coverage — IN DEVELOPMENT

After the database slice obtains exact-SHA executable evidence, continue the same dependency chain without creating a second semantic authority:

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

1. filesystem/storage beyond the verified slices;
2. process/system services beyond the single-process execution slice;
3. database mutation/transactions after a separate capability contract;
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

DNS, filesystem-read, process execution, and filesystem write retain their previously recorded evidence boundaries. Database query is **Dalam pengembangan / UNVERIFIED** until current-head executable evidence passes. Broader language-level orchestration remains **Dalam pengembangan**.
