# Status Fitur One

Dokumentasi pengguna One harus membedakan kemampuan yang benar-benar tersedia dari rancangan masa depan.

| Status | Arti |
|---|---|
| **Didukung** | Kemampuan tersedia dan bukti yang diperlukan sudah ada untuk klaim yang dibuat. |
| **Eksperimental** | Kemampuan tersedia secara nyata pada boundary yang dinyatakan, tetapi cakupan atau kontraknya masih terbatas dan dapat berkembang. |
| **Dalam pengembangan** | Implementasi sedang dikerjakan; jangan menganggapnya selesai. |
| **Dirancang** | Desain atau kontrak sudah ada, tetapi kemampuan belum tersedia untuk penggunaan biasa. |
| **Belum tersedia** | Belum dapat digunakan pada versi yang sedang didokumentasikan. |

## Aturan

1. Status tidak boleh ditentukan hanya dari roadmap atau nama fitur.
2. Registry atau daftar protokol tidak otomatis berarti runtime feature tersebut tersedia.
3. Klaim dukungan harus sesuai dengan implementation status dan verification evidence.
4. Jika bukti berubah, status dan halaman pengguna yang terdampak harus diperbarui.
5. Jika fitur dihapus, instruksi pengguna yang sudah tidak berlaku harus dihapus atau ditandai secara eksplisit; jangan dibiarkan sebagai petunjuk aktif.

## Kondisi One saat ini

One v0.1 memiliki executable language core, grammar, parser, semantic pipeline, reference interpreter, CLI, project scaffolding, conformance fixtures, deterministic artifacts, capability model, interoperability melalui proses, dan beberapa vertical slice network yang telah dibuktikan.

### Orchestration

**Eksperimental — DNS source-to-runtime vertical slice:** One sekarang memiliki jalur bahasa yang dapat dieksekusi untuk operasi `cari alamat` dengan izin CLI eksplisit:

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

Status **Eksperimental** berarti vertical slice DNS ini nyata dan terverifikasi, tetapi tidak berarti seluruh katalog capability/orchestration sudah tersedia atau didukung.

### Kemampuan yang masih future

Filesystem/process/database/HTTP orchestration tambahan, native C/C++, Python, Node, JVM bindings, compiler backend native penuh, package registry, IDE/editor, GUI framework integration, dan locale tambahan tidak boleh didokumentasikan sebagai kemampuan yang sudah tersedia hanya karena arah arsitekturnya sudah ditentukan.

Untuk daftar teknis yang lebih lengkap, lihat `docs/verification/one-v0.1-implementation-status.md` dan `docs/verification/one-v0.1-evidence-matrix.md`.
