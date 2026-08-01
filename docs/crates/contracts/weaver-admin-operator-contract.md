# weaver-admin / operator - contract

**Status:** MERGED. In `main` and the source of truth for now. Written on the human's
ruling of 2026-08-01: the two external boundaries take contracts before any Spec is
written, this document and `weaver-gate-world-contract` in one act, and both are
blockers on the Spec phase. One party is an external principal rather than a crate,
which is the seam category `weaver-admin-PRD` section 10 held open, settled by that
same ruling.

**Date filed:** 2026-08-01
**Revised:** 2026-08-01, second entry. The filename gains the `-contract` suffix on
the human's correction, a contract being named as one whatever its parties, and
every citation in the corpus follows the rename in the same act.
**Revised:** 2026-08-01, third entry. Section 3's tee promise states its own bounds,
on the human's instruction: the tail forfeited to process death, the pressure
election named and left to the spec seat, and a silent shed a broken build rather
than a policy. The earlier wording promised every committed event to the stream,
which no election but blocking could have kept.
**Revised:** 2026-08-01, fourth entry. Section 3 names the stream the program's
one fault carrier, per the fault-carrier ruling of this date: faults ride it as
`fault` events, the coordination seam's alert exchange retired in the same act.
**Document ID:** `weaver-admin-operator-contract`
**Parent:** `WeaverTools-PRD`, invariant 5.3
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The agreement over the operator surface: how an operator's asks enter the program and
how the program's state and its one output leave it. It governs the socket
`weaver-admin-PRD` section 8 names, and it is read alongside that charter.

It carries no representation beyond the two format rulings of sections 2 and 3. Those
are stated here rather than in a Spec because the party on the far end is outside the
program and builds against this page and nothing else. What a field list contains is
the Spec's. What shape a message takes on the wire is this document's.

**The filename carries the `-contract` suffix like every contract, on the human's
correction of 2026-08-01.** A contract is named as one whatever its parties, per
Document Format section 2, and the external category is carried by the party prose
below and by the party-edge category the Format is owed per section 8, never by a
withheld suffix. An earlier form of this document withheld it as an
exclusion-by-naming mark and was corrected the same day.

```graph
node: weaver-admin-operator-contract
kind: document

edge: party
from: weaver-admin-operator-contract
to: weaver-admin
```

The second party is the operator, a human role reaching the service through
membership in the `weaver-admin` group, per `weaver-admin-PRD` section 7. The graph
carries no node for a principal outside the program, so the party is named in prose
and the missing category is registered rather than improvised.

## 1. The channel

A localhost Unix socket, named, bound by the `weaver-admin` service, reachable only
from the host. It authenticates every connection by peer credential and admits
members of the `weaver-admin` group, which is the `peer-identity` and
`authorization-predicate` pair of `weaver-types-PRD` section 2.2 finding one of its
two consumers, the other being the gate's client socket. It is not network ingress
and breaches nothing Gate holds, on the grounds `weaver-admin-PRD` section 3 states.

## 2. What crosses in, and its format

The operator's asks, and no work. The ask set is the surface the admin charter's
section 8 names:

- the three verbs, `load`, `unload`, and `validate`, each naming an agent
- the stop conveyance, one bit and no work, naming an agent
- the observations, `list` and `show`, which transition nothing

**The format is newline-delimited JSON, UTF-8, one request per line.** The field list
of each request is the Spec's. The framing is this document's, because the operator's
tooling is built against it. A request that does not parse as one JSON value on one
line is refused as malformed, before any field is read.

No prompt, turn, task, or run crosses, per apex section 6, and a request carrying
work is refused whole rather than partially served.

## 3. What crosses out, and its format

**Answers.** Every request receives exactly one answer: the state asked for, the
aggregate of a directed transition, the turn's fate on a stop, or a typed
`lifecycle-refusal`. Answers are newline-delimited JSON on the same connection, one
answer per request, in request order.

**The output stream, which is the program's one output.** Per the human's ruling of
2026-08-01: the program tees the exact content of the working structure to an NDJSON
stream, one event per line, authored against the durable event schema `weaver-trace`
owns, and hands it to the operator. **Durability is the operator's responsibility and
not the program's.** Retention, indexing, persistence, and every view built on the
record are backend work, served by separate tooling on the operator's own compute,
addressed to that operator's specific needs, and outside this program entirely.

