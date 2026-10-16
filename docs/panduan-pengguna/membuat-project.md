# Membuat Project

Gunakan:

```text
one buat nama-project
```

CLI membuat skeleton:

```text
nama-project/
  one.toml
  src/
    main.one
```

Perintah `one buat` tidak menimpa folder yang sudah ada.

## Alur kerja

```text
one buat nama-project
one periksa nama-project/src/main.one
one jalankan nama-project/src/main.one
one test nama-project
```

Gunakan `one package nama-project` bila membutuhkan artifact package v0.1.
