# weaver-admin-harness-contract

**Status:** MERGED. In `main` and the source of truth. Written with
`weaver-admin-PRD` as one act, per apex section 10, and the two moved together on the
human's ruling of 2026-07-31.

**Date filed:** 2026-07-29
**Revised:** 2026-08-21, the elections become three at this seam. Section 8
draws `surprisal-election` on `weaver-spu-PRD` section 13.12 and draws
`field-election`, defined on this date and drawn by neither contract it
crosses. Both ride inside `spu-instruction` in the enter directive, as the
readout's election has since the route act. No exchange changes.
**Revised:** 2026-08-17, the gate instruction's draw is closed. The route act
named the gap and did not close it, its extent being the election's route, so
section 8's clause read as complete while a payload term crossed it unnamed.
Section 8 draws `gate-instruction`, and `weaver-types-PRD` section 2.1's count
of the contracts drawing it moves from two to three. Owed by #105.
**Revised:** 2026-08-10, the route act. The enter directive carries the SPU
instruction rather than the bare model binding, per `weaver-types-Spec`
section 2, so the readout election the SPU judges at admit crosses this seam
first, admin holding no channel to the SPU. Section 8 draws `model-binding`,
owed from the cut, and `residual-readout-election`, and names the gate
instruction's matching gap as owed rather than closed.
**Revised:** 2026-08-05, the socket inversion and the admin recut, one act. The
creating party inverts: the harness binds the coordination socket inside the agent's
sandbox as its first act and admin dials in, one connection per verb, so the channel
authenticates by credential at the harness's accept, root or refused, apex 5.1's
first case where the earlier form argued the second. The init system carries no
descriptor, admin is per-invocation with no standing end, and the
connection-lifetime rule restates against the listener. Section 5 gains the
harness's refusal guarantee, section 6 its case, and section 8's possession
negative inverts to a `peer-identity` draw.
**Revised:** 2026-08-19, the enter carries the tee's election. Section 3's
enter supplies the state election beside the two instructions, on the same
ground the ruling of `weaver-admin-PRD` section 6 gives them: admin has no
channel to the state member, so if the operator's election does not cross
this seam it crosses nowhere. Section 5's supplies follow by derivation.
Admin resolves an absent declaration to the ruled default of
`weaver-state-PRD` section 4 before the directive, so what crosses is
always the election whole.
**Revised:** 2026-08-14, the run identifies itself. Section 5's guarantee that the run
ordinal is the next one for its session becomes a guarantee that the run
reference distinguishes this run from every other run of that session. The
earlier wording assumed a standing admin and the recut of 2026-08-05 removed
that premise, so the clause asked for something a per-invocation party cannot
know. Section 3's supplies list renames with it.
**Document ID:** `weaver-admin-harness-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The agreement over the coordination seam: what crosses it, what each crossing means,
what each party may rely on, and how it fails. It is read alongside `weaver-admin-PRD`
and `weaver-harness-PRD`, and none of the three is complete without the other two.

**The seam has two initiators at the channel and there is one document for it.** Either
party may open an exchange by the channel's mechanics, and the two-initiator channel is
what makes admin an organ: an organ owns a domain and holds a two-initiator channel
with the harness, both properties and not either, and admin owns the lifecycle domain.
The property is the channel's rather than the exchange census's, the same reading the
half-chartered organ seams take, so the census standing at three exchanges, all
admin's since the fault-carrier ruling of 2026-08-01 rerouted the fault to the
stream, retires no half of what makes admin an organ. The invariant is authored in
the apex and this document is downstream of it.

**Two layers meet in this document and the boundary between them is a draw.** Sections 1
and 2 draw `weaver-organ-channel`, which states the channel mechanics once for every
organ the harness holds a two-initiator channel with, and keep only what is this seam's
own. Sections 3 through 7 are admin's instance, which is the exchange list and its
rules. The layering is the point: the channel does not know what a load directive is, in
the way that IP does not know what a name lookup is. The lift that document's section 0
records was anticipated here and landed on 2026-07-31.

It carries no representation. The types it names have a definition site and no field
list here, the ordering it fixes is stated as a rule rather than as a state machine,
and how any of it is encoded is the Spec's.

```graph
node: weaver-admin-harness-contract
kind: document