**What the program promises is the tee, and the promise states its own bounds.**
What reaches the stream is in order and identical in content to what the working
structure holds. What can fail to reach it is bounded and named, twice and no more.
The writer's queue trails the working structure and its tail is forfeited to process
death, per `weaver-trace-PRD` section 4.2, an abrupt exit covering nothing the queue
still held. And under sustained pressure from a slow sink the stream side does what
the tee back-pressure election rules, blocking the emitter, shedding with the gap
marked in the stream, or detaching with the detachment marked, an election the spec
seat takes against a real consumer at a real rate and nothing this contract
pre-decides. Nothing is ever shed silently: a stream that lost events says so in the
stream, so a record with no marks and a record that lost something are never
confusable, and a silent drop is a broken build rather than a policy.

**Where the stream lands is the operator's declaration.** The sink is named in the
agent's configuration, validated by admin at load like every other field, and the
stream is connected to it under admin's own principal. A file, a pipe, and a socket
into the operator's tooling are all conforming sinks, and the program treats them
alike: it writes the stream and holds no opinion about what stands behind the
descriptor. The mechanism is the Spec's.

**Custody survives the ruling.** The agent uid reaches neither this socket nor the
stream's sink, per the boundary `weaver-admin-PRD` section 2 constructs, so the
record's exclusion of the agent does not depend on who persists it.

**The stream is also the program's one fault carrier,** per the fault-carrier
ruling of 2026-08-01. A fault the worker survives rides it as the `fault` event of
`weaver-trace-PRD` section 3.1, in order with everything else, and the operator's
tooling keys on the fault fields for its own purposes. There is no second alert
path anywhere in the program, and tooling that decides a fault warrants action
comes back through this socket with a verb, per section 6.

## 4. Ordering

- Requests on one connection are served in order, and answers return in that order.
- A transition directive is refused while another transition for the same agent is
  in flight, one directive behind one operator intent, per the coordination seam.
- The stream is ordered by the writer's queue and trails the working structure, with
  no cadence to elect and no window to tune.
- Closure of the operator's connection cancels nothing. A directed transition runs
  to its aggregate, and the answer is lost rather than the act undone.

## 5. Failure

Every refusal is admin refusing an ask, typed as `lifecycle-refusal`, and the cases
this surface adds to the lifecycle's own:

- the peer fails the predicate, refused before any content is read
- the request is malformed, refused before any field is acted on
- the request names an agent the fleet does not hold
- the request carries work

A refusal answers the request that provoked it and closes nothing. The connection
survives its own refused requests.

## 6. Prohibitions

**On admin.** It carries no work inward, however the request frames it. It answers
no request from a peer that fails the predicate, and it does not degrade the
predicate to a warning. It emits the stream to the declared sink and to no other
reader, and it repairs, reconciles, and adjudicates nothing on the way through, per
`weaver-admin-PRD` section 2.

**On the operator's tooling.** Nothing behind the sink reaches back. The stream is
one-way, and tooling that wants to act on what it reads comes back through this
socket as an ask. The monitoring is the outside's job and the verb is admin's, per
the basic loop's section 2.

## 7. Vocabulary

**Drawn from `weaver-types`:** `peer-identity`, `authorization-predicate`,
`lifecycle-refusal`.

**Drawn from `weaver-trace`:** the durable event schema, as the content of the
output stream. It is drawn as published format rather than as a linked type, which
is the contract-coupled reading `weaver-admin-PRD` section 8 states.

**Drawn from `weaver-traits`:** nothing. The clause is present with that answer
because `weaver-types-PRD` section 5 asks for it even when it is empty.

This surface's request and answer definitions are owed to `weaver-types-PRD` section
2.3 on demand, with their names pending the naming ruling `weaver-spu-PRD` section
10 holds, the same gate the residency and gate seams stand behind.

## 8. What this document changes elsewhere

- `WeaverTools-Document-Format.md`: a party-edge category for an external principal,
  this document and `weaver-gate-world-contract` being the two instances. Owed to the
  Format's next revision.
- `weaver-admin-PRD` sections 8 and 10: the surface cites this contract and the
  operator-to-service cell closes. Landed in the same act.
- `weaver-types-PRD` section 2.3: the request and answer pair, pending the naming
  ruling, per section 7.
- The durable-record cut this ruling scopes landed on 2026-08-01 as its own batch:
  the program-side checksum, the manifest, the leave-time comparison, and
  record-based session resume left the corpus, `weaver-types-PRD` section 2.1
  gained `trace-sink` on this contract's demand, and what the batch left behind is
  the enter cell `weaver-admin-PRD` section 10 holds.
