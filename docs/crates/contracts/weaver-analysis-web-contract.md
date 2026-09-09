# weaver-analysis / weaver-web - contract

**Status:** MERGED. In `main` and the source of truth. **Ratification is not
claimed by the act that lands this text.** Whether it has cleared its gates
is the operator's to say.

**Date filed:** 2026-09-05
**Revised:** 2026-09-09, second of this date, the summary carries the seated
prefix's length. Section 2.2 gains, once per run, the resident length before the
run's first turn's input, which the emitter derives from the run's first generation
by subtracting that generation's drawn tokens, the terminator, and its input
identifiers from its closing count, per `weaver-spu-Spec` section 6's definition of
the input as the turn's delta with the prefix outside it. It is present only where
the drain began at or before the run's `load`, since a later start would name a
later turn's floor as the prefix. Section 3 asks it of the emitter and the
vocabulary clause names it. Per issue #527, for `weaver-web-Spec` section 5's
whole-run arm, whose branch position it is.
**Revised:** 2026-09-09, the summary names the record it was read from. Section
2.2 gains the record's session and the record's digest per generation, once per run
in the reader's row on the weights hash's own rule, and section 3 asks both of the
emitter, so the reader's run row can say which record it descends from and be
checked against it byte for byte. The emitter is the party that drains the bytes,
so the digest is the emitter's to compute and the reader never sees the record.
**The digest is the first noun this contract defines**, having no other authority,
and the vocabulary clause says so, the session being drawn from the trace charter.
`weaver-analysis-Spec` section 5 moves in the same act, as the parties' rule
requires. Per issue #521, on the operator's ruling of 2026-09-09.
**Revised:** 2026-09-06, the summary carries the record's identity, and the owed
sentences are current. Section 2.2 gains the weights hash per generation as the
record spelled it, section 3 asks it of the emitter beside the two counts, and the
vocabulary draws it from `weaver-spu-Spec` section 3, so the run row's artifact
identity has a source on this seam, per issue #465. Section 3's and section 8's
sentences reading the two counts as owed follow their landing at PR #462 and #463,
the token members' definitions landed there too, and section 8 records that
`weaver-analysis-Spec` now cites this document.
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
  agents' own box, per `weaver-web-PRD` section 5, which lands what crosses
  in its own store and draws from the store rather than from the wire. The
  browser is a display engine and holds no end of this seam.

No third party reaches this seam. **The agent holds no end of it**: the
emitter parses a finished record outside the agent as an operator principal,
and nothing on this seam reaches the harness, the SPU, or the model.

**This seam is a socket.** The emitter drains a file or a stream and its
emission leaves over a socket the reader consumes, per `weaver-web-Spec`
section 3. It is the third of section 7.3's three seams and the only one
carrying measurement rather than lifecycle.

**The emitter initiates and the reader never asks**, so the charter's seam
record runs `from: weaver-analysis`, which the act that lands this text
corrects: it had run from the reader, disagreeing with this contract's own
name under the Document Format's rule that the initiator is named first. Its
tag was `stream`, which is outside that document's seam vocabulary of
`socket`, `link` and `verb`.

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
surprisal is measured is the declaration's, per the same authority. The
**input** and **output token identifiers**, the turn's delta and the drawn
tokens, defined there since 2026-09-05.

**From `weaver-spu-Spec` section 3.** The **weights hash**, the record's
identity of the artifact, BLAKE3 over the canonical manifest that section
states, and its **sentinel**, the empty string a hash that could not be
computed reports.

**From `weaver-trace-PRD` section 3.1.** The **`model.measurement`** event
and its members, the output token identifiers, the entropies, the
generation's perplexity, and where their elections stand the surprisals.
The **`model.output`** event and its resident count and capacity as the
generation closed. **`model.field`** and its ranked candidates, which this
seam does not carry and which section 6 names as the other reader's.
**From `weaver-trace-PRD` section 2.1 and `weaver-trace-Spec` section 1**,
the **session**, the identity the runs share, carried on every line's
envelope, and the **run** and its ending at `unload` or at process death per
that charter's section 4.3.

