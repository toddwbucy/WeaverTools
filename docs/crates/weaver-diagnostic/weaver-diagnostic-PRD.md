# weaver-diagnostic - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. Ratified on its own
terms under the per-charter rule of 2026-08-23, conforming to the pattern the
2026-08-04 act established.

**Revised:** 2026-08-24, second of this date, the diagnostic binding writes no
record and this crate names the other product. Section 6 records that
`weaver-trace` has nothing to do in a replay at either end, not read because a
reader is downstream of a file and not written because a replay performs no
cognition to record, and names the **diagnostic-trace** as this crate's, a
trace in form and separate for how it is made rather than for what it looks
like: one begets the other, a diagnostic-trace being made from a trace in a
second context, so the two stand in a parent relation and a single type would
leave a file unable to say which of the two it is. Canonical form stays
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

`weaver-diagnostic` is **the diagnostic consumer**: a crate outside the agent
boundary, standing in the structural position Weaver Web stands in, that
consumes a finished trace and drives a replay of it against an agent loaded
for exactly that purpose. It is chartered by the operator's ruling of
2026-08-23, carried by the epic that tracked it, and it is the second half of
the taxonomy promotion the operator split on 2026-08-24: the binding kinds
landed in `weaver-agents-PRD` section 6 and the acts of that date, and the
loop class lands here.

**It never sees weights.** The tap is not part of it: the residual readout is
SPU-internal, elected at the load, and puts vectors on the wire. Everything
downstream of the wire is post-processing outside the boundary and belongs
here - the fitting, the projection, the layer trajectory, the artifact store,
and the reading.

**It binds no listening port and the agent never grips it.** The agent has no
channel to this crate, no knowledge of it, and no behavior conditioned on its
presence. What the agent has is a binding kind, declared at the load, per
`weaver-agents-PRD` section 6 as amended 2026-08-24.

**Its contract is with a binding rather than with an organ.** It requires an
agent loaded diagnostically and checks that precondition at the seam. A
diagnostic consumer pointed at a serving binding is refused, and the refusal
is cheap because the binding declared what it is at the load and nothing
transitions afterward.

```graph
node: weaver-diagnostic
kind: crate

edge: parent
from: weaver-diagnostic
to: WeaverTools
```

## 2. What it is not

**Not an organ.** An organ governs a domain and holds a duplex channel with
the harness, both properties and neither alone, and this crate has neither.
It governs nothing inside the agent and the harness never speaks to it.

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

**The preload crosses a second seam on the state member, per the operator's
ruling of 2026-08-24, and the papers stand**: `weaver-diagnostic-state-contract`,
initiated by this crate, checked against the operator principal, on the
precedent of the gate's two doors. The contract, `weaver-state-PRD`'s second
door, and `weaver-state-Spec`'s mechanics landed together in the seam-papers
act of the same date. The seam is declared here, from the initiating side,
which is the pattern the harness-state seam set: the from side's charter
carries the edge. An earlier form of this paragraph placed the declaration
with state and the landing in the future, and both halves are corrected by
the act that landed it.

```graph
edge: seam
from: weaver-diagnostic
to: weaver-state
via: weaver-diagnostic-state-contract
tag: socket
```

**Custody is unchanged and needs no exception.** The trace path is declared
material, so the driver reads the operator-held stream as an operator
principal, over the operator's own storage. The trace parser belongs to this
crate, outside the agent. `weaver-state` never opens a trace file and
`weaver-trace` keeps its write-only pin.

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
is the bulk of it and the reason it exists. Sharing the form is the point: an
instrument that already reads a record reads this one, and a reader that had
to learn a second shape to see the same run twice would be paying for a
distinction nothing needs. What differs is the making, and the making is
the whole distinction.

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

The custody differs on the same seam. A serving record is authored event by
event by the harness as the sole writer, inside the agent, into a sink admin
opened under root, bracketed by a turn as it happens. This one is assembled
outside the agent from what the replay returns, by a crate the boundary keeps
out of that custody chain, over a run whose turns are read from a record
rather than lived. Two writers with two custody stories cannot be one crate,
and `weaver-trace`'s charter says so from its own side: it has one caller and
no other crate submits an event to it. **So this crate carries
the diagnostic counterpart** to it, the writer that makes a diagnostic-trace,
standing to a replay as `weaver-trace` stands to a turn.

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

**The harness relays and never assembles.** The measurement returns from the
SPU by the same path the generation does, per apex section 3 step 6, so it
reaches the harness whatever the binding's kind. On a serving binding the
harness authors it into the record. On a diagnostic one there is no record to
author into, so the harness passes it outward holding it opaque, on the
precedent `weaver-trace` already sets for message payloads, whose shapes it
neither knows nor versions. **The diagnostic-trace is assembled at the far
end**, by this crate, from what the relay delivers, which is what keeps the
dependency running the one way the boundary allows: this crate sits outside
the agent, and a harness that named this crate's writer would depend outward.
Holding the payload opaque is what makes the relay cost nothing structurally.
The exit path and its directives are owed with the driver, below.

**The refusal rests on the binding's declaration, and its form is the door's
absence.** State learns the kind at its standing, from the party that stands
it, which resolved the kind at inventory, and binds the preload name only
under a diagnostic binding. A driver pointed at a serving agent therefore
finds nothing to dial, per the contract: no directive arrives to be refused,
because the seam it would cross does not stand. An earlier form of this
paragraph described a refusal at the seam, which the landed papers made
structural instead.

Open cells, each named rather than implied:

- **The licence boundary.** This crate is the piece that can be given away
  and it carries no cut-and-recompute, which is cleaner than expected. The
  intervention loop shares that mechanic with the calculator loop and is
  where the boundary runs. The operator's.
- **The capture artifact** - identity, custody, dataset shape, quota - is
  owed its own act and this charter does not shape it.
- **The instrument suite** this crate would eventually carry was named in
  the chartering ruling as a sketch that does not exist in this tree. The
  reference is recorded here so the ghost is a known gap rather than a
  silent one.
- **The driver and its Spec** are owed next, the seam papers having landed,
  and the null replay of section 4 is owed behind the driver.
- **The diagnostic-trace's own shape** is owed with the driver's Spec, which
  is the document permitted to name it and the writer that makes it. This
  section fixes what it is, whose it is, that it shares canonical form under
  `weaver-trace-Spec`'s authority, and that the harness holds it opaque. What
  it does not fix is how much of that crate's event vocabulary the diagnostic
  writer reuses and how the residual readout sits beside it, which is the
  Spec's election and the larger half of the work. **The route the
  measurements take outward is owed in the same act**: they leave by admin,
  per the operator's ruling of 2026-08-24, and no directive carries them
  until that act writes one, a declared route with no far end being the
  empty joint apex section 9 refuses. What travels that route is the relay's
  material rather than the assembled diagnostic-trace, the assembly standing
  at the far end by the paragraph above, and the act that writes the
  directives is the act that fixes where the far end sits.
- **The trace as an input format.** This crate parses a record whose shape is
  declared in `weaver-trace-Spec` section 3, and no code is shared between
  the writer and this parser, which is the boundary working as intended and
  is also two statements of one fact. The authority is `weaver-trace-Spec`
  and a divergence is a defect in this crate, per G5. **The record carries no
  version marker**, and where a serving run only ever grew its vocabulary, a
  replay reads that vocabulary as an input, so a parser written against one
  set and pointed at a record written under another does not fail and instead
  groups differently. Whether the marker lands on the record or the
  compatibility is stated some other way is owed ahead of the driver, and it
  is `weaver-trace`'s act rather than this crate's.
