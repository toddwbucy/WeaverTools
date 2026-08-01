# weaver-harness / weaver-gate - contract

**Status:** MERGED. In `main` and the source of truth for now, per the human's ruling
of 2026-08-01 that a document on `main` is merged and not a draft. This content
replaces the placeholder of 2026-07-29 at its own name, which is the consumption the
v0.6 stub ruling shapes. It governs the lifecycle half of this seam, the raise and
the lower. The exchanges that carry work arrive with the token workflow.

**Date filed:** 2026-07-31
**Revised:** 2026-07-31. The live-view write end leaves the fork enumeration and the
prohibitions, the live view retired under ruling A of the subtraction batch.
**Revised:** 2026-08-01. The status moves from draft to merged on the human's ruling
of this date, with the register landings recorded at `weaver-gate-PRD` section 11.
The client socket this seam raises gains its own contract, `weaver-gate-world-contract`,
written the same date.
**Revised:** 2026-08-01, again, the fault-carrier ruling. A gate death after the
aggregate is authored to the stream as the `fault` event rather than handled in a
shape a pending ruling would give it, and section 8's deferral restates against
the event kind's case set.
**Revised:** 2026-08-01, further, the naming ruling. This seam draws loop 0's
trio, the seam-owned pair it awaited dissolving with the sender convention, and
section 7's drift defense restates against the floor's single ownership.
**Document ID:** `weaver-harness-gate-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
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

**The seam is duplex and there is one document for it.** Either party may open an
exchange. That is not a feature this seam happens to have, it is what makes the gate
an organ under apex section 5.4, which requires a domain and a duplex channel with
the harness, both properties and not either. The gate's domain is the agent's
external boundary, its job simplified by the demotion and its standing unchanged.
This is an organ channel, so sections 1 and 2 draw `weaver-organ-channel` the way the
coordination and residency contracts do, keeping only what is this seam's own.

**Both of this pass's exchanges are opened by the harness.** The gate's own
direction, the turn exchanges that carry a client's work inward, arrives with the
token workflow, deferred rather than forbidden, the same half-chartered shape the
residency seam carries. The name records the initiator of the governed signals and
stands when that direction arrives, per `weaver-gate-PRD` section 6.

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
harness holds the trace descriptors, its channel to admin,
and its channel to the SPU, every one close-on-exec, the received ones by the receive
rule and the created ones from creation, per `weaver-admin-harness-contract` section
5 and `weaver-harness-spu-contract` section 1. That discipline is what keeps all of
them out of the gate's process: **the gate receives this seam's end and no other
descriptor, and a build in which the gate holds a trace, coordination, or
residency handle is broken whether or not it uses one.** The gate sets its own end
close-on-exec after its final exec and clears its dumpable flag in the same act, per
`weaver-gate-PRD` section 7.

## 2. The exchanges

Two, and no others in this pass. Both are opened by the harness.

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
proceeds, which is what stopped-first protects. In this pass no traffic exists, so
the close is the whole of it, and drain arrives with the token workflow.

**The instruction crosses once, in the raise.** It is not re-sent, revoked, or
replaced. A gate that needs an instruction it was not given has a failed raise rather
than a second request to make.

**No exchange carries work, and no exchange carries a path this crate did not need.**
The instruction names the socket the gate must bind, which is the one name this seam
exists to deliver, operator-declared and admin-validated. The gate learns no trace
path, no record name, and nothing of the interior.

**No exchange carries turn context, because neither exchange belongs to a turn.**
The same scoping the coordination and residency seams carry, filed once at
`weaver-admin-PRD` section 11, and this contract is a further case of that edit
rather than a new one.

## 3. Ordering

- Raise is first and happens exactly once on a channel.
- Lower is last, happens at most once, and is terminal on the channel.
- A lower with no completed raise before it is refused and is not queued, because
  there is no listener for it to close.
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
and the directive to lower.

**The harness guarantees** that the instruction it sends is the instruction admin
sent it, unaltered and uninterpreted. It guarantees that it opens no exchange this
document does not enumerate. It guarantees that it creates this seam's channel and
passes its descriptor across the fork and no other, every descriptor it holds being
close-on-exec at that moment, per section 1. It guarantees that it raises the gate
last and lowers it first within the fan-outs, per apex section 6. It guarantees that
it does not treat an answer as authorization for anything beyond the exchange that
produced it.

**The gate supplies** its confirmation of ready, its confirmation of stopped, and its
refusal with the reason.

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

## 6. Prohibitions

**On the harness.** It opens no exchange this document does not enumerate. It does
not alter or interpret the instruction. It does not pass a descriptor across the gate
fork other than this seam's. It does not treat the gate as a peer of the organs it
sequences: the gate confirms inside the aggregate like every other arm of the
fan-out.

**On the gate.** It opens no exchange this document does not enumerate, which in this
pass means it opens none, the turn direction deferred rather than forbidden. It reads
no content and translates nothing, in either direction. It authors no trace event and
holds no descriptor to the record. It dials no interior socket and
holds no channel to `weaver-admin` or the SPU. It binds no second listener. It
retains nothing about a turn after the response returns.

**On both.** Neither party carries a fact about the other's interior. The harness
does not know who is connected and the gate does not know what a turn contains, and
the exchanges above are the whole of what either learns in this pass.

## 7. Vocabulary

Every contract names the vocabulary it depends on, grouped by the crate that defines
it, and a group is stated even when empty.

**Drawn from `weaver-types`:** `organ-envelope`, `gate-instruction`,
`lifecycle-directive`, `lifecycle-answer`, and `lifecycle-refusal`.

`organ-envelope` is the carrier every organ channel draws, drawn here as the
coordination and residency contracts draw it.

`gate-instruction` is a field of the agent's configuration file, defined at
`weaver-types-PRD` section 2.1 beside `model-binding` per `weaver-gate-PRD` section
10, drawn here because it is what crosses and not interpreted here beyond being
carried. The operator writes it, admin validates it, the harness carries it, the gate
resolves it.

`lifecycle-refusal` is drawn rather than twinned, per section 5.

**This seam draws loop 0's trio and owes the floor nothing, per the naming ruling
of 2026-08-01.** Wire vocabulary is named for the loop whose traffic it carries,
and this contract draws the cases that cross its seam: raise and lower on the
directive, ready and stopped on the answer, this seam's refusals on the refusal.
The seam-owned reading an earlier version chose to prevent drift is answered at
its root rather than kept: the closed case sets have one owner, the floor, and
contracts draw rather than grow them, so the drift two independent enumerations
invited cannot occur.

**Drawn from `weaver-traits`:** nothing. The clause is present with that answer
because `weaver-types-PRD` section 5 asks for it even when it is empty.

**Drawn from `weaver-trace` and `weaver-harness`:** nothing. The gate reports and the
harness authors, and no event kind, envelope field, or frame crosses this seam in
this pass.

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

**What this document does not close.** The fault cases a running hook raises arrive
with the token workflow and land as `fault` events, the shape the fault-carrier
ruling of 2026-08-01 gave them, their case set closing with the organs' charters
per `weaver-spu-PRD` section 10.
