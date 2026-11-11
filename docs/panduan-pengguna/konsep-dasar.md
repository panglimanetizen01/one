# Konsep Dasar One

## Data

Data dapat diberi nama dengan bentuk deklarasi:

```text
nama adalah "Budi"
umur adalah 18
```

Assignment menggunakan `menjadi`:

```text
umur menjadi 19
```

## Nama lebih dari satu kata

One v0.1 mendukung grammar-bounded multi-word names ketika produksi yang digunakan mengizinkannya:

```text
nama pengguna adalah "Budi"
saldo rekening menjadi 100000
```

Batas nama ditentukan grammar, bukan tebakan berdasarkan kalimat terpanjang.

## Aksi

Aksi inti harus eksplisit. Contoh:

```text
tampilkan nama
sapa teman
buat daftar belanja
tambahkan apel ke daftar buah
hapus telur dari daftar belanja
```

Pembuatan daftar adalah deklarasi nama, bukan penimpaan variabel. Nama yang sudah dipakai pada ruang lingkup yang sama tidak dapat dibuat ulang; One mengembalikan diagnostik `E2001`. Ruang lingkup bersarang tetap dapat menggunakan nama lokal yang menutupi nama dari ruang lingkup luar.

Kalimat bebas tidak menjadi aksi secara otomatis.

## Kondisi

Contoh bentuk yang didukung:

```text
kalau umur sudah 17 tahun
    tampilkan "Boleh masuk"
kalau tidak
    tampilkan "Belum boleh masuk"
```

## Perulangan

Bentuk dasar:

```text
untuk setiap teman di daftar teman
    tampilkan teman
```

dan:

```text
selama kondisi
    tampilkan "jalan"
```

Kondisi tetap harus memenuhi grammar; One tidak menebak arti frasa yang belum memiliki produksi.

## Indentasi

Satu level indentasi adalah tepat empat spasi. Tab ditolak. Block tidak boleh kosong.

## Prinsip

> Natural pada permukaan; formal di bawahnya.

Ini berarti One berusaha mudah dibaca manusia tanpa mengorbankan determinisme compiler/interpreter.
