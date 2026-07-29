# weaver-harness / weaver-admin - contract

**Status:** STUB. Not merged, not ratified, and not a member of the document set.
It exists so the seam has a named home before it has content, and it is the first
document written when `weaver-admin-PRD` is chartered.

**Date filed:** 2026-07-29
**Document ID:** `weaver-harness-admin-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

**This file settles nothing.** It names two parties and the reason they will need an
agreement. Every question a contract exists to answer is open, and nothing in the
corpus may cite this document as having answered one.

## Why the seam will exist

`weaver-trace-PRD` section 4.2 puts a hash over each turn's events into both the
working structure and the durable record, and gives the trigger to `weaver-admin`
rather than to the harness. The recorder produces the value and never judges it, and
the harness answers a request for it and never schedules one. That leaves a request
crossing from `weaver-admin` to a running worker, which is a seam, and a seam needs a
contract.

`weaver-admin` is also the lifecycle party. Ordered load and unload, readiness
collection, and rollback of a partial transition are its work, and the harness
reports into that rather than driving it. Whether lifecycle and integrity ride one
agreement or two is itself unsettled, and this file does not presume one contract per
crate pair is the answer before the pair has been examined.

## What is open

Everything. Named to keep the list from being rediscovered rather than to bound it:
what admin asks for and in what shape, what a mismatch obliges either party to do,
how a request reaches a worker that binds no listening socket, whether the coordination
socket already carries it, whether lifecycle and integrity are one seam or two, and
what the failure vocabulary is.

## What is already decided elsewhere and is not this document's to reopen

The harness binds no listening socket. The agent uid never resolves a trace path. The
recorder holds no policy. The harness answers rather than schedules. Each of those is
carried by a charter already written, and this contract will be built on top of them
rather than around them.

## Graph

No records. A seam edge is declared once, by the crate that asks, and `weaver-admin`
is the crate that asks. Declaring the edge here would put a record in the graph on
another crate's behalf and would give a clean resolve to a seam that does not exist
yet, which is the failure `WeaverTools-PRD` section 11 names when it says a clean
automated gate can be evidence the gate did not fire.
