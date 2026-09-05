# weaver-web - Spec

**Status:** DRAFT, unratified. **A rewrite of this crate's Spec, not a new
one.** Authored 2026-09-04 from the thinkpad seat beside the rewritten
`weaver-web-PRD` of the same date. The prior text is replaced whole rather than
amended, and git is its archive.

**Revised:** 2026-09-05, against the acts of that date. #435 landed at
#440 and #436 at #444, #437 answered with a finding rather than a
catalogue, and two facts the record spells corrected this document where it
had guessed: the address carries the turn, and a position is a resident
length rather than an ordinal.

**Date filed:** 2026-09-04
**Document ID:** `weaver-web-Spec`
**Editorial:** Per the Working Rules. ASCII, absolute dates.

## 0. What this document is

The charter says what this crate is for. This document says what it is made
of, in the order a reader needs it: the store first, because every surface
reads from it, then the paths into and out of that store, then the seams,
then what holds each claim.

**Where this document elects something the charter left open, it says so.**
Where it leaves something open, section 10 names it rather than letting
silence settle it.

## 1. The crate

    src/
      store/        the registry: schema, migrations, the three reads
      ingest/       the write path, a consumer of the analysis stream
      queue/        staged experiments and their states
      surfaces/     one module per surface of charter section 3
      seams/        gate client, admin verbs, analysis stream reader
      link/         the connector and server halves

**This crate links no crate of the agent workspace** and reaches the agent
exactly as an outside consumer does: a socket dialed by path, a binary run
by the operator's verb, and a record read where the operator keeps it. That
property is load-bearing and is not spent by the seam of section 7.3, which
is a stream this crate reads rather than a crate it links.

## 2. The store

The grain is the grain the interface clicks at.

### 2.1 The position

**The address is the run, the turn, and the position**, and the composite of
the three is the primary key.

**This corrects the two-part key this document first carried.** The record's
own spelling is what settles it: turn keys repeat across a serving record's
runs, so an address without the run answers one line per run, which is why
`weaver-analysis field` takes `<turn>:<position>` and an optional `--run`.

**And `position` is the resident length at the draw, not an ordinal within
the turn.** The first generated token of a sixty-token prompt is position
sixty. Verified 2026-09-05 against a field-bearing record on the pool, whose
first `model.field` event carries `position: 60`. A surface that treats the
two as one word will address the wrong token, and section 6 states the
conversion.

Each row carries:

- the emitted token, by identifier and by surface text
- the surprisal and the entropy at that position
- the ranked alternatives with their probability mass
- the raw residual where the capture holds one

**`realized` is a rank and not a token.** It names which rank the draw
landed on, so a row carries `realized` as the record spells it and the drawn
token resolved beside it, and neither is presented as the other.

**The alternative count is the declaration's field election, not this
crate's parameter.** `model.field` carries the ranked candidates under
`field-election: { depth }`, and the records on hand carry forty and fifty.
The depth is the operator's ruling, open at the charter's section 9, and
this crate stores what the election kept rather than choosing a number.

**Raw residual rides alongside rather than a projected readout**, so a lens
refitted later can read a run captured earlier. This is the standing
raw-residual ruling and this document does not disturb it.

### 2.2 The run

Everything identifying the conditions lives in the run's own row:

- artifact identity **at a grain fine enough to catch a quantization
  difference**
- seed, and the full sampler configuration
- device, and precision
- the batching election
- the task, by source and identity
- the declared boundary set
- the parent run reference and branch position, where the run is a branch
- whether a token was forced, and which

**A reading without its tuple is a reading of an unnamed compound.** The
task is in this list for the same reason the artifact is: two runs of the
same benchmark item under different suite versions are not the same task,
and nothing else in the row would say so.

### 2.3 The indexes

    primary       (run, turn, position)
    secondary     (run, surprisal)

The secondary index exists so the largest spikes in a run are reachable
without pulling the run down. **Nothing is computed at read time.** A value
that must be derived is derived once at ingest and stored, because a value
computed in the interface is a value nobody else can reproduce.

## 3. The write path

**The write path is a consumer rather than a step in the loop.** The
analysis emission leaves over its own socket, a process on this side reads
it and lands it in the store, and **the decoder never waits on the store.**

- Writes are **bulk per turn or per window**, never per token.
- Writes are **idempotent on the run-and-position key**, so a replayed or
  partially failed ingest cannot produce two truths about one position.
- **The run row is written first and closed last**, carrying a status a
  surface can read, so a partially ingested run is visibly partial rather
  than quietly short.

## 4. The read path

Three queries, and the schema of section 2 exists to make each an index hit.

1. **One position's alternatives**, by run, turn and position. This is the
   click, and it is `weaver-analysis field` served from the store rather
   than re-derived.
2. **A contiguous range of positions** carrying the emitted token, surprisal
   and entropy. This is the timeline and the transcript.
3. **The run's tuple.** This is the label on every reading taken from it.

A surface that needs a fourth query is a surface this document has not
described, and it returns here before it is built.

## 5. Staged experiments

**This crate has no model behind it and forks nothing.** A click authors a
staged experiment holding the parent run reference, the branch position, the
forced alternative token where one is forced, and the parent declaration
with its diff. A runner drains the queue, and each result returns as a run
in the schema of section 2 carrying its parent reference and branch
position, so the comparison needs no reconstruction.

### 5.1 The five states

    draft       editable in every field
    registered  frozen, the claim on the record, eligible for a batch
    queued      handed to a runner, waiting
    running     a runner holds it
    returned    results in the store

