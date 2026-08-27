# weaver-diagnostic - Spec

**Status:** MERGED. Cut 2026-08-27, the first Spec of the diagnostic leg. Code is
written against it under the gates of Working Process section 6.

**Date filed:** 2026-08-27
**Document ID:** `weaver-diagnostic-Spec`
**Parent:** `weaver-diagnostic-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The representation of the mechanism `weaver-diagnostic-PRD` charters: the record a
replay makes, the shape of the events that compose it, and the surface the harness
authors through. **It is written against that charter and against
`weaver-harness-diagnostic-contract`**, and it develops no rationale of its own:
where reasoning appears it traces to a clause of one of those two, per G2.

**Three elections the charter left, and this document is where they land.** How much
of the serving vocabulary this writer reuses, where the residual readout sits beside
it, and whether the seam mirrors `weaver-trace`'s surface or takes its own. The
charter's section 6 names all three as owed here and calls the first two the larger
half of the work.

**A fourth lands here that was named from outside.** How a diagnostic-trace says it
ended and how it says what happened, owed to this document by
`weaver-analysis-PRD` section 4, which names the outcomes a marker has to separate
and states that the crate paying for its absence is not the crate that can supply
it.

## 1. The crate

**Layout.** One module per obligation, re-exported at the root.

    src/lib.rs        re-exports, and nothing else
    src/event.rs      the envelope, the kind set, the payload shapes, section 3
    src/recorder.rs   the receive, the submit, and the write, section 5
    src/failure.rs    the failure vocabulary, section 6

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
used.

**The dependency set is two crates and one internal one.** `serde` with `derive`,
`serde_json` with `raw_value` for splicing the harness's pre-rendered message
payloads without re-encoding them, and `weaver-traits` for the message model the
replayed contributions carry, which the contract's vocabulary clause draws.

**This crate does not link `weaver-trace`, and the negative is an election rather
than an accident.** The two records share a form and not a type: the canonical form
of `weaver-trace-Spec` section 2 is this record's by the charter's own naming, and
following a rule is not linking its author. Linking it would buy nothing this crate
needs, that crate's `Kind` being exhaustive and closed against the kinds a replay
authors, and would cost the counterpart relation the charter rests on, a mechanism
that depended on its twin being a submodule of it in all but name.

```graph
node: diagnostic-no-trace-dependency
kind: assertion
tag: manifest

edge: asserts
from: weaver-diagnostic
to: diagnostic-no-trace-dependency
```

**No async runtime and no socket crate in the resolved tree**, by the build-time
`cargo tree` assertion the floor Specs share. This crate crosses no process line and
holds a handle rather than a name, so a socket crate would be weight against a
capability it must not have.

```graph
node: diagnostic-no-runtime-no-socket-crate
kind: assertion
tag: manifest

edge: asserts
from: weaver-diagnostic
to: diagnostic-no-runtime-no-socket-crate

edge: grounds
from: diagnostic-no-runtime-no-socket-crate
to: axiom-floor-is-vocabulary-behavior-is-socket
```

## 2. Canonical form

**One rule, and it is not this document's.** Every event renders to exactly one line
of UTF-8 JSON with no interior newline, the newline terminating it as the record
separator, and every integer that can exceed the double-safe range renders as a
decimal string. That is `weaver-trace-Spec` section 2, named authoritative for this
record by `weaver-diagnostic-PRD` section 6 under G5, and a divergence here is a
defect against it rather than a second rule.

**What this document adds is the obligation to follow it in a crate that cannot
import it.** The rule crosses as prose, so the two renderings can drift where a
shared type could not, and the instrument is the one place that drift is visible: a
perturbation test renders an event of a kind both records carry and compares the
line against the same event's line from a serving record, byte for byte, watched to
fail when either renderer's field order or number spelling moves.

```graph
node: diagnostic-canonical-form-follows-trace
kind: assertion
tag: perturbation

edge: asserts
from: weaver-diagnostic
to: diagnostic-canonical-form-follows-trace
```

## 3. The event

### 3.1 The envelope

The envelope is the serving envelope's, field for field: `session`, `run`, `turn`,
`kind`, `sequence`, `subsystem`, `causal_parent`, and both timestamps, flattened
into the event so `kind` sits at the top level of every line where a consumer keys
on it. **The turn and the causal parent are optional and nothing else is**, per
`weaver-trace-Spec` section 3, and a replay fills neither to satisfy a shape: an
event belonging to no turn carries none, and an event the pass cannot attribute to
a cause carries no parent. The shapes
are `weaver-trace-Spec` section 3's and a divergence is a defect against it, per G5.

**The session is the diagnostic run's own and never the replayed one**, per the
contract's section 3. A derived record that wore its source's name would answer the
first question a reader asks of a file, which of the two it is, with the wrong
answer, and the charter's section 6 argues that identity at length. What names the
replayed session is the opening event's payload, where a reader looks for provenance
rather than inferring it from an envelope.

```graph
node: diagnostic-session-is-the-replays-own
kind: assertion
tag: perturbation

