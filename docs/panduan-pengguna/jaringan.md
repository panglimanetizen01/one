# Jaringan

One memiliki fondasi jaringan typed dan capability-aware. Dokumentasi pengguna harus mengikuti bukti runtime, bukan daftar protokol pada registry saja.

## Prinsip

Operasi effectful memakai capability yang eksplisit. Menulis source atau dokumentasi tidak otomatis memberikan akses host.

## Status

Beberapa vertical slice network telah memiliki implementasi dan bukti runtime pada source revision yang tercatat di verification inventory. Protokol/adapter yang hanya terdaftar pada registry tidak boleh dianggap otomatis tersedia.

Untuk daftar tepat yang sedang diverifikasi, lihat `docs/verification/one-v0.1-implementation-status.md` dan dokumen verification yang dirujuk olehnya.

## Batas

Jika sebuah adapter belum memiliki bukti runtime yang sesuai, dokumentasi pengguna harus menyebutnya belum tersedia atau belum terverifikasi. Jangan menggunakan nama protokol sebagai bukti dukungan.
