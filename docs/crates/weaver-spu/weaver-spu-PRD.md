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
**Revised:** 2026-08-01, a fifth entry. Sections 2 and 8 adopt the proto-stateful
vocabulary, per the human's rename of this date.
**Revised:** 2026-08-01, a seventh entry, the naming ruling. The section 10 naming
cell closes, the collision governing and wire vocabulary naming for the loop, this
seam drawing loop 0's trio with its refusal cases settling as `lifecycle-refusal`
cases, and the two section 11 entries the sender-named pair carried leave the
register.
**Revised:** 2026-08-01, a sixth entry, on review. Section 3 stops claiming the
gate binds a network socket, the demotion of 2026-07-31 having made it a local
hook, and the only-listener claim scopes to the agent.
**Revised:** 2026-08-02. The section 10 model-naming cell closes with the
recording levels, per the human's ruling of this date, and sections 0 and 8 drop
verbosity from what the token workflow carries.
**Revised:** 2026-08-02, a second entry this date, the decoder-cut ruling of the
composability batch. The section 10 head-of-line cell closes: decode does not
share the lifecycle channel, taking its own socket owned by this crate, the
sockets multiplying while the organ does not, and the measurement the cell
waited on is superseded by the structural answer. On the review seat's return
the same day, the closure defers one more thing by name: whether the decode
socket carries the organ envelope, and under what encoding, goes to the token
workflow, the every-channel sentence of `weaver-types-PRD` section 2.3 meeting
its first unclassified instance there rather than silently.
**Revised:** 2026-08-02, a third entry this date, the token workflow's charter
act. The decode half arrives: section 13 charters the decode submodule and the
token seam, the crate's second seam to the harness on its own socket per the
decoder-cut ruling, and section 14 charters the family libraries, the module
discipline the operator's ruling of this date shapes the crate around. The two
sections file after the children deliberately, sections 8 through 12 being
cited by number across the merged corpus, arrival order being filing order.
Section 2 gains the substrate paragraph, section 6 the token seam's row and
record, section 7 counts two channel ends across the fork, section 8's
deferred list shrinks to what stays deferred, and section 10's fault cell
closes its half here, the cases this crate raises named in 13.10 with the
gate's cases still owed.
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
sampling, or the fault cases this crate raises. Each of those is named as
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

**The umbrella's substrate.** The crate is about two things, per the operator's
ruling of 2026-08-02: the family libraries of section 14, where everything a
model family defines once is defined once, and the presentation of Unix
sockets over which the harness hands work in and takes results back. Each
semantic domain of processing is tied to its own submodule, the decoder the
first and in this pass the only one, per section 13. The encoder and the
later operation types arrive as their own submodules when their workflows
charter them, and nothing here reserves their shape.

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
in principle, which is why apex section 2 singles it out by name
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

**A listening socket of any kind.** `weaver-gate` binds the agent's only
listening socket, a local hook and no network socket, per the demotion ruling of
2026-07-31. This crate binds no socket at all, because the channel it uses was
created by the harness before the fork and arrives already connected.

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
| Token | `weaver-harness` | The harness opens the resident decode session, appends each turn's delta, and receives the generation with its measurement. The stop's cancel and the flush cross here. Per section 13 and `weaver-harness-spu-decode-contract`. |

```graph
edge: seam
from: weaver-spu
to: weaver-harness
via: weaver-harness-spu-contract
tag: socket

edge: seam
from: weaver-spu
to: weaver-harness
via: weaver-harness-spu-decode-contract
tag: socket

edge: floor-link
from: weaver-spu
to: weaver-types
```

**The token seam is the second seam to the same peer, on its own socket, and it
is not an organ channel.** The decoder-cut ruling of 2026-08-02 gave decode its
own socket so no lifecycle directive queues behind decode traffic, and section
13.2 carries the classification that ruling deferred: the organ test of apex
section 5.4 names one duplex channel, the lifecycle channel is that channel,
and this one is operation surface, so the organ envelope does not cross it and
the every-channel sentence of `weaver-types-PRD` section 2.3 stays scoped to
organ channels with no exception admitted.

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

