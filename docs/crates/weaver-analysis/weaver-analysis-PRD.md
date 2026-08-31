# weaver-analysis - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. Ratified on its own
terms under the per-charter rule of 2026-08-23, conforming to the pattern the
2026-08-04 act established.

**Date filed:** 2026-08-24
**Revised:** 2026-08-31, the discard's licence. Section 3 gains the operator's
ruling of 2026-08-30: a pipe-shaped sink retains nothing, the report is the
kept artifact carrying its evidence, the discard is licensed by bit-exact
replay and bounded by the card model, which the report must name, the record
event that would carry it named as owed. Nothing about the sink's governance
moves.

**Revised:** 2026-08-27, third of this date, the null replay's verb is corrected.
Section 3 said this crate elects the null replay and section 4's cell said the same,
which read as a control over the load this crate does not hold: the reader's
election rides the declaration per apex section 8. Both now say what
`weaver-analysis-Spec` section 5 settled, that this crate requires a certified null
pass before anything downstream and elects the order it consumes outcomes in. The
Spec had settled against this charter and the charter was open in the same act, so
landing it here rather than leaving the two documents disagreeing.
**Revised:** 2026-08-27, second of this date, the driver takes its shape.
`weaver-analysis-Spec` landed, closing section 4's last cell: the parser, the
election, the projection, the preload's three frames, and the reading's gate.
Fourteen assertion records, one grounding in the floor invariant read from
outside the agent, where a linked dependency would make a consumer a
compile-time dependent of the interior. Section 4's other cells stand as they
were, the instrument suite and the capture artifact being their own acts. Per
epic 293 row 12.
**Revised:** 2026-08-27, first of this date, the terminal marker this charter
waited on landed.
Section 4's cell on how a diagnostic-trace says it ended settles in
`weaver-diagnostic-Spec`, the document this cell named as owed it, and the cell
records the shape rather than restating its argument. Section 3's gate is
therefore honourable, and what this crate still owes is its own Spec. Per epic 293
row 12.
**Revised:** 2026-08-25, third of this date, the gate names what it rests on.
Section 3 claimed this crate reads the null replay's outcome from the record and
gates downstream work on it, and a reader at the end of the bytes available to
it cannot tell a certified run from a failed one, a partial one, or one still
going. The claim now carries its condition, and the honest behaviour under the
condition is to produce nothing rather than to read the end of available bytes
as the end of a run. Section 4 gains the owed act with the four outcomes a
marker has to separate, so the act has a criterion, and states why the rule is
not written here: a terminal event belongs to the diagnostic-trace's vocabulary,
which `weaver-diagnostic-PRD` section 6 owes to that crate's Spec, and a rule
written per sink shape would put back the discriminant assumption section 3
withdrew.
**Revised:** 2026-08-25, second of this date, section 3 states what it sends and
what governs the sink. The preload paragraph named distillates and a seal and
left out the election, which `weaver-analysis-state-contract` section 2 makes
the first traffic on every standing of the channel, carrying the replayed
session's own name so the holdings answer to the name the loop asks against.
All three are now named in their order. The delivery paragraph said no contract
governs it, which overstates an absence into a licence: what is absent is a
second contract with this crate as a party, and the sink itself is governed as
it always was, `trace-sink` on the declaration under either kind, admin's
custody and discriminant, and the operator contract on what crosses out.
**Revised:** 2026-08-25, first of this date, the name is this crate's alone.
`CLAUDE.md`'s scope
guardrails listed `weaver-analysis` among the crates ruled out of the
extraction, which the chartering act turned into one spelling naming two
things, the defect Document Format section 2 names with the halves swapped.
On the operator's ruling of this date the name is freed rather than
disambiguated: the previous program's crate leaves that list and this charter
holds the name outright. Nothing of the quarry's crosses on that account, the
standing rule of `CLAUDE.md` being that material crosses only where a step in
apex section 3 exercises it, which reaches every crate of the old tree whether
or not a list names it.
**Document ID:** `weaver-analysis-PRD`
**Parent:** the WeaverTools suite, whose governing document is deliberately
not yet written, per `weaver-agents-PRD` section 0. The graph parent edge
names the `WeaverTools` system node, and the header and the edge name the
same thing.
**Editorial:** Per the Working Rules.

