# weaver-analysis / weaver-web - contract

**Status:** MERGED. In `main` and the source of truth. **Ratification is not
claimed by the act that lands this text.** Whether it has cleared its gates
is the operator's to say.

**Date filed:** 2026-09-05
**Document ID:** `weaver-analysis-web-contract`
**Editorial:** Per the Working Rules. ASCII, absolute dates.

## Parties

- **`weaver-analysis`, the emitter and the only sender.** Its signals reader
  rides the shared drain of issue #408, needs no tap, no lens, and no
  weights, and pairs what every record already carries: the tokens drawn at
  each generated position, the entropy the generation measured, and the
  surprisal where that election stands. It decides what to send from the
  record's content and the elections the record declares, and asks nothing
  of the reader.
- **`weaver-web`, the reader and the only receiver.** The connector on the
  agents' own box, per `weaver-web-PRD` section 3, which lands what crosses
  in its own store and draws from the store rather than from the wire. The
  browser is a display engine and holds no end of this seam.

No third party reaches this seam. **The agent holds no end of it**: the
emitter parses a finished record outside the agent as an operator principal,
and nothing on this seam reaches the harness, the SPU, or the model.

**This seam is a stream.** The emitter drains a file or a socket and the
reader consumes what the drain emits, per `weaver-web-Spec` section 7.3. It
is the third of that section's three seams and the only one that carries
measurement rather than lifecycle.

```graph
node: weaver-analysis-web-contract
kind: document

edge: party
from: weaver-analysis-web-contract
to: weaver-analysis

edge: party
from: weaver-analysis-web-contract
to: weaver-web
```

## Vocabulary

Every contract names the vocabulary it depends on, grouped by where it is
defined, and a group is stated even when empty.

**From `weaver-spu-Spec` section 6.** The **entropy** at a decode position
and the **surprisal** of the token drawn there, both in bits, defined there
and drawn here rather than restated. **The election** that decides whether a
surprisal is measured is the declaration's, per the same authority.

**From `weaver-trace-PRD` section 3.1.** The **`model.measurement`** event
and its members, the input and output token identifiers, the entropies, the
generation's perplexity, and where their elections stand the surprisals.
**`model.field`** and its ranked candidates, which this seam does not carry
and which section 6 names as the other reader's.

**From `weaver-analysis-Spec`.** The **drain** of section 5, one for the
class with readers above it, and the **signals reader** that rides it.
**This second name has no clause in that Spec as of this date**, an H1 gap
tracked at issue #451, and this contract states nothing the reader does not
already do. Section 8 carries what this contract owes when that clause
lands.

**From `weaver-web-Spec`.** The **position**, the **run**, and the **turn**
of section 2.1, which are the store's address, and the **recorded query** of
section 2.4.

**From `weaver-diagnostic-PRD` section 4.** The **certified** close of a
replay bracket, which is the licence a diagnostic reading needs and a
serving reading does not.

**Defined here.** Nothing. This contract states what crosses and defines no
noun of its own, every term above having one authority already.

## 1. What this contract governs

**The shape a front end graphs from.** The emitter's output today has a
command line's shape, points on one stream and summary on another, and a
shape a view may rely on is a fact between two crates rather than inside
one. This contract names it.

It governs one direction. The emitter sends and the reader receives, and
the reader never asks the emitter for anything: what it wants more of, it
gets by reading a different record or by an act on the emitter's own
charter.

**It asks the emitter for one thing it does not do today**, named in section
3 and nowhere else in this document: the position beside the ordinal. Every
other clause states what `signals.rs` already emits as of `main` at
`20b9cdf`.

## 2. The traffic

**Two streams, and they stay two.** The per-position series and the
per-generation summary are separate, because they are indexed differently
and because a reader that wants one rarely wants the other at the same
grain.

### 2.1 The series

One entry per generated position, carrying:

```text
turn        the turn key, where the record carries one
ordinal     the position's index within its generation, zero-based
position    the resident length at the draw, per section 3
token       the drawn token's identifier
entropy     the distribution's entropy in bits, or absent
surprisal   the drawn token's surprisal in bits, or absent
```

**The token identifier crosses and its surface text does not.** Detokenizing
is the reader's, because the tokenizer that answers it is the artifact's and
this seam carries no artifact identity.

### 2.2 The summary

One entry per generation, carrying its turn and its perplexity where the
record holds one. **A generation whose record holds no perplexity has no
entry**, and an entry is never synthesized from the series.

### 2.3 The record's own facts ride beside both

**The opening kind and the bracket outcome**, which are how a record says
which record it is and whether its bracket closed certified. Section 5
states what the reader owes on them.

## 3. What the emitter owes

**It emits both coordinates, and the conversion happens once.** The series
is indexed by the ordinal within its generation and the store addresses by
position, the resident length at the draw, and a reader that treats the two
as one word addresses the wrong token. The two facts that resolve them meet
at the drain and nowhere later: `model.measurement` carries the input token
identifiers, whose count is the resident length at the generation's first
draw, and the ordinal is the index within the generation. **The position is
that count plus the ordinal.**

