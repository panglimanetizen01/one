# Status Fitur One

Dokumentasi pengguna One harus membedakan kemampuan yang benar-benar tersedia dari rancangan masa depan.

| Status | Arti |
|---|---|
| **Didukung** | Kemampuan tersedia dan bukti yang diperlukan sudah ada untuk klaim yang dibuat. |
| **Eksperimental** | Kemampuan tersedia secara nyata pada boundary yang dinyatakan, tetapi cakupan atau kontraknya masih terbatas dan dapat berkembang. |
| **Dalam pengembangan** | Implementasi sedang dikerjakan; jangan menganggapnya selesai. |
| **Dirancang** | Desain atau kontrak sudah ada, tetapi kemampuan belum tersedia untuk penggunaan biasa. |
| **Belum tersedia** | Belum dapat digunakan pada versi yang sedang didokumentasikan. |

## Kondisi One saat ini

One v0.1 memiliki executable language core, grammar, parser, semantic pipeline, reference interpreter, CLI, project scaffolding, conformance fixtures, deterministic artifacts, capability model, interoperability melalui proses, beberapa vertical slice network, dan native backend implementation yang sudah masuk ke canonical `main` melalui PR #64. Verifikasi executable pada merge revision masih menunggu evidence exact-SHA.

### Native backend

**Bukti implementasi — canonical `main`:** PR #64 telah digabung ke `main` pada merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`. Implementasi native backend menggunakan One Semantic IR yang sama dan menyediakan target hosted Linux AArch64, Linux x86_64, dan Windows x86_64.

**Evidence executable yang tersedia sebelum merge:** implementation SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd` membuktikan Linux AArch64 ELF dibangun dan dieksekusi pada host AArch64/Linux; Linux x86_64 menghasilkan ELF x86-64; Windows x86_64 menghasilkan PE32+. Linux x86_64 dan Windows x86_64 belum memiliki target-execution evidence.

**Boundary saat ini:** bukti tersebut tetap terikat pada implementation SHA dan environment yang tercatat. Jangan menyebut native backend sebagai executable-terverifikasi untuk merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf` sebelum exact-SHA verification dilakukan pada `main`.

Post-merge `One CI` run `39371290853` tidak menghasilkan execution evidence: kedua matrix job (`ubuntu-latest` dan `windows-2025`) gagal dengan `steps: []`. Ini adalah evidence kegagalan infrastructure/runner, bukan bukti kegagalan source code.

### Orchestration

**Eksperimental — DNS source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk operasi `cari alamat` dengan izin CLI eksplisit.

**Eksperimental — filesystem read source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk operasi `baca berkas` dengan izin CLI eksplisit. Boundary ini telah diverifikasi lintas Ubuntu + Windows Server 2025 pada source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` melalui mirror run `38903950612`.

**Eksperimental — process execution source-to-runtime vertical slice:** `jalankan proses` telah diverifikasi lintas Ubuntu + Windows Server 2025 pada source `9038ac75f197753ff1ab6af7478e63e4831903a2` melalui mirror run `38929072800`. Slice ini dibatasi pada satu program yang ditargetkan secara eksplisit tanpa argumen tambahan.

**Eksperimental — filesystem write source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk `tulis ke berkas` dengan permission `filesystem.write` dan target file yang sama. Evidence executable tetap dibatasi pada Termux/Android untuk exact SHA yang tercatat.

**Eksperimental — database query source-to-runtime vertical slice:** Implementasi SQLite sudah executable-verified pada boundary Termux/Android yang tercatat, tetapi cakupannya tetap read-only parameterized `SELECT` dan belum menjadi klaim lintas-platform.

**Eksperimental — HTTP GET source-to-runtime vertical slice:** Implementasi `ambil dari` sudah executable-verified pada boundary Termux/Android yang tercatat. Boundary-nya tetap explicit `http://` URL, HTTP/1.1 GET, exact target scope, response-body normalization, dan explicit `network.connect` permission.

### Kemampuan yang masih future

HTTP methods/HTTPS/API features yang lebih luas, native C/C++, Python, Node, JVM bindings, compiler backend native penuh, package registry, IDE/editor, GUI framework integration, dan locale tambahan tidak boleh didokumentasikan sebagai kemampuan yang sudah tersedia hanya karena arah arsitekturnya sudah ditentukan.

