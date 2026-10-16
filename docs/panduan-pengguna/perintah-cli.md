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

```text
one jalankan main.one --izinkan network.discover --target 127.0.0.1
```

Tanpa permission eksplisit, orchestration ditolak; target berbeda berada di luar scope.

## Membaca berkas melalui orchestration filesystem

Vertical slice filesystem read yang tersedia secara eksperimental adalah `baca berkas` dengan target eksplisit.

```text
one jalankan main.one --izinkan filesystem.read --target data/contoh.txt
```

Tanpa capability `filesystem.read`, source orchestration ditolak. Target harus sama persis dengan target yang diberi izin.

## Menulis berkas melalui orchestration filesystem — eksperimental

Vertical slice filesystem write saat ini tersedia secara eksperimental pada boundary terbatas. Source One:

```text
tulis ke berkas "data/hasil.txt" dengan "halo dari One"
```

Jalankan dengan permission dan target yang sama:

```text
one jalankan main.one --izinkan filesystem.write --target data/hasil.txt
```

Alias teknis:

```text
one run main.one --allow filesystem.write --target data/hasil.txt
```

Aturan boundary:

- `filesystem.write` harus diberikan secara eksplisit;
- `--target` wajib sama dengan file target yang diminta source;
- payload saat ini satu nilai teks;
- implementasi menulis satu payload ke satu file target;
- tanpa permission, policy menolak orchestration sebelum adapter filesystem dipanggil;
- fitur ini tidak boleh diperlakukan sebagai filesystem management umum atau sandbox/path-confinement guarantee.

Latest exact-SHA executable evidence: `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8`, Termux/Android. Contract check, build, Clippy `-D warnings`, targeted CLI filesystem-write test, seluruh workspace tests, dan offline verification lulus. Belum ada exact-SHA Ubuntu/Windows evidence untuk executable revision ini.

## Menjalankan proses — eksperimental

Source-level process execution yang telah diverifikasi lintas platform:

```text
jalankan proses "rustc"
```

```text
one jalankan main.one --izinkan process.execute --target rustc
```

Boundary saat ini: satu program yang ditargetkan secara eksplisit tanpa argumen tambahan.

## `bangun` dan `package`

Pada v0.1 saat ini:

- `one bangun` menghasilkan intermediate/debug artifact `.oneir` deterministik.
- `one package` menghasilkan `.onepkg` yang tervalidasi dan deterministik.

Keduanya **belum** boleh disebut sebagai native executable compiler atau platform installer.

## Verification boundary

Latest exact-SHA executable evidence for filesystem write/CLI is `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android. The recorded stages are contract check, build, Clippy with `-D warnings`, targeted filesystem-write CLI test, full workspace tests, and offline verification: all PASS.

The latest cross-platform evidence remains independent mirror run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` on Ubuntu + Windows Server 2025. It verifies the process/system boundary, not the later filesystem-write CLI changes.
