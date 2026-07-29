# weaver-harness / weaver-gate - contract

**Status:** STUB. Not merged, not ratified, and not a member of the document set.
It exists so the seam has a named home before it has content, and it is written when
`weaver-gate-PRD` is chartered.

**Date filed:** 2026-07-29
**Document ID:** `weaver-harness-gate-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

**This file settles nothing.** It names two parties and the reason they will need an
agreement. Nothing in the corpus may cite this document as having answered a question.

## Why the seam will exist

`weaver-harness-PRD` section 4 names three socket seams, ingress and decode and
coordination, and this is ingress. `WeaverTools-PRD` section 3 has the Gate holding
the only network socket, terminating the external protocol, translating to the
internal wire, and forwarding inward, with the response returning by the same path.
That is a request and a response crossing a process boundary, which is a seam, and a
seam needs a contract.

This stub exists because the other two ingress-class seams have one. Two of three
homes named is a reader counting and finding a gap, and a gap that is deliberate
should say so rather than look like an oversight.

## What is open

Everything. The internal wire format and which crate defines it, the request and
response shapes, how a turn is identified across the boundary, what the Gate may and
may not see of a payload, how backpressure and cancellation are expressed, and the
failure vocabulary.

## What is already decided elsewhere and is not this document's to reopen

The harness binds no listening socket, so the Gate connects inward rather than the
harness listening. A Gate process never outlives the agent it fronts. Supervising
gate lifetime is `weaver-admin`'s and not the harness's. A tool reaching outward is
not ingress and does not cross this seam. Every seam where one crate asks another
process to do something is a `SO_PEERCRED` Unix socket.

## Graph

No records. A seam edge is declared once, by the crate that asks. Which crate asks
here is itself unsettled, since ingress arrives from outside and the direction of the
ask is not the same as the direction of the data. That question is part of writing
this contract rather than something to presume in a stub.
