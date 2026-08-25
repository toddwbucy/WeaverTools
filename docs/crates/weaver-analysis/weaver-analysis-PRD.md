# weaver-analysis - PRD (crate charter)

**Status:** MERGED. In `main` and the source of truth. Ratified on its own
terms under the per-charter rule of 2026-08-23, conforming to the pattern the
2026-08-04 act established.

**Date filed:** 2026-08-24
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
`weaver-analysis-state-contract`. What crosses is distillates and a seal, the
seal being the fact the harness's replay ask answers at.

**It reads the diagnostic-trace off the sink.** The sink is admin's, opened for
the binding under root by whatever discriminant the declaration named, per
`weaver-admin-Spec` section 5, and **this charter assumes no discriminant**:
the operator declares the shape, and whether this crate reads a finished file,
drains a pipe, or holds a connection follows from that declaration rather than
from anything here. Whether the reading trails the run or runs beside it
follows the same way. **No contract governs the delivery and none is owed**,
because a sink's reader is downstream of it rather than a party to it, which is
the position every consumer of a serving trace already occupies, per
`weaver-trace-PRD` section 1.

**It reads nothing for meaning from a replay that did not certify**, per
`weaver-diagnostic-PRD` section 4, which carries the criterion because it
belongs with the mechanism being judged. **The comparison is not performed
here.** `diagnostic-replay-loop` section 3 walks it inside the run, which is
where it has to happen: the loop holds the recorded path in its holdings and
the recomputed identifiers as they arrive, so it alone can refuse before the
first forward pass and name the first divergent position rather than reporting
after a whole replay has run. What this crate does is elect the null replay,
read its outcome from the record, and gate everything downstream on it. An
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
- **This charter names no Rust item and elects no representation.** Its Spec
  is owed, and the driver's shape, the parser's, and the certification's
  mechanics land there rather than here.
