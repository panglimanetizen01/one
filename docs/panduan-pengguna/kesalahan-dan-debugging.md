# Kesalahan dan Debugging

One menolak source yang tidak sesuai grammar atau kontrak tipe; program tidak diperbaiki dengan menebak maksud penulis. Gunakan urutan diagnosis di bawah agar perbaikan kecil dan dapat diverifikasi.

## Alur diagnosis

1. Jalankan `one periksa program.one` terlebih dahulu.
2. Baca **kode diagnostic**, pesan, nomor baris, dan kolom. Buka baris source yang disebutkan.
3. Cocokkan source dengan grammar dan tipe yang berlaku; jangan menambahkan kata acak sampai parser menerima source.
4. Perbaiki satu penyebab, lalu ulangi `one periksa`.
5. Setelah validasi berhasil, jalankan `one jalankan program.one`.
6. Untuk project, gunakan `one test nama-project`; setelah itu baru jalankan `one bangun` atau `one package` sesuai kebutuhan.

Untuk melihat struktur parser, gunakan `one lihat struktur program.one` atau alias `one ast program.one`. AST membantu memeriksa bagaimana source diparse; AST bukan hasil eksekusi.

## Kesalahan grammar dan semantic yang umum

### Indentasi

One v0.1 menggunakan empat spasi untuk satu level. Tab ditolak. Pastikan semua baris di dalam blok memiliki indentasi yang konsisten.

### Pernyataan tidak lengkap

Contoh salah:

```text
nama menjadi
```

Tidak ada nilai yang boleh ditebak. Lengkapi expression dengan nilai yang valid.

### Kalimat percakapan bebas

Contoh salah:

```text
saya ingin kamu menampilkan nama pengguna
```

Kalimat tersebut bukan produksi action yang sah. Gunakan bentuk yang didukung, misalnya:

```text
tampilkan nama pengguna
```

### Nama atau fungsi tidak ditemukan

Jika diagnostic menyatakan nama atau pekerjaan belum dibuat, periksa ejaan nama, urutan deklarasi, dan apakah fungsi/built-in tersebut memang terdaftar. Built-in yang belum tersedia tidak dapat dipanggil hanya karena namanya tampak masuk akal.

### Jumlah argumen atau tipe salah

- `E2018`: jumlah argumen fungsi tidak sesuai.
- `E2043`: tipe argumen tidak sesuai dengan kontrak fungsi.

Bandingkan parameter yang diperlukan pada [referensi Standard Library](referensi-standard-library.md). Jangan mengubah nilai menjadi teks hanya untuk menyembunyikan error jika operasi membutuhkan angka atau daftar.

## Kesalahan runtime

- `R3022`: operasi runtime menerima nilai yang tidak valid atau operasi terperiksa gagal, misalnya pembagi nol atau timestamp di luar rentang.
- `R3023`: operasi aritmetika integer overflow. One tidak melakukan wraparound diam-diam.

Periksa nilai masukan dan batas representasi. Untuk JSON, ingat bahwa tipe angka One adalah integer; angka float/exponent tidak didukung oleh parser JSON saat ini.

## Capability dan scope

Operasi filesystem, process, database, dan network tidak otomatis mendapat izin hanya karena source valid.

- `N3205`: capability yang dibutuhkan tidak diberikan secara eksplisit.
- `N3209`: permintaan berada di luar scope yang diizinkan.

Contoh pembacaan file dengan target exact:

```sh
one jalankan program.one --izinkan filesystem.read --target data/input.txt
```

Untuk membatasi filesystem ke sebuah root:

```sh
one jalankan program.one --izinkan filesystem.read --root data --target data/input.txt
```

Target harus berada di bawah root berdasarkan komponen path; target di luar root ditolak. Root scope adalah pembatasan policy, bukan sandbox OS lengkap. Jangan menyelesaikan error scope dengan memberikan izin yang lebih luas dari kebutuhan.

Untuk database dan network, pastikan capability yang benar diberikan dan `--target` sama dengan resource/URL yang diminta source. Periksa juga batas dukungan fitur pada dokumentasi CLI dan status fitur; source yang lolos parser tidak berarti backend tersedia atau terverifikasi pada semua platform.

## Jika masalah tetap muncul

1. Simpan source minimal yang mereproduksi masalah.
2. Catat perintah persis, kode diagnostic, platform, versi CLI, dan SHA repository.
3. Tambahkan regresi yang gagal sebelum perbaikan dan lulus sesudah perbaikan.
4. Jalankan kontrak, build, Clippy, workspace tests, dan verifikasi platform yang relevan pada SHA yang sama.
5. Perbarui dokumentasi status berdasarkan bukti; jangan menyebut masalah sebagai lulus hanya karena tidak muncul pada satu mesin.

Semantik One berasal dari grammar, semantic analyzer, IR, dan runtime deterministik. LLM atau kalimat bebas bukan otoritas untuk mengubah arti program.
