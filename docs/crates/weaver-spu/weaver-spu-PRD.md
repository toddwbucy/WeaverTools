# weaver-spu - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth for now. It is chartered to the
end of the lifecycle workflow, an agent loaded, idle, and traced, and its remaining
sections arrive with the token workflow, gate to harness to SPU and back. **Half
chartered by ruling rather than incomplete:** a crate is chartered workflow by workflow
and this crate's part in the workflow now finishing is one thing, admitting a model and
later releasing it.

**Date filed:** 2026-07-31
**Revised:** 2026-07-31. The topology ruling landed as drafted, the SPU a child the
harness forks during the enter fan-out over a pair the harness creates before the
fork. The rationale is recorded at `weaver-harness-spu-contract` section 1 and the
markers that priced the ruling are removed.
**Revised:** 2026-07-31, again. Section 6 stops claiming the Document Format still
carries the superseded asks rule, that document's v0.6 landing the scoped one, and
the section 11 entry for `weaver-harness-PRD` section 4 leaves the register as its
edit lands in that charter in the same act.
**Revised:** 2026-07-31, a third entry. Admission becomes the one check on the
device, per ruling C: nothing upstream arbitrates, the no-auto-evict binding of apex
section 6 relocates here to the place that enforces it, and the second-arbitrator
framing of sections 2, 3, and 4.1 rewrites to sole authority.
**Revised:** 2026-08-01, a fourth entry, the fault-carrier ruling. The fault this
crate raises reaches the record as the `fault` event through the harness as author,
rather than reaching admin as an alert, and the section 10 cell restates its exit
against the event kind's case set.
**Document ID:** `weaver-spu-PRD`
**Parent:** `WeaverTools-PRD`
**Companion contract:** `weaver-harness-spu-contract`, drafted with this document
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The charter of the crate that holds the model on the device. It is drafted together with
`weaver-harness-spu-contract`, which governs the one seam this crate holds, and neither
is complete without the other.

Level discipline, stated once. This document carries what the crate needs and why,
including the order in which it admits and releases, because that order is this crate's
own work rather than a wire agreement. What crosses the seam, what it means, and how it
fails is the contract's. How any of it is represented is the Spec's and appears in
neither.

**What this charter reaches, and what it holds open.** It reaches the two exchanges the
enter and leave directives of `weaver-admin-harness-contract` section 3 fan out into,
the residency those exchanges move between, and the process facts a charter has to state
before another crate can build against it. It does not reach the decode seam or anything
crossing it, sessions, turn processing, measurement payloads, residual readout,
verbosity, sampling, or the fault cases this crate raises. Each of those is named as
deferred in section 8 or as a cell in section 10 rather than left out, because an
omission and a deferral read alike to a later reader and only one of them is a decision.

The test that drew the line is mechanical. A clause not needed to make enter and leave
true is out of this pass.

```graph
node: weaver-spu
kind: crate

edge: parent
from: weaver-spu
to: WeaverTools
```

## 1. What this crate is

**The organ that governs model residency, and one of them per agent.** It holds the
weights on the device, it holds them for exactly as long as the worker that forked it
lives, and it answers the harness about that residency across a duplex channel. Both
halves are what make it an organ under apex section 5.4, and neither alone would.
Governing a domain without a duplex channel would make it a submodule of the harness,
and a duplex channel without a domain would make it a second coordinator.

**It is the only crate in this program that holds device memory.** `weaver-harness-PRD`
section 3 gives it model residency, GPU arbitration at the device, decode compute,
embedding compute, and the cache, and has the harness holding no weights and performing
no forward pass. That is a statement about this crate read from the other side, and this
charter states it from this one so that a reader arriving here is not sent back.

**It is a domain root, and its members are not enumerated here.** The domain is semantic
processing, all of it, per the reading the stub carried and this charter keeps: decode
serving and encode serving are two jobs of one domain rather than two domains, and a
classifier or a small activation network the harness calls for effect is a third kind of
semantic processing under the same root. What organizes them is that they share
residency accounting, device arbitration, and lifecycle confirmation, and the harness
routes to them while holding none of them. Which of them become member crates is
discovered when they are chartered, per the Document Format's rule that depth is
discovered rather than designed, and this charter draws no subtree.

