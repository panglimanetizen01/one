# Changelog

## Unreleased — One v0.1

- Started migration of canonical Project ONE CI execution to CircleCI Linux-only product/release jobs. Windows Server 2025 product/native verification is separated into `panglimanetizen01/one-windows-verification` with an explicit exact-SHA source pin and independent evidence manifest; the three legacy GitHub Actions CI workflows are removed from the migration branch. Exact-SHA Linux CircleCI and independent Windows results remain separate required evidence.
- Independently validated the Windows split on exact canonical source `4d22db6eae48739fd79dce732d99c9b6d86b5858` with verifier run [#11](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43592922789): Windows Server 2025 native execution returned exit code 0 / `Budi`, and the downloaded artifact ZIP, manifest, and executable hashes matched. CircleCI Linux on the same SHA remains FAILURE; no overall cross-platform PASS is claimed.

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
- Added a CLI regression test proving `filesystem.write` permission reaches the adapter and writes the expected text to the exact target. The filesystem-write CLI verification was first recorded on Termux/Android and remains platform-bound to its recorded exact SHA.
- Added the database-query source/runtime slice: `kueri database` → parser → semantic validation → resource-scoped `database.query` capability → `database.sqlite.query` adapter → parameterized SQLite `SELECT` → deterministic UTF-8 normalization → CLI, with executable evidence recorded for its current Termux/Android boundary.
- Added the HTTP GET source/runtime slice: `ambil dari` → parser → semantic validation → target-scoped `network.connect` capability → `network.http.get` adapter → bounded HTTP/1.1 GET → deterministic response-body normalization → CLI, with executable evidence recorded for its current Termux/Android boundary.
- Added the native hosted backend contract and implementation slice for Linux AArch64, Linux x86_64, and Windows x86_64 on PR #64. Semantic IR remains target-independent and native code generation uses the existing deterministic backend/toolchain boundary.
- Native Linux AArch64 is verified on the exact PR #64 SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd` for an AArch64/Linux host boundary: ELF AArch64 artifact built and executed successfully.
- Native Linux x86_64 is build-proven on the same PR SHA: ELF x86-64 artifact built successfully using the verified cross-linker selection and the dedicated regression test passes. Target execution remains pending because no x86_64 executor was available.
- Native Windows x86_64 is build-proven on the same PR SHA: PE32+ x86-64 artifact proven. Target execution remains pending because no Windows/Wine executor was available.
- PR #64 was merged into canonical `main` at `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`. The pre-merge native evidence remains bounded to its recorded implementation SHA until exact-SHA post-merge verification is completed.
- The first post-merge `One CI` run `39371290853` failed on both matrix jobs before any executable step (`steps: []`); this is recorded as an infrastructure/runner failure, not as source-code failure evidence.
- Updated the native backend contract map, implementation-status inventory, evidence matrix, release boundary, user feature-status guide, and verification record to reflect the merged state and preserve exact-SHA evidence boundaries.

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution.

 
- Fixed the CircleCI Linux job's shell mismatch by removing PowerShell-only `$env:PATH` assignments from Bash commands and adding a static contract guard against reintroduction. Exact-SHA CircleCI verification remains pending.

 
- Strengthened Windows native artifact verification to assert the PE32+ optional-header magic (`0x20B`) and bounds before claiming a PE32+ AMD64 artifact.

 
- Latest CircleCI retry on `36a4051913c0063f0237bf41cf4b25ebc250e493` started workflow `e42b0304-1241-4327-807d-03926d7d0422`; Linux and Windows jobs were pending at last poll. The previous candidate failed on both platforms; exact failure report remains inaccessible.

 
- Structural audit at `54386f415062d4c21a7acb5508d90fea4ba5cdef` passed: 60 required paths, 78 docs-index refs, fixtures 6/6/3, Linux shell guard, PE32+ guard, and no legacy GitHub Actions paths. CircleCI Linux/Windows were pending at last poll.
