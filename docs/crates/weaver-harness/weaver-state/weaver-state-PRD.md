# weaver-state - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth.

**Revised:** 2026-09-04, second of this date, the identity is the session's first
holding. Section 4 lands the three rulings of issue #422: the declaration seeds
and the store governs, a missed read of the identity fails the enter closed, and
the seated prefix crosses the tee whole under every election. Within a session
only, the memory leg untouched.

**Revised:** 2026-09-04, the store is a port and the engine is the deployment's
election. Section 4 recuts the representation ruling: the seam names asks and
never a query language, the store holds its one opinion at the engine's port,
the embedded engine stays as the default an absent election means with its
2026-08-18 measurement as its ground, and the service engine joins on chapter
six's shape argument and on who this is built for. Custody becomes the
engine's, the service engine's wall being two gates and one identity, with the
engine, the database, and the role members of the binding and the grant
surface read back at the close. Section 3 states that a persisting substrate is
not a crossing. Section 5 gains the third-engine cell and the service
retirement cell. Per issue #411, items 1 and 2.
**Revised:** 2026-08-26, the first door loses its name. Section 3's seam clause
recuts per the operator's ruling of this date: the harness channel is a
socketpair admin creates at the member's spawn, one end inherited and one
crossing at the enter, authenticated by possession per the first invariant's
rule for a channel with no name. The preload door becomes the member's one
named socket and its name moves into this member's own territory. The door's
judgment clause stops describing an inversion of a judgment that no longer
exists and states its own: the operator principal admitted, every other peer
refused.
**Revised:** 2026-08-24, the second door stands. Section 3 gains the preload
door, per the operator's ruling of this date and the taxonomy promotion:
`weaver-analysis-state-contract`, standing only under a diagnostic binding,
admitting the operator principal and refusing the agent, carrying the first
door's distillate shapes drawn rather than redefined. Section 5's
who-else-may-ask cell is unchanged, the driver never asking. The opener
retires the declared session's prior holdings, so a retry replaces rather
than doubles. Every party to
the new contract merges in this act.

**Revised:** 2026-08-19, second of this date, the election reaches the
declaration. Section 5's tee-charter cell closes its remaining part: the
election's block in the agent's file is `state-election`, shaped at
`weaver-types-Spec` section 2, optional with its absence meaning this
charter's ruled default, and it rides the enter directive per
`weaver-admin-harness-contract` sections 3 and 5, every party merging in
the act.
**Revised:** 2026-08-19, the serve surface takes its shape. Section 5's
serve cell and calling-shape cell close: the first asker arrived as the
context-injection loop, the ask vocabulary landed in
`weaver-harness-state-contract`, the representation landed in
`weaver-state-Spec` section 4, and the loop's calling shape landed as the
seat's state port at `weaver-harness-Spec` section 6. Section 1's
build-order asymmetry is history and reads as such.
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
in and is kept, and what is kept answers asks. The two were not symmetric
at chartering, and the asymmetry was load-bearing for the build order. The
ingest had its producer standing and built whole against real traffic on a
real agent, landing 2026-08-18. The serve half waited for its first asker,
per the reserved-slot rule: a query surface elected against a guess about
its caller is an interface-shaped empty joint, and the fault report's own
history is the precedent, chartered and unshaped rather than absent until
its first real traffic gave it a shape. The asker arrived 2026-08-19 as the
context-injection loop, and the surface was elected against its real ask
rather than ahead of it, which is the rule honored rather than raced.

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
working derivative of the account. The distillation surface is the tee over
the canonical event stream, ruled by the operator 2026-08-12: the mechanism is
`weaver-trace`'s, because what is being tee'd is the trace's own rendering,
and the harness applies it as the one party that writes. Its output is this
crate's ingress: state receives what the tee elects, holds it organized, and
answers with it.

**The labor divides three ways and each part holds one.** The tee selects and
never computes. This crate transforms as part of organizing, per the
operator's ruling of 2026-08-18: a derived shape, an aggregate, an index are
custody's work on what was selected, and they carry no judgment about what
the turn should do with them. The harness's loops decide. A tee that
computed would smuggle state's work into the trace's crate, and a state that
decided would smuggle the loops' work into custody, and the three-way split
is what keeps each part answerable to its own charter.

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

