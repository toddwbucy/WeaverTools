# weaver-harness / weaver-gate - contract

**Status:** MERGED. In `main` and the source of truth, per the human's ruling
of 2026-08-01 that a document on `main` is merged and not a draft. This content
replaces the placeholder of 2026-07-29 at its own name, which is the consumption the
v0.6 stub ruling shapes. It governs the lifecycle half of this seam, the raise and
the lower. The exchanges that carry work arrive with the token workflow.

**Date filed:** 2026-07-31
**Revised:** 2026-08-22, the refusal on this seam is clerked. The organ's
refusal travels as it did and the harness authors the record's `refusal`
carrying `lifecycle-refusal`, per `weaver-trace-PRD` section 3.1's
twenty-first kind and the operator's ruling of this date. The organ authors
nothing and no exchange changes.
**Revised:** 2026-08-18, the tool boundary ruling re-grounds the execution
exchange. The gate holds one tool, the shell, which is this crate's own
outbound verb: the agent's effect on the world crosses this membrane, and the
shell is that crossing's general form, bounded by the uid that is the agent's
outer protective shell per `weaver-tools-vision` section 6. The calculator
leaves for `weaver-internal`, dispatched inward and never this seam's
traffic. The answer's contents grow from three to four, killed on the clock
joining as its own case, and the invocation carries the caller's clock under
the one-clock rule this act states. The `tool-trait` draw retires: with one
held tool and no dyn table this seam's execution speaks through no trait, and
the trait stays chartered at `weaver-traits-Spec` section 5 for the elected
outward corner it always constituted.
**Revised:** 2026-08-17, the tool workflow opens and the fifth exchange arrives,
the one section 2 promised to it: execute a tool, opened by the harness,
carrying the call the family parse recovered, answered with the result or the
tool's own failure. The gate executes per the mechanism election recorded at
`weaver-gate-PRD` section 7: the loop reaches tools through the gate, per the
ratified boundary of 2026-08-11, so the fork lives on the gate's side of the
loop's membrane and the result crosses back as an answer on this seam.
Section 7 draws `tool-name` and, with the traits charter's unblocking in the
same act, `tool-trait`. Owed by issue #116's arc.
**Revised:** 2026-08-07, the second seam is admitted. This document forbade a second
listener, which the egress ruling of the same date reversed at the apex, and until now
it governed against that ruling under Document Format section 7. The gate holds the two
seams its instruction names. **The exchange that reaches the second seam is not
enumerated here**, arriving with the tool workflow, so this act removes a contradiction
and shapes nothing. **The apex named this document as owed and this discharges it.**
**Document ID:** `weaver-harness-gate-contract`
**Parent:** `weaver-agents-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The agreement over the seam between the interior coordinator and the agent's mouth
and ears: what crosses it, what each crossing means, what each party may rely on, and
how it fails. It is read alongside `weaver-harness-PRD` and `weaver-gate-PRD`, and
none of the three is complete without the other two.

It carries no representation. The types it names have a definition site and no field
list here, the ordering it fixes is stated as a rule rather than as a state machine,
and how any of it is encoded is the Spec's.

**The seam has two initiators and there is one document for it.** Either party may open
an exchange. That is not a feature this seam happens to have, it is what makes the gate
an organ under apex section 5.4, which requires a domain and a two-initiator channel
with the harness, both properties and not either. The gate's domain is the agent's
external boundary, its job simplified by the demotion and its standing unchanged.
This is an organ channel, so sections 1 and 2 draw `weaver-organ-channel` the way the
coordination and residency contracts do, keeping only what is this seam's own.

**The gate's own direction arrived with the token workflow's act of
2026-08-02.** The lifecycle pair stays the harness's, and the gate now opens
the turn exchange that carries a client's work inward and the fault report,
the channel carrying two initiators exactly as this section anticipated. The
name records the initiator of the governed lifecycle signals and stands, per
`weaver-gate-PRD` section 6.

```graph
node: weaver-harness-gate-contract
kind: document

edge: party
from: weaver-harness-gate-contract
to: weaver-harness

edge: party
from: weaver-harness-gate-contract
to: weaver-gate

edge: draws
from: weaver-harness-gate-contract
to: organ-envelope

edge: draws
from: weaver-harness-gate-contract
to: gate-instruction

edge: draws
from: weaver-harness-gate-contract
to: lifecycle-refusal

edge: draws
from: weaver-harness-gate-contract
to: lifecycle-directive

edge: draws
from: weaver-harness-gate-contract
to: lifecycle-answer

edge: draws
from: weaver-harness-gate-contract
to: turn-frame