---

## 1. What this crate is

`weaver-analysis` is **the diagnostic consumer**, a crate outside the agent boundary
standing in the structural position Weaver Web stands in. It holds both ends of a replay
and the agent holds neither: it reads a finished record the operator holds, preloads it
through the state member's second door, and reads the diagnostic-trace the run produces
off the sink admin opened for the binding. Whether that reading trails the run or runs
beside it follows the sink's declared shape and is not assumed here, per section 3.
Everything downstream of the record is here - the fitting, the projection, the layer
trajectory, the artifact store, and the reading.

It is chartered by the operator's ruling of 2026-08-24, which split the
diagnostic leg in two. `weaver-diagnostic` is the mechanism the harness
authors a diagnostic-trace through, inside the agent as the harness's third
member. This crate is what reads that record. An earlier reading had one crate
holding both roles, which put a rendering mechanism and a consumer under one
name and would have had the harness linking a crate from outside its own
boundary.

**One crate at both ends, and that is deliberate.** The party that decides what
a replay is made of is the party that can say what its output means, because
only it knows what it elected: which message kinds it preloaded, which turns it
projected, and what it left out. A second consumer reading the record cold
would be interpreting a run whose shape it did not choose. The certification is
not the ground for this and cannot be, that comparison belonging to the loop
inside the run per section 3.

**It never sees weights.** The residual readout is SPU-internal and elected at
the load, and what reaches this crate is what the record carried. Nothing here
touches a device, and no election of this crate's changes what a replay
computes.

```graph
node: weaver-analysis
kind: crate

edge: parent
from: weaver-analysis
to: WeaverTools

edge: seam
from: weaver-analysis
to: weaver-state
via: weaver-analysis-state-contract
tag: socket
```

## 2. What it is not

**Not an organ.** An organ governs a domain and holds a duplex channel with
the harness, both properties and neither alone. This crate has neither. It
governs nothing inside the agent, and the harness has no channel to it, no
knowledge of it, and no behavior conditioned on its presence.

**Not a member of the agent domain.** It parents to the suite rather than to
`weaver-harness`, per `weaver-agents-PRD` section 0's rule that crates outside
the agent boundary do not enter that roster. `weaver-diagnostic` went the
other way in the same act and for the opposite reason.

**Not the writer of anything the agent reads.** Its one seam sends a preload
into `weaver-state`, and that is material rather than instruction: state holds
what it is given and the agent's loops decide, per
`weaver-analysis-state-contract`. Nothing this crate produces reaches a
decoder except as tokens a loop chose to feed.

**Not the intervention overlay.** Cut-and-recompute is not carried here, on
the same taxonomy ruling that keeps it out of `weaver-diagnostic`:
intervention changes the input and produces a counterfactual token path, which
is a different product from a measurement of a run that happened.

## 3. What it does, in the order it does it

**It reads the operator's record as an operator principal**, over the
operator's own storage, and parses it. The parse is this crate's, sharing no
code with the writer, which is the boundary working as intended and is also
two statements of one shape: `weaver-trace-Spec` section 3 is the authority
and a divergence is a defect here, per G5. The reader rules of
`weaver-trace-PRD` section 6 bind this parser in both directions, and section
6 of that document is where they are argued rather than restated here.

**It preloads what the parse projects**, across the one seam, per
`weaver-analysis-state-contract`, and it sends three things in one order. **The
election opens the channel**, whole, as the first traffic on every standing,
declaring the replayed session under its own name so the holdings answer to the
name the loop later asks against. **Then a distillate per elected event**, in
the record's sequence order, owed nothing back. **Then the seal**, one empty
frame after the last distillate, which is the fact the harness's replay ask
answers at and the only thing that tells a finished sender from a dying one.
This crate sends all three and asks nothing on this seam.

