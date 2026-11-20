# Filesystem root-confinement architecture (PR #113 candidate)

This focused architecture view documents the filesystem authority and adapter boundary in the unmerged Issue #104 candidate. It is a candidate implementation diagram, not a statement that the capability is released or accepted on `main`.

```mermaid
flowchart TB
    Host["Host startup"] --> ProjectRoot["Project root from host process current working directory (independent of source and --target)"]
    Host --> HostPlatform["PlatformKind::current_host()"]
    HostPlatform --> SelectAdapter["Orchestrator selects using actual host platform"]
    SelectAdapter --> AdapterMetadata["Filesystem metadata: Linux on Linux; Windows on Windows; none on Android/macOS"]
    ProjectRoot --> Authority["FilesystemAuthority::open(host_cwd, limits) — final-component no-follow + retained-handle identity check"]
    Authority --> Root["Retained trusted root directory handle"]
    RequestedTarget["Requested target"] --> Confine["Resolve against fixed root; reject outside-root absolute targets"]
    Authority --> Confine
    Confine --> ReadAdapter["FileReadAdapter"]
    Confine --> WriteAdapter["FileWriteAdapter"]
    AdapterMetadata --> ReadAdapter
    AdapterMetadata --> WriteAdapter
    SelectAdapter -->|No matching native backend| FailClosed

    ReadAdapter --> LinuxRead["Linux read backend"]
    LinuxRead --> Openat2Read["openat2(root_fd, relative)"]
    Openat2Read --> ReadPolicy["BENEATH · NO_SYMLINKS · NO_XDEV"]
    ReadPolicy --> ReadChecks["Regular-file check · read-size limit"]

    ReadAdapter --> WindowsRead["Windows read backend"]
    WindowsRead --> NtOpen["NtCreateFile with retained parent handles"]
    NtOpen --> WindowsPolicy["OBJ_DONT_REPARSE · reparse / volume checks"]
    WindowsPolicy --> ReadChecks

    WriteAdapter --> LinuxWrite["Linux write backend — direct root children only"]
    LinuxWrite --> Nested{"Nested parent path?"}
    Nested -->|Yes| FailClosed["UnsupportedPlatform — no write attempted"]
    Nested -->|No| LinuxLeaf["openat2 leaf validation"]
    LinuxLeaf --> LinuxPolicy["BENEATH · NO_SYMLINKS · NO_XDEV"]
    LinuxPolicy --> LinuxType["Regular file · same device · nlink = 1"]
    LinuxType --> LinuxTemp["Exclusive temp sibling via root_fd"]
    LinuxTemp --> LinuxSync["write_all + sync_all"]
    LinuxSync --> LinuxRename["renameat(root_fd, temp, root_fd, leaf)"]
    LinuxSync -->|failure| LinuxCleanup["unlinkat temporary sibling"]
    LinuxRename -->|failure| LinuxCleanup

    WriteAdapter --> WindowsWrite["Windows write backend"]
    WindowsWrite --> WinWalk["Handle-relative parent traversal"]
    WinWalk --> WinShare["Retained root/parent handles omit FILE_SHARE_DELETE"]
    WinShare --> WinPolicy["OBJ_DONT_REPARSE · reparse / volume / link checks"]
    WinPolicy --> WinTemp["Exclusive temp sibling under parent handle"]
    WinTemp --> WinSync["write + flush"]
    WinSync --> WinRename["NtSetInformationFile(FileRenameInformation)"]
    WinSync -->|failure| WinCleanup["Delete temporary sibling"]
    WinRename -->|failure| WinCleanup

    WriteAdapter --> Other["Other platforms"]
    Other --> FailClosed

    classDef trust fill:#e7f0ff,stroke:#315a9b,color:#17345d
    classDef guard fill:#fff3d6,stroke:#a66b00,color:#5c3b00
    classDef blocked fill:#ffe3e3,stroke:#a33,color:#641b1b
    class Host,ProjectRoot,Authority,Root trust
    class ReadPolicy,WindowsPolicy,LinuxPolicy,LinuxType,WinPolicy,WinShare guard
    class FailClosed,Nested blocked
```

## Verified implementation mapping