**Registration is the freeze**, not the launch. A draft is a draft and
editing one is what drafts are for, because a draft has no result its author
could have seen. Registering puts the claim on the record whether or not it
ever runs, and queueing is a separate act. **What was registered and never
run stays in the record**, which is what makes pre-registration a property
of the interface rather than a discipline imposed on it.

### 5.2 The diff is split by when it takes effect

Both kinds reach the agent through a reload, since the load boundary is the
only change boundary. They differ in **what the result licenses**:

- **Per generation** - seed, temperature, top-p, top-k, repetition,
  maximum tokens. Same weights, same window. The recorded prefix re-feeds to
  the state the parent had, so divergence below the branch position is
  attributable to the one value moved.
- **Load-time** - artifact, precision, devices, context capacity, and the
  elections. The prefix re-feeds **under different weights or a different
  window**, so the parent's internal state is not reproduced. The text
  upstream matches and the state does not, and **the comparison is
  structural rather than byte-exact.** A load-time move also derives a
  declaration, which stands in Agents beside its parent.

The interface states both consequences where the change is made.

### 5.3 Validation at authoring

An experiment is refused when it is authored, not when it is drained.

    the parent record is readable, and holds the branch position
    the context capacity holds the prefix
    the forced token is present in the capture at that position
    every artifact the diff names resolves

**A runner cannot ask.** A refusal discovered at three in the morning costs
a batch window; the same refusal at authoring costs nothing.

## 6. The surfaces

One module each, and each renders from the reads of section 4 and nothing
else. Their destinations are the charter's section 3 and are not restated.

**The state a surface holds is a query, never a location.** A filter chip is
a clause; clearing it widens the list in place. A card carries the operator
into a list with a chip already set, and no surface has a variant that
differs only by a pre-applied filter.

**A timeline is drawn on ordinals and a click addresses a position, and the
surface converts between them.** The per-generation series is indexed by the
ordinal within its generation; the field is addressed by the resident length
at the draw. They are different coordinates and the record spells both, so
the conversion is the surface's to make and to make once. A view that passes
an ordinal where a position is expected reads a different token and says
nothing about it.

**Absence renders as absence.** Entropy rides every generation
unconditionally; surprisal rides only where its election stands. A surface
that plots an absent surprisal as zero is lying about the election, so where
the election did not stand the surface says so rather than drawing a floor.

**A reading is produced only where the record's own bracket permits it.** A
serving record carries no gate. A diagnostic record carries one, and a
reading from an uncertified replay is a picture of an unknown run. This
crate honors that gate rather than re-deciding it.

## 7. The seams

### 7.1 The gate

A turn crosses at the gate as any client's does. The gate does not stream,
so a whole-turn answer is presented whole and an in-flight state is clear
rather than simulated. Closes render by kind, and an unnamed close is this
crate's own defect and surfaces as an application error rather than as an
agent's words.

### 7.2 The admin verbs

`validate`, `load`, `unload`, and as of 2026-09-04 the observation exchange:
`show` answers one agent's load facts and `list` answers one summary per
admitted agent in a single ask. Load state is therefore **the harness's own
word rather than an inference from a socket's existence**, and no surface
labels it as inferred.

No verb chains another. This crate offers each as a separate act and nothing
composite.

**`validate` is also the composition oracle.** It transitions nothing,
refuses an incoherent declaration naming the field, and carries the box
facts a load would meet, so the Compose surface writes its draft and asks
rather than judging. **This crate therefore carries no second copy of the
rules**, only a copy of the declaration's field shape written against
`weaver-types-Spec` section 2 **at a named corpus commit**, which is that
copy's staleness rule: when the floor moves, the pin says so and `validate`
refuses in a way the surface can name.

### 7.3 The analysis stream

The emission this crate ingests. Its shape is a contract between the two
crates and is owed as its own act, per issue #418. This document names the
seam and does not restate the contract.

## 8. Placement and the link

Two processes joined by one dialed link: a connector holding the box-bound
reaches, a server holding the presentation stack. Colocated by default and
separated by changing one address.

**Nothing in the read or write path is box-bound to the agents.** The reader
is a store client, the runner is a queue consumer, and the front end with
its store runs on one machine while the agents run on another. That crossing
is a declared boundary under the charter's section 5 rule and appears in the
cell record like any other.

## 9. What is enforced, and by which instrument

| claim | instrument |
|---|---|
| a position is addressed by run and position alone | compile-pin on the key type |
| ingest is idempotent on that key | perturbation: replay one window twice |
| nothing is computed at read time | review, over the three reads |
| a registered experiment is immutable | compile-pin: no mutating path off the frozen type |
| a forced run is marked in the record | perturbation: strip the mark, the read refuses |
| an absent surprisal renders as absent | perturbation: zero-fill, the view is wrong |
| an undeclared boundary refuses the load | perturbation, at the admit path |

**A watch that cannot fail is not a test.** For each perturbation above, the
act that lands it states what removal makes it fail and confirms it does.

## 10. Open elections

- **The store engine and its migrations.** Postgres is the charter's
  election; the schema's expression is this document's and lands with the
  first code act.
- **The field election's depth**, which is the charter's open cell and not
  this crate's to set.
- **Whether surprisal and entropy draw as one timeline or two.**
- **Whether the preset ladder is a picker or a wizard.**
- **Whether this crate scores**, which the charter's section 9 holds open
  and which decides whether a score column exists in section 2.2 at all.
- **The word "cell"**, which carries a second sense elsewhere in the corpus
  and must be settled once rather than twice.
