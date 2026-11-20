# Panduan pemula: aplikasi daftar tugas human-vNext

> **Eksperimental:** panduan ini untuk prototipe human-vNext saja. Perintah ONE v0.1 yang biasa tetap memakai parser v0.1; sintaks human-vNext tidak dipilih otomatis.

## 1. Lihat program contoh

Buka `examples/human-vnext/daftar-tugas.one`. Program mendefinisikan aksi menambahkan tugas, melihat semua tugas, menandai tugas selesai, serta kebutuhan penyimpanan.

Runner saat ini hanya menjalankan bentuk aksi dan nama yang didukung oleh contoh tersebut. Bahasa ini belum menjadi grammar normatif ONE secara umum.

## 2. Jalankan aplikasi

Dari root repository:

```sh
cargo run -p one-cli -- jalankan-human-vnext examples/human-vnext/daftar-tugas.one --data ./data/tugas.db
```

Opsi `--data` memilih lokasi file penyimpanan. Direktori induknya dibuat otomatis. Gunakan lokasi eksperimen, bukan file data produksi.

## 3. Gunakan menu

- Ketik `tambah`, lalu masukkan judul tugas satu baris.
- Ketik `lihat` untuk melihat ID dan status tugas.
- Ketik `selesai`, lalu masukkan ID tugas yang ingin diselesaikan.
- Ketik `keluar` untuk menutup aplikasi.

Judul kosong dan karakter kontrol ditolak. ID harus berupa angka dan harus merujuk ke tugas yang ada. Jika file penyimpanan rusak atau tidak bisa ditulis, aplikasi menampilkan error dan tidak mengklaim perubahan berhasil.

## 4. Pastikan penyimpanan bertahan

Tutup aplikasi dengan `keluar`, lalu jalankan perintah yang sama lagi dengan nilai `--data` yang sama. Perintah `lihat` harus menampilkan tugas yang sebelumnya disimpan. Mengganti jalur `--data` berarti menggunakan penyimpanan yang berbeda.

## 5. Verifikasi

Dari worktree bersih yang diambil dari GitHub, jalankan:

```sh
bash tools/verify-human-vnext.sh
```

Skrip memeriksa SHA lokal terhadap HEAD branch GitHub sebelum dan sesudah pemeriksaan kontrak, build, Clippy, dan test. Hasil lokal tidak boleh disebut sebagai GitHub Actions atau Windows pass; test yang diabaikan tetap harus dicatat.

## Batasan penting

- Ini adalah runner khusus aplikasi daftar tugas, bukan interpreter umum untuk semua kalimat bahasa Indonesia.
- Deklarasi data, koleksi penyimpanan selain `tugas`, dan aksi di luar bentuk yang didukung ditolak.
- Jangan menganggap sintaks yang berhasil diparse pasti dapat dijalankan.
- Jangan memakai perintah v0.1 biasa untuk menjalankan file human-vNext; gunakan `jalankan-human-vnext` secara eksplisit.
