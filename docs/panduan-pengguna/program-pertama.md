# Program Pertama

Mulai dengan program kecil yang dapat diperiksa dan dijalankan.

```text
nama adalah "Budi"
umur adalah 18

tampilkan nama
```

## Membaca program

- `nama adalah "Budi"` membuat data bernama `nama`.
- `umur adalah 18` membuat data angka.
- `tampilkan nama` meminta nilai `nama` ditampilkan.

## Periksa

Simpan source sebagai file `.one`, lalu jalankan:

```text
one periksa nama-file.one
```

Perintah `periksa` tidak menjalankan program. Ia memeriksa apakah source dapat diterima oleh pipeline bahasa.

## Jalankan

Jika pemeriksaan berhasil:

```text
one jalankan nama-file.one
```

## Jika gagal

Contoh source yang tidak lengkap:

```text
nama menjadi
```

Jangan mengharapkan One mengisi bagian yang hilang. Perbaiki source sesuai grammar lalu periksa ulang.

## Prinsip penting

Sintaks One terlihat natural, tetapi tetap formal. Hanya bentuk yang memiliki produksi grammar yang sah yang dapat dijalankan.
