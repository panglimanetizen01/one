# MASTER CONTINUATION PROMPT — PROJECT ONE

**CONTINUOUS EXECUTION · NOTION-SKILL-FIRST · IMPLEMENTATION-FIRST · GITHUB-FIRST · EXACT-SHA · EVIDENCE-FIRST · ZERO-REWORK · FULL-DOCUMENTATION-SYNC · NO-PREMATURE-STOP**

Lanjutkan Project ONE dari repository dan dokumentasi persisten. Fokus utama adalah menyelesaikan implementasi produk v0.1 sampai seluruh scope resmi benar-benar tersedia. Jangan biarkan pekerjaan berputar pada audit/tes berulang sementara fitur produk belum selesai. Baca sumber otoritatif, lalu implementasikan backlog nyata di branch/PR yang tepat.

## Identitas dan satu work queue

- Main repository: https://github.com/panglimanetizen01/one
- Independent verifier: https://github.com/panglimanetizen01/one-windows-verification
- Notion Skill: https://app.notion.com/p/407a9d23afaa813cb942d4d6a4d1c5ce
- Active PR terakhir diketahui: ONE #113 dan verifier #51. Refresh HEAD/base/state sebelum bertindak.
- Satu-satunya work queue: [ONE_RELEASE_BLOCKER_REGISTER.md](ONE_RELEASE_BLOCKER_REGISTER.md). Jangan membuat register/roadmap/evidence matrix/prompt duplikat.

## Wajib baca sebelum bertindak

1. Fetch dan baca Notion Skill; checkpoint terbaru mengalahkan snapshot lama, tetapi jangan hapus sejarah.
2. Baca `AGENTS.md`, `README.md`, `docs/README.md`, konstitusi, requirements, decisions, development protocol, serta domain contract relevan.
3. Baca [release contract](../release/one-v0.1.md), [completion roadmap](../superpowers/plans/2026-10-15-one-v0-1-completion-addendum.md), fixed ledger/DoD, [evidence matrix](one-v0.1-evidence-matrix.md), implementation status, verification index, dan live checkpoint.
4. Baca blocker register ini.
5. Periksa diagram yang sudah ada: full architecture map di PR #97 dan Mermaid source yang relevan. Jangan menggambar ulang/mengunggah duplikat.
6. Refresh GitHub PR/issue/branch/commit/status/workflow dan CircleCI pipeline mapping. Jangan percaya SHA/status lama tanpa readback.

## Cara kerja wajib

- Jalankan alur **DISCOVER → IMPLEMENT → INTEGRATE → DOCUMENT → FINAL VERIFY → RELEASE**.
- Prioritas utama: implementasi production code yang masih hilang pada Issue #104 dan backlog v0.1 yang diwajibkan kontrak. B-001 classifier/CircleCI sudah memiliki exact-SHA terminal success pada `979973cd...`; jangan mengulang investigasi itu tanpa failure baru yang relevan.
- Tambahkan regression tests bersama implementasi, tetapi tunda full matrix dan audit berulang sampai scope implementasi selesai. Jalankan pemeriksaan terarah lebih awal hanya bila diperlukan untuk mencegah perubahan security-sensitive yang tidak aman.
- Bedakan FACT, HYPOTHESIS, ROOT CAUSE, HISTORICAL, dan UNKNOWN.
- Untuk CircleCI, hubungkan job → workflow → pipeline → `.vcs.revision`; pastikan job anggota workflow. `--job-id` saja bukan bukti exact SHA. Jika `started_at=null`, jangan menyebutnya test timeout/failure.
- Semua klaim PASS harus mencantumkan exact source SHA, verifier SHA, platform, workflow/job, dan terminal result. Bukti parent SHA tidak berlaku ke child SHA.
- Pertahankan bukti yang sudah valid; jangan mengulang tes/audit tanpa perubahan input atau alasan teknis.
- Gunakan perubahan minimal dan test yang membuktikan akar masalah. Jangan mengubah kode security-sensitive tanpa build/test runner nyata.
- Setelah perubahan, sinkronkan dokumen yang terdampak dalam ekosistem existing: specs, roadmap/addendum, implementation status, evidence matrix, verification/live checkpoint, user guide/status fitur, diagram Mermaid, changelog, docs map, dan Notion Skill. Pertahankan sejarah dan cross-link; jangan membuat dokumen tandingan.
- Jangan berhenti setelah analisis jika masih ada tindakan aman yang dapat dieksekusi. Jika terhalang provider log/runner/akses, sebutkan bukti spesifik yang hilang dan minta hanya output redacted yang diperlukan.

## Batas keamanan dan otorisasi

Tanpa izin eksplisit, jangan merge PR, ubah `main`, force-push, hapus branch, publish/tag release, ubah protected verifier branch/pin, ubah fixed ledger/DoD/acceptance criteria, melemahkan security boundary, mengklaim platform support tanpa bukti, atau membeli plan/runner/credit. Jangan meminta/menampilkan token atau secret, termasuk `CIRCLECI_TOKEN`.

## Definisi selesai

ONE belum selesai/siap rilis sampai seluruh gate resmi release contract dan ledger terpenuhi pada satu frozen release-candidate SHA, required platform matrix terminal, security acceptance terbukti, dokumentasi sinkron, dan owner menyetujui tindakan rilis. Verifier pass parsial atau dokumentasi lengkap bukan release approval.

**Mulai sekarang:** baca Skill dan dokumen wajib, refresh exact state, buka blocker register, lalu lanjutkan implementasi produk pada blocker P0. Jangan kembali menjadikan B-001 atau pengulangan tes sebagai pekerjaan utama.
