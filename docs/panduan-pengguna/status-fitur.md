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

## Status verifikasi otomatis dan dampaknya bagi pengguna

Status job CI harus dibedakan dari status kemampuan bahasa One. Job yang tidak pernah memulai runner, atau berhenti saat menyiapkan toolchain, bukan bukti bahwa compiler, runtime, grammar, CLI, atau program pengguna gagal. Klaim bahwa suatu kemampuan telah terverifikasi tetap harus menyebut revisi sumber (exact SHA), platform, dan bukti eksekusi yang benar-benar dijalankan.

Untuk insiden yang tercatat pada 2026-11-16, job Windows historis #254 gagal saat inisialisasi keyring MSYS2 pada alur yang sudah tidak menjadi konfigurasi CI aktif. Job #426 dan #407–#409 tidak menunjukkan eksekusi tes dan penyebab pastinya belum diketahui; job #424 memiliki pesan kehabisan kredit yang terpisah. Lihat [catatan forensik CircleCI](../verification/circleci-infrastructure-incident-2026-11-16.md). Temuan infrastruktur ini tidak mengubah status fitur One.
