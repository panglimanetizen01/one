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
