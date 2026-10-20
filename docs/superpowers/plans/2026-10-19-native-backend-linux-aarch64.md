# Native Backend Linux AArch64 Implementation Plan

> **For agentic workers:** Implement task-by-task with TDD and verification before completion.

**Goal:** Menambahkan backend native Linux AArch64 yang menghasilkan executable nyata dari One Semantic IR.

**Architecture:** Backend memakai `TargetBackend` dan `ToolchainSpec` yang sudah ada. Codegen menggunakan subset IR native yang sama dengan backend Android, lalu `clang`/toolchain Linux AArch64 menghasilkan ELF executable.

**Tech Stack:** Rust, One Semantic IR, C codegen, Clang/LLD, ELF AArch64.

**Spec:** `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md`

## Global Constraints

- Semantic IR tetap target-independent.
- Tidak ada fallback interpreter.
- Target: `aarch64-unknown-linux-gnu`.
- Backend ID: `native-linux-aarch64`.
- Artifact: `Executable`.
- Unsupported IR harus gagal deterministik.
- Artifact wajib diperiksa sebelum build dianggap berhasil.
- Eksekusi wajib dibuktikan sebelum target disebut executable.

## Review Focus

- Toolchain target mismatch → ditolak.
- Artifact tidak terbentuk → build gagal.
- Identifier/literal tidak aman → codegen gagal.
- Unsupported IR → error deterministik.
- Artifact ELF AArch64 tidak dapat dieksekusi → status hanya BUILD TERBUKTI.

### Task 0: Validasi Toolchain Linux AArch64

**Files:**
- Tidak mengubah source.

- [x] Probe compiler, sysroot, linker, dan executor Linux AArch64.
- [x] Buktikan compile smoke test kecil ke ELF `aarch64-unknown-linux-gnu`.
- [x] Executor AArch64 tersedia pada host verifikasi dan smoke test native berhasil dijalankan.
- [x] Toolchain dipilih berdasarkan bukti aktual.

### Task 1: Backend Linux AArch64

**Files:**
- Implemented through the shared native backend in `crates/one-core/src/native_backend.rs`.
- Tests: `crates/one-core/tests/native_backend_linux_windows.rs`.

- [x] Tulis test target/toolchain dan mismatch.
- [x] Jalankan test dan pastikan gagal sebelum implementasi.
- [x] Implementasikan `NativeLinuxAarch64Backend`.
- [x] Implementasikan codegen subset IR yang disetujui.
- [x] Jalankan unit/integration test sampai PASS.

### Task 2: CLI Target Linux AArch64

**Files:**
- `crates/one-cli/src/native_main.rs`

- [x] Tambahkan pemilihan target `linux-aarch64`.
- [x] Pertahankan Android AArch64.
- [x] Test error toolchain secara deterministik.

### Task 3: Artifact dan Eksekusi

- [x] Build program smoke test nyata.
- [x] Periksa `file`/ELF architecture.
- [x] Eksekusi dengan executor Linux AArch64 yang tersedia.
- [x] Catat output `ONE_NATIVE_LINUX_AARCH64`.

## Status

**VERIFIED — exact PR #64 SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd`, AArch64/Linux host boundary.**

Post-merge exact-SHA verification on `main` remains required before making a canonical main support claim.