edge: party
from: weaver-admin-harness-contract
to: weaver-admin

edge: party
from: weaver-admin-harness-contract
to: weaver-harness
```

**The seam edge is declared once, by the organ rather than by the harness.** Under a
single-initiator reading the declaring crate was the one that asks, and both ask now.
The rule that replaces it is that the organ declares, because the harness is the hub
every organ holds its two-initiator channel with and a hub that declared its own edges
would carry the whole seam graph in one crate. Here the organ is `weaver-admin`. This
document names its parties and does not restate that edge.

## 1. The channel has two initiators

`weaver-organ-channel` section 1 states the message layer: two initiators, the exchange
as the unit, identity by opening party and ordinal, the minimal exchange, concurrent
exchanges, the non-guarantees, and delivery. Nothing in that layer is this seam's own
and nothing of it is restated here.

One fact of this seam lands at that layer. Either party may open an exchange on this
channel, the census of chartered exchanges is section 3's, and every one of them is
admin's today, per the fault-carrier ruling of 2026-08-01.

## 2. The channel

`weaver-organ-channel` section 2 states the process-boundary layer for the organ
channels, and this seam draws it in part since the inversion ruling of 2026-08-05.
What lands here unchanged: boundary preservation as a socket-type property, the
close-on-exec split, and closure never read as an answer. What does not: the
unnamed connected pair, authentication by possession, the holder in transit, and
the channel's life bound to the far process, each departed from below with the
departure stated as this seam's own. The organ channels the harness creates keep
the drawn shape whole, and this seam is the one that left it.

**The harness creates the channel, per the inversion ruling of 2026-08-05, and the
creating party is not the initiating party.** Any socket connecting to the harness
is an internal connection and lives inside the agent's sandbox, so the harness
binds the coordination socket and listens as its first act, before any directive
can arrive, and admin dials in. The earlier form had admin binding before the unit
started because admin was the only party that existed then, and the inversion
retires the premise: nothing needs to exist before the worker, because the worker
brings its own end.

**How each connection arrives is the dial, one per verb.** Admin is per-invocation,
per `weaver-admin-PRD` sections 1 and 7, so each verb's invocation connects to the
socket the worker holds, is served, and closes with the verb. The bind is the
worker's first act and the dial may arrive before it, so the dialing party retries
within a bound the Spec states, and a bound exceeded is a refusal of the verb
rather than a wait without end. The init system carries no descriptor and holds no
end, in transit or otherwise: it starts the unit and that is the whole of its part.

**The credential is this seam's authentication, per apex 5.1's first case.** The
invariant reads by credential where the channel has a name and by possession where
it has none, and this channel has a name the harness bound. The harness reads
`SO_PEERCRED` at every accept, before any byte, and refuses every peer that is not
root. The name is reachable from inside the sandbox, so the check is what refuses
an elected tool at the agent uid, and it discriminates where the earlier design's
credential check could not: the expected peer is root, which no tool of the
agent's holds. The second-opener case `weaver-organ-channel` section 2 rejects
stays rejected, by refusal at accept rather than by an absent name.

**The worker holds the agent uid from its first instruction and clears its dumpable
flag after its final exec.** There is no drop, because the init system starts the unit
at `weaver-<n>` under the delegation `weaver-admin-PRD` section 7 rules. An earlier
form of this clause ordered a drop against the handoff, and the ordering had a subject
only while the worker began life holding a higher principal.

**Nothing about the handoff rested on the drop, which is why removing it costs nothing
here.** A descriptor passed by `SCM_RIGHTS` is a capability, the kernel installs it
against the same open file description, and the receiving uid is never checked against
the file it refers to. So the uid the worker holds at the moment of receipt was never
what made the handoff safe, and the clause that said the drop does not gate what may
cross is now a statement about a uid that never changes.

**Clearing the dumpable flag is what stops a same-uid process from attaching** to the
worker and driving this channel and the trace descriptor directly, and it closes
`/proc/[pid]/fd` as a second route to them by reparenting the directory to root. **The
flag resets on `execve`,** so the requirement is stated against the last exec. This is
the whole of what the removed ordering was protecting and it stands unchanged.

**This section is authoritative for the flag.** It is a property admin relies on and
cannot verify from outside the process, which is what a contract is for, so
`weaver-admin-PRD` section 7 points here rather than restating it.

**On this seam the close-on-exec obligation of `weaver-organ-channel` section 2 lands
on the trace descriptor and this channel's own.** Close-on-exec rides the descriptor
rather than the open file description, so a
receiver calling `recvmsg` without `MSG_CMSG_CLOEXEC` accepts a handle with the flag
clear, and every subprocess a tool call spawns from that point inherits a writable
handle to the trace. Admin can open the file correctly and still lose the property at
the receive, so the obligation is the receiver's in section 5 rather than
the sender's. This channel's own descriptors are the simple case since the
inversion: the listener and every accepted connection are created after the
worker's last exec with the flag asked for in the creating and accepting calls,
so no set-again ordering exists on the worker's side, and admin's dialing end is
flagged at its connect and dies with the verb.

**The listener lives exactly as long as the worker, and a connection lives exactly
as long as its verb.** The lifetime rule of `weaver-organ-channel` section 2 lands
on the listener: bound once at the worker's start, closed by the worker's death,
shared with no second worker. Each accepted connection is one invocation's, closed
by admin when the verb answers, and the harness serves one connection at a time, a
second dial waiting at the listener rather than being answered concurrently.

## 3. The exchanges

Three, and no others, all opened by admin.

**Enter the run.** Opened by admin. Admin directs the harness to enter, supplying the
session identity, the run reference, the trace descriptor, the
SPU instruction, the gate instruction, and the state election the tee applies. The
last three are in the directive because the
ruling of `weaver-admin-PRD` section 6 gives admin no channel to the SPU, the gate,
or the state member, so if admin's intent for any of them does not cross this
seam it crosses nowhere. The harness
stands up an empty working structure, authors its `load` event, asks the SPU to
admit against the instruction it was handed, and starts Gate last. It answers ready
only when
every step of that fan-out has confirmed, or it refuses, and a refusal names where the
fan-out stopped, so that admin rolls back what was built without asking a second
question. The answer, either way, closes the exchange and is the aggregate: one
directive out, one answer back, and the organs appear in the answer's content rather
than as parties to this seam.

**Leave the run.** Opened by admin. Admin directs the harness to leave. The harness
stops Gate first, refuses while a turn is in flight, authors its `unload` event,
drains the writer's queue to the stream, and releases the SPU. It answers left, or
it refuses, and a refusal names where the sequence stopped. The stream ends where
the run did, finalized by nothing, per the ruling of 2026-08-01. As with enter, the
answer is the aggregate and the organs appear in its content rather than as parties
to this seam.

**Stop the turn.** Opened by admin. Admin conveys the operator's intent to stop, one
bit and no work. The harness aborts the turn in flight, the turn closes with the stop
reason marked in place of a response, and the run stays open. The harness answers
with the turn's fate, aborted naming the turn it closed, or at rest because nothing
was in flight, and both are clean closes of the exchange rather than refusals,
because the operator's intent is satisfied by the state either way. The answer is
given only after the close event is placed, which is the announce-after-record
discipline. Stop touches no run bracket. It is the channel
the operator interrupt of `weaver-harness-PRD` section 2 arrives on, and it exists on
this seam because the operator holds no other crossing. How the abort lands at the
decoder is the harness's interior and crosses nowhere.

**There is no alert exchange, per the fault-carrier ruling of 2026-08-01.** A fault
the worker survives is a `fault` event, authored by the harness into the stream
like every other event, per `weaver-trace-PRD` section 3.1, and the stream is the
program's one fault carrier: the operator's tooling keys on it there and comes back
by running a verb, per the basic loop's section 2. Admin learns nothing of a fault,
holding
custody of the sink and comprehension of nothing. An earlier form of this section
carried the fault to admin as a fourth exchange, the alert, and the ruling retired
it: with one outbound path carrying every event in order, a second carrier for the
same fact was a channel earning nothing.

**A fault the worker does not survive is not a `fault` event.** Death is observed
through closure per section 4, and the harness does not report its own death.

**The fault's case set is open with a defined exit.** The candidates named so far
originate in the SPU and reach the record through the harness as author. The set
closes when the organs that can raise a fault have charters naming what they raise,
and the first of those is `weaver-spu-PRD`. This document binds nothing of it, the
shape now being the event kind's, per `weaver-trace-PRD` section 3.1.

**The trace descriptor crosses once, in the enter exchange.** It is not re-sent, not
revoked, and not replaced. A harness that needs a descriptor it was not given has a
failed load rather than a second request to make, because there is no exchange in
which it asks for one.

**No exchange carries a path.** Admin sends handles and the harness never learns a
name, which is the descriptor discipline of `weaver-harness-PRD` section 5 stated as
an obligation on the party that could break it.

**No exchange carries work,** in any form and under any framing, in either direction.

## 4. Ordering

- The ordering below is the worker's rather than any connection's: exchange state
  survives a connection closing, because connections come and go with verbs and
  the run does not.
- Enter is first and happens exactly once in a worker's life.
- Stop is valid only between a completed enter and a leave, and a stop arriving at
  rest answers at rest rather than refusing.
- An organ fault before the enter aggregate is answered is a refusal on the enter
  exchange naming the arm, rather than a `fault` event, the report to admin and
  the account on the stream being two different things.
- Leave is last, happens at most once, and is terminal for the worker.
- Messages within one exchange are ordered.
- An answer to enter arrives only after the working structure is standing, the
  model is admitted, and Gate is started, so admin may rely on a ready answer meaning
  the interior is serving rather than starting. The reliance is exactly as large as
  the fan-out, per section 3.
- An answer to leave arrives only after the queue is drained, so admin may rely on a
  left answer meaning what was admitted reached the stream.
- A directive that arrives out of this order is refused and is not queued.

## 5. What each party supplies and guarantees

This section is derived from section 3 rather than prose beside it, because every
exchange payload change is a supplies change by construction, and a Spec writer reads
this list.

**Admin supplies** the session identity and the run reference for the run being
entered, the
trace descriptor, the SPU instruction the fan-out admits,
the gate instruction the fan-out starts, the state election the tee applies,
resolved to the ruled default where the declaration is silent, and the intent
to stop.

**Admin guarantees** that the trace descriptor it passes refers to the sink the
session's configuration declares, that the run reference distinguishes this run
from every other run of that session, distinctness being the guarantee rather
than any particular rendering of it and the session possibly spanning agents,
and
that the boundary the worker runs inside exists and is correct, because
admin verified it before the unit started and is the only party positioned to. The
guarantee is of verification rather than of authorship, since the boundary is the
operator's artifact. It guarantees that no directive carries work of any kind.

**The harness supplies** its readiness as the aggregate of the enter fan-out, its
confirmation of departure, and the turn's fate on a stop.

**The harness guarantees** that every connection is credential-checked at its
accept, before any byte is read, and that a peer that is not root is refused, per
section 2. It guarantees that every descriptor it accepts is accepted close-on-exec,
per section 2, which is an obligation on the receiving call and cannot be met by the
sender. It guarantees that it authors the run's bracket
events, that it writes only through the descriptor it was handed, that it resolves no
path, and that a ready answer is given only after a standing working structure, an
admitted model, and a started gate. It guarantees that a refusal names where the
fan-out stopped, so that admin rolls back on the answer alone. It guarantees that a
fault the worker survives is authored to the stream as a `fault` event, per the
fault-carrier ruling, and that no run blocks on anything downstream of the
emission. It guarantees that a stop answer follows the close event it reports, so
the record holds the abort before the channel does.

**Close-on-exec is the receiver's, and only the receiver can supply it.** The flag
rides the descriptor rather than the open file description, so it does not cross with
a passed handle and the receiving call is the one place it exists. It is a behavior
rather than a type property on the receive path, so it takes the perturbation-verified
test apex section 11 asks for rather than a compile-time pin. What can be pinned is
the shape: one receive site, taking no flag argument, returning a handle the rest of
the crate cannot construct another way.

**Neither party guarantees the stream's tail.** `weaver-trace-PRD` section 4.2
forfeits the writer's queue to process death and bounds the depth by the deployment,
so an answer to leave covers what was drained and an abrupt exit covers nothing.

## 6. Failure

Refusals are typed and enumerable, and every one of them is the harness refusing an
ask, because admin answers nothing. The cases:

- the dialing peer's credential is not root, and the connection is refused at the
  accept before any exchange opens
- the descriptor is absent, unusable, or does not carry the required flags
- an organ the enter fans out to refused, and the refusal names which organ and
  carries its reason, so the aggregate answer is one refusal rather than a report to
  parse
- the directive is out of order for the channel's state
- activity is not at rest, so the run cannot be left

**A stop at rest is not a refusal.** Nothing was in flight, the intent is satisfied
by the state, and the answer says at rest. The out-of-order case above still covers a
stop before enter or after leave.

What a refusal leaves behind is scoped by the authoring point of the fan-out.
Before the `load` event is authored, a refusal leaves the harness in the state it
was in before the directive: no run was entered, no bracket was opened, and the
stream never shows a run that was not entered. After the `load` event is authored,
a refusal from a later arm leaves the authored bracket on the stream with no
`unload` behind it, a truthful account of a load that did not complete rather than
corruption to repair, per `weaver-admin-PRD` section 5. The exit itself does not
change: the harness reports where the fan-out stopped, admin unwinds and publishes
no state, and nothing reaches back to erase what was authored.

**A worker that dies is not a refusal.** Admin observes the process exit and the
channel closure together, and what that leaves on the stream is a run whose `load`
has no `unload`, a truthful account of a death rather than corruption to repair,
per `weaver-admin-PRD` section 5.

**Nothing on this seam retries.** A refused directive returns to admin, which either
rolls back or reports. A harness that retried an author, or an admin that re-sent a
directive after a refusal, would put two attempts behind one operator intent.

## 7. Prohibitions

**On admin.** It sends no work, in any form and under any framing, and a run in progress
narrows nothing about that. It sends no path. It asks for no event to be authored on its
behalf. Into a running
turn it conveys the operator's intent to stop and nothing narrower, because the abort's
mechanics are the harness's, per `weaver-admin-PRD` section 3, and unload still waits on
rest rather than racing it.

**On the harness.** It opens no exchange at all, the alert retired to the stream by
the fault-carrier ruling, and this is the prohibition that replaces the older one
that it initiates nothing. It writes nothing
outside an exchange. It does not resolve a trace path or accept one. It does not report
its own death. It announces nothing it has not first recorded. It asks admin for
nothing, because a notification carrying a request
is a control surface wearing a notification's clothes. It does not treat a directive as
authorization for anything beyond the directive, which is the shape a lifecycle channel
would grow a control surface through.

**On both.** Neither party carries a fact about the other's interior. Admin does not
know what a turn contains and the harness does not know what a boundary is made of, and
the exchanges above are the whole of what either learns.

## 8. Vocabulary

**Drawn from `weaver-types`:** `organ-envelope`, `lifecycle-directive`,
`lifecycle-answer`, `lifecycle-refusal`, `peer-identity` as of the inversion
ruling of 2026-08-05, because the identity the harness reads at every accept is
the floor's, `model-binding` and `residual-readout-election` as of the
route act of 2026-08-10, `gate-instruction` as of 2026-08-17,
`state-election` as of 2026-08-19, and `field-election` with
`surprisal-election` as of 2026-08-21.

```graph
edge: draws
from: weaver-admin-harness-contract
to: peer-identity

