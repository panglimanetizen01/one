# Status Fitur One

Dokumentasi pengguna One harus membedakan kemampuan yang benar-benar tersedia dari rancangan masa depan.

| Status | Arti |
|---|---|
| **Didukung** | Kemampuan tersedia dan bukti yang diperlukan sudah ada untuk klaim yang dibuat. |
| **Eksperimental** | Kemampuan tersedia, tetapi kontrak/perilakunya masih dapat berubah. |
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

**Dalam pengembangan:** One memiliki fondasi runtime orchestration yang sudah diverifikasi: typed capability request, scope/governance, typed adapter boundary, normalized result/evidence, audit, semantic-resolution fixtures, dan `network.dns.resolve`.

Namun, **belum tersedia sebagai fitur bahasa tingkat sumber**: belum ada bukti bahwa program `.one` biasa dapat menyelesaikan semantic operation → capability resolution → governance → adapter → normalized One result melalui CLI/runtime. Jangan mendokumentasikan orchestration sebagai kemampuan bahasa yang sudah didukung sampai jalur end-to-end tersebut teruji.

### Kemampuan yang masih future

Native C/C++, Python, Node.js, JVM bindings, compiler backend native penuh, package registry, IDE/editor, GUI framework integration, dan locale tambahan tidak boleh didokumentasikan sebagai kemampuan yang sudah tersedia hanya karena arah arsitekturnya sudah ditentukan.

Untuk daftar teknis yang lebih lengkap, lihat `docs/verification/one-v0.1-implementation-status.md` dan `docs/verification/one-v0.1-evidence-matrix.md`.
