# Menjalankan contoh daftar tugas human-vNext

> **Eksperimental.** Ini adalah vertical slice awal untuk sintaks human-vNext, bukan bukti bahwa semua sintaks sudah menjadi bahasa normatif. Jalur v0.1 tetap menjadi default.

## 1. Source

Program ada di [daftar-tugas.one](../../examples/human-vnext/daftar-tugas.one). Program mendeklarasikan tiga tindakan: menambah tugas, menampilkan seluruh tugas, dan menandai tugas selesai. Deklarasi persistensi diwajibkan oleh runtime.

## 2. Build dan jalankan

Dari root repository, gunakan Cargo untuk menjalankan CLI:

```sh
cargo run -p one-cli -- jalankan-human-vnext examples/human-vnext/daftar-tugas.one
```

Penyimpanan default berada di `examples/human-vnext/daftar-tugas.tugas.db`. Untuk memilih lokasi lain:

```sh
cargo run -p one-cli -- jalankan-human-vnext examples/human-vnext/daftar-tugas.one --data ./data/tugas.db
```

CLI menampilkan menu perintah interaktif:

- `tambah` — minta judul tugas dan simpan.
- `lihat` — tampilkan semua tugas beserta statusnya.
- `selesai` — minta ID tugas dan simpan status selesai.
- `keluar` — tutup aplikasi.

Buka kembali proses dengan perintah yang sama untuk memuat data dari berkas. Data disimpan setiap kali ada perubahan melalui berkas sementara dan penggantian nama. Bila data rusak atau penyimpanan gagal, runtime mengembalikan error; ia tidak mengklaim sukses.

## 3. Batasan saat ini

- Sintaks human-vNext belum normatif dan hanya mendukung bentuk tindakan yang dipakai oleh contoh ini.
- Runner task-list adalah interpreter eksperimental khusus untuk vertical slice ini; AST belum diturunkan ke IR kanonis ONE v0.1.
- Judul tugas harus satu baris dan tidak boleh kosong atau mengandung karakter kontrol.
- File storage adalah format internal tab-delimited; migrasi format belum dijanjikan.
- Build, test, Clippy, dan CI pada commit yang berisi contoh ini harus lulus sebelum slice dinyatakan verified.
- Jangan gunakan direktori data produksi untuk eksperimen. Pilih path eksplisit melalui `--data` bila diperlukan.
