# Perintah CLI One

CLI adalah pintu masuk command-line One v0.1.

| Perintah | Alias | Kegunaan |
|---|---|---|
| `one buat <folder>` | `one new <folder>` | membuat skeleton project |
| `one periksa <file.one>` | `one check <file.one>` | memeriksa source tanpa eksekusi |
| `one jalankan <file.one>` | `one run <file.one>` | menjalankan melalui reference interpreter |
| `one test <folder>` | — | menjalankan program project dan fixture conformance lokal yang ditemukan |
| `one bangun <file.one>` | `one build <file.one>` | menghasilkan `.oneir` deterministik |
| `one package <folder>` | — | menghasilkan `.onepkg` deterministik setelah validasi |
| `one lihat struktur <file.one>` | `one ast <file.one>` | melihat AST |
| `one version` | — | melihat versi |
| `one help` | — | melihat bantuan |

## Menjalankan orchestration DNS

Vertical slice orchestration pertama yang tersedia secara eksperimental adalah pencarian alamat DNS melalui source One `cari alamat`.

Program:

```text
cari alamat "127.0.0.1"
```

Jalankan dengan izin eksplisit dan target yang sama:

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
```

Alias teknis:

```text
one run main.one --allow network.discover --target 127.0.0.1
```

Aturan keamanan:

- tanpa `--izinkan`, orchestration ditolak;
- `network.discover` adalah capability yang saat ini tersedia melalui permission CLI ini;
- `--target` membatasi policy ke target tersebut;
- target lain ditolak oleh governance;
- permission CLI bukan bagian dari grammar One dan tidak mengubah makna program.

Fitur ini **Eksperimental**, bukan klaim bahwa seluruh katalog orchestration sudah didukung.

## Membaca berkas melalui orchestration filesystem

Vertical slice filesystem pertama yang tersedia secara eksperimental adalah membaca teks UTF-8 melalui source One `baca berkas`.

Program:

```text
baca berkas "data/contoh.txt"
```

Jalankan dengan izin eksplisit dan target yang sama:

```text
one jalankan main.one --izinkan filesystem.read --target data/contoh.txt
```

Alias teknis:

```text
one run main.one --allow filesystem.read --target data/contoh.txt
```

Aturan keamanan:

- tanpa capability `filesystem.read`, source orchestration ditolak;
- `--target` wajib sama dengan target yang diminta program;
- target di luar scope yang diberikan ditolak oleh governance;
- kegagalan pembacaan backend dilaporkan sebagai kegagalan adapter, bukan hasil sukses palsu;
- fitur ini hanya membaca teks UTF-8 dan belum merupakan klaim dukungan filesystem umum.

Fitur ini **Eksperimental**, bukan klaim bahwa seluruh katalog filesystem sudah didukung.

## `bangun` dan `package`

Pada v0.1 saat ini:

- `one bangun` menghasilkan intermediate/debug artifact `.oneir` deterministik.
- `one package` menghasilkan `.onepkg` yang tervalidasi dan deterministik.

Keduanya **belum** boleh disebut sebagai native executable compiler atau platform installer.

## Verification boundary

Exact-SHA cross-platform evidence untuk filesystem-read terakhir dijalankan secara independen melalui mirror `panglimanetizen01/one-windows-verification` pada run `38901366447`, dengan target canonical `0e6104ccf79f9f9cb3782ccf14abcd62a109c01b`. Ubuntu dan Windows Server 2025 sama-sama menyelesaikan checkout exact-SHA, contract artifacts, build, Clippy, tests, real-network runtime tests, dan offline verification. Bukti ini memverifikasi revision tersebut; tidak berarti seluruh katalog orchestration sudah supported.

Dokumentasi verifikasi historis tetap mencatat revision sebelumnya `43cf7337ac963a0e18512aacf56dbfd7ab6bcd3a` dan run `38896533761` sebagai baseline P0 sebelumnya.