**Defined since 2026-09-05.** That section tables the measurement's members,
and the two section 3 reads were defined at issue #461: the token
identifiers by `weaver-spu-Spec` section 6 and the resident count by
`weaver-types-Spec` section 4.4 with that Spec's section 4 for why it
includes the terminator. This contract's first draft rested on the members'
names and asserted an arithmetic the records refuted, which is why the
definitions were asked before the ask was restated.

**From `weaver-analysis-Spec`.** The **drain** of section 5, one for the
class with readers above it, and the **signals reader** that rides it.
**That reader's clause landed 2026-09-05 at issue #451**, and this contract
draws its behaviour from there rather than restating it. Section 8 carries
what this contract owes when that clause lands.

**From `weaver-web-Spec`.** The **position**, the **run**, and the **turn**
of section 2.1, which are the store's address, and the **recorded query** of
section 2.6.

**From `weaver-diagnostic-PRD` section 4.** The **certified** close of a
replay bracket, which is the licence a diagnostic reading needs and a
serving reading does not.

**Defined here.** Two nouns, since 2026-09-09. **The record's digest** of
section 2.2, sha256 over a run's own lines as the emitter drained them, and
what makes a run whole for it: no other document holds it, the trace
charter's section 4.3 saying in as many words that the record carries no
checksum. **The seated prefix's length** of section 2.2, the resident
length before the run's first turn's input as the emitter derives it from
the first generation: the SPU records the figure at open per its section
4.2 and no event carries it, per the trace charter's section 3.1, so the
derivation that recovers it is this contract's. This contract is the
authority for both, and `weaver-analysis-Spec` section 5 and
`weaver-web-Spec` section 2.2 cite it here. Every other term above has one
authority already, and this contract states what crosses of them and
defines none.

## 1. What this contract governs

**The shape a front end graphs from.** The emitter's output today has a
command line's shape, points on one stream and summary on another, and a
shape a view may rely on is a fact between two crates rather than inside
one. This contract names it.

It governs one direction. The emitter sends and the reader receives, and
the reader never asks the emitter for anything: what it wants more of, it
gets by reading a different record or by an act on the emitter's own
charter.

