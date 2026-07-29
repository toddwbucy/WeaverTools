# weaver-spu - PRD (crate charter)

**Status:** STUB. Not merged, not ratified, and not a member of the document set.
It exists so the composition root has a named home before it has content.

**Date filed:** 2026-07-29
**Document ID:** `weaver-spu-PRD`
**Parent:** `WeaverTools-PRD`
**Editorial:** Per the Working Rules.

**This file settles nothing.** It names a domain and the reason the domain needs a
root. Nothing in the corpus may cite this document as having decided anything.

## The domain

Semantic processing, all of it. Decode serving and encode serving are two jobs of one
domain rather than two domains, and a classifier or a small activation network the
harness calls for effect is a third kind of semantic processing under the same root.
The organizing principle is that these share residency accounting, GPU arbitration,
and lifecycle confirmation, and the harness routes to them while holding none of them.

This is why the root exists ahead of its members. A flat crate list would put decode
serving beside the harness as though the two were peers, and they are not.

## What is already decided elsewhere and is not this document's to reopen

`WeaverTools-PRD` section 6 seats encoder residency here. `weaver-harness-PRD`
section 5 has the harness routing tokens or embeddings and holding no in-process
model of either kind. The encoder is not built in the stateless MVP, because nothing
in stage one retrieves by similarity and an embedder would produce vectors with no
consumer. Ownership is not usage, so naming the domain here builds nothing.

Stating the domain adds no affordance. No trait, no variant, no feature flag, no
config field. A charter naming a domain is a decided boundary. An unbuilt interface
waiting to be filled is what `WeaverTools-PRD` section 9 forbids.

## What is open

The member list and its depth. Whether decode serving and encode serving are separate
member crates or one. Where a classifier and an activation network sit. The residency
and arbitration model. Everything a charter exists to state.

## Graph

No records. This document declares no node and no edge. A stub that declared a crate
node would put a resolvable record in the graph for a crate with no charter behind it.
