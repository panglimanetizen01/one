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

## Current implementation boundary: list helpers

The implementation in `crates/one-core/src/stdlib.rs` currently has these concrete helper semantics. These are implementation observations, not a claim that every surface-language edge case has been independently verified:

- `display_value(value)` delegates to `Value::display()`.
- `list_append(list, value)` appends one value and returns the resulting list when the input is a list; it returns `None` for a non-list input.
- `list_remove(list, value)` removes only the first equal element when one exists, returns the resulting list even when no equal element exists, and returns `None` for a non-list input.
- The helpers do not themselves define user-facing diagnostics for `None`; callers own that behavior. Do not infer a diagnostic code or message from these helper signatures.

Compatibility guard: changing these helper return semantics, equality behavior, duplicate handling, or diagnostic mapping requires explicit contract updates and regression tests before implementation changes. Do not add new standard-library operations based only on these helpers.

## Capability-sensitive operations

Filesystem, process, network, and native operations are not implicitly available. They require explicit capability contracts.

## Stability rule

A standard-library operation is supported only when its behavior, error model, platform behavior, and tests are documented in this repository. Implementation presence alone is not sufficient evidence of support.
