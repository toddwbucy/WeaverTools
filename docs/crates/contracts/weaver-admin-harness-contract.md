# weaver-admin-harness-contract

**Status:** MERGED. In `main` and the source of truth. Written with
`weaver-admin-PRD` as one act, per apex section 10, and the two moved together on the
human's ruling of 2026-07-31.

**Date filed:** 2026-07-29
**Revised:** 2026-08-05, the role ruling and the grant mechanism's reopening. Per the
operator: `weaver-admin-role` is assumed by a human and never by an AI or an
automation, a statement of design intent and not a guarantee about conduct;
`weaver-admin-user` is a service account rather than a login account and is where the
delegation attaches; and the crate is the peer organ that account runs, whose narrow
domain includes custody of where the record leaves the system. The grant mechanism
reopens from the init system and re-closes on `sudo`, admin becoming the worker's
parent, which dissolves the cell asking how the coordination end reaches a process
admin did not fork.
**Document ID:** `weaver-admin-harness-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The agreement over the coordination seam: what crosses it, what each crossing means,
what each party may rely on, and how it fails. It is read alongside `weaver-admin-PRD`
and `weaver-harness-PRD`, and none of the three is complete without the other two.

**The seam is duplex at the channel and there is one document for it.** Either party
may open an exchange by the channel's mechanics, and the duplex channel is what
makes admin an organ: an organ owns a domain and holds a duplex channel with the
harness, both properties and not either, and admin owns the lifecycle domain. The
property is the channel's rather than the exchange census's, the same reading the
half-chartered organ seams take, so the census standing at three exchanges, all
admin's since the fault-carrier ruling of 2026-08-01 rerouted the fault to the
stream, retires no half of what makes admin an organ. The invariant is authored in
the apex and this document is downstream of it.

**Two layers meet in this document and the boundary between them is a draw.** Sections
1 and 2 draw `weaver-organ-channel`, which states the channel mechanics once for every
organ the harness is duplex with, and keep only what is this seam's own. Sections 3
through 7 are admin's instance, which is the exchange list and its rules. The layering
is the point: the channel does not know what a load directive is, in the way that IP
does not know what a name lookup is. The lift that document's section 0 records was
anticipated here and landed on 2026-07-31.

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
simplex reading the declaring crate was the one that asks, and both ask now. The rule
that replaces it is that the organ declares, because the harness is the hub every organ
is duplex with and a hub that declared its own edges would carry the whole seam graph
in one crate. Here the organ is `weaver-admin`. This document names its parties and
does not restate that edge.

## 1. The channel is duplex

`weaver-organ-channel` section 1 states the message layer: two initiators, the exchange
as the unit, identity by opening party and ordinal, the minimal exchange, concurrent
exchanges, the non-guarantees, and delivery. Nothing in that layer is this seam's own
and nothing of it is restated here.

One fact of this seam lands at that layer. Either party may open an exchange on this
channel, the census of chartered exchanges is section 3's, and every one of them is
admin's today, per the fault-carrier ruling of 2026-08-01.

## 2. The channel

`weaver-organ-channel` section 2 states the process-boundary layer: the unnamed
connected pair, one pair for both directions, boundary preservation as a socket-type
property, authentication by possession, the close-on-exec split, the holder in
transit, the channel's life bound to the far process, and closure never read as an
answer. What follows is this seam's own.

**Admin creates the pair, before the unit starts, because admin is the only party that
exists before the unit starts.** This is the creating-party rule of
`weaver-organ-channel` section 2 landing on this seam, and the creating party is not
the initiating party.

**How the worker's end arrives is settled, per the ruling of 2026-08-01.** Admin forks
the worker under the sudo delegation of `weaver-admin-PRD` section 7, per the operator's
ruling of 2026-08-05, so the channel is a pair admin creates before that fork and the
worker inherits across it, nameless and therefore undialable, per that charter's
sections 6 and 10. Namelessness restates as unreachability: the name exists, the agent's
tool surface cannot traverse to it, and no second opener exists in practice because the
kernel denies the path lookup, which is the same property the unnamed pair bought and
the second-opener case `weaver-organ-channel` section 2 rejects stays rejected.

**No third party holds the end in transit,** the ruling of 2026-08-05 having
retired the intermediary this clause once named: admin creates the pair and the
worker inherits it, so `weaver-organ-channel` section 2's retention rule has two
parties and no carrier between them. It may
touch the end while placing it and does not retain one.

**Possession is this seam's authentication, per apex 5.1's second case.** The
invariant reads by credential where the channel has a name and by possession where
it has none, restated on 2026-07-31 with this seam as the case that forced it, so
the seam is an instance of the rule rather than an exception admitted to it.

**The worker holds the agent uid from its first instruction and clears its dumpable
flag after its final exec.** There is no drop, because the delegating party execs the
worker at `weaver-<n>` under the delegation `weaver-admin-PRD` section 7 rules. An
earlier form of this clause ordered a drop against the handoff, and the ordering had a
subject only while the worker began life holding a higher principal.

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
the sender's. The same holds for this channel's own descriptor, which
reaches the supervisor rather than being received by it, so the flag is set on admin's
end before the channel is handed across and set again after the worker's last exec. The
second is a set and not a check, because `execve` can clear the flag and a step that
reports rather than repairs leaves the channel inheritable by every tool subprocess.

**The channel lives exactly as long as the worker,** which is `weaver-organ-channel`
section 2's lifetime rule with its far process named. It is not reconnected, not
reopened, and not shared with a second worker.

## 3. The exchanges

Three, and no others, all opened by admin.

**Enter the run.** Opened by admin. Admin directs the harness to enter, supplying the
session identity, the run ordinal, the trace descriptor, the
model binding, and the gate instruction. The last two are in the directive because the
ruling of `weaver-admin-PRD` section 6 gives admin no channel to the SPU or the gate, so
if admin's intent for either does not cross this seam it crosses nowhere. The harness
stands up an empty working structure, authors its `load` event, asks the SPU to
admit the model binding it was handed, and starts Gate last. It answers ready only when
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
through the operator surface with a verb, per `weaver-admin-operator-contract`
section 6 and the basic loop's section 2. Admin learns nothing of a fault, holding
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

- Enter is first and happens exactly once on a channel.
- Stop is valid only between a completed enter and a leave, and a stop arriving at
  rest answers at rest rather than refusing.
- An organ fault before the enter aggregate is answered is a refusal on the enter
  exchange naming the arm, rather than a `fault` event, the report to admin and
  the account on the stream being two different things.
- Leave is last, happens at most once, and is terminal on the channel.
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

**Admin supplies** the session identity and run ordinal for the run being entered, the
trace descriptor, the model binding the fan-out admits,
the gate instruction the fan-out starts, and the intent to stop.

**Admin guarantees** that the trace descriptor it passes refers to the sink the
session's configuration declares, that the run ordinal is the next one for that
session, and
that the boundary the worker runs inside exists and is correct, because
admin verified it before the unit started and is the only party positioned to. The
guarantee is of verification rather than of authorship, since the boundary is the
operator's artifact. It guarantees that no directive carries work of any kind.

**The harness supplies** its readiness as the aggregate of the enter fan-out, its
confirmation of departure, and the turn's fate on a stop.

**The harness guarantees** that every descriptor it accepts is accepted close-on-exec,
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
`lifecycle-answer`, `lifecycle-refusal`.

```graph
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

