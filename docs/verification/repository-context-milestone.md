# Repository Context Milestone — Session Continuity

**Status:** Accepted and recorded.

This document explains why the repository contains a persistent project contract.

## Problem

Chat sessions are not a reliable long-term project memory. A new session may not have the complete prior conversation and can otherwise repeat discovery, reopen settled decisions, or make incompatible assumptions.

## Solution

The One repository is the persistent project memory for accepted direction. `AGENTS.md` is the entry point for agents. `docs/ONE_CONSTITUTION.md` contains the project constitution. Supporting contracts hold requirements, architecture, compatibility, security, localization, interoperability, decisions, and development protocol.

## Rule for new sessions

A new development session must inspect the repository contract before planning or changing the project. It must use current source/tests and verification evidence to determine implementation status.

## What belongs here

Persist:

- vision and mission;
- target users;
- language philosophy;
- architecture invariants;
- accepted requirements;
- compatibility targets;
- security principles;
- localization rules;
- interoperability goals;
- accepted architectural decisions;
- development/evidence protocol;
- verified status and evidence boundaries.

Do not persist private conversation secrets or treat raw chat transcripts as authoritative source code documentation.

## Desired outcome

A future session should be able to enter the repository, read the contract, inspect the current implementation, and continue work in the correct direction without reconstructing the entire history of previous conversations.