**The fork carries exactly two descriptors across and the discipline that holds
them is the harness's.** The count was one until the token workflow's act: the
lifecycle channel's end, and now the decode socket's end beside it, both pairs
created by the harness before the fork, per the decoder-cut ruling and section
13.2. At the moment of the fork the worker holds the trace descriptor and the
coordination channel to admin. Both are close-on-exec under the receiver rule of
`weaver-organ-channel` section 2, bound on that seam by `weaver-admin-harness-contract`
section 2 and `weaver-harness-PRD` section 5, and that discipline is what keeps them out
of this process. This charter states the requirement it depends on rather than restating
the obligation: **this crate receives its two channel ends and no other
descriptor, and a
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
domain and it is this crate's, per `weaver-harness-PRD` section 3. An encoding is
useful only if something retrieves by similarity, nothing in the proto-stateful
turn retrieves, and an embedder would produce vectors with no consumer. So this
charter states the domain, adds no affordance for it, and builds nothing: no
trait, no variant, no feature flag, no configuration field. **Ownership is not
usage.** A charter naming a domain is a decided boundary, and an unbuilt interface
waiting to be filled is what apex section 9 forbids.

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

**Chartered by the token workflow's act of 2026-08-02, no longer deferred.** The
decode seam, sessions and their append-only protocol, turn processing, the
measurement obligation, residual readout as it reaches this crate, sampling and
its dispositions, and the flush all arrive in section 13, which is the
charter's next section arriving with its workflow exactly as this paragraph
promised. What stays out is the encoder's operations, per the paragraph
above, and the turn-path elaborations that belong to the gate's own charter,
streaming and concurrent clients among them, which reach this crate only
after that seam's turn-direction act shapes them.

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
case is caught here and nowhere earlier, per ruling C. **Settled by:** the type
half settled with the
naming ruling of 2026-08-01, the cases extending `lifecycle-refusal` as loop 0
cases, and the enumeration itself still arrives with the decode workflow's
measurements.

**Whether the record names the model that served the run is closed.** The cell
rested on a run recorded at the floor carrying no `model.measurement`, and the
ruling of 2026-08-02 retired the recording levels, so every run carries one and
every record names its model and weights hash. The question had no answer to give
once its premise left, which is a cell closing rather than being settled.

**Whether decode shares this channel is closed: it does not, per the decoder-cut
ruling of 2026-08-02.** Decode traffic takes its own Unix socket, owned by this
crate and distinct from the lifecycle channel, so no lifecycle directive ever
queues behind decode traffic and the head-of-line question dissolves rather than
being measured. The organ test still names one duplex channel and still holds:
the lifecycle channel is that channel, the decode socket is additional surface
this crate owns, and the sockets multiply while the organ does not. The
measurement this cell once waited on is superseded, because the risk it would
have measured is answered structurally. How the decode socket reaches the
harness, the exchanges that cross it, and whether it carries the organ envelope
at all, under what encoding, are the token workflow's to charter. The last is
named because `weaver-types-PRD` section 2.3 has every duplex channel with the
harness carrying that envelope, and this socket is the universal's first
unclassified instance: either it is such a channel and the envelope's encoding
on the hot path becomes a live question, the JSON election stopping short of
the decode seam by `weaver-types-Spec` section 4's own rule, or the universal
gains its first exception and says so, and that classification belongs to the
workflow that charters the traffic rather than to this closure.

**What this crate raises is named, and the corpus-wide cell is closed.** The
fault-carrier ruling of 2026-08-01 made the stream the fault's carrier and the
`fault` event kind its shape, with this charter named as the first of the
organs owing an enumeration. Section 13.10 carries it, the token workflow's
act having chartered the run this crate's faults occur in, and the gate's
half landed at `weaver-gate-PRD` section 13.4 in the same workflow's gate
act of 2026-08-02, which closed the set. What remains is the payload's
shape, which the trace act elects against the closed set rather than against
a guess, and that is the trace's work rather than a cell of this charter's.

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

