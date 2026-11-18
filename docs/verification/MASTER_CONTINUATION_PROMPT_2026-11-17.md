# MASTER CONTINUATION PROMPT — PROJECT ONE

**CONTINUOUS EXECUTION · NOTION-SKILL-FIRST · DOCUMENTATION-FIRST · GITHUB-FIRST · CIRCLECI-FIRST**

**EXACT-SHA · EVIDENCE-FIRST · NO-REWORK · NO-PREMATURE-STOP · SECURITY-FIRST · DOCUMENTATION-SYNCHRONIZED**

## 1. Identitas dan tujuan

Project: **ONE — Beginner-Friendly Programming Language**

Main repository: https://github.com/panglimanetizen01/one  
Independent Windows verifier: https://github.com/panglimanetizen01/one-windows-verification  
Notion Skill: **PROJECT ONE — Cross-Session Memory & Anti-Duplication Skill**  
Notion page ID: 407a9d23-afaa-813c-b942-d4d6a4d1c5ce

Tujuan sesi baru: lanjutkan pekerjaan berikutnya untuk membawa ONE sampai seluruh acceptance, keamanan, dokumentasi, CI, distribusi, dan release gate benar-benar selesai. Jangan mengulang audit atau pengujian yang sudah terbukti berhasil pada exact SHA yang sama. Mulai dengan membaca konteks otoritatif di bawah, kemudian langsung kerjakan next action yang belum selesai.

## 2. WAJIB DIBACA SEBELUM MENGUBAH APA PUN

Baca berurutan dan gunakan isi lengkapnya, bukan hanya judul atau snippet:

1. Notion Skill pada page ID di atas. Bagian checkpoint paling baru mengalahkan catatan historis. Perhatikan bahwa instruksi lama tentang ci-local extraction adalah scope historis yang sudah ditutup, bukan misi aktif.
2. Repository README.md dan docs/README.md.
3. docs/ONE_CONSTITUTION.md, docs/foundation/one-v0.1-foundation.md, docs/ARCHITECTURE.md, docs/REQUIREMENTS.md, docs/DEVELOPMENT_PROTOCOL.md, dan docs/DECISIONS.md.
4. Seluruh docs/spec/README.md dan kontrak normatif yang relevan dengan task berikutnya. Jangan hanya membaca satu file jika perubahan menyentuh lintas-layer.
5. Seluruh docs/panduan-pengguna/README.md dan status-fitur.md. Baca panduan terkait command/fitur sebelum mengubahnya.
6. docs/verification/README.md, docs/verification/forensic-audit-2026-11-17.md, docs/verification/one-v0.1-implementation-status.md, docs/verification/one-v0.1-evidence-matrix.md, docs/verification/one-v0.1-verification.md, docs/release/one-v0.1.md.
7. Completion ledger dan DEFINITION_OF_DONE.md dari PR #103 branch docs/restore-completion-ledger-dod-20261117. Jangan mengganti atau mengarang skor.
8. Diagram arsitektur ONE yang sudah ada di PR #97. Buka dan inspeksi visual file persis ini; jangan upload ulang atau menggambar ulang:
   https://github.com/panglimanetizen01/one/blob/docs/system-architecture-diagram-20261115/docs/architecture/one-full-architecture-map.png
   Blob SHA: c3a24f010ed56886ec240ee1c9a38b450331b04d
   Ukuran: 2,052,034 bytes; dimensi tercatat 5150 × 11552; SHA-256 tercatat e2f30a7fe3d11acd9ae76d26d2a4cd71ff3d4c5a3fa27cc13bdf9f35b313d638.
9. Setelah membaca semua itu, refresh GitHub live metadata dan status. Jangan menganggap SHA dalam dokumen sebagai head saat ini.

Jika dokumen historis menyatakan status berbeda, pertahankan fakta historisnya tetapi ikuti checkpoint terbaru dan live GitHub metadata untuk status sekarang.

## 3. Baseline GitHub yang telah dibuktikan

