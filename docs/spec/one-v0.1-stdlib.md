# One v0.1 Standard Library Contract

## Deterministic core

The v0.1 reference interpreter provides these initial operations:

- `tampilkan nilai` — append a value to program output.
- `sapa nama` — append a deterministic greeting.
- `buat daftar nama` — create an empty list in the current scope.
- `tambahkan nilai ke daftar` — append a value.
- `hapus nilai dari daftar` — remove the first equal value when present.
- arithmetic expressions using `ditambah`, `dikurangi`, `dikali`, and `dibagi`.
- comparison phrases `sama dengan`, `lebih besar dari`, `lebih kecil dari`, `sudah`, and `belum`.

## Capability-sensitive operations

Filesystem, process, network, and native operations are not implicitly available. They require explicit capability contracts.

## Stability rule

A standard-library operation is supported only when its behavior, error model, platform behavior, and tests are documented in this repository.
