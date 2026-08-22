---
title: weaver-harness
summary: the switchboard: the loops, the seams, and sole authorship of the trace
version: v0.1
date: 2026-08-22
commit: 0499eba
parent: WeaverTools Technical Documentation
---

# weaver-harness

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-harness-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-harness-PRD` sections 3 through 5. The loop, the ports, the granted
surface, trace authorship.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: five. Boundary, decode, classify, coordination, and state. See the contracts page
for each.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-harness-Spec` section 6, and `basic-inference-loop`. The frame grants
the seat, the turn runs, the close is authored.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: `weaver-harness-Spec`. No loop mints a port, no tool result is fabricated, no
state crosses a residency.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: the idle report is unbuilt, which
`harness-idle-report-authors-without-a-turn` waits on.
