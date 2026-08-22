---
title: WeaverTools Technical Documentation
summary: per-crate technical papers for a local-first agent framework whose primary artifact is the trace
version: v0.1
date: 2026-08-22
commit: 0499eba
parent: WeaverTools
---

# WeaverTools Technical Documentation

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027. These pages are published as a dated snapshot of a system under
construction, and each says on its face what it does not yet have.

One paper per crate, a contracts page every paper cites, and an overview holding
what belongs to no crate. A paper reads out a merged document and never decides
anything: where a paper and its source disagree, the paper is the defect.

## The floor

Two crates every domain draws from and no domain contains.

- [weaver-types](weaver-types.md) - the declaration, peer identity, and the wire
  vocabulary loop 0 speaks
- [weaver-traits](weaver-traits.md) - messages, roles, permission modes, and the
  tool surface

## The organs

A crate that governs a domain and holds a two-initiator channel with the harness.

- [weaver-harness](weaver-harness.md) - the switchboard, the loops, and sole
  authorship of the trace
- [weaver-spu](weaver-spu.md) - model residency, two decode engines, and the
  measurement that rides a generation
- [weaver-gate](weaver-gate.md) - the agent's boundary, and the shell as its own
  outbound verb
- [weaver-admin](weaver-admin.md) - lifecycle authorization, boundary
  verification, and custody of the sink

## Under the harness's domain

- [weaver-trace](weaver-trace.md) - the recorder, and the working structure the
  loop reasons over
- [weaver-state](weaver-state.md) - the session custodian, sqlite behind a
  credential-checked socket

## Neither

- [weaver-internal](weaver-internal.md) - callables the loop dispatches inward,
  and never through the gate

## Across all of them

- [Contracts](contracts.md) - every seam, its parties, and its governing document,
  including the two external contracts an outside consumer builds against
