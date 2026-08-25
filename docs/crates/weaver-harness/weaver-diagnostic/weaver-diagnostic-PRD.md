# weaver-diagnostic - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. Ratified on its own
terms under the per-charter rule of 2026-08-23, conforming to the pattern the
2026-08-04 act established.

**Revised:** 2026-08-24, fourth of this date, this crate composes inside. The
operator's ruling of this date moved it from outside the agent boundary to the
harness's third member, the mechanism the harness authors a diagnostic-trace
through, standing to a replay as `weaver-trace` stands to a turn. The document
moves to the harness container, the parent edge goes to `weaver-harness`, and
the crate enters the apex roster by that document's inside rule. What was the
consumer half is chartered separately as `weaver-analysis`, which takes the
preload seam and its edge, the parser and its obligations, the certification's
performance, the capture artifact, the instrument suite, and the licence
boundary. The seam contract is renamed with its party. The three entries below
describe a crate outside the boundary and stand unamended so the move is
visible in the history rather than erased from it.
**Revised:** 2026-08-24, third of this date, the input-format cell closes. It
asked whether a version marker lands or the compatibility is stated some other
way, and `weaver-trace-PRD` section 6 answers the second in the same act: the
schema extends rather than changes, so no marker lands and a reader keys on
nothing. What remains here is the obligation the answer puts on this crate's
parser, to skip an unknown kind, ignore an unknown payload member, and let
neither decide a grouping, which the replay's own grouping satisfies because it
runs on the envelope's run and turn and on landing order, and to read a record
that omits a member added after it was written without rejecting it and without
deriving the member from the ones beside it, which is where this crate pays for
the rule because the measurement's layer counts are younger than the traces it
will be pointed at.
**Revised:** 2026-08-24, second of this date, the diagnostic binding writes no
record and this crate names the other product. Section 6 records that
`weaver-trace` has nothing to do in a replay at either end, not read because a
reader is downstream of a file and not written because a replay performs no
cognition to record, and names the **diagnostic-trace** as this crate's, a
trace in form and separate for how it is made rather than for what it looks
like: one begets the other, a diagnostic-trace being made from a trace in a
second context, so the two stand in a parent relation and a single type would
leave a file unable to say which of the two it is. What the section shares is
the canonical serialization and nothing above it, reader compatibility being
the driver Spec's to define once the vocabulary and the readout's placement
are elected, and the input-format cell now runs both directions because the
diagnostic-trace is versionless on the same terms. Canonical form stays
`weaver-trace-Spec`'s under G5, and this crate
carries the diagnostic counterpart to that crate, the writer standing to a
replay as `weaver-trace` stands to a turn. The harness relays opaque and
assembles nothing, which keeps the dependency from running outward across the
boundary. The crate defines `diagnostic-trace` as a term in the same section.
The owed list gains the diagnostic-trace's form and exit, both to the driver's
Spec, and gains the trace-as-an-input-format cell, which is `weaver-trace`'s
act and sits ahead of the driver. The entry below gains the ordinal the
convention asks for, which it was filed without.
**Revised:** 2026-08-24, first of this date, the seam papers land. Section 6's seam
paragraph records the landing and corrects its own account of the declaring side: the
seam edge is declared here, from the initiating side, per the pattern the harness-state
seam set, and not by state as the paragraph first said. The owed list narrows to the
driver, its Spec, and the null replay. The serving-binding refusal is restated in the
papers' terms, the door's absence rather than a refusal at a seam that does not stand.
**Date filed:** 2026-08-24 **Document ID:** `weaver-diagnostic-PRD` **Parent:** the
WeaverTools suite, whose governing document is deliberately not yet written, per
`weaver-agents-PRD` section 0. The graph parent edge names the `WeaverTools` system
node, and the header and the edge name the same thing. **Editorial:** Per the Working
Rules.

---

## 1. What this crate is

`weaver-diagnostic` is **the harness's third member and the diagnostic-trace's
mechanism**, standing to a replay exactly as `weaver-trace` stands to a turn.
It is chartered by the operator's ruling of 2026-08-23, carried by the epic
that tracked it, and it is the second half of the taxonomy promotion the
operator split on 2026-08-24: the binding kinds landed in `weaver-agents-PRD`
section 6 and the acts of that date, and the loop class lands here.

