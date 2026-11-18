# Filesystem root-confinement architecture (PR #113 candidate)

This focused architecture view documents the filesystem authority and adapter boundary in the unmerged Issue #104 candidate. It is a candidate implementation diagram, not a statement that the capability is released or accepted on `main`.

```mermaid
flowchart TB
    Host["Host startup / trusted project configuration"] --> Authority["FilesystemAuthority::open(root, limits)"]
    Authority --> Root["Retained trusted root directory handle"]
    Authority --> ReadAdapter["FileReadAdapter"]
    Authority --> WriteAdapter["FileWriteAdapter"]

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
    WinWalk --> WinPolicy["OBJ_DONT_REPARSE · reparse / volume / link checks"]
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
    class Host,Authority,Root trust
    class ReadPolicy,WindowsPolicy,LinuxPolicy,LinuxType,WinPolicy guard
    class FailClosed,Nested blocked
```

## Verified implementation mapping

| Diagram component | Source |
|---|---|
| Host-injected authority, root handle, size limits, target normalization | `crates/one-core/src/filesystem.rs` |
| Linux `openat2` read and direct-child atomic write | `crates/one-core/src/filesystem.rs` |
| Windows handle-relative traversal and atomic replacement | `crates/one-core/src/filesystem.rs` |
| Adapter injection and adversarial filesystem tests | `crates/one-core/src/adapter.rs` |
| Normative security and acceptance requirements | `docs/security/issue-104-filesystem-root-confinement-design.md` |
| Exact-SHA adversarial test contract | `../one-windows-verification/.github/workflows/e4-adversarial-regression.yml` in the independent verifier repository |

## Explicit limitations

- Linux nested writes fail closed because a retained nested-directory handle can be moved outside the root after opening. Direct-child writes use the retained root handle as their parent authority.
- Linux and Windows write paths are candidate implementations; the latest exact-SHA verification remains pending.
- Other platforms fail closed.
- A successful artifact build or one passing platform job does not imply full platform support or Issue #104 acceptance.
- The full architecture map in PR #97 is a separate non-normative overview image. This focused source diagram supplements it; it does not claim that the image itself has been visually inspected or reconciled.

## Maintenance rule

When the authority boundary, path resolution, platform backend, write transaction, or failure cleanup changes, update this Mermaid source and the corresponding rendered Markdown view in the same PR. Keep the implementation-status and evidence ledger separate from this architecture description.
