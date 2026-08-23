---
title: weaver-internal
summary: callables the loop dispatches inward, and never through the gate
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools Technical Documentation
---

# weaver-internal

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

## What it is

Not drafted. One paragraph of purpose, and what this crate is not. The IS-NOT
half is load-bearing: a reader arriving from another agent framework brings
assumptions this section spends early.

Sources: `weaver-internal-PRD` section 1, and its section stating what the crate is not.

## What it owns

Not drafted. The objects the crate holds and the invariants it keeps.

Sources: `weaver-internal-PRD` section 1. The promotion space and its first member, the
calculator.

## Seams

Not drafted. Each seam named once, its contract linked to
[the contracts page](contracts.md), and its socket-or-link tag with the grounds
stated. The contract is linked and never restated.

Seams: none. It is linked into the worker and holds no channel with the harness.

## How it works

Not drafted. One pass through the crate's primary operation, in enough detail to
follow. Dense prose unless the operation is a true sequence.

Sources: `weaver-internal-Spec`. One function, called by the loop, answering
deterministically.

## What it refuses

Not drafted. The refusals the design encodes, each with its ground. A refusal
without a ground reads as an omission.

Sources: `weaver-internal-PRD`. Membership is an operator decision and never a
contributor convenience.

## What is not built

Not drafted. This crate's slice of the overview page's appendix B, each entry
named to the work or the measurement that would close it.

Known today: its classification. It fails the organ test and the submodule definition
does not reach it.
