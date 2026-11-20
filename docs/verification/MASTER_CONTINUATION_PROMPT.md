# MASTER CONTINUATION PROMPT — PROJECT ONE

**CONTINUOUS EXECUTION · NOTION-SKILL-FIRST · IMPLEMENTATION-FIRST · GITHUB-FIRST · EXACT-SHA · EVIDENCE-FIRST · ZERO-REWORK · FULL-DOCUMENTATION-SYNC · NO-PREMATURE-STOP**

Lanjutkan Project ONE dari repository dan dokumentasi persisten. Fokus utama adalah menyelesaikan implementasi produk v0.1 sampai seluruh scope resmi benar-benar tersedia. Jangan biarkan pekerjaan berputar pada audit/tes berulang sementara fitur produk belum selesai. Baca sumber otoritatif, lalu implementasikan backlog nyata di branch/PR yang tepat.

## Identitas dan satu work queue

- Main repository: https://github.com/panglimanetizen01/one
- Independent verifier: https://github.com/panglimanetizen01/one-windows-verification
- Notion Skill: https://app.notion.com/p/407a9d23afaa813cb942d4d6a4d1c5ce
- Active PR terakhir diketahui: ONE #113 dan verifier #51. Refresh HEAD/base/state sebelum bertindak.
- Satu-satunya work queue: [ONE_RELEASE_BLOCKER_REGISTER.md](ONE_RELEASE_BLOCKER_REGISTER.md). Jangan membuat register/roadmap/evidence matrix/prompt duplikat.

## Wajib baca sebelum bertindak

1. Fetch dan baca Notion Skill; checkpoint terbaru mengalahkan snapshot lama, tetapi jangan hapus sejarah.
2. Baca `AGENTS.md`, `README.md`, `docs/README.md`, konstitusi, requirements, decisions, development protocol, serta domain contract relevan.
3. Baca [release contract](../release/one-v0.1.md), [completion roadmap](../superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md), fixed ledger/DoD, [evidence matrix](one-v0.1-evidence-matrix.md), implementation status, verification index, dan live checkpoint.
4. Baca blocker register ini.
5. Periksa diagram yang sudah ada: full architecture map di PR #97 dan Mermaid source yang relevan. Jangan menggambar ulang/mengunggah duplikat.
6. Refresh GitHub PR/issue/branch/commit/status/workflow dan CircleCI pipeline mapping. Jangan percaya SHA/status lama tanpa readback.

## Cara kerja wajib

- Jalankan alur **DISCOVER → IMPLEMENT → INTEGRATE → DOCUMENT → FINAL VERIFY → RELEASE**.
- Prioritas utama: implementasi production code yang masih hilang pada Issue #104 (filesystem confinement), Issue #105 (process containment), dan backlog v0.1 yang diwajibkan kontrak. B-001 classifier/CircleCI tidak diulang tanpa failure baru. B-001 classifier/CircleCI sudah memiliki exact-SHA terminal success pada `979973cd...`; jangan mengulang investigasi itu tanpa failure baru yang relevan.
- Tambahkan regression tests bersama implementasi, tetapi tunda full matrix dan audit berulang sampai scope implementasi selesai. Jalankan pemeriksaan terarah lebih awal hanya bila diperlukan untuk mencegah perubahan security-sensitive yang tidak aman.
- Bedakan FACT, HYPOTHESIS, ROOT CAUSE, HISTORICAL, dan UNKNOWN.
- Untuk CircleCI, hubungkan job → workflow → pipeline → `.vcs.revision`; pastikan job anggota workflow. `--job-id` saja bukan bukti exact SHA. Jika `started_at=null`, jangan menyebutnya test timeout/failure.
- Semua klaim PASS harus mencantumkan exact source SHA, verifier SHA, platform, workflow/job, dan terminal result. Bukti parent SHA tidak berlaku ke child SHA.
- Pertahankan bukti yang sudah valid; jangan mengulang tes/audit tanpa perubahan input atau alasan teknis.
- Gunakan perubahan minimal dan test yang membuktikan akar masalah. Jangan mengubah kode security-sensitive tanpa build/test runner nyata.
- Setelah perubahan, sinkronkan dokumen yang terdampak dalam ekosistem existing: specs, roadmap/addendum, implementation status, evidence matrix, verification/live checkpoint, user guide/status fitur, diagram Mermaid, changelog, docs map, dan Notion Skill. Pertahankan sejarah dan cross-link; jangan membuat dokumen tandingan.
- Jangan berhenti setelah analisis jika masih ada tindakan aman yang dapat dieksekusi. Jika terhalang provider log/runner/akses, sebutkan bukti spesifik yang hilang dan minta hanya output redacted yang diperlukan.

## Batas keamanan dan otorisasi

Tanpa izin eksplisit, jangan merge PR, ubah `main`, force-push, hapus branch, publish/tag release, ubah protected verifier branch/pin, ubah fixed ledger/DoD/acceptance criteria, melemahkan security boundary, mengklaim platform support tanpa bukti, atau membeli plan/runner/credit. Jangan meminta/menampilkan token atau secret, termasuk `CIRCLECI_TOKEN`.

## Definisi selesai

ONE belum selesai/siap rilis sampai seluruh gate resmi release contract dan ledger terpenuhi pada satu frozen release-candidate SHA, required platform matrix terminal, security acceptance terbukti, dokumentasi sinkron, dan owner menyetujui tindakan rilis. Verifier pass parsial atau dokumentasi lengkap bukan release approval.