edge: asserts
from: weaver-diagnostic
to: diagnostic-session-is-the-replays-own
```

### 3.2 The kind set

**Sixteen kinds, exhaustive, and the set is this crate's own.**

    replay.opened          the pass's bracket opens, and the record identifies itself
    replay.identity        the input identity the pass established
    replay.closed          the pass's bracket closes, carrying its outcome
    turn.started           a replayed turn opens
    turn.closed            a replayed turn closes
    message.system         the seated prefix, as the record carried it
    message.user           a replayed contribution
    message.assistant      a replayed contribution
    message.tool_result    a replayed contribution
    model.request          what was fed to the forward pass
    model.output           what the forward pass produced
    model.measurement      the reading, the residual readout riding it
    model.field            the elected field, where the pass elected one
    flush                  a cut the loop drove, as the record carries one
    refusal                a typed refusal answering an ask the pass sent
    fault                  a death, named

**Thirteen spellings are the serving vocabulary's and mean there what they mean
here.** A kind that names the same fact carries the same spelling and the same
payload shape, which is what makes reader compatibility a rule rather than a
coincidence, per section 4. **Three are this record's own**, the `replay.` trio,
and no serving record carries any of them.

**`flush` is carried because the loop is granted the flush by name.**
`diagnostic-replay-loop` section 1 enumerates what the seat grants it, the state
port, the decode surface, and the flush, and `weaver-harness-Spec` section 6 has the
harness author a `flush` event on the flush's confirmation. A record that could not
carry it would drop an act its own loop is chartered to perform, which is the
accumulation reading broken exactly where the serving record protects it.

**`refusal` is carried because the same section authors every typed refusal, and a
replay produces two.** `weaver-harness-Spec` section 6's ruling of 2026-08-22 has a
refusal answering an ask this harness sent become a `refusal` event naming the ask
and carrying the seam's own case. A replay sends asks that can be refused: **the
flush it was just granted**, whose `keep` that section says nothing else records,
and **the decode ask mid-replay**, which is `diagnostic-replay-loop` section 4's
third named failure and which that document requires the record to carry, the
partial account being the point of authoring one. Routing either to `fault` would
collapse a refusal into a death, which is the collapse this section refuses twice
elsewhere.

**The eight serving kinds this set does not carry are absent by construction rather
than by omission.** A replay runs no Gate, calls no tool, and asks no classifier, so
`tool.call.started`, `tool.call.completed`, and the classify pair have nothing to
author them, and a variant standing for a case nothing produces is the reserved slot
apex section 9 forbids. `load`, `unload`, and `session.closed` are
absent for a different reason: they bracket a serving load and this record's bracket
is the pass, which the `replay.` trio carries. **`elision` is absent on the
narrower ground the flush's presence leaves standing**: the loop's grant names the
flush and not the elision, and a later act that grants one adds the kind with its
own argument rather than finding a variant seated for it.

```graph
node: diagnostic-kind-set-exhaustive
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-diagnostic
to: diagnostic-kind-set-exhaustive
```

### 3.3 The payload shapes

**The mapping is total: sixteen kinds, and every kind's accepting shape is
named.** Thirteen take the serving payload of the same name, spliced or shaped as
`weaver-trace-Spec` section 3 shapes it, that document being authoritative and a
divergence a defect against it. Three are declared here.

    pub struct ReplayOpened {
        pub reader_elected: bool,
    }

    pub struct ReplayIdentity {
        pub replayed_session: String,
        pub model: ModelId,
        pub weights_hash: WeightsHash,
        pub template: TemplateId,
    }

    pub struct ReplayClosed {
        pub outcome: ReplayOutcome,
    }

    pub enum ReplayOutcome {
        Certified,
        Diverged { divergence: Divergence },
        Abandoned { reason: AbandonReason },
    }

    pub enum Divergence {
        TokenPath {
            position: u64,
            recorded: TokenId,
            recomputed: TokenId,
        },
        Readout { position: u64, layer: u32 },
    }

**`ReplayOpened` carries only what the load declared, and the provenance rides its
own kind.** The pass's bracket opens when the run opens, which is before the replay
ask has been answered, so what the harness holds at that moment is the pass's own
election and nothing about the record it is about to read: the declaration does not
name the replayed record, the driver's invocation does, per `weaver-analysis-PRD`
section 4's placement of the path. **`replay.identity` carries the input identity
once step one of `diagnostic-replay-loop` section 3 has established it**, which is
the first moment the replayed session, the model, its weights hash, and the
template are known from the answered holdings. **Established means read and
checked, not merely read**: a step one that read the holdings and refused them has
established nothing, so the pass authors no identity event and closes
`Abandoned`, which is the distinction this kind exists to keep.

**What `reader_elected` separates is the null replay from the pass beside it**, per
that loop's section 3 step 3, each pass running as its own run under its own
reference, so the flag rather than the reference is what a reader keys on.

```graph
node: diagnostic-identity-absent-not-invented
kind: assertion
tag: perturbation

