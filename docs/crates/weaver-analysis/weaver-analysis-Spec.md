# weaver-analysis - Spec

**Status:** MERGED. Cut 2026-08-27, the second Spec of the diagnostic leg. Code is
written against it under the gates of Working Process section 6.

**Date filed:** 2026-08-27
**Revised:** 2026-08-31, the reading learns to drain. Section 5 gains the
pipe-shaped sink's discipline per the charter's section 3 as amended: read
once, keep the report and its evidence, retain nothing drained, name the card
model or state that it cannot be established. The kind-set count follows
`weaver-diagnostic-Spec` to seventeen, and section 7 adds the lens artifacts
as an open election beside the capture artifact.
**Document ID:** `weaver-analysis-Spec`
**Parent:** `weaver-analysis-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The representation of the driver `weaver-analysis-PRD` charters: how it parses a
finished record, what it projects across the preload seam, and how it reads the
record a replay makes. It is written against that charter, against
`weaver-analysis-state-contract`, and against `weaver-diagnostic-Spec` for the
record it reads, and it develops no rationale of its own.

**What the charter left here is named there.** Section 4's closing cell owes this
document the driver's shape and the parser's, and says the certification's
mechanics are not among them: that comparison belongs to the loop inside the run,
and what this document settles about it is only how this crate elects a null
replay and reads the outcome.

**One cell it owed closed before this document opened.** How a diagnostic-trace
says it ended settled in `weaver-diagnostic-Spec` section 3.3 on 2026-08-27, which
is what makes section 5's gate writable at all: a driver that could not tell a
finished record from a truncated one could hold this shape and not use it.

## 1. The crate

**Layout.** One module per obligation, re-exported at the root.

    src/lib.rs        re-exports, and nothing else
    src/record.rs     the parse of a serving record, section 2
    src/project.rs    the election and the projection, section 3
    src/preload.rs    the seam's sender, section 4
    src/reading.rs    the diagnostic-trace's parse and the gate, section 5

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
used.

**The dependency set is two crates and no internal one.** `serde` with `derive`,
and `serde_json` with `raw_value`, which sections 2 and 3 elect for carrying a
payload's elected values as the record spelled them rather than re-encoding them.

**No `weaver-*` dependency at all, and the negative is the boundary in the
manifest.** This crate stands outside the agent, per the charter's section 1, and
its one seam draws its whole vocabulary from documents rather than from types:
`weaver-analysis-state-contract` draws the election and the distillate from
`weaver-harness-state-contract` and the event names from `weaver-trace`, and every
one of those crosses this crate's wire as JSON the record already spells. Linking
any of them would make an outside consumer a compile-time dependent of the agent's
interior, which is the coupling the boundary exists to prevent, and would buy
nothing: the parse shares no code with the writer by the charter's own election.

```graph
node: analysis-no-internal-dependency
kind: assertion
tag: manifest

edge: asserts
from: weaver-analysis
to: analysis-no-internal-dependency

edge: grounds
from: analysis-no-internal-dependency
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**No async runtime and no socket crate in the resolved tree**, the floor Specs'
build-time `cargo tree` assertion read here for a different reason: this crate
dials one Unix socket and binds none, so the standard library's own client is the
whole of what it needs.

```graph
node: analysis-no-runtime-no-socket-crate
kind: assertion
tag: manifest

edge: asserts
from: weaver-analysis
to: analysis-no-runtime-no-socket-crate
```

**It binds no listening port and holds no server.** The charter's section 2 has
this crate governing nothing inside the agent, with the harness holding no channel
to it and no behavior conditioned on its presence, and the absence of a listener is
what makes that true rather than merely asserted: a driver that listened would be
reachable, and a consumer the agent cannot reach is the structural claim this
crate's position rests on.

**The instrument is review, and what the manifest reaches is named beside it so
the two are not confused**, on the reason its sibling's compile-fail set gives: a
compile-fail doctest buys an absence from a crate's own surface,
naming a call that crate does not offer, and binding a listener is `std`'s call
rather than this crate's, so no doctest can refuse it. What the manifest reads is
that no socket crate stands in the resolved tree, which is the same assertion above
read for this claim's sake, and the residue - that this crate's own code calls
`bind` nowhere - is review's, named here rather than claimed as bought.

```graph
node: analysis-binds-no-port
kind: assertion
tag: review

edge: asserts
from: weaver-analysis
to: analysis-binds-no-port
```

## 2. The parse