**It is mortal in the same way the harness is.** `weaver-harness-PRD` section 1 has
one harness, one agent, one process, one uid, and this crate inherits that shape by
being the harness's child. An SPU serving several agents would put the model residency
of several principals inside one principal, which is the arrangement the architecture
exists to avoid, and it would also give residency a lifetime that outlives the thing
residency is for.

## 2. What this crate owns

**Model residency.** The device-side fact that one model's weights are present and ready
to serve, established by an admit and ended by a release. Residency is what the two
exchanges of section 4 move between, and it is the whole of what this crate publishes
about itself in this workflow.

**Admission is the one check on the device, and this crate is its one authority.**
Nothing upstream weighs the GPU: admin's load-time concern is the configuration file
and what it points at, per ruling C of 2026-07-31, so a device conflict is discovered
here, at model admission, and nowhere earlier. **Admission refuses and never
evicts.** A conflict is rejected until the operator explicitly unloads the occupant,
which is apex section 6's binding relocated to the place that enforces it, so no
load, at any point in its sequence, auto-evicts another agent. The refusal names
itself and travels to admin inside the enter aggregate like any other refused arm of
the fan-out.

**Release of the device.** Freeing what admission took, so that the device is available
to the next load rather than to an unload that is merely intended. Apex section 6 seats
GPU release here and the binding rule that a load never auto-evicts an occupant is what
makes an unconfirmed release expensive: an occupant that is gone but not confirmed gone
blocks a device that is free. Section 4.2 states the ordering that follows.

**The hot cache, and the rule apex section 2 asks this charter to state.** The cache is
the one surface in this program holding state that nothing could reconstruct even
in principle, which is why apex section 2 names it the sole exception to statelessness
and assigns its ownership rule here rather than to the crate that uses it. The rule has
three parts and all three are already settled in the merged corpus. **This crate owns
the cache.** **The harness owns the flush decision,** per `weaver-harness-PRD` section
2, so flushed on the harness's terms is an obligation of the seam rather than a policy
this crate elects. **The harness is forbidden to touch it,** holds no handle to it, and
so cannot protect or corrupt any region of it. What this pass does not state is how a
flush is expressed on the wire, which belongs to the decode seam and is deferred with
it.

**The cache ends with the residency and never outlives it.** A release frees the
device, and the cache is on the device, so nothing survives a release to be reattached
to a later admission. This is the same fact as residency ending at unload, stated once
about the surface most likely to grow quietly into session state.

**What it reports about itself, and nothing it authors.** The harness is the sole writer
of the trace, so this crate reports and the harness authors, per apex section 6 and
`weaver-harness-trace-contract`. That holds at every exchange in section 4 and it is why
this charter names no event kind.

## 3. What this crate must not hold

**Trace authorship, and any route to the record.** It authors no event, holds no event
kind, and holds no descriptor to the stream's sink. The route it must not have is
concrete rather than abstract: the worker holds the trace descriptors and the
coordination channel to admin in its address space at the moment it forks, so an SPU
that inherited either would hold a writable handle to the record its own agent produces
and a channel to the supervisor of that agent. What stops it is the close-on-exec
discipline the corpus already binds the harness to, and section 7 states the obligation
this crate places on the fork.

**A channel to `weaver-admin`.** Admin holds one seam and reaches no organ directly, per
`weaver-admin-PRD` section 6, so what this crate has to tell admin reaches admin through
the harness as hub. That is not a limitation on this crate. It is what makes the harness
the coordinator rather than one peer among several.

**Any state that crosses a residency.** There is no warm reload, no retained
compiled artifact handed from one residency to the next, and no cached device allocation
held past release in anticipation of the next admit. Residency ends at unload, plainly,
with no gesture toward keeping the expensive part warm. This is free rather than
costly, because the process holding the state dies with the unit and
there is nowhere for it to be kept.

**Fleet knowledge.** It knows what it was asked to admit and what it holds. It does
not know what other agents exist, what they hold, or whether one of them would
rather have this device, and it does not need to: it is the one authority on the
device, per ruling C, and it judges the device it can see rather than the fleet it
cannot. Whether the fleet should spend this device on this agent is the operator's
question, answered in the configuration before a load is directed at all.