edge: draws
from: weaver-admin-harness-contract
to: organ-envelope

edge: draws
from: weaver-admin-harness-contract
to: lifecycle-directive

edge: draws
from: weaver-admin-harness-contract
to: lifecycle-answer

edge: draws
from: weaver-admin-harness-contract
to: lifecycle-refusal
```

**`model-binding` and `residual-readout-election` are drawn as of the route act
of 2026-08-10, and the first was owed from the cut.** The enter directive has
carried the model binding since this contract was written, and a payload term
the clause never named left the interface short of the completeness apex
section 5.3 demands. The election joins it because admin holds no channel to
the SPU, per `weaver-admin-PRD` section 6, so what the SPU judges at admit
crosses this seam first or crosses nowhere. Both are fields of the agent's
configuration, defined at `weaver-types-PRD` section 2.1, and both cross inside
`spu-instruction`, the section `weaver-types-Spec` section 2 shapes. The
section is representation rather than a term of its own, so the draws name the
definitions and not the grouping.

**`field-election` and `surprisal-election` are drawn as of 2026-08-21, and
the first was owed from the act that defined it.** Both cross inside
`spu-instruction` beside the binding, and both reach the SPU by this seam
first or by none, which is the route act's own argument applied to the two
elections added after it. The field's election was defined on this date and
drawn by neither contract it crosses, so the graph carried a definition no
seam admitted to carrying while the SPU judged its depth at admit. **The
rule that catches this is `weaver-types-PRD`'s**, that a field the SPU
judges is a field that reached it across a seam, and it was stated for the
readout and not applied to what followed.

**`gate-instruction` is drawn as of 2026-08-17, and the route act named it
owed.** It crosses inside the same directive as the two above and had the same
gap: the enter directive has carried it since the fan-out was drawn, per
sections 3 and 5, and a payload term the clause never names leaves the interface short
of the completeness apex section 5.3 demands. The argument is the election's,
one seat over: admin holds no channel to the gate, so the instruction that
names the seams the gate holds crosses this seam first or crosses nowhere, and
the harness carries it to the gate spawn. The definition is
`weaver-types-PRD` section 2.1's, and this contract is the third to draw it,
after the two gate seams that consume it.

```graph
edge: draws
from: weaver-admin-harness-contract
to: model-binding