**The read types are this crate's own and share no code with the writer.** The
charter's section 3 elects that separation and calls it the boundary working as
intended, so this crate spells the envelope and the payload members it reads rather
than importing them. **`weaver-trace-Spec` section 3 is authoritative for every
shape spelled here**, and a divergence is a defect against it per G5, which is the
same arrangement the charter states from its side.

**The parse is envelope-first and payload-lazy.** Every line yields its envelope
eagerly, that being what the projection groups and orders by, and a payload is held
as raw text until a key path is read out of it. That is what lets an elected value
cross this crate byte-identical to what the record spelled, per section 3, and it
is the same reason `weaver-trace`'s tee holds a payload as raw text on the other
side of the same wire.

**A kind this crate does not know is skipped, and a payload member it does not know
is ignored.** The record carries no version marker and needs none, per
`weaver-trace-PRD` section 6, so a reader keys on nothing and every vintage is the
one schema. **Neither may decide a grouping**: the walk runs on run and turn from
the envelope and on request-to-measurement pairing in landing order, none of which
an unrecognised record can move, which is what makes the rule satisfiable here
rather than merely stated. The charter's section 4 argues this at length and this
clause represents it.

```graph
node: analysis-parse-skips-the-unknown
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-parse-skips-the-unknown
```

**A member a record does not carry is absent and is never derived from the members
beside it.** That is the harder direction of the same rule, per the charter's
section 4 and `weaver-trace-PRD` section 6, and this crate is where it costs
something: a record written before the layer and forward counts existed omits them,
and deriving a layer count from the length of a norm array is exactly the
arithmetic those counts were added to retire. **A replay over such a record is a
replay whose layer count is unknown rather than one whose layer count is guessed**,
and this crate carries the unknown forward rather than closing it.

```graph
node: analysis-derives-no-absent-member
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-derives-no-absent-member
```

## 3. The election and the projection

**The election this crate declares is composed from what the replay reads, not
declared by an operator.** A serving load's election is the operator's, per
`weaver-harness-state-contract`, that contract's `election` term making it a
load-declared fact the operator states. A preload's is this crate's, because this
crate knows what the loop it is feeding will ask for: `diagnostic-replay-loop`
walks by run and turn from the envelope, pairs request to measurement in landing
order, and establishes input identity before any forward pass. **So the election
names the kinds those steps read and the payload key paths they read out of them,
and nothing further.**

**The ceiling is the loop's reading and not this crate's judgment of size.** What
bounds the election from above is that a kind `diagnostic-replay-loop` never reads,
in its walk of section 2 or its certification of section 3, is a kind whose
holdings nothing would ask for. So the rule is checkable against that document
rather than against a preference of this one, and a step added there widens this
election in the act that adds it.

**`load` is elected, and naming it matters because the identity does not come from
the holdings alone.** Step one establishes the five re-feed items and the template
from the events it walks, and takes **the tee's election from the record's `load`
event**, which that step says outright it reads there and never from the holdings:
the holdings are what that rule produced, so recovering it from them would be
reading a projection to learn what did the projecting. Since this crate's election
decides what reaches the holdings at all, an election omitting `load` would land a
session whose certification cannot check the rule that built it, which is the
failure `weaver-agents-PRD` section 8 added the criterion to prevent.

**This document is authoritative for the election's content**, per G5, and
`diagnostic-replay-loop` section 2's step 2 sketches it for a reader walking the
three acts rather than fixing it. A divergence there is a defect against this
section.

**The declared session is the replayed session's own name**, per the contract's
section 2, because the loop's asks on the other door bind to the opener's session
and a preload declaring anything else would land holdings no ask can reach.

**What the opener declares is what the stream delivers.** Kinds outside the
election do not cross, which is the contract's owing in section 3 and is checkable
against the opener the same channel carried.

```graph
node: analysis-election-declares-what-follows
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-election-declares-what-follows
```

**A distillate is the envelope whole and the elected pairs beside it, each value as
the record spelled it.** The shape is `weaver-harness-state-contract`'s, drawn by
this crate's own contract rather than restated, and the projection reaches it by
splicing raw payload text rather than re-encoding a parsed value. **That is what
makes the preload's indistinguishability claim true rather than approximate**: a
number the record spelled one way cannot reach the holdings spelled another, so a
holding cannot say by its own bytes whether a tee or this crate landed it.

```graph
node: analysis-projection-splices-verbatim
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-projection-splices-verbatim
```

**Every distillate carries all five envelope fields, in the record's sequence
order**, per the contract's section 3. An unattributable distillate is a defect in
this sender, and an out-of-order one is worse: the loop's pairing runs in landing
order, so a stream that reordered would pair a request with a measurement from
another generation and the replay would read a turn that never happened.

