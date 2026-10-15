# One v0.1 — CLI Contract

**Status:** Draft normative contract for review

## 1. Purpose

The One CLI is the stable command-line entry point for project creation, checking, running, testing, building, packaging, and diagnostics.

## 2. Core commands

The v0.1 command surface reserves:

- `one new` — create a project skeleton;
- `one check` — validate source/contracts without executing the program;
- `one run` — execute through the selected runtime;
- `one test` — run project/conformance tests;
- `one build` — build a target artifact;
- `one package` — produce distributable package output;
- `one version` — report tool/language versions;
- `one help` — explain supported commands/options.

Exact flags are implementation details until the CLI syntax contract is finalized.

## 3. Exit status

The CLI must expose deterministic machine-readable success/failure status. A zero status means the requested command completed successfully according to its command contract. Non-zero statuses identify failure categories through structured diagnostics/output.

## 4. Output modes

Human-readable output is localized. A machine-readable mode must provide stable diagnostic codes, severity, source spans, and structured fields without relying on prose parsing.

## 5. Project discovery

Commands operate on an explicit project root or deterministic current-project discovery rule. The CLI must not search arbitrary parent/sibling directories unpredictably.

## 6. Capability policy

`run` and other effectful commands load explicit project/runtime capability policy. The CLI must not silently grant capabilities merely because a host operation is requested.

## 7. Offline behavior

`check`, parser/semantic tests, and other commands that do not require external resources must work offline. Commands requiring network effects must report the dependency explicitly.

## 8. Determinism

Command behavior must not depend on terminal history, shell aliases, editor state, filesystem enumeration order, or LLM output.

## 9. Acceptance criteria

- stable core command names;
- clear separation between check/run/test/build/package;
- deterministic exit status;
- structured diagnostics;
- explicit capability policy;
- offline validation for source-only operations;
- Windows/Linux support through the platform abstraction contract.