**It does not produce the diagnostic-trace.** The harness authors. This crate
is the mechanism the harness authors through, and the distinction is the whole
of the charter, restated from `weaver-trace-PRD` section 1 because the two
members stand in one relation to one author and a reader holding both should
meet the same sentence twice. `weaver-trace` is the mechanism under a serving
binding and this crate is the mechanism under a diagnostic one, the harness
being the sole writer either way.

**It never sees weights.** The tap is not part of it: the residual readout is
SPU-internal, elected at the load, and puts vectors on the wire. What this
crate receives is what the wire carried, and it renders that into the record.
The fitting, the projection, the layer trajectory, the artifact store, and the
reading are downstream of the record and belong to `weaver-analysis`, outside
the agent, per section 6.

**It binds no listening port and holds no cognition**, which is the property it
shares with `weaver-trace` and the reason neither is an organ. It governs no
domain and holds no duplex channel with the harness. It is reached by being
linked and called, one caller and no other, and its output leaves by the sink
admin opened for the binding.

**Which member the harness authors through is the binding kind's to settle, and
nothing else's.** A serving binding composes `weaver-trace` and a diagnostic
binding composes this crate, decided at the load and never entered afterward,
so no run holds both and no code chooses between them at a turn's grain.

```graph
node: weaver-diagnostic
kind: crate

edge: parent
from: weaver-diagnostic
to: weaver-harness
```

## 2. What it is not

**Not an organ.** An organ governs a domain and holds a duplex channel with
the harness, both properties and neither alone, and this crate has neither.
It governs no domain, and the harness reaches it by linking rather than by a
channel, which is `weaver-trace`'s position and for `weaver-trace`'s reason:
a type definition does not travel over a socket and a rendering mechanism is
not a peer.

**Not the consumer.** An earlier form of this charter placed this crate
outside the agent as the diagnostic consumer, and the operator's ruling of
2026-08-24 corrected it. The consumer is `weaver-analysis`, chartered in the
same act, which reads the record this crate renders. Nothing of the reading
belongs here.

**Not the intervention overlay.** Cut-and-recompute is a mechanic this crate
does not carry, per the taxonomy's ruling: intervention composes with either
substrate and its product is a counterfactual token path, which is a
different deliverable with a different custody story. The calculator loop
holds that mechanic on the production side, and where the licence boundary
runs through it is the operator's open item, named in section 6.

**Not a memory leg.** It reads records and writes analysis artifacts. Nothing
it produces enters an agent's state, its prompt, or its working structure,
and statefulness returns through `weaver-agents-PRD` section 9's door or not
at all.

**Not an evaluator.** The mechanic-motive line of section 5 is this crate's
outer edge: it claims a faithful readout with replayability and claims
nothing about what the readout is worth.

## 3. The diagnostic loop is a class, and the readout is interchangeable

The diagnostic substrate is defined by three refusals, against the production
substrate's grants:

- **Gate never starts.** A diagnostic binding declares no Gate, per
  `weaver-agents-PRD` section 6, so nothing enters from outside and the
  wrong arrangement is unrepresentable rather than guarded against.
- **The working structure is preloaded from a finished trace** and read
  positionally as the source of prompts, rather than accumulating. Same
  organ, opposite direction. The preload crosses the ruled seam of section
  6, owed to the seam-papers act, and until that act lands this refusal is
  chartered and not yet drivable.
- **Nothing writes back**, so the substrate under examination is immutable
  for the loop's duration.

**The residual tap is one passive reader on a faithful re-execution, and the
loop is a class because the reader swaps without the loop changing.** The
membership test is mechanical: a member needs only a forward pass over a
fixed token path, and writes nothing. Attention-pattern capture passes.
Per-layer logit-lens decoding passes and is the cheapest member.
Recomputation of per-position entropy and surprisal passes and is close to
free, the measurement payload already carrying the production figures to
compare against. Activation patching fails, because it changes the input and
so produces a different run - it is the intervention overlay meeting this
substrate, not a member of the class.

**What is authored once is the loop, the custody rule, the artifact
identity, and the certification. Every instrument after the first is a tap
plus a Spec clause.** Naming the loop for one instrument would make the
class look like one thing, and the second instrument would then either
wrongly amend it or wrongly fork it.

## 4. Certification, and what inherits what

