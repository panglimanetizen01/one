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

## Event

Event menggunakan produksi eksplisit `ketika`:

```text
ketika pengguna menekan tombol beli
    tampilkan "dibeli"
```

Pemicu event diproses sesuai representasi event One; teks bebas tidak otomatis menjadi semantic event tersembunyi.

Untuk detail grammar, lihat `docs/spec/one-v0.1-grammar.md`.
