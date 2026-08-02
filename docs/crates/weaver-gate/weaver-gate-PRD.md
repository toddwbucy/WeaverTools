# weaver-gate - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth for now, per the human's ruling
of 2026-08-01 that a document on `main` is merged and not a draft. It reaches the
lifecycle half of this crate, the hook the enter and leave directives raise and
lower. The exchanges that carry work arrive with the token workflow.

**Date filed:** 2026-07-31
**Revised:** 2026-07-31. The full re-draft the edit register of this date orders. The
network face, the external dialect ruling, and the acquired-parts argument for the
domain die. The local hook, the opaque pass-through, the boundary predicate, and the
lifecycle position live, and the prior draft's lifecycle mechanics, refusal
discipline, identity posture, and open cells are carried where the demotion does not
reach them.
**Revised:** 2026-07-31, again, on the human's correction. The gate is its own organ
with its job simplified, not a member of the harness domain: the re-draft had read
the dying domain-root argument as a demotion out of organ standing, and the crate
returns to its own root, its parent to the apex, and its seam to its own declaration
under the organ rule.
**Revised:** 2026-07-31, once more. The live-view mention leaves section 3, the live
view retired under ruling A of the subtraction batch.
**Revised:** 2026-08-01. The status moves from draft to merged on the human's ruling
of this date, and the on-merge edits of section 11 land in the same act:
`gate-instruction` enters `weaver-types-PRD` section 2.1, the predicate citation
lands in 2.2, and `weaver-harness-PRD` section 4 resolves turn ingress through the
contract. Section 10's client-boundary and wire-framing cells close against
`weaver-gate-world-contract`, written this date on the same ruling.
**Revised:** 2026-08-01, again, the fault-carrier ruling. Section 8's staged fault
cases land as `fault` events on the stream rather than in a shape a pending ruling
would give them.
**Revised:** 2026-08-01, further, the naming ruling. The section 11 entry for the
seam's wire pair dissolves, the contract drawing loop 0's trio.
**Revised:** 2026-08-01, once more, on review. Section 2's one-listening-socket
claim scopes from the program to the agent, admin's operator surface having
falsified the wider wording.
**Revised:** 2026-08-02, the token workflow's gate act. Section 13 arrives with
the workflow, chartering the relay, the concurrency resolution, the lower with
traffic present, and the fault cases this crate raises, which closes the
corpus-wide `fault` case set. Section 8's deferred list resolves to what
section 13 now carries, keeping streaming and its backpressure deferred by
name, and section 9's staged list is ratified in place rather than awaiting a
pass that has arrived.
**Document ID:** `weaver-gate-PRD`
**Parent:** `WeaverTools-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The charter of the crate that holds the agent's mouth and ears. It is drafted together
with `weaver-harness-gate-contract`, which governs the one seam this crate holds, and
neither is complete without the other.

Level discipline, stated once. This document carries what the crate needs and why,
including the order in which the hook rises and falls, because that order is this
crate's own work rather than a wire agreement. What crosses the seam, what it means,
and how it fails is the contract's. How any of it is represented is the Spec's and
appears in neither.

**What this charter reaches, and what it holds open.** It reaches the two exchanges
the enter and leave directives of `weaver-admin-harness-contract` section 3 fan out
into on this seam, the hook those exchanges raise and lower, the boundary predicate at
the client socket, and the process facts a charter has to state before another crate
can build against it. It does not reach the traffic: the turn exchanges this crate
will open toward the harness, the framing a client speaks at the socket, streaming,
backpressure, cancellation, drain on stop, or the fault cases a running hook raises.
Each of those is named as deferred in section 8 or as a cell in section 10 rather than
left out, because an omission and a deferral read alike to a later reader and only one
of them is a decision.

The test that drew the line is mechanical. A clause not needed to make enter and leave
true is out of this pass.

```graph
node: weaver-gate
kind: crate

edge: parent
from: weaver-gate
to: WeaverTools

