# weaver-harness / weaver-state - contract

**Status:** MERGED. In `main` and the source of truth.

**Date filed:** 2026-08-18
**Document ID:** `weaver-harness-state-contract`
**Editorial:** Per the Working Rules.

---

## Parties

- **`weaver-harness`, the feeder and the asker.** Applies the tee of
  `weaver-trace-PRD` section 11 and sends what it elects across this seam. The
  only party that will ever ask, its own loops asking through it. Decides
  everything: what is elected, what any held fact is worth, and what to do
  with an answer.
- **`weaver-state`, the custodian.** Receives the distillate, holds it
  organized, and will answer asks when the ask exists. Transforms as part of
  organizing, per its charter, and decides nothing.

No third party reaches this seam. The model has no path to it, per
`weaver-state-PRD` section 2, and no other crate holds an end.

**This seam is a wire.** State crosses a process line, per apex section 9's
re-entry door, so this contract governs a protocol on a transport: a Unix
socket, stood at load, authenticated by credential per the first invariant's
rule for a channel with a name.

```graph
node: weaver-harness-state-contract
kind: document

edge: party
from: weaver-harness-state-contract
to: weaver-harness

edge: party
from: weaver-harness-state-contract
to: weaver-state
```

## Vocabulary

Every contract names the vocabulary it depends on, grouped by the crate that
defines it. A contract without this clause is not a valid contract, and a group
is stated even when empty.

**From `weaver-trace`.** The canonical event JSON and its envelope fields:
`session`, `run`, `turn`, `kind`, and `sequence`, spelled as that crate's
canonical form spells them. The distillate is a projection of the canonical
form and never a reshaping of it, so every name that crosses this seam is a
name the record already carries. The harness draws it.

**From `weaver-types`.** Nothing. The distillate's shape is this seam's own
vocabulary, defined below, and the floor carries no member for it, per the
custody rule of apex section 5.2: the floor carries only what the harness
itself consumes, and what crosses here is consumed by state.

**This seam's own.** The `distillate`: one distilled event, carrying the
envelope whole and the elected payload pairs beside it, each pair a payload
key path and the value the canonical JSON held at it. The election that
produced it is a load condition and does not ride each event. The serve
direction's vocabulary is deliberately absent, per section 3.

```graph
edge: draws
from: weaver-harness-state-contract
to: canonical-event

node: distillate
kind: term

edge: defines
from: weaver-harness-state-contract
to: distillate
```

## 1. What this contract governs

The one seam between the harness and its state member: the channel's standing,
the ingest traffic that flows today, the serve direction that is chartered and
unshaped, what each party owes, how the seam fails, and what neither party may
do. It is read alongside `weaver-state-PRD` and neither is complete without
the other.

## 2. The traffic

**Ingest, flowing, one direction.** The harness sends a `distillate` per
elected event, in sequence order, and is owed nothing back: the fact has one
home, state's holdings, and a confirmation whose one reader would discard it
is the retired receipt's error again. The harness does not wait, and a
distillate is not a turn's work: nothing about a turn's completion depends on
this seam accepting anything.

**Serve, chartered and unshaped.** The second direction exists in charter and
carries no vocabulary yet, per `weaver-state-PRD` section 5: its first asker
is the context-injection control loop, and the ask's shape is elected in that
loop's act, against real asks, never before. This section is the named cell's
contract-side face, and an implementation that invents an ask shape ahead of
that act has built a reserved slot.

## 3. What the harness owes

- **The election applied faithfully.** Every event matching the election
  crosses, whole per the election, in the order the record assigned. The
  harness neither thins what was elected nor adds what was not.
- **The envelope always.** Every distillate carries all five envelope fields
  as the canonical form spelled them. An unattributable distillate is a
  defect in the sender.
- **Its own judgment kept to itself.** What a fact is worth is the harness's
  loops' business and crosses this seam in neither direction.

## 4. What state owes

- **Custody whole.** What arrived is held, organized, and attributable by its
  envelope, and nothing that arrived is judged, ranked, or discarded by any
  policy of the custodian's own. Retention within the session is total.
- **Transformation without judgment.** Derived shapes, aggregates, and
  indexes are custody's work and carry no opinion about what a turn should
  do, per the three-way division of `weaver-state-PRD` section 2.
- **Nothing back, yet.** Until the serve direction takes its shape, state
  sends nothing on this seam. A custodian that spoke unasked would be
  initiating, which its charter forbids.

## 5. Failure vocabulary

**A dead peer costs the distillate and never the turn.** If state is gone,
the harness observes closure, drops what it would have sent, and serves turns
exactly as it did before the leg existed, per the loss clause of
`weaver-state-PRD` section 3. The holdings meanwhile stand in state's file,
and the next load's channel reopens against them. There is no buffering, no
retry, and no backpressure onto the turn path: the derivative is rebuildable
from the record, so the cheapest honest answer to a broken seam is to stop
distilling until the next load.

**A malformed distillate is the sender's defect.** State refuses it by
closing nothing: the event is dropped, the defect is state's to surface when
the serve direction gives it a voice, and the record remains authoritative
for what was elected. The seam does not fault the worker for a bad row.

## 6. What neither party may do

- Neither party writes the trace through this seam, in either direction. The
  distillate is a projection of the record and nothing here flows back.
- Neither party exposes this seam to the model. There is no tool, no verb,
  and no path from the loop's interior to either end.
- Neither party persists across the session through this seam. The file's
  life is `weaver-state-PRD` section 3's and no traffic here extends it.

## 7. Change protocol

A change to the distillate's shape, to the election's semantics, or to the
serve direction's opening touches this contract, and every party merges in
the same act. The serve direction's first shape is a change under this
protocol, landing with the loop act the charter names.

## 8. Conformance

The ingest direction is testable against the living producer: a real load, a
real election, real events crossing, and the holdings queried for exactly
what the election named, attributable by envelope. The dead-peer clause is
testable by killing the member mid-run and watching the turn path not
notice. Both land with the code act that opens the seam.
