# One Interoperability Contract

## Principle

One must integrate with the existing technical ecosystem instead of becoming an isolated language island.

## Target ecosystems

The interoperability roadmap includes:

- C and C++
- Python
- Java/JVM
- JavaScript/Node.js
- HTML/CSS/web tooling
- operating-system APIs
- native libraries
- external processes and CLI tools
- databases
- network services
- cloud/platform APIs

## Boundary types

Interop may use:

- FFI/ABI;
- process execution;
- generated source/artifacts;
- stable APIs;
- protocol clients;
- shared libraries;
- platform APIs;
- package/module interfaces.

The selected mechanism must be explicit and documented.

## Semantic rule

Interop is an implementation boundary, not a license to change One's semantics. External systems must be represented through typed contracts where practical.

## Security rule

Interop must respect capability, scope, policy, and audit requirements. Calling a native library or external command must not bypass governance.

## Toolchain rule

The One compiler/runtime should expose stable interfaces so existing build systems, CI, package systems, editors, and IDEs can invoke One without embedding One-specific assumptions into the host.

## Progressive adoption

Users should be able to introduce One into an existing project incrementally rather than requiring an all-at-once rewrite.