edge: floor-link
from: weaver-gate
to: weaver-types
```

## 1. What this crate is

**The agent's mouth and ears, and nothing else.** A local Unix socket hook on the
front of the agent: work in, response out, both passed through opaque, with no
translation and no opinions about content in either direction. Whoever connects gets
to converse with the agent and gets nothing else. There is no listening network socket
here and none anywhere in the program, per the demotion ruling of 2026-07-31, whose
apex correction is deposited on the correction list. What the earlier reading built as
a network face, protocol termination and a ruled dialect and a translation layer, dies
with that ruling, and what remains is the thinnest boundary that lets a local client
reach a loaded agent.

**It is an organ with its job simplified, and a domain root with no members.** The
organ test of apex section 5.4 is a domain and a duplex channel with the harness,
both properties and neither alone, and this crate passes it: its domain is the
agent's external boundary, the crossing that brackets every turn, and its channel
with the harness is duplex, this pass chartering the direction the harness opens and
the turn direction arriving with the token workflow, the same half-chartered shape
the SPU carries. What the demotion kills is the acquired-parts argument, protocol
termination and authentication and translation as future member crates, so the root
holds one PRD and no subtree, which the Document Format's depth rule names as the
ordinary state of a root whose members have not been discovered. Admin brackets the
run and this crate brackets the turn, the same mechanism pointed at two scopes, and
neither is a member of the domain it borders.

**It writes nothing.** The harness is the sole writer of the trace, and both of a
turn's crossings are witnessed by the harness's authorship, the open off the inbound
crossing and the close at the final answer, per `weaver-harness-PRD` section 5. A
logged pass-through means the crossings land in the record, never that this crate
logs.

**A tool reaching outward is not ingress.** An outbound connection made by a tool
under apex step 7 does not pass through this crate. The hook is about what enters, and
the only thing that enters is work.

## 2. What this crate owns

**The hook.** This crate binds the one listening socket the agent has, of any
kind: a named local Unix socket. The claim is scoped to the agent deliberately,
the operator surface of `weaver-admin-operator-contract` being a listening local
socket that stands outside every agent, per apex section 12. Ready means bound.
Stopped means closed. The socket it binds is not this crate's decision: it
arrives as the gate instruction inside the
enter fan-out, operator-declared and admin-validated, carried by the harness
uninterpreted, resolved here. The parallel to `model-binding` is exact and deliberate.

**The boundary predicate.** A named socket is dialable by anything that can reach it,
so the hook authenticates every connection by peer credential and admits front-end
principals only, which is the `peer-identity` and `authorization-predicate` pair of
`weaver-types-PRD` section 2.2 finding the consumer that subsection describes: a seam
that admits an outside principal, authenticated by `SO_PEERCRED` and judged by the one
shared rule. **The predicate excludes the agent uid.** An elected tool that could dial
the agent's own mouth would let the agent prompt itself through its own front door,
and the exclusion is stated here as the boundary fact, with the mechanism the Spec's.

**The relay.** Inbound bytes to the harness, outbound bytes to the client, opaque both
ways, order preserved, nothing retained after the response returns.

## 3. What this crate must not hold

**No trace authorship, and no descriptor to the record.** It holds
its seam end and its listener and nothing else, which the contract's fork discipline
makes structural rather than behavioral.

**No channel to `weaver-admin`.** Admin holds one seam, and what this crate has to
say to admin travels through the harness as hub, per `weaver-admin-PRD` section 6.

**No translation and no dialect.** The earlier draft ruled an external dialect and
this re-draft removes it. Bytes cross unread, and what a client's bytes mean is the
harness's question, per the framing cell of section 10.

**No work state.** A request forwarded is a request gone. This crate retains nothing
about a turn after the response has returned through it, holds no session, and
replays nothing.

**No reading of the agent's configuration file.** The gate instruction arrives on the
seam. A crate that read the file would put a second reading of the operator's
declaration beside admin's, the same argument the SPU charter makes for the model
binding.

**No process it spawns, and no second listener.** One socket, bound once per raise,
is the whole of this crate's surface area.

## 4. Raising and lowering, from this crate's side

**Raising.** The harness forks this crate last in the enter fan-out, after the SPU
has confirmed residency, over a pair the harness creates before the fork. The order is
apex section 6's binding rule, and the reason is the boundary: a hook that rises
before the interior is whole would accept work the agent cannot yet do. On its side of
the fork this crate performs its final exec, sets its own end of the channel
close-on-exec, clears its own dumpable flag, receives the gate instruction, binds the
socket the instruction names, and confirms ready. Ready is a fact about the listener,
not about the process: it is sent only after the bind has returned.

**Refusing.** A bind that fails is a refusal with the reason, typed, and a refusal
leaves nothing held: no listener, no half-bound socket, nothing a retry would trip
over. A refusal is answered to the harness rather than exited on, for the reason the
SPU charter states, a party that exited would replace a typed reason with an observed
death.

**Lowering.** The harness stops this crate first in the leave fan-out. Stopping
closes the listener before anything else happens anywhere else, which is what
stopped-first means and why it is first. Confirmation of stopped is sent only after
the listener is closed. What happens to a connection accepted and in flight at that
moment is drain, and drain is the token workflow's, deferred in section 8. In this
pass no traffic exists, so closed is the whole of it.

**Never outliving.** A gate process never outlives the worker interior it protects,
per apex section 6. The mechanism this pass states is the channel: the pair closes
when the harness is gone, and a gate that observes closure closes its listener and
exits. Whether a second mechanism backs this is the Spec's choice, and the requirement
does not depend on it.

## 5. What a failure partway through leaves behind

A refusal at the bind leaves the agent interior loaded and the aggregate not ready,
and the rollback that follows is `weaver-admin-harness-contract` section 3's, not this
crate's. This crate's obligation is that its refusal is true: nothing held, so the
rollback has nothing of this crate's to unwind.

A gate death after ready is the loss of the agent's reachability, observed by the
harness as channel closure. What the harness does with that observation is the
coordination seam's. This crate holds nothing whose loss corrupts the interior, so its
death is an availability fact and not an integrity fact.

## 6. The seam

One seam, with the harness, governed by `weaver-harness-gate-contract`, drafted with
this charter as one act.

**The organ declares, and the record is here.** This is an organ channel, so Document
Format section 4's organ rule reaches it: the organ declares and the harness does
not, because the harness is the hub every organ is duplex with and a hub that
declared its own edges would carry the whole seam graph in one crate.
`weaver-harness-PRD` section 4 points at the contract and gains no record, per
section 11.

```graph
edge: seam
from: weaver-gate
to: weaver-harness
via: weaver-harness-gate-contract
tag: socket
```

**The contract's name stands as filed.** The Format names a contract for its parties,
initiator first, and the harness initiates the governed signals: the data flows
inward, the ask flows outward, and the name follows the ask. The open-items note that
the direction was unsettled settles with this act. When the token workflow adds
gate-opened turn exchanges the channel carries two initiators and the name does not
change, because the name records the governed signal's initiator and not the traffic's
direction.

## 7. Identity, process boundaries, and the tool-uid cell

This crate runs as the agent uid, `weaver-<n>`, inside the boundary
`weaver-admin-PRD` section 7 constructs. After its final exec it clears its own
dumpable flag and sets its own channel end close-on-exec, the same two acts in the
same order the SPU charter states, and for the same reason: an outward-facing process
in the agent's uid is exactly the process a same-uid attach should not reach.

**The cell this charter inherits.** Whether external tool processes run as the agent
uid or under a uid this crate owns is this crate's to rule, per `weaver-admin-PRD`
section 7, which names the agent-uid case as its assumption and files the cell here so
it is inherited as a constraint rather than rediscovered.

**PENDING, ruling candidate for ratification.** Stage one ratifies the agent-uid
case. Apex step 7 is merged and describes the reference tool running as the agent's
own constrained Linux user, bounded by the kernel through filesystem permissions,
sudoers, and cgroups, and this charter may not contradict a merged document. Under
that case the boundary between the agent and its worker is hardening rather than
kernel-enforced separation, and the hardening is named: the cleared dumpable flag on
every process of the worker's family, and close-on-exec on every descriptor at every
fork, including the creating party's own ends. The separate-uid arm, under which the
worker's descriptors become unreachable from tool code by construction at the cost of
an explicit grant for every reach into the agent's home, is staged in section 8, and
what it awaits is the tool workflow chartering plus a stated threat measurement,
because a boundary upgrade priced without a threat model is a motive wearing a
mechanic's clothes.

## 8. What does not cross, and what waits

**Chartered by the token workflow's act of 2026-08-02, no longer deferred:** the
turn exchange this crate opens toward the harness, how a turn is identified
across the boundary, concurrent clients against the one-turn loop, drain on
stop, and the fault cases a running hook raises. Each lands in section 13,
which is this charter's next section arriving with its workflow.

**Still deferred, and named rather than omitted:** streaming responses and the
connection the hook holds open while the interior generates, and the
backpressure that rides them. `weaver-gate-world-contract` section 3 makes one
line in and one line out the resting shape and streaming an extension to that
page rather than a replacement, so it arrives when a client needs it and this
crate gains nothing in anticipation.

**Staged:** the separate-uid arm of the section 7 cell.

None of these acquires a trait, a variant, a feature flag, or a config field in this
pass. A charter naming a domain boundary is a decided boundary. An unbuilt interface
waiting to be filled is the thing apex section 9 forbids.

## 9. Staged requirements

**The token workflow arrived on 2026-08-02 and ratified this list in place.**
The forwarded prompt reaches the harness with the turn not yet in existence, so
this crate carries no `turn_key` inward and mints none. Responses return by the
path the request took. This crate sees octets and nothing of the model, the
trace, or the session. Section 13 is where each of these now binds, and this
section keeps them as the record of what the earlier pass could already see.

## 10. Open cells

**The tool-uid ruling.** Stated in section 7 with its candidate. **Settled by:** the
architecture seat's ratification of the candidate, or the tool workflow chartering
with a threat measurement if the ratification is declined. Until settled, admin's
assumption stands and nothing builds against the separate-uid arm.

**The refusal type.** This seam's refusal reuses `lifecycle-refusal` rather than
taking a definition of its own, on the grounds the SPU charter states: the refusal has
to reach admin inside the enter aggregate unchanged. Whether the case set grows to
hold bind failure is the cell `weaver-spu-PRD` section 10 holds for its cases, and
this charter rides that cell rather than opening a twin. **Settled by:** the ruling
that settles it there.

**Wire framing at the client socket is closed.** Newline-delimited JSON, one request
line and one response line per turn, ruled at `weaver-gate-world-contract` section 2 on
2026-08-01. The field shapes stay with the spec seat and streaming with the token
workflow, per that contract's section 3.

**The gate instruction's home is closed.** A field of the agent's configuration
file, defined at `weaver-types-PRD` section 2.1 beside `model-binding`, on the
identical grounds: the operator writes it, admin validates it, the harness carries
it, this crate resolves it. The types edit landed with the merge of this charter on
2026-08-01.

**The client boundary's seam category is closed.** The client socket takes a
contract of its own, `weaver-gate-world-contract`, per the human's ruling of
2026-08-01 that the external boundaries are contracted before any Spec. The party
category the
Document Format lacked is owed to the Format by `weaver-admin-operator-contract`
section 8, one register entry for both instances.

## 11. Edits owed in the same act

Named here because a document whose reach cannot be read for the reach is a trap.
These are owed by this act and this list is the authoritative one under G5. An entry
leaves this register when the edit lands.

**Four left on 2026-08-01, with the merge of this pair.** The `gate-instruction`
field landed in `weaver-types-PRD` section 2.1, the predicate's consumer citation in
2.2, the turn-ingress resolution in `weaver-harness-PRD` section 4, and the
open-items entry on this contract's name settled with the name standing as filed.

- The section 2.3 entry this register carried dissolved with the naming ruling of
  2026-08-01: the seam draws loop 0's trio and owes the floor nothing, per the
  contract's section 7.
- `WeaverTools-PRD`: the gate's rows and prose join the correction list's demotion
  entry, the component table, the section 3 path, and the organ framing of section 6,
  all filing with the re-authoring.

## 12. Children

Specs to be written against this charter once the PRD set is ratified. Named so the
set is bounded, not drafted here, and incomplete for the same reason the charter is.

- The hook, covering the bind, the instruction's resolution, the predicate, and the
  refusal set.
- The fork and the descriptor discipline of section 7.
- Stop, covering the close and the confirmation ordering.

## 13. Serving the turn

Arrived with the token workflow's act of 2026-08-02, filed after the children
for the reason `weaver-spu-PRD` section 13 states: sections 8 through 12 are
cited by number across the merged corpus, and arrival order is filing order.
Everything here is derived from `weaver-harness-gate-contract` section 2 as
that act extends it, `weaver-gate-world-contract`, and the turn grammar of
`basic-inference-loop`.

### 13.1 The relay

**One exchange per client request, opened by this crate, opaque both ways.**
A request admitted at the hook becomes an exchange opened toward the harness
carrying the client's line as octets this crate has not read. The response
returns on that exchange and goes back out the connection it came in on. The
exchange's own identity is the correlation, so this crate holds no table of
its own and invents no identifier: what it needs to route a response is the
thing the channel already gives it.

**The turn is not this crate's to name.** No `turn_key` crosses inward,
because the turn does not exist until the harness opens it, per section 9,
and nothing here mints one. What this crate knows about a turn is that a
line went in and a line came out.

**Retention ends at the answer.** Per section 3, nothing about a turn
survives the response returning through it, and the exchange's close is where
that is enforced rather than promised.

### 13.2 Concurrency, and where it resolves

**Clients may speak at once and this crate relays as they do.** More than one
exchange may be open toward the harness, and the harness serves them one turn
at a time in arrival order, per the contract's ratified ruling. This crate
refuses nothing on the grounds that the interior is busy, because waiting is
what a conversation already means and a busy interior is not a boundary
condition. Order per connection holds in both directions, which is what a
client is owed and the whole of what it is owed: no promise is made across
clients, and none is needed, since a client sees only its own connection.

### 13.3 Lowering with traffic present

**Drain is modest by construction and this section states why.** A lower
arrives only when the run is at rest, per the coordination seam's own rule
that leave refuses while a turn is in flight, so no turn is outstanding when
the listener closes. What remains is connections a client is holding open,
which this crate closes after the listener and before answering stopped. A
client that reconnects finds no listener, which is refusal by absence and the
boundary the lifecycle protects.

### 13.4 The faults this crate raises

The enumeration section 8 deferred, each a fault the worker survives,
reported to the harness on the exchange the contract names and authored by
the harness as the `fault` event:

- **The listener is lost.** The bound socket became unusable while the hook
  was raised, so the agent is unreachable with its interior healthy. This is
  the reference case, and it is why the fault direction exists at all: the
  harness cannot observe this from its side, the channel being fine while
  the boundary is gone.
- **A client connection failed mid-turn.** The response could not be
  delivered because the client is gone. The record already holds the turn's
  close, per `weaver-gate-world-contract` section 3, so this reports a lost
  delivery rather than a lost turn, which is the distinction that keeps a
  consumer from reading a delivered answer where none arrived.
- **Admission is failing systematically.** The predicate is refusing every
  peer, which is either a boundary the operator misconfigured or something
  probing it, and either way the operator wants it in the record.

A peer failing the predicate is not a fault. It is the boundary working, and
a fault event per refused dial would make the record noisiest exactly when
the boundary is doing its job.

**With these named the corpus-wide case set closes across all three organs.**
The `fault` event's cases are the SPU's at `weaver-spu-PRD` section 13.10,
this crate's here, and the harness's own three at `weaver-harness-PRD`
section 5, that last enumeration landing in this same act on the review
seat's finding, and it cannot be derived from the other two because an organ
death is reported by the party that survives rather than the one that died.
The payload's shape lands with the trace act against the closed set rather
than against a guess.