edge: asserts
from: weaver-diagnostic
to: diagnostic-identity-absent-not-invented
```

**Splitting them is what lets the record carry the loop's first named failure.**
That loop's section 4 opens with the replay answer being absent, where the run
"ends its run having replayed nothing and the account says so in the
diagnostic-trace, which is how the operator learns it." A bracket whose opening
event required the provenance could not open at all in that case, so either no
bracket would exist and the operator would learn nothing, or one would open
carrying members filled from nothing. **A bracket with no `replay.identity` is a
pass that never established one**, which is the same absence-means-something
discipline the outcome below runs on.

**The monotonic clock originates at `replay.opened`.** A serving record originates
it at `load`, per `weaver-harness-trace-contract` section 3, and this set carries no
`load`, so the pass's own opening event is the origin and every reading in the
bracket is relative to it.

**`ReplayOutcome` is the terminal marker, and it names three of four outcomes.**
`weaver-analysis-PRD` section 4 names the four a reader must separate: certified and
ended, failed its comparison and ended, ended without finishing, and not ended. The
first three are facts the pass knows and authors. **The fourth is the absence of a
`replay.closed` event and is therefore free**, which is that section's own
observation read forward: a reader that finds a bracket opened and never closed
holds a pass that did not end, and no event has to say so.

**`Divergence` separates the two comparisons certification performs**, per
`weaver-diagnostic-PRD` section 4: the token path matches exactly or it does not,
which is integers, and the reader's vectors compare within the GPU float tolerance
the apex names. A single shape typed to the token path would have had a vector
divergence routed to `Abandoned`, which collapses the second outcome into the third
exactly as manufacturing a close would collapse the fourth. Each carries the first
divergent position, per the loop's section 3 step 2, and the token path's carries
both identifiers so a reader can say how the two differ without rerunning anything.

**No outcome is authored for a pass that died.** The contract's section 5 forbids
manufacturing one, and the reason is the fourth outcome: a closing event written at
a death would collapse unended into abandoned and tell a reader a pass ended when it
stopped. **`Abandoned` is for a pass that reached its own end without certifying**,
whatever stopped it, the loop's likeliest being a refusal at input identity before
any forward pass ran. What `AbandonReason` enumerates is section 8's.

```graph
node: diagnostic-outcome-absent-not-manufactured
kind: assertion
tag: perturbation

edge: asserts
from: weaver-diagnostic
to: diagnostic-outcome-absent-not-manufactured
```

### 3.4 Where the readout sits

**On the measurement, exactly where a serving record puts it.** The residual readout
rides `model.measurement` as `weaver-trace-Spec` section 3 shapes it, with the layer
count and the forward count beside the figures as the act of 2026-08-24 landed them.

**What differs between the two records is density and not shape.** On a serving load
the readout is an election the operator pays for and may decline, and on a
diagnostic pass it is the point of the run, per the charter's section 6. That is a
fact about how often the member is populated and never about what the member is, and
a representation that forked on it would make one reading of one quantity wear two
shapes for no reason a reader could act on.

**Two consequences follow, and both are why this election is the right one.** An
instrument that reads a serving record's measurement reads this one's without
learning a second shape, which is section 4's compatibility rule doing real work
rather than being asserted. And the raw-over-fold ruling this leg already carries is
honored rather than restated: nothing here widens a reduction or freezes a fitting
into a capture, because nothing here reshapes what the SPU rendered.

```graph
node: diagnostic-readout-rides-the-measurement
kind: assertion
tag: review

