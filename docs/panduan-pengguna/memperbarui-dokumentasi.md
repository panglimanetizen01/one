# Memperbarui Dokumentasi

Dokumentasi One adalah bagian versioned dari repository. Ia boleh ditambah, diperbaiki, dipindahkan, atau dihapus ketika tidak lagi sesuai.

## Siklus perubahan

```text
ubah implementasi/kontrak
→ verifikasi
→ cari dokumentasi yang terdampak
→ tambah/ubah/hapus instruksi
→ periksa contoh dan tautan
→ commit
→ review/pull request
```

## Aturan

1. Jangan mengklaim fitur berdasarkan roadmap saja.
2. Contoh yang disebut valid harus cocok dengan grammar dan evidence yang berlaku.
3. Jika feature behavior berubah, cari semua panduan yang merujuknya.
4. Jika instruksi sudah salah, hapus atau perbaiki; jangan mempertahankan informasi salah sebagai dokumentasi aktif.
5. User guide bukan pengganti normative contract.
6. Perubahan dokumentasi tidak memerlukan GitHub paid upgrade.

## Saat fitur baru selesai

Perbarui implementation status dan evidence terlebih dahulu. Setelah itu tambahkan atau perbarui panduan pengguna, contoh, status fitur, dan FAQ yang terdampak. Dokumentasikan hanya batas yang benar-benar telah dibuktikan.
