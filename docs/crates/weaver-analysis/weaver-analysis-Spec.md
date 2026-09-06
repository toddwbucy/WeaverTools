# weaver-analysis - Spec

**Status:** MERGED. Cut 2026-08-27, the second Spec of the diagnostic leg. Code is
written against it under the gates of Working Process section 6.

**Date filed:** 2026-08-27
**Revised:** 2026-09-06, the summary carries the record's identity, and the contract
is cited. Section 5's summary gains, per generation, the weights hash the measurement
carries as the record spelled it, sentinel included, so the run row of
`weaver-web-Spec` section 2.2 has a source on the wire, under one perturbation
record. Section 0 names `weaver-analysis-web-contract` among what this document is
written against, the citation the contract's section 8 recorded as owed since
2026-09-05, per issue #451's second half. Section 6's counts move by one. Per issue
#465.
**Revised:** 2026-09-05, second of this date, the summary carries the residency. Section
5's summary gains, per generation, the resident count as it closed and the count of
output tokens beside the perplexity, both read from the record and neither derived,
so a store keyed by position converts once at ingest from facts the emitter reports.
Section 7's election of this date closes on that answer, the operator having pointed
the seat at issue #461, and its record moves to section 5 as a perturbation. Section
6's counts move by it.
**Revised:** 2026-09-05, the signals reader is authorized. Section 5 gains the clause
for the class's second reader, standing in code since #408 on 2026-09-02 with no
sentence here: the per-position series paired from every generation's measurement on
the same drain, addressed by ordinal where the field read is addressed by position,
absence kept, the spike a rule whose caller names `k`, gated only where the record
has a gate, with the `signals` verb's usage. Section 1's layout gains `stream.rs`
and `signals.rs`. Section 6's counts are retaken from the records. Section 7 names
what the series must carry for a store keyed by position as open, the record's
residency the source and the input count ruled out by measurement. Per issue #451.
**Revised:** 2026-09-04, fourth of this date, a position's field is read from the
record. Section 5 gains the `field` verb: the one `model.field` event at an asked turn
and position, drained from a serving or diagnostic record, file or pipe, spliced as the
record spelled it, holding one position and never the record, and gated on no certified
close because the field is the record's own fact about a position rather than a reading
over a replay. Section 1's layout gains `field.rs`. Per issue #436.

**Revised:** 2026-09-04, third of this date, the preload takes a cut. Section 4's
`preload` accepts `--through <run>:<turn>`, projecting the record through that turn's
close, and `--as <session>`, landing the projection under another session name. Per
issue #432.

**Revised:** 2026-09-04, second of this date, the file's default spread is defined.
Section 5 names it eight positions evenly spaced over the record's generated positions
by index, first and last included, taken by a second read of the file, where the clause
had named a default and defined none. Per the last open defect of issue #386.

**Revised:** 2026-09-04, first of this date, the derived identity is the seed. The
derivation's prefix member names its standing under `weaver-state-PRD` section 4 as
revised this date: the seed of the derived declaration, the preloaded store answering
the replayed run's identity ask from the same events. Per issue #422.

**Revised:** 2026-09-03, the head is applied across the cores. Section 5 states that the
unembedding's rows are split across scoped threads with each row summed in one order, so
the reading is bit-identical to the single-thread one, and the claim gains its record,
`analysis-threaded-head-is-bit-identical`. Forced by the 8B, whose control over two
thousand positions took fifteen minutes on one core.

**Revised:** 2026-09-03, the weights identity is per file. Section 3's lens manifest
carries its weights digest in the shape the model on disk takes, a map of shard digests
for a sharded model, and the reader follows the model's index to the shards it needs,
verifying each under its own name. The identity refusal gains its record,
`analysis-lens-refuses-other-weights`, which the clause had argued without one. Forced
by the 8B, whose head and final norm sit in different shards.

**Revised:** 2026-09-02, the reading drains the stream. Section 3 states that the
analyst's sink input carries its shape, so a pipe elects the discard and a file declines
it, and section 5 states how a reading is taken as the stream drains: one drain for the
class with readers above it, the held state bounded by the turn in flight and the
analyst's named positions, the named positions required where no whole record exists to
spread over, and the reading emitted only after a certified close. One assertion, bought
by the act that streams.

