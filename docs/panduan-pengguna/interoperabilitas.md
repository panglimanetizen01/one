# Interoperabilitas

One dirancang untuk bekerja dengan ecosystem lain, tetapi status setiap bentuk interoperability harus mengikuti implementation evidence.

## Saat ini

Process interoperability sudah menjadi bagian dari executable core dan memiliki contract/tests. Ini berbeda dari native language bindings.

## Belum otomatis tersedia

Native C/C++, Python, Node.js, dan Java/JVM bindings tercatat sebagai future pada implementation inventory dan tidak boleh dipakai sebagai klaim dukungan saat ini.

## Prinsip

Adapter atau tool eksternal adalah backend. Semantic model One tetap menjadi sumber makna; external tool tidak menjadi semantic authority.

Jika binding baru selesai, dokumentasi harus diperbarui bersama contract, tests, implementation status, dan verification evidence.


## Process execution containment boundary — Issue #105 — 20 November 2026

The current process adapter enforces a 10-second direct-child deadline and 1 MiB capture limit per output stream. On Unix, stdout/stderr readers are nonblocking and have a 100 ms shutdown grace (20 ms idle grace); this prevents an escaped pipe writer from keeping the caller blocked indefinitely. However, cleanup still kills only the original process group. A descendant that calls `setsid()` or creates a new process group can survive after the invocation returns. Bounded reader shutdown is **not** process-tree containment or a sandbox.

Windows uses a Job Object assigned during process creation and terminates remaining job members during cleanup. The cross-platform guarantee is therefore not identical.

Evidence boundary: SEC-05 run #44566187162 passed on its pinned source `722d662...` for the bounded timeout/output contract, but does not prove escaped-session cleanup. On candidate source `c7f460a4e38be61de635ad5af9f7ff987a941f01`, the Windows SEC-05 job #135776430245 is SUCCESS while the Linux job #135776429279 remains IN_PROGRESS at native cross-compiler installation. Issue #105 remains OPEN; no process-tree containment claim is made.


## Batas keamanan process interoperability — Issue #105

Process interoperability memiliki batas waktu 10 detik untuk proses langsung dan batas output 1 MiB per stream. Pada Unix, reader stdout/stderr menggunakan nonblocking read dan shutdown grace 100 ms agar program One tidak tertahan tanpa batas jika pipe masih terbuka.

Namun, penghentian Unix saat ini hanya membunuh process group awal. Descendant yang membuat session/process group baru dapat tetap hidup setelah perintah selesai. Jadi fitur ini **bukan sandbox process-tree** dan tidak boleh dipakai sebagai jaminan isolasi proses yang tidak tepercaya. Pada Windows, Job Object dipasang saat proses dibuat dan anggota job dihentikan saat cleanup.

Issue #105 masih OPEN untuk menyelesaikan containment descendant Unix atau mempersempit kontrak dukungan secara eksplisit.

## Pembaruan status filesystem — 20 November 2026

- Pemeriksaan independen pada source `fb5c8bc66a5cbdf8a6670d9d1e9dbe0caa631a20` telah lulus untuk regresi hard-link di Linux dan Windows.
- File yang memiliki hard link ke luar project root ditolak untuk operasi baca dan tulis; pengujian memeriksa bahwa isi file di dalam maupun di luar root tidak berubah.
- Windows juga telah lulus pengujian junction/reparse-point, target absolut di dalam root, penolakan target di luar root tanpa side effect, atomic write, batas ukuran, dan parent traversal.
- Linux telah lulus pengujian no-follow, mount-boundary, traversal, race pada parent symlink, serta direct-child write.
- Status dukungan tetap terbatas: filesystem backend tersedia hanya pada Linux dan Windows. Android dan macOS belum memiliki filesystem adapter. Pada Linux, nested writes tetap ditolak secara fail-closed.
- Ini merupakan bukti regresi dan verifikasi teknis pada kandidat PR, bukan penerimaan rilis. Issue #104 tetap OPEN / NOT ACCEPTED; jangan menganggap fitur sebagai sandbox universal atau mengaktifkan dukungan platform yang belum tersedia.
- CircleCI `ci/circleci: one-linux` pada source yang sama berstatus SUCCESS. Bukti ini tidak otomatis berlaku untuk SHA berikutnya.