Canonical main saat audit: d35db95153f6ddd79885494c72f5aee16c899751. Repository PUBLIC. Main masih memakai GitHub Actions dan belum memiliki .circleci/config.yml.

Audit tree lengkap (tidak truncated): 232 file total; 98 file docs; 105 file dalam cakupan dokumentasi/kontrak/CI/tooling. Terdapat 200 branch dan 25 open PR pada snapshot audit. Jangan audit ulang seluruh 98 dokumen dari nol; gunakan forensic-audit-2026-11-17.md dan hanya periksa ulang dokumen yang berubah atau relevan dengan task baru.

### Kandidat migrasi dan source

- PR #102: branch fix/commit-cargo-lockfile-20261117; exact SHA 6c72d8e548bbbc28d85be39b8de427614989a038; GitHub Actions run #2229 / 44086429416; Ubuntu dan Windows Server 2025 PASS.
- PR #107: branch fix/unix-reader-shutdown-rebased-20261117; exact SHA 21e393157e52728d9510699ac6c9120987e3852c; run #2230 / 44088328030; Ubuntu dan Windows Server 2025 PASS.
- PR #108: branch ci/circleci-migration-rebased-20261117; base PR #102 source. The candidate includes PR #107's interop implementation, CI migration, and documentation synchronization. Pre-prompt-refresh candidate HEAD at 2026-11-18: `7b298a297a5aa2173139fa4203fedcc8b5581971`; the prompt synchronization itself was committed at `05bf359caf6b5beeed5e7328444869aa19b2c55a`. PR remains OPEN/DRAFT/UNMERGED. The candidate includes the Cargo.lock-aware cache-key change (`e91e092e5fca5b1e7ee2f8502b779dd5afd4888c`), normative cache contract (`c968c6a9dc067be85bc1160c95c0a91c555468ea`), release workflow deduplication (`a7fc834bb7e88f3fee535d3167ab6b870f658964`), release dedup contract (`d4152f1cf6b8f3b9300575f649824551b408f910`), and forensic audit synchronization (`7b298a297a5aa2173139fa4203fedcc8b5581971`). Refresh live metadata before every further write or status claim.
- PR #103 ledger previous exact SHA 041b359b8504551c2c1705093050d9c467368bac passed run #2231 / 44089449162 on Ubuntu and Windows. After the audit sync, current PR #103 HEAD is fde2b81b2f63743b990e9fbbade47ec8076e2151; run #2232 / 44092982606 is IN_PROGRESS on Ubuntu and Windows. Do not claim the new head passed until it is terminal.
- PR #97 architecture/docs branch: exact SHA 029471e0396504c030db60833fcda5b5baff24dd; run #2220 / 44082263286; Ubuntu and Windows PASS. Contains the architecture PNG.
- PR #95 is an older CircleCI candidate at 3f0ddb83b70d91b73aad3875d9eb1e4f5637064c. Do not merge it directly over newer process hardening.
- PR #106 old Unix reader branch is stale/diverged. PR #107 is the rebased candidate; do not force-push PR #106.

### Isolated fixes that already passed — DO NOT RE-RUN without relevant source/toolchain/verifier changes

| PR | Exact SHA | GHA run | Result |
|---|---|---|---|
| #98 | a83321bc41b3d0d28378841e29a6169abed8ac15 | #2018 / 43923604400 | Ubuntu + Windows PASS |
| #99 | 8800b885a4ffcb1931a7a3c71f3eaff888e9f5e9 | #2019 / 43923948205 | Ubuntu + Windows PASS |
| #100 | 6375356c9c275ace5fd35cf4a81799a9546e1e1d | #2020 / 43924465030 | Ubuntu + Windows PASS |
| #101 | cdf7bd5b8cff86e1ce271c8f03ef8c01ff61005f | #2138 / 44032422058 | Ubuntu + Windows PASS |
| #102 | 6c72d8e548bbbc28d85be39b8de427614989a038 | #2229 / 44086429416 | Ubuntu + Windows PASS |
| #107 | 21e393157e52728d9510699ac6c9120987e3852c | #2230 / 44088328030 | Ubuntu + Windows PASS |
| #103 | 041b359b8504551c2c1705093050d9c467368bac | #2231 / 44089449162 | Ubuntu + Windows PASS |
| #97 | 029471e0396504c030db60833fcda5b5baff24dd | #2220 / 44082263286 | Ubuntu + Windows PASS |

