---
title: weaver-gate
summary: the agent's boundary, and the shell as the crate's own outbound verb
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-gate

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-gate-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-gate-PRD` sections 1 and 2. Two doors out, one door in, and the deny
rule that excludes the agent's own uid.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](../contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: one internal, to `weaver-harness`, governed by `weaver-harness-gate-contract`,
tagged **socket**. Plus the world-facing surface, one of the two external contracts.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-gate-Spec` section 3. Accept, authenticate by credential, relay unread,
and supervise the shell.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: `weaver-gate-PRD`. It does not parse the line, holds no work state, and admits
no name as a reason.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: the agent-opened socket. Chartered for registered applications, and no
exchange reaches it.
