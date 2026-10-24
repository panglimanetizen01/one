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

Exact-SHA cross-platform evidence on candidate `1dec26abcc90ac27d892cf3ff0a83dedb8b90fd8`: [mirror run `40084595236`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40084595236) passed Ubuntu and Windows Server 2025 workspace tests, including `filesystem_write_requires_explicit_capability_and_writes_target`, `run_command_executes_filesystem_write_from_one_source`, final-symlink denial, and bounded-write regressions. The earlier Termux/Android result at `a7aa94cb1f8ef778dc9c1a24e61cf714622567c8` remains historical. This is still an experimental, explicitly target-scoped write slice—not a general filesystem manager or sandbox guarantee.

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

SQLite-backed database query tersedia sebagai vertical slice terbatas; jangan menyamakannya dengan API persistence umum.

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
- adapter menggunakan SQLite nyata dan parameterized `SELECT`;
- hasil dinormalisasi sebagai teks UTF-8;
- INSERT/UPDATE/DELETE/DDL belum termasuk boundary query ini;
- query, parameter, row/output, VM-opcode, dan waktu eksekusi dibatasi;
- final-component symlink diblokir oleh `SQLITE_OPEN_NOFOLLOW`, dengan regresi Unix;
- exact-SHA cross-platform run [`40084595236`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40084595236) menjalankan `database_vertical_slice` dan `database_orchestration` pada Ubuntu + Windows; focused run [`40084595238`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40084595238) lulus batas sumber daya SQLite dan symlink regression.

## Mengambil HTTP — eksperimental

HTTP GET tersedia sebagai vertical slice terbatas, bukan HTTP client/API SDK umum.

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
- adapter `network.http.get` menggunakan HTTP/1.1 GET;
- hanya skema `http://` yang termasuk slice ini;
- response body dinormalisasi ke UTF-8 dan dibatasi;
- response non-2xx diperlakukan sebagai adapter failure;
- HTTPS, method selain GET, custom headers/body, redirects, cookies, dan API-specific authentication belum termasuk boundary ini;
- exact-SHA cross-platform run [`40084595236`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40084595236) menjalankan `http_get_vertical_slice` dan `http_get_orchestration` pada Ubuntu + Windows; real-network tests juga lulus pada kedua platform.

## `bangun` dan `package`

Pada v0.1 saat ini:

- `one bangun` menghasilkan intermediate/debug artifact `.oneir` deterministik.
- `one package` menghasilkan `.onepkg` yang tervalidasi dan deterministik.

Keduanya **belum** boleh disebut sebagai native executable compiler atau platform installer.

## Verification boundary

The current candidate evidence is [cross-platform run `40084595236`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40084595236), which checks out source SHA `1dec26abcc90ac27d892cf3ff0a83dedb8b90fd8` and passes Ubuntu + Windows Server 2025 workspace tests, CLI vertical slices, real-network tests, offline verification, and bundle generation. Focused standard-library/database/provenance regressions pass in [run `40084595238`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40084595238); Ubuntu 24.04 and Debian 12 pass in [run `40084595259`](https://github.com/panglimanetizen01/one-windows-verification/actions/runs/40084595259).

These links are exact evidence for the SHA named in the logs, not automatic evidence for a later documentation commit. If source changes, rerun the required matrix on the new SHA. The canonical repository's Actions jobs have also failed before executable steps with empty `steps: []`; those are infrastructure failures, not source-test passes or failures.

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