**Revised:** 2026-09-01, third of this date, the reading takes its surface. Section 5
gains what the crate-borne read needs and no more: the lens loaded and applied by the
source's own arithmetic against the artifact's own weights, the control gating every
reading, the by-turn pairing rule, and the exact capture comparison that performs
certification step 3 where both records are held and licenses the discard. Section 1's
layout gains `lens.rs` and `capture.rs`, and its dependency set gains `safetensors` and
`sha2`, the format reader the artifact election implies and the digest its identity
check recomputes, neither an engine. The reading-as-artifact election narrows to its
rendered form. Two assertions, bought by the code act in the same stack.

**Revised:** 2026-09-01, second of this date, the artifacts take their representation.
Section 3 gains the lens artifact's shape - safetensors matrices, elected over the
fitting tool's serialization so both sides of the boundary read the artifact without the
other's runtime, beside the measured JSON manifest with its refusals - and states that
the capture artifact has no second representation, being a certified record kept whole.
Two open elections close. Per the charter as amended this date and the measurement acts
of the same date.

**Revised:** 2026-09-01, first of this date, the declaration derives from the record.
Section 3 gains the third projection per the charter as amended on issue #394: every
derived member names its record source, disagreement and absence each refuse naming the
member, the analyst's three inputs and the two fixed spellings are enumerated, and
section 1's layout gains `src/declare.rs` beside the invocation's composition root,
which it had left implicit. The watch is the code act's, bought in the same stack.

**Revised:** 2026-08-31, second of this date, the gate takes both members. Section 5's
drain paragraph gates the reproducibility claim on the charter's whole licence by
citation - the device model and the code identity, at the precisions that clause states
- where the entry below records a single-member gate the body no longer has.

**Revised:** 2026-08-31, the reading learns to drain. Section 5 gains the pipe-shaped
sink's discipline per the charter's section 3 as amended: read once, keep the report and
its evidence, retain nothing drained, and gate the reproducibility claim rather than the
drain on naming the device model or stating it cannot be established. The kind-set count
follows `weaver-diagnostic-Spec` to seventeen, and section 7 adds the lens artifacts as
an open election beside the capture artifact.

