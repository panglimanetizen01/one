# Mulai dari Nol

Panduan ini untuk pengguna yang belum terbiasa dengan bahasa pemrograman.

## 1. Pahami model One

One memakai permukaan sintaks yang berorientasi manusia, tetapi tetap memiliki grammar formal. Artinya tulisan One boleh terasa lebih dekat dengan bahasa sehari-hari, namun tidak berarti semua kalimat bahasa Indonesia dapat dijalankan.

Contoh yang sesuai grammar:

```text
nama adalah "Budi"
umur adalah 18

tampilkan nama
```

Contoh seperti `saya ingin kamu menampilkan nama` tidak otomatis menjadi program karena One tidak menebak maksud kalimat bebas.

## 2. Alur dasar

Untuk pekerjaan sederhana, pola yang digunakan adalah:

```text
buat project
→ tulis program
→ periksa
→ jalankan
→ test jika project memiliki test
→ bangun/package sesuai kebutuhan
```

## 3. Pemeriksaan pertama

Gunakan:

```text
one periksa src/main.one
```

Pemeriksaan memvalidasi source tanpa menjalankannya. Jika gagal, baca diagnostic code dan lokasi source yang dilaporkan sebelum memperbaiki program.

## 4. Menjalankan program

Jika pemeriksaan berhasil:

```text
one jalankan src/main.one
```

Alias teknis `one check` dan `one run` tetap tersedia.

## 5. Jika terjadi error

Jangan mengubah banyak hal sekaligus. Periksa baris yang dilaporkan, bandingkan dengan grammar, perbaiki satu penyebab, lalu jalankan `one periksa` lagi.

One menolak syntax yang tidak lengkap atau ambigu daripada menebak maksud pengguna.

## 6. Catatan status

Cara instalasi/distribusi final dan compiler native belum boleh diasumsikan tersedia hanya dari dokumentasi arsitektur. Ikuti instruksi pada halaman instalasi untuk batas kemampuan versi yang sedang digunakan.