**The agent's configuration file.** `weaver-types-PRD` section 2.1 has one writer, the
operator, and two readers, admin and the harness. This crate is not a third. The model
binding reaches it across the seam because the harness was handed it in the enter
directive, so a read here would be a second interpretation of a file admin already
validated, with no way to disagree usefully and every way to disagree quietly.

**A listening socket of any kind.** `weaver-gate` binds the only network socket. This
crate binds no socket at all, because the channel it uses was created by the harness
before the fork and arrives already connected.

**Policy about whether a load should happen.** It answers whether this device can take
this model now, and whether the agent should be loaded at all is the operator's.

**Cognition.** It runs a forward pass and it does not decide what to run one for. The
prompt is assembled by the harness, the tool decision is the model's, and the
interpretation of either is the harness's.

## 4. Loading an agent, from this crate's side

Loading an agent means one thing here. Admit a model, and later release it. Everything
in this section is derived from `weaver-admin-harness-contract` section 3 and
`weaver-admin-PRD` sections 4.1 and 4.2, which are the documents that create the
obligations this charter exists to make true.

**The two exchanges are the whole of the fan-out that reaches this crate.** Admin
directs the harness to enter, the harness fans that directive out along its own seams,
and one arm of the fan-out is the admit below. Admin directs the harness to leave, and
one arm of that unwinding is the release. Admin holds no channel here and learns of this
crate only through the aggregate answer the harness returns, which is why a refusal
below has to carry a reason the harness can place in that aggregate without translation.

### 4.1 admit

Opened by the harness, carrying the model binding it was handed in the enter directive.
It ends in residency confirmed or in a typed refusal.

1. **Resolve the binding to an artifact.** A binding naming a model this crate
   cannot find is refused before the device is touched.
2. **Read what the artifact declares about itself, without loading it.** The old
   tree reads a model file's header and metadata block to answer what family this is
   and what its basic dimensions are, without reading tensor data and without
   touching the device. That is a mechanic worth keeping, because it converts the
   most common shape of a bad binding, an artifact that is present and wrong, into a
   refusal that costs no device work.
3. **Judge the device against what admission requires.** What the artifact needs
   plus the working headroom the residency requires must fit what the device has
   free. This is the one check on the device, per section 2, and nothing upstream
   performed an earlier one.
4. **Take the device and load the weights.**
5. **Confirm residency.** The answer confirms and carries nothing else, per section
   4.4.

**Every step before the fourth is refusable at no cost, and that ordering is the
substance.** A refusal reaching the harness before any device work has happened is a
refusal the enter fan-out can unwind cheaply, and a load that fails after the device is
taken is the case section 5 is written for.

**Exactly one model is admitted, because exactly one binding crosses.** The agent's
configuration file carries a model binding, singular, per `weaver-types-PRD` section
2.1, and the enter directive carries what admin was handed. Whether the domain later
admits more than one, which is what an encoder beside a decoder would mean, arrives with
the members and is not settled by this workflow admitting one.

**Idempotence has no subject here.** The old tree allowed a model load to be
idempotent for an identical resident model request, which was a real property of a
long-lived server that several agent loads passed through. This crate begins each life
empty, admits once, and dies, so there is no prior residency for a second admit to match
and no second admit in the workflow. A charter that carried the idempotence rule anyway
would be carrying a mechanic whose premise it had removed.

### 4.2 release

Opened by the harness during the leave fan-out. It ends in the device freed.

1. **Stop serving.** Nothing new is accepted against this residency.
2. **Free the device.** The weights, the working allocations, and the cache go together,
   because they are one residency rather than three things that happen to be resident.
3. **Confirm release.**

**The confirmation is what the harness's answer to admin rests on, and the ordering is
not negotiable.** The old tree freed the reservation and deferred the release, so a
subsequent load could be admitted while the prior model still occupied the device, and
its own record names the resulting overcommit. The merged corpus already orders this
correctly, at `weaver-admin-PRD` section 4.2, where the release sits inside the leave
directive and admin publishes unloaded only on the aggregate. This charter states the
obligation from this side: **release is confirmed after the device is free and never
before it,** so that a confirmation is a fact about the device rather than a statement
of intent.

**Release is the orderly end of a residency and process death is the abrupt one.**
Both end it. Only one is confirmed. The abrupt path costs nothing that has to be
reaped, because the process holding the device dies with the unit
and the device is reclaimed with the process, which is section 5's second half.

