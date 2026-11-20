# Referensi API Standard Library One v0.1

Dokumen ini merangkum built-in yang benar-benar terdaftar pada jalur parser → semantic analysis → IR → reference interpreter saat ini. Ini bukan janji bahwa semua domain standard library yang direncanakan sudah tersedia. Periksa [status fitur](status-fitur.md) dan ledger verifikasi sebelum menjadikan API sebagai dependency rilis.

## Sintaks pemanggilan

Argumen pertama memakai kata `dengan`; argumen berikutnya dipisahkan dengan `dan`.

```text
panjang adalah panjang teks dengan "Halo"
hasil adalah ganti teks dengan "kucing-kucing" dan "kucing" dan "anjing"
tampilkan panjang
tampilkan hasil
```

## Teks

| Built-in | Parameter | Hasil | Perilaku |
|---|---|---|---|
| `panjang teks` | teks | angka | Menghitung Unicode scalar value, bukan grapheme cluster. |
| `teks memuat` | teks, needle | boolean | Pencarian substring literal. |
| `teks diawali` | teks, prefix | boolean | Memeriksa prefix literal. |
| `teks diakhiri` | teks, suffix | boolean | Memeriksa suffix literal. |
| `pangkas teks` | teks | teks | Menghapus whitespace di awal/akhir. |
| `ganti teks` | teks, from, to | teks | Penggantian literal. |
| `pisah teks` | teks, separator | daftar | Pemisahan berdasarkan separator literal; separator kosong ditolak. |
| `ubah teks ke angka` | teks | angka | Mengubah teks menjadi integer; nilai yang tidak valid gagal saat runtime. |
| `huruf kecil` | teks | teks | Pemetaan lowercase Unicode. |
| `huruf besar` | teks | teks | Pemetaan uppercase Unicode; satu karakter dapat menjadi beberapa karakter. |
| `gabung teks` | daftar teks, separator | teks | Menggabungkan item teks dengan separator. |

Contoh:

```text
teks adalah "  kucing-kucing  "
bersih adalah pangkas teks dengan teks
ada adalah teks memuat dengan bersih dan "kucing"
awal adalah teks diawali dengan bersih dan "kucing"
akhir adalah teks diakhiri dengan bersih dan "kucing"
tampilkan bersih
tampilkan ada
tampilkan awal
tampilkan akhir
```

## Daftar dan pemetaan

| Built-in | Parameter | Hasil | Perilaku |
|---|---|---|---|
| `panjang daftar` | daftar | angka | Jumlah item. |
| `ambil indeks` | daftar, indeks angka | nilai | Indeks negatif/di luar rentang gagal. |
| `jumlah daftar angka` | daftar angka | angka | Penjumlahan integer terperiksa; tipe salah atau overflow gagal. |
| `nilai pemetaan` | pemetaan, kunci teks | nilai | Membaca nilai untuk kunci teks. |
| `atur pemetaan` | pemetaan, kunci teks, nilai | pemetaan | Menghasilkan pemetaan dengan nilai yang diperbarui. |

Operasi pemetaan bergantung pada grammar pemetaan human-core yang opt-in. Jangan menganggap sintaks pemetaan eksperimental sebagai bagian dari parser v0.1 default.

## Integer

| Built-in | Parameter | Hasil | Perilaku |
|---|---|---|---|
| `nilai mutlak` | angka | angka | Gagal untuk nilai yang tidak dapat direpresentasikan, termasuk nilai minimum integer bertanda. |
| `bagi bulat` | angka, angka | angka | Pembagian integer terperiksa; pembagi nol ditolak. |
| `angka sisa bagi` | angka, angka | angka | Sisa pembagian integer terperiksa; pembagi nol dan overflow ditolak. |
| `angka minimum` | angka, angka | angka | Nilai terkecil. |
| `angka maksimum` | angka, angka | angka | Nilai terbesar. |

Angka v0.1 adalah integer bertanda 64-bit. Float dan angka pecahan bukan bagian dari tipe nilai ini. Operasi aritmetika yang overflow menghasilkan diagnostic runtime, bukan wraparound diam-diam.

## JSON

| Built-in | Parameter | Hasil | Perilaku |
|---|---|---|---|
| `ubah nilai ke json` | nilai | teks JSON | Serialisasi deterministik; kunci pemetaan diurutkan. |
| `nilai dari json` | teks JSON | nilai | Parsing bounded untuk teks, integer, boolean, nothing/null, daftar, dan pemetaan. |

Batas parser/serializer adalah 16 MiB berdasarkan byte UTF-8. Input JSON kosong, nesting berlebihan, kunci duplikat, angka float/exponent, integer di luar rentang, dan bentuk JSON tidak valid ditolak. JSON bukan jalan pintas untuk menambahkan tipe float ke runtime.

## UTC dan waktu

| Built-in | Parameter | Hasil | Perilaku |
|---|---|---|---|
| `waktu utc dari unix` | angka integer Unix seconds | teks | Format deterministik `YYYY-MM-DDTHH:MM:SSZ`, tahun 0000–9999. |
| `unix dari waktu utc` | teks timestamp UTC | angka integer Unix seconds | Parsing ketat timestamp `YYYY-MM-DDTHH:MM:SSZ`. |

Parser menolak tanggal kalender yang tidak valid, offset selain `Z`, fractional seconds, leap seconds, dan bentuk non-kanonis. Built-in ini tidak membaca jam host atau menebak zona waktu lokal.

## Error dan batas dukungan

- `E2018`: jumlah argumen built-in tidak cocok.
- `E2043`: tipe argumen tidak cocok dengan kontrak built-in.
- `R3022`: nilai runtime tidak valid atau operasi terperiksa gagal.
- `R3023`: overflow aritmetika integer.

Domain concurrency, logging/configuration, randomness, dan API persistence yang lengkap belum tersedia sebagai standard-library API stabil. Operasi effectful filesystem, database, process, dan network tetap tunduk pada capability/policy serta status verifikasi masing-masing; built-in murni tidak memberikan izin efek secara otomatis.

## Verifikasi

Tes end-to-end source-level berada di `crates/one-core/tests/stdlib_builtins.rs`; helper Rust berada di `crates/one-core/src/stdlib.rs`. Referensi ini menjelaskan implementasi saat ini, bukan bukti bahwa seluruh ekosistem standard library sudah lolos milestone 100%.