**The channel is the seam's own and reuses nothing.** Apex section 9 says a new
socket and this charter reads it literally: the coordination seam keeps its
one kind of traffic, and state's seam is a second, distinct channel. **It has
no name, per the operator's ruling of 2026-08-26.** Admin creates a socketpair
at the member's spawn, this member inherits one end, the harness receives the
other at the enter, and the channel authenticates its peer by descriptor
possession, per the first invariant's rule for a channel with no name. A name
would have to stand where a dialer can reach, and the one directory the worker
could reach is the one place a name can be replaced before it is dialed, so
the channel that needs no name carries none. How the ends travel is the
contract's mechanics, deliberately absent here.

**A second door stands on this member as of 2026-08-24, and only under a
diagnostic binding.** `weaver-analysis-state-contract` names it, initiator
first: the diagnostic driver preloads the holdings from a finished trace it
parsed outside the agent, and this member receives on a second socket what a
live tee would have fed on the first, the same distillate shapes drawn from
the first door's contract rather than redefined. The door's judgment is a
credential's, this member's one such judgment since the first door
authenticates by possession: it admits the operator principal and refuses
every other peer, the agent's among them. Its name
stands in this member's own territory, per the operator's ruling of
2026-08-26, where the agent's identity holds nothing. The door itself does
not exist under a serving binding, so
the serving membrane of `weaver-agents-PRD` section 0 is untouched. No ask
crosses it, ever: the driver is a sender and never an asker, so section 5's
who-else-may-ask cell keeps its answer, the serve direction having exactly
the two ends it had. The door's opener retires the declared session's prior
holdings in the same transaction, per the contract's section 2, so a preload
lands against empty whatever stood, and a retry after a dead driver is a
replacement rather than a double - the recovery invariant stated before the
loss clause leans on it. With that in place the loss clause below covers the
new door without amendment, the preload being rebuildable from the record
more directly than any holding the tee fed.

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
and the close of the session is what retires them. The service engine keeps
its rows on the same terms the file does, per the ruling of 2026-09-04: rows
that stand in a database after a session closes are the same fact as rows that
stand in a file, and the boundary holds in the answers, every answer bounded to
the session its opener declared per the contract, so a persisting substrate is
not a crossing. Persistence across
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

The input is the inspected artifact's lineage: canonical event JSON as the trace renders
it, selected per event by the tee's key-based filter, fixed at load. **The tee's rulings
of 2026-08-18**: the envelope always rides and is not electable, session, run, turn,
kind, and sequence crossing on every distilled event so no election can produce an
unattributable row, and the election ranges over payload keys alone, with one exception
ruled 2026-09-04 and stated below. An elected kind with no payload keys is a meaningful
election, because presence itself is state. **The default election is the envelope of
every kind and nothing more**, the operator electing payload keys on top of it, so a
deployment that elects nothing still holds the session's shape - what happened, in what
order, in which turn - and pays for no payload it never asked to keep. An event the
election does not match costs nothing and is dropped at the tee, the trace remaining
complete regardless, because the tee reads the stream and never thins it. The run
identifies itself in that stream since the ruling of 2026-08-14, admin's run reference
having replaced the ordinal, so what state receives is attributable to the run that
produced it without this crate minting any identity of its own.