| Diagram component | Source |
|---|---|
| Host-selected project root (process current working directory), authority injection, and target scope | `crates/one-cli/src/main.rs` |
| Retained root handle, size limits, target normalization | `crates/one-core/src/filesystem.rs` |
| Linux `openat2` read and direct-child atomic write | `crates/one-core/src/filesystem.rs` |
| Windows handle-relative traversal, no-delete-share parent handles, and atomic replacement | `crates/one-core/src/filesystem.rs` |
| Adapter injection and adversarial filesystem tests | `crates/one-core/src/adapter.rs` |
| Normative security and acceptance requirements | `docs/security/issue-104-filesystem-root-confinement-design.md` |
| Exact-SHA adversarial test contract | [`.github/workflows/e4-adversarial-regression.yml`](https://github.com/panglimanetizen01/one-windows-verification/blob/probe/issue-104-platform-metadata-20261120/.github/workflows/e4-adversarial-regression.yml) in the independent verifier repository |

## Explicit limitations

- Linux nested writes fail closed because a retained nested-directory handle can be moved outside the root after opening. Direct-child writes use the retained root handle as their parent authority.
- Linux direct-child and Windows write paths have protected E4 evidence on code-bearing source SHA `722d662e7a81711ba3607d26571f4048a292b3ed` (protected push run #44519577693). Linux nested writes remain fail-closed. The native Windows probe is limited to Windows Server 2025 build 26100.
- Other platforms fail closed.
- A successful artifact build or one passing platform job does not imply full platform support or Issue #104 acceptance.
- The full architecture map in PR #97 is a separate non-normative overview image. This focused source diagram supplements it; it does not claim that the image itself has been visually inspected or reconciled.

## Maintenance rule

When the authority boundary, path resolution, platform backend, write transaction, or failure cleanup changes, update this Mermaid source and the corresponding rendered Markdown view in the same PR. Keep the implementation-status and evidence ledger separate from this architecture description.


## Protected verifier reconciliation — 2026-11-19

PR #44 merged as `a44c32e18f884212c4c9250c4e7a9b8ad1ed9ccd`; protected branch `verification` now pins `722d662e7a81711ba3607d26571f4048a292b3ed`. Protected E4 run #44519577693 succeeded for the Windows native OBJ_DONT_REPARSE probe, Windows reparse/handle checks, and Unix no-follow/resource-bound checks. The native Windows probe evidence is limited to Windows Server 2025 build 26100. Linux nested writes and unsupported platforms remain fail-closed; Issue #104 remains OPEN / NOT ACCEPTED because the fixed-weight release gate has not passed.


## Host-platform selection reconciliation — 2026-11-20

The diagram now includes `PlatformKind::current_host()` and host-specific filesystem adapter metadata. Implementation: `crates/one-core/src/adapter.rs` (`filesystem_platforms`, `PlatformKind::current_host`) and `crates/one-core/src/interpreter.rs` (actual host passed to orchestration). Linux/Windows E4 metadata test and Linux distro integration evidence are recorded above. Android/macOS filesystem adapters remain unsupported; the separate Android runtime proof does not imply filesystem support.

## Exact-source hard-link regression evidence — 2026-11-20

The architecture itself is unchanged by the test-only hard-link increment. The cross-platform regression `filesystem::hardlink_tests::authority_rejects_hard_linked_targets_without_side_effects` now proves that a multiply-linked target is rejected for read and write and that the in-root and outside names retain their original bytes. E4 Linux and Windows jobs passed this test on source SHA `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` (verifier run #44651796986). The Mermaid diagram remains the architecture source of truth; no visual node or dependency changed. Linux nested writes and unsupported Android/macOS filesystem adapters remain fail-closed.

## Release-control traceability

The Mermaid diagram above remains the existing architecture source of truth; this release-control update does not change its nodes or security semantics. The prioritized work queue is [ONE_RELEASE_BLOCKER_REGISTER.md](../verification/ONE_RELEASE_BLOCKER_REGISTER.md), and exact-SHA evidence remains in [the evidence matrix](../verification/one-v0.1-evidence-matrix.md). Do not recreate the diagram to represent project status.

## Live implementation update — 2026-11-20 (host-owned filesystem root)

- Candidate PR #113 implementation commit `7cbcb1bf575defe03a2d9991dc5094569de95f03` changes filesystem authority acquisition: Linux now opens the supplied root with `O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC`, validates that the canonical path still identifies the opened directory by device/inode, and fails closed if the path changed during acquisition.
- CLI filesystem authority now comes only from the host process working directory. The source-file path and program-controlled `--target` no longer choose or widen the trusted root. A CLI regression fixture places the `.one` source outside the working directory while reading an in-root target.
- Implementation commits: `8b1d13e02c181fc06b97fb7f147e1ccd013d3aa1` (CLI root binding), `5e39430a341629eb9fda11ee4e11b0f6f40487ab` (CLI fixture), and `7cbcb1bf575defe03a2d9991dc5094569de95f03` (Linux root-handle acquisition).
- This candidate has NOT been built or tested after these changes. Do not transfer prior exact-SHA evidence. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable; Issue #104 remains OPEN / NOT ACCEPTED.

## Windows root no-reparse implementation — 2026-11-20

- Windows root acquisition no longer canonicalizes through the final component before opening it. It canonicalizes only the parent, preserves the final name, opens it with the existing `OBJ_DONT_REPARSE` / `FILE_OPEN_REPARSE_POINT` native path, then compares volume serial and file index against the path identity.
- Added `filesystem::windows_root_tests::authority_rejects_reparse_root_without_following_final_component` in commit `8dd92aebc033a94abb14a8d7ec498f6933d4ba8e`; it creates a directory junction and requires root acquisition to fail closed.
- Windows implementation commit: `b208a1a5bdf4a9ffb00a7c17aa4a401ed4a237ce`. This and the Linux/CLI root changes are NOT BUILT OR TESTED. Latest code-bearing candidate: `8dd92aebc033a94abb14a8d7ec498f6933d4ba8e`.
- Preserve Issue #104 OPEN / NOT ACCEPTED. Linux nested writes remain fail-closed; Android/macOS filesystem adapters remain unavailable.
