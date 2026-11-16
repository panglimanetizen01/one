# Status Fitur One


## Current exact-SHA CI candidate checkpoint — 2026-11-17

- Code/config source SHA yang telah diuji: `07d8cbf7422b8340f8efda3c2f9d4425f85b7537`, PR #95 branch `ci/circleci-migration-20261113`, base `main` `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI Linux `one-linux`: SUCCESS pada SHA tersebut; workflow `0e03e28b-dfb9-4198-b897-3ce5d16dc2d8`, job `b2f536a1-6c87-49b5-9a3f-2db2f2908595`.
- Independent Windows Server 2025 run [#28](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43865524749): SUCCESS pada source SHA yang sama; job `133292953068`, 32/32 main steps, target `x86_64-pc-windows-gnu`, exit code `0`, output `Budi`.
- Windows artifact ID `14225302303`; ZIP SHA-256 `7dd4435e41bc8f0660f454768711c0fc460e2ca0fe38f2dee1034c59c0f767a5`; executable SHA-256 `9004e87f0c9d0357ee0197a1bce2a26474cb714aa8d71d54990236c9266dcfeb`; manifest SHA-256 `d95ae44ee24c5b793aeef59fb006e48d4c5b3e62de4cd5012b2b5a769cbf92a4`.
- Kedua bukti PASS terikat pada source code/config SHA `07d8cbf...`. HEAD PR #95 sekarang `d06d2529d4c96eecd563a2c3cb4ff98b910cb061`, hanya tujuh perubahan dokumentasi setelah SHA tersebut; hasil CI tidak diklaim sebagai run baru pada HEAD dokumentasi.
- PR #95 tetap OPEN / DRAFT / UNMERGED. `main` tetap `d35db95153f6ddd79885494c72f5aee16c899751` dan masih memuat tiga workflow GitHub Actions lama; migrasi belum aktif pada main.
- CircleCI `release-check` opsional belum dijalankan. CI PASS bukan persetujuan release atau bukti bahwa seluruh proyek ONE telah selesai 100%. Tidak ada merge, tag, atau release yang diizinkan.

## Kondisi One saat ini

One v0.1 memiliki executable language core, grammar, parser, semantic pipeline, reference interpreter, CLI, project scaffolding, conformance fixtures, deterministic artifacts, capability model, interoperability melalui proses, beberapa vertical slice network, dan native backend implementation yang sudah masuk ke canonical `main` melalui PR #64. Last code-bearing canonical source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` passed independent exact-SHA Ubuntu/Windows matrix run `39554661451`. Klaim target-native execution tetap dibatasi pada artifact dan executor yang benar-benar tercatat.

### Native backend

**Bukti implementasi — canonical `main`:** PR #64 telah digabung ke `main` pada merge commit `ec844aa652a4d35c3891fc810d344c9c9db2f5cf`. Implementasi native backend menggunakan One Semantic IR yang sama dan menyediakan target hosted Linux AArch64, Linux x86_64, dan Windows x86_64.

**Evidence executable yang tersedia sebelum merge:** implementation SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd` membuktikan Linux AArch64 ELF dibangun dan dieksekusi pada host AArch64/Linux; Linux x86_64 menghasilkan ELF x86-64; Windows x86_64 menghasilkan PE32+. Linux x86_64 dan Windows x86_64 belum memiliki target-execution evidence.

**Boundary saat ini:** source SHA `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` telah lulus exact-SHA Ubuntu/Windows host matrix run `39554661451` untuk contract artifacts, build, Clippy, workspace tests, real-network tests, dan offline verification. Matrix tersebut tidak otomatis membuktikan eksekusi setiap cross-compiled target. Linux AArch64 execution tetap terikat pada bukti host AArch64 yang tercatat; Linux x86_64 dan Windows x86_64 tidak boleh disebut target-execution verified tanpa executor evidence yang sesuai.

Historical canonical GitHub Actions runs `39372356906` and `39381125667` failed with `steps: []`; these are infrastructure evidence, not source failures. CircleCI has since been selected as the canonical CI provider; its migration and current unverified execution state are tracked in `docs/CI.md`.

### Orchestration

**Eksperimental — DNS source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk operasi `cari alamat` dengan izin CLI eksplisit.

**Eksperimental — filesystem read source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk operasi `baca berkas` dengan izin CLI eksplisit. Boundary ini telah diverifikasi lintas Ubuntu + Windows Server 2025 pada source `f482c6b75505d9965dd33f4f25bb0cd04072cedb` melalui mirror run `38903950612`.

**Eksperimental — process execution source-to-runtime vertical slice:** `jalankan proses` telah diverifikasi lintas Ubuntu + Windows Server 2025 pada source `9038ac75f197753ff1ab6af7478e63e4831903a2` melalui mirror run `38929072800`. Slice ini dibatasi pada satu program yang ditargetkan secara eksplisit tanpa argumen tambahan.

**Eksperimental — filesystem write source-to-runtime vertical slice:** One memiliki jalur bahasa yang dapat dieksekusi untuk `tulis ke berkas` dengan permission `filesystem.write` dan target file yang sama. Evidence executable tetap dibatasi pada Termux/Android untuk exact SHA yang tercatat.

**Eksperimental — database query source-to-runtime vertical slice:** Implementasi SQLite executable-verified pada boundary Termux/Android yang tercatat. Source/core/CLI regression tests juga lulus dalam exact-SHA Ubuntu/Windows matrix `39554661451` pada source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`. Cakupannya tetap read-only parameterized `SELECT`, bukan general database abstraction.

**Eksperimental — HTTP GET source-to-runtime vertical slice:** Implementasi `ambil dari` executable-verified pada boundary Termux/Android yang tercatat. Source/core/CLI regression tests juga lulus dalam exact-SHA Ubuntu/Windows matrix `39554661451` pada source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`. Boundary-nya tetap explicit `http://` URL, HTTP/1.1 GET, exact target scope, response-body normalization, dan explicit `network.connect` permission.

### Kemampuan yang masih future

HTTP methods/HTTPS/API features yang lebih luas, native C/C++, Python, Node, JVM bindings, compiler backend native penuh, package registry, IDE/editor, GUI framework integration, dan locale tambahan tidak boleh didokumentasikan sebagai kemampuan yang sudah tersedia hanya karena arah arsitekturnya sudah ditentukan.

Untuk daftar teknis yang lebih lengkap, lihat `docs/verification/one-v0.1-implementation-status.md` dan `docs/verification/one-v0.1-evidence-matrix.md`.