**The identity is the session's first holding, per the operator's ruling of 2026-09-04
on issue #422.** The system prompt is the first bounding of the possibility space the
decoder samples from, which is to say it is context, and context is this crate's
material and not the declaration's. Three rulings land it. **The declaration seeds and
the store governs**: a session's first load seats the declaration's `identity` field and
the tee lands it here as the turnless `message.system` events at the run's opening, and
from then on what this crate holds under that kind is the session's identity, answered
to the harness's `identity` ask at every later load's opening, the declaration's field
authoritative for the seed alone. **A missed read fails closed**: the harness refuses
the enter where the ask misses, per `weaver-harness-Spec` section 2, because a run with
no bounding is not a run this charter's custody can stand behind, and this is the one
ask the dead-peer clause does not convert. **The identity's kind cannot be elected
out**: the seated prefix crosses the tee whole under every election, per
`weaver-trace-PRD` section 11 as revised this date, the one exception to the key-based
rule, so no election produces a session whose identity this crate never held. Within
this act the store's identity is what the session's first load seated, and a mechanism
that lands a revised prefix mid-session is a further act, taking effect at the next load
because the decode seam holds the prefix permanent for a residency. Across sessions
nothing moves, per apex section 9: an identity that individuates across sessions is the
memory leg's, a schema extension with its own socket and contract, and this ruling is
compatible with that path and does not take it. Where no member stands, the declaration
governs alone, which is what it did before the ruling.

One member instance serves one session: it stands with each run, ingests a
stream whose events already carry their session, run, and turn identity, and
its process retires with each unload while its holdings stand for the next
run, so nothing this crate holds needs an identity it minted itself and
nothing survives the session's close.

What organizing means at this charter's level: the holdings are queryable by
the facts the record already carries, the run, the turn, the kind, and the
keys the tee elected.

**The store is a port, and the engine behind it is the deployment's election,
per the operator's ruling of 2026-09-04.** This crate's custody has no opinion
about which database keeps its rows. The seam to the harness names asks and
never a query language, which `weaver-harness-state-contract` already holds
and this ruling makes a rule: what crosses the seam is the same whatever engine
answers it, and the only place this crate holds an opinion about a database is
the port that integrates that database, one integration per engine, like a
plugin. Two engines are chartered, and the declaration elects one per agent.
**The embedded engine, sqlite, is the default an absent election means**, and
the election of 2026-08-18 stands as its ground: the regime's backend
comparison held its registered prediction that an in-process query is a
function call while a service on loopback pays a round trip per ask, so where
nothing outside the member needs the rows, the embedded engine is the right
one and stays. **The service engine, postgres, is elected where the rows are
worth asking from outside the member's process**, and its ground is two facts
the embedded engine cannot buy. The first is the shape argument the merged
document's chapter six makes: a store is worth having when it can be asked,
and similarity, ranking, a classifier in front of the selection and a reranker
behind it are cheap against a queryable structure and impossible against a
flat file, the two holding the same thing and one of them answerable. The
second is who this is built for: an engineer pulling a cloud application down
to a local box, where the service store is the thing already on the machine,
and where holding the level-A problems fixed - the run deterministic, recorded,
and comparable - is the same substrate that later lets the individuation
question be explored on the object the engineer already has rather than on a
rebuilt one. The store is the framework's drawing of a line and not a
requirement on anyone else's, per the paper's community-instrument position:
an adopter needs the framework's line and its record, not its store. **A third
engine is not laid in.** The previous program tied a graph store into its base
code, and this ruling keeps every engine out of the base by the same port that
admits the two: a further engine arrives as an integration in its own act, and
section 5 names that cell. Under either engine the tee's distillate feeds the
store and the indexes for the elected keys are built at load, so extension
within a session is rows accumulating under standing indexes and extension of
the schema is a new load's new election. What remains the Spec's: the port's
shape, each engine's table and index shapes, the query surface, and each
dependency's own clause.

