# Implementation Plan — One Target Backend Foundation

## Goal

Implement the minimum target/backend contract required to prepare One for cross-compilation and constrained/firmware targets without introducing device-specific language semantics.

## Phase 1 — Repository baseline

- Verify branch starts from canonical `main` SHA `62a605c35786f3feaf1853adf640beb1d7744846`.
- Run the existing exact-SHA verification before modifications.
- Confirm the working tree is clean.

## Phase 2 — RED

Add focused tests for the target model and backend boundary before production implementation.

Required failing behaviors:

- construct a valid hosted target;
- construct a constrained/firmware target descriptor;
- reject unsupported architecture;
- reject unsupported execution profile;
- reject mismatched backend/target declarations;
- produce deterministic diagnostics;
- preserve semantic IR independently of target selection.

Run only the focused tests and record the expected failures.

## Phase 3 — GREEN

Implement the smallest target/backend abstractions needed by the RED tests.

Constraints:

- preserve current hosted behavior;
- avoid adding concrete hardware support claims;
- keep target metadata independent from semantic IR meaning;
- use explicit enums/typed identifiers rather than free-form strings where practical;
- keep diagnostics deterministic and testable;
- do not add device APIs in this slice.

## Phase 4 — Verification

Run:

- focused target/backend tests;
- full workspace tests;
- `cargo build --workspace --all-targets`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- deterministic artifact tests;
- existing vertical-slice tests;
- contract checker.

Then run the exact-SHA verification script for the resulting commit.

## Phase 5 — Documentation

Update only documentation whose claims are proven by the implementation and tests. Explicitly keep concrete hardware support as unverified until a real target has executable evidence.

## Phase 6 — Integration

Commit the minimal implementation and documentation changes on `feat/target-backend-foundation`. Do not force-push or rewrite history. Review the diff against canonical `main` before proposing merge.

## Next independent slice

After this foundation is verified, evaluate cross-compilation/toolchain abstraction as the next dependency layer. Device I/O and concrete MCU/RTOS integrations remain downstream work.
