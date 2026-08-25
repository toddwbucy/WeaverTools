# Diagnostic Replay Loop

**Status:** MERGED v0.3, 2026-08-24. Third state on the day it was filed, and the middle
one was wrong. v0.1 had the run's record holding what diverged. v0.2 removed the record
on the reading that a diagnostic binding authors nothing. The operator's ruling of the
same date restored the record as a different record: the run authors a diagnostic-trace
through `weaver-diagnostic`, the harness's third member, and `weaver-analysis` is the
crate outside that preloads the replay and reads what it produced. So v0.1's instinct
was right and its record was the wrong one. The workflow document for the diagnostic
replay loop, filed under the harness's `Loops/` container per the Document Format's
container entry. It argues no edges of its own: the seams it walks are declared in the
crate charters, and a graph block here would duplicate a record that already has a home.

**Document ID:** `diagnostic-replay-loop`
**Editorial:** Per the Working Rules.

---

## 1. What this loop is

The diagnostic substrate's workflow, per `weaver-diagnostic-PRD` section 3:
the loop that re-executes a finished session's forward passes under a
diagnostic binding, with whatever passive readers the load elected observing.
The null replay of that charter's section 4 is this loop run with no reader
elected, and every certified instrument after it is this loop with a reader
on, which is what the charter means by the class being authored once.

**It runs in the harness's seat, as the agent's declared loop.** The
diagnostic declaration names this loop the way any declaration names its
loop, riding `loop_file` per `weaver-harness-PRD` section 2, and it runs at
the run's opening, on the precedent the context-injection loop set: a loop
needs no arriving frame to act, and under a diagnostic binding no frame can
arrive, there being no Gate. The loop composes what the seat grants and
nothing else - the state port, the decode surface, the flush - and it mints
no port, per `weaver-harness-Spec` section 6.

**What it refuses is the substrate's three refusals**, per the charter:
nothing enters from outside, the working structure is preloaded and read
positionally, and nothing writes back to what is under examination.

## 2. The walk

The operator sequences three acts, and the loop is the third:

1. **Load the diagnostic agent.** Admin stands the interior without Gate and
   the state member with its preload door, per `weaver-agents-PRD` section 6
   and `weaver-analysis-state-contract`. The run opens, the loop takes its
   seat, and its first act is the replay ask below, so the agent at this
   moment is waiting on custody rather than idle. **This loop's entry is not
   a turn's.** A serving loop runs when work arrives through Gate, and a
   diagnostic binding has no Gate, so nothing arrives to start this one: it
   runs from the run's opening. The mechanism is the loop entry's concrete
   signature, deferred per `weaver-harness-Spec` section 6.2 and listed in
   that Spec's section 9, and this document records the difference rather
   than inventing a lifecycle event for it, a declared trigger nothing
   raises being the empty joint apex section 9 refuses. **The seat's
   chartered criterion does not reach this case and that is owed rather
   than assumed here.** Section 6.2 grants the seat "on work that arrives
   owed an answer, and on nothing else", which a diagnostic binding never
   sees, having no Gate for work to arrive through. Widening that criterion
   or writing a second one beside it is the harness Spec's act, named here
   because this loop is the first case to need it.
2. **Run the driver.** `weaver-analysis` parses the operator-held record
   outside the agent, dials the preload door as an operator principal,
   sends the election and the distillates, and seals. The election elects
   what the replay needs: the message kinds, `model.request` for each
   turn's rendered contribution and template identity, and
   `model.measurement` for the recorded token path the certification
   compares against.
3. **The loop replays.** Its replay ask answers at the seal, per
   `weaver-harness-state-contract` section 2, returning the session's
   elected events in landing order. The loop walks the turns positionally
   and re-feeds each turn's recorded contribution through the decode seam,
   and the readers elected at the load observe the forward passes.

**The walk's unit is the generation, grouped from the envelope, and a
grouping that does not resolve rejects the replay.** The answer's events
group by run and turn from their envelopes, in landing order. Turn-bearing
events feed the walk, and turnless ones, the seated prefix, the run
brackets, a flush, inform input identity and feed nothing positionally.
Within a turn, `model.request` and `model.measurement` events pair in
landing order, each request to the first unpaired measurement after it,
because a turn holds one pair per generation and tool rounds make several
generations of one turn the ordinary case. A measurement with no preceding
unpaired request, a request left unpaired at the turn's end, or counts
that disagree reject the replay before any forward pass, naming the run
and turn, and nothing is ever paired across turns: a replay over a
grouping the record does not determine would be a replay of a conversation
that never happened.