### 4.3 The residency this crate moves between

Two states, observable at the seam, and no third.

**Empty.** No model resident and no device memory held on this residency's account. It
is the state this crate is in when the harness first speaks to it, and the state a
refused admit leaves behind.

**Admitted.** One model resident under the binding it was admitted with, and the device
holding it.

**The transition is not observable and this charter does not name it as a state.** The
exchange is the unit, the harness has one directive outstanding, and nothing in this
workflow reads residency in the middle of an exchange. A third state would be a state
machine in a charter, which is level material this corpus keeps out of contracts and has
no better claim to here.

**A refused admit leaves empty and never leaves partial.** Whatever a failed admission
took, it gives back before it answers, so the refusal the harness receives is true about
the device and not merely true about the attempt. This is the same rule as a partial
load never being published as loaded, one level down, and it is what lets admin's
rollback treat a refusal from this arm as needing nothing undone here.

**A refused admit does not end the process.** This crate answers the refusal on the
channel and stays alive to be reaped by the unit stop that follows. An organ that exited
on refusal would close the channel instead of answering, and the harness would be left
observing a death where a typed reason was available, which converts a refusal that
names its cause into one that does not.

### 4.4 What the answer carries, and what it deliberately does not

**A confirmation confirms and carries no payload.** Nothing in this workflow consumes a
description of the admitted model. The candidate content is the model's identity and its
weights hash, which apex section 8 does require, and which reach the record inside
`model.measurement` at decode rather than at load. Adding them to this answer because
they exist would be a payload field whose only reader is elsewhere, which
`weaver-trace-PRD` section 8 and apex section 9 both name as a reserved slot in data
form.

**That leaves a gap worth naming rather than closing here.** All three `model.*` kinds
are elected, per `weaver-trace-PRD` section 5, so a run recorded at the floor carries no
measurement payload and its record therefore names no model. The run happened, it was
served by particular weights, and nothing in the record says which. Whether the run's
`load` event should carry the admitted model's identity is a question about the record
rather than about this seam, and section 10 files it where it can be answered.

## 5. What a failure partway through leaves behind

Stated by where it failed, because the obligations differ and a charter that says a
refusal leaves nothing has not yet said what nothing means at a device.

**A refusal at any step before the device is taken leaves nothing.** No allocation, no
handle, no residency, and an answer the harness can place in its aggregate.

**A failure after the device is taken leaves the device freed before the answer.** Per
section 4.3, whatever was taken is given back first. The cost of getting this wrong is
that a device is held by an agent that failed to load, and the next load is refused
against a conflict that does not exist.

**A failure this crate cannot recover from, having taken the device, is process death
and is not an answer.** The device is reclaimed when the process exits, so the failure
mode that would be worst under a long-lived unit, a leaked residency with no owner and
no reaper, has no subject here. What the harness observes is closure, which the contract
governs.

**A release that cannot be confirmed is survivable here for the same reason.** The
harness reports it as unconfirmed rather than synthesizing a confirmation, admin's leave
answer names where the sequence stopped, and admin's own step of stopping the unit frees
the device regardless.

**Nothing here retries.** A refused admit returns to the harness, which unwinds. This
crate does not re-attempt an admission under one directive, because two attempts behind
one operator intent is the shape `weaver-admin-harness-contract` section 6 forbids on
the seam above and there is no reason for this seam to differ.

## 6. The seam

This crate holds one seam. It is a duplex channel to `weaver-harness`, governed by
`weaver-harness-spu-contract`.

| Seam | Peer | What crosses |
|---|---|---|
| Residency | `weaver-harness` | The harness asks this crate to admit the model binding it was handed and later to release it. This crate confirms or refuses with a reason the enter aggregate can carry unchanged. |

```graph
edge: seam
from: weaver-spu
to: weaver-harness
via: weaver-harness-spu-contract
tag: socket

edge: floor-link
from: weaver-spu
to: weaver-types
```

**The tag is `socket` because the seam crosses a process line,** which is the test apex
section 5.1 states after its restatement. It does not imply the credential mechanism. On
a connected pair created by one process and handed to another, `SO_PEERCRED` reports the
creating process at both ends and distinguishes nothing, so this seam authenticates by
possession, which is the apex's second case rather than an exception to its first.

