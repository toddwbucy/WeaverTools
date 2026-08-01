# weaver-harness / weaver-spu - contract

**Status:** MERGED. In `main` and the source of truth for now. Written with
`weaver-spu-PRD` as one act, per apex section 10. It governs the coordination seam
only, which is the residency half of what this pair will eventually agree on. The
exchanges that carry work arrive with the token workflow.

**Date filed:** 2026-07-31
**Revised:** 2026-07-31. The topology ruling landed as drafted, the SPU a child the
harness forks during the enter fan-out over a pair the harness creates before the
fork, and the markers that priced the ruling are removed.
**Revised:** 2026-07-31, second. The harness's own end of the channel carries
close-on-exec from the pair's creation, closing the tool-fork inheritance the review
found, and the possession sentence in section 1 recovers the argument the first
revision compressed away.
**Revised:** 2026-07-31, third. The residue framing rewrites to sole authority under
ruling C, admission being the one check on the device with nothing upstream
arbitrating, and the prohibitions carry the relocated no-evict binding.
**Document ID:** `weaver-harness-spu-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The agreement over the seam between the interior coordinator and the organ that holds
the model: what crosses it, what each crossing means, what each party may rely on, and
how it fails. It is read alongside `weaver-harness-PRD` and `weaver-spu-PRD`, and none
of the three is complete without the other two.

It carries no representation. The types it names have a definition site and no field
list here, the ordering it fixes is stated as a rule rather than as a state machine, and
how any of it is encoded is the Spec's.

**The seam is duplex and there is one document for it.** Either party may open an
exchange. That is not a feature this seam happens to have, it is what makes the SPU an
organ under apex section 5.4, which requires a domain and a duplex channel with the
harness, both properties and not either. This pass charters the exchanges the harness
opens. The direction the SPU opens is the fault it raises, and its case set is deferred
with the workflow that produces faults, per section 8.

**This document draws its channel from `weaver-organ-channel` rather than describing
one.** The mechanics of an organ channel, the envelope, the exchange as the unit, what
the layer does and does not provide, are the same for every organ the harness is duplex
with, and they are stated once in `weaver-organ-channel` rather than in each contract.
What stays here is what is specific to this seam, which is section 1.

```graph
node: weaver-harness-spu-contract
kind: document

edge: party
from: weaver-harness-spu-contract
to: weaver-harness

edge: party
from: weaver-harness-spu-contract
to: weaver-spu

edge: draws
from: weaver-harness-spu-contract
to: organ-envelope

edge: draws
from: weaver-harness-spu-contract
to: model-binding

edge: draws
from: weaver-harness-spu-contract
to: lifecycle-refusal

edge: draws
from: weaver-harness-spu-contract
to: harness-directive

