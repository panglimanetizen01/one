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

**Eksperimental — DNS source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk operasi `cari alamat` dengan izin CLI eksplisit:

```text
One source
→ parser
→ semantic validation
→ typed orchestration IR
→ governed Orchestrator
→ DNS adapter
→ normalized result
→ CLI output
```

Permission CLI saat ini:

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
```

Tanpa permission tersebut, policy menolak orchestration. Target juga dibatasi secara eksplisit.

Status **Eksperimental** berarti vertical slice DNS ini nyata dan terverifikasi pada exact-SHA cross-platform evidence untuk canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` melalui mirror run `38903950612`. Ini tidak berarti seluruh katalog capability/orchestration sudah tersedia atau didukung.

**Eksperimental — filesystem read source-to-runtime vertical slice:** One juga memiliki jalur bahasa yang dapat dieksekusi untuk operasi `baca berkas` dengan izin CLI eksplisit:

```text
One source
→ parser
→ semantic validation
→ typed orchestration IR
→ governed Orchestrator
→ filesystem.read adapter
→ normalized UTF-8 text result
→ CLI output
```

Permission CLI saat ini menggunakan capability dan target eksplisit:

```text
one jalankan main.one --izinkan filesystem.read --target <path>
```

Tanpa permission tersebut, policy menolak orchestration. Target yang berbeda dari target yang diberi izin berada di luar scope.

Filesystem read slice tersebut telah diverifikasi pada canonical source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` melalui independent mirror run `38903950612` pada Ubuntu dan Windows Server 2025. Status ini hanya mencakup filesystem read pada boundary tersebut; bukan klaim bahwa filesystem write, database, atau seluruh orchestration catalog sudah tersedia.

**Dalam pengembangan — process execution source-to-runtime vertical slice:** Implementasi `jalankan proses` sudah melewati parser → semantic validation → typed orchestration IR → governed `process.execute` adapter → normalized process output dan memiliki TDD tests pada development branch. Exact-SHA execution evidence untuk implementasi terbaru masih pending, sehingga fitur ini belum boleh disebut eksperimental atau verified.

Permission CLI yang ditargetkan:

```text
one jalankan main.one --izinkan process.execute --target <program>
```

Slice saat ini hanya mengeksekusi satu program yang ditargetkan secara eksplisit tanpa argumen tambahan. Ini bukan klaim dukungan umum untuk process management, service management, shell scripting, atau arbitrary command orchestration.

### Kemampuan yang masih future

Filesystem write, database orchestration, HTTP orchestration tambahan, native C/C++, Python, Node, JVM bindings, compiler backend native penuh, package registry, IDE/editor, GUI framework integration, dan locale tambahan tidak boleh didokumentasikan sebagai kemampuan yang sudah tersedia hanya karena arah arsitekturnya sudah ditentukan.

Untuk daftar teknis yang lebih lengkap, lihat `docs/verification/one-v0.1-implementation-status.md` dan `docs/verification/one-v0.1-evidence-matrix.md`.