**The null replay certifies the substrate.** The mechanic's correctness is
not established by whether a readout is interpretable. It is established by
exact-match comparison of a replay against its original, and then again with
the readout elected, to show the read is passive. Exact match is a claim
about the token path, which is integers and matches or does not: the elected
readout's vectors are deterministic given the same weights within GPU float
tolerance, per `weaver-agents-PRD` section 8, so the passive-read comparison
holds the token path exact and the vectors within that tolerance rather than
demanding bitwise equality of floats. A readout from an
uncertified replay is a picture of an unknown run.

**Certification is two claims and not one.** An output comparison means
nothing unless the input was the same one, so the certification checks the
input first, established from the record: input token identifiers, output
token identifiers, model identity with its weights hash, sampling
parameters, and the prompt-block partition, per `weaver-agents-PRD` section
8, with the template's identity traveling with them so a replay re-feeds
rather than re-renders. The same conversation rendered under a later
template is a different prompt, and a replay that re-rendered would compare
two different runs and find them different.

**No second instrument lands before the first replay is certified**, because
a second tap sharing an uncertified replay inherits the uncertainty rather
than dividing it.

**The production column needs no replay certification and its mechanic is
not yet shown.** The intended ordering is that cut-and-recompute earns its
correctness on the production side and the diagnostic side inherits it, and
that inheritance is conditional on a splice that has not landed, per
`weaver-internal`'s own cell. A plan rather than a record, stated as such.

## 5. The mechanic and the motive

**In scope, and this program's to answer:** whether replay re-executes the
same forward passes, whether the readout perturbs the pass, whether the
capture joins back to the forward pass that produced it, whether the
artifact is identified by what determines it, and whether the vectors reach
disk uncorrupted.

**Out of scope and the operator's:** whether a readout means anything, how
it compares against a cheaper instrument, how many fitting sequences
suffice, what a trajectory across layers indicates, and whether one
binding's readouts agree with another's. Two bindings serving the same
weights at different precisions are, to the mechanic, two bindings it must
serve identically, and whether their readouts agree is a finding.

## 6. The seam, the custody, and what this charter leaves owed

**This crate holds no seam.** It is linked and called, one caller and no
other, which is `weaver-trace`'s position and for the same reason: a rendering
mechanism is not a peer and nothing it makes travels except by the sink its
author writes into.

**The preload crosses a second seam on the state member, per the operator's
ruling of 2026-08-24, and the papers stand**, but the near side of it is not
this crate. `weaver-analysis-state-contract` is initiated by `weaver-analysis`
from outside the agent, checked against the operator principal, on the
precedent of the gate's two doors, and the edge is declared in
`weaver-analysis-PRD` where the initiating side's charter carries it. An
earlier form of this section declared that seam here, when this crate was
chartered as the consumer, and the ruling that moved it inside moved the seam
with the party rather than leaving an inside crate holding an operator's door.

**Custody is unchanged and needs no exception.** The trace path is declared
material, so `weaver-analysis` reads the operator-held stream as an operator
principal, over the operator's own storage, and preloads it through the door
below. The trace parser belongs to that crate, outside the agent, on the same
ground the reading does. `weaver-state` never opens a trace file and
`weaver-trace` keeps its write-only pin. Nothing inside the agent opens the
record the replay runs from.

**The record runs one way only, and this crate owns the other product.** A
diagnostic binding writes no trace, per `weaver-agents-PRD` section 6 as
amended 2026-08-24, so `weaver-trace` has nothing to do in a replay at either
end: it is not read, because a reader of a finished record is downstream of a
file rather than a party to that crate, and it is not written, because a
replay performs no cognition of its own to record. What a diagnostic run
produces instead is the **diagnostic-trace**, this crate's own record of the
replay, named here for the first time.

**The diagnostic-trace is a trace, and the two are separate for how they are
made rather than for what they look like.** It carries the run the way a
serving record carries one and carries the residual readout beside it, which
is the bulk of it and the reason it exists. **What is shared and settled here
is the canonical serialization and nothing above it**, one line of UTF-8 JSON
per record with the newline as the separator, per `weaver-trace-Spec` section
2. Whether an instrument that reads a serving record reads this one is **not
claimed**: that follows from the event vocabulary this writer reuses and from
where the residual readout sits beside it, both of which are the driver
Spec's election, so reader compatibility is that document's to define and to
state the rules of. What differs is the making, and the making is the whole
distinction.