edge: draws
from: weaver-harness-spu-contract
to: spu-answer
```

**The seam edge is declared by the organ and appears in `weaver-spu-PRD` section 6.**
This document names its parties and does not restate that edge, because an edge cannot
be the target of an edge and the two `party` records are what make the pair checkable
from this side.

## 1. The channel, and what is this seam's own

`weaver-organ-channel` states the mechanics. Four facts are this seam's.

**The creating party is the harness and the pair is unnamed.** The harness creates a
connected pair before it forks, and the child holds one end from its first instruction.
The harness creates because it is the party that exists first, which is the
creating-party rule of `weaver-organ-channel` section 2 landing on this seam, and it is
the ruled topology of 2026-07-31 rather than an assumption. This is apex section 5.1's
possession case with the harness as the creating party, and it is the simple form of
that case: the pair is created and inherited inside one act, with no third process
holding an end in transit and no interface question about how the far end travels. The
coordination seam above pays a cell for that question and this one does not.

**Possession is the authentication, per `weaver-organ-channel` section 2, and one fact
of it is this seam's.** A named socket admits any number of connections, and a dial
from the agent uid proves nothing about which process dialed, an elected `bash` tool
being one candidate. The unnamed pair removes the dial entirely, so the peer is the
exact process the harness forked, because holding an end is the only way onto the
channel.

**The channel lives exactly as long as the residency it is about.** It is not
reconnected, not reopened, and not shared with a second SPU. A harness that has lost
this channel has lost the residency, not a connection to it, and section 5 says what
that means.

**The fork carries this end and nothing else.** At the moment of the fork the
harness holds the trace descriptors and its channel to admin, both close-on-exec under
the receiver rule of `weaver-organ-channel` section 2, bound on that seam by
`weaver-admin-harness-contract` section 2 and `weaver-harness-PRD` section 5. That
discipline is what keeps them out of the SPU's process, and this contract binds the
harness to it here rather than leaving the reader to derive it: **the harness passes
this seam's descriptor across the fork and no other, and every descriptor it holds on
another party's behalf is close-on-exec at the moment it forks.** The obligation is the
harness's because the harness is the only party that can meet it, in the same way
close-on-exec at the receive is the harness's obligation on the seam above and not
admin's. The SPU clears its own dumpable flag after its final exec, per `weaver-spu-PRD`
section 7, which stops a same-uid process from attaching to it and driving this channel.
The SPU also sets close-on-exec on its own end of this channel after that same final
exec, and the step is a set rather than a check, per `weaver-organ-channel` section 2,
so that no subprocess a later workflow spawns from the SPU inherits the seam. The
harness's own end of this channel carries the same flag from the moment the pair is
created, before any fork rather than after one. The flag fires at exec, so the copy of
that end the SPU fork carries closes at the SPU's own final exec, which is what keeps a
second end of this seam out of the SPU's address space, per `weaver-spu-PRD` section
7's rule that the crate receives the channel end and no other descriptor. And it is
what keeps this end out of every tool the harness elects for the life of the seam,
since the harness forks a subprocess per tool call and a descriptor without the flag
survives every one of those forks, an inherited end there handing the tool surface a
release directive on the residency seam.

## 2. The exchanges

Two, and no others in this pass. Both are opened by the harness.

**Admit the model.** Opened by the harness during the enter fan-out, carrying the model
binding admin supplied in the enter directive. The SPU resolves the binding, reads what
the artifact declares about itself without loading it, judges the device against what
admission requires, takes the device, and loads the weights. It answers residency
confirmed, or it refuses. **The refusal carries a reason the harness places in the enter
aggregate without translation,** which is what makes `weaver-admin-harness-contract`
section 6's refusing-organ case one refusal rather than a report to parse. The answer,
either way, closes the exchange.

**Release the model.** Opened by the harness during the leave fan-out. The SPU stops
serving against the residency, frees the device, and answers released. **The
confirmation is given after the device is free and never before it,** so a confirmation
is a fact about the device rather than a statement of intent, and admin's binding rule
that a load never auto-evicts an occupant is not defeated by an occupant that is gone
but unconfirmed.

**A confirmation confirms and carries no payload.** Nothing in this workflow consumes a
description of the admitted model, and the candidates that exist reach the record
elsewhere, per `weaver-spu-PRD` section 4.4. What that leaves open about the record is a
cell in that charter's section 10 rather than a field added here against no reader.

**The binding crosses once, in the admit exchange.** It is not re-sent, not revoked, and
not replaced. An SPU that needs a binding it was not given has a failed admission rather
than a second request to make, because there is no exchange in which it asks for one.

**No exchange carries a path.** The harness sends the binding it was handed and the SPU
resolves it against what it can reach, and neither party learns a trace path or a record
name from the other. This is the descriptor discipline of `weaver-harness-PRD` section 5
reaching a second seam.

**No exchange carries turn context, because neither exchange belongs to a turn.** Apex
section 5.2 has every request crossing a seam carrying the trace context of the turn it
belongs to. A residency directive belongs to a load rather than to a turn, so it is a
counterexample to the invariant as written, and it is the same counterexample the
coordination seam already is. `weaver-admin-PRD` section 11 files the scoping edit, and
this contract is the second case that edit has to cover rather than a second edit.

## 3. Ordering

- Admit is first and happens exactly once on a channel.
- Release is last, happens at most once, and is terminal on the channel.
- A release with no completed admit before it is refused and is not queued, because
  there is no residency for it to end.
- Messages within one exchange are ordered.
- A directive that arrives out of this order is refused and is not queued.
- An answer to admit arrives only after the device holds the weights, so the harness may
  rely on a confirmation meaning the model is resident rather than loading. The reliance
  is exactly as large as the sequence, and it is what the harness's own ready answer to
  admin rests one arm of the enter aggregate on.
- An answer to release arrives only after the device is free, so the harness may rely on
  a confirmation meaning the device is available to the next load.

**Closure is not an answer, per `weaver-organ-channel` section 2, and it is not
restated here.** What a closure means on this seam is section 5's. The reason the rule
varies by no organ shows plainest on this one: a synthesized success is the one failure
mode that produces a load published as complete against an interior that is not.

## 4. What each party supplies and guarantees

This section is derived from section 2 rather than prose beside it, because every
exchange payload change is a supplies change by construction and a Spec writer reads
this list.

**The harness supplies** the model binding it was handed in the enter directive, and the
directive to release.

**The harness guarantees** that the binding it sends is the binding admin sent it,
unaltered and uninterpreted, because a harness that adjusted a binding would put a
second reading of the agent's configuration between the operator's declaration and the
device. It guarantees that it opens no exchange this document does not enumerate. It
guarantees that it creates this seam's channel and passes its descriptor across the
fork, and that every descriptor it holds on another party's behalf is close-on-exec at
that moment, per section 1. It guarantees that its own end of this channel is
close-on-exec from the pair's creation, per section 1, so that no process it forks for
the life of this seam inherits it. It guarantees that it does not treat a confirmation
as authorization for anything beyond the exchange that produced it.

**The SPU supplies** its confirmation of residency, its confirmation of release, and its
refusal with the reason.

**The SPU guarantees** that a confirmation of residency is given only after the device
holds the weights, and a confirmation of release only after the device is free. It
guarantees that a refusal leaves no device memory held on this residency's account, so
that a refusal is true about the device rather than merely true about the attempt. It
guarantees that it answers a refusal rather than exiting on one, because a party that
exited would replace a typed reason with an observed death. It guarantees that it
authors no trace event and holds no descriptor to the record. It guarantees that its own
end of this channel is close-on-exec after its final exec, per section 1, a set rather
than a check. It guarantees that it retains nothing across a residency, so a released
device is released whole.

**Neither party guarantees the device against what it did not put there.** A device
occupied by something outside this program is caught at admission and nowhere
earlier, the SPU being the one authority on the device per `weaver-spu-PRD` section
2. What neither
party can guarantee is that such an occupant does not arrive between the judgment and
the allocation, so an admit that passes its device check and fails its allocation is a
real case rather than a contradiction, and section 5 names it.

## 5. Failure

Refusals are typed and enumerable, and every one of them is the SPU refusing an ask,
because the harness answers nothing on this seam. The cases:

- the binding does not resolve to an artifact this crate can reach
- the artifact does not parse, or declares a shape this crate cannot serve
- the device cannot take what admission requires
- the directive is out of order for the channel's state
- there is no residency to release

**The set is open and its exit is named.** `weaver-spu-PRD` section 10 holds the cell,
carrying the candidate list the old tree's device authority offers as mechanics, and the
cases above are the shape rather than the enumeration. What this contract binds is that
every refusal is typed, that it names its case, and that the harness can carry it into
the enter aggregate unchanged.

**A refusal leaves the SPU in the state it was in before the directive.** A refused
admit means no model was admitted and no device memory is held, which is what keeps a
refused load from leaving a device occupied against the next admission.

**An admit that fails after taking the device frees it before answering.** The two
failures are one refusal to the harness and two different obligations on the SPU, and
naming only the first would leave the expensive case unstated.

**An SPU that dies has refused nothing, and what the harness reports depends on when.**
The harness observes the process exit and the channel closure together, and no typed
reason accompanies either. Before the enter aggregate is answered, the harness carries
the death upward as a refusal on the enter exchange naming this arm, because there is no
run yet for a fault to belong to, which is the same rule `weaver-admin-harness-contract`
section 4 applies to a fault before ready. The refusal is the harness's report, not the
SPU's answer. After the aggregate is answered, the death is a fault the worker survived,
and what the harness does with it is the deferred alert case of section 8.

**A release that cannot be confirmed is reported unconfirmed.** The harness does not
synthesize a release from a closure and does not retry one. What makes this survivable
is that the device is reclaimed when the process exits and admin's own stop of the
unit follows the leave answer regardless, so an unconfirmed release is a reporting
failure rather than a leaked residency.

**Nothing on this seam retries.** A refused directive returns to the harness, which
unwinds along the same seams it fanned out on and returns the refusal to admin. A
harness that re-sent a directive after a refusal would put two attempts behind one
operator intent.

## 6. Prohibitions

**On the harness.** It opens no exchange this document does not enumerate. It sends no
path. It does not alter the binding it was handed. It does not ask the SPU to author an
event on its behalf, because it is the sole writer of the trace and an organ that
authored would end that property. It does not treat a directive as authorization for
anything beyond the directive. It does not pass a descriptor across the fork other than
this seam's.

**On the SPU.** It authors no trace event and holds no descriptor to the record. It
opens no exchange this document does not enumerate, which in this pass means it opens
none, and the direction reserved for it is deferred rather than forbidden. It reaches no
other crate and holds no channel to `weaver-admin`, because admin holds one seam and
what this crate has to say to admin travels through the harness as hub. It does not read
the agent's configuration file. It does not decide whether a load should happen, and
it evicts no occupant, refusal being admission's only answer to a conflict, per the
relocated binding of apex section 6. It binds no socket. It
retains nothing across a residency.

**On both.** Neither party carries a fact about the other's interior. The harness does
not know how the device is allocated and the SPU does not know what a turn contains, and
the exchanges above are the whole of what either learns in this workflow.

## 7. Vocabulary

Every contract names the vocabulary it depends on, grouped by the crate that defines it,
and a group is stated even when empty, because an explicit nothing is an assertion
someone checked and an absent group is silence.

**Drawn from `weaver-types`:** `organ-envelope`, `model-binding`, `lifecycle-refusal`,
`harness-directive`, `spu-answer`.

`organ-envelope` is the carrier every organ channel draws, and it is drawn here rather
than defined here. The definition stays in `weaver-types` and the mechanics it serves
live in `weaver-organ-channel`, per `weaver-types-PRD` section 2.3.

`model-binding` is a field of the agent's configuration file, defined at
`weaver-types-PRD` section 2.1, drawn here because it is what crosses this seam and not
interpreted here beyond being carried. Neither party redefines it. The operator writes
it, admin validates it, the harness carries it, and the SPU resolves it.

`lifecycle-refusal` is drawn rather than extended with a parallel type of this seam's
own. The refusal has to reach admin inside the enter aggregate unchanged, and a second
refusal type would oblige the harness to translate one into the other, which is exactly
the report-to-parse that `weaver-admin-harness-contract` section 6 rules out. Whether
its case set grows to hold this seam's cases is the cell `weaver-spu-PRD` section 10
holds.

`harness-directive` and `spu-answer` are the two definitions this act owes the floor,
per section 8. Their naming follows the sending party, which is the convention
`admin-directive` and `harness-answer` set, and the collision it produces is filed as a
cell rather than resolved here.

**Drawn from `weaver-traits`:** nothing. The clause is present with that answer because
`weaver-types-PRD` section 5 asks for it even when it is empty. `provider-trait` is the
abstraction the harness issues decode requests through, constructed at the worker
composition root, so it sits on the harness's side of this seam's transport rather than
crossing it. Whether the decode workflow changes that answer is that workflow's to
state.

**Drawn from `weaver-trace`:** nothing. The SPU reports and the harness authors, so no
event kind, no envelope field, and no payload shape crosses this seam. This is the
negative that keeps the sole-writer property checkable from this side rather than only
asserted from the harness's.

**The two definitions land in `weaver-types` and are owed by this act.**
`weaver-types-PRD` section 2.3 rules that nothing enters it until another contract draws
it, and this is that contract. They belong to the floor rather than to either party,
because the harness and the SPU both need them and neither may depend on the other. The
records below belong in `weaver-types-PRD` section 2.3 and are written unfenced
deliberately, so that a mapper reading this document does not ingest records this
document is not the source of:

    node: harness-directive
    kind: vocabulary

    node: spu-answer
    kind: vocabulary

    edge: defines
    from: weaver-types
    to: harness-directive

    edge: defines
    from: weaver-types
    to: spu-answer

Two definitions and no more. A directive with its cases and an answer with its cases is
what sections 2 and 5 demand, and the carrier and the refusal are already defined. A
third added because a third felt tidy would be a reserved slot in data form.

## 8. What this document changes elsewhere

Named here because a document whose reach cannot be read for the reach is a trap. These
are owed by this act and are also carried in `weaver-spu-PRD` section 11, which is that
crate's register and the authoritative list under G5.

- `weaver-types-PRD` section 2.3. Two definitions arrive on demand, so that subsection
  goes from five to seven. This contract draws five values in all, three of which
  already exist, and `model-binding` is drawn from section 2.1 rather than from 2.3.
- `weaver-harness-PRD` section 4. The decode seam resolves through this contract and
  gains no record in that charter, because the organ declares.

**What this document does not close, and the corpus is waiting on it.**
`weaver-admin-harness-contract` section 3 rules the alert case set open with its exit
condition being the organs that can raise a fault acquiring charters naming what they
raise, and names `weaver-spu-PRD` as the first of those. This pass charters the
residency half of this seam and not the run, so the exit condition is unmet and stays
unmet. Stated here rather than left to a reader who checks.
