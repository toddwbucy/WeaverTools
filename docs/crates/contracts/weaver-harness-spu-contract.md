# weaver-harness / weaver-spu - contract

**Status:** STUB. Not merged, not ratified, and not a member of the document set.
It exists so the seam has a named home before it has content, and it is written when
`weaver-spu-PRD` is chartered.

**Date filed:** 2026-07-29
**Document ID:** `weaver-harness-spu-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

**This file settles nothing.** It names two parties and the reason they will need an
agreement. Nothing in the corpus may cite this document as having answered a question.

## Why the seam will exist

`WeaverTools-PRD` section 3 has the harness issuing a decode request over the decode
socket, carrying `turn_key` and `session_key` so the SPU can parent its own spans to
the turn, and the SPU returning the generation together with a measurement payload
tagged with the key it was given. That is a request and a response crossing a process
boundary, which is a seam, and a seam needs a contract.

One contract covers the pair. Encoding and decoding are one domain under
`weaver-spu-PRD`, and the harness routes rather than holding either, so this is one
seam rather than two. The previous tree carried a decode-serving contract and an
embedding-serving contract as separate agreements because encode and decode were
treated as different kinds of thing, and collapsing them is a consequence of that
domain ruling rather than a naming preference.

## What is open

Everything. Named to keep the list from being rediscovered rather than to bound it:
the request and response shapes, whether tokens or embeddings or both cross, how the
KV cache flush decision is expressed given that the harness owns it and the SPU owns
the cache, the residency and arbitration vocabulary, lifecycle confirmation during
load and unload, and the failure vocabulary.

## What is already decided elsewhere and is not this document's to reopen

The harness owns the flush decision and the SPU owns the cache. The harness holds no
in-process model. The trace is authored by the harness alone, so the SPU reports and
does not emit. Every seam where one crate asks another process to do something is a
`SO_PEERCRED` Unix socket.

## Graph

No records. A seam edge is declared once, by the crate that asks. The harness is the
crate that asks here, and it will declare the edge in its own charter when this
contract exists to name in the `via` field. Declaring it now would give a clean
resolve to a seam with no contract behind it.