**Mulai sekarang:** baca Skill dan dokumen wajib, refresh exact state, buka blocker register, lalu lanjutkan implementasi produk pada blocker P0. Jangan kembali menjadikan B-001 atau pengulangan tes sebagai pekerjaan utama.


**Implementation continuation override — 2026-11-21:** Current PR #113 code candidate `4c46a41fe45baa7627557a2a25b9c71812840479` is NOT BUILT/TESTED. Continue production implementation for Issue #104 and #105 plus every remaining in-scope v0.1 product requirement. Issue #104 now includes Linux detached-mount nested-write candidate, Windows native backend, Android/macOS direct-child backends (Android statx mount ID; macOS fstatfs fsid+mount point); unsupported nested operations fail closed. Issue #105 has inherited seccomp process containment on Linux/Android and Windows Job Objects; macOS process execution remains fail-closed. Add focused regression tests alongside code, but defer broad verification cycles until implementation scope is complete. Do not repeat B-001 without a new relevant failure. Keep docs/Notion Skill synchronized; preserve `main`, verifier pins, fixed ledger/DoD, acceptance, and release gates.


## Implementation continuation refresh — 2026-11-21

Latest code-bearing candidate and PR #113 HEAD at this checkpoint: `fd1ccc5ca56f48a306ac8f8c551189f46b8fe44c` (OPEN / DRAFT / UNMERGED; NOT BUILT/TESTED). This supersedes the previous 2026-11-21 candidate pointer.

Issue #105 regression coverage now uses ignored helper tests in the same Rust test binary. The helper forks a descendant and calls `setsid(2)` directly, avoiding dependence on an external `setsid` utility on Android/Termux. The parent tests cover normal completion, timeout, and output overflow, record any escaped PID, and clean up the escaped group before reporting failure. The tests are NOT RUN.

Issue #104 implementation changes from the previous checkpoint remain: Linux guarded detached-mount nested writer; Android direct-child `statx(STATX_MNT_ID)` backend; macOS direct-child `fstatfs` filesystem-ID/mount-point backend; exclusive temp replacement with file and directory sync; hard-link regressions. Windows native backend remains unchanged. Nested Android/macOS operations fail closed.

Issues #104 and #105 remain OPEN / NOT ACCEPTED. No exact-SHA build/test or broad verification is claimed. Continue implementation-first and preserve all release/security gates.

## Process-group cleanup hardening — 2026-11-21

Latest code-bearing source/test candidate: `72eae103bc3304b0961c66a0cf61f716ddfa9ac4` (NOT BUILT/TESTED). Linux/Android process supervision now observes direct-child exit using `waitid(P_PID, WEXITED | WNOHANG | WNOWAIT)` rather than reaping via `try_wait()` before process-group cleanup. The child remains a zombie (its PID/PGID cannot be recycled) while the wrapper sends SIGKILL to the invocation process group; only then does `Child::wait()` reap it. The redundant post-reap group kill was removed. All timeout, output-overflow, wait-error and normal-completion paths terminate the group before reaping.

Process escape regressions use in-binary ignored helpers with direct `fork` + `setsid(2)` syscalls, record any escaped PID and clean up before failing. Three paths: normal completion, timeout, output overflow. NOT RUN. Issue #105 remains OPEN / NOT ACCEPTED.

## Implementation continuation — Linux FIFO blocking-read fix — 2026-11-21

- Code commit: `50923b545f2c804b24aa773ea5bb60129d1e0ff2`; it changes `crates/one-core/src/filesystem.rs` only.
- Linux `openat2` read now adds `O_NONBLOCK` so a FIFO with no writer cannot block before the adapter rejects its non-regular-file type. Added Linux regression `authority_rejects_fifo_without_blocking` using `mkfifo`.
- This is a focused implementation correction and regression-test addition, not a test result. No build, test, CI, verifier, or broad matrix was run. Issue #104 remains OPEN / NOT ACCEPTED.
- PR #113 remains OPEN / DRAFT / UNMERGED, base SHA `181ad350d8f0db53cda654c4d378425fbdd6ab24`. At the read before documentation synchronization, HEAD was `ac3e6037e102fc088c8137d8e7a7e1c8c6ffec99`; documentation commits following that read will advance HEAD without changing the code-bearing source.
- Continue implementation-first against the release blocker register. Preserve exact-SHA evidence, protected verifier pins, fixed ledger/DoD, fail-closed unsupported paths, and release authorization.


## Filesystem read-allocation and resource-limit hardening — 2026-11-21

Code commit: `8792f7a1f707f444d00026d916fa8c18b7f3e7f1`.

- Replaced the unchecked `u64 as usize` conversion used to preallocate filesystem read buffers with `usize::try_from` and `Vec::try_reserve_exact`. Oversized capacities now fail with a defined filesystem error instead of truncating on 32-bit hosts or panicking on reservation failure.
- Added focused Linux regression `authority_enforces_read_and_write_limits_without_partial_replacement`; it verifies an oversized read is rejected, an oversized write is rejected, and the existing target bytes remain unchanged.
- This regression is NOT RUN. No build, test, CI, verifier, or platform matrix is claimed. Issue #104 remains OPEN / NOT ACCEPTED.