**It reads the diagnostic-trace off the sink.** The sink is admin's, opened for
the binding under root by whatever discriminant the declaration named, per
`weaver-admin-Spec` section 5, and **this charter assumes no discriminant**:
the operator declares the shape, and whether this crate reads a finished file,
drains a pipe, or holds a connection follows from that declaration rather than
from anything here. Whether the reading trails the run or runs beside it
follows the same way. **No separate diagnostic delivery contract is introduced
and none is owed**, because a sink's reader is downstream of it rather than a
party to it, which is the position every consumer of a serving trace already
occupies, per `weaver-trace-PRD` section 1. **What governs the sink is
unchanged and reaches both kinds**: the declaration carries `trace-sink` under
either binding per `weaver-types-Spec` section 2, admin opens it by its
discriminant and holds its custody per `weaver-admin-Spec` section 5, and
`weaver-admin-operator-contract` section 3 governs what crosses out and whose
durability it is. This crate inherits that arrangement rather than standing
outside it.

**A pipe-shaped sink retains nothing, and the report is the kept artifact.**
Per the operator's ruling of 2026-08-30: a diagnostic run whose sink is a pipe
streams through this crate, the reading is taken as the stream drains, the
report carries its evidence, and the raw capture is kept nowhere. **What
licenses the discard is bit-exact replay and nothing weaker.** A capture a
replay regenerates exactly is derivable rather than data, so keeping it would
store what can be recomputed, and the trace with its declaration is already
the capture's compressed form. **The licence is bounded by the card model and
the report must name it**: reissue is bit-exact within a card model and not
across one, so a report whose evidence was discarded is reproducible on the
architecture that produced it, and a report that does not name that
architecture asserts a reproducibility it cannot carry. The serving device
does not yet ride the run's own record: it reaches the deposit through the
driver that took it, per the run-records-what-served act of 2026-08-28, and a
record event carrying it is owed its own act, named here as owed rather than
assumed present. Until it lands the report reads the architecture from the
deposit the operator holds, and a record alone does not license the discard.

**It reads nothing for meaning from a replay that did not certify**, per
`weaver-diagnostic-PRD` section 4, which carries the criterion because it
belongs with the mechanism being judged. **The comparison is not performed
here.** `diagnostic-replay-loop` section 3 walks it inside the run, which is
where it has to happen: the loop holds the recorded path in its holdings and
the recomputed identifiers as they arrive, so it alone can refuse before the
first forward pass and name the first divergent position rather than reporting
after a whole replay has run. What this crate does is **require a null replay
before anything downstream**, read its outcome from the record, and gate every
later reading on it. **The reader's election is not this crate's to make**, riding
the declaration at the load per apex section 8, so what this crate elects is the
order it consumes outcomes in and never which pass the agent runs. An earlier form
of this sentence said this crate elects the null replay, which read as a control
over the load it does not hold, and `weaver-analysis-Spec` section 5 settled the
distinction this clause now carries.

**That gate rests on telling a finished record from a truncated one, and the
record carries the fact as of 2026-08-27.** A reader that has consumed every byte
available to it once could not say whether the replay certified and ended, failed
its comparison and ended, ended without finishing, died mid-replay leaving a
partial record, or was still running, and all of them looked alike at the end of
what it had. **`replay.closed` separates the three a pass can state from the two
it cannot**, per `weaver-diagnostic-Spec` section 3.3: its `ReplayOutcome` names
certified, diverged, and abandoned, each authored by a pass that reached its own
end, and **a pass that died authors no close at all**, which is that Spec's own
refusal to manufacture one.

**So the absence is one answer and not two.** A bracket with no `replay.closed`
is a pass that did not end, and whether it died or is still running is not a
distinction this record makes or this crate needs: both leave the same absence,
both may yet be followed by nothing, and reading either as an ending would be
treating the end of available bytes as the end of a run, which is what this
paragraph refused before the marker existed and still refuses. So this crate
gates on the outcome the record states: it produces its reading where a bracket
closed certified, produces the divergence where one closed diverged, produces
neither where one closed abandoned, and **produces nothing for any unclosed
bracket, on the same terms whichever way it came to be unclosed**. Where that
marker landed is section 4's cell, now settled. An
earlier form of this paragraph had the comparison here on the ground that this
crate holds both records, which is true and is not the reason the loop cannot,
so it would have put a second implementation of one check on the other side of
a sink.

