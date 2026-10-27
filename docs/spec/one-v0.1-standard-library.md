# One v0.1 — Standard Library Boundary

**Status:** Approved normative contract

## 1. Purpose

The standard library provides reusable One-level APIs without becoming a hidden runtime or capability bypass.

## 2. Layering

Standard library modules are built on the language semantic model, CSIR/runtime operations, and explicit capability effects. They do not replace the compiler, parser, or runtime.

## 3. Initial domains

The v0.1 boundary reserves domains for:

- text;
- collections;
- math/basic numeric operations;
- console;
- filesystem;
- process;
- networking;
- time;
- randomness;
- testing utilities.

Only domains whose semantic/effect contracts are already defined may be implemented as stable v0.1 APIs.

## 4. Current deterministic host-helper implementation

The `one_core::stdlib` Rust module currently exposes the following pure helper operations for runtime integration and tests:

- `display_value`: stable display representation of a runtime value.
- `text_length`, `text_contains`, `text_starts_with`, `text_ends_with`, `text_lowercase`, `text_uppercase`, `text_trim`, `text_replace`, `text_split`, and `text_to_integer`: deterministic text operations. Length counts Unicode scalar values, not grapheme clusters; split/replace use literal separators/needles; empty split separators and invalid input types return `None`.
- `list_append`, `list_remove`, `list_length`, `list_get`, `list_join_text`, and `list_sum_numbers`: checked list mutation/access, joining, and integer summation. Append returns a new list; remove deletes only the first equal value and treats a missing value as a deterministic no-op. Negative/out-of-range indexes, non-text joins, non-integer sums, and sum overflow fail.
- `map_get`, `map_set`, `map_remove`, and `map_from_entries`: text-keyed mappings backed by ordered keys for deterministic display.
- `value_to_json` and `json_to_value`: bounded deterministic JSON serialization/parsing for text, integer, boolean, empty, list, and mapping values; JSON floating-point/exponent numbers are rejected because the runtime has no float type.
- `number_abs`, `number_divide`, `number_modulo`, `number_min`, and `number_max`: checked integer helpers; overflow, division by zero, and the signed-integer remainder overflow edge case are rejected.
- `unix_seconds_to_utc_text`: deterministic Gregorian UTC formatting for Unix epoch seconds, returning `YYYY-MM-DDTHH:MM:SSZ`; accepts only integer seconds and years 0000 through 9999. It does not read the host clock or infer local time zones.
- `utc_text_to_unix_seconds`: strict parsing of Gregorian UTC text in `YYYY-MM-DDTHH:MM:SSZ` form. It rejects invalid calendar dates/times, offsets, fractional seconds, leap seconds, and non-canonical forms. It does not read the host clock or infer local time zones.

The source-level built-in `waktu utc dari unix` formats an integer Unix timestamp in UTC and rejects invalid types or dates outside the supported four-digit year range with the standard runtime diagnostic. The source-level built-in `unix dari waktu utc` parses the strict UTC text form back to Unix epoch seconds, with Gregorian calendar validation and the same four-digit year range.

The following subset is now registered as source-level calls in the canonical parser/semantic-analysis/interpreter path: `panjang teks`, `teks memuat`, `teks diawali`, `teks diakhiri`, `huruf kecil`, `huruf besar`, `pangkas teks`, `ganti teks`, `pisah teks`, `ubah teks ke angka`, `panjang daftar`, `tambah elemen daftar`, `hapus elemen daftar`, `gabung teks`, `jumlah daftar angka`, `ambil indeks`, `nilai mutlak`, `bagi bulat`, `angka sisa bagi`, `angka minimum`, `angka maksimum`, `ubah nilai ke json`, `nilai dari json`, `waktu utc dari unix`, and `unix dari waktu utc`. Mapping operations `nilai pemetaan` and `atur pemetaan` are also registered, but currently rely on the opt-in human-core mapping grammar to create a mapping value. Semantic analysis checks their arity and known argument types; runtime errors use code `R3022` for invalid values or failed checked operations. A source-level end-to-end test file exercises this path. The mapping helper functions and `display_value` remain Rust-level helpers rather than source-level built-ins. The list helpers `list_append` and `list_remove` are exposed to source programs through the deterministic `tambah elemen daftar` and `hapus elemen daftar` calls; these calls return new list values and do not mutate their input bindings.

This is a partial integration, not completion of the standard-library gate. Full public API stability, broader stdlib domains, conformance across supported backends, and exact-SHA test evidence remain required before the row can receive verified completion points.

## 5. Capability enforcement

Effectful library calls retain their declared capability requirements. Wrapping `filesystem.read` in a library function cannot bypass or erase that requirement.

## 5. Determinism

Pure functions must be deterministic. Effectful functions explicitly identify external inputs/effects. Time and randomness use controlled providers in deterministic test mode.

## 6. Error model

Library failures use structured One diagnostics/runtime errors. Host exception types are not exposed as the stable API.

## 7. Versioning

Public library modules and functions have stable names and documented signatures. Breaking changes require a versioned language/library contract change.

## 8. Localization

Library semantic APIs are localization-independent. Surface names and documentation may be localized without changing their operation identity.

## 9. Acceptance criteria

- no capability bypass;
- no hidden global mutable state;
- deterministic pure operations;
- explicit effects;
- stable errors and signatures;
- compatibility with reference interpreter and compiler backends.