**Document ID:** `weaver-analysis-Spec`
**Parent:** `weaver-analysis-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The representation of the driver `weaver-analysis-PRD` charters: how it parses a
finished record, what it projects across the preload seam, and how it reads the
record a replay makes. It is written against that charter, against
`weaver-analysis-state-contract`, against `weaver-analysis-web-contract` for the
shape the signals reader's summary crosses in, and against `weaver-diagnostic-Spec`
for the record it reads, and it develops no rationale of its own.

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

    src/main.rs       the invocation's composition root, and nothing else
    src/lib.rs        re-exports, and nothing else
    src/record.rs     the parse of a serving record, section 2
    src/project.rs    the election and the projection, section 3
    src/declare.rs    the declaration derived from the record, section 3
    src/preload.rs    the seam's sender, section 4
    src/reading.rs    the diagnostic-trace's parse and the gate, section 5
    src/lens.rs       the lens artifact, loaded and applied, section 5
    src/capture.rs    a capture's columns and their comparison, section 5
    src/field.rs      a position's field, read from the record, section 5
    src/stream.rs     the drain, one road under every reader, section 5
    src/signals.rs    the per-position series, read on the drain, section 5

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
used.

**The dependency set is four crates and no internal one.** `serde` with
`derive`, and `serde_json` with `raw_value`, which sections 2 and 3 elect
for carrying a payload's elected values as the record spelled them rather
than re-encoding them, `safetensors`, which section 5's reading elects to
open the lens artifact and the weights its unembedding needs, and `sha2`,
which the same section's identity check elects to recompute the digest a
manifest names. **Neither addition is an engine**: one maps a file and
answers tensors, the other answers a digest, so what enters this crate is
a parser for the container section 3 elected and the arithmetic that
checks an identity, with no inference runtime - which is the whole reason
that election named a format both sides of the boundary can read.
`safetensors` is the crate `weaver-spu` links for the native backend's
weights and `sha2` already stands in this workspace's resolved tree, both
vetted here rather than newly admitted. **The digest is not hand-rolled**:
an identity check written here would be this crate's approximation of a
standard, and a wrong one would refuse good artifacts or admit bad ones.

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

**The declaration is the third projection, and the record is its one source for every
source-run fact**, per the charter's section 3 as amended on issue #394. The derivation
reads members the record already spells and writes the declaration the operator loads,
so the diagnostic run is correct to the run and never to the analyst's memory. Each
derived member names its source: the session from the envelope, the artifact from
`model.measurement`'s `model`, the seated identity prefix from the turnless
`message.system` events at the run's opening in landing order with each payload carried
verbatim, which since the ruling of 2026-09-04 is the seed the derived declaration
carries while the preloaded store answers the replayed run's `identity` ask, the two
agreeing by construction because both are the same record's events, the seed from
`model.request`'s `sampling.seed`, the per-turn ceiling from that request's
`stop.max_tokens`, and the context capacity from `model.output`'s `capacity`. **A member
the record spells two ways refuses the derivation naming the member**, disagreement
being a question for the operator and never a pick, and **a derived member the record
does not carry refuses the same way** rather than defaulting: completeness is
claim-relative here exactly as it is at input identity, and the claim is the whole
declaration. The rule reaches the derived members alone - the fixed and analyst-supplied
members below come from no record and refuse on no absence.

**Three members are the analyst's inputs and three take fixed values.**
Device placement, the readers' elections, and the diagnostic sink arrive
from the invocation, per the charter's three exceptions. **The sink's
input carries its shape and not only its name**: the charter has this
crate assume no discriminant, so the analyst who elects a pipe is electing
that the run retains nothing and the reading is taken as the stream
drains, and the analyst who elects a file is declining that licence and
keeping a capture. A derivation that could only write one shape would
make that election the crate's rather than the operator's. `binding-kind` is
`diagnostic` by construction, `tool-set` takes the empty list, and
`permission-mode` takes `ask` - the fixed values for members the record
does not carry and a run under this binding never reads, stated here so
the derivation writes a spelling rather than a guess.

```graph
node: analysis-declaration-derives-from-the-record
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-declaration-derives-from-the-record
```

**The lens artifact's representation, per the charter's clauses of this
date.** The matrices are stored as safetensors, one tensor per source layer
named by its index, f32 - elected over the fitting tool's own torch
serialization because both sides of the boundary must read the artifact:
the fitting runs under the reference implementation and this crate reads
the artifact without it, and a pickle-bearing format would put the fitting
tool's runtime inside this crate's parse. The manifest is JSON beside the
matrices: `lens-manifest{-tag}.json` beside the lens file
`jacobian_lens_{model}{-tag}.safetensors`. Its members are the measured
shape: `lens` naming the sibling file, `fitted_for` with the model path,
its `model_safetensors_sha256`, and the dtype the fit ran in, `corpus`
with source, selection rule, and `prompts_sha256`, `estimator` with the
implementation, its revision, and its parameters, `environment`,
`fit_seconds`, and `lens_shape` with `d_model`, `source_layers`, and
`n_prompts`. **The weights digest takes the shape the model on disk takes**,
as of 2026-09-03: one digest for a model kept in one file, and for a sharded
model a map from each shard's file name to its digest, because the reader
recomputes against the files it opens and a single digest over a sharded
model would name a file that does not exist to hash. A reader handed a
directory follows the model's own index to the shards holding the head and
the final norm, opens those and nothing else, and verifies each under its
own name. A shard the map does not name is a file the fit never saw and
refuses as a wrong digest does, and a map naming no shard refuses as its own
case, an empty identity verifying nothing. **A reader
refuses before it reads**: a manifest naming another lens file, other
weights (the hash recomputed against each file the read opens, never
trusted from the name, and the identity's shape and the model's crossing
refusing before a byte is hashed), a width disagreeing with the loaded
matrices, or a `source_layers` set the tensor names do not match one for one
- a missing layer and an extra tensor alike - each refuse naming the member,
the identity discipline the first-light act exercised. The reading names
the files it verified, so a report rests on weights it can point to.

```graph
node: analysis-lens-refuses-other-weights
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-lens-refuses-other-weights
```

**The capture artifact takes no representation here** because it has one
already: a capture is a certified diagnostic record kept whole, per the
charter, and its shape is `weaver-diagnostic-Spec`'s. What this document
adds is only the reading side's rule, section 5's gate unchanged: a record
is a capture exactly where its bracket closed certified.

## 4. The preload

**Three things in one order, and the seam is owed nothing back.** The election
opens the channel, a distillate per elected event follows in sequence order, and
the seal ends it, per the contract's section 2. This crate asks nothing on this
seam and reads nothing from it.

**The seal is an empty JSON object on its own line**, `{}` canonically, and this
crate writes that spelling. A blank line is framing residue and not a seal, per the
contract, so a driver that emitted one would have closed without sealing and the
parked replay ask on the other door would never answer.

**The preload takes a cut, as of 2026-09-04.** `preload` accepts `--through
<run>:<turn>`, a run of the record by its reference and a turn within it, and projects
every event of the record through that turn's close and none after it, the seal
following as before, so a session can stand on a prefix of a record rather than the
whole, per `weaver-state-PRD` section 4 and issue #432. The cut is by turn because the
turn is the record's own unit and a cut inside one would land a generation without its
close. A run the record does not hold, or a turn that run does not hold, refuses before
anything is sent, naming it, and a turn named without its run is refused for the same
reason the floor's `Cut` carries one: a turn's number recurs across runs. `--as
<session>` lands the projection under another session name, every distillate's session
member rewritten as it crosses and run and sequence as recorded, which is what a branch
needs, because the member bounds every answer to the session its opener declared. Under
a restoring load this verb is the door's driver as it is under the diagnostic binding,
per `weaver-analysis-state-contract` section 1 as revised 2026-09-04: admin names the
door and dials it never, the load's enter parks until this verb's seal, and the operator
runs the two side by side as the diagnostic flow already does.

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

```graph
node: analysis-gates-on-the-stated-outcome
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-gates-on-the-stated-outcome
```

**Where the sink is a pipe the reading drains it and keeps nothing but the
report**, per the charter's section 3 as amended on the operator's ruling of
2026-08-30. The stream is read once in landing order, the report carries the
evidence it rests on, and no member of this crate retains the drained bytes:
retention was the sink shape's to give and it did not, and the charter
carries the licence, the certification's exactness per payload, and the
bound. **What the licence's members gate is the claim and never the drain**: the
report names the device model and the code identity the evidence came from -
both members of the charter's licence, read at the precisions that clause
states from the deposit the operator holds until the record event it names
as owed lands - and a report that cannot establish either says so and
carries no reproducibility claim, the absent member otherwise reading as
one.

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

**The lens is loaded here and applied here, and the reading it produces is
the layer trajectory.** The artifact is section 3's: the manifest judged
whole before the file is opened, the header's tensor names answering
before any tensor's data materializes - which is what the format election
bought - and the matrices then held to the manifest one layer for one. The
application is the source's own arithmetic and this crate restates it
rather than inventing one: `unembed(J_l @ h)`, the transport at the layer
the column came from, then the model's own final norm and unembedding, per
`weaver-analysis-PRD` section 1's naming of the reading as this crate's.
**The weights this crate reads for that step are the artifact's own**, the
same safetensors the manifest's hash identifies, so the reading needs no
inference runtime: a matrix multiply, a norm, and a second multiply are
the whole of it. **The second multiply is applied across the cores by
disjoint row ranges**, as of 2026-09-03: the head is a hundred and fifty
thousand rows against one normalized residual, and each row's sum runs in
one thread in index order, so the logits are the single-thread reading's
to the bit and the exactness section 6's compare rests on is untouched by
the parallelism. The threads are the standard library's, scoped, because
section 2's dependency set is four crates and speed is not a reason to
make it five.

```graph
node: analysis-threaded-head-is-bit-identical
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-threaded-head-is-bit-identical
```

**The control precedes every reading and gates it.** At the final layer,
with no transport, the model's own unembedding must rank the token each
position drew at or within the reading's stated bar, and below the bar
this crate produces nothing and says the rate: the pairing, the layer
convention, and the numerics are what the control establishes, and a
trajectory printed over an unestablished pairing is a picture of an
unknown alignment. This is the no-reading-from-an-uncertified-replay rule
one level down, and it is the crate's own bar rather than the record's.

```graph
node: analysis-control-gates-the-reading
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-control-gates-the-reading
```

**A capture's columns are paired by turn and by the measurement's own
order.** A record holds several brackets and positions repeat across them,
so a column is keyed by its turn beside its position, and a turn's
measurement consumes exactly the positions gathered for that turn - the
output order being the draws' own order, the same pairing the field's
realized rank encodes. A column that pairs with no drawn token is not read.

**Two captures compare exactly, and this is certification step 3's own
check performed where both records are held.** Per
`weaver-diagnostic-PRD` section 4 as measured: within one device model the
comparison is exact, so two captures of one source under one declaration
agree value for value or the comparison names the first disagreement with
its turn, position, and layer. **Cardinality is checked and never
truncated**: differing layer counts, differing widths, and an empty column
set each refuse rather than comparing what happens to align, an equal-and-
empty comparison being a verdict over no evidence. **This is what licenses
the discard**: a capture the comparison vouches for is derivable, per the
charter's section 3, and the licence is only as good as the check.

```graph
node: analysis-captures-compare-exactly
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-captures-compare-exactly
```

**The reading is taken as the stream drains, and the drain is the class's
rather than the lens's.** `diagnostic-replay-loop` names the diagnostic
loop a class with an interchangeable reader, so what this crate builds is
one drain over the record's events with readers above it: a reader
consumes events as they land and holds only what its own reading needs.
The lens is the first such reader and sets no precedent the next one must
break.

**What a reading holds while it drains is bounded by one turn.** The
control needs each position's final-layer column against the token that
position drew, and the drawn tokens arrive with the turn's measurement
after its columns, so the final-layer columns of the turn in flight are
held until that measurement pairs them, the ranks are taken, and the
columns are dropped. The trajectory's own columns are held only for the
positions the analyst named. **So a reading over a pipe costs one turn's
final layers and the named positions, never the record**, which is what
makes the discard licence a live property rather than a claim about
storage.

**On a stream the analyst names the positions, and the default spread is the file's
alone.** A spread over the whole record cannot be chosen without the whole record, and a
reader that buffered to find one would have kept what the pipe exists not to keep, so a
stream with no named positions refuses rather than silently retaining. **The file's
default spread is eight positions**, defined here since 2026-09-04 because the clause
named one and nothing defined it, which left the file branch refusing with an empty
list: the record's generated positions in order, the first and the last among the eight
and the rest evenly spaced between them by index, every position where the record holds
eight or fewer. The file is read twice to take it, once to learn its positions and once
to read, because a file can be, and the analyst's `--positions` replaces the spread
rather than adding to it. The control runs over every position in both cases.

**A reading is produced only where the record's own bracket closed
certified**, per the charter's section 3 and the gate above: the outcome
arrives at the end of the stream, so the reading accumulates while the
stream runs and is emitted after the close, and a bracket that closed
otherwise or did not close produces nothing. A readout from an
uncertified replay is a picture of an unknown run whether it was read from
a file or drained from a pipe.

```graph
node: analysis-reading-drains-within-a-turn
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-reading-drains-within-a-turn
```

**A position's field is read from the record on the same drain, one position at a
time and never the record.** The record already carries what else had mass at a
generated position: under the field election a `model.field` event stands at every
position the generation retained, per `weaver-trace-Spec` section 3, carrying the
position as the resident length at the draw, the ranked candidates with their
probabilities, and the rank the draw landed on. So the `field <record> --position
<turn>:<position> [--run <run>]` verb, added 2026-09-04 for the trace surface's click
on a spike, adds no reading of its own. Its reader drains a file or a stream as the
signals reader does, keeps the one `model.field` event whose turn and position match
the address, splices the ranked list as the record spelled it rather than parsing and
re-rendering a probability, and drops every other event as it lands, so twenty
thousand positions cost the read one position. **The drawn token is the candidate at
the realized rank** where that rank is within the list, and where the draw fell past
the reported depth it is the entry of the generation's `model.measurement`
`output_tokens` at the field's ordinal within that generation, the fields pairing with
the drawn tokens one for one in landing order because the stop token is neither
retained nor ranked, and where neither answers it is absent rather than invented. One
line answers per run holding the position, each naming its run, because turn keys
repeat across the runs of a serving record, and `--run` narrows the read to one run
and ends it when that run has answered. **It gates on no certified close**, unlike the
lens: the field is the record's own fact about a position and not a reading taken
over a replay, so a serving record and a diagnostic record answer alike, the
diagnostic bracket's close ending the read as it ends every reader's. Three refusals,
each typed as the others are. A record holding no `model.field` event at all refuses
as the field not having been elected, or as elected and unproduced where the `load`
carried a depth. A position the record does not hold at that turn refuses naming the
address. An address that is not `<turn>:<position>` refuses naming what was given.

**The per-position signals are read from the record on the same drain, as a
series, and this reader needs no tap, no lens, and no weights.** Every generation's
`model.measurement` carries the tokens it drew, the distribution's entropy at each
position unconditionally, the drawn token's surprisal where that election stands,
each paired position for position with the drawn tokens, and a `perplexity` for the
generation wherever a distribution was read, per `weaver-spu-Spec` section 6. So the
series a reader wants, where the model was uncertain and where the token it drew
surprised it, is in every record, serving and diagnostic alike, and this reader
pairs and emits it and derives nothing. It is the class's second reader, standing
since 2026-09-02 with the drain: it rides the road the lens rides, holds one turn at
a time, and shares nothing with the lens but the road. **A point is addressed by its
ordinal within its generation**, zero-based, which is what a series is drawn
against, **and not by the position the field read above addresses**, the resident
length at the draw. The record carries both coordinates for one drawn token, each
reader emits the one it reads by, and the conversion between them belongs to a
reader of the record, which holds the residency fact it needs, and never to a
consumer of the series, which does not, per section 7's open election. **Absent
stays absent.** A missing
vector is not invented and a vector shorter than the tokens is not stretched, so a
generation without the surprisal election carries an entropy at every point and a
surprisal at none, and a consumer that fills an absence with a zero is lying about
the election. **A spike is a rule and not a threshold**: the positions whose
surprisal exceeds the series' mean by `k` of its deviations, the caller naming `k`,
empty where fewer than two positions carry a surprisal, and which figure a view may
draw from is a contract's clause rather than this crate's. **It gates only the
record that has a gate.** A serving record carries no bracket and none is owed, it
being an account of what happened rather than a claim that something was
reproduced, and a diagnostic record's series is produced only where its own bracket
closed certified, per the gate above and `weaver-diagnostic-PRD` section 4, the
close ending the read as it ends every reader's. **The summary carries, per
generation, what a store keyed by position converts from**, as of 2026-09-05 per
issue #461: the turn, the perplexity where the record holds one, the resident count
as the generation closed as `model.output` reported it, and the count of output
tokens, and this reader reports the two counts and derives nothing from them. The
counts are the record's own facts, the closing count including the terminator per
`weaver-types-Spec` section 4.4, with `weaver-spu-Spec` section 4 for why, and the
input identifiers being the turn's delta per that Spec's section 6, so the derivation
is the consumer's at ingest, subtracting the drawn
tokens and the terminator from the closing count, and a reader that reported the
previous closing count plus the delta in the resident's place would be wrong on
every first generation by the identity prefix. A generation whose `model.output`
the record does not hold carries no resident count, absent rather than derived.
**The summary carries the record's identity of the artifact too**, as of 2026-09-06
per issue #465 and `weaver-analysis-web-contract` section 3: the weights hash the
generation's measurement carries, per `weaver-spu-Spec` section 3, spelled as the
record spelled it, so the sentinel crosses as the empty string it is, a hash the SPU
could not compute being a fact of the record and not an absence, and the member is
absent only where the measurement carries none. This reader reports it and derives
nothing, and it is the source the run row of `weaver-web-Spec` section 2.2 fills
from. The
`signals <record> [<k>]` verb, `k` two deviations where none is named, renders the
summary first, one object naming the position count, how many carry an entropy and
how many a surprisal, and the generations by turn with those members, then one line
per point carrying turn, ordinal, token, entropy, and surprisal, and the spikes with
their bar on standard error. A record holding no measured generation refuses, typed
as the others are.

```graph
node: analysis-summary-reports-residency
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-summary-reports-residency