Untuk daftar teknis yang lebih lengkap, lihat `docs/verification/one-v0.1-implementation-status.md` dan `docs/verification/one-v0.1-evidence-matrix.md`.

## Catatan audit keamanan dan integrasi — 2026-11-17

Status fitur harus dibaca bersama revisi source yang sedang digunakan. PR #108 kini mengintegrasikan source dan regression tests dari PR #98–#101 dalam satu kandidat; integration source commit `e8cd41ec212c3224bdbecf85eaf18c5850096e16`. Hasil PASS historis masing-masing PR tetap berlaku hanya pada exact SHA asalnya dan tidak dipindahkan ke kandidat gabungan. Matriks Ubuntu/Windows pada exact SHA gabungan belum dijalankan; kandidat tetap UNVERIFIED.

Perbaikan kandidat tersebut belum digabung ke main dan belum menjadi rilis. Karena itu:

- Process execution tetap Eksperimental. Batasnya 10 detik untuk direct child, maksimum 1 MiB per stream, dan reader shutdown grace 100 ms pada Unix. Descendant yang membuat session/process group baru masih dapat bertahan; ini bukan jaminan process-tree containment (Issue #105).
- Filesystem read/write tetap Eksperimental dan belum boleh dianggap sebagai sandbox project-root. Race-resistant path confinement dengan handle-relative open belum tersedia pada main (Issue #104).
- Database query tetap Eksperimental. Kandidat PR #108 kini mengintegrasikan read-only file-open dan regression test, tetapi exact-SHA gabungan belum diverifikasi; main belum berubah.
- Issue #82 untuk eksekusi runtime Android AArch64 masih terbuka; build artifact saja bukan bukti runtime execution.
- CircleCI belum aktif pada main. Migrasi masih draft pada PR #108 dan belum memiliki bukti job CircleCI terminal PASS.


## Pembaruan filesystem project-root — kandidat PR #113 (18 November 2026)

Status canonical `main` dan kandidat PR harus dibaca terpisah:

- **Canonical `main`:** filesystem read/write belum boleh dianggap sebagai sandbox project-root yang terkonfigurasi dengan batas root yang aman. PR #113 belum digabung dan tidak mengubah status rilis `main`.
- **Kandidat eksperimental PR #113:** implementasi `FilesystemAuthority` menggunakan root handle yang dibuat host, pembacaan Linux relatif terhadap root handle melalui `openat2`, dan backend Windows berbasis handle relatif. Kandidat ini juga menambahkan jalur write Windows dengan temporary sibling file, pemeriksaan handle, dan handle-relative rename untuk mengganti target; Linux write tetap fail-closed.
- **Verifikasi kandidat:** E4, Windows native reparse probe, full Ubuntu/Windows matrix, SEC-05, Debian/Ubuntu, dan Android AArch64 artifact build telah lulus pada source SHA `4c3bccaf59d965ce8b446e0b84cb84c00ff2eed6`. Setelah bukti tersebut, source kandidat berubah menjadi `5c5bfb53e086222f89dc8178426a8c6b65b50a04` dengan perubahan tambahan pada filesystem write. Hasil PASS dari SHA sebelumnya **tidak** membuktikan perubahan baru ini.
- **Status saat dokumentasi ini diperbarui:** verifikasi Windows/Linux exact-SHA untuk perubahan terbaru belum dibuktikan di sini; Android API 35 AArch64 runtime masih berjalan pada run yang mem-pin SHA sebelumnya; CircleCI `ci/circleci: one-linux` masih PENDING pada SHA terbaru. Karena itu fitur tetap **Dalam pengembangan / NOT VERIFIED pada kandidat terbaru**, bukan Didukung atau Accepted.
- **Batas keamanan Windows:** penggunaan `NtCreateFile`, `OBJ_DONT_REPARSE`, pemeriksaan reparse, dan handle-relative rename adalah implementasi kandidat yang masih harus divalidasi pada runner Windows dan terhadap batas kontrak API yang terdokumentasi. Satu probe pada Windows Server 2025 tidak membuktikan dukungan universal semua versi Windows.
- **Issue #104** tetap OPEN. Jangan mengandalkan kandidat PR #113 sebagai kemampuan yang sudah tersedia pada rilis atau canonical `main`.

Pengguna sebaiknya tidak mengandalkan filesystem capability sebagai sandbox keamanan project sampai issue ini diterima dan dirilis setelah seluruh acceptance criteria, verifikasi exact-SHA, serta dokumentasi final terpenuhi.


## Checkpoint terbaru kandidat filesystem confinement — 18 November 2026

Kandidat PR #113 telah bergerak melewati implementasi atomic write awal. Source HEAD terbaru yang diperiksa adalah `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9`; PR tetap OPEN / DRAFT / UNMERGED dan `main` tidak berubah.

- Pada source SHA `20ede9fb1d3e97c04f81d4fe8f79f642a8cc8e78`, independent E4 run [#44329905887](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44329905887) PASS, termasuk tujuh Windows filesystem regression checks (positive write, size limit, dan denied atomic replacement preservation), Unix adversarial checks, dan Windows Server 2025 native reparse contract probe.
- Setelah itu, source berubah pada SHA `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9`: rename staged sibling diubah menjadi operasi nama sederhana dari source handle dengan `RootDirectory = NULL`. Ini perubahan code-bearing; hasil PASS pada `20ede9...` tidak otomatis memverifikasi `f6acb...`.
- Independent verifier diagnostic pin sekarang menunjuk ke `f6acb...`; E4, cross-platform, SEC-05, Linux distro, Android artifact, dan Android runtime workflow baru sedang queued/in progress. Belum ada hasil final untuk SHA tersebut.
- CircleCI `ci/circleci: one-linux` PENDING pada SHA `f6acb...`.
- Status fitur tetap **Dalam pengembangan / NOT VERIFIED pada HEAD terbaru**. Implementasi atomic write telah lulus pada SHA sebelumnya, tetapi perubahan rename terbaru menunggu verifikasi exact-SHA. Jangan menyebut filesystem root confinement Didukung atau Accepted.
- Canonical `main` tetap belum berisi PR #113; Issue #104 tetap OPEN.

Jangan gunakan filesystem capability sebagai sandbox keamanan project sampai kandidat terbaru memenuhi seluruh acceptance criteria dan telah diterima serta dirilis.


## Checkpoint regression-test cleanup — 18 November 2026

A fresh Ubuntu/Windows candidate check at source `f6acb23a2a0038281c613fb6fd187bc04d2f3dc9` found Windows Clippy rejecting the test cleanup call `permissions.set_readonly(false)`. This was test-only code, not a production filesystem failure. The test now holds the destination open without `FILE_SHARE_DELETE` to cause a real Windows replacement denial, then verifies original bytes and temporary-file cleanup without changing permission attributes.

The fix is committed at source SHA `a79665056149bf1ff302de79a7b86a346e0353d8`. The independent verifier diagnostic pin now targets this exact SHA. E4 run [#44331243475](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44331243475) is QUEUED; CircleCI `ci/circleci: one-linux` is PENDING at this SHA. Do not classify the updated test as verified until the new run completes.

Filesystem root confinement remains in development in PR #113; canonical `main` remains unchanged and Issue #104 remains OPEN.


## Linux filesystem write scope — 18 November 2026

The PR #113 candidate now contains a Linux atomic write backend for files that are **direct children of the host-injected trusted project root**. It opens/validates the leaf with `openat2` using `RESOLVE_BENEATH | RESOLVE_NO_SYMLINKS | RESOLVE_NO_XDEV`, creates an exclusive temporary sibling relative to the retained root directory descriptor, writes and syncs it, and atomically replaces the leaf with `renameat`. Oversized writes, symlink leaves, hard links, and mount-point crossings fail closed.

**Nested Linux writes are intentionally unsupported for now.** A retained nested-directory handle can be moved outside the trusted root while an operation is in progress; this implementation refuses nested targets rather than risk writing through a moved parent directory. Windows candidate writes use a separate native handle-relative implementation.

The Linux write implementation and new tests are committed in candidate source SHA `cc54597b471493f4350d0dc0f3a2896babb33ea4`. The independent E4 verifier pin now targets this SHA; fresh E4 and related workflows are queued. This is IMPLEMENTED but NOT VERIFIED until those exact-SHA jobs finish. PR #113 remains draft/unmerged; canonical `main` has not gained this feature. Issue #104 remains OPEN / NOT ACCEPTED.


## Linux atomic writer compile correction — 18 November 2026

The first Linux writer candidate `cc54597b471493f4350d0dc0f3a2896babb33ea4` failed the Android AArch64 cross-target build because four stale `parent_handle` references remained after the implementation was restricted to the retained root descriptor. No runtime test or artifact was produced by that failed job.

Those references are now corrected to use the retained root file descriptor in source SHA `090c84dbab02e48af0a493faeb8d4fe0ff80a876`. The independent verifier pin targets this SHA. Fresh E4, cross-platform, Linux distro, Android artifact/runtime, and resource-guard jobs are queued or pending; CircleCI `ci/circleci: one-linux` is PENDING. The Linux direct-child writer remains IMPLEMENTED but NOT VERIFIED until the exact-SHA runs finish. Nested Linux writes remain fail-closed.


## Android AArch64 artifact revalidation — 18 November 2026

The Android AArch64 device-artifact build now passes on exact source SHA `090c84dbab02e48af0a493faeb8d4fe0ff80a876`: [independent build run #44332808717](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44332808717). The job verified the source pin, generated and linked the Android AArch64 executable, inspected the ELF header (`ELF64`, `AArch64`, Android linker `/system/bin/linker64`), and uploaded the artifact.

This is artifact/build evidence only. It does not establish Android API 35 runtime execution or full filesystem support on Android. The Android runtime, E4, cross-platform, distro, SEC-05, and CircleCI checks remain pending for the current candidate.


## Strict Clippy correction after Linux writer compile fix — 18 November 2026

The first compiling Linux writer candidate `090c84dbab02e48af0a493faeb8d4fe0ff80a876` built the Android AArch64 artifact successfully, but the Ubuntu distro Clippy step rejected one redundant field initializer in the Linux temporary-file cleanup guard. The code now uses Rust field shorthand in source SHA `3d8e0d0e3af54310837eb06d442d30a02df821d0`.

The earlier Android artifact PASS applies to `090c84d...`, not the new source `3d8e0d...`. The diagnostic verifier pin now targets `3d8e0d...`; fresh E4, cross-platform, distro, Android artifact/runtime, SEC-05, resource-guard, and CircleCI results must be checked on the new SHA before the updated candidate is considered verified.


## CLI integration update for Linux filesystem writes — 18 November 2026

The Debian 12 workspace suite found an outdated CLI regression test that still expected all non-Windows filesystem writes to fail. The CLI test now distinguishes the supported Linux direct-child case from nested Linux paths:

- Absolute direct-child target: expected to write successfully beneath the host-injected target parent.
- Nested relative target: expected to fail closed with `UnsupportedPlatform`, preserve original bytes, and create no temporary sibling.
- Other non-Windows/non-Linux platforms: remain expected to fail closed.

The test correction is source SHA `2fa1436daa102d3cf32125da6c440ae19cc83ba7`. The diagnostic verifier pin now targets this SHA. Fresh cross-platform, distro, E4, Android, SEC-05, and resource-guard runs are queued/pending; CircleCI is pending. This source remains NOT VERIFIED until those checks complete.


## CLI host-root selection and absolute-target confinement — 18 November 2026

The CLI previously derived the filesystem authority root from the requested absolute target's parent. That could let the target choose its own root and did not satisfy project-root confinement. It has been corrected.

The host now selects the root from the source/project context: nearest ancestor containing `one.toml`; otherwise the process working directory when the source is beneath it, or the canonical source parent for a standalone source outside the working directory. The requested target no longer influences root selection. Absolute targets outside the selected root are rejected.

New CLI tests cover:
- Absolute write target outside source project root: rejected, outside sentinel unchanged.
- Absolute read target outside source project root: rejected, sentinel is not disclosed.
- Relative source path: root resolves from host working context.
- Linux direct-child write: succeeds.
- Linux nested relative write: fails closed and preserves original bytes.

Source SHA: `79a98861080fed2ceaa0a7a2f56e07d6fbc19fca`; focused diagram updated in `d4687325f4b1236509bfe484cae46b72649d839f`. Diagnostic verifier pin now targets the latest source SHA. Fresh exact-SHA workflows are pending; do not mark host-root integration VERIFIED until they complete.
