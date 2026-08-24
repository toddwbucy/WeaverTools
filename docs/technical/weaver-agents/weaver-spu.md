---
title: weaver-spu
summary: model residency, two decode engines, and the measurement that rides a generation
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-spu

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-spu-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-spu-PRD` sections 2 and 4. Residency, the hot KV cache, admission as
the one check on the device.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](../contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: three, all to `weaver-harness`: residency, decode, and classify. Each has its own
contract.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-spu-PRD` section 4.1, the five admit steps, and `weaver-spu-Spec`
sections 4 and 6.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: `weaver-spu-PRD` section 10 and the residency contract section 5. Admission
refuses and never evicts.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: the GGUF readout tap. The pin exists and nothing drives it, so
`spu-two-taps-one-shape` closed 2026-08-22 when the GGUF tap landed carrying it.