All these PRs are still separate unmerged candidates. Their PASS results are not combined-source evidence. Once the changes are integrated into one candidate, run the complete required matrix ONCE on the new exact SHA. Do not transfer PASS status from a parent SHA to a changed SHA.

## 4. CircleCI state and migration rules

CircleCI is the intended canonical Linux CI provider, but migration is not active on main. Main still has:
- .github/workflows/ci.yml
- .github/workflows/release-check.yml
- .github/workflows/windows-native-verification.yml

PR #108 candidate removes those three files and adds .circleci/config.yml. It also fixes two static migration defects discovered during this audit:
1. tools/check_contract.py formerly required the deleted GitHub Actions files; it now requires .circleci/config.yml, Cargo.lock, and the CI verification contract, and fails closed if any of the three retired workflows reappear.
2. tools/verify_offline.sh and tools/verify_offline.ps1 formerly omitted --locked on Cargo commands; all cargo build/clippy/test/run invocations now include --locked.

Static readback on the pre-audit-doc candidate found:
- CircleCI config present.
- No .github/workflows files on the candidate.
- Cargo.lock present.
- All 39 Cargo commands across CircleCI config and both offline scripts include --locked.
- User-guide internal links/status labels: 17 files checked; zero broken internal links and zero invalid status labels.

These are static checks, not execution. The updated PR #108 has no CircleCI status context/job evidence at the audit checkpoint. CircleCI UI/logs were inaccessible in the prior session. If an authenticated CircleCI connector is available, use it. Otherwise use CircleCI CLI/API in the user's authorized environment and ask only for redacted output. NEVER ask for a token to be pasted into chat and never claim CircleCI PASS without terminal job-level evidence.

CircleCI contexts #669/#671/#673 on refs without .circleci/config.yml are pipeline/configuration errors, not evidence of Rust source-test failures. Only pipeline #643 on old PR #106 had the explicit message “No configuration was found in your project”; do not generalize this exact text to other pipeline IDs without logs.

## 5. Open blockers — actual next engineering work

### P0 — integrate already-passing fixes into one candidate

PR #98–#101 each fix a confirmed main-branch bug and pass on separate exact SHAs:
- #98: one test manifest validation and safe package name/version components.
- #99: database.query must open existing database files read-only; missing paths must not be created.
- #100: interpreter lexical scopes must be popped/restored even after runtime errors.
- #101: parent filesystem deny and approval policies must inherit to child filesystem read/write capabilities.

Integrate these changes into one branch based on the refreshed PR #108 candidate without copying unrelated legacy milestone branches. Resolve conflicts against current PR #107/#102 source and normative contracts. Then run full Ubuntu + Windows exact-SHA CI once on the combined candidate.

### P0 — Issue #104 filesystem root confinement

Issue #104 remains OPEN. Current main adapter uses path-based std::fs::read/write and does not provide race-resistant project-root confinement. Required design:
- trusted root derived from explicit runtime/project configuration, never from the requested path;
- handle-relative component traversal (Unix openat + O_NOFOLLOW; Windows handle/reparse-point validation);
- explicit rejection/handling of parent traversal, symlink/junction, mount crossing, hard-link, and path-prefix cases;
- bounded read/write sizes and safe cleanup/atomicity;
- adversarial tests with actual side-effect assertions on Unix and Windows;
- preserve existing capability/scope gates.
Do not call filesystem capability scope a sandbox. Keep filesystem operations experimental until exact-SHA tests and independent review pass.

### P0 — Issue #105 Unix escaped-descendant containment

PR #102 enforces a 10-second direct-child deadline, 1 MiB per-stream output cap, Unix process-group best-effort cleanup, and Windows Job Object assignment/termination. PR #107 adds nonblocking Unix output readers and a 100 ms shutdown grace.