**The naming of the wire definitions is closed.** The collision governed, per the
human's ruling of 2026-08-01, and the convention changed: wire vocabulary is named
for the loop whose traffic it carries, direction being a fact about a loop's walk
rather than a name on a duplex channel. This seam draws loop 0's trio and owes the
floor nothing, per `weaver-types-PRD` section 2.3.

## 11. Edits owed in the same act

Apex section 10 requires that a change touching a contract merges with every party in
one act, and this section is that register. Nothing below is applied by this document.
An entry leaves this register when the edit lands, because a ruling recorded and not
landed reads as settled and an entry landed and not cleared reads as outstanding.

- `open-items.md`, the note owed to this crate's stub: discharged by section 2 of this
  charter carrying its substance directly, so the item leaves the list rather than
  moving into the stub it was owed to. Named by substance because that list renumbers.
- `open-items.md`, the encoder item: it moves into section 8 of this charter, which is
  where staged and excluded work belonging to a crate lives once the crate has a
  charter to hold it.
- `weaver-types-PRD` section 2.3, owed by the decode contract's act: the token
  vocabulary this seam draws, named for the seam's currency under the naming
  ruling's ratified extension, this seam's loop being loop 1, the builder's
  and variable, defined at the floor because both parties need it and
  neither may depend on the other.
- `weaver-harness-PRD` section 2: landed in this same act. The framing
  candidate of section 13.4 ratified, the per-model assembly paragraph
  rescopes, the deterministic floor staying the harness's and the family
  template's render seating in the family library, with the rendered reality
  returning on the report path.
- `weaver-trace-PRD` section 3.1, by the token workflow's trace act: the three
  `model.*` payload shapes, produced against section 13.6's obligations.

## 12. Children

Specs to be written against this charter once the PRD set is ratified. Named so the set
is bounded, not drafted here, and incomplete for the same reason the charter is.

- Admission, covering artifact resolution, the header read, the device judgment, and the
  refusal set.
- Release, covering the ordering that makes a confirmation a fact about the device.
- The organ channel's construction and the fork, covering the descriptor discipline of
  section 7.
- The decode submodule of section 13: the session, the token-boundary stop, the
  measurement production, the readout tap per backend, and the encoding of the
  token seam, elected with a measurement.
- The family libraries of section 14: the per-family module surface and the
  capability declaration.

Contracts this crate is party to are written with the PRDs of their other parties, one
per seam in section 6, and are not children of this document.

## 13. Serving the turn: the decode submodule and the token seam

Arrived with the token workflow, per section 8's promise, and filed after the
children because sections 8 through 12 are cited by number across the merged
corpus and a renumbering that broke every citation would buy tidiness at the
wrong price. Everything here is derived from the turn path of apex section 3,
the grammar of `basic-inference-loop`, and the operator's rulings of
2026-08-02, with the salvage survey at `docs/project/` as the record of what
the prior program learned.

### 13.1 The submodule

**The decoder is the first operation submodule, and each semantic domain gets
its own.** The crate's shape is the substrate of section 2: family libraries
plus socket presentation, with the processing itself in per-domain
submodules. In this pass the decoder is the only submodule and it shares the
crate's one process. A later operation type arrives as its own submodule in
its own process under this domain root, with its own socket and its own
contract, the sockets and processes multiplying while the organ does not.
Nothing is reserved for the later types: no trait, no variant, no socket
bound early, per apex section 9.

### 13.2 The token seam

**The second seam to the harness, on its own socket, created by the harness
before the fork like the first.** The decoder-cut ruling gave decode its own
socket, and this section charters what that ruling deferred. The pair is
created beside the lifecycle pair and crosses the same fork, so possession
authenticates it the same way, and section 7 counts both ends.

**It is not an organ channel, and the classification is this act's.** Apex
section 5.4's test names one duplex channel and the lifecycle channel is it.
This socket is operation surface, so the organ envelope does not cross it,
`weaver-types-PRD` section 2.3's every-channel sentence stays scoped to organ
channels with no exception admitted, and the seam's vocabulary is its own,
named for the seam's currency under the naming ruling's ratified extension,
a loop name being what this seam cannot take with loop 1 the builder's and
variable, defined at
the floor, and owed to `weaver-types-PRD` section 2.3 by the contract's act.

