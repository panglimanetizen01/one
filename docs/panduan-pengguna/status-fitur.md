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


## Manifest-root CLI regression test — 18 November 2026

Added a cross-platform CLI integration test proving that the nearest ancestor containing `one.toml` is selected as the filesystem authority root for a source under `src/`. The test writes to a direct-child target under that project root. This complements outside-root read/write rejection and relative-source tests.

Test source SHA: `1b7c3bb1dbc29a33c086b4df21a9996025c1e075`; diagnostic verifier pin updated to this SHA. Exact-SHA workflows are pending.


## Project-root ancestor lookup correction — 18 November 2026

The source-context resolver now continues searching ancestors above the process working directory for the nearest `one.toml` manifest before falling back to the working directory or standalone source parent. This avoids selecting a subdirectory as root when the source itself is directly beneath a project subdirectory.

Source SHA: `722d662e7a81711ba3607d26571f4048a292b3ed`; diagnostic verifier pin updated. Exact-SHA verification is pending.


## Exact-SHA checkpoint refresh — 18 November 2026

The latest code-bearing candidate remains `722d662e7a81711ba3607d26571f4048a292b3ed`. The current PR #113 HEAD is a documentation-only descendant `1eebed322c696fe4f958578a3c7947abce75e173`; the focused architecture diagram now explicitly shows retained Windows root/parent handles opened without `FILE_SHARE_DELETE` to prevent parent-directory rename races during the handle-relative walk and atomic replacement.

The independent diagnostic verifier pin is confirmed as `722d662...`. Fresh exact-source verification is still incomplete: Linux distro run #44335797294 completed SUCCESS: Debian 12 and Ubuntu 24.04 jobs both passed; E4 #44335797344 is queued; cross-platform #44335797200 is pending; Android artifact #44335797320, Android runtime #44335797240, SEC-05 #44335797365, and resource guard #44335797191 are queued. CircleCI `ci/circleci: one-linux` is PENDING on source `722d662...`: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94.

Classification remains IMPLEMENTED / NOT VERIFIED for the CLI host-root resolver and Linux direct-child writer; Windows handle-relative atomic replacement has historical E4 evidence only on earlier exact source SHA and needs the current exact-SHA run. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 OPEN / DRAFT / UNMERGED; canonical main and protected verifier pin remain unchanged.


## Exact-SHA verification refresh — 2026-11-18

Code candidate: 722d662e7a81711ba3607d26571f4048a292b3ed. The independent diagnostic verifier pin matches this exact source.

- E4 Unix + Windows filesystem adversarial matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797344.
- Windows Server 2025 build 26100 native probe: ordinary file open with OBJ_DONT_REPARSE succeeded; direct and intermediate junctions were denied with STATUS_REPARSE_POINT_ENCOUNTERED (0xC000050B); outside sentinel remained unchanged.
- Seven Windows filesystem regressions passed, including in-root absolute read, outside-root confinement without side effects, normal atomic write, oversized-content rejection, denied atomic replacement preserving original bytes and cleaning the temp sibling, junction escape rejection, and parent traversal rejection.
- Ubuntu + Windows Server 2025 cross-platform matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797200.
- Debian 12 + Ubuntu 24.04 distro matrix: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797294.
- Android AArch64 artifact build: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797320; artifact ID 14442665831; archive SHA-256 3bc37c20a92087464abb087be1fe174fac314dbddd635116656e34ad1cee4123. Android API 35 ARM64 runtime, run #44335797240, remains IN PROGRESS.
- SEC-05 Linux + Windows: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797365. The targeted interop.rs formatting check passed; the separate whole-workspace formatting diagnostic reports diffs but is configured non-blocking.
- Subprocess resource guard: PASS, run https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44335797191.

CircleCI one-linux on source SHA 722d662 reports FAILURE: https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94. The prior documentation HEAD b94e85db5d83c9a4bec582609405d9c65e4db841 separately reports PENDING: https://app.circleci.com/workflow/a09cd902-a88f-4884-a212-148ba0c6b80c/job/125775a6-33ba-4bac-b5f5-74249b8e0a19. CircleCI logs are unavailable through the current tools; failure cause is unknown and is not inferred.