**Drawn from `weaver-traits`:** nothing. The clause is present with that answer
because `weaver-types-PRD` section 5 asks for it even when it is empty.

**`organ-envelope` belongs to the floor and not to this seam,** because it is the
carrier every organ contract draws rather than a thing admin and the harness agreed on
between themselves. It is named here because this was the first contract to need it. The
definition stays in `weaver-types` and the mechanics it serves live in
`weaver-organ-channel`, per `weaver-types-PRD` section 2.3.

**`peer-identity` and `authorization-predicate` are not drawn here, and the negative is
stated rather than left to the absence of an edge.** This seam authenticates by
possession per section 2, so it reaches neither definition. `weaver-types-PRD` section
2.2 rests its scoped claim on this contract being the counterexample to a universal,
and a claim about what another document says is only checkable if that document says
it.

**Drawn from `weaver-trace`:** nothing. No event kind, envelope field, or payload
shape crosses this seam, and this contract names no field of the record's envelope.

**The five definitions land in `weaver-types` and are owed by this act.**
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
- The G4 union grows from three values to five, because the duplex rewrite of this
  contract draws `organ-envelope` and `harness-alert` where the simplex form drew
  three. The count of contracts is unchanged, both crates remain party to one, and an
  earlier form of this line said the union was unchanged by reading the first fact for
  the second.

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