edge: draws
from: weaver-admin-harness-contract
to: residual-readout-election

edge: draws
from: weaver-admin-harness-contract
to: field-election

edge: draws
from: weaver-admin-harness-contract
to: surprisal-election

edge: draws
from: weaver-admin-harness-contract
to: gate-instruction
```

**`state-election` is drawn as of 2026-08-19, with the declaration act that
adds it to the enter.** The argument is the standing one, a third seat over:
admin holds no channel to the state member, so the election the tee applies
crosses this seam first or crosses nowhere, and a payload term the clause
never named would leave the interface short of the completeness apex section
5.3 demands. The definition is `weaver-types-PRD` section 2.1's, the shape is
`weaver-types-Spec` section 2's, and the shape's groupings, `StateElection`
and `ElectedKindConfig`, are representation rather than terms of their own,
per this section's standing rule: the draws name the definitions and not the
grouping.

```graph
edge: draws
from: weaver-admin-harness-contract
to: state-election
```

**Drawn from `weaver-traits`:** nothing. The clause is present with that answer
because `weaver-types-PRD` section 5 asks for it even when it is empty.

**`organ-envelope` belongs to the floor and not to this seam,** because it is the
carrier every organ contract draws rather than a thing admin and the harness agreed on
between themselves. It is named here because this was the first contract to need it. The
definition stays in `weaver-types` and the mechanics it serves live in
`weaver-organ-channel`, per `weaver-types-PRD` section 2.3.

**`peer-identity` is drawn as of the inversion and `authorization-predicate` still
is not, each stated rather than left to edges.** This seam authenticates by
credential per section 2, and what the harness reads at accept is the floor's
`peer-identity`. The rule it applies is fixed at root rather than configured, so no
predicate definition is reached and `authorization-predicate` stays undrawn.
`weaver-types-PRD` section 2.2 rested its scoped claim on this contract being the
counterexample to a universal, and this act re-aims that claim in the same batch,
the counterexample having inverted.

**Drawn from `weaver-trace`:** nothing. No event kind, envelope field, or payload
shape crosses this seam, and this contract names no field of the record's envelope.

**The definitions land in `weaver-types` and were owed by the act that wrote this
section, four of them, the fifth having left with `harness-alert`.**
`weaver-types-PRD` section 4 rules that wire vocabulary is absent on demand and that
the shared representation arrives when the first socket contract is written. This is
that contract, so the demand exists now and the definitions belong to the floor rather
than to either party, since admin and the harness both need them and neither may
depend on the other. The records below belong in `weaver-types-PRD` section 4 and are
written unfenced deliberately, so that a mapper reading this document does not ingest
records this document is not the source of:

    node: organ-envelope
    kind: vocabulary

    node: lifecycle-directive
    kind: vocabulary

    node: lifecycle-answer
    kind: vocabulary

    node: lifecycle-refusal
    kind: vocabulary

    edge: defines
    from: weaver-types
    to: organ-envelope

    edge: defines
    from: weaver-types
    to: lifecycle-directive

    edge: defines
    from: weaver-types
    to: lifecycle-answer

    edge: defines
    from: weaver-types
    to: lifecycle-refusal

Four definitions and no more. A carrier, a directive with its cases, an answer with
its cases, and a refusal with its cases is what sections 1, 3, and 6 demand, and a
fifth added because a fifth felt tidy would be a reserved slot in data form. The
stop exchange adds a case to `lifecycle-directive` and a case to `lifecycle-answer` and
adds no fifth definition, which is the enumeration growing where the shape already
lives. `harness-alert` was the fifth until the fault-carrier ruling of 2026-08-01
retired the alert exchange, the fault travelling as a `fault` event on the stream,
and the definition left `weaver-types-PRD` section 2.3 in the same act.

## 9. What this document changes elsewhere

Named here because a document whose revision reaches other documents and cannot be read
for the reach is a trap. These are owed by this act and belong in `weaver-admin-PRD`
section 11's register.

- `weaver-admin-PRD`. The seam is bilateral and the charter says so. That is a
  statement about admin being an organ rather than a statement about alerts.
- `weaver-admin-PRD` section 10. The descriptor cell is unchanged in count, because one
  pair still carries the seam, and unchanged in its exit condition.
- The G4 union grew from three values to five with the duplex rewrite of this
  contract, which drew `organ-envelope` and `harness-alert` where the simplex form
  drew three. The count of contracts is unchanged, both crates remain party to one,
  and an earlier form of this line said the union was unchanged by reading the first
  fact for the second.
- **The union stands at five again by a different route, as of 2026-08-05.** The
  fault-carrier ruling of 2026-08-01 retired `harness-alert` and left four, and the
  inversion of this act adds `peer-identity`, the identity the harness reads at
  every accept. No definition is owed to the floor by either move: `peer-identity`
  is defined at `weaver-types-PRD` section 2.2 and `harness-alert`'s definition left
  that charter with the exchange. The four definitions section 8 lists unfenced are
  what this contract's own act owed and remain four.

**Three of these landed on 2026-07-31 and are struck rather than deleted, so that a
reader of an earlier revision can tell a closed item from one that was never there.**

- `weaver-types-PRD`. Two definitions were added to the three already owed, and
  `organ-envelope` is marked as floor vocabulary rather than as this seam's. Landed in
  section 2.3, which is where that charter keeps definitions, rather than in section 4,
  which is where this list said they would go and where only the departure argument
  lives.
- `weaver-harness-PRD`. The harness opens exchanges, and the alert emit point is not
  designed on the assumption that the record is its only sink. Landed in section 4,
  beside the seam table the clause describes.
- `WeaverTools-PRD`. The organ definition, its two properties, and the harness as hub
  rather than spoke. Landed as invariant 5.4, taken early as a named exception to
  Working Process section 7 and recorded as such in apex section 5. This is the one
  item on this list that was registered and not applied under the apex rule, and the
  exception is what released it.

**Landed with the fifth revision, recorded rather than owed,** because the batch of
2026-07-31 carried every party in one act:

- `weaver-admin-PRD` sections 3, 4.1, and 8. The activity-control split, the
  descriptor payload at load, and the operator surface's stop conveyance.
- `weaver-harness-PRD` section 2. The interrupt's citation.
- `weaver-trace-PRD` section 3.1 and `weaver-harness-trace-contract` section 3. The
  `turn.closed` payload states its close kind.
- The `WeaverTools-PRD` correction list, deposited at review rather than by this
  act and cited by substance rather than position: the gate binds no network socket,
  restated at the apex re-authoring.
