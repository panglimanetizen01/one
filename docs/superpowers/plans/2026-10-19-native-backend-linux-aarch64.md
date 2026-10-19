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

- [ ] Probe compiler, sysroot, linker, dan executor Linux AArch64.
- [ ] Buktikan compile smoke test kecil ke ELF `aarch64-unknown-linux-gnu`.
- [ ] Jika executor tersedia, jalankan smoke test.
- [ ] Jika executor tidak tersedia, catat target sebagai BUILD TERBUKTI saja dan jangan klaim executable.
- [ ] Pilih toolchain berdasarkan bukti aktual, bukan asumsi.

### Task 1: Backend Linux AArch64

**Files:**
- Create/modify: `crates/one-core/src/native_backend_linux_aarch64.rs`
- Modify: `crates/one-core/src/lib.rs`
- Test: `crates/one-core/tests/native_backend_linux_aarch64.rs`

- [ ] Tulis test target/toolchain dan mismatch.
- [ ] Jalankan test dan pastikan gagal sebelum implementasi.
- [ ] Implementasikan `NativeLinuxAarch64Backend`.
- [ ] Implementasikan codegen subset IR yang disetujui.
- [ ] Jalankan unit/integration test sampai PASS.
- [ ] Commit.

### Task 2: CLI Target Linux AArch64

**Files:**
- Modify: `crates/one-cli/src/native_main.rs`

- [ ] Tambahkan pemilihan target `linux-aarch64`.
- [ ] Pertahankan Android AArch64.
- [ ] Test error toolchain secara deterministik.
- [ ] Commit.

### Task 3: Artifact dan Eksekusi

- [ ] Build program smoke test nyata.
- [ ] Periksa `file`/ELF architecture.
- [ ] Eksekusi dengan executor Linux AArch64 yang tersedia.
- [ ] Catat output sebagai bukti.
- [ ] Jika hanya cross-build dan executor belum tersedia, jangan klaim executable.