```graph
node: analysis-sequence-order-preserved
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-sequence-order-preserved
```

## 4. The preload

**Three things in one order, and the seam is owed nothing back.** The election
opens the channel, a distillate per elected event follows in sequence order, and
the seal ends it, per the contract's section 2. This crate asks nothing on this
seam and reads nothing from it.

**The seal is an empty JSON object on its own line**, `{}` canonically, and this
crate writes that spelling. A blank line is framing residue and not a seal, per the
contract, so a driver that emitted one would have closed without sealing and the
parked replay ask on the other door would never answer.

```graph
node: analysis-seal-ends-the-preload
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-seal-ends-the-preload
```

**It dials as an operator principal and never as the agent.** The door refuses the
agent's credential before any byte is read, per the contract's section 4, so a
driver running as the agent's uid has no business this contract recognises. The
instrument is review: what this crate can assert of itself is that it opens the
socket under whatever identity it was invoked with and mints none, and whether that
identity is the operator's is the operator's arrangement rather than a property a
test of this crate reaches.

```graph
node: analysis-dials-as-invoked
kind: assertion
tag: review

edge: asserts
from: weaver-analysis
to: analysis-dials-as-invoked
```

**One preload per standing of this driver.** The owing is the contract's and this
crate meets it by structure: a run of this crate opens one channel, sends one
preload, seals, and closes, so a second preload is a second run. **The member's
door outliving this driver is not this crate's concern**, per the contract's same
clause, and a retry is a new run of this crate rather than a second preload inside
one. The instrument is a compile-fail pin on the shape that would break it: no call
sends a second opener on a channel that carried one, the sender being consumed by
the seal.

```graph
node: analysis-one-preload-per-run
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-analysis
to: analysis-one-preload-per-run
```

## 5. The reading, and the gate

**The diagnostic-trace's parse is the serving parse's sibling and answers to a
different authority.** The line is the same line and the envelope the same
envelope, per `weaver-diagnostic-Spec` sections 2 and 3.1, so section 2's rules
above bind here unchanged: skip the unknown, derive nothing absent, hold payloads
raw. **What differs is the kind set**, seventeen rather than twenty-one, and
`weaver-diagnostic-Spec` section 3.2 is authoritative for it, a divergence being a
defect against that document rather than this one.

**Which record this crate holds is answered by the record.** A bracket opening with
`replay.opened` is a diagnostic-trace and one opening with `load` is a serving
record, per that Spec's section 4, so this crate reads the first event rather than
its invocation to know what it was pointed at, and a record that answers neither is
refused as neither.

**The gate is the outcome the record states.** Per the charter's section 3 as
settled: this crate produces its reading where a bracket closed certified, produces
the divergence where one closed diverged, produces neither where one closed
abandoned, and **produces nothing for any unclosed bracket, on the same terms
whichever way it came to be unclosed**. A pass that died and a pass still running
leave one absence between them, and this crate does not try to tell them apart,
because treating the end of available bytes as the end of a run is exactly what the
marker exists to stop.

**Where the sink is a pipe the reading drains it and keeps nothing but the
report**, per the charter's section 3 as amended on the operator's ruling of
2026-08-30. The stream is read once in landing order, the report carries the
evidence it rests on, and no member of this crate retains the drained bytes:
what would be kept is regenerable by replay, and the charter carries why that
licence holds and where it is bounded. **The report names the card model the
evidence came from**, read from the deposit the operator holds until the
record event owed by that same section lands, and a report that cannot
establish the architecture states that it cannot rather than omitting the
member, the absence otherwise reading as a reproducibility the report does
not have.

```graph
node: analysis-gates-on-the-stated-outcome
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-gates-on-the-stated-outcome
```

**The null replay is elected by this crate's own procedure and not by its control
over the load.** The reader's election rides the declaration and is the operator's,
per apex section 8, so what this crate elects is the order it consumes outcomes in:
it reads a null pass's outcome first and gates every reading downstream on it, and
where no certified null pass stands in the record it produces nothing regardless of
what a reader pass beside it reported. **A readout from an uncertified replay is a
picture of an unknown run**, per `weaver-diagnostic-PRD` section 4, and the gate is
this crate's half of that rule.

```graph
node: analysis-null-replay-gates-the-rest
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-null-replay-gates-the-rest
```

**It writes no record and nothing it produces reaches a decoder.** What this crate
makes is a reading, held or written wherever the operator directs it, and its one
seam sends material rather than instruction, per the charter's section 2. The
instrument is the compile-fail absence of any write surface toward either record:
no call constructs a trace writer, and the preload's sender takes distillates and
never events.

