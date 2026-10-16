# Data dan Nama

Nama One dapat terdiri dari beberapa kata jika grammar pada posisi tersebut mengizinkannya.

```text
nama pengguna adalah "Budi"
saldo rekening adalah 100000
```

Gunakan nama secara konsisten:

```text
tampilkan nama pengguna
```

Kata struktural seperti `adalah`, `menjadi`, `ke`, dan `dari` dapat menjadi batas sintaks sesuai produksi aktif. Jangan menganggap semua kata tersebut selalu dilarang berada dalam setiap nama.

Jika suatu source dapat menghasilkan lebih dari satu struktur semantic yang berbeda, parser menolak ambiguity daripada memilih secara diam-diam.