**It asks the emitter for what it does not do today**, named in section 3 and nowhere
else in this document, each on the summary stream and each added on the date its header
entry names: the two counts per generation, the weights hash, the record's session and
digest, and the seated prefix's length. Every other clause states what the signals
reader already emits as of `main` at `20b9cdf`, per `weaver-analysis-Spec` section 5.

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
token       the drawn token's identifier
entropy     the distribution's entropy in bits, or absent
surprisal   the drawn token's surprisal in bits, or absent
```

**The position is not a member of this entry.** It is derived by the reader
from the summary's two counts, per section 3, and stored beside the entry
rather than carried on the wire.

**The token identifier crosses and its surface text does not.** Detokenizing
is the reader's, because the tokenizer that answers it is the artifact's and
this seam carries the artifact's identity, on the summary, and never the
artifact.

### 2.2 The summary

**One entry per measured generation, in landing order**, carrying its turn,
its perplexity where the record holds one, **the resident count at the
generation's close beside the count of output tokens**, which are what
section 3's conversion reads, and **the weights hash the generation's
measurement carries**, the record's identity of the artifact per
`weaver-spu-Spec` section 3, spelled as the record spelled it.

**The weights hash is per generation on the wire and once per run in the
reader's row.** A run admits one artifact, so its generations agree, and a
run whose generations disagree is a defect the reader names rather than a
run with two artifacts. **The sentinel crosses as the empty string it is**:
a hash the SPU could not compute is a fact of the record, distinct from a
measurement carrying no member, which crosses absent, and a reader that
folded the two together could not tell a failed identity from an older
record.

**The record's session and the record's digest cross the same way: per
generation on the wire and once per run in the reader's row.** The session is
the envelope's `session` every line of the record carries, per
`weaver-trace-Spec` section 1, spelled as the record spelled it, and it is
the identity the run's session shares with the runs before and after it,
which `weaver-web-Spec` section 2.2 stores so a session's runs are readable
as a family. The digest is sha256 over the bytes of the run's own lines as
the emitter drained them, each line with its terminating newline, in the
order they were read, spelled as lowercase hex, so a reader holding the
record can recompute it and a row can be checked against the record it
descends from. **Every line names a run, and a line belongs to the digest
of the run it names**, the envelope's `run` being a required member per
`weaver-trace-Spec` section 1, so two emitters over one file agree. **A
run's generations
agree on both, and a run whose generations disagree is a defect the reader
names.** **The digest is present only for a whole run, and a run is whole
when its `unload` landed and the drain began at or before its `load`.** A
run that ended at process death has no `unload`, per `weaver-trace-PRD`
section 4.3, its queue's tail forfeited, and an emitter that drained every
line that exists has still not drained the run, and a drain that opened
after the run's first line saw a suffix. In both the digest is absent, so an
absent digest says the emitter could not vouch for the bytes rather than
vouching for the bytes it happened to see, and the reader's refusal on a
mismatch never reads a digest of a truncated record as a digest of the
whole. The session is absent where the record predates the envelope's
session member and never otherwise.

**The seated prefix's length crosses the same way: on every generation's entry, and once
per run in the reader's row.** It is the resident length before the run's first turn's
input, the position that turn's input begins at, which `weaver-spu-Spec` section 4.2 has
the open record and no event carries as a member, per `weaver-trace-PRD` section 3.1.
**The emitter derives it from the run's first generation and from nothing later**: the
closing count less the drawn tokens, less the terminator, less the count of that
generation's input identifiers, the input being the turn's delta with the prefix outside
it per `weaver-spu-Spec` section 6, so what remains is the prefix and only the prefix.
**It is present only where the drain began at or before the run's `load`**, because a
drain that opened later would take some later turn's floor for the prefix and name a
wrong position with a right-looking number, and it is absent where the first generation
carries no closing count or no measurement, each member absent on its own terms. A
restoring run's prefix includes what the load restored, per `weaver-harness-Spec`
section 8, and the figure says so by being larger, which is what a whole-run arm's
branch position needs it to say. Every entry repeats it as every entry repeats the
weights hash, so a run's generations agree on it as they do on the session, and a run
whose entries disagree is the defect the reader names.

**The entry does not depend on the perplexity.** A generation whose record
carries none still carries its counts, because the residency is what a store
keyed by position converts from and it is owed whether or not a perplexity
was taken. **Each member is absent on its own terms**: a perplexity the
record does not hold, a resident count no `model.output` reported. An entry
keyed to any one member's presence would drop the others where they are
still wanted, which is the same absent-not-empty rule section 3 states for
the series applied one grain up.

**An absent resident count costs the reader its positions and not its
entry.** The entry still lands, carrying the turn, the output count, and
the perplexity where one was taken, and what the reader cannot do is form
the address section 4 requires for that generation's points. The summary
says what the record held either way, and a consumer reading it can tell a
generation it could not address from one that was never measured.

**No entry is ever synthesized from the series.** The output count is the
generation's drawn tokens as the record spells them and not the length of
what this seam happened to carry, so a truncated or partially ingested
series never changes what a summary entry says.

### 2.3 The record's own facts ride beside both

**The opening kind and the bracket outcome**, which are how a record says
which record it is and whether its bracket closed certified. Section 5
states what the reader owes on them.

## 3. What the emitter owes

**The series is addressed by the ordinal and the store by the position, and
the summary carries what converts between them.** The two are different
coordinates: the ordinal is the index within a generation, which is what a
series is drawn against, and the position is the resident length at the
draw, which is what `weaver-web-Spec` section 2.1 keys on and what the field
read addresses. **A consumer that treats them as one word addresses the
wrong token.**

**The emitter therefore carries, per generation, the resident count as the generation
closed and the count of output tokens**, beside the perplexity on the summary stream,
**and the record's session and digest per section 2.2**, the emitter being the party
that drained the bytes and the only one that can name them, **and the seated prefix's
length per the same section**, derived from the first generation once per run. **The
output count is the length of that generation's `model.measurement` `output_tokens`
sequence**, the drawn tokens with the terminator outside them per `weaver-spu-Spec`
section 6, and not a separate scalar the record carries. **A generation whose
measurement holds no readable `output_tokens` produces no summary entry and no points at
all**, so `O` is never absent from an entry that exists and a consumer never meets a
half-formed one. Both are facts the record already holds, on `model.output` and
`model.measurement`, so the emitter reports them and derives nothing, which is the
property `weaver-analysis-Spec` section 5 argues for.

**The two counts landed at PR #462 and #463 on 2026-09-05**, with the
definitions the members wanted, and the emitter carries them today.

**The emitter also carries, per generation, the weights hash the
measurement holds**, as of 2026-09-06 per issue #465, so that the run row
of `weaver-web-Spec` section 2.2 has a source on the wire: a reading without
its artifact is a reading of an unnamed compound, and the identity is the
recorded half's fact rather than the declaration's, a declaration naming a
path and being editable after the run. The emitter reports the member as
the record spelled it and derives nothing, per section 2.2.

**The conversion is exact and was measured rather than reasoned.** Where `R`
is the resident count at the generation's close, `O` the count of output
tokens, and `j` the ordinal:

```text
position = (R - O - 1) + j
```

The `- 1` is the turn terminator, which the SPU makes resident before the
answer returns. **It is one token, measured** across three records on the
2026-09-05 hub, nine generations, two precisions, and two finish kinds,
checked against the position `model.field` reports directly. The obvious
alternative, the previous generation's resident count plus the turn's input
delta, is exact from the second generation and wrong on the first by the
session prefix, which the first turn's delta does not carry.

**The arithmetic belongs to the reader and the facts to the emitter.** The
reader derives once at ingest, per `weaver-web-Spec` section 2.7, a value
derived at ingest and stored being one a second reader can reproduce and a
value computed in the interface being one nobody can.

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

The series-relative spike rule stays what it is, a rule whose caller
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
whose query was stored, per `weaver-web-Spec` section 2.6, and the reader
that served it is named there by name and version. This seam is one such
reader.

**It stores what it receives and draws from the store.** A value that must
be derived is derived once at ingest and stored, per `weaver-web-Spec`
section 2.7, so nothing on a screen is computed from the wire.

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
took it with the act that landed this text, and `weaver-analysis-Spec`
section 0 names it among what that document is written against as of
2026-09-06, the citation having been owed since the signals reader's clause
landed at issue #451.

**Section 3's asks have landed and the wire tells the states apart.** The
two counts landed at PR #462 and #463 and the weights hash with issue #465,
each an additive member per section 7. A reader meeting an older emitter
finds the counts absent and cannot key its store, whose primary key is the
run, the turn and the position, so it stores what crosses and addresses by
ordinal within a run and turn, converting nothing. A reader finding the
counts and no weights hash can key its store and cannot join the run to
the catalog. Each state is a fact on the wire rather than a version to
negotiate.

**What this contract asserts is asserted in the parties' Specs and not
here.** A contract carries no assertion records of its own, per the
Document Format, and the clauses above trace to `weaver-web-Spec` section 9
on the reader's side, which gains the rows for the derivation at ingest and
the uncertified refusal with the act that lands this text, and to
`weaver-analysis-Spec` section 5 on the emitter's, whose clause for the
signals reader landed 2026-09-05 at issue #451.
