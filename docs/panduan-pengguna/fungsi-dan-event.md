# Fungsi dan Event

## Fungsi

v0.1 memiliki bentuk fungsi eksplisit menggunakan `buat pekerjaan`.

```text
buat pekerjaan hitung total
    terima harga barang
    terima pajak
    hasilnya adalah harga barang ditambah pajak
```

Pemanggilan juga eksplisit, misalnya:

```text
total adalah hitung total dengan harga dan pajak
```

One tidak menganggap kalimat arbitrer sebagai pemanggilan fungsi.

## Fungsi bawaan teks, daftar, dan bilangan

Pemanggilan fungsi bawaan memakai tata bahasa pemanggilan yang sama, tetapi nama dan jumlah argumennya ditentukan oleh kontrak runtime:

```text
teks adalah "  apel,jeruk  "
bersih adalah pangkas teks dengan teks
bagian adalah pisah teks dengan bersih dan ","
jumlah adalah panjang daftar dengan bagian
tampilkan jumlah
```

Subset bawaan yang saat ini terhubung ke jalur parser → analisis semantik → IR → interpreter:

- Teks: `panjang teks`, `teks memuat`, `pangkas teks`, `ganti teks`, `pisah teks`, `ubah teks ke angka`.
- Daftar: `panjang daftar`, `ambil indeks` (indeks mulai dari nol).
- Bilangan: `nilai mutlak`, `bagi bulat`, `angka minimum`, `angka maksimum`.

Analisis semantik memeriksa jumlah dan tipe argumen yang diketahui. Indeks negatif/di luar rentang, konversi yang gagal, pembagian nol, dan operasi integer yang tidak dapat direpresentasikan menghasilkan error; operasi tersebut tidak boleh dianggap berhasil atau menghasilkan nilai tebakan. Helper pemetaan saat ini belum seluruhnya tersedia sebagai fungsi bawaan sumber.

## Event

Event menggunakan produksi eksplisit `ketika`:

```text
ketika pengguna menekan tombol beli
    tampilkan "dibeli"
```

Pemicu event diproses sesuai representasi event One; teks bebas tidak otomatis menjadi semantic event tersembunyi.

Untuk detail grammar, lihat `docs/spec/one-v0.1-grammar.md`.