PR #107 PASS proves bounded reader shutdown only. A descendant that calls setsid() or creates a new process group may survive in production. Tests clean up their own escaped fixture; they do not prove the production descendant is gone before return. Keep Issue #105 OPEN. Do not add global subreaper or PID enumeration as a shortcut. Investigate a host-owned non-escapable boundary such as a properly delegated cgroup v2 provider with atomic placement and host-owned cgroup.kill; otherwise explicitly narrow the supported security contract and keep the limitation visible.

### P1 — Issue #82 Android AArch64 E3

Real Android AArch64 runtime execution remains NOT ACCEPTED. Cross-compilation/ELF inspection/emulator boot failure is not execution evidence. Use a real supported Android arm64 device/runtime, record API/ABI/source SHA/exact command/exit/logs.

### P1 — Issue #81 and #90

Reconcile old E4 subprocess issue #81 with PR #102/#107 and Issue #105 without closing it prematurely. Issue #90 runner allocation is infrastructure history, not source-test failure.

### P2 — old open branches and PRs

There are 25 open PRs in the audit snapshot. In addition to #95–#108 and #98–#103, older PRs include #76–#94 and several legacy health-scanner, milestone, MSYS2, and ci-local branches. Triage them by current relevance and dependency. Do not merge wholesale. Security scanner and CI tooling remain development-side; do not reintroduce them into ONE language/product UX. ci-local extraction is closed and must not be reopened without concrete residual evidence.

## 6. Completion and product constraints

- Definition of Done: 3 checked / 10 unchecked / 13 total.
- E1–E6 verified points: 0 until full acceptance contracts are met.
- E6: NOT IMPLEMENTED — FINAL RELEASE GATE NOT PASSED.
- No 100% completion, merge, release, or distribution readiness claim.
- Do not change ONE grammar, semantics, AST/IR meaning, compiler/runtime behavior, CLI vocabulary, or beginner UX merely to satisfy CI/security tooling.
- Keep security tooling and CI infrastructure development-side.
- No force-push, branch rewrite, PR merge, release, visibility change, token/secret change, paid-plan/credit purchase, billing mutation, or repository deletion without explicit authorization.
- Evidence must identify repository, branch, exact source SHA, verifier SHA where applicable, provider, run/job IDs, platform, executed steps, artifacts/checksums, and limits.

## 7. Documentation and Notion synchronization law

After every successfully completed task:
1. Update the normative contract/specification affected.
2. Update implementation status, evidence matrix, verification record, release contract, and completion ledger/DoD as relevant.
3. Update affected user docs, examples, CLI references, documentation indexes, diagrams/links, and generated/derived Markdown artifacts.
4. Run appropriate docs/contract/link checkers on the new exact SHA.
5. Append an exact evidence checkpoint to the Notion Skill page 407a9d23-afaa-813c-b942-d4d6a4d1c5ce.
6. Verify the update actually succeeded.
7. Preserve old records as history; new top-level checkpoint supersedes stale status, not historical evidence.
8. Never promote DoD checkboxes or E1–E6 points based on partial test success.

## 8. Mandatory first actions in the new session

1. Read this complete prompt.
2. Fetch the Notion Skill page and read the latest audit/checkpoint; do not assume its old ci-local/PR95 description is current.
3. Open and inspect the exact architecture diagram on PR #97.
4. Read docs/verification/forensic-audit-2026-11-17.md and all listed canonical docs/links.
5. Refresh live PR #108 HEAD/base, PR #102/#107/#103/#97/#95 states, Issues #104/#105/#82/#81/#90, and exact commit statuses.
6. Do not repeat any successful run listed in Section 3.
7. First seek authenticated CircleCI validation/log evidence for the updated PR #108 exact SHA. If unavailable, state the access limitation precisely and proceed to integrate PR #98–#101 fixes and implement Issue #104; do not stall or pretend CI results exist.
8. Continue execution until a real blocker requiring owner action is reached. After every task that passes, synchronize all relevant docs, derivative artifacts, ledger, and Notion Skill immediately.

