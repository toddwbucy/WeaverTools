# weaver-analysis - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. Ratified on its own
terms under the per-charter rule of 2026-08-23, conforming to the pattern the
2026-08-04 act established.

**Date filed:** 2026-08-24
**Revised:** 2026-09-01, the kept artifacts take their identity. Section 3
gains the capture-artifact clauses - identity as the closure of the claim,
custody the sink's existing arrangement, shape the record's own, quota the
operator's sink election - and the lens-artifact clauses - identity by
weights, corpus, estimator, and environment, the weights hash as the
version, refits as new artifacts, the fit size elected at two hundred
prompts from the saturation measurement. Section 4's cell closes. Every
member writes from the measurement acts of this date, and the
representation is `weaver-analysis-Spec` section 3's in the same act.

**Revised:** 2026-09-01, the declaration derives from the record. Section 3
gains the rule of issue #394 on the operator's direction: every fact of the
source run comes from the record and the analyst declares only the
diagnostic run's own three - device placement, the readers' elections, and
the sink - with the record-silent-property class as the ground and the
later random-seed case carried free. `weaver-analysis-Spec` section 3 takes
the representation and `diagnostic-replay-loop` section 2 the walk's
sentence, every party in the same act.

**Revised:** 2026-08-31, second of this date, code identity joins the bound.
The licence names code identity beside the device model, each identifier at
its own precision - the commit by hash, the toolchain and driver by pinned
version, the binaries and engine libraries by the driver's sha256 - the same
status and the same reader, the record events owed.

**Revised:** 2026-08-31, the discard's licence. Section 3 gains the operator's
ruling of 2026-08-30: a pipe-shaped sink retains nothing, the report is the
kept artifact carrying its evidence, and the discard is licensed by the
replay's certification at the exactness the corpus grants each payload, token
path exact and vectors within the diagnostic charter's stated tolerance,
bounded by the device model the deposit names, which the report names or
states it cannot, the record event that would carry it named as owed. Nothing
about the sink's governance moves.

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

**It derives the diagnostic declaration from the record it parsed, and the
analyst declares only the diagnostic run's own facts.** Per the operator's
direction of 2026-08-31 on issue #394. Every fact of the source run comes
from the record - the artifact, the seated identity prefix, the declared
seed and every tunable the effective sampling and its bounds name, the
session's own name - because every value an analyst re-types into a
declaration is a chance to be correct to memory instead of to the run,
which is the record-silent-property defect class entering through the
config file. Three facts stay the analyst's to declare, each for its own
reason. **Device placement**, because the record deliberately names no
silicon and a replay on other silicon is a legitimate act the record must
not forbid. **The readers' elections**, riding the declaration per apex
section 8, the analyst's question and never the source's property. **The
diagnostic sink**, the new record's home. A member the record does
not carry and the run under this binding does not read takes the fixed
spelling the Spec names rather than a guess. The rule is also what makes a
later randomly drawn seed free: a value that lands in the record's
sampling members is picked up by the derivation with no further act, where
a re-typed declaration would be wrong by construction. The replay's own
identity checks then guard the derivation itself - a drifted derivation
fails certification rather than replaying a run that never was.

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

**Where the sink is a file, the kept record is the capture artifact, and
this clause is what a kept capture must carry to be one.** Per the cell
this act closes, every member measured before it was papered. **Identity**:
a capture is a certified diagnostic record, and its identity is the closure
of what its claim rests on - the source trace it replayed, the declaration
the driver derived (itself the record's projection plus the analyst's
three), the device model and code identity the licence clause already
requires, and the certification outcome in the record's own
`replay.closed`. An uncertified record is an account and never a capture,
per the no-second-instrument rule. **Custody**: the operator's, outside the
agent, on the arrangement the sink already has - admin opens it, the
operator contract governs what crosses out - and this crate reads it as an
operator principal like every record. **Dataset shape**: the
diagnostic-trace's own, per `weaver-diagnostic-Spec`, at whatever density
the load elected. No second format exists, because a capture is a record
kept rather than a record converted. **Quota**: the sink shape is the
operator's retention election - a pipe retains nothing and a file retains
whole - and this charter adds no ceiling of its own: what bounds keeping is
the operator's storage, and the discard licence stands wherever
certification does, a kept capture being the operator declining a licence
rather than lacking one.

**The lens artifact is the fitted transport, versioned against the weights
it was fitted to and meeting only their captures.** **Identity**: the
weights by content hash, the corpus by source, selection rule, and content
hash, the estimator by implementation revision and parameters, and the
environment that ran the fit - each spelled in a manifest beside the
matrices, and a reader refuses a lens whose manifest names other weights,
recomputing the hash against the model in hand rather than trusting the
name. **Versioning**: the weights hash is the version, so a refit is a new
artifact beside the old and never a mutation - a lens refitted later
applies to a capture recorded earlier, the capture holding activations
rather than readouts, which is the provenance property the stream design
bought. **The fit-size election is made from measurement**: two hundred
corpus prompts, the paper-scale fit at five times the compute having moved
no evaluation number, per the measurement act of 2026-09-01. **Custody**:
the operator's storage beside the captures, outside the agent, the fit
never touching the agent at all.

**A pipe-shaped sink retains nothing, and the report is the kept artifact.**
Per the operator's ruling of 2026-08-30: a diagnostic run whose sink is a pipe
streams through this crate, the reading is taken as the stream drains, the
report carries its evidence, and the raw capture is kept nowhere. Retention is
not this crate's to choose there - the sink's shape made it - so what this
clause governs is the claim a report may make about what was not kept.

**What licenses the discard is the replay's own certification, stated at the
exactness the corpus grants each payload and no more.** The token path is
held exact and the vectors within the tolerance `weaver-diagnostic-PRD`
section 4's comparison states, that document declining bitwise equality of
floats on purpose, and this clause claims nothing stronger for the columns
than the certification that regenerates them claims. A capture the
certification vouches for is derivable rather than data, so keeping it would
store what can be recomputed to the same certified exactness, and the trace
with its declaration is already the capture's compressed form. The evidence
in hand is the measured half: within one device the weekend's replays of
2026-08-29 through 30 reproduced token paths and per-token entropies
byte-identical across 5,530 sessions and five precisions - the reduction
rather than the columns, which is why the tolerance clause above carries the
columns' share.

**The licence is bounded by the device model the deposit names, and the
report must name it or say that it cannot.** Reissue holds within one device
model and is refuted across them by the measurement of issue #346, so a
report whose evidence was discarded is reproducible on the silicon that
produced it and nowhere else, and a report that cannot establish which states
that plainly, carrying no reproducibility claim in place of a member that
would read as one. The serving device does not yet ride the run's own record:
it reaches the deposit through the driver that took it, per the
run-records-what-served act of 2026-08-28, and a record event carrying it is
owed its own act, named here as owed rather than assumed present. Until it
lands the report reads the device model from the deposit the operator holds.
**Code identity has the same status and the same reader**: regeneration runs
the seam's code as well as the silicon, and the repository commit reaches
the deposit by hash, the toolchain and the driver by pinned version string,
and the engine libraries and organ binaries by the sha256 the driver takes
of each, so the report reads them where it reads the device, and a report
surviving a rebuild claims nothing the deposit's identifiers do not carry.

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
- **The capture artifact** - identity, custody, dataset shape, quota -
  **closed 2026-09-01** in section 3's kept-artifact clauses, every member
  written from the measurement acts of the same date rather than assumed,
  and the lens artifact beside it. `weaver-diagnostic-PRD` section 6 carried
  it while one crate held both roles.
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
