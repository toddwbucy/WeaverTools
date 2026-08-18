# weaver-state - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth.

**Date filed:** 2026-08-18
**Document ID:** `weaver-state-PRD`
**Parent:** `weaver-harness-PRD`
**Companion contract:** `weaver-harness-state-contract`, owed by the act that opens
the seam and named here so the seam cannot open without it.
**Editorial:** Per the Working Rules.

---

## 1. What this crate is

`weaver-state` is the custodian of the agent's working state. It stores what it
is handed, organizes what it stores, and serves what it is asked for, and it
does nothing else. The charter is the operator's ruling of 2026-08-18, and the
sentence is short enough to carry whole: the crate holds state, and the
management of state as it concerns context for the decoder belongs to the
harness and its control loops.

**Two functions and no third: ingest and serve.** The tee's distillate flows
in and is kept, and what is kept answers asks. The two are not symmetric
today, and the asymmetry is load-bearing for the build order. The ingest has
its producer standing: the trace renders canonical events now, the tee's
election is ruled, and the stream this crate drinks from is real traffic on a
real agent. The serve has no consumer yet: its first asker is the control
loop that injects context for the model, and that loop is unwritten. So the
ingest half builds whole against its living producer, and the serve half's
shape waits for the loop's act, per the reserved-slot rule: a query surface
elected against a guess about its caller is an interface-shaped empty joint,
and the fault report's own history is the precedent, chartered and unshaped
rather than absent until its first real traffic gave it a shape.

This is the statefulness leg returning by the door apex section 9 built for it.
Proto-stateful was the deliverable: real state within a session and none across,
with the memory leg out entirely and its return chartered in advance as a schema
extension plus a new socket and contract, never as a retrofit. This document is
that return's first paper. Nothing in the base set moves to make room for it,
which is what the door was for.

```graph
node: weaver-state
kind: crate

edge: parent
from: weaver-state
to: weaver-harness
```

## 2. What it is not

**It does not manage.** The harness's control loops decide what enters the
decoder's context, what leaves it, when a flush is worth its cost, and what any
stored fact is worth to the turn at hand. Every one of those is policy, every
one is the harness's, and this crate answers asks without holding an opinion
about why it was asked. The trace charter drew this line for recording and it
holds here for keeping: custody without policy is the whole of the charter, and
a member that judged its own contents would be a second reasoning loop wearing a
filing cabinet's name.

**It does not initiate.** Nothing in this crate fires on a condition, watches a
threshold, or acts unasked. A control loop that consults state is the operator's
code in the harness's seat, per the tool boundary ruling's placement of control
loops, and this crate is a place such a loop reaches rather than a place one
lives.

**It is not the trace and does not compete with it.** The trace is the primary
artifact and the one authoritative record, per apex section 1, and what this
crate holds is a distillation of the record, never stored back into it, per the
ruling carried at `weaver-trace-PRD` section 3.2. Where the two disagree the
trace is right by construction, because the trace is the account and state is a
working derivative of the account. The distillation surface is the harness's
tee over the canonical event stream, ruled by the operator 2026-08-12, and its
output is this crate's ingress: state receives what the tee elects, holds it
organized, and answers with it.

**It is not the model's to reach.** The harness reads state for its own
assembly and its loops' decisions, and the model receives only what the harness
hands it as rendered context, the same wall `weaver-harness-PRD` section 5
holds for the trace. There is no model-facing read path and no tool that opens
one.

## 3. The seam

The seam is the apex's own prescription read literally: a schema extension plus
a new socket and contract. State crosses a process line from the harness under
`weaver-harness-state-contract`, initiator first in the name because the
harness asks and state answers, on a Unix socket that authenticates its peer
per the first invariant. Two kinds of traffic and no third: the tee's
distillate flowing in as the harness applies its filter, and served answers
flowing back when the harness or a control loop in its seat asks. Both ride the
one seam, because both are the harness talking to its member and a second
channel would be a topology fact no need has produced.

**The socket is the seam's own and reuses nothing.** Apex section 9 says a new
socket and this charter reads it literally: the coordination seam keeps its
one kind of traffic, and state's seam is a second, distinct channel with a
name of its own. It stands at load under the same coordination that stands
every organ channel, and it authenticates its peer by credential, per the
first invariant's rule for a channel with a name. Which end binds, how the
descriptor travels, and the credential's exact judgment are the contract's
mechanics, deliberately absent here.

The nesting under `weaver-harness` carries domain membership and nothing else,
per apex section 5.4: nesting is never process topology, and this member
crossing a process line while the trace links in-process is two right answers
to two different questions. The trace must be unreachable by the agent and is
written by exactly one party, so it lives inside that party. State serves reads
back toward its writer's loops and holds volume the harness's own process
should not carry, so it stands beside the harness rather than inside it.