Classification: host-root resolver, Linux direct-child writer, Windows adapter regressions, E4, cross-platform, distro, SEC-05, and resource guard are VERIFIED on the exact source SHA and stated environments. Linux nested writes remain fail-closed. Windows native probe evidence is limited to Windows Server 2025 build 26100. Android artifact is BUILD-VERIFIED only; Android runtime is not yet verified. CircleCI remains unresolved. Issue #104 remains OPEN / NOT ACCEPTED; PR #113 remains OPEN / DRAFT / UNMERGED; main and the protected verifier pin remain unchanged.


## CircleCI status correction — 2026-11-18

A fresh combined-status read confirms that CircleCI `ci/circleci: one-linux` is FAILURE on the code-bearing source `722d662e7a81711ba3607d26571f4048a292b3ed` (job https://app.circleci.com/workflow/9fb2893b-4373-4ef1-ad97-60b804ebedda/job/9f3acc50-11b9-47e5-b841-6a943300be94). The previously recorded documentation HEAD `b94e85db5d83c9a4bec582609405d9c65e4db841` is PENDING, and its descendant documentation HEAD `add130412cf42088ca9171fa2660295f7c30ae65` is also PENDING (job https://app.circleci.com/workflow/bfe4df8d-85a8-49bc-b7f2-21910a4692cd/job/d2aa7f1d-0c7a-4e7b-8ba4-ecaf595674c2). The failure cause on the code SHA remains UNKNOWN because CircleCI logs are inaccessible through the current tools. This correction does not change the exact-source independent verification results or Issue #104 acceptance classification.


## Pembaruan status pengguna — 19 November 2026

Pembatasan akses filesystem masih **eksperimental dan belum diterima sebagai fitur rilis**.
- Linux: pembacaan memakai root directory handle; penulisan hanya untuk anak langsung root. Penulisan bertingkat tetap fail-closed.
- Windows: traversal dan atomic replacement memakai handle-relative backend. Bukti native junction/reparse-point terbatas pada Windows Server 2025 build 26100.
- Platform lain tetap fail-closed. Keberhasilan build dan runtime Android API 35 ARM64 tidak berarti adapter filesystem Android sudah didukung.
- Protected E4 pada source `722d662e7a81711ba3607d26571f4048a292b3ed` lulus, tetapi fixed-weight release gate masih gagal karena beberapa workflow verifier lain perlu integrasi dan verifikasi ulang pada protected branch.
- PR #113 tetap OPEN / DRAFT / UNMERGED; Issue #104 tetap OPEN / NOT ACCEPTED.

Jangan menyimpulkan fitur aman untuk semua platform atau siap rilis hanya dari satu kelompok test yang lulus.

## Status pembaruan verifikasi — 19 November 2026

Perbaikan workflow verifikasi independen untuk pemeriksaan paket, instalasi compiler Linux, runtime Windows native, dan pengujian bahasa ONE v0.1 telah digabungkan ke branch verifikasi independen. Sebagian pemeriksaan protected-branch sudah lulus, sementara pemeriksaan lintas platform dan runtime Windows lainnya masih berjalan.

**Batas penting:** hasil verifikasi workflow bukan berarti seluruh fitur keamanan filesystem telah diterima atau aplikasi siap dirilis. Issue #104 masih OPEN / NOT ACCEPTED. Penulisan nested pada Linux dan adapter platform yang belum didukung tetap ditolak secara aman. Project ONE belum dinyatakan selesai atau 100% diterima.

PR #113 tetap draft dan belum digabung; branch utama ONE tidak berubah. Hasil CircleCI SUCCESS pada SHA 64c490e9045aa29506772f74e3697a63a66b9695 berlaku hanya untuk SHA tersebut. Commit dokumentasi berikutnya memerlukan hasil CircleCI baru.

## Hasil verifikasi terbaru — 19 November 2026

Enam workflow verifikasi independen setelah integrasi perbaikan telah selesai dengan status SUCCESS. Cakupannya meliputi integritas paket, pemeriksaan subprocess SEC-05, runtime native Windows, pengujian bahasa ONE v0.1 pada Linux dan Windows, serta regresi standard library dan database.

Hasil ini merupakan bukti untuk cakupan pengujian yang disebutkan, bukan pernyataan bahwa seluruh proyek sudah diterima. Issue #104 tentang pembatasan akses filesystem masih OPEN / NOT ACCEPTED. Penulisan nested pada Linux dan adapter platform yang belum didukung tetap ditolak secara aman. Project ONE belum dinyatakan siap rilis atau 100% selesai.

PR #113 tetap draft dan belum digabung. Branch utama ONE tidak berubah. CircleCI SUCCESS sebelumnya pada SHA 64c490e berlaku hanya untuk SHA itu; status untuk commit dokumentasi berikutnya harus diperiksa secara terpisah.

## CircleCI status update — 19 November 2026

CircleCI now reports SUCCESS for the exact PR #113 documentation SHA eeafcab579cebb762af34d1daf4830f95cba87e7 (workflow 0621c480-6054-4fc1-8c82-b9028f105735). This result applies only to that SHA. It does not change the draft status of PR #113 or mean Issue #104 is accepted. CircleCI step logs were not separately available for detailed test-count claims.


## Pembaruan bukti platform — 19 November 2026

Runtime native ONE pada Android API 35 arm64-v8a telah berhasil dijalankan pada exact source SHA `722d662e7a81711ba3607d26571f4048a292b3ed`; hasil ELF exit code 0 dengan marker `ONE_NATIVE_BACKEND_OK`. Ini memverifikasi cakupan runtime Android AArch64 untuk backend native.

Bukti tersebut **tidak berarti adapter filesystem Android sudah didukung**. Pembatasan filesystem tetap eksperimental: penulisan nested Linux dan adapter platform yang belum tersedia tetap fail-closed. Issue #104 masih OPEN / NOT ACCEPTED. Project ONE belum dinyatakan selesai atau siap rilis.


## Pembaruan status langsung — 20 November 2026

CircleCI `ci/circleci: one-linux` telah berstatus **SUCCESS** pada exact HEAD PR #113 `b0d854124191587882705c832e1825ad2a46cc8f` (workflow `ecd5d877-e5f5-429c-884c-426196fc7e78`, job `cc77e7cb-8daf-4d9c-845d-df991e0f68bf`). Ini adalah bukti status job untuk branch draft, bukan bukti bahwa konfigurasi sudah masuk ke branch `main` atau bahwa seluruh proyek siap rilis.

Verifikasi E4 filesystem independen tetap terikat pada source pin `722d662e7a81711ba3607d26571f4048a292b3ed` melalui run [#44519577693](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44519577693). Bukti Windows native junction/reparse-point tetap terbatas pada Windows Server 2025 build 26100. Penulisan nested Linux dan adapter platform yang belum tersedia tetap fail-closed. Issue #104 masih OPEN / NOT ACCEPTED; gate penerimaan berbobot tetap belum lulus.

## Pembaruan status — 20 November 2026

CircleCI `ci/circleci: one-linux` telah lulus pada exact HEAD PR #113 `051c7815cc5cf3dbca840ac8cbb38e3e663da6b7` (workflow `3b89115a-ff07-41ac-940e-fb78baccd7ac`, job `241f1d78-0aaf-4f47-8ea2-57fbd3b35a1c`). PR #113 tetap draft dan konfigurasi belum masuk ke `main`.

Bukti runtime native Android API 35 arm64-v8a kini juga memiliki artifact metadata yang telah diperiksa pada source pin `722d662e7a81711ba3607d26571f4048a292b3ed`; Issue #82 ditutup untuk tugas bukti runtime tersebut. Ini tidak berarti adapter filesystem Android sudah didukung. Issue #104 tetap OPEN / NOT ACCEPTED, penulisan nested Linux tetap fail-closed, dan proyek belum siap rilis.

## Pembaruan CI — 20 November 2026

CircleCI `ci/circleci: one-linux` telah SUCCESS pada exact HEAD PR #113 `26a34efb6d0908bd02738a7b05090f96051a6dc7` (workflow `a7016ead-ed2d-467f-9f1b-4d20c60a966f`, job `6cc9778c-307c-48fc-9111-39c97d288393`). Ini membuktikan status job untuk kandidat draft tersebut; konfigurasi belum masuk ke `main` dan PR belum digabung.

Bukti runtime Android API 35 arm64-v8a memiliki metadata artifact yang sudah diperiksa dan Issue #82 telah ditutup untuk cakupan runtime tersebut. Adapter filesystem Android tetap tidak dinyatakan didukung. Issue #104 masih OPEN / NOT ACCEPTED dan proyek belum siap rilis.

## Pembaruan dukungan filesystem — 20 November 2026

Adapter filesystem kini hanya mengiklankan backend native pada platform yang benar-benar tersedia: Linux pada build Linux dan Windows pada build Windows. Android, macOS, dan platform lain belum memiliki backend filesystem dan tetap fail-closed. Pemilihan adapter memakai identitas platform host sebenarnya, bukan nilai generik.

Bukti kandidat source `c7f460a4e38be61de635ad5af9f7ff987a941f01`: test metadata lulus pada Linux dan Windows melalui E4 run #44585270928; workspace test lulus pada Ubuntu 24.04 dan Debian 12 melalui run #44585282690. CircleCI untuk source ini masih PENDING. Issue #104 tetap OPEN / NOT ACCEPTED dan fitur belum dirilis ke `main`. Bukti runtime Android API 35 adalah bukti runtime native yang terpisah, bukan dukungan adapter filesystem Android.

## Pembaruan status filesystem — 20 November 2026

- Pemeriksaan independen pada source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` telah lulus untuk regresi hard-link di Linux dan Windows.
- File yang memiliki hard link ke luar project root ditolak untuk operasi baca dan tulis; pengujian memeriksa bahwa isi file di dalam maupun di luar root tidak berubah.
- Windows juga telah lulus pengujian junction/reparse-point, target absolut di dalam root, penolakan target di luar root tanpa side effect, atomic write, batas ukuran, dan parent traversal.
- Linux telah lulus pengujian no-follow, mount-boundary, traversal, race pada parent symlink, serta direct-child write.
- Status dukungan tetap terbatas: filesystem backend tersedia hanya pada Linux dan Windows. Android dan macOS belum memiliki filesystem adapter. Pada Linux, nested writes tetap ditolak secara fail-closed.
- Ini merupakan bukti regresi dan verifikasi teknis pada kandidat PR, bukan penerimaan rilis. Issue #104 tetap OPEN / NOT ACCEPTED; jangan menganggap fitur sebagai sandbox universal atau mengaktifkan dukungan platform yang belum tersedia.
- CircleCI `ci/circleci: one-linux` pada source yang sama berstatus SUCCESS. Bukti ini tidak otomatis berlaku untuk SHA berikutnya.


## Pembaruan bukti runtime Android — 20 November 2026

Kandidat PR #113 dengan source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` kini memiliki bukti eksekusi native backend pada Android API 35 ARM64 melalui verifier PR #51 HEAD `226360d00bc26f858128d3127591fce13ecd52d9`, workflow [#44652693606](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/44652693606). Job build #136011228119 dan runtime #136012787863 keduanya SUCCESS. Log menunjukkan ELF AArch64 berhasil dieksekusi pada emulator dengan exit code 0 dan output `ONE_NATIVE_BACKEND_OK`.

Bukti ini hanya untuk native backend runtime pada environment Android API 35 ARM64 yang diuji. **Ini bukan bukti dukungan filesystem adapter Android.** Adapter filesystem Android/macOS tetap belum tersedia dan fail-closed. Linux nested writes tetap fail-closed. Issue #104 masih OPEN / NOT ACCEPTED; kandidat PR belum digabung ke `main` dan belum menjadi rilis.

CircleCI pada code-bearing source `fb5c8bc...` berstatus SUCCESS (workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`). Namun CircleCI pada dokumentasi HEAD PR #113 `3c6853886b2d7b43f5bcfb021cff81d6a48ad48e` berstatus FAILURE (workflow `5f0f4f99-436b-4581-9096-0863954836c3`, job `34bb08ef-767c-45b8-b898-ee28d39e071d`, report `38db588c-4a20-468d-b344-0dd124659f63`). Failed step/root cause belum diketahui karena provider log belum dapat diakses; jangan mengubah konfigurasi berdasarkan dugaan.


## Checkpoint verifikasi terbaru — 20 November 2026

- PR #113 tetap OPEN / DRAFT / UNMERGED. Dokumentasi HEAD yang dibaca: `71d518ae4b9aa3b1bf05c738e9d6e862883ffcd0`; source implementasi yang diuji: `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20`. Base SHA tetap `181ad350d8f0db53cda654c4d378425fbdd6ab24`.
- CircleCI pada source implementasi `fb5c8bc...` = SUCCESS (workflow `3a9c0613-a1bc-43ae-b2d6-319841061048`, job `c4faf162-80e9-4f17-a83a-a72634bb592f`). CircleCI pada dokumentasi HEAD `71d518a...` = FAILURE (workflow `980d47d2-704a-426f-87b4-b0687657f42d`, job `a288a97c-38ff-448b-b74d-8abcea612342`). Step gagal dan akar penyebab belum diketahui; jangan menyimpulkan penyebab atau mengubah konfigurasi tanpa log.
- Verifier PR #51 tetap OPEN / DRAFT / UNMERGED pada HEAD `e5632e75c473546527c7e9a404262631d827f429`, dengan pin branch kandidat `fb5c8bc...`. E4 adversarial run #44656893733 SUCCESS: test hard-link yang sama benar-benar dijalankan dan lulus di Linux serta Windows. Linux distro #44656893931, Windows native runtime #44656893882, cross-platform #44656893832, SEC-05 #44656893697, artifact integrity #44656893650, resource guard #44656893627, E1 Windows #44656893639, dan focused stdlib/database #44656893788 juga SUCCESS pada workflow verifier HEAD ini.
- E1 Milestone 1 #44656893655 dan Android AArch64 runtime host #44656893597 masih IN_PROGRESS pada verifier HEAD terbaru. Bukti Android API 35 ARM64 emulator #44652693606 telah SUCCESS pada source `fb5c8bc...` di verifier HEAD sebelumnya; cakupannya native backend runtime, bukan filesystem adapter Android.
- Adapter filesystem tetap hanya tersedia pada Linux dan Windows. Android/macOS tetap fail-closed; Linux nested writes tetap fail-closed. Issue #104 tetap OPEN / NOT ACCEPTED. Tidak ada perubahan pada `main`, protected verifier pin, ledger, atau status merge.

## Filesystem root-confinement candidate — 20 November 2026

Filesystem read/write remains **Eksperimental** in the unmerged PR #113 candidate. The trusted root is the host process current working directory; the source file path and `--target` do not select or widen it. Linux currently supports reads and direct-child atomic writes, while nested writes fail closed. Windows uses handle-relative traversal and atomic writes. Android/macOS filesystem adapters remain unavailable.

The latest candidate implementation is `7cbcb1bf575defe03a2d9991dc5094569de95f03`; its host-root and Linux root-handle changes have not yet been built or tested. This is not a release claim, sandbox certification, or status change on `main`.

## Android host identity — 2026-11-20

- `crates/one-core/src/platform.rs` now exposes `Platform::Android` and returns it for `target_os = "android"` instead of collapsing Android into `Platform::Other` (`f8e46856803c9f45f54402e71d7e19853bdf46bc`).
- This is host identity only; it does NOT enable Android filesystem operations. The filesystem adapter remains fail-closed on Android until a safe resolver is available that avoids process-killing seccomp behavior and preserves root/symlink/mount/race guarantees.
- The platform unit test was updated but has NOT been run.

## Explicit desktop/mobile host identity — 2026-11-20

- `Platform` now identifies Windows, Linux, Android, and macOS explicitly rather than collapsing Android/macOS into `Other`; the platform unit test covers all four target cfgs (`5846eb70f2a462331873cc0023d1f1ab1106f86a`).
- This is platform identification only. It does not imply filesystem adapter support on Android/macOS; those adapters remain fail-closed pending platform-safe resolver implementations.
- The platform test has NOT been run.

## Android filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `7dd7b0049a7f5d71ac8a3d59570c2975fed835ec` (NOT BUILT OR TESTED).
- Android now has a native `FilesystemAuthority` root open and a handle-relative direct-child read/atomic-replace backend. Root and leaf are opened without following final symlinks; `statx(STATX_MNT_ID)` is required to reject mount crossings; reads reject non-regular and hard-linked files; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The backend deliberately avoids `openat2` because Android seccomp may terminate the process with `SIGSYS` for that syscall. It requires kernel `statx` mount-ID support; if unavailable it fails closed.
- Scope is intentionally limited to direct children of the trusted root. Nested Android paths return `UnsupportedPlatform`; no claim of full Android filesystem support or Issue #104 acceptance is made.
- Three Android-targeted regression tests cover direct-child read/write, leaf-symlink and nested-path denial, and final-component root-symlink rejection. They have NOT been run. Actual Termux/device execution is still required.
- Linux nested writes remain fail-closed; macOS filesystem adapter remains unavailable. Issue #104 remains OPEN / NOT ACCEPTED.

## macOS filesystem adapter implementation candidate — 2026-11-20

- Source/test candidate: `4be1b8fd579092d8195c3fb775b7c765f6221814` (NOT BUILT OR TESTED).
- macOS now has a native retained-root open and direct-child read/atomic-replace backend. Root and leaf operations use directory/file descriptors with `O_NOFOLLOW`; file and root device identity are compared; hard-linked and non-regular targets are rejected; writes use an exclusive temporary sibling, `fsync`, and root-relative `renameat`.
- The macOS adapter is deliberately limited to direct children of the trusted root. Nested paths return `UnsupportedPlatform`. This implementation has no target-host execution evidence yet; its mount/device and race behavior still require platform-specific review.
- Three macOS-only regression tests cover direct-child read/write, symlink/nested-path denial, and final-component root-symlink rejection. They have NOT been run.
- Android direct-child backend remains statx-mount-ID dependent; Linux nested writes remain fail-closed. Issue #104 remains OPEN / NOT ACCEPTED.

## Issue #105 — Linux process-group containment implementation candidate — 2026-11-20

- Latest code-bearing candidate: `0a9685bded788ae46596227bd61457d10db1179c` (NOT BUILT OR TESTED).
- Linux subprocess startup now installs an inherited seccomp filter in the child before `exec`. It denies `setsid`, `setpgid`, `unshare`, `setns`, clone namespace flags, and x32 syscall ABI entry; `clone3` returns `ENOSYS` so compatible runtimes can fall back to `clone`. The existing dedicated process group is therefore intended to remain non-escapable for descendants; group cleanup still runs on normal return, timeout, and output overflow.
- If the kernel refuses `PR_SET_NO_NEW_PRIVS` or filter installation, process spawn fails closed. Windows Job Object containment remains unchanged.
- Process execution is now explicitly unavailable on Android and macOS until equivalent non-escapable containment exists; `ProcessAdapter` advertises only Linux and Windows.
- Replaced the old escaped-session reader-only probes with three Linux adversarial regressions for attempted `setsid` on normal completion, timeout, and output overflow. These tests have NOT been run.
- This is an implementation candidate, not Issue #105 acceptance. Exact-SHA build/execution and adversarial evidence are still required. Preserve bounded reader shutdown and all existing process limits.
