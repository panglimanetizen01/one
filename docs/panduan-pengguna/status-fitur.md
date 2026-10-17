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

**Dalam pengembangan — database query source-to-runtime vertical slice:** Implementasi SQLite sudah ada pada PR #57 head `0aa4acdf3796235eac432138fec829b6508879e3`, termasuk `kueri database`, capability `database.query`, resource-scoped governance, adapter `database.sqlite.query`, parameterized SELECT, normalization, dan test CLI/core. Namun exact-SHA executable verification belum tersedia. Jangan menyebutnya Didukung atau Eksperimental sampai evidence executable tersedia.

Source:

```text
kueri database ":memory:" dengan "SELECT ?1" dan "42"
```

Permission CLI:

```text
one jalankan main.one --izinkan database.query --target :memory:
```

Boundary implementasi saat ini: read-only parameterized SQLite SELECT; database resource harus sama persis dengan resource yang diberi izin. Status tetap **Dalam pengembangan** sampai build/test/clippy/runtime pada exact SHA terbukti.

### Kemampuan yang masih future

HTTP orchestration tambahan, native C/C++, Python, Node, JVM bindings, compiler backend native penuh, package registry, IDE/editor, GUI framework integration, dan locale tambahan tidak boleh didokumentasikan sebagai kemampuan yang sudah tersedia hanya karena arah arsitekturnya sudah ditentukan.

Filesystem write tetap **Eksperimental**, bukan **Didukung**, karena bukti lintas-platform untuk executable revision `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` belum ada.

Untuk daftar teknis yang lebih lengkap, lihat `docs/verification/one-v0.1-implementation-status.md` dan `docs/verification/one-v0.1-evidence-matrix.md`.
