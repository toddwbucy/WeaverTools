# weaver-harness / weaver-state - contract

**Status:** MERGED. In `main` and the source of truth.

**Revised:** 2026-08-19, third of this date, recall joins the asks. The
vocabulary's ask set gains `recall` by section 7's own door, elected
against the context-management loop's real need per issue #221's arc: a
decode context is a working set the loop rebuilds after a flush, and the
rebuilding material is custody's. Section 2 shapes the ask and its
answer, section 8 gains its conformance.
**Revised:** 2026-08-19, the serve direction takes its shape. The vocabulary
gains the `ask` and the `answer` and the one ask name `shape`, section 2's
serve paragraph replaces its cell-face with the flowing direction, sections
3 through 5 gain each party's serve obligations and the serve failure
vocabulary, section 7 records the change this protocol named as landed, and
section 8 gains the serve conformance. The change arrives with the
context-injection loop's act, which is the landing the charter's cell
required, every party merging in it.
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

**This seam's own.** Four terms. The `election`: the seam's opener, the
elected kinds and their payload key paths as the load declared them, sent
whole at every standing of the channel and never per event. The
`distillate`: one distilled event, carrying the envelope whole and the
elected payload pairs beside it, each pair a payload key path and the value
the canonical JSON held at it. The `ask`: one question the harness puts to
the holdings on the standing channel, carrying a name from the closed
vocabulary section 2 enumerates. The `answer`: the custodian's one reply to
one well-formed ask, sent only when asked and at no other time.

```graph
edge: draws
from: weaver-harness-state-contract
to: canonical-event

node: distillate
kind: term

edge: defines
from: weaver-harness-state-contract
to: distillate

node: election
kind: term

edge: defines
from: weaver-harness-state-contract
to: election

node: ask
kind: term

edge: defines
from: weaver-harness-state-contract
to: ask

node: answer
kind: term

edge: defines
from: weaver-harness-state-contract
to: answer
```

## 1. What this contract governs

The one seam between the harness and its state member: the channel's standing,
the ingest traffic that flows today, the serve direction that is chartered and
unshaped, what each party owes, how the seam fails, and what neither party may
do. It is read alongside `weaver-state-PRD` and neither is complete without
the other.

## 2. The traffic

**Ingest, flowing, one direction, and the election opens it.** The first
traffic on every standing of the channel is the election itself, whole: the
elected kinds and their payload key paths, as the load declared them. The
custodian needs the election before the first distillate, because its
indexes are built from it at load, and a restarted member receives the
identical election with its reopened channel, which is what keeps the
selection deterministic across the processes of one load. After the opener,
the harness sends a `distillate` per elected event, in sequence order, and
is owed nothing back: the fact has one home, state's holdings, and a
confirmation whose one reader would discard it is the retired receipt's
error again. The harness does not wait, and a distillate is not a turn's
work: nothing about a turn's completion depends on this seam accepting
anything.

**Serve, flowing, the second direction, asked and answered on the one
channel.** The cell that stood here closed 2026-08-19: the first asker
arrived as the context-injection loop and the shape below is elected
against its real ask, per `weaver-state-PRD` section 5. After the ingest's
opener, the harness may put an `ask` on the standing channel at any point
in the stream, and the custodian sends exactly one `answer` per well-formed
ask, in the order the asks arrived, and speaks at no other time. **An ask
is answered against exactly the holdings the seam carried before it**,
which is what makes a served fact attributable to a position in the stream:
every distillate sent ahead of the ask is in the answer's view and nothing
sent after it is.

**The ask vocabulary is closed and enumerated here, and it holds two
names: `shape` and `recall`.** The shape ask carries no members, one member instance holding
one session, and asks for the session's shape - what happened, in what
order, in which run, which is the phrase the charter uses for what the
default election holds. Its answer carries the session's runs in the order
custody first saw them, each with its run reference and its held event
counts by kind, every name spelled as the envelope spelled it. The counts
are organized envelope fact and carry no judgment: what a kind's count
means to a turn is the asking loop's business, per the three-way division
of `weaver-state-PRD` section 2.

