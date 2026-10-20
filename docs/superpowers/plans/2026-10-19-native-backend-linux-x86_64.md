# Native Backend Linux x86_64 Implementation Plan

> **For agentic workers:** Implement task-by-task with TDD and verification before completion.

**Goal:** Menambahkan backend native Linux x86_64 yang menghasilkan executable nyata dari One Semantic IR.

**Architecture:** Backend memakai kontrak target/toolchain yang sudah ada dan menghasilkan executable ELF x86_64 melalui toolchain Linux nyata.

**Tech Stack:** Rust, One Semantic IR, C codegen, Clang/LLD atau toolchain Linux x86_64 yang terbukti tersedia.

**Spec:** `docs/superpowers/specs/2026-10-19-native-backends-linux-windows.md`

## Global Constraints

- Semantic IR tetap target-independent.
- Tidak ada fallback interpreter.
- Target: `x86_64-unknown-linux-gnu`.
- Backend ID: `native-linux-x86_64`.
- Artifact: `Executable`.
- Unsupported IR harus gagal deterministik.
- Artifact wajib diperiksa sebelum build dianggap berhasil.
- Eksekusi wajib dibuktikan sebelum target disebut executable.

## Review Focus

- Sysroot/runtime Linux tidak lengkap → build harus gagal jelas.
- Target mismatch → ditolak.
- Artifact bukan ELF x86_64 → ditolak.
- Unsupported IR → error deterministik.
- Tidak ada executor → status bukan EXECUTABLE TERBUKTI.

### Task 0: Validasi Toolchain Linux x86_64

**Files:**
- Tidak mengubah source.

- [x] Probe compiler, sysroot, linker, dan executor Linux x86_64.
- [x] Buktikan compile smoke test kecil ke ELF `x86_64-unknown-linux-gnu`.
- [ ] Executor x86_64 belum tersedia pada environment verifikasi.
- [x] Toolchain dipilih berdasarkan bukti aktual.

### Task 1: Backend Linux x86_64

**Files:**
- Implemented through the shared native backend in `crates/one-core/src/native_backend.rs`.
- Tests: `crates/one-core/tests/native_backend_linux_windows.rs`.

- [x] Tulis test target/toolchain, codegen, determinisme, dan unsupported IR.
- [x] Regression test untuk cross-linker ditulis dan terbukti RED sebelum fix.
- [x] Implementasikan `NativeLinuxX86_64Backend`.
- [x] Integrasikan toolchain nyata yang terbukti.
- [x] Jalankan test sampai PASS.

### Task 2: CLI Target Linux x86_64

**Files:**
- `crates/one-cli/src/native_main.rs`

- [x] Tambahkan pemilihan target `linux-x86_64`.
- [x] Pertahankan target Android.
- [x] Tambahkan error toolchain deterministik.

### Task 3: Artifact dan Eksekusi

- [x] Build smoke test.
- [x] Periksa ELF x86_64.
- [ ] Jalankan pada executor Linux x86_64 yang terbukti — executor belum tersedia.
- [x] Catat status sebagai BUILD TERBUKTI, bukan executable-terverifikasi.

## Status

**BUILD TERBUKTI — exact PR #64 SHA `797c6fe75bd3fa9081329f73dd59e34278f27ddd`; target execution pending.**

Post-merge exact-SHA verification on `main` remains required.