edge: draws
from: weaver-harness-gate-contract
to: fault-report
```

**The seam edge is declared by the organ and appears in `weaver-gate-PRD` section
6.** On an organ channel the organ declares and the harness does not, per Document
Format section 4. This document names its parties and does not restate the edge.

## 1. The channel, and what is this seam's own

`weaver-organ-channel` states the elected mechanics. Four facts are this seam's.

**The creating party is the harness and the pair is unnamed.** The harness creates a
connected pair during the enter fan-out, after the SPU has confirmed residency, and
forks the gate holding one end, the same act the residency seam uses. The pair is
created and inherited inside one act, with no third process holding an end in transit.

**Possession is the authentication on this pair, and the named socket this seam
raises is the opposite case on purpose.** The interior channel has no name and no
second opener. The client socket the gate binds is named and dialable, which is why
it authenticates every connection by credential under the boundary predicate of
`weaver-gate-PRD` section 2. One seam, both of apex section 5.1's cases, each where
its argument holds.

**The channel lives exactly as long as the gate process.** It is not reconnected,
not reopened, and not shared with a second gate. A harness that observes closure has
lost the agent's reachability, not a connection to it, and section 5 says what that
means. A gate that observes closure closes its listener and exits, which is how a
gate process never outlives the interior it fronts.

**The fork carries this end and nothing else.** At the moment of the gate fork the
harness holds the trace descriptor, its channel to admin,
and its channel to the SPU, every one close-on-exec, the received ones by the receive
rule and the created ones from creation, per `weaver-admin-harness-contract` section
5 and `weaver-harness-spu-contract` section 1. That discipline is what keeps all of
them out of the gate's process: **the gate receives this seam's end and no other
descriptor, and a build in which the gate holds a trace, coordination, or
residency handle is broken whether or not it uses one.** The gate sets its own end
close-on-exec after its final exec and clears its dumpable flag in the same act, per
`weaver-gate-PRD` section 7.

## 2. The exchanges

Five, and no others in this pass. Three are opened by the harness, raise, lower,
and the tool execution as of the tool workflow's opening act of 2026-08-17, and
two by the gate, the turn and the fault report, which is the two-initiator
channel carrying both directions as of the token workflow's gate act. **The
egress ruling of 2026-08-07 gave the gate a second seam toward the world**, the
agent-opened socket for registered applications, and no exchange of this seam
reaches it in this pass: the shell executes inside the gate, the one tool it
holds as of the tool boundary ruling of 2026-08-18, and the egress socket
waits on a registered application to address.

**Raise the hook.** Opened by the harness, last in the enter fan-out, carrying the
gate instruction admin supplied in the enter directive, uninterpreted by the harness.
The gate resolves the instruction, binds the socket it names, and answers ready, or
it refuses. **Ready is sent only after the bind has returned,** so the harness's own
ready answer to admin rests on a bound listener and never on a starting process. The
refusal carries a reason the harness places in the enter aggregate without
translation, which is what makes `weaver-admin-harness-contract` section 6's
refusing-organ case one refusal rather than a report to parse. The answer, either
way, closes the exchange.

**Lower the hook.** Opened by the harness, first in the leave fan-out. The gate
closes the listener and answers stopped. **Stopped is sent only after the close has
returned,** so nothing new can arrive anywhere in the interior once the harness
proceeds, which is what stopped-first protects. Drain is modest by
construction, per the token workflow's act: leave waits on rest, so no turn
is in flight at a lower, and the gate closes its accepted connections after
the listener, holding nothing that needs finishing.

**Carry a turn.** Opened by the gate, one exchange per client request, from
the token workflow's act of 2026-08-02. The gate relays the client's line
inward as a `turn-frame`, opaque octets it has not read, and the exchange's
identity, the opening party and its ordinal per the channel's own mechanics,
is the correlation the response returns on. The harness interprets the line,
runs the turn, and answers with the response frame, which the gate relays to
the client by the path the request took. The gate carries no `turn_key`
inward and mints nothing, the turn not existing until the harness opens it,
per `weaver-gate-PRD` section 9. A line the harness cannot parse answers as
a refused turn inside the frame, content to the client and an ordinary
answer on this seam, per `weaver-gate-world-contract` section 2.

**Turns serialize at the harness, and the ruling is ratified with this
act.** The channel's layer permits concurrent exchanges, so the gate opens
one per request as clients speak, and the harness serves one turn at a time
in arrival order, answers returning as turns close. Order per connection is
the gate's own relay discipline, per the world contract, and no promise is
made across clients. A gate that refused instead of relaying while a turn
was in flight would push the one-turn loop's shape onto every client as an
error surface, where waiting is what a conversation already means.

**Report a fault.** Opened by the gate, carrying a `fault-report` naming a
case of `weaver-gate-PRD` section 13.4 that arose outside any exchange, the
listener lost above all. A failure inside a turn exchange, the client gone before its
response above all, is that exchange's own account instead. The harness
authors what it is handed as the `fault` event, per the fault-carrier
ruling, and answers received, promising authorship and nothing else.

**Execute a tool.** Opened by the harness, one exchange per call the model's
emission recovered, valid only inside the raised window like the turn. The
harness supplies the call - the tool's name, its arguments as the family
parse recovered them, and the caller's clock - and interprets nothing.

**One clock, the caller's.** Every invocation carries a timeout the caller
states. The tool declares a maximum, the caller's number must be equal to or
less than that maximum, and the gate adopts the caller's number as the kill
clock for that invocation. A clock beyond the maximum refuses rather than
clamps, because a clamped clock silently changes the caller's wait and the
two-clock disagreement returns by the back door. The tool cannot outlive the
wait, so no orphaned process survives the exchange and no stale return
arrives at a decision point that is already gone.

**Every opened execution completes with an answer, and the answer carries
one of four contents**, told apart by tag alone, the rule beneath the four
being who speaks in the return:

- **Result.** The invocation ran and the tool answers in its own words. For
  the shell, a nonzero exit status is a result and not an error: the exit is
  the shell's answer, accounted in content.
- **Refused.** Nothing ran, and the gate speaks in its own voice: a name it
  does not hold - never a nearest match, the registry discipline of the
  family table applied one organ over - malformed arguments, or a clock
  beyond the declared maximum. No side effect exists and a corrected call is
  safe to re-ask.
- **Errored.** The invocation machinery failed - the fork, a pipe, the
  supervisor - and the account's speaker is the infrastructure, never the
  tool.
- **Killed.** The caller's clock expired and the gate killed the invocation's
  whole process group, descendants included, because a kill that reaped only
  the leader would leave the pipes held open and the promise above false.
  The case carries no account from the tool by construction - the absence of
  the tool's words is the fact - and output drained before the kill rides
  the case as an attachment, never folded into a result, so a partial cannot
  masquerade as an answer.

All four are content rather than channel faults, per the layer split every
seam of this program runs, because each is a fact the model must learn: a
tool that does not exist is an answer the next turn reasons over, not a hole
in the conversation.

**Executions are serial and ordered by the emission.** The harness opens at
most one at a time, in the order the family parse recovered the calls, and
the next opens after the previous answered. Correlation is the channel's own
exchange identity, opening party and ordinal, so no call identifier rides
the payload, and the conversation's ordering is the emission's: the harness
authors the tool-result turns in the order it opened the executions, which
is the order the model spoke them.

**The answer crosses this seam exactly once per call and nothing else mints
what it grants**, which is the half of the loop closure this contract
carries: the harness's granted tool-result value is constructed at this
exchange's completion and nowhere else, whichever of the four contents
arrived, per `weaver-harness-Spec` section 6.

**The instruction crosses once, in the raise.** It is not re-sent, revoked, or
replaced. A gate that needs an instruction it was not given has a failed raise rather
than a second request to make.

**No exchange carries work, and no exchange carries a path this crate did not need.**
The instruction names the sockets the gate must hold, which is the one name this seam
exists to deliver, operator-declared and admin-validated. The gate learns no trace
path, no record name, and nothing of the interior.

**No exchange carries turn context, because no exchange here belongs to a turn.**
The same scoping the coordination and residency seams carry, filed once at
`weaver-admin-PRD` section 11, and this contract is a further case of that edit
rather than a new one. A tool call belongs to a turn and would carry it under apex
section 5.2, which is one of the things the tool workflow settles when it charters
that exchange.

## 3. Ordering

- Raise is first and happens exactly once on a channel.
- Lower is last, happens at most once, and is terminal on the channel.
- A lower with no completed raise before it is refused and is not queued, because
  there is no listener for it to close.
- Turn exchanges, tool executions, and fault reports are valid only between a
  completed raise and a lower, the window being the raised hook.
- More than one turn exchange may be open at once, the harness serving them
  one at a time in arrival order, per section 2.
- Messages within one exchange are ordered.
- A directive that arrives out of this order is refused and is not queued.
- An answer to raise arrives only after the bind has returned, and an answer to
  lower only after the close has returned, so each answer is a fact about the
  listener rather than a statement of intent.

**Closure is not an answer, per `weaver-organ-channel` section 2, and it is not
restated here.** What a closure means on this seam is section 5's.

## 4. What each party supplies and guarantees

This section is derived from section 2 rather than prose beside it, because every
exchange payload change is a supplies change by construction, and a Spec writer reads
this list.

**The harness supplies** the gate instruction it was handed in the enter directive,
the directive to lower, and, per call, the tool execution's name, arguments
as the family parse recovered them, and the caller's clock, uninterpreted,
opened serially in emission order.

**The gate guarantees** an answer to every execution opened inside the raised
window, carrying one of the four contents of section 2, so the harness's
grant construction site fires exactly once per opened call and a call never
dangles. It guarantees the kill clock is the caller's number and the kill
reaches the invocation's whole process group.

**The harness guarantees** that the instruction it sends is the instruction admin
sent it, unaltered and uninterpreted. It guarantees that it opens no exchange this
document does not enumerate. It guarantees that it creates this seam's channel and
passes its descriptor across the fork and no other, every descriptor it holds being
close-on-exec at that moment, per section 1. It guarantees that it raises the gate
last and lowers it first within the fan-outs, per apex section 6. It guarantees that
it does not treat an answer as authorization for anything beyond the exchange that
produced it.

**The gate supplies** its confirmation of ready, its confirmation of stopped, its
refusal with the reason, the turn frames it relays inward, and its fault
reports.

**The gate further guarantees**, from the token workflow's act: that every
frame it relays crossed its predicate's admission, that order holds per
connection in both directions, that it opens one exchange per client request
and correlates the response to it, that it reads no frame and retains none
past its answer, and that a delivery it cannot complete becomes the turn
exchange's own account rather than silence.

**The harness further guarantees** that it serves turn exchanges one at a
time in arrival order, and that every answer is the turn's close rendered as
the frame the world contract fixes, clean or stopped with its kind named.

**The gate guarantees** that ready follows the bind and stopped follows the close. It
guarantees that a refusal leaves nothing held, no listener and no half-bound socket,
so a refusal is true about the boundary rather than merely true about the attempt. It
guarantees that it answers a refusal rather than exiting on one. It guarantees that
it admits only the principals the boundary predicate names and that the predicate
excludes the agent uid, per `weaver-gate-PRD` section 2. It guarantees that it
authors no trace event, holds no descriptor beyond this seam's end and its listener,
retains nothing across a raise and a lower, and exits on observing closure.

## 5. Failure

Refusals are typed and enumerable, and every one of them is the gate refusing an ask,
because the harness answers nothing on this seam. The cases:

- the instruction does not resolve to a socket this crate can bind
- the bind fails, with the reason carried
- the directive is out of order for the channel's state

**The refusal reuses `lifecycle-refusal`,** so the enter aggregate carries it
unchanged, and whether that type's case set grows to hold bind failure rides the cell
`weaver-spu-PRD` section 10 holds.

**A refusal leaves the gate in the state it was in before the directive,** which for
a refused raise is nothing held at all, so admin's rollback treats a refusal from
this arm as needing nothing undone here.

**A gate that dies has refused nothing, and what the harness reports depends on
when.** Before the enter aggregate is answered, the death is a refusal on the enter
exchange naming this arm, per the rule `weaver-admin-harness-contract` section 4
applies to a fault before ready. After the aggregate, the death is the loss of the
agent's reachability, observed through closure and authored to the stream as the
`fault` event, per the fault-carrier ruling of 2026-08-01, the operator's tooling
keying on it there.

**Nothing on this seam retries.** A refused directive returns to the harness, which
unwinds, and a re-sent directive would put two attempts behind one operator intent.

**A refusal on this seam is clerked, 2026-08-22.** Per the operator's ruling
that a refusal is clerked in one kind for every seam. The organ's refusal
travels to the harness, and the harness authors the record's `refusal`
naming this seam and carrying `lifecycle-refusal`, per `weaver-trace-PRD`
section 3.1's twenty-first kind. **The organ authors nothing**, which is
unchanged, and the refusal still reaches admin in the enter aggregate as it
did.

**Where the refusal falls before the run's bracket stands it reaches no
record**, the `load` event being what opens the run. That is named at
`weaver-admin-harness-contract` and is not closed by this act.

## 6. Prohibitions

**On the harness.** It opens no exchange this document does not enumerate. It does
not alter or interpret the instruction. It does not pass a descriptor across the gate
fork other than this seam's. It does not treat the gate as a peer of the organs it
sequences: the gate confirms inside the aggregate like every other arm of the
fan-out.

**On the gate.** It opens no exchange this document does not enumerate, the
turn exchange and the fault report being its two. It reads
no content and translates nothing, in either direction. It authors no trace event and
holds no descriptor to the record. It dials no interior socket and
holds no channel to `weaver-admin` or the SPU. **It holds the two seams the
instruction names and no third**, a count the egress ruling of 2026-08-07 raised from
one, and which end binds the agent-opened seam is that seam's own contract's to say,
per `weaver-gate-PRD` section 2. What this prohibition still forbids is a surface the
gate opens on its own judgment. It retains nothing about a turn after the response
returns.

**On both.** Neither party carries a fact about the other's interior. The harness
does not know who is connected and the gate does not know what a turn contains, and
the exchanges above are the whole of what either learns in this pass.

## 7. Vocabulary

Every contract names the vocabulary it depends on, grouped by the crate that defines
it, and a group is stated even when empty.

**Drawn from `weaver-types`:** `organ-envelope`, `gate-instruction`,
`lifecycle-directive`, `lifecycle-answer`, `lifecycle-refusal`, and
`turn-frame` and `fault-report`, the last two from the token workflow's act.
`turn-frame` is one definition for both directions, a client line carried
opaque, inward as the ask and outward as the answer, refusals riding inside
it as content the harness authored, named for the seam's currency under the
naming ruling's ratified extension. `fault-report` is what section 2's fault
exchange carries, and it is drawn rather than invented here because every
reporting seam hands the harness the same fact and the harness authors all of
them into one event kind. Both are owed to `weaver-types-PRD` section 2.3 by
this act and land with it, entering the envelope's payload as the variants
that carry them, the frame under that enum's later-loop rule and the fault
report on the different ground that Spec's section 4.1 states.

`organ-envelope` is the carrier every organ channel draws, drawn here as the
coordination and residency contracts draw it.

`gate-instruction` is a field of the agent's configuration file, defined at
`weaver-types-PRD` section 2.1 beside `model-binding` per `weaver-gate-PRD` section
10, drawn here because it is what crosses and not interpreted here beyond being
carried. The operator writes it, admin validates it, the harness carries it, the gate
resolves it.

`lifecycle-refusal` is drawn rather than twinned, per section 5.

`tool-name` is drawn as of the tool workflow's opening act: the execution
exchange carries it, and it is the same definition the family parse mints
from an emission, so the name that crosses is the name the model spoke.

**Drawn from `weaver-traits`:** nothing, as of the tool boundary ruling of
2026-08-18. The act of 2026-08-17 drew `tool-trait` here on the reading that
the trait was the gate's executor surface, and the ruling retires that
reading: the gate holds one tool, the shell, its own verb dispatched with no
table, and `weaver-traits-Spec` section 5's constituency sentence already
confined the trait to the elected outward corner - the registered service the
egress seam awaits - which no exchange of this seam reaches. The trait stays
chartered where it is for the corner it constitutes, and this clause draws it
again on the day that corner's exchange arrives here or on the egress seam's
own contract.

```graph
edge: draws
from: weaver-harness-gate-contract
to: tool-name
```

**This seam draws loop 0's trio and owes the floor nothing, per the naming ruling
of 2026-08-01.** Wire vocabulary is named for the loop whose traffic it carries,
and this contract draws the cases that cross its seam: raise and lower on the
directive, ready and stopped on the answer, this seam's refusals on the refusal.
The seam-owned reading an earlier version chose to prevent drift is answered at
its root rather than kept: the closed case sets have one owner, the floor, and
contracts draw rather than grow them, so the drift two independent enumerations
invited cannot occur.

**Drawn from `weaver-trace` and `weaver-harness`:** nothing. The gate reports and the
harness authors, and no event kind or envelope field crosses this seam. The
turn frame is the floor's definition above rather than the trace's, and what
the record holds of a turn is authored inside, on the other side of this
seam.

## 8. What this document changes elsewhere

Named here because a document whose reach cannot be read for the reach is a trap.
These are owed by this act, and `weaver-gate-PRD` section 11 is the authoritative
register under G5.

- `weaver-types-PRD` sections 2.1 and 2.2, per that register: the instruction
  field and the predicate's consumer citation, both landed. The seam pair once
  owed to 2.3 dissolved with the naming ruling of 2026-08-01, the loop trio
  covering it.
- `weaver-harness-PRD` section 4: the sentence holding turn ingress open until this
  crate is chartered resolves by pointing at this contract, gaining no record there,
  the organ declaring in its own charter. On merge.

**What this act closes.** The fault cases a running hook raises are named at
`weaver-gate-PRD` section 13.4, and with them the corpus-wide case set behind
the `fault` event closes across all three organs: the SPU's at its section
13.10, the gate's at its 13.4, and the harness's three at
`weaver-harness-PRD` section 5, landed in this same act. The payload's shape
lands with the trace act against the closed set.
