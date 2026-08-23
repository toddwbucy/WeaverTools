---
title: WeaverTools Technical Documentation
summary: per-crate technical papers for a local-first agent framework whose primary artifact is the trace
version: v0.1
date: 2026-08-22
commit: unreleased
parent: WeaverTools
---

# WeaverTools Technical Documentation

**Status:** technical documentation. Describes, decides nothing.

The code described here is unreleased and is scheduled for release in the first
quarter of 2027. These pages are published as a dated snapshot of a system under
construction, and each says on its face what it does not yet have.

One paper per crate, a contracts page every paper cites, two pages for the surfaces
an operator writes against, a page for an instrument that is not built, and an
overview holding what belongs to no crate. A paper
reads out a merged document and never decides anything: where a paper and its
source disagree, the paper is the defect.

This page is the roster. [The introduction](README.md) says what the set is, what
governs it, and where to start depending on what you came for.

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

## The surfaces an operator writes against

Where judgment enters the program rather than travels through it.

- [The loop](loop.md) - loop 0 and loop 1, the seat's eight calls, the one
  crossing, and everything the framework refuses to decide
- [Extending the program](extending.md) - the floor, the contracts, a new organ,
  and the ritual that carries a change to all of them in one act

## Instruments

The built instruments are described where they are built, in the crate papers and
the contracts. This one has its own page because it is unbuilt, and because its
preconditions are the part worth knowing early.

- [The Jacobian lens](jacobian-lens.md) - a per-layer readout of the interior, why
  it captures during the run and reads at analysis, what it would cost, and the two
  questions that gate it

## Across all of them

- [Contracts](contracts.md) - every seam, its parties, and its governing document,
  including the two external contracts an outside consumer builds against