**The `recall` ask returns the conversation as custody holds it**, added
2026-08-19 against the context-management loop's need: after a flush the
decode context is empty and the session's knowledge is not, so the loop
asks for the material and composes its own re-entry. The ask carries one
optional member, `last-turns`: a count bounding the answer to the most
recent turns, absent meaning the session whole. The answer carries the
events of the four message kinds in landing order, each with its envelope
whole and its elected pairs beside it - the distillate's own shape served
back - so what returns is exactly what the election kept, no more
recallable than it was distillable. Selection bounds and ordering are
custody's organizing licence, and every judgment about what to keep,
summarize, or drop in the rebuilt context is the loop's. A further ask
name is a change under section 7 and does not exist until it merges
there.

## 3. What the harness owes

- **The election applied faithfully.** Every event matching the election
  crosses, whole per the election, in the order the record assigned. The
  harness neither thins what was elected nor adds what was not.
- **The envelope always.** Every distillate carries all five envelope fields
  as the canonical form spelled them. An unattributable distillate is a
  defect in the sender.
- **Its own judgment kept to itself.** What a fact is worth is the harness's
  loops' business and crosses this seam in neither direction.
- **Asks from the enumerated vocabulary only, and a bounded wait.** The
  harness sends no ask this contract does not name, and it does not wait
  unboundedly for an answer: an answer that has not arrived inside the
  harness's own bound is a missing answer, treated as the dead peer of
  section 5, and the turn proceeds without the fact.

## 4. What state owes

- **Custody whole.** What arrived is held, organized, and attributable by its
  envelope, and nothing that arrived is judged, ranked, or discarded by any
  policy of the custodian's own. Retention within the session is total.
- **Transformation without judgment.** Derived shapes, aggregates, and
  indexes are custody's work and carry no opinion about what a turn should
  do, per the three-way division of `weaver-state-PRD` section 2.
- **The answer, only when asked.** Exactly one answer per well-formed ask,
  in arrival order, each answered against the holdings the stream carried
  before its ask, and no other traffic ever. A custodian that spoke
  unasked would be initiating, which its charter forbids.

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

**A dead or silent peer costs the answer and never the turn.** The serve
direction fails the way the ingest does: where state is gone, or an answer
does not arrive inside the harness's bound, the harness proceeds as if
nothing were held, the loop composes its turn without the fact, and no
retry follows on this standing of the channel. A malformed ask is dropped
by the custodian without an answer, which the harness's bound converts into
the same missing-answer outcome, and a malformed answer is dropped by the
harness to the same effect. In every one of these the record remains whole
and the next load's channel asks again against holdings that never moved.

## 6. What neither party may do

- Neither party writes the trace through this seam, in either direction. The
  distillate is a projection of the record and nothing here flows back.
- Neither party exposes this seam to the model. There is no tool, no verb,
  and no path from the loop's interior to either end.
- Neither party persists across the session through this seam. The file's
  life is `weaver-state-PRD` section 3's and no traffic here extends it.

## 7. Change protocol

A change to the distillate's shape, to the election's semantics, to the
ask vocabulary, or to the answer's shape or its ordering guarantees
touches this contract, and every party merges in the same act. The serve
direction's first shape was a change under this protocol and landed
2026-08-19 with the loop act the charter named, which is the
sentence above kept as the rule it demonstrated: a second ask name enters
by the same door.

## 8. Conformance

The ingest direction is testable against the living producer: a real load, a
real election, real events crossing, and the holdings queried for exactly
what the election named, attributable by envelope. The dead-peer clause is
testable by killing the member mid-run and watching the turn path not
notice. Both land with the code act that opens the seam.

The serve direction is testable against the living pair: a real load, real
events landed, and the shape ask answered with exactly the runs and counts
the record shows for the session, in first-seen order. The recall ask is
testable the same way: the answer's events are exactly the elected
message-kind rows, in landing order, bounded to the named turn count where
one was given, byte-faithful to what the tee carried in. The answered-against
clause is testable in time: asks interleaved with distillates, each answer
holding every count the stream carried before its ask and nothing sent
after it. The serve half of the dead-peer clause is testable by asking with
the member gone and watching the turn complete without the fact inside the
bound. All three land with the loop act that shapes the surface.
