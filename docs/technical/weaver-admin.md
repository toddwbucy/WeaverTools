---
title: weaver-admin
summary: lifecycle authorization, boundary verification, and custody of the sink
version: v0.1
date: 2026-08-22
commit: 0499eba
parent: WeaverTools Technical Documentation
---

# weaver-admin

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-admin-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-admin-PRD` sections 1 and 4. The verbs, the allow-list, the sink, the
rollback.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: one internal, to `weaver-harness`, governed by `weaver-admin-harness-contract`,
tagged **socket**. Plus the operator surface, the second of the two external contracts,
and the systemd contract facing the init system.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `load-unload-path` sections 2 and 3. Six acts of approach, then the enter
fan-out.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: `weaver-admin-PRD` section 3. It creates no principal, carries no turn, and
authors no trace event.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: the status ask refuses today, the init system's three values not mapping
onto four agent states.
