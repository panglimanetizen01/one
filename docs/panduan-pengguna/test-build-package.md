# Test, Build, dan Package

## Test

```text
one test nama-project
```

Perintah ini menjalankan program utama project dan fixture conformance lokal yang tersedia secara terurut.

## Build

```text
one bangun nama-project/src/main.one
```

Pada v0.1 hasilnya adalah artifact `.oneir` deterministik. Ini bukan executable native.

## Package

```text
one package nama-project
```

Hasilnya `.onepkg` deterministik setelah project divalidasi. Ini belum merupakan installer platform atau signed distribution package.

## Offline

Operasi parsing, semantic checking, deterministic local tests, dan build/package yang tidak membutuhkan resource eksternal dapat dilakukan offline. Operasi yang membutuhkan network harus menyatakan dependency tersebut.
