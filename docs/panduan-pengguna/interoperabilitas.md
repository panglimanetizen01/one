# Interoperabilitas

One dirancang untuk bekerja dengan ecosystem lain, tetapi status setiap bentuk interoperability harus mengikuti implementation evidence.

## Saat ini

Process interoperability sudah menjadi bagian dari executable core dan memiliki contract/tests. Ini berbeda dari native language bindings.

## Belum otomatis tersedia

Native C/C++, Python, Node.js, dan Java/JVM bindings tercatat sebagai future pada implementation inventory dan tidak boleh dipakai sebagai klaim dukungan saat ini.

## Prinsip

Adapter atau tool eksternal adalah backend. Semantic model One tetap menjadi sumber makna; external tool tidak menjadi semantic authority.

Jika binding baru selesai, dokumentasi harus diperbarui bersama contract, tests, implementation status, dan verification evidence.
