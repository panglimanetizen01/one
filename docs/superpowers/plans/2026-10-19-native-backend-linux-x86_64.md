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

- [ ] Probe compiler, sysroot, linker, dan executor Linux x86_64.
- [ ] Buktikan compile smoke test kecil ke ELF `x86_64-unknown-linux-gnu`.
- [ ] Jika executor tersedia, jalankan smoke test.
- [ ] Jika executor tidak tersedia, catat target sebagai BUILD TERBUKTI saja dan jangan klaim executable.
- [ ] Pilih toolchain berdasarkan bukti aktual, bukan asumsi.

### Task 1: Backend Linux x86_64

**Files:**
- Create/modify: `crates/one-core/src/native_backend_linux_x86_64.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/tests/native_backend_linux_x86_64.rs`

- [ ] Tulis test target/toolchain, codegen, determinisme, dan unsupported IR.
- [ ] Jalankan test dan pastikan gagal sebelum implementasi.
- [ ] Implementasikan `NativeLinuxX86_64Backend`.
- [ ] Integrasikan toolchain nyata yang terbukti.
- [ ] Jalankan test sampai PASS.
- [ ] Commit.

### Task 2: CLI Target Linux x86_64

**Files:**
- Modify: `crates/one-cli/src/native_main.rs`

- [ ] Tambahkan pemilihan target `linux-x86_64`.
- [ ] Pertahankan target Android.
- [ ] Tambahkan error toolchain deterministik.
- [ ] Commit.

### Task 3: Artifact dan Eksekusi

- [ ] Build smoke test.
- [ ] Periksa ELF x86_64.
- [ ] Jalankan pada executor Linux x86_64 yang terbukti.
- [ ] Catat output.
- [ ] Jangan klaim executable tanpa bukti eksekusi.
