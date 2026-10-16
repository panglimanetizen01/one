# One v0.1 Verification Record

## Canonical repository

`panglimanetizen01/one`, branch `main`.

## Last independently verified feature baseline

The last independently verified source revision containing the language-level DNS orchestration and CLI permission vertical slice is `3c033aaa85b957d4ecf7ccba5a5eeda10f1e6ab1`. Independent run `38874287857` checked out that exact private-source SHA and passed the full release matrix on Ubuntu and Windows Server 2025.

That verified feature baseline contains the general-purpose orchestration foundation plus the first source-level path: `cari alamat` → parser → semantic validation → typed orchestration IR → target-scoped governance → `network.dns.resolve` → normalized result → CLI output.

## Exact-SHA evidence

Independent verification run `38874287857` in `panglimanetizen01/one-windows-verification` checked out canonical source commit `3c033aaa85b957d4ecf7ccba5a5eeda10f1e6ab1` and passed on both Ubuntu and Windows Server 2025. Both jobs passed canonical SHA verification, contract-artifact consistency, workspace build, Clippy, all workspace tests, real-network runtime tests, and platform-specific offline verification. The offline verifiers also checked deterministic `.oneir` and `.onepkg` artifacts.

The source-level CLI test explicitly verified `--izinkan network.discover --target <alamat>`, while the core runtime tests verified success with explicit grant and policy denial without it.

## Contract-artifact gate

`tools/check_contract.py` checks root release/security/contribution/CI artifacts, normative contracts, retained v0.1 artifacts, executable derivatives, offline scripts, documented artifacts, fixture minimums, grammar markers, and the no-GitHub-upgrade rule.

## Independent verification infrastructure

The temporary public repository `panglimanetizen01/one-windows-verification` is verification infrastructure only. It checks out the private canonical repository at an exact SHA. GitHub `main` remains the source of truth.

No paid GitHub upgrade is required or authorized merely to obtain hosted CI execution.

## Verification boundary

`VERIFIED` is reserved for claims backed by executable evidence for the exact recorded source SHA. A later canonical SHA requires a fresh exact-SHA verification before that later SHA is called verified.

The current documentation synchronization is deliberately treated as a later canonical revision and receives its own fresh exact-SHA verification before it is called the verified current head.
