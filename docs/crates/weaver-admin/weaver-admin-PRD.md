# weaver-admin - PRD (crate charter)

**Status:** STUB. Not merged, not ratified, and not a member of the document set.
It exists so the composition root has a named home before it has content.

**Date filed:** 2026-07-29
**Document ID:** `weaver-admin-PRD`
**Parent:** `WeaverTools-PRD`
**Editorial:** Per the Working Rules.

**This file settles nothing.** It names a domain and the reason the domain needs a
root. Nothing in the corpus may cite this document as having decided anything.

## The domain

Agent administration, all of it. Lifecycle is the part already seated here, and it is
seated here because supervising worker and gate lifetimes is long-lived and
fleet-wide while a harness is mortal and dies with its agent. The backend tooling
that reads a finished record belongs here for the same reason: it is work about an
agent rather than work the agent does.

The tooling is the reason this is a root rather than a crate. A durable record is a
file, and once written, what reads it is downstream of the recorder and creates no
seam with `weaver-trace`. That leaves a family of readers, exporters, and audit tools
with one organizing principle and no natural home in a flat list.

**The scope line runs through this root and must be held.** Some of what looks like
administration is framework instrument and belongs to WeaverTools. Some is
deployment-track derivation and does not. A root that collects everything related to
administration will collect both, and the charter has to draw that line rather than
inherit it.

## What is already decided elsewhere and is not this document's to reopen

`weaver-admin` opens a session's record while the worker still holds that principal
and passes the descriptor with `SCM_RIGHTS`, so the agent uid never resolves a trace
path. It reads the durable record directly and the agent has no path to it.
`weaver-trace-PRD` section 4.2 gives it the trigger for the per-turn integrity check
and the frequency that goes with it. `WeaverTools-PRD` section 6 seats lifecycle
intent and custody of the boundary here.

The apex still seats lifecycle orchestration on `weaver-harness` in its own prose, and
that is a filed correction against the apex rather than a position this document
inherits.

## What is open

The member list. The binary layout, since the CLI and the daemon both run as
`weaver-admin` and one crate with two binaries versus two crates is a
code-organization call rather than a security one. The GID mask at lifecycle
boundaries. Whether an admin-side database ever becomes a trace consumer. The
workflows, beginning with load and unload. Everything a charter exists to state.

## Graph

No records. This document declares no node and no edge.