**Every request on this seam belongs to a turn and carries its context.**
Apex section 5.2 scopes the join-key invariant to requests that belong to an
existing turn, and unlike the lifecycle directives that forced the scoping,
decode traffic is exactly what the invariant is for: every ask carries the
turn's context and every answer carries it back, so the reports this crate
returns are attributable with more than one turn in flight or none.

### 13.3 The session

**One resident decode session per residency, append-only, and the discipline
is forced rather than elected.** The prior program proved on live hardware
that hybrid and recurrent decoder families cannot roll their state back: a
protocol that rewinds resident state fails silently on the families that
keep recurrent layers, and the failure surfaces as position errors far from
its cause. So the session advances only forward. Each turn appends its
delta at the resident end, nothing ever asks resident state to rewind, and
the protocol is uniform across families because the weakest family sets the
rule.

**The identity prefix is established at open and permanent for the session's
life.** `weaver-harness-PRD` section 2 rests the prefix's permanence on this
seam as an invariant the SPU honors, and this is where it is honored: no
operation of this seam removes or alters the prefix short of the flush of
13.9, whose outcome is defined against it.

**A session that cannot take the next delta refuses, typed, and sheds
nothing.** There is no eviction and no compaction inside this crate, because
either would be this crate deciding which part of the agent's context
matters, which is cognition and the harness's. The refusal names the
overflow and the harness decides what a full context means for the turn.

### 13.4 Framing, and where it is performed

**Ratified by the operator, 2026-08-02, in this act.** The family library
renders.
The harness sends canonical messages, the message model of `weaver-traits`,
and the submodule renders them through the family's template into what the
model sees. The rendered reality returns on the report path of 13.6, the
template's identity, the token identifiers, and the block partition, so the
record holds the mapping from the canonical conversation to what the model
saw, per the operator's end-to-end requirement of 2026-08-02: input
formatted correctly for the family, verified per family, and output style
handled by the trace holding both the verbatim emission and the canonical
parse.

The grounds. Family knowledge lives in one home, section 14's, rather than
splitting across two domains or forcing the harness to link an SPU-domain
member across the topology's grain. The harness's deterministic assembly
floor is untouched: order of parts, the message sequence read from the
working structure, and everything `weaver-harness-PRD` section 2 fixes stay
the harness's, and what moves is only the family template's application. The
trace's authorship is untouched, because the render reports back and the
harness authors the report. The operator ratified the candidate with this
act and directed the whole change in one place, so the
`weaver-harness-PRD` edit lands in this same act rather than riding the
register as a condition, the change tied to its reason.

### 13.5 The turn, and the stop

**Append and generate is the turn's shape on this seam.** The harness
appends the turn's delta and the generation returns with its measurement,
the two crossings of the fork the basic loop's section 4 draws.

**Ratified at the act's merge, 2026-08-02.** The stop lands at the token
boundary. The generation checks for the harness's cancel between sampled
tokens, which at production decode rates bounds the stop's latency well
under the operator's perception, and a mid-kernel abort would buy
milliseconds at the cost of device-state certainty. **An aborted generation
still leaves the session well-framed:** the family's turn terminator is made
resident before the answer returns, the prior program's own lesson, because
an append-only session whose last turn ends mid-emission is malformed for
every turn that follows. The partial output returns marked stopped, the
harness closes the turn with the stop reason per the grammar, and the run
stays open.

### 13.6 The measurement obligation

**This crate produces what the record requires, at production time, and the
harness authors it.** `weaver-trace-PRD` section 3.1 is authoritative for
the payload's field list. What this charter fixes is the producing side:
the per-token signals are computed against the pre-sampler distribution,
positionally paired with the token identifiers, and absent rather than
zeroed when not produced, because an empty vector and a certain model are
different facts. The timings, the model's identity with its weights hash,
the template identity, and the block partition of 13.4 travel with the
generation, and nothing is retained here afterward: produced, reported,
gone, per section 3's no-state rule.

### 13.7 Residual readout