```graph
node: analysis-writes-no-record
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-analysis
to: analysis-writes-no-record
```

**How this crate reaches the sink follows the operator's declaration and not this
document.** The charter's section 3 assumes no discriminant, so this crate takes a
byte stream from its invocation and reads records off it, and whether that stream
is a finished file, a drained pipe, or a held connection is the operator's
arrangement. **What this document refuses to do is elect per shape**, which would
put back the assumption the charter withdrew and make a consumer's reading depend
on how an operator declared a sink.

## 6. What is enforced, and by which instrument

**Enforced by compile-fail tests, because the property is an absence.** No write
surface toward either record: a doctest constructing a trace writer fails to
compile, and **the buyable half is the sender's shape**, which takes distillates
and never events, the no-writer half being bought already by the absent dependency
below. No second opener on one channel, the sender consumed by the seal.

**Enforced by the manifest.** No `weaver-*` dependency at all, read against the
graph under gate H2, this crate declaring one `seam` tagged `socket` and no
`floor-link`. No async runtime and no socket crate in the resolved tree.

**Requiring a perturbation-verified test.**

- The parse skips a kind and a payload member it does not know, confirmed by
  feeding a record carrying an invented kind and an invented member and watching
  the grouping stay identical, and by watching a rejection appear when the skip is
  removed.
- No absent member is derived: a record without the layer and forward counts yields
  an unknown layer count, watched to fail when a derivation from the norm array's
  length is put back.
- The election declares what follows: the stream carries no kind the opener did not
  name, watched to fail when a kind is projected past the election.
- The projection splices verbatim: a value the record spelled in a way a
  re-encoding would change crosses byte-identical, watched to fail when the
  projection re-encodes a parsed value.
- Sequence order is preserved: distillates leave in the record's order, watched to
  fail when the projection sorts or groups before sending.
- The seal ends the preload: an empty JSON object follows the last distillate,
  watched to fail when a blank line or a close alone is substituted.
- The gate reads the stated outcome: a record whose bracket never closed produces
  nothing, watched to fail when the end of available bytes is read as an ending.
- The null replay gates the rest: a reader pass's outcome produces nothing where no
  certified null pass stands in the record, watched to fail when the ordering is
  removed.

**Enforced by review, two claims.** That this crate dials as an operator principal
is the operator's arrangement rather than a property a test of this crate reaches,
per section 4: what a suite can confirm is that this crate mints no identity and
opens the socket under the one it was invoked with, and the credential's rightness
is judged at the far end by the door. That this crate's own code binds no listener
is the residue section 1 names: the manifest reaches the dependency and no
instrument here reaches the absence of a call `std` offers every crate, so the
claim is review's and says so rather than borrowing the manifest's coverage.

**Where the records sit.** The assertion records are at the clauses that argue the
claims, across sections 1 through 5, rather than gathered here, per Document Format
section 6. Fourteen sit there and none sits here.

**Which invariant each claim serves.** One carries a `grounds` edge.
`axiom-floor-is-vocabulary-behavior-is-socket` is why this crate links no internal
crate: its whole vocabulary crosses a socket as drawn names rather than as shared
types, which is that invariant read from outside the agent, where a linked
dependency would have made a consumer a compile-time dependent of the interior.
The other four axioms reach none of these claims. **Thirteen claims grounding in no
invariant is the expected result and not a gap**, per Document Format section 4:
most of this document is representation.

## 7. Open elections

- **The instrument suite**, per the charter's section 4: what this crate carries
  beyond the certification is named there as a sketch that does not exist in this
  tree, and nothing here is built against it.
- **The capture artifact** - identity, custody, dataset shape, quota - is owed its
  own act and lands here when it lands, per the same section.
- **The lens artifacts.** The charter names the fitting as this crate's, and the
  fitted per-layer matrices are artifacts with identity, versioning against the
  weights hash they were fitted to, and custody, none of which is settled here.
  They land with the suite's act beside the capture artifact above.
- **What the reading is, as an artifact.** This document settles when a reading is
  produced and refuses to settle what it looks like, that being the suite's
  question rather than the gate's.
- **The satellite types.** The parse's newtypes over the record's identity strings,
  the election's spelling here, and the byte-stream reader's shape. Identifier
  choices with no cross-crate consequence, listed so what this Spec leaves to a
  builder is complete rather than implied.
- **The licence boundary** is the operator's, per the charter, and this document
  takes no position beyond noting that nothing here carries cut-and-recompute.
