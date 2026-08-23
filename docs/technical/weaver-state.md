---
title: weaver-state
summary: the session custodian: sqlite behind a credential-checked socket, ingest and serve
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-state

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-state-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-state-PRD` sections 1 and 4. The distillate, the two asks, the store.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: one, to `weaver-harness`, governed by `weaver-harness-state-contract`, tagged
**socket**. A member seam rather than an organ channel: one party asks.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-state-Spec` section 4. The election opens the channel, the tee feeds
it, shape and recall answer.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: `weaver-state-PRD` section 2. It does not manage, does not initiate, and is not
the trace.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: four conformance headers against 1,178 lines, and no integration test
directory.