**The seam edge is declared here because this crate is the organ.** Under the rule
`weaver-admin-harness-contract` section 0 states, the organ declares and the harness
does not, because the harness is the hub every organ is duplex with and a hub declaring
its own edges would carry the whole seam graph in one crate. The older rule, that the
crate which asks declares, had no unique answer on a duplex seam, which is why it was
replaced, and Document Format section 4 now carries the scoped rule.

**The channel is duplex because this crate is an organ, and this pass charters one
direction of it.** Apex section 5.4 makes a duplex channel with the harness one of the
two properties of an organ, and the property does not bend. Both exchanges chartered
here are opened by the harness. The direction this crate opens is the fault it raises,
which reaches the record through the harness as author, written as the `fault` event
of `weaver-trace-PRD` section 3.1 per the fault-carrier ruling of 2026-08-01, and
the case set for it is deferred with a named exit in section 10. A reader should not
take a half-chartered direction for an absent one.

**One channel carries the organ's traffic, and whether decode shares it is not settled
here.** Apex section 5.4 names a duplex channel, singular, and `weaver-types-PRD`
section 2.3 has `organ-envelope` as the carrier every organ channel draws. Drafting a
second channel for decode would be a second seam under the Document Format, needing its
own name and its own contract, and this pass has no measurement to justify one. What it
does have is a real question, since a boundary-preserving ordered channel puts a release
directive behind whatever decode traffic is ahead of it, and this program's own
principle is that latency is the enemy of agency. Section 10 files it against the
workflow that can measure it.

**This crate links `weaver-types` and does not link `weaver-traits`.** It draws the
carrier and the binding it is asked to admit from the floor, and it draws no trait. The
`provider-trait` of `weaver-traits-PRD` section 3.2 is the abstraction the harness
issues decode requests through, constructed at the worker composition root and injected
there, so it is the far side of this seam's transport rather than something this crate
implements. Whether the decode workflow changes that answer is that workflow's to state.

**The non-link is a declared surface rather than a build exclusion.** `weaver-types`
floor-links `weaver-traits`, so those definitions sit in this crate's dependency tree
whatever this charter says. What the declaration buys is a checkable statement that
nothing draws them on this crate's behalf.

## 7. Identity, process boundaries, and the model artifact

**This crate runs as the agent.** The worker holds the agent uid from its first
instruction, per `weaver-admin-PRD` section 7, and a child it forks holds the same uid.
There is no drop to order and no window to get wrong, and there is also no separation:
the SPU is a same-uid peer of the worker rather than a process the kernel holds apart
from it.

**Which puts this crate inside the hardening boundary rather than outside it.**
`weaver-admin-PRD` section 7 names same-uid reach as a live hole, closed by the worker
clearing its dumpable flag so that a same-uid process cannot attach to it and drive its
descriptors. This crate is a same-uid process by construction. It holds the weights and
one end of a channel to the interior coordinator, so **it clears its own dumpable flag
after its final exec** for the same reason and by the same mechanism. The flag resets on
`execve`, so the requirement is stated against the last exec and is a set rather than a
check.

**The fork carries exactly one descriptor across and the discipline that holds it is the
harness's.** At the moment of the fork the worker holds the trace descriptors and the
coordination channel to admin. Both are close-on-exec under the receiver rule of
`weaver-organ-channel` section 2, bound on that seam by `weaver-admin-harness-contract`
section 2 and `weaver-harness-PRD` section 5, and that discipline is what keeps them out
of this process. This charter states the requirement it depends on rather than restating
the obligation: **this crate receives the channel end and no other descriptor, and a
build in which it holds a trace descriptor is broken whether or not it writes through
one.** The contract binds the party that can meet it.

**The model artifact is owned outside the agent uid and readable into it.** The
agent must read the weights and must not be able to alter them, and ownership is what
buys the second half, because an owner can restore its own access whatever the mode
says. So the artifact is owned by a principal that is not the agent, readable by the
agent uid through group or mode, and writable by neither the agent nor anything running
as it. This is the trace custody argument inverted, and it lands the same way: the
property comes from non-ownership rather than from the bits.