**This is the one ask.** `Point` carries the ordinal today and not the
position. The conversion is derivable from what the measurement already
holds, so it asks for no new capture, and it belongs at the drain rather
than in the reader for the reason `weaver-web-Spec` section 2.5 gives:
nothing is computed at read time, because a value computed in the interface
is a value nobody else can reproduce. Converting in the browser would put
one arithmetic rule in every consumer and no authority anywhere.

The pairing is not new to the corpus. `weaver-analysis-Spec` section 5's
field reader already pairs `model.field`'s position against
`model.measurement`'s `output_tokens` at the field's ordinal within that
generation. This clause asks the signals reader to carry the same pairing
the field reader already makes.

**Absence crosses as absence.** An entropy the generation did not measure
and a surprisal whose election did not stand are absent, never zero. A
vector shorter than the tokens is not stretched and a missing one is not
invented, which is what the emitter does today.

**The elections are the record's and the emitter reports them.** The emitter
does not decide whether a surprisal exists. It reports what the record
carries, and the record carries what the declaration elected.

## 4. What the reader owes

**It draws a spike against an absolute bar in bits.** A bar computed as
`mean + k * deviation` over the series drifts with the body of the
distribution, so a session whose median entropy falls fourfold across its
length manufactures spikes late where none stand. Run 3 of the depth series
is the measurement: the median falls fourfold while tokens above six bits
hold both their rate and their height. **The absolute figure is the one that
held still.**

`Series::spikes(k)` stays what it is, a series-relative rule whose caller
names its `k`, and this clause governs which figure a view may graph from
rather than what the emitter may compute.

**A rarity reference rides beside any spike view.** Surprisal measures
rarity and not wrongness. The OCR ground truth is the measurement: flagged
artifacts clear the page's p95 at 1.2 to 1.7 times the rate of the honest
null, and legitimate words occur once in a whole book. A view that shows a
spike without the means to tell a rare name from damage is inviting the
reader to read damage into a proper noun.

**It plots an absence as an absence.** A view that draws a missing surprisal
as zero is lying about the election, and the ordinary posture of a serving
agent is that the election did not stand. Entropy rides every generation
unconditionally and surprisal does not, so a timeline carrying both will
routinely hold one series and not the other.

**It records what it read.** A reading a second person reruns is a reading
whose query was stored, per `weaver-web-Spec` section 2.4, and the reader
that served it is named there by name and version. This seam is one such
reader.

**It stores what it receives and draws from the store.** Nothing on a screen
is computed from the wire, per `weaver-web-Spec` section 3.

## 5. The licence, and which records answer

**A serving record has no gate and a diagnostic one does.** A series read
from an uncertified replay is a picture of an unknown run, exactly as a
readout is, per `weaver-diagnostic-PRD` section 4.

The emitter reports the record's opening kind and its bracket outcome and
leaves the judgment to its caller, which is what it does today. **The reader
is that caller and owes the judgment**: a diagnostic record whose bracket
did not close certified is not drawn, and the refusal names the outcome
rather than rendering an empty view.

This gate is `weaver-analysis-Spec` section 5's and is not restated here.

## 6. What neither party may do

**Neither derives a value the record does not carry.** Not a stretched
vector, not an interpolated position, not a perplexity computed from the
series when the record holds none.

**Neither reads the field through this seam.** The ranked candidates at a
position are `model.field`'s and are addressed by `<turn>:<position>`
through the other reader, per `weaver-analysis-Spec` section 5. A series
carrying the alternatives would put twenty thousand positions' worth of
ranked lists on a stream whose whole point is that it is cheap.

**Neither party reaches the agent.** The emitter parses a finished record
and the reader draws from its own store.

**The reader does not present a forced trajectory as a sampled one.** Where
the record says a token was forced, per `weaver-web-PRD` section 3.5, the
series through it is not quotable as one the model produced on its own.

## 7. Change protocol

**The wire shape is versioned and grows additively**, per the compatibility
discipline of issue #338. An added member is optional at the read, and **the
act that adds it says what its absence means**, because a reader meeting an
older emitter must be able to tell a member that was never sent from one
sent empty.

A member is never repurposed and never narrowed in place. A member whose
meaning changes is a new member beside the old, and the old one's retirement
is its own act.

**A change to what the emitter emits is a documents act through this
contract**, not a code act that the contract follows. That is the direction
of the ask in section 3, which is why it is named there rather than
performed here.

## 8. Conformance

**This contract is cited by both parties.** `weaver-web-Spec` section 7.3
takes it with the act that lands this text. **`weaver-analysis-Spec` owes
its citation and does not carry it yet**, sequenced behind issue #451's
authorizing clause for the signals reader: a contract tracing to a reader
its own crate's Spec does not describe is tracing to nothing, per G2. Until
both citations stand this contract is half-bound and says so here rather
than reading as complete.

**What this contract asserts is asserted in the parties' Specs and not
here.** A contract carries no assertion records of its own, per the
Document Format, and the clauses above trace to `weaver-web-Spec` section 9
on the reader's side and to `weaver-analysis-Spec` on the emitter's once
issue #451 lands.