**The re-feed is by the record and never by rendering.** The loop feeds the
rendered contribution `model.request` recorded, per the operator's ruling of
2026-08-12 that the record holds the rendered form precisely so a replay
does not re-render through a template that may have changed. Tokenization
identity is verified against the recorded token identifiers of
`model.measurement`, which is `weaver-agents-PRD` section 8's reproducible
claim exercised rather than assumed, and a mismatch is a failed
certification and not a lesser reading.

**The re-feed exchange on the decode seam is owed and named here rather
than assumed.** The serving seam's append-and-generate samples, and a
replay samples nothing: what the null replay needs is a drive that appends
the recorded path, runs the forward passes, emits the measurements, and
draws no token of its own. That drive does not exist on
`weaver-harness-spu-decode-contract` today. It is the one act standing
between these papers and a running null replay, it lands on the decode
seam with the SPU and the harness merging, and this document's walk is
written against it as owed. Nothing else in this walk waits on it.

## 3. Certification, walked

The charter's section 4 procedure, as this loop performs it:

1. **Input identity first.** The loop establishes, from the answered
   holdings alone, that what it is about to feed is what the record says
   was fed: the rendered contributions, their template identities, the
   sampling parameters, the model identity and weights hash against the
   binding the load declared, and the prompt-block partition. A record
   missing what its claim requires fails here, before any forward pass,
   which is the completeness-is-claim-relative rule doing its work.
2. **The null replay.** No reader elected. The recorded path re-feeds, the
   recomputed token identifiers match the recorded ones exactly, integers,
   or the certification fails naming the first divergent position.
3. **Again, with the reader, as its own load.** The readout election rides
   the declaration and is read at the load, per `weaver-agents-PRD` section
   8, so the reader pass is a second load of the same declaration with the
   election on rather than a later phase of the first: the decode state is
   fresh by construction, each pass runs as its own run with its own run
   reference, and the two passes' records never share a bracket, which is
   what keeps the token-path comparison and the passivity claim clean of
   each other. The token path must match as before, and the reader's
   presence changing
   any token is the read failing its own passivity claim. The reader's
   vectors compare within the GPU float tolerance the apex names, per the
   charter's section 4 scoping.

**A failed certification refuses the readout and keeps the run.** What was
attempted and where it diverged lands in the run's own record, the
diagnostic-trace this run authors through `weaver-diagnostic`, per
`weaver-agents-PRD` section 6 as ruled 2026-08-24. `weaver-analysis` reads it
off the sink and judges there. An earlier form of this paragraph had the run
authoring nothing and the divergence reaching the driver as an answer, which
the same date's ruling replaced. The capture artifact act shapes what an
artifact would have carried. An uncertified replay
producing no artifact is the no-second-instrument rule's floor case: not
even the first instrument reads from a replay that did not certify.

## 4. Failure, in the loop's terms

- **The replay answer is absent, for any of its reasons.** The seat's
  replay port serves nothing alike for a leg that is down, an answer that
  is malformed, and a bound that expired on a seal that never came, per
  `weaver-harness-Spec` section 6, and the loop does not tell them apart
  by outcome: none is an empty replay. An empty session is a sealed answer
  carrying zero events and fails input identity in the ordinary way, where
  an absent answer is no answer at all, so the loop ends its run having
  replayed nothing and the account says so in the diagnostic-trace, which is
  how the operator learns it, before unloading and retrying. A dead driver's
  prefix needs no cleanup, the next preload's opener retiring it.
- **The holdings fail input identity.** The loop refuses at certification
  step one, named above.
- **The decode seam refuses mid-replay.** The loop ends the run and the record
  it authors carries where the replay stopped. **The no-write guarantee is the
  third refusal's and does not reach that record**, which is the point of
  authoring one: a partial diagnostic-trace is the honest account of a replay
  that stopped, bracketed and readable, and refusing to write it would leave
  the operator with a silence to interpret. What the guarantee scopes to is
  what a later reading would rest on: no holding and no artifact takes a byte
  from the partial work, and `weaver-analysis` produces nothing downstream of
  a replay that did not certify. The decode context the forwards
  mutated and whatever measurement work was in flight are transient, die
  with the run at the leave, and a retry is a fresh load meeting them
  never, which is the same freshness the certification's two-load split
  already rests on.

## 5. What this document does not carry

The capture artifact - identity, custody, dataset shape, quota - is
`weaver-analysis-PRD` section 4's owed act, that crate holding the reading, and this
loop writes no artifact until that act lands. The re-feed exchange is the decode seam's
owed act, named in section 2. The driver's own shape is `weaver-analysis`'s Spec, owed,
and the record's shape is `weaver-diagnostic`'s, owed beside it. This document is the
workflow that binds them, and it moves when any of them lands, per the Working Rules on
documents that cite owed acts.
