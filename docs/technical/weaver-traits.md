---
title: weaver-traits
summary: the floor's vocabulary: messages, roles, permission modes, and the tool surface
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-traits

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-traits-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-traits-Spec`. The message and role shapes, the three permission modes,
the tool trait.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: none. Floor, as above.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-traits-Spec`, the definition sites the contracts draw from.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: the provider module is documentation with no code, deliberately.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: `tool-trait` is drawn by no vocabulary clause and waits on the tool
workflow.
