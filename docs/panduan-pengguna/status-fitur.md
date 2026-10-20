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

One v0.1 memiliki executable language core, grammar, parser, semantic pipeline, reference interpreter, CLI, project scaffolding, conformance fixtures, deterministic artifacts, capability model, interoperability melalui proses, beberapa vertical slice network, dan native backend development untuk hosted Linux/Windows.

### Native backend

**Terverifikasi — Linux AArch64 pada boundary PR #64:** backend `native-linux-aarch64` menghasilkan executable ELF AArch64 dari One Semantic IR dan artifact berhasil dieksekusi pada host Linux AArch64. Evidence exact-SHA: `797c6fe75bd3fa9081329f73dd59e34278f27ddd`.

**Build terbukti — Linux x86_64 pada boundary PR #64:** backend `native-linux-x86_64` menghasilkan executable ELF x86-64 dan regression test cross-linker lulus. Target execution belum diverifikasi karena executor x86_64 tidak tersedia pada environment verifikasi.

**Build terbukti — Windows x86_64 pada boundary PR #64:** backend `native-windows-x86_64` menghasilkan artifact PE32+ x86-64. Eksekusi Windows/Wine belum diverifikasi karena executor tidak tersedia.

Status native backend di atas masih **branch-bound** sampai PR #64 digabung ke `main` dan SHA hasil merge diverifikasi ulang. Jangan menyebut Linux x86_64 atau Windows x86_64 sebagai executable-terverifikasi sebelum target execution evidence tersedia.

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