**It is an operator artifact and this program verifies rather than authors it.** That is
the same posture `weaver-admin-PRD` section 1 takes toward the boundary, and it reaches
here through the two failure points admin already names. Admin refuses a load whose
configuration names a binding it cannot satisfy, before a process exists. This crate
refuses an admit whose artifact it cannot read or parse, which is the failure admin
could not have seen. Neither repairs anything.

**This crate's code compiles into this crate's processes and into no other.** It is
not linked into the worker's address space, so no build exists in which harness code
contains a forward pass it could reach without crossing the seam. This is the same
structural argument `weaver-admin-PRD` section 1 makes about admin's code and the
worker, applied one level down, and it is what keeps the routing claim of
`weaver-harness-PRD` section 3 from being a discipline someone has to keep.

## 8. What does not cross, and what waits

**The encoder is named as domain and is not built.** Encoding and decoding are one
domain and it is this crate's, per `weaver-harness-PRD` section 3. An encoding is useful
only if something retrieves by similarity, nothing in the stateless turn retrieves, and
an embedder would produce vectors with no consumer. So this charter states the domain,
adds no affordance for it, and builds nothing: no trait, no variant, no feature flag, no
configuration field. **Ownership is not usage.** A charter naming a domain is a decided
boundary, and an unbuilt interface waiting to be filled is what apex section 9 forbids.

**The memory leg in every form.** No substrate, no recall, no consolidation. When memory
returns it returns as a socket peer under its own contract, per apex 5.1.

**The framework migration argument, entirely.** The old tree's charter for this crate is
substantially an argument about which tensor framework to depend on and how to relate to
it, decided against a workspace and a customer profile this program does not have. It is
mechanics at best and shape never, and nothing in it crosses. What framework this crate
uses is a Spec question in a program whose apex names one deliverable.

**Multi-tenant serving infrastructure.** Continuous batching, paged attention, prefix
sharing across unrelated clients. These answer how one device is shared among many
concurrent network clients, and this architecture has one agent per residency by
construction.

**Deferred to the token workflow, and named as deferred rather than omitted.** The
decode seam and everything that crosses it, sessions and their append-only protocol,
turn processing, the measurement payload, residual readout, verbosity as it reaches this
crate, sampling, and the flush mechanism. Each of these is real, each is this crate's,
and each depends on a workflow that starts at the gate. The charter's next sections
arrive with that workflow.

## 9. Staged requirements

Recognized work with an entry condition that holds it out of this pass. This section is
authoritative for staged work belonging to this crate, per the rule `weaver-trace-PRD`
section 9 sets.

**Admission's headroom figure.** Section 4.1 step 3 requires that what the artifact
needs plus working headroom fit what the device shows free. What the headroom is, and
whether it is a constant, a fraction, or derived from the artifact's declared shape, is
not settled. The old tree carried a headroom term in its admission inequality and stated
no derivation for it. **Entry condition:** a measurement on a real artifact
against a real device, which the decode workflow produces as a side effect of existing.

**Whether the artifact's declared shape is trusted or verified.** Section 4.1 step 2
reads what the artifact says about itself. An artifact that declares one shape and holds
another is a load that fails late rather than early, and whether this crate checks the
declaration against the tensor data before taking the device is a cost this pass cannot
price. **Entry condition:** the Spec pass, with a measurement of what the check costs.

## 10. Open cells

Each names what settles it. A cell with a proposed reading and a named test is a handoff
rather than a hole, and a draft with no cells has hidden them.

**The admit refusal reason set.** Section 4.1 names four points at which an admit can
refuse and this charter does not enumerate the reasons. The old tree's device authority
carried a usable candidate list, which is offered as mechanics rather than as the
answer: the artifact does not resolve, the artifact does not parse or declares a shape
this crate cannot serve, the artifact is present and unreadable by the agent uid, the
device ordinal is not present, the device is occupied by something this program did not
put there, and what admission requires exceeds what the device has free. Every device
case is caught here and nowhere earlier, per ruling C. **Settled by:** the ruling on
whether these cases extend `lifecycle-refusal` or take a definition of their own, which
section 11 files as a conditional edit to `weaver-types-PRD`.

