# Rencana Implementasi Backend Native Android AArch64

## Tujuan

Merealisasikan jalur pertama `One Source → Semantic IR → Backend → Toolchain Host/Target → Artifact` tanpa mengubah Semantic IR menjadi bergantung pada platform.

## Urutan kerja

1. Tetapkan kontrak pengujian untuk backend, codegen deterministik, penolakan IR yang belum didukung, dan metadata toolchain.
2. Implementasikan backend native Android AArch64 berbasis source C.
3. Validasi target dan toolchain sebelum compiler dipanggil.
4. Gunakan invocation compiler melalui argumen terpisah, bukan shell mentah.
5. Hasilkan executable native Android AArch64 dan gagal eksplisit jika codegen/toolchain/linker/artifact gagal.
6. Tambahkan binary `one-native build` sebagai jalur native tanpa mengganggu artefak `.oneir` yang sudah ada.
7. Verifikasi exact-SHA di lingkungan Termux/Android yang telah membuktikan Clang AArch64 Android dan Rust native AArch64 dapat menghasilkan serta menjalankan ELF.
8. Setelah slice Android AArch64 terbukti, target Linux x86_64, Linux AArch64, Windows, dan target lain dikerjakan sebagai slice independen dengan bukti toolchain masing-masing.

## Batas implementasi pertama

Backend pertama hanya mendukung deklarasi, assignment, literal teks/angka/boolean, identifier yang dapat dipetakan aman ke identifier C, dan aksi `tampilkan`. Ekspresi binary, pemanggilan fungsi, `nothing`, loop, kondisi, event, orchestration, network, serta aksi lain gagal eksplisit dengan diagnostic backend; tidak ada fallback ke interpreter.

## Kriteria selesai slice

- Pengujian backend lulus.
- Codegen deterministik terbukti.
- Target/toolchain mismatch ditolak sebelum invocation.
- `one-native build` menghasilkan ELF AArch64 Android pada lingkungan yang sesuai.
- Artifact dapat dieksekusi dan menghasilkan output yang sama dengan semantik reference untuk fixture yang didukung.
- Jalur interpreter dan `.oneir` tidak rusak.
- Exact-SHA verification selesai sebelum klaim selesai.
