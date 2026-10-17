# One Constitution

**Status:** Accepted project constitution. Implementation status is governed by executable evidence, not by this document alone.

## 1. Identity

One is a programming language created in Indonesia, starting with Bahasa Indonesia, designed for global use.

> **Dibuat di Indonesia, dimulai dengan bahasa Indonesia, dibangun untuk dunia.**

One is intended to become a real general-purpose programming language and toolchain rather than a chat interface, command alias layer, or language-specific wrapper around an LLM.

## 2. Vision

One should make programming approachable to ordinary people while retaining the semantic rigor, performance options, security model, interoperability, and engineering depth required for serious software.

The surface language should be understandable like a person giving a clear instruction to another person. The underlying language must remain formal, deterministic, testable, and machine-executable.

One's general-purpose scope includes not only desktop, server, web, automation, data, networking, and developer software, but also software for robots, embedded controllers, and electronic appliances such as televisions, refrigerators, air conditioners, fans, cooling/refrigeration systems, and other programmable devices.

## 3. Mission

Build a human-oriented, general-purpose programming language that:

- starts with Bahasa Indonesia;
- can be localized to other human languages without changing semantic meaning;
- works across operating systems, Linux distributions, terminals, editors, IDEs, containers, and devices;
- can scale from small scripts to large software systems;
- can target robots, embedded systems, controllers, and programmable electronic appliances where an appropriate backend/runtime exists;
- interoperates with existing languages and ecosystems;
- remains independent of any LLM for program meaning and execution;
- provides explicit security and capability governance;
- gives beginners a progressive path to advanced programming;
- is practical for global professional use.

## 4. Human-language programming

One's conversational surface is bounded by formal grammar. Examples such as `tampilkan nama`, `kalau umur sudah 17 tahun`, or `untuk setiap teman di daftar teman` are valid only when covered by a language contract.

Natural language is a user interface to formal semantics, not an invitation to guess intent.

Ambiguity is rejected or diagnosed deterministically. An LLM may assist a developer or user outside the execution authority boundary, but it cannot silently invent program meaning.

## 5. Core constructs

The official core construct families are:

1. DATA
2. ACTION
3. CONDITION
4. LOOP
5. FUNCTION
6. EVENT

These are language concepts, not merely UI labels.

## 6. Semantic architecture

The authoritative pipeline is:

```text
Localized Source
    -> Localized Lexer
    -> Parser
    -> AST
    -> Name/Scope Analysis
    -> Type/Semantic Analysis
    -> One Semantic IR
    -> Reference Interpreter / Compiler Backend
    -> Runtime / Platform
```

External orchestration must consume this semantic pipeline rather than create a parallel execution authority.

## 7. Determinism

For the same defined program, inputs, environment contract, and permitted capabilities, One must have deterministic language semantics and deterministic diagnostics where the language contract requires them.

External nondeterminism must be explicit in the runtime/effect model rather than hidden by natural-language interpretation.

## 8. LLM independence

One programs must remain executable without an LLM. The compiler, interpreter, runtime, standard library, package system, diagnostics, and capability governance are authoritative. AI assistance is optional tooling.

## 9. General-purpose scope

One is not restricted to one domain. Its architecture must be able to support application development, systems work, automation, web/API work, data and scientific workloads, developer tooling, networking, security engineering, media/graphics, device integration, robotics, embedded systems, appliance control, and other legitimate computing domains.

The device and embedded scope includes software that may execute on or control robots, microcontrollers, embedded computers, industrial/device controllers, and programmable household or consumer electronics such as TVs, refrigerators, AC units, fans, air-cooling/refrigeration equipment, and similar appliances.

This is a direction and architectural requirement, not a claim that every capability already exists.

## 10. Cross-platform and cross-environment portability

One must not be architecturally dependent on a particular:

- operating system;
- Linux distribution;
- shell;
- terminal application;
- editor;
- IDE;
- Android application;
- container runtime;
- CPU architecture where a backend can reasonably support it;
- microcontroller, embedded board, robot platform, or appliance vendor.

Initial first-class platform targets remain Windows and Linux. Linux compatibility must include a path for major distributions such as Ubuntu, Debian, Kali, Alpine, Arch, Fedora, and other distributions through portable binaries/toolchains or supported build paths.

Android environments such as Termux and Linux-container environments such as UserLAnd are integration targets, not definitions of One.

Embedded and device targets must be handled through platform abstractions and backends rather than by changing One's core semantics for each device vendor.

## 11. Terminal independence