edge: asserts
from: weaver-diagnostic
to: diagnostic-readout-rides-the-measurement
```

## 4. What a reader may assume

**The charter declines to claim reader compatibility and assigns the claim here.**
Section 6 of it states that whether an instrument reading a serving record reads
this one follows from this document's elections, so this section states the rule
rather than leaving a reader to test it.

**An instrument that keys on `kind` and skips what it does not know reads both
records, and reads every shared kind identically.** That holds because of three
elections above and no others: the line is the same line, section 2, the envelope is
the same envelope, section 3.1, and a shared spelling means a shared shape, section
3.3. Nothing else is promised.

**What tells the two records apart is the opening kind, and it is in the file.**
A diagnostic-trace opens every bracket with `replay.opened`, which no serving record
carries, and a serving record opens with `load`, which this one does not. The
charter's section 6 argues that a reader's first question of a file in hand is which
of the two it holds and that a shape answering it nowhere would be a defect, so the
answer sits in the first event of every bracket rather than in a member a reader has
to hunt for.

```graph
node: diagnostic-record-identifies-itself-at-the-open
kind: assertion
tag: perturbation

edge: asserts
from: weaver-diagnostic
to: diagnostic-record-identifies-itself-at-the-open
```

**The versionless-schema rule binds a reader of this record exactly as it binds a
reader of a serving one**, per `weaver-trace-PRD` section 6: the schema extends and
does not change, a reader skips a kind and a payload member it does not know, and a
record written before a member existed omits it rather than carrying a default.
`weaver-analysis-PRD` section 4 states what that costs the reader, and this document
adds only that this record is on the same terms.

## 5. The recorder

**The surface mirrors `weaver-trace`'s receive and submit, and shares no type with
it.** That is the charter's open election settled toward the mirror, and the harness
Spec's section 9 item settles with it.

    pub struct Recorder { /* private */ }

    impl Recorder {
        pub fn receive(
            sink: OwnedFd,
            run: RunRef,
            session: SessionRef,
        ) -> Result<Recorder, Failure>;

        pub fn submit(&mut self, event: Event) -> Result<Sequence, Failure>;
    }

**Mirroring is what lets one call site serve both mechanisms**, which is the
election's whole argument: the harness holds the two recorders under one shape and
the binding's kind selects the arm, per `weaver-agents-PRD` section 6, so the
authorship path does not fork per site. A surface of its own would have made every
authoring site in the harness ask which record it was writing.

**Sharing no type is what keeps the mirror from becoming a dependency.** A trait
would need a home, and neither crate may hold it: `weaver-trace` links nothing at
all, and a floor trait for two writers that never meet on a wire would put behavior
in the vocabulary layer the floor invariant keeps out of it. So the shape is shared
and the types are not, which is the same forced duplication the election types
already carry across this corpus, named rather than resolved.

```graph
node: diagnostic-surface-mirrors-the-recorder
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-diagnostic
to: diagnostic-surface-mirrors-the-recorder
```

**No path-taking constructor.** The receive takes a descriptor and this crate holds
no name at any point, per the contract's section 6 and `weaver-harness-PRD`
section 5's handle discipline. The instrument is the compile-fail set the sibling
writer already runs: constructing a recorder from a `&str`, a `String`, or a
`PathBuf` each fails to compile.

```graph
node: diagnostic-no-path-taking-constructor
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-diagnostic
to: diagnostic-no-path-taking-constructor
```

**Admission precedes the write, and the sequence is gapless over the run's whole
admitted traffic.** A refused submission touches the sink at no point and consumes
no sequence, per the contract's section 2, so a gap in a record is a lost write and
never a refusal.

```graph
node: diagnostic-admission-precedes-the-write
kind: assertion
tag: perturbation

