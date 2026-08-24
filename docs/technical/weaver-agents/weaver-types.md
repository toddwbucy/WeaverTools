---
title: weaver-types
summary: the floor's shapes: the agent declaration, peer identity, and the wire vocabulary loop 0 speaks
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-types

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-types-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-types-Spec` sections 2 and 4. The declaration's fields, the loop-0 wire
enums, the access rule.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](../contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: none. It is floor, drawn by every domain and asking nothing of any of them.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-types-Spec` section 2, the declaration parse.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: the config parse's refusals, and the elections a declaration may not set.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: the floor moves whenever an election is chartered. Five landed since
2026-08-14.
