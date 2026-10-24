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

Tanpa capability `filesystem.read`, source orchestration ditolak. Tanpa opsi `--root`, target harus sama persis dengan target yang diberi izin. Untuk pembatasan direktori, gunakan `--root <dir>` bersama target yang berada di bawah root tersebut.

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
- `--target` wajib cocok dengan file target yang diminta source; jika `--root` diberikan, target juga harus berada di dalam root berdasarkan komponen path;
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

## Menjalankan kueri database — dalam pengembangan

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
- current implementation lineage `0aa4acdf3796235eac432138fec829b6508879e3` belum executable-verified;
- hosted CI run `550` gagal tanpa step yang dieksekusi, sehingga bukan source-failure evidence.

## Mengambil HTTP — dalam pengembangan

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
- current implementation head `913e644ea665711c669590a80d7ff19866cb7671` belum executable-verified;
- latest inspected hosted CI run `585` gagal dengan zero reported steps pada Ubuntu dan Windows, sehingga bukan source-failure evidence.

## `bangun` dan `package`

Pada v0.1 saat ini:

- `one bangun` menghasilkan intermediate/debug artifact `.oneir` deterministik.
- `one package` menghasilkan `.onepkg` yang tervalidasi dan deterministik.

Keduanya **belum** boleh disebut sebagai native executable compiler atau platform installer.

## Verification boundary

Latest exact-SHA executable evidence for filesystem write/CLI is `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` on Termux/Android. The recorded stages are contract check, build, Clippy with `-D warnings`, targeted filesystem-write CLI test, full workspace tests, and offline verification: all PASS.

The latest cross-platform evidence remains independent mirror run `38929072800` against `9038ac75f197753ff1ab6af7478e63e4831903a2` on Ubuntu + Windows Server 2025. It verifies the process/system boundary, not the later filesystem-write, database, or HTTP changes.


## Pembatasan root filesystem

Untuk operasi baca/tulis, `--root <dir>` menambahkan aturan containment berbasis komponen path:

```text
one jalankan main.one --izinkan filesystem.read --root data --target data/input.txt
one jalankan main.one --izinkan filesystem.write --root data --target data/output.txt
```

Target di luar root, path dengan komponen parent `..`, dan ketidakcocokan path absolut/relatif ditolak. Tanpa `--root`, perilaku exact-target lama tetap berlaku. Root scope tidak boleh dipasarkan sebagai sandbox sistem operasi lengkap; proteksi adapter no-follow/reparse-point dan batas sumber daya tetap berlaku.

## Menandatangani dan memverifikasi provenance paket

Perintah rilis terpisah tersedia untuk publisher yang telah memiliki seed privat dan trust store publisher yang dikirim melalui kanal independen:

```text
one sign-package <file.onepkg> --publisher <id> --private-key <seed-hex-file> --source-root <project-dir>
one verify-package <file.onepkg> --trusted-publishers <trust-store> --source-root <project-dir>
```

Jangan menaruh seed privat di source project atau artefak. Untuk alur rilis terlindungi, konfigurasi environment dan langkah operasional, baca [panduan provenance paket rilis](provenance-rilis.md). Paket hasil `one package` sendiri belum otomatis ditandatangani.

## Catatan bukti

Nomor SHA/run yang tercantum pada bagian verification di atas adalah bukti historis untuk revision yang disebut di sana, bukan otomatis bukti untuk branch head terbaru. Untuk status milestone-100 saat ini, gunakan `docs/verification/one-completion-ledger.md` dan `docs/verification/one-milestone-100-session-checkpoint.md`. Candidate belum boleh disebut rilis final sebelum semua gerbang exact-SHA, Android, standard library, keamanan, dokumentasi, dan Definition of Done lulus.
