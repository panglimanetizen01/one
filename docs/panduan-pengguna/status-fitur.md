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

One v0.1 memiliki executable language core, grammar, parser, semantic pipeline, reference interpreter, CLI, project scaffolding, conformance fixtures, deterministic artifacts, capability model, interoperability melalui proses, dan beberapa vertical slice network yang telah dibuktikan.

### Orchestration

**Eksperimental — DNS source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk operasi `cari alamat` dengan izin CLI eksplisit.

**Eksperimental — filesystem read source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk operasi `baca berkas` dengan izin CLI eksplisit. Boundary ini telah diverifikasi lintas Ubuntu + Windows Server 2025 pada canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` melalui mirror run `38903950612`.

**Eksperimental — process execution source-to-runtime vertical slice:** `jalankan proses` telah diverifikasi lintas Ubuntu + Windows Server 2025 pada source `9038ac75f197753ff1ab6af7478e63e4831903a2` melalui mirror run `38929072800`. Slice ini dibatasi pada satu program yang ditargetkan secara eksplisit tanpa argumen tambahan.

**Eksperimental — filesystem write source-to-runtime vertical slice:** One sekarang memiliki jalur bahasa yang dapat dieksekusi untuk `tulis ke berkas` dengan permission `filesystem.write` dan target file yang sama. Latest exact-SHA verification untuk executable boundary `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` lulus di Termux/Android melalui verifikasi berikut: contract check, build, Clippy `-D warnings`, targeted CLI filesystem-write test, seluruh workspace tests, dan offline verification. Bukti ini belum merupakan verifikasi Ubuntu/Windows untuk SHA tersebut.

**Dalam pengembangan — database query source-to-runtime vertical slice:** Implementasi SQLite sudah ada pada PR #57 lineage `0aa4acdf3796235eac432138fec829b6508879e3`, termasuk `kueri database`, capability `database.query`, resource-scoped governance, adapter `database.sqlite.query`, parameterized SELECT, normalization, dan test CLI/core. Exact-SHA executable verification belum tersedia. Jangan menyebutnya Didukung atau Eksperimental sampai evidence executable tersedia.

**Dalam pengembangan — HTTP GET source-to-runtime vertical slice:** Implementasi `ambil dari` sudah ditambahkan pada PR #57 lineage terbaru, mencakup parser → semantic validation → typed orchestration IR → target-scoped `network.connect` governance → `network.http.get` adapter → bounded HTTP/1.1 GET → normalized response body → CLI permission. Exact-SHA executable verification belum tersedia. Jangan menyebutnya Didukung atau Eksperimental sampai evidence executable tersedia.

Source:

```text
ambil dari "http://127.0.0.1:8080/"
```

Permission CLI:

```text
one jalankan main.one --izinkan network.connect --target http://127.0.0.1:8080/
```

Boundary implementasi saat ini: explicit `http://` URL, HTTP/1.1 GET, exact target scope, response body normalization, dan explicit `network.connect` permission. HTTPS, arbitrary HTTP methods, custom headers/bodies, redirects, cookies, dan API-specific authentication belum termasuk source-level slice ini.

### Kemampuan yang masih future

HTTP methods/HTTPS/API features yang lebih luas, native C/C++, Python, Node, JVM bindings, compiler backend native penuh, package registry, IDE/editor, GUI framework integration, dan locale tambahan tidak boleh didokumentasikan sebagai kemampuan yang sudah tersedia hanya karena arah arsitekturnya sudah ditentukan.

Filesystem write tetap **Eksperimental**, bukan **Didukung**, karena bukti lintas-platform untuk executable revision `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` belum ada.

Untuk daftar teknis yang lebih lengkap, lihat `docs/verification/one-v0.1-implementation-status.md` dan `docs/verification/one-v0.1-evidence-matrix.md`.
