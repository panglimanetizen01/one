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
- reject backend/target runtime mismatch;
- produce deterministic diagnostics;
- preserve semantic IR independently of target selection.

The RED-phase test update was pushed first at `7f7dd37fbac31575187e7c1959a6c7b92d01ccbd`. The pull-request CI runner then reported failure before executing any job step (`steps: []` on both matrix jobs), so that infrastructure result is not treated as executable RED evidence.

## Phase 3 — GREEN

Implement the smallest target/backend abstractions needed by the RED tests.

Implemented on `feat/target-backend-foundation`:

- `TargetBackend` is an explicit backend contract trait rather than a concrete backend singleton;
- `ReferenceTargetBackend` is the reference implementation;
- target metadata includes deterministic `RuntimeClass` derived from `TargetEnvironment`;
- backend declarations expose identity, required runtime, architecture support, profile support, and environment support;
- validation rejects backend identity, runtime, architecture, profile, and environment mismatches deterministically;
- target metadata remains independent from One Semantic IR.

Constraints:

- preserve current hosted behavior;
- avoid adding concrete hardware support claims;
- keep target metadata independent from semantic IR meaning;
- use explicit enums/typed identifiers rather than free-form strings where practical;
- keep diagnostics deterministic and testable;
- do not add device APIs in this slice.

## Phase 4 — Verification

Required verification:

- focused target/backend tests;
- full workspace tests;
- `cargo build --workspace --all-targets`;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- deterministic artifact tests;
- existing vertical-slice tests;
- contract checker;
- exact-SHA verification for the resulting commit.

A prior full verification existed at the pre-audit head `83cd765c918f5ac5bcb23c5480fc1d768def06ee`. After the audit changes, fresh executable verification is still required. GitHub Actions has so far created PR runs at post-change heads but the matrix jobs terminated before executing any step (`steps: []`), so those runs are not evidence of code failure or success.

## Phase 5 — Documentation

Documentation is reconciled with the implementation:

- the specification describes the `TargetBackend` trait and `ReferenceTargetBackend` implementation;
- runtime class metadata and its deterministic environment mapping are explicit;
- required-runtime compatibility is part of backend validation;
- the specification keeps concrete hardware support unverified;
- this plan records the CI runner limitation without converting it into a false test result.

## Phase 6 — Integration

Commit the minimal implementation and documentation changes on `feat/target-backend-foundation`. Do not force-push or rewrite history. Review the diff against canonical `main` before proposing merge.

## Next independent slice

After this foundation is independently verified, evaluate cross-compilation/toolchain abstraction as the next dependency layer. Device I/O and concrete MCU/RTOS integrations remain downstream work.
