# Status Fitur One


## Current exact-SHA CI candidate checkpoint — 2026-11-16

- Last fully verified canonical source SHA: `bf37ccd3af924c95594b8ff3226b720f71dfdbb7`, PR #95 branch `ci/circleci-migration-20261113`, base `main` `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux`: SUCCESS for that exact SHA; status ID `59019926594`; workflow `742c32f4-6257-4b50-8de2-052ba494756a`; job `c516b092-744c-42c7-9959-683715952beb`.
- Independent Windows Server 2025 run [#27](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43858874677): SUCCESS for that same exact SHA; 32/32 steps; native target `x86_64-pc-windows-gnu`; exit code `0`; output `Budi`.
- Windows artifact ID `14222217931`; ZIP SHA-256 `9a95c4b824c2f8ae938ece2d984ded52604bb5d9db1a70fbd4ade87d1bc29a98`; executable SHA-256 `7b5e04217fd38eb91d7deb8b67c9c8e28604f47a75865d3f74cc56c25cdb7dbe`; manifest SHA-256 `b9ccd1f8d9358cacb483718bc9d9d5c095bdf94b7ab5a765141a7a07061543e0`. Artifact, manifest, PE32+ AMD64 headers, and source pin were independently checked.
- Both required CI gates passed on the same exact SHA `bf37ccd...`. This evidence does not automatically transfer to later documentation commits.
- PR #95 remains OPEN / DRAFT / UNMERGED; `main` remains `d35db95153f6ddd79885494c72f5aee16c899751` with the three legacy GitHub Actions workflows. Migration is not live on main until authorized merge.
- Opt-in CircleCI `release-check` remains unexecuted. No merge, tag, or release is authorized.


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
