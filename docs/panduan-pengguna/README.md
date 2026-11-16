# Panduan Pengguna One

Selamat datang di dokumentasi resmi pengguna **One**.

One dirancang agar pemrograman lebih mudah dipahami manusia, dengan sintaks awal berbahasa Indonesia. Dokumentasi ini ditulis untuk pengguna yang ingin belajar dan mencoba One tanpa harus memahami detail compiler terlebih dahulu.

> **Penting:** One masih dalam pengembangan. Halaman ini adalah dokumentasi resmi yang **versioned dan dapat berubah**. Fitur dapat ditambahkan, diubah, atau dihapus ketika implementasi dan kontraknya berkembang.

## Mulai di sini

Jika baru mengenal One, ikuti urutan berikut:

1. [Mulai dari nol](mulai-dari-nol.md)
2. [Instalasi dan lingkungan](instalasi.md)
3. [Program pertama](program-pertama.md)
4. [Konsep dasar](konsep-dasar.md)
5. [Membuat project](membuat-project.md)
6. [Perintah CLI](perintah-cli.md)

Setelah memahami dasar, lanjutkan ke topik yang sesuai kebutuhan:

- [Data dan nama](data-dan-nama.md)
- [Kondisi dan perulangan](kondisi-dan-perulangan.md)
- [Fungsi dan event](fungsi-dan-event.md)
- [Kesalahan dan debugging](kesalahan-dan-debugging.md)
- [Test, build, dan package](test-build-package.md)
- [Jaringan](jaringan.md)
- [Interoperabilitas](interoperabilitas.md)
- [FAQ](faq.md)

## Status fitur

Baca [status fitur](status-fitur.md) sebelum mengandalkan fitur yang belum familiar. Dokumentasi tidak boleh menyamakan rancangan masa depan dengan kemampuan yang sudah diverifikasi.

## Aturan dokumentasi

- **Didukung** — tersedia dan memiliki bukti yang sesuai dengan dokumentasi status/verification.
- **Eksperimental** — tersedia tetapi kontrak atau perilakunya masih dapat berubah.
- **Dalam pengembangan** — sedang dikerjakan dan belum boleh dianggap selesai.
- **Dirancang** — sudah memiliki desain/kontrak, tetapi belum tersedia sebagai kemampuan yang dapat digunakan.
- **Belum tersedia** — belum dapat digunakan pada versi yang sedang didokumentasikan.

Contoh pada halaman ini yang disebut **dapat digunakan** harus mengikuti grammar, CLI contract, implementation status, dan bukti verifikasi yang berlaku.

## Jika menemukan dokumentasi yang salah

Jangan mempertahankan instruksi yang sudah tidak benar hanya karena pernah benar. Dokumentasi One dapat:

- ditambahkan;
- diperbaiki;
- diperbarui;
- dipindahkan;
- atau dihapus jika sudah tidak sesuai.

Perubahan dokumentasi mengikuti Git seperti perubahan kode: periksa implementasi/kontrak, ubah halaman yang terdampak, periksa tautan dan contoh, lalu commit/pull request.

Lihat [cara memperbarui dokumentasi](memperbarui-dokumentasi.md).

## Batas otoritas

Panduan pengguna menjelaskan cara menggunakan kemampuan One yang sudah ada. Panduan ini **bukan** sumber kebenaran semantik bahasa. Grammar, CLI contract, implementation status, dan verification evidence tetap menjadi sumber normatif sesuai area masing-masing.

One juga tidak mengubah kalimat bebas menjadi perintah executable secara otomatis. Sintaks yang digunakan harus sesuai grammar yang didukung.

## Ketersediaan pemeriksaan otomatis

Pemeriksaan otomatis di layanan CI adalah bagian dari infrastruktur pengembangan. Jika runner CI tidak tersedia atau proses penyiapan toolchain gagal, hal itu **tidak otomatis berarti program One atau kode pengguna salah**. Status kemampuan One tetap ditentukan oleh implementasi, kontrak, dan bukti pengujian pada revisi yang tepat.

Untuk melihat arti status fitur, baca [status fitur](status-fitur.md). Catatan teknis insiden CI tersedia di [catatan infrastruktur CircleCI](../verification/circleci-infrastructure-incident-2026-11-16.md); catatan tersebut bukan instruksi penggunaan One.
