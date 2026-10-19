# Spec: Native Backend Linux AArch64, Linux x86_64, Windows x86_64

## Tujuan

Menambahkan backend native nyata untuk:

1. Linux AArch64
2. Linux x86_64
3. Windows x86_64

Backend menghasilkan executable native dari One Semantic IR melalui toolchain nyata.
Tidak ada fallback interpreter pada jalur native.

## Kontrak arsitektur

Alur wajib:

One Source
→ Parser
→ Analyzer
→ One Semantic IR
→ Target Backend
→ Toolchain
→ Native Executable

Semantic IR tetap target-independent.

Fondasi berikut dipakai tanpa redesign:

- `TargetBackend`
- `TargetSpec`
- `ToolchainSpec`
- `ToolchainTarget`
- `ToolchainCapability`
- `ToolchainOperation`
- `ArtifactFormat`
- `LinkerStrategy`

Setiap backend harus melakukan validasi target dan toolchain sebelum build.

## Backend

### Linux AArch64

ID: `native-linux-aarch64`

Target: `aarch64-unknown-linux-gnu`

- architecture: `Aarch64`
- environment: `Linux`
- profile: `Hosted`
- runtime: `Hosted`
- artifact: `Executable`

Status PR #64 exact SHA: **VERIFIED** pada boundary host Linux AArch64 karena artifact ELF AArch64 berhasil dibangun dan dieksekusi.

### Linux x86_64

ID: `native-linux-x86_64`

Target: `x86_64-unknown-linux-gnu`

- architecture: `X86_64`
- environment: `Linux`
- profile: `Hosted`
- runtime: `Hosted`
- artifact: `Executable`

Status PR #64 exact SHA: **BUILD TERBUKTI**. Artifact ELF x86-64 berhasil dibangun. Pada environment verifikasi, Clang memerlukan cross-linker `--ld-path=/usr/bin/x86_64-linux-gnu-ld`. Execution target belum terbukti karena executor x86_64 tidak tersedia.

### Windows x86_64

ID: `native-windows-x86_64`

Target: `x86_64-pc-windows-gnu`

- architecture: `X86_64`
- environment: `Windows`
- profile: `Hosted`
- runtime: `Hosted`
- artifact: `Executable`

Status PR #64 exact SHA: **BUILD TERBUKTI**. Artifact PE32+ x86-64 terbukti. Execution belum terbukti karena Windows/Wine executor tidak tersedia pada environment verifikasi.

## Codegen

Ketiga backend menggunakan subset IR yang sama dengan backend Android pada tahap pertama:

- declaration
- assignment
- literal text
- literal number
- literal boolean
- identifier
- action `tampilkan`

Konstruksi lain harus gagal deterministik dengan error backend native.

Tidak boleh mengubah semantic analysis atau IR hanya untuk target tertentu.

Codegen harus deterministik:
IR sama + backend sama → source native sama.

Identifier dan literal wajib di-escape/validasi secara aman.

## Toolchain

Toolchain nyata wajib digunakan.

Backend tidak boleh mengklaim build berhasil apabila:

- compiler gagal
- linker gagal
- artifact tidak terbentuk
- artifact bukan executable target yang benar

Path executable compiler/linker dapat dikonfigurasi melalui environment variable bila diperlukan.

Tidak boleh menggunakan shell interpolation untuk command build.

## Verifikasi executable

Untuk setiap target wajib ada:

1. unit test backend
2. test validasi target/toolchain
3. test codegen deterministik
4. test unsupported IR
5. build artifact nyata
6. pemeriksaan format/arsitektur artifact
7. eksekusi smoke test pada executor target yang nyata

Jika target dapat cross-build tetapi belum dapat dieksekusi pada environment verifikasi, statusnya hanya:
`BUILD TERBUKTI`, bukan `EXECUTABLE TERBUKTI`.

PR #64 membuktikan seluruh gate build/format yang tersedia. Gate execution masih terbuka untuk Linux x86_64 dan Windows x86_64.

## CLI

Perlu tersedia pemilihan target yang eksplisit, misalnya:

`one-native build --target linux-aarch64 ...`
`one-native build --target linux-x86_64 ...`
`one-native build --target windows-x86_64 ...`

Perilaku lama Android AArch64 harus tetap berjalan.

Jika target tidak tersedia toolchain-nya, CLI harus gagal dengan pesan/error yang jelas dan deterministik.

## Batasan

Tidak termasuk dalam spec ini:

- MCU
- firmware
- RTOS
- bare metal
- interpreter baru
- perubahan semantic IR
- perubahan parser/analyzer
- dukungan Windows MSVC apabila toolchain yang dipilih adalah GNU
- klaim dukungan target lain

## Kriteria selesai

Satu target hanya dianggap selesai apabila seluruh bukti berikut tersedia:

`source → IR → backend → toolchain → executable → format benar → eksekusi berhasil`

Ketiga target harus diverifikasi secara independen.

Untuk PR #64, Linux AArch64 memenuhi kriteria tersebut pada boundary host AArch64. Linux x86_64 dan Windows x86_64 belum memenuhi gate execution.

Setelah perubahan masuk `main`, verifikasi ulang harus dilakukan pada SHA merge/main yang tepat.