**The purpose of this prompt is to prevent rediscovery and repeated successful audits. The next session must start from the current live PR #108 candidate and unresolved engineering gates, not restart the project or re-run completed checks.**


## CircleCI config review continuation — 2026-11-18

- Exact PR #108 HEAD at this checkpoint: `7b298a297a5aa2173139fa4203fedcc8b5581971`; OPEN/DRAFT/UNMERGED. Main remains unchanged.
- Cargo cache restore/save primary keys both use the same `one-cargo-v2` expression and include `Cargo.lock`; broader v1 fallback is performance-only.
- Duplicate `release-linux` job was removed. The ordinary `one-ci` workflow runs only when `run_release_checks=false`; `release-check` runs only when true. Both invoke one `one-linux` baseline job. Real-network tests and the offline verifier remain conditional release-only gates. The offline verifier intentionally repeats build/Clippy/tests under `CARGO_NET_OFFLINE=true`; this is a separate offline acceptance condition, not redundant online work to remove without redesigning the contract.
- All Cargo build/Clippy/test/run commands in the CircleCI config use `--locked`. Config, normative contract Section 14, and forensic audit have been read back from GitHub at this exact SHA.
- CircleCI connector/plugin discovery found no available authenticated CircleCI actions in this ChatGPT session. GitHub combined status for the exact SHA is empty. CircleCI config schema validation, pipeline creation, and terminal job execution remain NOT VERIFIED. No CircleCI PASS is claimed.
- Bootstrap review: `rust-toolchain.toml` specifies floating `stable`; do not claim reproducible Rust compiler version. Pinning requires an explicitly selected and tested Rust version. The runner uses `android:default` (mutable image tag); changing/pinning the image or removing cross-toolchains requires a measured exact-SHA trial and verification of all consumers. Do not weaken the acceptance matrix or alter release gates.
- ci-local extraction is CLOSED as a mission: standalone ci-local PR #2 is merged; ONE main code search found no ci-local references. The older Notion scope-correction block that calls extraction the active mission is superseded by this current master prompt and CircleCI task history; preserve it as historical record, not current scope.
- Next action: obtain CircleCI CLI validation and job-level logs in an authorized owner environment, then inspect actual execution. If unavailable, continue static config review only where evidence supports a safe change; never ask for a token in chat.


## LIVE CI TOOLCHAIN PIN UPDATE — 2026-11-18

- The CircleCI candidate pins the repository toolchain to Rust `1.100.0` in `rust-toolchain.toml` instead of floating `stable`.
- CircleCI bootstraps rustup with retries and `--default-toolchain none`, then reads the exact channel from `rust-toolchain.toml` and installs/selects it once. `tools/check_contract.py` enforces an exact x.y.z pin and verifies CircleCI consumes that pin.
- Independent verifier run `44243755612` previously observed Rust `1.100.0` on both Linux and Windows for source SHA `bac3a2e6f3ba6db5b821b9eafa6c5576f9fa8554`; this is a compatibility baseline, not evidence for the new commit.
- The updated exact source SHA must receive a fresh CircleCI result and an independent Linux/Windows candidate matrix. Refresh the live PR head before recording the new SHA. Keep PRs draft/unmerged and do not move the E4 pin.


## CI pin follow-up correction — 2026-11-18

- Static readback of the initial Rust pin commit exposed two formatting defects in the newly added shell backreference and contract-checker insertion before that candidate was accepted.
- The fix-up commit corrects the `sed` backreference to a single capture reference and rebuilds the contract-checker block as line-oriented parsing. The new exact SHA must be used for all follow-up validation; do not treat the initial pin commit as accepted evidence.

## Issue #104 implementation continuation checkpoint — 2026-11-18

