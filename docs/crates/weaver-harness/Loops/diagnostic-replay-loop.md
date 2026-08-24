# Diagnostic Replay Loop

**Status:** MERGED v0.1, 2026-08-24. The workflow document for the diagnostic
replay loop, filed under the harness's `Loops/` container per the Document
Format's container entry. It argues no edges of its own: the seams it walks
are declared in the crate charters, and a graph block here would duplicate a
record that already has a home.

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
   and `weaver-diagnostic-state-contract`. The run opens, the loop takes its
   seat, and its first act is the replay ask below, so the agent at this
   moment is waiting on custody rather than idle.
2. **Run the driver.** `weaver-diagnostic` parses the operator-held record
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
3. **Again, with the reader.** The same replay with the elected readout on.
   The token path must match as before, and the reader's presence changing
   any token is the read failing its own passivity claim. The reader's
   vectors compare within the GPU float tolerance the apex names, per the
   charter's section 4 scoping.

**A failed certification refuses the readout and keeps the run.** The run's
record holds what was attempted and where it diverged, the capture artifact
act shaping how, and an uncertified replay producing no artifact is the
no-second-instrument rule's floor case: not even the first instrument reads
from a replay that did not certify.

## 4. Failure, in the loop's terms

- **The seal never arrives.** The replay ask's patience is the loop's own,
  per the contract's bounded-wait rule. At the bound the loop ends its run
  having replayed nothing, and the operator reads the account, unloads, and
  retries with the driver. A dead driver's prefix needs no cleanup, the
  next preload's opener retiring it.
- **The holdings fail input identity.** The loop refuses at certification
  step one, named above.
- **The decode seam refuses mid-replay.** The loop ends the run with the
  account of where, and the partial forward work writes nothing anywhere,
  the substrate being immutable by the third refusal.

## 5. What this document does not carry

The capture artifact - identity, custody, dataset shape, quota - is
`weaver-diagnostic-PRD` section 6's owed act, and this loop writes no
artifact until that act lands. The re-feed exchange is the decode seam's
owed act, named in section 2. The driver's own shape is
`weaver-diagnostic`'s Spec, owed. This document is the workflow that binds
them, and it moves when any of them lands, per the Working Rules on
documents that cite owed acts.
