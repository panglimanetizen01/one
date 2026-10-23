# Frontend instruksi manusia vNext menuju runtime kanonis

## Status

Frontend ini merupakan jalur integrasi bertahap. Baris pertama wajib mendeklarasikan program:

```one
Buat program kalkulator
```

Baris sesudahnya menggunakan grammar instruksi Indonesia yang telah dipahami parser ONE: deklarasi dengan `adalah`, perubahan dengan `menjadi`, ekspresi aritmetika/kesetaraan, kondisi `kalau` / `kalau tidak`, pengulangan, aksi `tampilkan`, dan fungsi `buat pekerjaan`.

Jalankan melalui perintah eksplisit:

```sh
one jalankan-human-core examples/human-vnext/kalkulator.one
```

Alias teknis:

```sh
one run-human-core examples/human-vnext/kalkulator.one
```

## Otoritas semantik

Frontend tidak membuat evaluator terpisah. Ia memvalidasi deklarasi program, lalu meneruskan isi ke pipeline resmi ONE:

```text
Deklarasi program manusia
  -> parser kanonis
  -> semantic analysis dan resolusi nama/tipe
  -> lowering ke One IR
  -> reference interpreter
```

Dengan demikian, ekspresi, fungsi, kondisi, dan diagnostik menggunakan implementasi semantik kanonis yang sama. Kesalahan parse/semantik menghasilkan kode keluar bukan nol. Perintah v0.1 tetap menjadi jalur terpisah dan tidak otomatis mendeteksi format vNext.

## Contoh

- `examples/human-vnext/kalkulator.one` — fungsi, parameter, hasil, ekspresi, kondisi, dan output.
- `examples/human-vnext/pencatat-skor.one` — nilai, ekspresi dan kondisi.

## Batasan yang belum diklaim selesai

Deklarasi modul/import multi-file, tipe koleksi baru pada frontend vNext, dan abstraksi persistensi lintas aplikasi belum disediakan oleh adapter ini. Adapter memakai grammar, tipe, IR, dan runtime kanonis yang telah ada; ia bukan klaim bahwa seluruh cakupan milestone 60% sudah lengkap. Jangan menghitung fitur yang belum diintegrasikan sebagai selesai.
