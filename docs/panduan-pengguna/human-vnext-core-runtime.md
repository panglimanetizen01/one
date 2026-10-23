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

## Modul multi-file

Program dapat mengimpor modul relatif yang berada di dalam direktori proyek. Deklarasi modul harus berada tepat setelah header program:

```one
Buat program aplikasi modular
Gunakan modul "matematika.one"

hasil adalah jumlahkan dengan 20 dan 22
tampilkan hasil
```

File modul menggunakan header `Buat modul matematika` lalu mendeklarasikan fungsi dengan grammar ONE yang sama. Jalur absolut, `..`, ekstensi selain `.one`, siklus, dan import ganda ditolak. Semua badan modul digabungkan sebelum parse/semantic analysis kanonis, sehingga konflik nama terdeteksi oleh analyzer yang sama.

Jalankan contoh dengan `one jalankan-human-core examples/human-vnext/modul-utama.one`.

## Penyimpanan reusable

Core kini menyediakan `one_core::persistence::KeyValueStore`: format `ONE-KV-1` berversi, urutan serialisasi deterministik, batas ukuran/jumlah entri, penyimpanan melalui file sementara, serta penolakan data rusak dan kunci duplikat. API ini sudah memiliki unit tests untuk restart, corrupt storage, dan penghapusan. Aplikasi daftar tugas yang ada masih memakai format penyimpanan sendiri; migrasi ke API baru belum diklaim selesai.

## Contoh

- `examples/human-vnext/kalkulator.one` — fungsi, parameter, hasil, ekspresi, kondisi, dan output.
- `examples/human-vnext/pencatat-skor.one` — nilai, ekspresi dan kondisi.
- `examples/human-vnext/modul-utama.one` + `matematika.one` — import modul dan fungsi lintas file.

## Batasan yang belum diklaim selesai

Deklarasi modul/import multi-file, tipe koleksi baru pada frontend vNext, dan abstraksi persistensi lintas aplikasi belum disediakan oleh adapter ini. Adapter memakai grammar, tipe, IR, dan runtime kanonis yang telah ada; ia bukan klaim bahwa seluruh cakupan milestone 60% sudah lengkap. Jangan menghitung fitur yang belum diintegrasikan sebagai selesai.