**Whether the record names the model that served the run.** Section 4.4 leaves it open.
A run recorded at the floor carries no `model.measurement`, so its record names no model
and no weights hash, and the run is not replayable by construction under
`weaver-trace-PRD` section 5. Whether that is correct, or whether the `load` event
should carry the admitted model's identity so that a floor run at least says what it
ran, is a question about `weaver-trace-PRD` rather than about this seam. The cost of the
second reading is an edit to a closed payload shape and to the contract that names it.
**Settled by:** the human's ruling, weighing a record that cannot say what served it
against an edit to a floor event's payload.

**Whether decode shares this channel.** Section 6 charters one channel because the organ
test names one, and names the head-of-line question it leaves open: a
boundary-preserving ordered channel puts a lifecycle directive behind whatever decode
traffic precedes it. **Settled by:** a measurement taken when there is decode traffic to
measure, which is the token workflow. Named here so the seam is inherited as a question
rather than rediscovered as a surprise.

**What this crate raises, which the `fault` event is waiting on.** The fault-carrier
ruling of 2026-08-01 made the stream the fault's carrier and the `fault` event kind
its shape, and the case set behind that kind closes when the organs that can raise
a fault have charters naming what they raise, **with this charter named as the
first of those.** This pass does not close it, because a fault the worker survives
is a fault during a run and the run belongs to the workflow that has not been
chartered. The exit condition is therefore unmet by this pass and stays unmet, and
that is stated plainly rather than left for a reader to discover by checking.
**Settled by:** the token workflow's pass over this crate.

**Device-state reporting, which the old tree answers in a way this corpus has not ruled
on.** That tree read free memory from the device driver through a command-line tool and
marked the reading as diagnostics only, holding its own accounting of what it had
allocated as the authority for admission decisions. The two disagree whenever something
outside the program holds memory, which is the case section 2 says this crate exists to
catch, so preferring the internal accounting is preferring the number that cannot see
the thing the check is for. **Settled by:** a ruling on which reading admission judges
against, taken with a measurement of what the driver query costs on the admit path.
That tree recorded a preference and not a reason, and shelling to an external tool on an
admission path is the visible cost a reason would have named.

**The naming of the two owed wire definitions.** Section 11 owes `weaver-types` a
directive and an answer for this seam. Named for the sending party, following the
convention `admin-directive` and `harness-answer` set, they are `harness-directive` and
`spu-answer`. That gives the harness both a `harness-answer` it sends upward and a
`harness-directive` it sends downward, which is a true statement of the hub model and
also two identifiers a reader has to hold apart. The Working Rules treat visual
collision as a defect in a graph block as much as on a page. **Settled by:** a ruling on
whether the convention or the collision governs.

## 11. Edits owed in the same act

Apex section 10 requires that a change touching a contract merges with every party in
one act, and this section is that register. Nothing below is applied by this document.
An entry leaves this register when the edit lands, because a ruling recorded and not
landed reads as settled and an entry landed and not cleared reads as outstanding.

- `weaver-types-PRD` section 2.3: two definitions arrive on demand, a directive the
  harness sends across this seam and an answer this crate returns. That subsection rules
  that nothing enters it until another contract draws it, and
  `weaver-harness-spu-contract` is that contract. Names are a cell in section 10.
- `weaver-types-PRD` section 2.3, conditionally: whether the admit refusal cases extend
  `lifecycle-refusal` or take a definition of their own. The draft reuses
  `lifecycle-refusal`, on the grounds that a reason the enter aggregate carries without
  translation has to be the type that aggregate already carries. Owed if the ruling goes
  the other way.
- `open-items.md`, the note owed to this crate's stub: discharged by section 2 of this
  charter carrying its substance directly, so the item leaves the list rather than
  moving into the stub it was owed to. Named by substance because that list renumbers.
- `open-items.md`, the encoder item: it moves into section 8 of this charter, which is
  where staged and excluded work belonging to a crate lives once the crate has a
  charter to hold it.

## 12. Children

Specs to be written against this charter once the PRD set is ratified. Named so the set
is bounded, not drafted here, and incomplete for the same reason the charter is.

- Admission, covering artifact resolution, the header read, the device judgment, and the
  refusal set.
- Release, covering the ordering that makes a confirmation a fact about the device.
- The organ channel's construction and the fork, covering the descriptor discipline of
  section 7.

Contracts this crate is party to are written with the PRDs of their other parties, one
per seam in section 6, and are not children of this document.
