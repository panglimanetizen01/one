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
| `teks diawali` | 2 | Boolean | Literal prefix test |
| `teks diakhiri` | 2 | Boolean | Literal suffix test |
| `huruf kecil` | 1 | Text | Unicode lowercase mapping |
| `huruf besar` | 1 | Text | Unicode uppercase mapping |
| `pangkas teks` | 1 | Text | Trim Unicode whitespace at both ends |
| `ganti teks` | 3 | Text | Replace all literal, non-empty occurrences |
| `pisah teks` | 2 | List | Split on a literal, non-empty separator |
| `ubah teks ke angka` | 1 | Number | Parse a base-10 signed 64-bit integer; whitespace and overflow are rejected |
| `waktu utc dari unix` | 1 | Text | Format integer Unix epoch seconds as strict Gregorian UTC `YYYY-MM-DDTHH:MM:SSZ`; years 0000–9999 are supported |
| `unix dari waktu utc` | 1 | Number | Parse strict Gregorian UTC `YYYY-MM-DDTHH:MM:SSZ`; invalid dates/times, offsets, fractional seconds, leap seconds, and non-canonical forms are rejected |
| `panjang daftar` | 1 | Number | List element count |
| `tambah elemen daftar` | 2 | List | Return a new list with the value appended; non-list receivers fail closed with `E2043` or `R3022` when statically unknown |
| `hapus elemen daftar` | 2 | List | Return a list with only the first equal value removed; a missing value is a deterministic no-op; non-list receivers fail closed with `E2043` or `R3022` when statically unknown |
| `gabung teks` | 2 | Text | Join a list containing only text values with a literal separator |
| `jumlah daftar angka` | 1 | Number | Checked sum of integer values; overflow and non-integer items are errors |
| `ambil indeks` | 2 | Any | Read a list element using a zero-based integer index |
| `nilai mutlak` | 1 | Number | Checked integer absolute value |
| `bagi bulat` | 2 | Number | Checked signed integer division, truncating toward zero |
| `angka sisa bagi` | 2 | Number | Checked signed-integer remainder; divisor zero and `i64::MIN` remainder `-1` are errors |
| `angka minimum` | 2 | Number | Minimum of two integers |
| `angka maksimum` | 2 | Number | Maximum of two integers |
| `nilai pemetaan` | 2 | Any | Read a text-keyed mapping entry; missing keys are errors |
| `ubah nilai ke json` | 1 | Text | Deterministic JSON serialization with ordered map keys; nesting deeper than 128 levels or output larger than 16 MiB is rejected |
| `nilai dari json` | 1 | Any | Parse JSON text, supporting strings, signed integers, booleans, null, arrays, and objects; duplicate keys, floats/exponents, invalid escapes, and excessive nesting are rejected |
| `atur pemetaan` | 3 | Map | Return a mapping with the specified text key inserted or replaced |

Argument count is checked during semantic analysis and again at runtime for directly constructed IR. Known argument types are checked statically when possible; a known mismatch produces diagnostic `E2043`. Invalid runtime values, failed conversions, missing mapping keys, invalid indexes, and division by zero produce runtime error `R3022`. Checked arithmetic overflow produces `R3023`. These errors must not be converted to guessed values.

Mapping values are currently constructed through the explicit, opt-in human-core mapping grammar. The public Rust helper APIs and source-level call surface are a partial library slice; they do not imply that filesystem, time, networking, serialization, concurrency, or every other reserved standard-library domain is complete.

## Capability-sensitive operations

Filesystem, process, network, and native operations are not implicitly available. They require explicit capability contracts.

## Stability rule

A standard-library operation is supported only when its behavior, error model, platform behavior, and tests are documented in this repository.


## Public Rust helper API contract

The reusable helper surface in `one_core::stdlib` is public and independently testable. It complements the source-level call table above; it is not a claim that every planned standard-library domain is implemented.

| Rust API | Accepted values / behavior | Failure contract |
|---|---|---|
| `display_value` | Stable display representation of any runtime `Value` | Infallible |
| `list_append`, `list_remove` | Append any value; remove only the first equal value; removing a missing value is a no-op | Return `None` for a non-list receiver |
| `text_length`, `text_contains`, `text_starts_with`, `text_ends_with`, `text_trim`, `text_replace`, `text_split`, `text_to_integer`, `text_lowercase`, `text_uppercase` | Unicode-scalar length, literal substring/prefix/suffix, Unicode whitespace trimming/case mapping, literal replacement/splitting, signed i64 decimal conversion | Return `None` for wrong types, empty replace/split needles, invalid decimal input, or overflow |
| `list_length`, `list_get`, `list_join_text`, `list_sum_numbers` | Count values; zero-based indexed read; join text-only lists; checked sum of integer-only lists | Return `None` for wrong types, negative/out-of-range indices, non-text join members, non-integer sum members, or sum overflow |
| `map_get`, `map_set`, `map_from_entries` | Text-keyed lookup/update and deterministic BTreeMap construction | Return `None` for wrong receiver/key types; missing-key behavior at the source-level builtin is runtime diagnostic `R3022` |
| `number_abs`, `number_divide`, `number_modulo`, `number_min`, `number_max` | Signed i64 operations with checked absolute value, division, and remainder | Return `None` for wrong types, zero divisors, and signed overflow such as `i64::MIN / -1` |
| `value_to_json`, `json_to_value` | Deterministic JSON for text, i64, booleans, null, arrays, and objects; object keys sort lexicographically | Return `None` for unsupported values/types, malformed JSON, duplicate keys, invalid escapes/surrogates, float/exponent numbers, depth over 128, or input/output over 16 MiB |
| `unix_seconds_to_utc_text`, `utc_text_to_unix_seconds` | Strict Gregorian UTC conversion for years 0000–9999 | Return `None` for wrong types, invalid dates/times, noncanonical timestamps, and values outside the supported year range |

The source-level interpreter maps invalid built-in arguments to stable diagnostics: semantic type mismatch `E2043`, invalid runtime input/conversion/index/key/division to `R3022`, checked arithmetic overflow to `R3023`, and invalid arity to `E2018`. The Rust helper layer itself returns `Option` for recoverable invalid-input cases; callers must not replace `None` with guessed values.

### E1 executable acceptance matrix

The focused suite `cargo --locked test -p one-core --test stdlib_builtins` exercises source-level calls through parse → semantic analysis → IR → interpreter. `cargo --locked test -p one-core --lib` covers library unit tests. The full workspace test, warnings-denied Clippy, and `tools/check_contract.py` complete the E1 lab. The tests cover valid/invalid types, empty text/lists, Unicode scalar behavior, i64 boundaries and overflow, negative/out-of-range indices, deterministic map/JSON order, duplicate/malformed JSON and invalid surrogates, JSON nesting, UTC leap days/year boundaries/noncanonical forms, and stable repeated diagnostics. Direct Rust API tests additionally cover reusable list mutation helpers and deterministic map construction.

A workflow start is not acceptance evidence: the E1 row earns its fixed 10 points only when the complete required suite succeeds on the same recorded candidate SHA and the ledger/checkpoint are synchronized.
