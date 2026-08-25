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

**The directories carry the boundary.** `weaver-agents/` holds the agent domain:
nine crate papers and the two surfaces an operator writes against. `consumers/`
holds what sits outside that boundary and reaches an agent only across a contract.
The contracts page stays at the top because it is the seam itself, and the two
external contracts on it are where that boundary is actually drawn.

A paper reads out a merged document and never decides anything: where a paper and
its source disagree, the paper is the defect.

This page is the roster. [The introduction](README.md) says what the set is, what
governs it, and where to start depending on what you came for.

## The floor

Two crates every domain draws from and no domain contains.

- [weaver-types](weaver-agents/weaver-types.md) - the declaration, peer identity, and
  the wire vocabulary loop 0 speaks
- [weaver-traits](weaver-agents/weaver-traits.md) - messages, roles, permission modes,
  and the tool surface

## The organs

A crate that governs a domain and holds a two-initiator channel with the harness.

- [weaver-harness](weaver-agents/weaver-harness.md) - the switchboard, the loops, and
  sole authorship of the trace
- [weaver-spu](weaver-agents/weaver-spu.md) - model residency, two decode engines, and
  the measurement that rides a generation
- [weaver-gate](weaver-agents/weaver-gate.md) - the agent's boundary, and the shell as
  its own outbound verb
- [weaver-admin](weaver-agents/weaver-admin.md) - lifecycle authorization, boundary
  verification, and custody of the sink

## Under the harness's domain

- [weaver-trace](weaver-agents/weaver-trace.md) - the recorder, and the working
  structure the loop reasons over
- [weaver-state](weaver-agents/weaver-state.md) - the session custodian, sqlite behind a
  credential-checked socket

## Neither

- [weaver-internal](weaver-agents/weaver-internal.md) - callables the loop dispatches
  inward, and never through the gate

## The surfaces written against

Where judgment enters the program rather than travels through it. **The loop is
written per turn and takes effect on the next crossing. Extending the floor is a
build-time change** and is grouped here because both are places a person decides
something rather than places the framework does.

- [The loop](weaver-agents/loop.md) - loop 0 and loop 1, the seat's eight calls, the one
  crossing, and everything the framework refuses to decide
- [Extending the program](weaver-agents/extending.md) - the floor, the seams, a new
  organ, and where the framework's requirements stop

## Consumers

Outside the agent boundary, reaching an agent only across the two external
contracts. Instruments built inside the boundary are described where they are
built, in the crate papers. What is here either reaches in from outside or is
noted here for what it proves.

- [weaver-web](consumers/weaver-web.md) - the first consumer: the channel, the
  lifecycle view, and the live trace view, and the working proof that a consumer
  needs the two contracts and nothing else
- [The Jacobian lens](consumers/jacobian-lens.md) - a per-layer readout of the interior,
  why it captures during the run and reads at analysis, what it would cost, and the two
  questions that gate it
- [The calculator](consumers/calculator.md) - **not a consumer**, noted here because
  the perturbation mechanic a diagnostic consumer would need is already built and
  running in production, and this is what built it

## Across all of them

- [Contracts](contracts.md) - every seam, its parties, and its governing document,
  including the two external contracts an outside consumer builds against
