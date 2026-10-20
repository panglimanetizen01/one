# Native Backend Windows x86_64 Implementation Plan

> **For agentic workers:** Implement task-by-task with TDD and verification before completion.

**Goal:** Menambahkan backend native Windows x86_64 yang menghasilkan executable PE/COFF nyata.

**Architecture:** Backend memakai `TargetBackend` yang ada dan toolchain GNU Windows yang terbukti, dengan target `x86_64-pc-windows-gnu`.

**Tech Stack:** Rust, One Semantic IR, C codegen, MinGW/Clang Windows toolchain.

**Spec:** `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md`

## Global Constraints

- Semantic IR tetap target-independent.
- Tidak ada fallback interpreter.
- Target: `x86_64-pc-windows-gnu`.
- Backend ID: `native-windows-x86_64`.
- Artifact: `Executable`.
- Windows MSVC tidak termasuk.
- Artifact wajib diperiksa sebelum build dianggap berhasil.
- Eksekusi wajib dibuktikan sebelum target disebut executable.

## Review Focus

- MinGW/runtime DLL tidak tersedia → build/execution harus dibedakan.
- Target mismatch → ditolak.
- Artifact bukan PE x86_64 → ditolak.
- Unsupported IR → error deterministik.
- Tidak ada Wine/Windows executor → hanya BUILD TERBUKTI.

### Task 0: Validasi Toolchain Windows x86_64

**Files:**
- Tidak mengubah source.

- [x] Probe MinGW/Clang Windows, linker, runtime, dan executor availability.
- [x] Buktikan compile smoke test menjadi PE/COFF x86_64.
- [ ] Windows/Wine executor belum tersedia pada environment verifikasi.
- [x] Toolchain dipilih berdasarkan bukti aktual.

### Task 1: Backend Windows x86_64

**Files:**
- Implemented through the shared native backend in `crates/one-core/src/native_backend.rs`.
- Tests: `crates/one-core/tests/native_backend_linux_windows.rs`.

- [x] Tulis test target/toolchain, codegen, determinisme, dan unsupported IR.
- [x] Jalankan test dan pastikan gagal sebelum implementasi.
- [x] Implementasikan `NativeWindowsX86_64Backend`.
- [x] Integrasikan toolchain Windows GNU yang terbukti.
- [x] Jalankan test sampai PASS.

### Task 2: CLI Target Windows x86_64

**Files:**
- `crates/one-cli/src/native_main.rs`

- [x] Tambahkan pemilihan target `windows-x86_64`.
- [x] Pertahankan target lain.

### Task 3: Artifact dan Eksekusi

- [x] Build smoke test.
- [x] Periksa PE/COFF x86_64.
- [ ] Eksekusi dengan Windows/Wine executor yang terbukti — executor belum tersedia.
- [x] Catat status sebagai BUILD TERBUKTI, bukan executable-terverifikasi.

## Status

**BUILD TERBUKTI — exact PR #64 SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd`; target execution pending.**

Post-merge exact-SHA verification on `main` remains required.