node: analysis-summary-reports-the-record-identity
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-summary-reports-the-record-identity
```

```graph
node: analysis-signals-keep-absence
kind: assertion
tag: perturbation

edge: asserts
from: weaver-analysis
to: analysis-signals-keep-absence
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
- The summary reports the residency: a generation's resident count is the one
  `model.output` carried, watched to fail when the reader derives it from the
  previous closing count and the delta, which the first generation refutes.
- The summary reports the record's identity as spelled: a measurement carrying the
  sentinel crosses as the empty string and one carrying no member crosses absent,
  watched to fail when the reader folds the sentinel into absence.
- Absence stays absent in the series: a record without the surprisal election
  yields points carrying an entropy and no surprisal, watched to fail when the
  reader fills an absent vector with zero.

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
section 6. Twenty-three sit there and none sits here, retaken from the records on
2026-09-05, the count having read fourteen while acts since 2026-09-01 added six
without moving it, and the two acts of 2026-09-05 adding the last two.

**Which invariant each claim serves.** One carries a `grounds` edge.
`axiom-floor-is-vocabulary-behavior-is-socket` is why this crate links no internal
crate: its whole vocabulary crosses a socket as drawn names rather than as shared
types, which is that invariant read from outside the agent, where a linked
dependency would have made a consumer a compile-time dependent of the interior.
The other four axioms reach none of these claims. **Twenty-two claims grounding in no
invariant is the expected result and not a gap**, per Document Format section 4:
most of this document is representation.

