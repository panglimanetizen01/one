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

- Teks: `panjang teks`, `teks memuat`, `teks diawali`, `teks diakhiri`, `huruf kecil`, `huruf besar`, `pangkas teks`, `ganti teks`, `pisah teks`, `ubah teks ke angka`.
- Daftar: `panjang daftar`, `ambil indeks` (indeks mulai dari nol), `gabung teks` (hanya daftar teks), `jumlah daftar angka` (penjumlahan integer dengan pemeriksaan overflow).
- Bilangan: `nilai mutlak`, `bagi bulat`, `angka minimum`, `angka maksimum`.
- Pemetaan: `nilai pemetaan`, `atur pemetaan` (saat ini digunakan bersama grammar pemetaan human-core yang opt-in).
- Serialisasi/pertukaran data: `ubah nilai ke json` dan `nilai dari json` mendukung JSON deterministik untuk nilai ONE yang tersedia; angka pecahan/eksponen dan kedalaman lebih dari 128 tingkat ditolak.
- Waktu UTC: `waktu utc dari unix` memformat detik Unix menjadi `YYYY-MM-DDTHH:MM:SSZ`; `unix dari waktu utc` mem-parsing format UTC ketat itu kembali menjadi detik Unix. Keduanya memakai kalender Gregorian tahun 0000–9999 dan tidak membaca jam lokal host.

Analisis semantik memeriksa jumlah dan tipe argumen yang diketahui. Indeks negatif/di luar rentang, konversi yang gagal, pembagian nol, dan operasi integer yang tidak dapat direpresentasikan menghasilkan error; operasi tersebut tidak boleh dianggap berhasil atau menghasilkan nilai tebakan. Helper pemetaan saat ini belum seluruhnya tersedia sebagai fungsi bawaan sumber.

## Contoh waktu UTC

```text
waktu adalah waktu utc dari unix dengan 0
epoch adalah unix dari waktu utc dengan waktu
tampilkan waktu
tampilkan epoch
```

Format parser harus tepat `YYYY-MM-DDTHH:MM:SSZ`. Tanggal yang tidak valid, offset zona waktu, pecahan detik, dan detik kabisat ditolak secara deterministik.

## Event

Event menggunakan produksi eksplisit `ketika`:

```text
ketika pengguna menekan tombol beli
    tampilkan "dibeli"
```

Pemicu event diproses sesuai representasi event One; teks bebas tidak otomatis menjadi semantic event tersembunyi.

Untuk detail grammar, lihat `docs/spec/one-v0.1-grammar.md`.
