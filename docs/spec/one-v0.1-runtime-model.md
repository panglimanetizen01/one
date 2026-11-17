# One v0.1 — Runtime Model

**Status:** Draft normative contract for review

## 1. Purpose

This document defines the execution model for validated Common Semantic IR (CSIR). It is platform-neutral and does not define a particular VM, native ABI, bytecode format, or operating-system API.

## 2. Runtime invariants

1. Only valid CSIR may execute.
2. Execution is deterministic for deterministic programs and identical runtime inputs.
3. Runtime behavior is defined by CSIR semantics, not source spelling or localization.
4. Runtime does not parse source text.
5. Runtime does not perform name resolution or type inference.
6. Runtime does not call an LLM or network service for semantic decisions.
7. Effects are checked through the capability contract before the effect occurs.
8. Runtime errors are structured values/events, not undefined host-language behavior.

## 3. Execution state

A runtime instance contains:

- program/module state;
- call stack;
- lexical environment frames;
- value store;
- control-flow position;
- pending effect request, if any;
- event registrations, when events are enabled;
- runtime configuration and granted capabilities.

Each function invocation creates a frame containing parameters, locals, and return state. Frames are destroyed when invocation completes or terminates with an uncaught runtime error.

## 4. Values

Runtime values correspond to the v0.1 type contract:

`Nothing`, `Boolean`, `Integer`, `Text`, `List<T>`, function value, and event-related runtime values where explicitly defined.

Values are typed; runtime must reject an invalid operation rather than reinterpret a value using host-language coercion.

## 5. Evaluation

Expressions are evaluated in the order specified by the semantic model and represented by CSIR. Operands are evaluated left-to-right unless a later normative contract explicitly defines another order.

A failed expression evaluation produces a structured runtime error and does not silently continue with an invented value.

## 6. Calls

A function call:

1. evaluates the function reference;
2. evaluates arguments in defined order;
3. validates arity and runtime representation;
4. creates a new frame;
5. executes the function body;
6. returns the result or structured error.

Recursive calls are permitted unless a later resource-limit contract restricts them.

## 7. Control flow

Runtime executes CSIR control-flow operations without reparsing source. Conditionals select exactly one branch. Loops repeat according to their CSIR condition/iteration semantics. A return exits the current function invocation.

## 8. State mutation

Mutation occurs only through CSIR operations that explicitly permit mutation. An operation that fails must not leave a partially applied semantic mutation unless the operation contract explicitly defines transactional partial effects.

## 9. Effects

Filesystem, process, network, environment, native, UI, and other host interactions are effects. An effect request crosses the capability boundary before host execution. Denied effects produce a structured capability error and do not execute.

## 10. Runtime errors

Minimum categories:

- invalid operation;
- invalid value/state;
- division by zero;
- out-of-range access;
- missing callable target;
- failed assertion/contract;
- capability denied;
- external effect failure;
- resource exhaustion;
- unhandled event/function error.

Each error carries a stable code, operation context, source/provenance information when available, and a deterministic message key.

## 11. Events

Event registration is runtime state. An event handler executes only when a matching event is delivered through the defined event interface. Event delivery must not bypass capability checks. Ordering is deterministic where the event source defines an ordering; otherwise the runtime must expose the nondeterministic boundary rather than fabricate ordering guarantees.

## 12. Resource limits

The runtime may enforce explicit limits for stack depth, memory, execution steps, open resources, and effect payload sizes. Limits are configuration, not semantic coercion. Exceeding a limit produces a structured resource error. The process adapter additionally bounds the direct-child deadline and stdout/stderr capture. On Unix, process-group cleanup is escapable by descendants that create a new session or group; reader shutdown is bounded, but escaped descendants may survive. Strong Unix process-tree containment remains an open security requirement (Issue #105). Windows uses Job Object containment.

## 13. Host isolation

Host-specific APIs are not directly exposed as arbitrary runtime escape hatches. They are mediated by effect/capability adapters. Platform adapters may differ internally but must preserve CSIR-visible semantics.

## 14. Determinism

Given identical CSIR, runtime version, localization-independent semantic inputs, capability grants, deterministic external inputs, and resource configuration, execution must produce identical observable semantic results.

The runtime must not use hash iteration order, filesystem enumeration order, wall-clock timing, process scheduling, machine-local history, or LLM output as hidden semantic input.

## 15. Reference behavior

The reference interpreter is the executable semantic oracle for v0.1. Other backends are conforming only when their observable results and defined errors agree with the reference interpreter for the conformance suite.

## 16. Implementation boundary

This contract does not define garbage-collection algorithm, bytecode encoding, register allocation, JIT strategy, native ABI, threading implementation, or platform-specific storage. Those are backend implementation details unless they change observable CSIR semantics.

## 17. Acceptance criteria

- valid CSIR executes;
- invalid CSIR is rejected before execution;
- type-invalid operations cannot silently execute;
- effect requests pass through capability enforcement;
- deterministic tests are reproducible;
- structured runtime errors preserve provenance;
- reference interpreter and future compiler backends share the same observable semantics.