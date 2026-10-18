# Backend Native Android AArch64 — Spesifikasi Desain

## Tujuan

Membuat jalur nyata `One Source → Semantic IR → Backend → Toolchain Host/Target → Artifact` dengan target pertama Android AArch64, tanpa mengubah Semantic IR menjadi bergantung pada platform.

## Batasan

- Semantic IR tetap target-independent.
- Backend menerima `IrProgram` dan menghasilkan representasi sumber/objek yang dapat diproses toolchain nyata.
- Toolchain dipilih dan divalidasi melalui kontrak target/toolchain yang sudah ada.
- Target pertama adalah Android AArch64 pada lingkungan host `aarch64-linux-android` yang telah terbukti dapat menghasilkan dan menjalankan ELF Android.
- Artifact pertama adalah executable native Android.
- Tidak mengklaim dukungan Linux x86_64, Windows, MCU, firmware, RTOS, atau platform lain sebelum ada backend/toolchain dan verifikasi nyata masing-masing.
- Tidak mengubah semantik bahasa untuk menyesuaikan target.
- Tidak menggunakan interpreter sebagai pengganti backend native pada jalur build native.

## Arsitektur

`Source One → Lexer/Parser → Analysis → Semantic IR → Native Backend → Toolchain Adapter → Native Artifact`

Backend bertanggung jawab memetakan subset Semantic IR yang sudah memiliki semantik deterministik ke bentuk yang dapat dikompilasi. Toolchain adapter bertanggung jawab pada invocation compiler/linker, target triple, artifact output, dan pelaporan kegagalan. Validasi target/toolchain dilakukan sebelum invocation.

## Kontrak backend

Backend harus memiliki identitas stabil, deklarasi target yang didukung, dan operasi kompilasi dari `IrProgram` ke artifact. Backend tidak boleh mengubah target semantics di Semantic IR. Jika sebuah konstruksi IR belum dapat dikodegen secara aman, build harus gagal secara eksplisit dan deterministik, bukan diam-diam kembali ke interpreter.

## Kontrak toolchain

Toolchain adapter menggunakan `ToolchainSpec` yang sudah tersedia. Invocation nyata harus menggunakan konfigurasi target yang sesuai dan menghasilkan artifact pada path yang diminta. Tidak boleh menganggap `ReferenceToolchain` sebagai bukti invocation compiler native.

## Artifact dan determinisme

Build yang sama terhadap input dan konfigurasi target/toolchain yang sama harus menghasilkan artifact yang dapat diverifikasi. Pada tahap awal, pengujian memprioritaskan format ELF yang valid, arsitektur AArch64, executable bit/loader yang benar, serta eksekusi smoke-test pada host Android yang sama. Determinisme byte-level hanya diwajibkan jika backend/toolchain dapat menjaminnya secara realistis.

## Keamanan dan kegagalan

- Tidak ada eksekusi compiler/linker tanpa target/toolchain validation.
- Command invocation tidak boleh dibangun dari input shell mentah tanpa pemisahan argumen.
- Kegagalan parser, analyzer, lowering, codegen, toolchain, dan linker harus tetap dapat dibedakan.
- Build native tidak boleh melaporkan berhasil jika artifact tidak terbentuk.

## Verifikasi minimum

1. Unit test backend: IR sederhana menghasilkan output native yang valid.
2. Unit/integrasi test: target mismatch dan capability mismatch ditolak sebelum invocation.
3. Integrasi: build menghasilkan ELF AArch64 Android.
4. Eksekusi: artifact menjalankan program dan menghasilkan output yang diharapkan.
5. Regresi: jalur interpreter dan artifact `.oneir` yang sudah ada tetap berfungsi.
6. Verifikasi exact-SHA dilakukan setelah perubahan masuk ke branch dan sebelum klaim selesai.

## Perluasan berikutnya

Setelah backend Android AArch64 selesai dan diverifikasi, backend/toolchain Linux x86_64, Linux AArch64, Windows, dan target lain dapat ditambahkan sebagai slice independen. Setiap target memerlukan bukti toolchain dan artifact nyata sendiri.