- PR #110 is a stacked OPEN/DRAFT design checkpoint based on PR #108 branch at exact parent SHA `c4bb4b690e698a83e4ddaf279596479d3dc282ca`. Its design commit started at `af5fef0a38bd892983b7ea87f8822c1e3a5a043f`; latest PR #110 head must be refreshed after this prompt synchronization.
- Added `docs/security/issue-104-filesystem-root-confinement-design.md`, normative Section 20 in `docs/spec/one-v0.1-effect-capability.md`, and a docs index link.
- The design requires trusted-root authority from explicit host configuration and a retained root handle; handle-relative traversal on Unix and Windows; no symlink/reparse traversal; device/volume boundary enforcement; strict hard-link policy; bounded reads/writes; atomic same-directory writes; and adversarial tests proving no outside side effects.
- This is not an implementation or security PASS. No source code/tests changed or executed. Issue #104 remains OPEN; keep filesystem features experimental and E4/DoD unchanged.
- The older `security/issue-104-filesystem-confinement-20261118` branch is stale (0 ahead, 8 behind PR #108 at the last compare); do not implement on it. Continue on PR #110's branch based on the current PR #108 candidate.
- CircleCI provider validation and terminal job logs remain NOT VERIFIED. If access remains unavailable, continue engineering work rather than stalling.
- Next: inventory every FileReadAdapter/FileWriteAdapter construction and interpreter/CLI host configuration call site; define and implement explicit trusted-root injection; implement Unix and Windows handle-relative primitives with no path fallback; add real race/side-effect/resource tests; then run exact-SHA Unix and Windows verification and update all derived docs/evidence.

### Issue #104 verifier test names — 2026-11-18

Before implementation is considered test-ready, preserve the exact nonzero-match filters in the independent verifier workflow:
- Unix prefixes/counts: 2 `descriptor_relative_read_rejects_`, 1 `descriptor_relative_write_rejects_`, 1 `descriptor_relative_walk_rejects_`; exact tests `adapter::filesystem_nofollow_tests::descriptor_relative_read_race_never_follows_swapped_parent_symlink` and `adapter::filesystem_nofollow_tests::final_component_mount_boundary_denial_does_not_truncate_file`.
- Windows exact tests: `filesystem_reader_rejects_directory_junction_escape`, `filesystem_adapter_reads_absolute_target_inside_project_root`, `filesystem_adapters_confine_absolute_targets_to_project_root_without_side_effects`, `filesystem_write_adapter_writes_text_target`, `filesystem_write_adapter_rejects_oversized_content`, `filesystem_adapters_reject_parent_directory_traversal`.
The verifier fails if expected tests are filtered out or do not execute. Do not weaken these assertions. Refresh the workflow before the final run.

### Issue #104 mount identity requirement — 2026-11-18

Do not rely on st_dev alone for mount confinement: same-device bind mounts can share the same device number. On Linux, use statx STATX_MNT_ID (or stronger mount identity) together with device identity. On other Unix platforms, require an equivalent verified mount-identity primitive or fail closed. Keep this requirement in adversarial tests.

### Host-owned filesystem authority API — 2026-11-18

Implement the explicit root-injection contract from the Issue #104 design:
- one immutable ProjectFilesystem authority shared by read/write adapters;
- explicit FilesystemLimits for max read/write bytes;
- a dedicated interpreter constructor accepting the authority;
- CLI obtains current_dir independently from --target and injects the root only for filesystem operations;
- library callers and tests explicitly inject a trusted root; no deriving root from target or target parent;
- existing interpreter construction without a root fails closed for filesystem adapters.
Keep policy-first ordering and PR #101 inherited deny/approval behavior unchanged.

### Historical PR #80 reuse boundary

Use PR #80 only as a source of reviewed Unix primitives and test patterns. Do not copy its current_dir-derived root, per-operation root path walk, st_dev-only non-Linux fallback, non-atomic write path, or Windows canonicalize/check/open implementation. The current Issue #104 design and normative contract supersede those mechanisms.

### E4 verifier stale-candidate diagnosis — 2026-11-18

Historical run `43998206030` failed before any Issue #104 tests ran: both Unix and Windows logs show Cargo could not create `source/Cargo.lock` under `--locked`. The verifier's canonical source SHA was `e50748a1b25922d9090eba1f5fb90548d7c7d003`, which has no Cargo.lock. Current PR #108 base SHA `c4bb4b690e698a83e4ddaf279596479d3dc282ca` does have Cargo.lock. Treat this as a stale verifier input/lockfile precondition failure, not a security test result. Do not update the verifier canonical SHA to a candidate lacking the required Issue #104 test names. First implement tests and source, then update CANONICAL_COMMIT.txt to that exact candidate and run both jobs.
### Issue #104 current-source implementation map — 2026-11-18

Exact PR #108 base source SHA: `c4bb4b690e698a83e4ddaf279596479d3dc282ca`.

- `crates/one-core/src/adapter.rs`: replace the current parameterless `FileReadAdapter::new()` / `FileWriteAdapter::new()` and direct `std::fs::read/write` calls. Inject shared `Arc<ProjectFilesystem>`; read accepts no args, write accepts exactly one Text arg and must enforce max bytes.
- `crates/one-core/src/interpreter.rs`: add a dedicated constructor accepting policy + filesystem authority; this is the one adapter registration point. Preserve orchestrator policy-first evaluation and no-authority fail-closed behavior.
- `crates/one-cli/src/main.rs`: capture host cwd independently from `--target` and inject only for filesystem capability dispatch. Non-filesystem dispatch stays unchanged.
- `crates/one-core/src/lib.rs`: export public authority/limits types, not platform handle internals.
- `crates/one-core/Cargo.toml`: libc and windows-sys 0.61 already exist as target dependencies; no dependency changes until required APIs are verified.
- Migrate adapter unit tests and `filesystem_vertical_slice.rs` to explicit temporary roots; preserve policy/scope assertions. Review `filesystem_write_runtime.rs`. CLI build/package direct file operations are separate host operations, not automatically covered by adapter confinement.

Implementation sequence: authority + limits; Unix/Windows handle-relative primitives; adapters; interpreter; CLI; adversarial tests; Unix + Windows same-SHA validation.

Runner gate: no authenticated CircleCI tool or GitHub Actions workflow-dispatch action is exposed in this session, and the local clone route previously failed DNS resolution. Do not write uncompiled security-sensitive platform code. Static map only; no production source edits or test execution.

## Mandatory continuation update — 2026-11-18 (independent baseline verification completed)

- The previous note that no GitHub Actions workflow-dispatch route was available is superseded for this verifier task: a branch-scoped push-triggered Linux-only workflow was added to `one-windows-verification` and completed successfully.
- Verifier workflow file: `.github/workflows/linux-candidate-only.yml`; latest verifier branch commit `9e4b961af96ca51741355257cdc1710c999f20ee`. It validates the candidate pin, checks out its exact SHA, installs Rust 1.100.0, verifies source identity, installs MinGW with bounded retries, and runs the full `one-core` suite.
- Linux result: run `44267886845`, job `134676844501`, candidate `a20676be53e5f78c465832379d86c3675a2ed7c8`, SUCCESS, 239 passed / 0 failed / 1 ignored (27 result summaries).
- Windows result: run `44263674485`, job `134662858839`, same candidate SHA, SUCCESS, 232 passed / 0 failed / 1 ignored (27 result summaries).
- Do not repeat these unchanged-SHA baseline runs. The old matrix run's Linux job is stale; its Windows success is still valid. The separate Linux-only run is the passing Linux evidence.
- PR #110 remains OPEN/DRAFT/UNMERGED at `2ab82ea9cccb1c9aa0fef25c3591a1310ab84b5c`; CircleCI `one-linux` SUCCESS applies to that exact SHA. The independent Windows run above is pinned to PR #108 SHA `a20676be...`, not PR #110 SHA.
- Issue #104 remains OPEN / NOT IMPLEMENTED. Do not claim root confinement, E4 acceptance, DoD completion, or release readiness. Continue with the next source implementation step only when the exact Rust runner and cross-platform validation path are available; retain fail-closed design requirements.

