# Status Fitur One


## Current exact-SHA CI candidate checkpoint — 2026-11-16

- Canonical candidate: PR #95, branch `ci/circleci-migration-20261113`, exact source SHA `e92d3a3003b68283d5f0fe1315dcb41dcc33bd62`; base `main` SHA `d35db95153f6ddd79885494c72f5aee16c899751`.
- CircleCI `ci/circleci: one-linux` is terminal SUCCESS on that exact source SHA (workflow `87ef4587-00f4-46a6-9837-01f9d760f9e9`, job `72e6485f-ae89-4302-b4b8-e084b5de60bd`). This is the Linux product job, not the opt-in release job.
- Independent Windows Server 2025 run [#22](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43852530420) is terminal SUCCESS for the same exact source SHA. Subsequent verifier-documentation validation runs [#23](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43853509916) and [#24](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/43853556846) also completed SUCCESS; run #24 is the latest verifier-branch run.
- Latest Windows run #24: job `133254250473`; 32/32 steps succeeded; verifier merge-ref SHA `2b9d70894d7b166695a571311c67460d505b9c0f`; target `x86_64-pc-windows-gnu`; Windows Server 2025 AMD64; native exit code `0`; output `Budi`.
- Run #24 artifact ID `14220981534`; ZIP SHA-256 `e65a6925c67b325667cc6e029d00e1542601facdfb2474cf1766546619c24567`, independently recomputed and matched GitHub's artifact digest. Embedded executable SHA-256 `cf07f9179783eb90b8637192a6418cba25d42461504da9e5a76c159cd02fffeb`; MZ, PE signature, AMD64 `0x8664`, and PE32+ `0x20B` were independently inspected. Manifest source SHA equals `e92d3a3003b68283d5f0fe1315dcb41dcc33bd62` and records verifier SHA, run identity, host, binary digest, exit code, and output.
- Structural audit of the candidate tree: no `.github/workflows/` paths and no `ci-local` paths. The canonical CircleCI config contains only `one-linux` by default and an opt-in `release-linux` job; no CircleCI Windows executor is present.
- IMPORTANT: PR #95 is still OPEN / DRAFT / UNMERGED. The migration is not active on canonical `main` until an authorized merge. Current `main` remains `d35db95153f6ddd79885494c72f5aee16c899751` and still contains the three legacy GitHub Actions workflows. Do not claim the migration is live on main.
- The opt-in CircleCI `release-check` workflow (pipeline parameter `run_release_checks=true`) has not yet been executed for this candidate; release approval is separate. This candidate checkpoint is CI PASS, not final project/release completion.
- The only changes since the earlier Windows-tested source `f85375e279257dcbb7d4ded9632800fcd1b78cc9` are three documentation-only commits across eight Markdown files; no product source, tests, or CircleCI config changed in that interval. Exact-SHA evidence is nevertheless bound to `e92d3a3...`.

Dokumentasi pengguna One harus membedakan kemampuan yang benar-benar tersedia dari rancangan masa depan.

| Status | Arti |
|---|---|
| **Didukung** | Kemampuan tersedia dan bukti yang diperlukan sudah ada untuk klaim yang dibuat. |
| **Eksperimental** | Kemampuan tersedia secara nyata pada boundary yang dinyatakan, tetapi cakupan atau kontraknya masih terbatas dan dapat berkembang. |
| **Dalam pengembangan** | Implementasi sedang dikerjakan; jangan menganggapnya selesai. |
| **Dirancang** | Desain atau kontrak sudah ada, tetapi kemampuan belum tersedia untuk penggunaan biasa. |
| **Belum tersedia** | Belum dapat digunakan pada versi yang sedang didokumentasikan. |

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
