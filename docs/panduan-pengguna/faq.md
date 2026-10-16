# FAQ One

## Apakah One sudah selesai 100%?

Belum. One masih dikembangkan. Karena itu dokumentasi menggunakan status fitur dan tidak menyamakan roadmap dengan kemampuan tersedia.

## Apakah semua kalimat bahasa Indonesia bisa dijalankan?

Tidak. One memiliki grammar formal. Permukaan sintaksnya berorientasi manusia, tetapi source yang tidak cocok dengan grammar ditolak.

## Kenapa One tidak menebak maksud saya?

Agar program deterministik. Ambiguity harus ditolak, bukan dipilih berdasarkan tebakan.

## Apa beda `periksa` dan `jalankan`?

`periksa` memvalidasi source tanpa eksekusi. `jalankan` menjalankan source melalui reference interpreter.

## Apakah `bangun` menghasilkan EXE?

Belum. v0.1 menghasilkan `.oneir` deterministik.

## Apakah `package` menghasilkan installer?

Belum. v0.1 menghasilkan `.onepkg` deterministik.

## Di mana melihat status fitur?

Gunakan `status-fitur.md` dan implementation-status repository.

## Bolehkah dokumentasi diubah?

Ya. Dokumentasi resmi memang versioned dan editable. Jika sebuah instruksi salah atau sudah usang, perbaiki atau hapus melalui Git.