## 4. Open cells, each named rather than implied

- **The trace as an input format, settled 2026-08-24 and left here as the
  obligation it puts on this crate.** The record carries no version marker and
  needs none, per `weaver-trace-PRD` section 6: the schema extends and does
  not change, so every vintage is the one schema and a reader keys on nothing.
  What that costs this crate is a rule its parser owes rather than a question
  it was owed. **The parser skips a kind it does not know and ignores a
  payload member it does not know, and lets neither decide a grouping.** The
  replay's grouping runs on run and turn from the envelope and on
  request-to-measurement pairing in landing order, none of which an
  unrecognised record can move, so the rule is satisfiable here rather than
  merely stated. **The other direction binds this parser harder**: a record
  written before a member existed omits it, and the parser reads that record
  without rejecting it and without deriving the missing member from the
  members beside it, per the same section. This crate is the place that rule
  costs something, the layer counts on the measurement payload being younger
  than the traces it will be pointed at, and deriving a layer count from a
  norm array is the arithmetic the counts were added to retire. A replay over
  a record that predates them is a replay whose layer count is unknown rather
  than one whose layer count is guessed. The same rules bind the
  diagnostic-trace when this crate reads it, that record being versionless on
  the same terms and for the same reason.
- **The instrument suite.** What this crate carries beyond the certification
  is named in the chartering ruling as a sketch that does not exist in this
  tree. The reference is recorded so the ghost is a known gap rather than a
  silent one, and no crate is built against it.
- **The capture artifact** - identity, custody, dataset shape, quota - is owed
  its own act and moves here with the reading, `weaver-diagnostic-PRD` section
  6 having carried it while one crate held both roles.
- **The licence boundary.** This crate is the piece that can be given away and
  it carries no cut-and-recompute, which is cleaner than expected. The
  intervention loop shares that mechanic with the calculator loop and is where
  the boundary runs. The call is the operator's.
- **How a diagnostic-trace says it ended, and how it says what happened.** The
  gating of section 3 depends on it and cannot be honoured without it. **It is
  not settled here, and settling it here would reach past two things the corpus
  holds open on purpose.** The first is the diagnostic-trace's event
  vocabulary, which `weaver-diagnostic-PRD` section 6 owes to that crate's Spec
  and which is where a terminal event would have to be declared, a serving
  record's `unload` and `session.closed` being that vocabulary's answer rather
  than a shape this crate may assume carries over. The second is the sink's
  discriminant, which section 3 states this charter assumes nothing about, so a
  rule written per shape - what a closed file, a drained pipe, or a dropped
  connection each mean - would put back the assumption that section withdrew
  and would make the consumer's reading depend on an operator's declaration.
  **The outcomes a marker has to separate are nameable now and are named here
  so the owed act has its criterion**: certified and ended, failed its
  comparison and ended, ended without finishing, and not ended. The first three
  are facts the run knows and can author. The fourth is the absence of the
  other three, which is why it costs nothing to distinguish once any of them
  exists. **Settled 2026-08-27 in `weaver-diagnostic-Spec`**, which was owed it
  and which lands the marker as this cell's criterion asked: the first three
  outcomes ride `replay.closed`'s `ReplayOutcome`, the second splitting by which
  of certification's two comparisons diverged, and the fourth is that event's
  absence and costs nothing, exactly as this cell read it forward. The identity a
  claim rests on rides its own kind for a reason this cell did not foresee: a pass
  whose replay answer never arrived can open a bracket and author no identity,
  which is how the account this crate reads says that nothing was replayed.
- **This charter names no Rust item and elects no representation.** **Its Spec
  landed 2026-08-27** and the driver's shape and the parser's are there: the
  parse's own read types answering to `weaver-trace-Spec` section 3 under G5,
  the election composed from what the replay reads rather than declared by an
  operator, the projection splicing raw payload text so a holding cannot say by
  its own bytes which side landed it, and the gate on the outcome this
  charter's section 3 now rests on. **The certification's mechanics do not**,
  that comparison belonging to
  the loop inside the run per section 3, and what this crate's Spec settles
  about it is only the order it consumes outcomes in, requiring a certified
  null pass before any reading downstream.
