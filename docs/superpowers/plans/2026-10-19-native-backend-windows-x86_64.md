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

- [ ] Probe MinGW/Clang Windows, linker, runtime, dan executor Windows/Wine.
- [ ] Buktikan compile smoke test kecil menjadi PE/COFF x86_64.
- [ ] Jika executor tersedia, jalankan smoke test.
- [ ] Jika executor tidak tersedia, catat target sebagai BUILD TERBUKTI saja dan jangan klaim executable.
- [ ] Pilih toolchain berdasarkan bukti aktual, bukan asumsi.

### Task 1: Backend Windows x86_64

**Files:**
- Create/modify: `crates/one-core/src/native_backend_windows_x86_64.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/tests/native_backend_windows_x86_64.rs`

- [ ] Tulis test target/toolchain, codegen, determinisme, dan unsupported IR.
- [ ] Jalankan test dan pastikan gagal sebelum implementasi.
- [ ] Implementasikan `NativeWindowsX86_64Backend`.
- [ ] Integrasikan toolchain Windows GNU yang terbukti.
- [ ] Jalankan test sampai PASS.
- [ ] Commit.

### Task 2: CLI Target Windows x86_64

**Files:**
- Modify: `crates/one-cli/src/native_main.rs`

- [ ] Tambahkan pemilihan target `windows-x86_64`.
- [ ] Pertahankan target lain.
- [ ] Commit.

### Task 3: Artifact dan Eksekusi

- [ ] Build smoke test.
- [ ] Periksa PE/COFF x86_64.
- [ ] Eksekusi dengan Windows/Wine executor yang terbukti.
- [ ] Catat output.
- [ ] Jangan klaim executable tanpa bukti eksekusi.
