# weaver-harness / weaver-state - contract

**Status:** MERGED. In `main` and the source of truth.

**Revised:** 2026-08-24, the replay ask joins the vocabulary. The ask set
gains `replay` by section 7's own door, elected against the diagnostic
replay loop's real need per the loop act of this date: the session's
elected events whole, in landing order, and answered only at a seal where
the member stands with the preload door, whatever the door's transport is
doing, so the loop never walks a prefix that looks whole and never answers
before the driver has dialed. The parked ask steps out of the arrival
order as section 2's stated exception, later asks passing it, answers
naming the ask they answer, and a second replay ask replacing the
parked one, the first cleared unanswered, because the asker's bound
expires invisibly to the custodian and the retry must not find the channel
jammed by an ask nobody is waiting on. Its view is the seal's position
rather than
the ask's, the one exception to the pre-ask snapshot, stated at that
clause. Section 2 shapes it, section 8 gains its
conformance, and every
party merges in the act.
**Revised:** 2026-08-20, the opener names its session. The `election`
term gains the session the load declared, so the custodian can bound its
answers to it. The ground is `weaver-state-PRD` section 4's within-a-session
ruling, which the code could not honor while the member learned a session
only as a column on arriving distillates. Per the operator's ruling of this
date the fact rides the opener rather than the ask, being load-declared and
standing for the channel's life.
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
session the load declared and the elected kinds with their payload key paths
as the load declared them, sent whole at every standing of the channel and
never per event. **The session rides the opener rather than the ask**, per
the operator's ruling of 2026-08-20 on the custody defect: it is a
load-declared fact standing for the channel's life, the same shape as the
election it sits beside, and a restarted member relearns it with its
reopened channel exactly as it relearns the election. An asker naming its
own session on every question would put the fact on the wire per ask and
would make the asking loop state something it has no reason to know. What
the member does with it is its own, per section 2: the holdings answer
within the declared session and not across it, which is
`weaver-state-PRD` section 4's boundary made reachable rather than assumed. The
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
sent after it is. The parked replay ask of the paragraph below is this
clause's one exception, its view being the seal's position rather than the
ask's: it parks precisely because the holdings it is for arrive after it,
so every distillate received through the seal is in its answer's view. The
shape and recall asks keep the pre-ask view without exception.

**Every answer is bounded to the session the opener declared**, per the
operator's ruling of 2026-08-20. Holdings a member accumulated under an
earlier session are outside every answer's view, whatever else is true of
them, so an ask cannot reach across the boundary `weaver-state-PRD` section
4 draws. This is a property of the answers rather than of the file: what
becomes of an earlier session's rows on disk is deliberately not settled
here, per the same ruling, and stands as its own question.

**The ask vocabulary is closed and enumerated here, and it holds three
names: `shape`, `recall`, and `replay`.** The shape ask carries no members, one
member instance holding
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
summarize, or drop in the rebuilt context is the loop's.

**The `replay` ask returns the session's elected events whole, in landing
order**, added 2026-08-24 against the diagnostic replay loop's need: the
loop walks a preloaded session positionally, and the four message kinds the
recall serves are less than a replay reads, the rendered contributions and
the recorded measurements being the point. The ask carries no members and
the answer serves every held event of the declared session as the
distillate's own shape, envelope and pairs, in landing order, no more
replayable than it was distillable.

**On a member standing with the preload door, a replay ask answers only at
a seal.** The preload door of `weaver-analysis-state-contract` seals its
stream, and the ask parks until a seal has landed, whatever the door's
transport is doing: not yet dialed, open mid-stream, or closed without the
seal all park it alike. The last is the clause's point, because a dead
driver's channel closes and its prefix then looks exactly like holdings at
rest, and a retry's opener may yet retire that prefix and seal a whole
preload the parked ask should answer against. So the seal is the only fact
that answers, transport openness answers nothing, and an ask on a standing
that never seals is converted by the asker's bound into the missing answer
it always was, per section 3's bounded-wait rule. This is the one ask whose
answer may wait, the waiting is not the custodian initiating, and one
answer still follows one ask. **The parked ask steps out of the arrival
order, and that is this clause's stated exception to section 4's ordering
rule**: a shape or recall ask arriving while a replay ask parks is answered
in its own arrival order, against the holdings the stream carried before
it, and the replay's answer follows the seal whenever that is, its view
the seal's position, every distillate received through the seal in it. What keeps
the pairing unambiguous without a correlation member is that every answer
names the ask it answers, per the answer's own shape, and at most one
replay ask parks per channel: **a second replay ask arriving while one
parks replaces it**, the first cleared unanswered, the seal answering the
newest alone. Replacement is the retry's whole mechanism, because the
asker's bound is the asker's own and its expiry crosses this seam as
nothing at all: the custodian cannot clear a parked ask on a fact it
cannot see, so what clears one is the next ask or the channel's close, and
an asker whose patience ran out retries by asking again rather than by any
un-ask this seam does not carry. The cleared ask's answer is never owed,
its asker's bound having already converted it to the missing answer.
An asker that cannot tell a late replay answer from a prompt shape answer
has not read the answer's name, and no further identity crosses the seam
for it. On a member standing without the preload
door, the ask answers immediately, against the holdings the stream carried
before it, like its two siblings.

A further ask
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
  before its ask, and no other traffic ever. The one stated exception is
  the parked replay ask of section 2, which later asks lawfully pass. A
  custodian that spoke
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

The replay ask is testable against the living pair under a diagnostic load:
a preload landed and sealed, the ask answered with every elected event in
landing order, and the same ask observed waiting in all three unsealed
states, before the dial, mid-stream, and after a sealless close, rather
than answering over emptiness or a prefix, which is the perturbation
its assertion in `weaver-state-Spec` section 4 watches. The retry sequence
is its own case: a parked ask outlived by its asker's bound, a second
replay ask replacing it on the same channel, the seal landing, and the
answer arriving once, for the newest ask alone, the replaced one cleared
unanswered.

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
