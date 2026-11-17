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

## Menjalankan kueri database — eksperimental

SQLite-backed database query sudah diimplementasikan, tetapi belum memiliki exact-SHA executable verification. Karena itu pengguna tidak boleh menganggapnya sebagai fitur yang didukung.

Source One:

```text
kueri database ":memory:" dengan "SELECT ?1" dan "42"
```

Permission CLI:

```text
one jalankan main.one --izinkan database.query --target :memory:
```

Boundary implementasi saat ini:

- capability `database.query` wajib diberikan secara eksplisit;
- `--target` adalah resource database dan harus cocok persis;
- adapter menggunakan SQLite nyata dan parameterized SELECT;
- hasil dinormalisasi sebagai teks UTF-8;
- INSERT/UPDATE/DELETE/DDL belum termasuk boundary query ini;
- executable runtime evidence tercatat pada boundary Termux/Android;
- source/core/CLI regression tests juga lulus pada exact-SHA Ubuntu/Windows Server 2025 matrix `39554661451` untuk source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`;
- status tetap eksperimental dan tidak menyatakan dukungan database lintas-platform umum.

## Mengambil HTTP — eksperimental

HTTP GET sudah diimplementasikan sebagai vertical slice terbatas, tetapi belum memiliki exact-SHA executable verification. Karena itu pengguna tidak boleh menganggapnya sebagai fitur yang didukung.

Source One:

```text
ambil dari "http://127.0.0.1:8080/"
```

Permission CLI:

```text
one jalankan main.one --izinkan network.connect --target http://127.0.0.1:8080/
```

Boundary implementasi saat ini:

- capability `network.connect` wajib diberikan secara eksplisit;
- `--target` harus sama persis dengan URL source;
- adapter `network.http.get` menggunakan HTTP/1.1 GET yang sudah tersedia di network runtime;
- hanya skema `http://` yang termasuk slice ini;
- hasil yang dikembalikan adalah response body di bawah ekspektasi UTF-8;
- response non-2xx diperlakukan sebagai adapter failure;
- HTTPS, method selain GET, custom headers/body, redirects, cookies, dan API-specific authentication belum termasuk boundary ini;
- executable runtime evidence tercatat pada boundary Termux/Android;
- source/core/CLI regression tests juga lulus pada exact-SHA Ubuntu/Windows Server 2025 matrix `39554661451` untuk source `77a89a6552c4d4306bd1a97752789b04ae1bbb0c`;
- status tetap eksperimental dan tidak menyatakan dukungan HTTPS, method lain, atau API umum.

## `bangun` dan `package`

Pada v0.1 saat ini:

- `one bangun` menghasilkan intermediate/debug artifact `.oneir` deterministik.
- `one package` menghasilkan `.onepkg` yang tervalidasi dan deterministik.

Keduanya **belum** boleh disebut sebagai native executable compiler atau platform installer.

## Verification boundary

Latest exact-SHA executable evidence for filesystem write/CLI is `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android. The recorded stages are contract check, build, Clippy with `-D warnings`, targeted filesystem-write CLI test, full workspace tests, and offline verification: all PASS.

The latest code-bearing canonical source revision `77a89a6552c4d4306bd1a97752789b04ae1bbb0c` passed independent exact-SHA matrix run `39554661451` on Ubuntu + Windows Server 2025. That matrix covers build, Clippy, workspace tests, real-network runtime tests, and offline verification; it also covers the database-query and HTTP GET source/core/CLI regression suites. Database-query and HTTP GET runtime execution evidence remains bounded to the separately recorded Termux/Android environment. The older process/system run `38929072800` remains historical evidence for its own exact source SHA.
