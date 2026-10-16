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
- Independently verified the filesystem source-level orchestration/CLI revision on Ubuntu and Windows Server 2025 through exact-SHA checkout, contract-artifact checks, build, Clippy, workspace tests, real-network runtime tests, offline verification, and deterministic artifact verification.
- Refreshed the implementation-status and CLI user-guide documentation to distinguish the verified filesystem vertical slice from the still-in-development general orchestration catalog.

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution.