edge: asserts
from: weaver-diagnostic
to: diagnostic-admission-precedes-the-write
```

**No working structure, and the absence is the contract's clause represented.** This
crate holds no RAM copy of the record it writes, per the contract's section 2: a
replay's present is the holdings `weaver-state` serves through the replay ask, so a
second copy here would hold by one road what the loop already holds by another. The
instrument is the compile-fail absence of any accessor: no method on `Recorder`
yields a held event.

```graph
node: diagnostic-holds-no-working-structure
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-diagnostic
to: diagnostic-holds-no-working-structure
```

## 6. The failure vocabulary

Exhaustive, so a new case reaches every caller.

    pub enum Failure {
        SubmitRefused { refusal: SubmitRefusal },
        WriteFailed { error: WriteError },
    }

    pub enum SubmitRefusal {
        UnknownKind,
        PayloadMalformed,
        PayloadKindMismatch,
        RequiredFieldAbsent { field: FieldName },
    }

**A refusal has no effect on the sink and a write failure is terminal for the
record**, per the contract's section 5. What the harness does with either is its
own, and this crate reports rather than decides, which is the no-policy half of the
charter's section 1.

```graph
node: diagnostic-failure-enum-exhaustive
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-diagnostic
to: diagnostic-failure-enum-exhaustive
```

## 7. What is enforced, and by which instrument

**Enforced by the compiler.** The kind enum is exhaustive, so a kind added beyond
the set section 3.2 declares breaks every consumer's match. The count is not
restated here, a second copy of it being one more place for it to go stale. The
failure enum is exhaustive, so a new case reaches every caller. The receive shape
is read by a doctest, so an argument added to the one constructor stops the build
loudly.

**Enforced by compile-fail tests, because the property is an absence.** No
path-taking constructor, three named shapes. No accessor yielding a held event.

**Enforced by the manifest.** No `weaver-trace` dependency, read against the graph
under gate H2. No async runtime and no socket crate in the resolved tree.

**Requiring a perturbation-verified test.**

- Canonical form follows `weaver-trace-Spec` section 2: a shared kind's line
  compared byte for byte against the same event's serving line, watched to fail
  when either renderer's field order or number spelling moves.
- The session is the replay's own: a pass over a record of session `alpha` renders
  its envelope under the diagnostic run's session and names `alpha` in the identity
  payload alone, watched to fail when the two are crossed.
- The record identifies itself at the open: every bracket's first event is
  `replay.opened`, watched to fail when a pass authors any other kind first.
- No identity is invented: a pass whose replay answer never arrived authors no
  `replay.identity`, and neither does one whose step one read the holdings and
  refused them, watched to fail when either yields an event filled from defaults.
  The second is the case that bites, an absent answer having nothing to build from
  while a refused reading has everything and must still author nothing.
- No outcome is manufactured: a pass ended without its closing event leaves an
  unclosed bracket, watched to fail when a death path authors a `replay.closed`.
- Admission precedes the write: a refused submission leaves the sink untouched and
  consumes no sequence, watched to fail when the refusal is moved after the write.

**Enforced by review, one claim.** That the readout rides the measurement where a
serving record puts it is a claim about where a shape sits rather than a behavior a
run can falsify, and no instrument here reaches it: a test could confirm this
writer's placement and could not confirm it still matches the other's. The
byte-comparison above is the nearest thing and watches the line rather than the
member. **Review means the instrument was not bought and never that none exists**,
and what would buy it is a shared fixture the two writers render, which wants the
sibling crate's participation and is not this document's to elect.

**Where the records sit.** The assertion records are at the clauses that argue the
claims, across sections 1 through 6, rather than gathered here, per Document Format
section 6. Fourteen sit there and none sits here.

**Which invariant each claim serves.** One carries a `grounds` edge.
`axiom-floor-is-vocabulary-behavior-is-socket` is why this crate's manifest holds no
socket crate: a seam that crosses no process line has no socket to hold, and a
mechanism that acquired one would be claiming a boundary it does not have. The other
four axioms reach none of these claims. **Thirteen claims grounding in no invariant
is the expected result and not a gap**, per Document Format section 4: most of this
document is representation, and representation is what the invariants are not
about.

## 8. Open elections

- **The diagnostic-trace's own instrument set**, beyond what a certification reads.
  `weaver-analysis-PRD` section 4 names the suite as a sketch that does not exist in
  this tree, and nothing here is built against it.
- **Whether a replay that elides gains `elision`.** Section 3.2 carries `flush`,
  the loop's grant naming it, and excludes `elision` because that same grant does
  not. A loop later granted the elision port reopens the question with its own
  argument rather than finding the variant waiting, and the refusal that would
  answer such an ask is already carried.
- **The satellite types.** `Sequence`, `Subsystem`'s spelling here, `FieldName`,
  `WriteError`, `AbandonReason`'s case set, and `RunRef`, `SessionRef`, and
  `TurnRef` as this crate's own newtypes over owned strings, on the same
  no-dependency ground the sibling writer states. Identifier choices with no
  cross-crate consequence, listed so what this Spec leaves to a builder is complete
  rather than implied.
- **The null replay** of the charter's section 4 is what certifies this mechanism,
  and it is owed behind this document rather than asserted in it.
