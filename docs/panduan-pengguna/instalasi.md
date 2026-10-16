# Instalasi dan Lingkungan

One v0.1 saat ini memiliki executable core dan CLI, tetapi distribusi installer final untuk pengguna umum belum menjadi klaim yang boleh dibuat dari dokumentasi arsitektur saja.

## Yang tersedia

Repository menyediakan CLI, reference interpreter, project scaffolding, test/conformance infrastructure, build/package artifact generation, serta jalur verifikasi Linux dan Windows yang terdokumentasi.

## Jika menggunakan source repository

Gunakan instruksi build/verifikasi yang terdapat di repository dan cocok dengan platform yang sedang dipakai. Jangan mengarang nama installer, package manager, atau binary release yang belum tercatat sebagai artefak resmi.

## Windows dan Linux

Windows dan Linux adalah target platform yang diperlakukan sebagai first-class pada arsitektur dan memiliki jalur verifikasi independen. Bukti dukungan harus selalu dikaitkan dengan source revision yang diuji.

## Catatan penting

`one bangun` saat ini menghasilkan `.oneir`, bukan executable native. `one package` menghasilkan `.onepkg`, bukan platform installer.

Ketika mekanisme distribusi resmi berubah, halaman ini harus diperbarui atau diganti; jangan mempertahankan langkah instalasi lama yang sudah tidak benar.
