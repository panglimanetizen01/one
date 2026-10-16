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

## Urutan umum

Untuk project baru:

```text
one buat project-saya
one periksa project-saya/src/main.one
one jalankan project-saya/src/main.one
one test project-saya
```

Perintah harus dijalankan dengan argumen yang sesuai kontrak. CLI tidak menafsirkan kalimat natural-language bebas sebagai command.

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

## `bangun` dan `package`

Pada v0.1 saat ini:

- `one bangun` menghasilkan intermediate/debug artifact `.oneir` deterministik.
- `one package` menghasilkan `.onepkg` yang tervalidasi dan deterministik.

Keduanya **belum** boleh disebut sebagai native executable compiler atau platform installer.

## Exit status

- `0` — command berhasil;
- `1` — kegagalan source, semantic, runtime, atau build-domain;
- `2` — kesalahan penggunaan CLI atau environment/tool.

Untuk kontrak lengkap, lihat `docs/spec/one-v0.1-cli-contract.md`.