**Within a session, and not across, and a session spans its runs.** This
crate's holdings live and die with the session that produced them, which is
proto-stateful's boundary honored rather than crossed - and the session, not
the run, is the boundary, per the operator's ruling of 2026-08-18. The trace
file already persists across load and unload cycles within one session, and
the state file persists beside it the same way: an unload retires a run and
leaves the holdings standing, the next load of the same session reopens them,
and the close of the session is what retires them. Persistence across
sessions is a second return through the same apex door with its own paper,
and nothing here lays in for it: no export surface a future act would wish
existed, per the no-reserved-slots rule. The prohibition is on outliving the
session, not on encoding: the seam's traffic serializes the way every seam's
does, and refusing that would refuse the socket itself.

**Losing the member loses the derivative and never the account.** State can
die while the session lives, and the session goes on: the trace is the
authoritative record, this crate holds a working derivative of it, and a
harness whose state member is gone serves turns the way it did before the leg
existed. The holdings are rebuildable by construction, because everything they
distilled is still in the record, and whether a restarted member is refilled
by replaying the tee or stands empty is the harness's policy like every other
judgment. What this charter forbids is the inversion: no design in this leg
may make the session's continuation depend on the derivative surviving.

## 4. Its material

The input is the inspected artifact's lineage: canonical event JSON as the
trace renders it, selected per event by the tee's key-based filter, fixed at
load, with the shipped kind filter as the default election. The run identifies
itself in that stream since the ruling of 2026-08-14, admin's run reference
having replaced the ordinal, so what state receives is attributable to the run
that produced it without this crate minting any identity of its own.

One member instance serves one session: it stands with each run, ingests a
stream whose events already carry their session, run, and turn identity, and
its process retires with each unload while its holdings stand for the next
run, so nothing this crate holds needs an identity it minted itself and
nothing survives the session's close.

What organizing means at this charter's level: the holdings are queryable by
the facts the record already carries, the run, the turn, the kind, and the
keys the tee elected. **The representation is ruled**: the operator elected an
embedded relational store, sqlite, on 2026-08-18, and the election stands on
this workshop's own measurements rather than on preference, the regime's
backend comparison having held its registered prediction that an in-process
query is a function call while a service on loopback pays a round trip per
ask. The store lives inside this member's process as its internal
representation, the tee's distillate feeding it and the indexes for the
elected keys built at load, so extension within a session is rows accumulating
under standing indexes and extension of the schema is a new load's new
election. What remains the Spec's: the table and index shapes, the query
surface, and the dependency's own clause.

**The file lives beside the trace, in the operator's territory, owned by the
member's own account.** The operator's ruling of 2026-08-18: the state file
sits in the per-agent directory on the operator's side where the session
record already lives, never in the agent's home, because the wall of section
2 is enforced at the filesystem and the agent's home is the one place the
agent's uid writes - a state file the agent's uid could read would hand the
model its own state through an ordinary tool call. The custody diverges from
the trace's in one named way: the trace is held by descriptor, opened by
admin and handed down, while an embedded relational store opens by path and
keeps sibling files beside itself, so the state member holds its territory by
owning it, a uid of its own over one subdirectory the agent's uid cannot
enter. The stakes tolerate the divergence, because the derivative is
rebuildable from the record and the account never depends on it. The path is
the operator's configuration the way the trace sink's is, and its exact key
is the deployment's.

**The store is reached through the seam and never as a file.** A database file
opened from two processes would be a seam crossing a process line without a
socket, which the first invariant forbids however convenient the driver makes
it. The harness's speed rides the named socket, which this workshop has
measured well below any control loop's cadence, and a caller that someday
needs faster than the seam is a ruling for that day rather than a shared file
today.

## 5. Open cells

- **The schema extension.** Apex section 9's door names one and this charter
  does not write it: the shape of the distillate the tee emits is settled with
  `weaver-harness-state-contract`, because the schema is the seam's vocabulary
  and a contract is a complete interface or it is not a valid contract. The
  store's internal shape is ruled at section 4 and is not this cell: what the
  seam carries and what the store holds are two facts, and only the first is
  the contract's.
- **The tee's charter section.** The distillation surface is the harness's
  mechanism and its paper lands in `weaver-harness-PRD`, owed by the same
  workflow that writes the contract, named here so neither document reads the
  other as already settled.
- **Who else may ask.** Today the harness is the one peer, and every ask
  arrives through it. Whether a later operator surface reads state directly or
  through an admin verb is a cell for the day such a reader exists, refused
  until then by the seam having exactly two ends.
- **The serve surface's shape.** Chartered here and deliberately unshaped:
  its first asker is the context-injection control loop, and the surface is
  elected in that loop's act, against real asks, never before. Until then the
  seam carries ingest traffic whole and the serve direction stands as this
  named cell, which is the fault emission's own pattern on the decode seam.
- **What a control loop's ask looks like.** The loops are the operator's code
  in the harness's seat, so their asks ride the harness's end of the seam. The
  calling shape a loop uses is settled where the loop surface is, not here.
