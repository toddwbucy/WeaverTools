# weaver-diagnostic - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. Ratified on its own
terms under the per-charter rule of 2026-08-23, conforming to the pattern the
2026-08-04 act established.

**Revised:** 2026-08-24, the seam papers land. Section 6's seam paragraph
records the landing and corrects its own account of the declaring side: the
seam edge is declared here, from the initiating side, per the pattern the
harness-state seam set, and not by state as the paragraph first said. The
owed list narrows to the driver, its Spec, and the null replay.
**Date filed:** 2026-08-24
**Document ID:** `weaver-diagnostic-PRD`
**Parent:** the WeaverTools suite, whose governing document is deliberately
not yet written, per `weaver-agents-PRD` section 0. The graph parent edge
names the `WeaverTools` system node, and the header and the edge name the
same thing.
**Editorial:** Per the Working Rules.

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

**The refusal at the seam rests on the binding's declaration.** State learns
the kind at its standing, from the party that stands it, which resolved the
kind at inventory. A preload directive arriving at a serving binding's state
member is refused at the seam, before any content crosses.

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