**Custody is the engine's, and each engine holds the wall its own way.** The
wall of section 2 is one requirement: the agent's uid reaches no store, because
a store the model's uid could read would hand the model its own state through
an ordinary tool call. **Under the embedded engine the wall is the filesystem**,
per the operator's ruling of 2026-08-18: the state file sits in the per-agent
directory on the operator's side where the session record already lives, never
in the agent's home, which is the one place the agent's uid writes. The custody
diverges from the trace's in one named way: the trace is held by descriptor,
opened by admin and handed down, while an embedded store opens by path and
keeps sibling files beside itself, so the member holds its territory by owning
it, a uid of its own over one subdirectory the agent's uid cannot enter. The
stakes tolerate the divergence, because the derivative is rebuildable from the
record and the account never depends on it. **Under the service engine the wall
is two gates and one identity**, per the ruling of 2026-09-04. The service gate
is kernel-class: the store's socket is a unix domain socket, and the member
dials it under its own account, the peer verified by credential as every
internal seam's is. It is checkable by looking and moves only by creating a
different process. The object gate is the store's own access model: the
database and the role's grants draw what the verified identity may touch. That
gate is configuration and therefore mobile, so it is disciplined: **the engine,
the database, and the role are members of the binding**, declared in the
agent's file, changing only across the load boundary, and named on the load
event the way every fact that decides a record is. Peer authentication welds
the object gate to the service gate, the store mapping the member's kernel
identity to its role, so the object gate's identity is derived from the kernel
fact rather than asserted a second time, and the agent's uid, mapping to no
role, is refused at the second gate where it was not already refused at the
first. **The close reads the boundary back**: the grant surface is read from
the store's catalog at the enter and re-read at the leave, and the leave event
carries the reading as unchanged, varied, or unreadable, the envelope the
confirm drivers already carry for provenance. A grant surface that varied
mid-session is a boundary move the record must carry and never absorb. The
territory's path and the store's socket, database, and role are the operator's
configuration the way the trace sink's is, and their exact keys are the
declaration's, shaped at `weaver-types-Spec` section 2.

**The store is reached through the seam, never as a file and never as a
connection.** A database file opened from two processes would be a seam
crossing a process line without a socket, which the first invariant forbids
however convenient the driver makes it, and a second connection to the service
engine from the harness or a loop would be the same crossing with a better
excuse: the member is the one holder of the engine under either election, and
the harness asks the member. The harness's speed rides the seam's standing
channel, which this workshop has measured well below any control loop's
cadence, and a caller that someday needs faster than the seam is a ruling for
that day rather than a shared file or a shared connection today.

## 5. Open cells

- **The schema extension.** Apex section 9's door names one and this charter
  does not write it: the shape of the distillate the tee emits is settled with
  `weaver-harness-state-contract`, because the schema is the seam's vocabulary
  and a contract is a complete interface or it is not a valid contract. The
  store's internal shape is ruled at section 4 and is not this cell: what the
  seam carries and what the store holds are two facts, and only the first is
  the contract's.
- **The tee's charter section. Closed 2026-08-19.** The distillation
  surface's mechanism is `weaver-trace`'s, per the ruling of 2026-08-18,
  its paper standing at `weaver-trace-PRD` section 11 since the seam act,
  and the deployment's shape settled with the declaration act: the
  election's block is `state-election` in the agent's file, shaped at
  `weaver-types-Spec` section 2, riding the enter directive per
  `weaver-admin-harness-contract`, absent meaning section 4's ruled
  default.
- **A third engine.** The port admits an engine by an integration of its
  own, and the previous program's graph store is the one most likely to ask,
  having been tied into that program's base code. It arrives as a port
  implementation in its own act, with its own custody clause and its own
  binding members, and never in the base: the base holds the port and the
  two engines this workshop runs, per the ruling of 2026-09-04.
- **The retirement of a session's rows under the service engine.** The
  embedded engine's cell at `weaver-state-Spec` section 6 asks what a session's
  close does to the disk. The service engine asks the same of a database that
  many sessions share, and the answer is elected with the first act that gives
  sessions a close in practice, the boundary holding in the answers meanwhile.
- **Who else may ask.** Today the harness is the one peer, and every ask
  arrives through it. Whether a later operator surface reads state directly or
  through an admin verb is a cell for the day such a reader exists, refused
  until then by the seam having exactly two ends.
- **The serve surface's shape. Closed 2026-08-19.** Chartered here and held
  deliberately unshaped until its first asker, which is the fault
  emission's own pattern on the decode seam. The context-injection loop
  arrived and the surface was elected in its act against its real ask: the
  vocabulary in `weaver-harness-state-contract` section 2, the
  representation in `weaver-state-Spec` section 4.
- **What a control loop's ask looks like. Closed 2026-08-19.** The loops
  are the operator's code in the harness's seat, so their asks ride the
  harness's end of the seam, and the calling shape landed where the loop
  surface is: the seat's state port, `weaver-harness-Spec` section 6.
