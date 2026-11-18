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
