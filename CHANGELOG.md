# Changelog

## Unreleased — One v0.1

- Added formal lexical and concrete grammar contracts.
- Added Rust 2024 workspace and Linux/Windows CI matrix.
- Added deterministic lexer with indentation/newline tokens.
- Added AST and Indonesian-first parser for core constructs.
- Added initial semantic analysis and type model.
- Added language-independent Semantic IR.
- Added reference interpreter with step limits.
- Added structured diagnostics and source spans.
- Added explicit capability model.
- Added cross-platform process interoperability primitive and regression coverage for denied capabilities and preserved process results.
- Added core standard-library primitives.
- Added deterministic multi-word name boundary enforcement and end-to-end core pipeline conformance coverage.
- Added the complete v0.1 CLI command surface: conversational Indonesian commands plus technical aliases, testing, packaging, version, and help.
- Added strict v0.1 project manifest validation for required project identity fields.
- Added canonical deterministic `.oneir` and `.onepkg` serialization.
- Added project-local deterministic conformance fixture execution to `one test`.
- Added core pipeline, function/event, security, project, and CLI integration tests.
- Added executable conformance fixtures and golden output contracts.
- Added Linux/Windows offline verification scripts so verification does not depend on purchasing hosted CI capacity.
- Added deterministic artifact checks to both offline verification paths.
- Added a manual v0.1 release-check workflow.
- Recorded the current GitHub-hosted runner blocker without claiming CI-green status.
