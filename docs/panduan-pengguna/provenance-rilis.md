# Panduan provenance paket rilis

Panduan ini menjelaskan batas penggunaan provenance paket ONE. Rincian operasional workflow ada di dokumentasi engineering; halaman ini tidak menggantikan pemeriksaan keamanan atau persetujuan rilis.

## Sebelum rilis

- Pastikan workflow rilis sudah tersedia pada branch default repository. Workflow `workflow_dispatch` yang baru ditambahkan ke branch pull request belum bisa dijalankan dari Actions sampai perubahan workflow masuk ke branch default.
- Minta pemilik repository mengonfigurasi environment terlindungi `one-release` dengan aturan persetujuan yang sesuai.
- Secret `ONE_RELEASE_SIGNING_SEED` harus berisi seed Ed25519 32-byte dalam 64 karakter heksadesimal. Jangan simpan seed di repository, input workflow, log, atau artefak.
- Secret `ONE_TRUSTED_PUBLISHERS` harus berisi pemetaan `publisher-id=public-key-hex` yang disediakan melalui kanal distribusi kunci tepercaya yang independen. Jangan membentuk trust store dari seed privat pada job rilis.
- Tinjau commit, manifest `one.toml`, dan `src/main.one` sebelum menyetujui environment rilis.

## Menjalankan workflow

Setelah workflow berada di branch default dan environment telah dikonfigurasi:

1. Buka Actions dan pilih **ONE Package Provenance Release**.
2. Jalankan workflow dengan path proyek relatif terhadap repository dan ID publisher yang sudah ada di trust store.
3. Pastikan langkah build, penandatanganan provenance V2, dan `one verify-package` semuanya berhasil.
4. Simpan artefak `.onepkg`, sidecar `.onepkg.provenance`, dan checksum SHA-256 bersama-sama.
5. Distribusikan trust store secara terpisah dari paket. Verifikasi paket tidak membuktikan keaslian trust store apabila sumber kunci itu sendiri tidak tepercaya.

## Batas jaminan

Provenance V2 mengikat artefak ke identitas publisher dan snapshot sumber yang diverifikasi oleh CLI. Keberhasilan workflow hanya membuktikan input, commit, artefak, dan trust store yang digunakan pada run itu. Itu bukan persetujuan otomatis atas seluruh rilis ONE. Semua gerbang platform, standard library, keamanan, kompatibilitas, dokumentasi, dan Definition of Done tetap harus lulus pada SHA rilis final.