**The election governs production, and this crate is where production
happens.** The agent's configuration elects the readout per load, per apex
section 4's definition of done, and the election reaches this crate at
admit with the binding. Elected, the per-layer activations are reduced in
place at the tap and the reductions return on the same path as the
generation, per apex section 3 step 6. Not elected, no tap runs and no
affordance idles. **A binding that elects readout against a backend whose
engine cannot tap refuses at admit,** because a load that grants an
observability election it cannot honor fails at its cheapest moment or
lies at its most expensive one.

### 13.8 Sampling, and the dispositions

**Every sampling knob carries a disposition, and seed is a knob.** Per the
composability ruling of 2026-08-02: each knob is frozen at the worker's
composition root or left operator-tunable, the builder's election per knob,
and the effective values are recorded whichever side set them, because a
disposition changes who sets a value and never whether the record holds
it. The prior program never made its seed configurable at all, so the
disposition mechanism is the seed's first real home, and a frozen seed
plus a frozen sampling surface is what makes a binary's declared starting
field re-enterable, per apex section 8's stochastic re-entry arrangement.
The knob enumeration and its types are the Spec's and the floor's.

### 13.9 The flush

**The harness owns the decision and this seam carries it.** The flush is
the harness's ask on this seam, and its outcome is defined against the
prefix invariant: after a flush the identity prefix is resident and the
accumulated turns are gone. Where a family's state permits truncation the
outcome is reached by truncation, and where it cannot roll back it is
reached by re-establishing the prefix fresh, the invariant being the
outcome rather than the mechanism, and the mechanism the Spec's per
family. The harness still holds no handle to the cache and touches
nothing, per section 2.

### 13.10 The faults this submodule raises

The enumeration section 10's cell waited on, each a fault the worker
survives, reported to the harness and authored by it as the `fault` event:

- **Device fault during generation.** The device errored mid-forward and
  the generation cannot complete. The turn fails, the residency's
  continued fitness is this crate's next answer, and the report names the
  device's account of itself.
- **Residency degraded.** The weights or the session state are no longer
  servable, discovered outside any single generation.
- **Readout fault while elected.** The tap failed with readout elected,
  the generation itself surviving, reported because an elected
  observability that silently stopped observing is the lie 13.7 refuses.

An overflow is a typed refusal on the exchange rather than a fault, and a
death is observed through closure rather than reported, both per the
contracts. The gate's cases arrive with its turn direction, and the
corpus-wide case set closes there, per section 10.

## 14. The family libraries

**Everything a model family defines is defined once, in that family's
module, and nowhere else.** A family, Qwen or Gemma or the Harmony
speakers, shares its template, its marker vocabulary, its tokenizer
conventions, its configuration shapes, and its orchestration quirks across
every operation type that serves its models, and the prior program proved
the premise concretely: its encoder's text tower was a decoder-family
member, sharing architecture and tokenizer with the decoder path that
never knew it. The module discipline is the boundary: one module per
family holding the template, the types, the parsing and rendering, and the
forward orchestration, with nothing family-specific living outside its
module and the kernels shared beneath, which is the prior program's own
share-kernels-own-orchestration rule promoted to a charter line.

**Both directions of the end-to-end template requirement live here and are
tested here.** Inbound, the family module is what makes input formatted
correctly for the family, and the reference test shape is the prior
program's marker promotion, every control marker tokenizing to exactly
one token because a degraded marker is structure read as prose. Outbound,
the family module's parsers are the recorded bridge from the verbatim
emission to the canonical form, with parse failures carried as their own
distinct facts rather than collapsed into clean turns.

**The modules become member crates when the second consumer exists, and
not before.** This round the decoder is the only consumer, and a shared
library with one consumer is a reserved slot by apex section 9's own
test. The boundary is drawn now so the extraction is a local move in the
act that charters the encoder, which is the reversibility test applied
to this crate's own future, and the destination is recorded where
destinations live rather than here.

**A family declares its capabilities and the declaration is consulted at
admit.** What operations a family's models serve, what template identity
it renders, and whether its engine can tap for readout are facts the
family module states, and admission judges a binding against them, which
is how 13.7's refusal knows to fire. The declaration's shape is the
Spec's.