## 7. Open elections

- **The instrument suite**, per the charter's section 4: what this crate carries
  beyond the certification is named there as a sketch that does not exist in this
  tree, and nothing here is built against it.
- **The capture artifact, closed 2026-09-01**: a certified diagnostic record
  kept whole, the charter's section 3 carrying identity, custody, shape, and
  quota, and section 3 here adding that no second representation exists.
- **The lens artifacts, closed 2026-09-01**: representation in section 3 -
  safetensors matrices beside a JSON manifest, refused before read where the
  identity disagrees - the criteria the charter's clauses of the same date.
- **What the reading is, as an artifact, narrowed 2026-09-01.** Section 5
  settles the reading's content - the layer trajectory, the control that
  gates it, and the capture comparison - and leaves its rendered form to
  the suite's act, that being a presentation question rather than the
  gate's.
- **The satellite types.** The parse's newtypes over the record's identity strings,
  the election's spelling here, and the byte-stream reader's shape. Identifier
  choices with no cross-crate consequence, listed so what this Spec leaves to a
  builder is complete rather than implied.
- **The licence boundary** is the operator's, per the charter, and this document
  takes no position beyond noting that nothing here carries cut-and-recompute.
- **What the series carries so a store keyed by position can address it, closed
  2026-09-05.** Section 5's summary carries the resident count as the generation
  closed and the output count, both reported from the record, and the consumer
  derives the position once at ingest. The other shape, the record carrying the
  count as the generation opened, was not taken: the closing count and the drawn
  tokens already determine it once the terminator's width is stated, which
  `weaver-spu-Spec` section 4 now does, and the opening count would be a new
  member on a new event where the closing count has been on the wire since
  2026-08-19, so the cheaper answer is also the one that asks the record for
  nothing. Per issue #461.
