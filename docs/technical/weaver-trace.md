---
title: weaver-trace
summary: the recorder, and the in-RAM working structure the loop reasons over
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-trace

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-trace-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-trace-PRD` sections 2 and 3. The closed kind set, the flattened
envelope, the two clocks, the working structure.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: one, to `weaver-harness`, governed by `weaver-harness-trace-contract`, tagged
**link** rather than socket because no process line is crossed.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-trace-Spec` sections 2 and 5. Submit, admit, assign the sequence,
render once, fan out to structure and writer.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: `weaver-trace-PRD` section 4.1. No path-taking write surface, no recording
level, no filter, no mutation surface on the structure.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: nothing crate-local. The kind set stands at twenty-one: `model.field`
arrived 2026-08-21, `elision` and `refusal` on 2026-08-22, and the Spec's variant,
rename, and ordinal counts all stand with it.