One must be invocable from real technical terminals and shells without becoming a shell language itself. Bash, Zsh, Fish, POSIX-style shells, BusyBox environments, SSH sessions, CI runners, and other supported terminal hosts are integration surfaces.

The One CLI must provide stable machine-facing interfaces so host shells can launch One without changing One's semantics.

## 12. Editor and IDE independence

One must be usable from editors and IDEs through stable tooling contracts rather than editor-specific language semantics.

The ecosystem target includes VS Code, Acode, SuperCode, terminal editors, and future editors/IDEs. Integration mechanisms may include CLI, formatter, language server, diagnostics protocol, debugger protocol, project metadata, and build/test interfaces.

## 13. Android and constrained environments

One should be able to run in constrained environments when the target runtime/toolchain can be built or packaged for them. Termux, UserLAnd, Pydroid-hosted workflows, and similar environments are compatibility targets where technically appropriate.

Unsupported host limitations must be reported as environment/toolchain constraints, not disguised as language semantics.

Embedded targets may impose tighter constraints on memory, storage, CPU, real-time behavior, power, peripherals, boot process, and available operating systems/RTOS. These constraints belong in the backend/runtime/platform contract while preserving one language semantic model.

## 14. Interoperability

One must not become a closed island. Stable interfaces should allow integration with existing technologies including C/C++, Python, Java/JVM, JavaScript/Node.js, HTML/CSS/web systems, native libraries, operating-system APIs, embedded/RTOS APIs, microcontroller SDKs, hardware abstraction layers, robot middleware, appliance/device protocols, and external command/tool ecosystems where security and semantic contracts permit.

Interop must preserve explicit boundaries and must not bypass One's governance model.

## 15. Localization

Bahasa Indonesia is the initial language. The architecture must support English, Japanese, Chinese, Spanish, Arabic, and other human languages through localized lexing/parsing and shared semantic identity.

Localization must preserve:

- semantic capability identity;
- effects;
- authorization requirements;
- scope rules;
- result types;
- diagnostics meaning;
- deterministic execution semantics.

Changing human language must not silently change authority.

## 16. Security and capability governance

External effects require explicit capability and scope contracts. Security-sensitive operations must have machine-readable identity, privilege requirements, target/resource scope, policy decisions, adapter identity, and audit semantics as applicable.

One may be used for legitimate security engineering, defensive automation, vulnerability research, authorized penetration testing, and security tooling. The language itself remains general-purpose; security policy belongs in the governance plane and host environment.

Device and robotic control must also respect safety, authorization, scope, and platform-specific constraints. The existence of a device-control API does not imply permission to operate a physical system.

## 17. Progressive complexity

Beginners should be able to write simple instructions without learning the entire runtime model. Advanced users must be able to access explicit parameters, types, capabilities, effects, platform controls, modules, packages, and interop without abandoning the language.

Simple source and expert source should converge on the same semantic model.

## 18. Architecture invariants

The following are non-negotiable unless an explicit architectural decision supersedes them:

- Formal deterministic grammar.
- Semantic-first architecture.
- Common semantic model/IR.
- Localization designed from the beginning.
- LLM-independent execution.
- Explicit capability and scope governance for external effects.
- Human-readable diagnostics.
- General-purpose design, including device, embedded, and robotics domains.
- Cross-platform architecture.
- Deep interoperability.
- Security by explicit boundaries rather than hidden privilege.
- Progressive complexity.
- Device/platform differences must not fork core language semantics.

## 19. Development law

```text
DISCOVER -> EXECUTE -> VERIFY -> FIX -> REVERIFY -> DOCUMENT -> CONTINUE
```

No claim of completion without evidence.

## 20. Definition of done

A language capability is not Supported merely because source code exists. The normal lifecycle is:

```text
Specified
 -> Designed
 -> Implemented
 -> Tested
 -> Verified
 -> Documented
 -> Supported
```

Evidence must identify the relevant source revision and environment boundary.

## 21. Evidence discipline

Current execution evidence takes precedence over historical evidence. A failed hosted workflow with no executable job steps does not prove a Rust source failure. Offline verification is valid only when the environment, toolchain, command, exact revision, and result are recorded.

## 22. Originality framing

One's originality is not asserted as absolute historical novelty of every individual idea. The project distinguishes prior art from its intended combination: conversational surface syntax, formal deterministic grammar, shared semantic IR, native localization, progressive complexity, general-purpose capability, cross-platform execution, deep interoperability, and explicit governance.

## 23. Change control

Accepted architectural decisions belong in `docs/DECISIONS.md`. New behavior must not silently contradict the constitution. If the constitution must change, the change itself must be documented as an explicit architectural decision and verified against existing source/tests.
