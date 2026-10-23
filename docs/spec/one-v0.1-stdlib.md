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


## Source-level built-in call subset (milestone-100 workstream)

The following calls use the existing deterministic expression-call syntax and canonical semantic IR. They do not invoke an LLM or delegate expression meaning to a host language.

| Call name | Arity | Result | Contract |
|---|---:|---|---|
| `panjang teks` | 1 | Number | Unicode scalar-value count; not bytes or grapheme clusters |
| `teks memuat` | 2 | Boolean | Literal substring search |
| `pangkas teks` | 1 | Text | Trim Unicode whitespace at both ends |
| `ganti teks` | 3 | Text | Replace all literal, non-empty occurrences |
| `pisah teks` | 2 | List | Split on a literal, non-empty separator |
| `ubah teks ke angka` | 1 | Number | Parse a base-10 signed 64-bit integer; whitespace and overflow are rejected |
| `panjang daftar` | 1 | Number | List element count |
| `ambil indeks` | 2 | Any | Read a list element using a zero-based integer index |
| `nilai mutlak` | 1 | Number | Checked integer absolute value |
| `bagi bulat` | 2 | Number | Checked signed integer division, truncating toward zero |
| `angka minimum` | 2 | Number | Minimum of two integers |
| `angka maksimum` | 2 | Number | Maximum of two integers |
| `nilai pemetaan` | 2 | Any | Read a text-keyed mapping entry; missing keys are errors |
| `atur pemetaan` | 3 | Map | Return a mapping with the specified text key inserted or replaced |

Argument count is checked during semantic analysis and again at runtime for directly constructed IR. Known argument types are checked statically when possible; a known mismatch produces diagnostic `E2043`. Invalid runtime values, failed conversions, missing mapping keys, invalid indexes, and division by zero produce runtime error `R3022`. Checked arithmetic overflow produces `R3023`. These errors must not be converted to guessed values.

Mapping values are currently constructed through the explicit, opt-in human-core mapping grammar. The public Rust helper APIs and source-level call surface are a partial library slice; they do not imply that filesystem, time, networking, serialization, concurrency, or every other reserved standard-library domain is complete.

## Capability-sensitive operations

Filesystem, process, network, and native operations are not implicitly available. They require explicit capability contracts.

## Stability rule

A standard-library operation is supported only when its behavior, error model, platform behavior, and tests are documented in this repository.
