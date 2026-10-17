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
- embedded and RTOS APIs
- microcontroller SDKs
- hardware abstraction layers
- robot middleware and device-control protocols
- electronic appliance firmware/device interfaces

## Boundary types

Interop may use:

- FFI/ABI;
- process execution;
- generated source/artifacts;
- stable APIs;
- protocol clients;
- shared libraries;
- platform APIs;
- package/module interfaces;
- embedded/RTOS SDK interfaces;
- hardware abstraction layers;
- device buses/protocols and vendor interfaces where permitted.

The selected mechanism must be explicit and documented.

## Device and robotics rule

One should be able to participate in software stacks that control or communicate with physical systems, including robots, embedded controllers, and programmable household electronics such as TVs, refrigerators, AC units, fans, cooling/refrigeration equipment, and similar devices.

Device-specific implementation belongs in backends, runtimes, adapters, SDK bindings, or generated artifacts. The device vendor, board, chipset, firmware, or RTOS must not redefine One's core language semantics.

A device integration is not automatically a support claim. The target must be tested and verified at the relevant revision before it is documented as Supported.

## Semantic rule

Interop is an implementation boundary, not a license to change One's semantics. External systems must be represented through typed contracts where practical.

## Security rule

Interop must respect capability, scope, policy, and audit requirements. Calling a native library or external command must not bypass governance.

Physical-device and robotic control must additionally respect authorization and safety constraints imposed by the host system and target environment.

## Toolchain rule

The One compiler/runtime should expose stable interfaces so existing build systems, CI, package systems, editors, IDEs, embedded toolchains, and device-development workflows can invoke One without embedding One-specific assumptions into the host.

## Progressive adoption

Users should be able to introduce One into an existing project incrementally rather than requiring an all-at-once rewrite.
