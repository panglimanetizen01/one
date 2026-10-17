# Changelog

## Unreleased — One v0.1

- Added formal lexical and concrete grammar contracts.
- Added Rust 2024 workspace and Linux/Windows CI matrix.
- Added deterministic lexer, AST, parser, semantic analysis, Semantic IR, reference interpreter, diagnostics, and capability model.
- Added cross-platform process interoperability and core standard-library primitives.
- Added deterministic project lifecycle, conversational Indonesian CLI commands plus technical aliases, manifest validation, `.oneir`/`.onepkg` artifacts, conformance fixtures, and golden outputs.
- Added Linux/Windows offline verification scripts and deterministic artifact checks so verification does not depend on purchasing hosted CI capacity.
- Added the One Network semantic/runtime foundation and verified native runtime slices for DNS/system resolution, TCP, UDP, TLS/HTTPS, HTTP/1.1, HTTP/2, QUIC, TFTP, NTP, FTP, SMTP, POP3, and IMAP within explicit limits.
- Added the general-purpose human-language orchestration foundation: typed semantic capability requests, scoped deterministic governance, typed adapters, deterministic orchestration, normalized evidence, audit records, Indonesian/English conformance fixtures, and the first real `network.dns.resolve` adapter.
- Added the first experimental language-level orchestration slice: `cari alamat` reaches the governed DNS adapter through typed IR and returns a normalized CLI result when `network.discover` is explicitly granted for an exact target.
- Added the second experimental language-level orchestration slice: `baca berkas` reaches the governed `filesystem.read` adapter through typed IR and returns normalized UTF-8 text when `filesystem.read` is explicitly granted for an exact target.
- Added policy-denial, out-of-scope, backend-failure, and unavailable-adapter coverage for the filesystem source-level orchestration path.
- Independently verified the filesystem source-level orchestration/CLI revision on Ubuntu and Windows Server 2025 through exact-SHA checkout, contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, offline verification, and deterministic artifact verification at canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` via mirror run `38903950612`.
- Refreshed the implementation-status, release, CLI contract, grammar, and user-guide documentation to distinguish the verified filesystem vertical slice from the still-in-development orchestration catalog.
- Added the language-level process-execution slice: `jalankan proses` → parser → semantic validation → typed orchestration IR → target-scoped governance → `process.execute` adapter → normalized output → CLI, with TDD coverage for success, denial, and backend failure.
- Independently verified the process-execution source/runtime slice at canonical source `9038ac75f197753ff1ab6af7478e63e4831903a2` via mirror run `38929072800` on Ubuntu and Windows Server 2025. The verified boundary is one explicitly targeted program without additional arguments and explicit `process.execute` permission.
- Added the filesystem-write source-level slice: `tulis ke berkas` → parser → semantic validation → typed orchestration IR → target-scoped governance → `filesystem.write` adapter → CLI.
- Added a CLI regression test proving `filesystem.write` permission reaches the adapter and writes the expected text to the exact target. The filesystem-write CLI verification was first recorded at canonical `main` `a332dc5e58760460c26ebebb33f32f910bfdf4b3` on Termux/Android.
- Filesystem write remains **Eksperimental** and platform-bound to the Android/Termux evidence until exact-SHA Ubuntu/Windows verification exists.
- Consistency audit synchronized the normative CLI contract and grammar with the implemented `filesystem.write` surface and refreshed all current-main verification references to exact SHA `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` after exact-SHA Termux/Android verification of the current executable revision.
- Added the database-query source/runtime slice: `kueri database` → parser → semantic validation → resource-scoped `database.query` capability → `database.sqlite.query` adapter → parameterized SQLite `SELECT` → deterministic UTF-8 normalization → CLI. The source/test revision is `0aa4acdf3796235eac432138fec829b6508879e3`.
- Added database vertical-slice tests for adapter success, source-to-runtime success, default denial, resource out-of-scope denial, backend failure, deterministic normalization, and CLI success/denial/scope denial.
- Database query is **IMPLEMENTED / UNVERIFIED**. It is limited to read-only parameterized SQLite `SELECT` and is excluded from the verified/support boundary until exact-SHA executable evidence exists. Hosted CI run #550 failed both Ubuntu and Windows jobs with zero reported steps, so it is infrastructure/no-step evidence rather than source-failure evidence.
- Synchronized the grammar, capability/effect specification, orchestration plan/specification, implementation-status, evidence matrix, release contract, README, CLI guide, and feature-status guide with the database implementation boundary and evidence state.

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution.
