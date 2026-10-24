# Alur Pengembangan ONE dari Perubahan hingga Rilis

Panduan ini adalah urutan kerja untuk mengembangkan, menguji, dan menyiapkan rilis ONE. Ikuti tahapannya berurutan; keberhasilan satu tahap tidak otomatis membuktikan tahap berikutnya.

## 1. Buat dan jalankan proyek

```sh
one buat contoh
# edit contoh/src/main.one
one periksa contoh/src/main.one
one jalankan contoh/src/main.one
one test contoh
one bangun contoh/src/main.one
one package contoh
```

- `periksa` memvalidasi source tanpa menjalankannya.
- `jalankan` menjalankan program melalui reference interpreter.
- `test` menjalankan program utama dan fixture conformance yang tersedia di project.
- `bangun` menghasilkan artefak intermediate `.oneir`, bukan executable native.
- `package` menghasilkan artefak `.onepkg` dan, untuk project human-core, module lock deterministik.

Baca [referensi Standard Library](referensi-standard-library.md) untuk built-in yang tersedia dan batas tipenya.

## 2. Verifikasi perubahan source

Dari root repository, jalankan pemeriksaan kontrak dan seluruh rangkaian lokal:

```sh
python tools/check_contract.py
cargo --locked build --workspace --all-targets
cargo --locked clippy --workspace --all-targets -- -D warnings
cargo --locked test --workspace --all-targets
cargo --locked test -p one-core --features real-network --all-targets
bash tools/verify_offline.sh
```

Di Windows PowerShell, jalankan `.\tools\verify_offline.ps1` sebagai pengganti skrip offline Linux. Gunakan toolchain dan dependency lock yang disyaratkan workflow. Jangan menghapus test yang gagal atau menandainya sukses tanpa memahami penyebab dan menambahkan regresi yang sesuai.

## 3. Catat bukti pada SHA yang benar

```sh
git rev-parse HEAD
git status --short
```

- Catat SHA source yang diuji, workflow/run ID, platform, hasil tiap langkah, dan artifact yang dihasilkan.
- Pastikan job memeriksa exact SHA sebelum build/test.
- Jika source berubah setelah pengujian, ulangi gerbang wajib pada SHA baru.
- Hasil lokal, Termux, atau offline bukan GitHub-hosted CI; laporkan sebagai jenis bukti yang berbeda.
- Job hosted dengan `runner_id: 0` dan `steps: []` tidak menghasilkan bukti source-test PASS atau source-test FAIL. Gunakan fallback verifikasi independen yang disetujui dan catat gangguan runner secara terpisah.

## 4. Review platform dan keamanan

- Jalankan matriks Ubuntu dan Windows; jangan menyamakan build-only dengan target execution.
- Untuk target Android AArch64 yang diwajibkan, bukti harus menunjukkan ELF AArch64 berhasil dibuat dan benar-benar dieksekusi pada lingkungan Android yang sesuai.
- Review capability/policy, filesystem root scope, no-follow/reparse-point handling, batas ukuran, batas database/network, dan threat model.
- Periksa compatibility contract, contoh, dokumentasi pengguna, serta output deterministik.

## 5. Siapkan package provenance

Workflow manual `ONE Package Provenance Release` di GitHub Actions membangun proyek yang telah di-commit, menghasilkan final `.onepkg`, menandatangani dengan Ed25519 V2 source-bound provenance, memverifikasi publisher dan source tree, lalu mengunggah package, sidecar, dan checksum.

Sebelum menjalankannya, admin harus mengonfigurasi environment terlindungi `one-release`, aturan approval, secret `ONE_RELEASE_SIGNING_SEED`, dan `ONE_TRUSTED_PUBLISHERS`. Trust store harus diprovisikan melalui proses yang terautentikasi secara independen; jangan membuatnya dari private key selama job rilis. Jangan memasukkan private key ke repository, chat, log, atau artifact.

Lihat [panduan provenance rilis](../RELEASE_PROVENANCE.md).

## 6. Gate final

Jangan menyatakan `PASS GREEN LOCKED` hanya karena build atau satu workflow hijau. Final gate membutuhkan semua baris ledger terverifikasi, semua platform yang diwajibkan lulus pada source SHA final, package provenance diterima, dokumentasi/tooling ditinjau, dan audit forensik Definition of Done tanpa persyaratan wajib yang tersisa. PR yang masih draft atau belum di-merge bukan rilis final.