**One begets the other, in a different context.** A diagnostic-trace is made
from a trace: the serving record is the replay's input, and the
diagnostic-trace is what the replay returns, so the two stand in a parent
relation rather than side by side. That is why they cannot be one type
however alike they read. A single type would have a record and the record
derived from it wearing one name, and the first question anyone asks of a
file in hand, which of the two it is, would have no answer in the file. It
would also make the derivation circular on its face, a shape defined in terms
of a run over itself, when what happened is that one run was read and a
second was performed against it.

The making differs on the same author. Both records are authored event by
event by the harness as the sole writer, inside the agent, into a sink admin
opened for the binding. What differs is the run underneath: a serving record
brackets a turn as it happens, and this one runs over turns read from a
record rather than lived, carrying the residual readout that only a replay
elects. Two provenances that far apart cannot share one mechanism without the
mechanism losing the ability to say which it made, and `weaver-trace`'s
charter closes the question from its own side: it has one caller and no other
crate submits an event to it. **So this crate is the counterpart mechanism**,
the one the harness authors a diagnostic-trace through, standing to a replay
as `weaver-trace` stands to a turn.

**The shared form has one authority and it is not this crate.** Canonical form
is `weaver-trace-Spec` section 2's, one line of UTF-8 JSON per record with the
newline as the separator, and where this crate's writer diverges from it the
defect is this crate's, per G5. That is the same authority the parser side of
section 6 answers to, so both directions of this crate's traffic in records
read one document rather than two.

```graph
node: diagnostic-trace
kind: term

edge: defines
from: weaver-diagnostic
to: diagnostic-trace
```

**The harness authors through this crate and the record leaves by the sink.**
The measurement returns from the SPU by the same path the generation does,
per apex section 3 step 6, so it reaches the harness whatever the binding's
kind. On a serving binding the harness authors it through `weaver-trace`. On
a diagnostic one it authors it through this crate, into the diagnostic-trace,
beside the replayed run the same record carries. The rendering leaves by the
sink admin opened for the binding, which is a socket rather than a file
because the consumer is a process rather than an archive, and
`weaver-analysis` is what stands at the far end of it. **Admin's custody of
the sink is unchanged**, the discriminant and the open site being section 5
of `weaver-admin-Spec` whichever kind the binding declared, so this path adds
a shape to what admin opens rather than a second way of opening.

**Nothing here crosses the boundary outward.** The harness links a member of
its own domain, which is the relation it already has with `weaver-trace` and
`weaver-state`, and the one crate outside the agent reaches the record only
after it has left by the sink. An earlier form of this section had the
harness relaying to an outside assembler, which would have put an agent crate
in reach of a consumer's shape, and the ruling of 2026-08-24 removed the
question rather than answering it.

**The refusal rests on the binding's declaration, and its form is the door's
absence.** State learns the kind at its standing, from the party that stands
it, which resolved the kind at inventory, and binds the preload name only
under a diagnostic binding. A driver pointed at a serving agent therefore
finds nothing to dial, per the contract: no directive arrives to be refused,
because the seam it would cross does not stand. An earlier form of this
paragraph described a refusal at the seam, which the landed papers made
structural instead.

Open cells, each named rather than implied:

- **This crate's Spec** is owed, and the diagnostic-trace's shape lands there
  because a Spec is the only document permitted to name it. This charter
  fixes what the record is, whose it is, and that its canonical form is
  `weaver-trace-Spec` section 2's under G5. What it does not fix is how much
  of that crate's event vocabulary the diagnostic writer reuses and how the
  residual readout sits beside it, which is the election and the larger half
  of the work.
- **The seam this crate presents to the harness** is owed with that Spec:
  whether it mirrors `weaver-trace`'s receive and submit surface, so one call
  site in the harness serves both mechanisms, or takes its own. Named as
  owed in `weaver-harness-Spec` section 9 from the other side.
- **The null replay** of section 4 is owed behind both, and it is the run
  that certifies this mechanism rather than a property this charter can
  assert.
- **What this charter no longer carries.** The licence boundary, the capture
  artifact, the instrument suite, and the parser's obligations moved to
  `weaver-analysis-PRD` with the reading, on the operator's ruling of
  2026-08-24 that split this leg in two. They are named here as departed
  rather than dropped, because this document carried them for one day and a
  reader of its history will meet them.
