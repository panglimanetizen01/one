# Kesalahan dan Debugging

Jika One menolak program, gunakan proses sederhana berikut:

1. Baca diagnostic code dan lokasi source.
2. Buka baris yang disebutkan.
3. Bandingkan bentuk source dengan grammar yang berlaku.
4. Perbaiki satu penyebab.
5. Jalankan `one periksa` lagi.
6. Setelah lolos, gunakan `one jalankan` atau `one test` sesuai kebutuhan.

## Kesalahan umum

### Indentasi

One v0.1 menggunakan empat spasi untuk satu level. Tab ditolak.

### Pernyataan tidak lengkap

```text
nama menjadi
```

Tidak ada nilai yang boleh ditebak. Lengkapi expression.

### Kalimat percakapan bebas

```text
saya ingin kamu menampilkan nama pengguna
```

Kalimat tersebut bukan produksi action yang sah. Gunakan bentuk action yang didukung, misalnya:

```text
tampilkan nama pengguna
```

### Ambiguitas

Jika lebih dari satu semantic parse berbeda mungkin terjadi, One menolak source tersebut. Ini disengaja agar program tidak berubah makna berdasarkan tebakan.

## Prinsip debugging

Jangan menyelesaikan error dengan menambahkan kata acak sampai parser menerima source. Cari produksi grammar yang dimaksud, lalu tulis source yang tepat.
