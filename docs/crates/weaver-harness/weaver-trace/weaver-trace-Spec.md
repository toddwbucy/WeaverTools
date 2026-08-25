# weaver-trace - Spec

**Status:** MERGED. Cut 2026-08-01, third of the Spec pass and the last of the floor's
build order. Code is written against it under the gates of Working Process section 6.

**Date filed:** 2026-08-01
**Revised:** 2026-08-24, the record states its compatibility. Section 3
represents charter section 6's guarantee as an absence, the envelope carrying
no version member, and states what growth may do and may not do without an apex
act first: a variant added beside the ones standing is extension, and a rename,
a retirement, a member's type changed, or a member's meaning changed is the
breaking change. The instrument is review rather than perturbation because no
run of this crate can fail on it, the damage landing on a reader of an older
record, which is stated rather than left as a gap in the enforcement table.
**Revised:** 2026-08-22, fourth of this date, the classify outcome loses its
refusal. `ClassifyOutcome` becomes `ClassifyScored`, a struct rather than an
enum, its `Refused` variant having moved to the `refusal` kind per the
charter's clause of this date and a single-variant enum being a reserved
slot. The counts are unchanged, one disposition retiring as another arrives.
**Revised:** 2026-08-22, third of this date, pressure leaves the failure
vocabulary. `Failure` loses `CommitPressure`, which `submit` returned after the
event was already in the working structure and the writer's queue, so a caller
reading that `Err` was told the opposite of what had happened. The depth is read
from the recorder instead, a caller ignoring it being correct by default. What the
harness does with it is unchanged and was never implemented: it authors a `fault`
naming `RecorderCommitPressure`, per the fault-carrier ruling this clause already
carried.
**Revised:** 2026-08-22, second of this date, the refusal takes its kind.
Section 3's `Kind` gains `Refusal` with its explicit rename and `Payload`
gains `Refusal`, spliced on the custody rule that carries an organ's
account opaque, the counts moving to twenty-one kinds and fifteen
dispositions. `StopReason` gains `Refused`. Two counts that had gone stale
ride with the recount, the variant sentence reading eighteen and the
compiler note naming a seventeenth kind.
**Revised:** 2026-08-22, second of this date, the elision records where it
fell. `Elision` carries `ElisionSpan`, `from` and `to` beside the resident
counts, rather than the `FlushCounts` the first draft of this act gave it.
A flush is described by the count it leaves and an interior removal is not,
so the counts alone could not support the replay charter section 3.2
promises. The disposition count is unchanged at fourteen.
**Revised:** 2026-08-22, the elision takes its kind. Section 3's `Kind`
gains `Elision` with its explicit rename, the counts moving to twenty, and
the kind-to-payload mapping gains `Elision` carrying `FlushCounts`, the
same two facts under a different kind rather than a second struct with
identical members.
**Revised:** 2026-08-21, third of this date, the elections become three.
Section 3's `Elections` gains `surprisal`, a plain boolean serialized even
when false, per the charter's same-act edit on issue #258: absent, false,
and true are three states, the first being a record older than the
election. The field's `Option` is unchanged.
**Revised:** 2026-08-21, second of this date, the prefix becomes a
recorded contribution and the mapping is recounted. Per the charter's
same-act edit on issue #258. `message.system` gains a second author, the
seated identity prefix reaching the record at the run's opening with no
turn on the classify precedent, so the accumulation rule's base sits inside the
record rather than in the configuration. `message.system` becomes
turn-optional, the first kind whose turn is sometimes rather than always
present, the other three message kinds unmoved. Section 3's `Payload` and
its kind-to-payload paragraph are recounted to nineteen kinds and thirteen
dispositions: `ModelField`, `Elections`, and `Flush` stood in the crate and
in neither, added by acts that recounted the kind set and not this mapping.
**Revised:** 2026-08-21, the field enters the record and the load names
its elections. Section 3's `Kind` gains `ModelField` with its explicit
rename, the counts moving to nineteen, and the kind-to-payload mapping
gains `ModelField` with `Candidate` beside it. The `load` event's payload
becomes `Elections`, each diagnostic election named individually and none
bundled, per the charter's same-act edit.
**Revised:** 2026-08-20, the record holds the context position. Section
3's `ModelOutput` gains `resident` and `capacity`, plain integers meeting
the same shaping test its emission and finish meet, on the charter's
same-act edit. The election stated beside them is that they land in this
payload rather than in the measurement's splice, which the harness may not
open without retiring the splice rule. One perturbation assertion lands.
**Revised:** 2026-08-19, fourth of this date, the classify kinds take
shape. `Kind` gains `ClassifyRequest` and `ClassifyOutput` with their
explicit renames, the counts move to eighteen, and the kind-to-payload
mapping gains two dispositions shaped here on the flush's precedent:
`ClassifyAsk`, the content sent, and `ClassifyScored`, the labels the
artifact's head returned, per the charter's same-act edit.
**Revised:** 2026-08-19, third of this date, the flush reaches the record.
`Kind` gains `Flush` with its explicit rename, the counts move to sixteen
and the ordinals to seventeenth, and the kind-to-payload mapping gains the
ninth disposition: `flush` carries `FlushCounts`, the resident tokens
before and after, per the charter's same-act edit.
**Revised:** 2026-08-19, second of this date, the finish tells the truth.
`Finish` gains `Length`, mirroring the floor's same-act addition per the
two-names-one-fact arrangement of section 1: the turn's token limit
reached, which the record had flattened into `Completed`.
**Revised:** 2026-08-19, the system role lands. `Kind` gains
`MessageSystem` with its explicit rename, the counts move to fifteen
throughout, the message kinds become four, and the kind-to-payload mapping
carries it under `Message` with its siblings, per the charter's same-act
edit.
**Revised:** 2026-08-17, the trace stamps the engine where the organ is too
coarse, per issue #103. `Subsystem` gains `spu_decoder` for the three model
events, whose wiring landed stamping `spu` before the case existed. `Spu`
stays for residency, admit, release, and fault attribution, which are the
organ's. The growth rule re-keys from crates to producing parties at the
granularity a reader needs, which is the generalization `Tool` already was,
and `spu_encoder` is named as deliberately absent until an encoder gives it
an emitter, per apex section 9.
**Revised:** 2026-08-12, the request is the turn's contribution, per the
operator's ruling of this date. Section 4's splice sentence follows
`weaver-trace-PRD` section 3.2 as narrowed: the turn's delta as rendered,
not the prompt as the model received it, splicing whole as before.
**Revised:** 2026-08-11, the model events splice. `Payload::ModelRequest` and
`Payload::ModelMeasurement` become spliced `RawValue`, the organ producing their
content and the harness carrying it opaque per the custody model, reversing the
earlier reading that shaped the measurement here. `ModelOutput` stays shaped, its
emission a string the harness consumes and its finish a two-case enum, neither an
opaque blob. The typed `ModelRequest` and `ModelMeasurement` structs retire from
the schema, and `trace-measurement-absent-not-zero` retires with them, the
absent-not-zero property relocating to the SPU's `spu-absent-not-empty-vector`
where the rendering now happens. The crate becomes 38 assertions.
**Revised:** 2026-08-14, the run identifies itself. `RunOrdinal` becomes
`RunRef`, so the envelope's triple reads `SessionRef`, `RunRef`, and
`TurnRef`, and the newtype clause drops its small integers because all three
are now strings.
**Document ID:** `weaver-trace-Spec`
**Parent:** `weaver-trace-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-trace`: the module layout, the item signatures, the
canonical byte rule, the in-memory representation of the working structure, the
writer's shape, and the elections a builder would otherwise invent. It is derived
from `weaver-trace-PRD` and from `weaver-harness-trace-contract`, which is the one
contract this crate is party to.

Level discipline. The charter says what the crate needs and why. This document says
how it is represented, and per gate G2 as ruled on 2026-08-01 it elects against
grounds the charter states rather than developing grounds of its own. Where this
document and the charter disagree the charter yields nothing.

**This document declares its crate's assertion records, less two claims stated
elsewhere,** per Document Format sections 3 and 4 as of the notation of 2026-08-03,
which retired the no-records sentence this paragraph replaces. It sources no other
record. The charter stays the source of this crate's node, its parent edge, and its
six vocabulary definitions, and a Spec that restated them would give the mapper two
sources for one record, per Document Format section 1. `asserts` runs from the crate
rather than from this document, so the document still needs no node of its own.
**Two claims this Spec states are declared elsewhere and carry no record here.** The
close-on-exec test of section 10 is owed to `weaver-harness-Spec` and discharged at
that document's section 8, an assertion belonging where its test lives. The tagging
test section 3 applies to `TurnClose` is the one `weaver-types-Spec` section 4.3
declares, node and both edges, as the test the two floor Specs share so they cannot
drift, and this crate applying it is not a third party to it. Declaring either here
would be the duplicate the format forbids, and dropping either silently would leave
part of this crate's set with nothing recording where it went.

**It is written from the merged corpus alone.** The old tree carried four Specs for
this crate, including one for the durability primitive, and the ruling of
2026-08-01 keeps them out of the Spec pass. The charter's staged item asking how
much of that implementation's weight survives is therefore answered from this
crate's own obligations and from measurement, per section 11, rather than by
reading what a different program built against different obligations.

## 1. The crate

**Layout.** One module per obligation, re-exported at the root.

    src/lib.rs        re-exports, and nothing else
    src/event.rs      the envelope, the kind set, the payload shapes, section 3
    src/canonical.rs  the byte rule, section 2
    src/structure.rs  the working structure, section 4
    src/writer.rs     the stream writer and the commit boundary, section 6
    src/failure.rs    the failure vocabulary, section 9

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
used.

**The dependency set is two crates and no internal one.** `serde` with `derive`,
and `serde_json` with `derive`'s companion feature **`raw_value`**, which section 3
elects for splicing the harness's pre-rendered message payloads without
re-encoding them. The feature is named here rather than discovered at build time,
the same way the floor Specs name `derive`. **No `weaver-*` dependency at
all**, which is the charter's section 1 claim read as a manifest property: this
crate depends on nothing internal, and gate H2 reads that against a graph in which
it declares no `floor-link` and one `seam` tagged `link`.

```graph
node: trace-serde-json-raw-value-feature
kind: assertion
tag: manifest

edge: asserts
from: weaver-trace
to: trace-serde-json-raw-value-feature

node: trace-no-internal-dependency
kind: assertion
tag: manifest

edge: asserts
from: weaver-trace
to: trace-no-internal-dependency
```

**The identity fields are opaque newtypes here, and the harness converts, which is
the election the no-dependency rule forces.** The envelope identifies a session, a
run, and a turn, and `weaver-types` owns `SessionId` and `TurnKey` for the wire.
Linking that crate to reuse them would give this crate an internal dependency the
charter forbids and would make the floor's wire vocabulary a dependency of the
recorder, which records rather than speaks. So this crate defines
`SessionRef`, `RunRef`, and `TurnRef` as newtypes over owned strings, carries
them without interpreting them, and the harness, which links both
crates, converts at the submit call. The cost is that one concept has two type
names in two crates. The alternative costs the charter's central structural claim,
and the conversion is a total function at one call site rather than a judgment
spread across the crate.

```graph
node: trace-identity-newtypes-harness-converts
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-identity-newtypes-harness-converts
```

## 2. Canonical form

One rule, used everywhere, per charter section 4.2.

**An event renders to exactly one line of UTF-8 JSON with no interior newline, and
the newline that terminates it is the record separator.** This is what makes the
stream NDJSON without a framing layer above it. Verified against serde_json 1.x:
`to_string` emits no raw newline for any value, escaping an embedded newline in a
string to `\n`, so a payload carrying prose cannot split one event into two lines.

```graph
node: trace-one-line-per-event
kind: assertion
tag: perturbation

edge: asserts
from: weaver-trace
to: trace-one-line-per-event
```

**Every integer that can exceed the double-safe range serializes as a decimal
string.** That is the monotonic reading in nanoseconds and the sequence, and it is
elected because a consumer parsing JSON numbers as doubles gets a silently
different value with no error and no way back. Verified: `9007199254740993`
rendered bare and read as a double returns `9007199254740992`, and the same value
rendered as `"9007199254740993"` round-trips exactly. The wall-clock stamp in
milliseconds stays a bare number, being far below the range where the loss begins.

```graph
node: trace-large-integers-as-decimal-strings
kind: assertion
tag: perturbation

edge: asserts
from: weaver-trace
to: trace-large-integers-as-decimal-strings
```

**Field order is declaration order and the renderer is deterministic.** The same
event renders to the same bytes on every run of every build, because the working
structure and the stream hold that one rendering and a consumer comparing two
copies of one event is comparing bytes. Serde's derive emits struct fields in
declaration order, so this is a property of not reordering fields rather than of
sorting them at render time. **No instrument reaches byte-identity across builds,
so the property is review's,** a suite running inside one build being able to
confirm that one rendering reaches two holders and not that two builds agree.

```graph
node: trace-render-deterministic
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-render-deterministic
```

## 3. The event

```rust
pub struct Event {
    #[serde(flatten)]
    pub envelope: Envelope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

pub struct Envelope {
    pub session: SessionRef,
    pub run: RunRef,
    pub turn: Option<TurnRef>,
    pub sequence: Sequence,
    pub kind: Kind,
    pub subsystem: Subsystem,
    pub causal_parent: Option<Sequence>,
    pub wall_ms: u64,
    pub monotonic_ns: MonotonicNs,
}

#[serde(rename_all = "snake_case")]
pub enum Subsystem { Admin, Harness, Spu, SpuDecoder, Gate, Tool }

pub enum Kind {
    #[serde(rename = "load")]                 Load,
    #[serde(rename = "unload")]               Unload,
    #[serde(rename = "session.closed")]       SessionClosed,
    #[serde(rename = "turn.started")]         TurnStarted,
    #[serde(rename = "turn.closed")]          TurnClosed,
    #[serde(rename = "flush")]                Flush,
    #[serde(rename = "elision")]              Elision,
    #[serde(rename = "refusal")]              Refusal,
    #[serde(rename = "message.system")]       MessageSystem,
    #[serde(rename = "message.user")]         MessageUser,
    #[serde(rename = "message.assistant")]    MessageAssistant,
    #[serde(rename = "message.tool_result")]  MessageToolResult,
    #[serde(rename = "tool.call.started")]    ToolCallStarted,
    #[serde(rename = "tool.call.completed")]  ToolCallCompleted,
    #[serde(rename = "fault")]                Fault,
    #[serde(rename = "model.request")]        ModelRequest,
    #[serde(rename = "model.output")]         ModelOutput,
    #[serde(rename = "model.field")]          ModelField,
    #[serde(rename = "model.measurement")]    ModelMeasurement,
    #[serde(rename = "classify.request")]     ClassifyRequest,
    #[serde(rename = "classify.output")]      ClassifyOutput,
}

#[serde(untagged)]
pub enum Payload {
    Message(Box<serde_json::value::RawValue>),
    TurnClosed(TurnClose),
    Fault(Box<serde_json::value::RawValue>),
    ModelRequest(Box<serde_json::value::RawValue>),
    ModelOutput(ModelOutput),
    ModelField(ModelField),
    Elections(Elections),
    Flush(FlushCounts),
    Elision(ElisionSpan),
    Refusal(Box<serde_json::value::RawValue>),
    ModelMeasurement(Box<serde_json::value::RawValue>),
    ClassifyRequest(ClassifyAsk),
    ClassifyOutput(ClassifyScored),
    Deferred(Box<serde_json::value::RawValue>),
}

pub struct ModelOutput {
    pub emission: String,
    pub finish: Finish,
    pub resident: u64,
    pub capacity: u64,
}

pub struct ModelField {
    pub position: u64,
    pub ranked: Vec<Candidate>,
    pub realized: u32,
}

pub struct Candidate {
    pub token: u32,
    pub probability: f32,
}

pub struct Elections {
    pub residual_readout: bool,
    pub field: Option<u32>,
    pub surprisal: bool,
}

pub struct ElisionSpan {
    pub from: u64,
    pub to: u64,
    pub resident_before: u64,
    pub resident_after: u64,
}

pub struct FlushCounts {
    pub resident_before: u64,
    pub resident_after: u64,
}

pub struct ClassifyAsk {
    pub content: String,
}

pub struct ClassifyScored {
    pub labels: Vec<(String, f64)>,
}

#[serde(tag = "close", rename_all = "snake_case")]
pub enum TurnClose {
    Clean,
    Stopped { reason: StopReason },
}
```

**Every kind carries an explicit rename, because no scheme produces the charter's
names.** Charter section 3.1 spells them with dots, and verified against serde 1.x
the derive default emits `"MessageUser"` and `rename_all = "snake_case"` emits
`"message_user"`, while only a per-variant rename emits `"message.user"`. Leaving
it to a scheme would put a second spelling of every kind on the wire, which is the
one-name-two-nodes defect the Document Format rules against for identifiers and
which reads the same way for a consumer keying on a kind. The mapping is total:
twenty-one variants, twenty-one renames, and the wire spelling is the charter's.

```graph
node: trace-kind-explicit-renames
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-kind-explicit-renames
```

**The envelope carries no version member, and the vocabulary grows by addition
alone.** Charter section 6 rules why: while the schema only extends, every
vintage of the record is the one schema met at a different moment and a reader
needs nothing to key on, so a version member would be a field whose only reader
is unbuilt. This section represents that as an absence in `Envelope` above,
which is the whole of it on the writing side. Growth is a variant added with
its own rename beside the ones standing, and the count above moves in the act
that adds it. What this Spec may never do without an apex act first is rename a
variant, retire one, change a payload member's type, or change what a member
already there means, each of those being the breaking change apex section 3
names rather than the extension section 9 permits.

**The instrument is review rather than perturbation, and the reason is worth
stating.** No run of this crate can fail because a later act reshaped a kind,
the damage landing on a reader of an older record rather than on the writer, so
there is no behaviour here for a test to perturb. What holds the line is the
gate at each act, which is why the rule is written where an author of the next
act will meet it rather than only in the charter.

```graph
node: trace-no-version-member
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-no-version-member
```

**`Subsystem` names the producing party at the granularity a reader needs, and
takes the plain snake-case scheme.** **The case set is this Spec's election and
the charter fixes only that the field exists**, so the grounds are stated here
rather than assumed. Four cases are the crates that can produce a report,
`weaver-admin`, `weaver-harness`, `weaver-spu`, and `weaver-gate`. `Tool` is a
producing party and not a crate, because a tool's result reaches the record
through the harness and a record that attributed it to the harness would lose
the one fact an operator reading a tool result wants first. **`SpuDecoder` is
that argument one level down, per the ruling gathered on issue #103**: the
SPU's chartered domain is every semantic operation in the text modality, so as
engines accumulate behind the one organ, an event stamped `spu` loses the fact
a reader wants first, which engine produced it. The three model events carry
it. **`Spu` stays beside it**, because residency, admit, and release belong to
the organ rather than to any engine inside it, and a fault report is the
organ's for the same reason. The rule an earlier wording keyed on crates is
retired: **a case arrives per producing party at the granularity a reader
needs, with its first emitter, as a floor edit in the same act.** The encoder's
case, `spu_encoder`, is deliberately absent until the act that builds an
encoder gives it an emitter, a case with no producer being a reserved slot in
enum form, per apex section 9.

```graph
node: trace-subsystem-case-set
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-subsystem-case-set
```

**Eighteen kinds, exhaustive, matching charter section 3.1 exactly.** The enum is
exhaustive rather than `#[non_exhaustive]` because the set is closed by ruling and
adding one is an edit to the charter and to every contract naming the set: an
attribute that let a consumer absorb a further kind into a wildcard would defeat
the closure the corpus keys on.

```graph
node: trace-kind-enum-exhaustive
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-trace
to: trace-kind-enum-exhaustive

edge: grounds
from: trace-kind-enum-exhaustive
to: axiom-contract-is-a-complete-interface
```

**`turn` is optional and `causal_parent` is optional, and nothing else is.** A
run-level event belongs to no turn, which is what the option expresses, and the
recorder never infers one, per the contract's section 3. A malformed submission
carrying no turn on a turn-level kind is refused rather than defaulted.

**The never-inferred half is what the join-key invariant asks of a recorder.** A
turn the recorder supplied would be a key that never travelled with the work, and
an event carrying one would be attributed by the recorder's guess rather than by
the key the harness held. The option is therefore the absence of a turn and never
a turn the recorder has yet to work out.

```graph
node: trace-turn-optional-never-inferred
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-turn-optional-never-inferred

edge: grounds
from: trace-turn-optional-never-inferred
to: axiom-join-key-travels-with-the-work
```

**The message payloads splice rather than nest, and the mechanism is elected
rather than assumed.** This crate neither defines the message model nor decodes
it, per charter section 3, so the harness supplies those three payloads
pre-rendered. Verified against serde_json 1.x: holding them as a `String` and
rendering the enclosing event escapes them into a JSON string,
`"payload":"{\"role\":\"user\"...}"`, which every consumer must then unescape
and which is a second representation arriving by the back door.
`serde_json::value::RawValue` splices the bytes as they stand,
`"payload":{"role":"user",...}`, and it is what `Payload::Message` holds.

```graph
node: trace-message-payloads-splice
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-message-payloads-splice
```

**That election sharpens why admission precedes the fan-out.** `RawValue` splices
whatever bytes it holds, so a malformed rendering from the harness would become a
corrupt line rather than a refused submission. The octet well-formedness check of
the admission step is the only thing standing between the two, which is a
concrete reason for an ordering the charter states abstractly. `RawValue`
construction validates, so the check has a mechanism rather than a promise.

**The envelope flattens into the event and the payload does not, which is an
election and takes an argument.** An event renders as one flat object,
`{"session":...,"run":0,"kind":"load","subsystem":"harness"}`, rather than nesting
the envelope under a member of its own. The reason is the reader: this line is what
an operator's tooling consumes, and every such consumer keys on `kind` first, so a
nesting level between the line and its kind is a level every consumer pays on every
event. Verified against serde 1.x that the flatten path preserves what section 2
claims, one line with no interior newline, declaration order with `payload` last,
and byte-identical output across renders, since flatten serializes through a map
and the determinism claim had to survive that.

```graph
node: trace-envelope-flattens
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-envelope-flattens
```

**Nothing collides because no `Envelope` field is named `payload`, and that becomes
a constraint on every later envelope edit.** Flattening puts the envelope's members
and the payload member in one object, so a field added to `Envelope` later and
named `payload` would produce two members of one name. The constraint lives here
rather than nowhere: **no field added to `Envelope` may be named `payload`.**

```graph
node: trace-no-envelope-field-named-payload
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-no-envelope-field-named-payload
```

**The sibling Spec's envelope elects the opposite layout, and the difference is
principled rather than accidental.** `weaver-types-Spec` section 4.1 states that
nothing is flattened in `OrganEnvelope`, because its payload is adjacently tagged
and carries `kind` and `body` members of its own, so flattening would put two
layers' keys in one object and widen the collision surface the adjacent tagging was
elected to close. This crate's payload is untagged and contributes exactly one
member, so the same flatten costs nothing and buys the reader a level. Two
envelopes, two exposures, two elections, and each says why.

**`Payload` is untagged and the envelope's `kind` is its discriminant, which is a
fourth case the floor Specs' shared test does not cover.** That test elects a
tagging scheme from a variant's shape, and it has no case for an enum that may not
be tagged at all. This one may not: a tag would wrap the spliced message bytes in
an object of this crate's making, which is the double encoding the `RawValue`
election exists to avoid. So the scheme is untagged, the `kind` already present in
the envelope selects the variant, and **admission is what enforces the pairing**,
since serde no longer can. A submission whose kind and payload shape disagree is
refused at step one of section 5 rather than rendered, which is another thing the
ordering buys.

```graph
node: trace-payload-untagged-kind-discriminant
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-payload-untagged-kind-discriminant
```

**A bracket kind carries no payload member at all, rather than a null one.** The
run and session brackets and the turn's opening are identified entirely by their
envelope, so `payload` is `Option<Payload>` and those kinds carry `None` with
`skip_serializing_if`, emitting `{"kind":"load"}`. Verified against serde_json
1.x: a unit variant inside an untagged enum renders `"payload":null` instead,
which is a member whose only content is the statement that there is no content,
and a consumer keying on member presence would see two stream shapes for one
absence.

```graph
node: trace-bracket-kind-omits-payload
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-bracket-kind-omits-payload
```

**`TurnClosed` carries the close kind, internally tagged under the shared test.**
`Clean` is fieldless and `Stopped` is struct-shaped, so the enum falls in the
test's second case and renders `{"close":"clean"}` and `{"close":"stopped",
"reason":...}`, one shape for both closes. Verified that the default emits a bare
string for the first and an object for the second, which is two shapes for one
field. This is the one payload the merged corpus fixes today, per
`weaver-harness-trace-contract` section 3 and charter section 3.1. **The test
itself is not this crate's claim,** its node and both its edges living at
`weaver-types-Spec` section 4.3 where the two floor Specs share it so they cannot
drift, and what this clause asserts is the election the test yields here.

```graph
node: trace-turn-close-internally-tagged
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-turn-close-internally-tagged
```

**The kind-to-payload mapping is total, twenty-one kinds and fifteen
dispositions**, the payload-free case counting as one of them. `refusal`
carries `Refusal`, spliced, the organ's own account of what it turned away. `unload`,
`session.closed`, and `turn.started` carry `None`.
`load` carries `Elections`. The four message kinds carry `Message`.
`turn.closed` carries `TurnClosed`. `fault` carries `Fault`. `flush` and
`elision` each carry `FlushCounts`, the resident token counts before and
after, both plain integers, two kinds over one shape because the two
operations report the same two facts. The four model kinds carry their four
own shapes, one each. The
classify pair carries its two own shapes, `ClassifyAsk` and
`ClassifyScored`. **A refused classify authors no output at all** and
reaches the record under `refusal`, so a refusal the exchange met is still
the record's fact and never a fabricated answer, carried by the kind the
class gives it rather than by the outcome's own variant. The tool bracket's
two carry `Deferred`. Three plus one plus four plus one plus one plus two
plus four plus two plus two plus one is twenty-one, which is the whole of
charter section 3.1's set.

**The count is stated because it has twice been wrong, and the second time
it was wrong silently.** An earlier draft assigned thirteen and left
`turn.started` homeless. Then the acts of 2026-08-19 and 2026-08-21 added
`flush`, the classify pair, `model.field`, and the load's elections to the
kind set and to the code, and this paragraph and the enum above it were
recounted in neither, so both stood at eighteen kinds and eleven dispositions
while the crate compiled thirteen. **A mapping that is stale reads exactly
like a mapping that is total**, which is why the recount lands as prose here
and as a member list above rather than as a claim about totality alone.

```graph
node: trace-kind-payload-mapping-total
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-kind-payload-mapping-total
```

**What is spliced and what is shaped, and the custody model of 2026-08-11
draws the line.** The trace owns the boxes and the organ owns the contents, so
a payload whose content an organ produced splices, and a payload this crate
consumes and shapes is the exception. `Fault` splices, `fault-report` being the
floor's shape the reporting organ renders. The classify pair shapes on the
flush's precedent: plain small data the harness authors from typed wire
answers, no organ-rendered box standing to splice. **`model.request` and
`model.measurement` splice on the same rule, reversing an earlier reading of
this section.** The earlier text shaped the measurement here because its
readings are what no other crate defines, but the streaming ruling made the SPU
the definer and the producer of those readings, so under the custody model they
are the organ's content and the harness splices them exactly as it splices a
fault. The request the model was asked this turn, the turn's delta as rendered
with its template and its effective sampling per the ruling of 2026-08-12, is
the family library's rendering and the SPU's, so
it splices whole rather than being assembled here from members this crate would
have to parse the splice to read. **`model.output` is the one model payload
this crate shapes,** because its emission is a plain string the harness holds
typed and consumes into the working structure, and its finish is a two-case
enum the turn's close reads, so neither is an opaque blob and shaping them
forces no transform. The rule is the charter's section 2.5 read through the
custody model: this crate defines the record's own schema, the envelope and the
kinds and the finish, and carries every organ-produced content opaque.

**The session's two counts join it 2026-08-20 and meet the same test.** The
resident count and the capacity as the generation closed are plain integers
the harness already holds typed, reading them at every close to answer the
seat's fullness port, so shaping them forces no transform either and this
crate's own `FlushCounts` is the standing precedent for a count in its
schema. The charter's grounds are its accumulation ruling: a generation
moves the resident context as a flush does, and the pair is what makes the
accumulation checkable rather than merely reconstructible. **They land in
this payload rather than in the measurement's splice**, which is the
election worth stating, because the measurement is the SPU's rendering
carried opaque and a harness that opened it to insert two members would be
editing an organ's content, retiring the splice rule in the same breath
that invoked it. The output is where the harness's own reading of the
generation belongs, beside the finish it converts at its one site.

```graph
node: trace-splice-or-shape
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-splice-or-shape
```

**`model.field` is shaped here and not spliced, on the same test the
output meets.** Its members are a decode position, a vector of pairs, and an index,
all of them plain values the harness holds typed on their way through, so
shaping forces no transform and the splice rule's subject does not arise.
It is the record's first per-position kind, and the granularity is the
charter's rather than this document's election.

**The `load` event carries an `Elections` payload**, per the charter's
same-act edit, naming each diagnostic election of the load individually.
A boolean for the readout and the field's depth where it stands, absent
where it does not. **The shape refuses a profile name deliberately**:
a set named once drifts as members join it, and every record already
carrying that name becomes a record of something else without any event
saying so. Naming each election is what keeps a record's posture
recoverable from the record.

**`elision` carries its coordinates and `flush` does not need to.** An
earlier draft of this section gave the elision `FlushCounts` on the reading
that the two operations report the same two facts. They do not. **A flush is
fully described by the count it leaves**, its outcome being a prefix, so
`resident_after` names the surviving sequence exactly. An elision removes an
interior, and two elisions with identical counts can remove different
positions and leave different sequences, so the counts alone say how much
went and never which.

**Without the span the record cannot support the replay this crate promises.**
Charter section 3.2 has a consumer reconstruct a context by replaying the
edits, and an edit that does not say where it fell is not replayable. So
`ElisionSpan` carries `from` and `to` beside the two counts, and the counts
stay because they are the SPU's own account of the outcome against the
harness's account of the ask, which is a disagreement worth being able to
see.

**`ClassifyOutcome` becomes `ClassifyScored` and stops being an enum.** It
held two variants and one of them, the free-form `Refused { refusal: String
}`, moved to the `refusal` kind on 2026-08-22. **What is left is one shape
and takes a struct**, because a single-variant enum kept against a second
that may come is a reserved slot for a reader that does not exist, which
this corpus refuses. A second outcome arriving later brings its own shape in
the act that adds it.

**The kind-to-payload mapping is unchanged in its counts.** One disposition
retires and one arrives in the same act, `ClassifyOutcome` for
`ClassifyScored`, so twenty-one kinds and fifteen dispositions still hold. A
refused classify authors no `classify.output` at all.

**The crate carries `ClassifyOutcome` until the act that migrates it**,
which follows this one: seven references stand in `weaver-trace` and
`weaver-harness`, and `engine.rs` still authors a refusal as
`Kind::ClassifyOutput` with the string inside. **Read this section as what is
authorized and the crate as what is built**, per gate H1's direction, the
window between a merged Spec and the code answering to it being the ordinary
one.

**`refusal` splices where `elision` and the flush shape**, which is the
custody line rather than an inconsistency: a refusal is the refusing party's
account and this crate carries an organ's account opaque, exactly as it
carries `fault`. The counts on an elision are this crate's to shape because
the harness holds them as plain values on the way through. A refusal's case
set is four seam vocabularies wide and grows with the seams, and declaring
it here would make this crate depend on what it must not depend on and
version what it does not own.

**`StopReason` gains `Refused`**, per the charter's same-act edit. It is a
satellite by section 11 and its variants are not enumerated here, but the
addition is the charter's rather than a naming choice: a close that cannot
say a refusal ended the turn says something else instead.

**The identity prefix's events carry no turn, and `message.system`
becomes turn-optional to admit them.** A prefix is seated at open and
precedes every turn of the run, so the turn member is absent rather than
filled with the turn that happens to come first, on the precedent the
classify pair set for an exchange belonging to no turn. **The kind moves
sides in the recorder's turn rule**, which held every message kind to a
turn and refused a turnless one as a malformed submission. The rule above
is unchanged in what it means: the turn is present exactly when the event
belongs to one, and `message.system` is the first kind for which that is
sometimes and not always. The other three message kinds do not move, a
user, assistant, or tool-result message with no turn being malformed as it
ever was. **The door is a second one rather than the message
door widened.** `author_message` takes a turn key by value and the licensing
rule it applies is the turn's, so widening it to an optional turn would put
a turnless path inside the door every turn message goes through. The prefix
gets its own door, stated in `weaver-harness-Spec` section 6 in this act,
and the record shows the same kind from both.

**Nothing re-reads the prefix into a later prompt.** The assembled structure
sweeps the three turn-bearing message kinds, `message.user`,
`message.assistant`, and `message.tool_result`, and takes the identity from
the declaration, so `message.system` entering the record adds no message to
any assembly. That exclusion predates this act and is what makes recording
the prefix a record change rather than a behavior change, which is the
property an elected diagnostic is held to and the one this act holds itself
to as well.

**The surprisal's election is a plain boolean and is present rather than
skipped when false**, which is the opposite election from the field's
`Option` beside it. The field's absence means no field was asked for and
its presence carries the depth, so `None` is the whole of what an absent
election has to say. The surprisal's `false` is a fact worth writing: it
distinguishes a record whose operator declined the vector from a record
written before the election existed, where the member is absent altogether.
**Absent, false, and true are three states and the shape keeps them three.**

**The counts reaching the record is a perturbation, not a review read.** A
member that serializes is easy to add and easy to lose, and losing it is
silent: the record still renders, every consumer still parses, and only an
analysis run months later finds the field absent where it needed it. The
test renders a `model.output` payload and reads both counts back off the
canonical form, watched to fail with either member dropped from the shape.

```graph
node: trace-output-carries-the-counts
kind: assertion
tag: perturbation

edge: asserts
from: weaver-trace
to: trace-output-carries-the-counts
```

**`model.request`'s splice holds the sampling values and `model.measurement`'s
does not, which is the charter's corrected row rather than this Spec's choice.**
Apex section 8's five re-feed inputs therefore span the pair, four in the
measurement and the fifth in the request, joined by the turn both carry, per
charter section 3.1, and each rides inside its own spliced content rather than
a member this crate names.

**The measurement's optional members are absent rather than zero, and the
absence is now the SPU's to produce rather than this crate's to serialize.**
The measurement splices, so this crate names none of its members and elects no
`skip_serializing_if` over them: the unproduced reading emits no member because
the SPU rendered it absent, per `weaver-spu-Spec` section 6's
`spu-absent-not-empty-vector`, and the trace carries what the SPU rendered
without a serde election of its own. The property the retired
`trace-measurement-absent-not-zero` asserted is the SPU's now, a relocation the
custody model forces rather than a property dropped.

**Untagged with seven variants is a serialization device, and the deserializing
consumer keys on `kind`.** Serde resolves an untagged enum by trying variants in
order, which is unambiguous while writing and ambiguous while reading once more
than one variant is struct-shaped. This crate never reads an event back: the
working structure holds rendered lines, per section 4, and no resume path
survived the cut of 2026-08-01. A consumer that decodes does so kind-first,
which the flatten election of this section already put at the top level of every
line for exactly this reason, so the discriminant a reader needs is available
before the payload is reached. **`Payload` therefore derives `Serialize` and not
`Deserialize`,** which makes the asymmetry a compile property rather than a
convention, and admission's kind-to-shape check remains what enforces the
pairing on the writing side. **The instrument is a compile-fail doctest,** the
missing half being an absence and an absence being what a runtime test
structurally cannot demonstrate, the same reading `weaver-types-Spec` section 3
gives the missing `Deserialize` on `PeerIdentity`.

```graph
node: trace-payload-serialize-only
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-trace
to: trace-payload-serialize-only
```

**`Deferred` holds the payloads whose shapes their own workflows settle**, which
since the trace act of 2026-08-02 is the tool bracket's two alone, the fault and
the three model kinds having landed at charter section 3.2. It is listed in
section 11 with what settles it. The variant holds raw bytes in the interim
rather than a
placeholder struct, because a struct shaped against no chartered content would be
the reserved slot apex section 9 forbids, and because a kind whose payload has no
chartered shape cannot be submitted with a shape this crate invented.

**What admission may judge is bounded by that split**, per the contract's refusal
case: the envelope binds, the required fields are present, the kind is known, and
the payload is well-formed as octets. The interior of a message is the harness's,
and a recorder that parsed one would have taken a judgment the charter denies it.

## 4. The working structure

**An append-only log of rendered lines, with an index over the envelope fields a
reader selects on.** Per the ruling of 2026-08-01 the structure holds the same
canonical NDJSON the stream carries, so the stored value is the rendered line
itself and not a second representation of it.

```graph
node: trace-structure-holds-rendered-lines
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-structure-holds-rendered-lines
```

```rust
pub struct WorkingStructure { /* private */ }

impl WorkingStructure {
    pub fn new(run: RunRef) -> Self
    pub fn len(&self) -> usize
    pub fn iter(&self) -> impl Iterator<Item = &Record>
    pub fn by_kind(&self, kind: Kind) -> impl Iterator<Item = &Record>
    pub fn in_turn(&self, turn: &TurnRef) -> impl Iterator<Item = &Record>
}

pub struct Record {
    pub sequence: Sequence,
    pub kind: Kind,
    pub turn: Option<TurnRef>,
    pub line: Arc<str>,
}
```

**Append-only is structural and the type system is where it lives.** There is no
`remove`, no `truncate`, no `get_mut`, no `IndexMut`, and no method returning `&mut
Record`. The append itself is not public: only this crate's emit path of section 5
can extend the structure, so a caller holding a `&WorkingStructure` has no
expressible way to alter what landed. Charter section 2.2 rests this on the agent
running as its own uid with bash, and a mutation surface reachable from that uid is
a surface the agent could reach, so the guarantee has to be architectural rather
than a discipline. **Two instruments hold this and they hold different halves.**
Every public accessor yields a shared reference and the append is crate-private,
which a compiling test pins over the signatures, and that is what makes alteration
after landing unrepresentable rather than merely absent. **The named methods are
pinned by the compile-fail doctests of section 10,** a finite set reaching
`remove`, `truncate`, and `get_mut` and not the open set of all the mutators
someone could write. The two are two records for that reason, per section 10.

```graph
node: trace-structure-no-mutation-surface
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-trace
to: trace-structure-no-mutation-surface

node: trace-structure-mutators-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-trace
to: trace-structure-mutators-pinned-by-doctest
```

**The index is the envelope fields and never the payload.** `sequence`, `kind`, and
`turn` are lifted out of the line so a reader selects without parsing, which is what
keeps the harness's per-turn reads from scanning the whole run. Nothing else is
lifted, because every further field would be a second copy of data the line already
holds.

```graph
node: trace-index-envelope-fields-only
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-index-envelope-fields-only
```

**Reads are sequential and the cost is stated rather than hidden.** The harness
assembles a prompt by taking the message-kind records in sequence order and
decoding their payloads, which is one JSON parse per message per assembly. That is a
real cost and it grows with the conversation. It is elected over caching decoded
messages because a decoded cache is the second representation the working-structure
ruling retired, and the parse is small against a decode that pays network-class
latency. **Entry condition to reopen: a measurement showing prompt assembly on the
turn path costing more than a stated fraction of the decode it precedes.**

**`Arc<str>` rather than `String` for the line**, because the same rendering goes to
the structure and to the writer's queue in one act and the two must not be two
copies of one line. This is the mechanical form of one rendering, held and handed.

```graph
node: trace-one-rendering-two-holders
kind: assertion
tag: perturbation

edge: asserts
from: weaver-trace
to: trace-one-rendering-two-holders
```

## 5. The emit path

The four steps of charter section 4.2, in order, always, with admission ahead of
the fan-out.

```rust
pub struct Recorder { /* private */ }

impl Recorder {
    /// The crate's one constructor and its one receive site.
    pub fn receive(sink: OwnedFd, run: RunRef, session: SessionRef)
        -> Result<Self, Failure>

    pub fn submit(&mut self, event: Event) -> Result<Sequence, Failure>
    pub fn structure(&self) -> &WorkingStructure
    pub fn boundary(&self) -> Boundary
    pub fn drain(&mut self) -> Result<(), Failure>
}
```

**`Recorder` is the crate's principal type and owns both sinks.** The emit path
fans out to the working structure and the writer's queue in one act, so one item
holds both, and that item is this one. Nothing else in the crate can append to the
structure or enqueue to the writer, which is what makes the fan-out's atomicity a
property of the type rather than of a caller's discipline.

1. **Admit or refuse.** The envelope binds, required fields are present for the
   kind, and the payload is well-formed octets. A refusal returns `Failure` and has
   no effect on either sink.
2. **Assign the sequence.** Run-scoped, strictly increasing, gapless over admitted
   events, starting at zero for the run's first admitted event, which is always its
   `load`. The sequence is the order and the monotonic reading is the instrument,
   and the two are never read for each other's job.
3. **Render once,** per section 2, producing the line both sinks receive.
4. **Fan out,** appending to the working structure and handing the same `Arc<str>`
   to the writer's queue in the same act, then returning the assigned sequence to
   the harness.

```graph
node: trace-sequence-gapless
kind: assertion
tag: perturbation

edge: asserts
from: weaver-trace
to: trace-sequence-gapless
```

**The structure lands first and the return is the acknowledgment.** The turn
proceeds on the return, with the stream write still in flight, which is the trade
charter section 4.2 makes deliberately. A submission that cannot append is a failure
before anything is queued, so the two sinks cannot disagree about what was admitted.

```graph
node: trace-admission-precedes-fan-out
kind: assertion
tag: perturbation

edge: asserts
from: weaver-trace
to: trace-admission-precedes-fan-out
```

**`submit` takes `&mut self` and there is one `Recorder`.** It is not shared
across threads and the harness is its one caller, per charter section 1, so
ordering needs no lock and the sequence cannot interleave. A `Send` handle to the
writer's queue is the only part that crosses a thread.

## 6. The writer and the commit boundary

**A bounded queue and one writer, draining to the descriptor the harness was
handed.**

```rust
pub struct Writer { /* private */ }

pub struct Boundary {
    pub committed: Option<Sequence>,
    pub admitted: Option<Sequence>,
    pub queued: usize,
    pub last_error: Option<WriteError>,
}

impl Writer {
    pub fn boundary(&self) -> Boundary
    pub fn drain(&mut self) -> Result<(), Failure>
}
```

**The queue's depth is a construction parameter and not a configuration field.**
Charter section 4.2 makes the loss bound a property of the deployment and gives
this crate no policy field to tune it, so the depth is supplied by the worker's
composition root at construction rather than read from the agent config. That keeps
it a deployment fact without making it an operator election, which is the
distinction the charter draws.

```graph
node: trace-queue-depth-not-config
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-queue-depth-not-config
```

**The three boundary states are derived rather than stored.** Committed is the
highest sequence handed to the sink, admitted is the highest assigned, queued is
the difference, and a failed write is the last error. A reader interrogates the
boundary while the process lives, and nothing about it survives the process, per
charter section 6.

```graph
node: trace-boundary-derived-not-stored
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-boundary-derived-not-stored
```

**Whole events only.** The writer hands the sink one complete line plus its
terminator per write and never a partial line, so a consumer's account truncates at
an event boundary. Where the sink is a file opened append-only, the kernel's append
semantics carry that, and where it is a pipe or a socket the writer's own framing
does, a short write being retried to completion rather than reported as success.

```graph
node: trace-whole-events-only
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-whole-events-only
```

**Pressure is reported to the harness and never authored by this crate.** When the
queue exceeds its high-water mark while the sink remains writable, the recorder
surfaces the queue's depth to its caller. The harness authors a `fault` event in
response, per the fault-carrier ruling of 2026-08-01, which is the only way a
pressure condition reaches the record: this crate authors no event and holds no
event kind, per charter section 2, and a recorder that emitted its own pressure
event would have ended the sole-writer property in the act of reporting on it.

**Pressure is not a failure and stopped being carried as one 2026-08-22.** It was
`Failure::CommitPressure`, returned from `submit` **after** the event was pushed to
the working structure and enqueued to the writer. The event had landed. A caller
reading that `Err` was told its event did not, which is the opposite of what
happened, and every caller that discarded the result treated a recorded event as a
lost one. **The record was right and the caller was wrong about it**, which is a
corruption the diagnostic posture cannot catch: reading the trace finds no lie
because the trace holds none.

**So the depth rides the success path and the failure vocabulary loses the
variant.** A submission that landed answers that it landed. What the depth is at
that moment is a property of the recorder rather than of the submission, so it is
read from the recorder rather than widened into every caller's `Ok`: a caller with
no interest in pressure is correct without saying so, and a caller that wants to
throttle asks. **The mistake becomes unavailable rather than corrected**, which is
what the ten discard sites could not achieve by being patched, the type having
stayed able to produce the same bug at the next port anyone adds.

```graph
node: trace-pressure-reported-not-authored
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-pressure-reported-not-authored

edge: grounds
from: trace-pressure-reported-not-authored
to: axiom-join-key-travels-with-the-work
```

**A write that fails against a live process is named and surfaced, never
swallowed**, per charter section 4.2. A tail lost to process death is the different
failure the charter separates, and this crate reports the first and cannot report
the second.

## 7. Open and drain

**Open takes descriptors and offers no path-taking constructor.** There is no
`open<P: AsRef<Path>>` in this crate, not behind a feature, not for tests. Charter
section 4.1 makes the absence the custody model's API consequence, and the previous
tree's trace-root resolver with zero production callers is the failure it names.
**The three named shapes are pinned by the compile-fail doctests of section 10,**
because an absence is what a runtime test structurally cannot demonstrate, **and
the prohibition itself is review's,** a finite set of doctests reaching `&str`,
`String`, and `PathBuf` and not the open set of every type a path could arrive as.
The two are two records for that reason, per section 10.

```graph
node: trace-no-path-taking-constructor
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-no-path-taking-constructor
```

**The receive site is `Recorder::receive`, declared in section 5, and it takes no
flag argument.** It accepts an `OwnedFd` and nothing that could name a path.
Close-on-exec is supplied by the harness at its own receive, per
`weaver-admin-harness-contract` section 5, and append-only rides the open file
description from admin's open. Neither flag is a type property of this crate, so
neither is pinned here: what the pin reaches is the shape, one constructor
returning a `Recorder` the rest of the crate cannot build another way.
**That shape takes its pin in section 10, and the flag prohibition itself stays
review's,** a call with the three declared arguments compiling being mechanical
where the absence of every argument a flag could arrive as is not. The two are
two records for that reason, per the split this document already makes for the
path shapes, and the pin's record sits with the bullet that argues it.

```graph
node: trace-receive-site-takes-no-flag
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-receive-site-takes-no-flag
```

**Drain empties the queue and returns.** At unload the harness calls `drain` before
answering left, so a left answer means everything admitted reached the sink, per the
coordination contract's ordering. At process death nothing drains and the tail is
forfeited, which this crate cannot report because the process reporting it is the
one that died.

## 8. Instrumentation

**This crate holds no recording level, because none exists.** The ruling of
2026-08-02 retired them: what an operator elects at load governs what the agent
produces, and everything produced is recorded, per `weaver-trace-PRD` section 5.
The harness submits every event it authors and this crate records every event it
admits, so neither party holds a level and neither drops an event for being one.

```graph
node: trace-holds-no-recording-level
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-holds-no-recording-level
```

**No drawn vocabulary crosses this seam into this crate**, which is what keeps the
no-internal-dependency rule of section 1 free of an H4 question.
`weaver-harness-trace-contract`'s `weaver-types` clause is empty: the recorder
receives events and never reads the operator's declaration. An earlier draft of
this Spec reported the contract's owed-policy sentence as a residue needing its own
act, and the ruling dissolved the subject instead.

## 9. The failure vocabulary

```rust
pub enum Failure {
    RefusedOnSubmit { reason: SubmitRefusal },
    CommitFailed { sequence: Sequence, source: WriteError },
    AppendFailed { sequence: Sequence },
    WriteTargetUnusable { source: WriteError },
}

pub enum SubmitRefusal {
    UnknownKind,
    PayloadMalformed,
    RequiredFieldAbsent { field: FieldName },
}
```

Four cases, matching `weaver-harness-trace-contract` section 5 one to one.
**Both counts moved together on 2026-08-22**, when pressure stopped being a
failure and left each enumeration in the same act. Nothing returns a partial
result with a success status, and every refusal names its case rather than
carrying a string, so a caller branches on a value.
`Failure` is exhaustive, so a fifth case reaches every caller at compile time in
the act that adds it.

**The crate carries the retired variant until the act that removes it**, which
follows this one. `Failure::CommitPressure` stands in the built enum and `submit`
still returns it after the event has reached the working structure and the
writer's queue, which is the behaviour this section stopped specifying rather
than the behaviour it describes. **A reader comparing the two should read this
section as what is authorized and the crate as what is built**, per gate H1's
direction: a Spec merges and code answers to it, so the window between them is
the ordinary one and not a divergence to reconcile.

```graph
node: trace-failure-enum-exhaustive
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-trace
to: trace-failure-enum-exhaustive

edge: grounds
from: trace-failure-enum-exhaustive
to: axiom-contract-is-a-complete-interface
```

**`AppendFailed` fails loudly and does not attempt recovery.** The contract gives
the recorder the resolution and there is no record to rebuild from since the cut of
2026-08-01, so the only honest response is to fail rather than continue against a
structure that is missing what the stream holds.

```graph
node: trace-append-failed-no-recovery
kind: assertion
tag: review

edge: asserts
from: weaver-trace
to: trace-append-failed-no-recovery
```

## 10. What is enforced, and by which instrument

**Enforced by the compiler.**

- The kind enum is exhaustive, so a twenty-second kind breaks every consumer's match.
- `WorkingStructure` exposes no mutation surface: every public accessor yields a
  shared reference and the append is crate-private, so alteration after landing is
  unrepresentable rather than merely forbidden. This is the signature half of the
  claim, the named mutators being the compile-fail bullet below, per the split
  section 4 states.
- The failure enum is exhaustive, so a new case reaches every caller.
- The receive shape of section 5 is read by a doctest: a call passing an
  `OwnedFd`, a `RunRef`, and a `SessionRef` and binding a `Recorder`
  compiles, so an argument added to the one constructor stops the build loudly.
  This is the pinned half of section 7's no-flag claim, the prohibition itself
  staying review's per the split stated there, an absence being what a positive
  doctest structurally cannot demonstrate.

**Enforced by compile-fail tests, because the property is an absence.**

- No path-taking write surface: doctests attempting to construct a writer from a
  `&str`, a `String`, and a `PathBuf` each fail to compile. Three named shapes
  rather than a claim about all possible shapes, with the general prohibition
  staying review's, per the split `weaver-traits-Spec` section 7 makes. **The
  split is two assertions rather than one,** the pinned shapes here and the
  prohibition itself at the section 7 clause that argues it: a single record
  tagged for the mechanical half would claim the instrument for the whole, which
  is the overclaim this corpus refuses in prose and has no reason to admit in a
  graph.
- No `remove`, `truncate`, or `get_mut` on the working structure. **These are the
  named half of the mutation-surface claim,** its signature half being the
  compiler bullet above, and the two are two records for the same reason the
  path-taking pair are.
- No `Deserialize` on `Payload`, per section 3, this crate reading no event back.

**Enforced by the manifest.** No `weaver-*` dependency, read against the graph
under gate H2. No async runtime and no socket crate in the resolved tree, by the
build-time `cargo tree` assertion the floor Specs share. The `serde_json`
`raw_value` feature of section 1 is the same instrument read the other way, a
feature the manifest must carry rather than one it must not, and its record sits
at the dependency clause with the rest of that election.

**Where the records sit, and the two claims another document declares.** The
assertion records are at the clauses that argue the claims, across sections 1
through 9, rather than gathered here, per Document Format section 6: this section
sorts by instrument and the arguments are elsewhere, so a block here would sit
apart from the prose that earns it. Thirty-six sit there and three sit at the end of
this section, being the claims argued only here. Seventeen of the thirty-nine come
from this section's own sorting and twenty-two from the elections outside it, the
path-taking prohibition counting here rather than as an election because it was
divided out of this section's own bullet and never elected, per Document Format
section 3.
**The close-on-exec test below is declared by `weaver-harness`,** whose Spec
section 8 carries it as the first of its threat walks and has discharged the
owing, because an assertion belongs where its test lives. **The tagging test
section 3 applies is declared by `weaver-types-Spec` section 4.3,** node and both
edges, as the test the two floor Specs share, and what this document records is
the election that test yields for `TurnClose`. The threat walk closing this
section takes no node of its own, per Document Format section 5, and it names no
test of its own either, its instrument being the compile-fail set already
recorded.

**Which invariant each claim serves, and why most serve none.** Four of the
thirty-nine carry a `grounds` edge, two to `axiom-join-key-travels-with-the-work`
and two to `axiom-contract-is-a-complete-interface`. The other two axioms take
nothing from this crate. `axiom-floor-is-vocabulary-behavior-is-socket` reaches
none of it because this crate is not floor, so the vocabulary clause governs none
of its manifest, and its one seam does not cross a process line, so the socket
clause governs none of its elections. `axiom-organ-and-submodule` reaches none of
it because a submodule's channel with its own organ is that organ's business by
the invariant's own words, which leaves this document nothing to claim under it.
**The test applied is whether the axiom is the reason the claim exists.** Remove
the join-key invariant and this crate has no reason to refuse to infer a turn and
no reason to report pressure rather than author it, so those two ground in it.
Remove it and the kind renames are still dotted, the subsystem set is still six
cases, and the payload is still untagged, so those ground in nothing.
**Thirty-five claims grounding in no invariant is the expected result and not a
gap**, per Document Format section 4: fourteen of the thirty-nine are section 3's
event schema and twelve of those fourteen ground in nothing, a schema being
representation and representation being what the invariants are not about.

**Two calls are worth stating rather than leaving to be read.** The two contract
edges are the closure of drawn vocabulary. `weaver-harness-trace-contract` draws
the event-kind set and the failure vocabulary, and an enum that let a consumer
absorb a new case into a wildcard would leave the drawn set complete in prose and
open in code, which is that invariant's completeness enforced at the type level.
The kind-to-payload mapping's totality reads like the same claim and is not: it is
this document's bookkeeping over its own schema, and no contract is less complete
for it. The second call is section 1's manifest claim. `weaver-types-Spec` grounds
its look-alike no-socket-no-runtime claim in the floor invariant, and the
resemblance does not carry, because that edge runs through the floor-is-vocabulary
clause and this crate is not floor. The apex names this crate's independence where
it classifies it, which is the apex citing a fact rather than supplying the reason
the fact exists.

**Requiring a perturbation-verified test.**

- Canonical form: a monotonic reading beyond the double-safe range round-trips
  exactly, confirmed by watching a consumer parsing it as a double return a
  different number when the decimal-string rule is removed.
- One line per event: an event whose payload carries an embedded newline renders to
  one line, confirmed by watching the stream gain a spurious record when the
  escaping is bypassed.
- Admission precedes the fan-out: a refused submission leaves no record in the
  structure and no line on the stream, confirmed by watching a row appear when the
  refusal is moved after the append.
- One rendering, two holders: the bytes in the structure are the bytes handed to
  the writer, confirmed by comparing them and by watching the comparison fail when
  the writer re-renders from the event.
- Gapless run-scoped sequence under the run's whole traffic, confirmed by watching
  a gap appear when a refused submission consumes a sequence.
- Absent rather than zero: an event whose measurement carries no signal
  vectors emits no member for them, confirmed by watching an empty array
  appear when the skip election is removed, since an empty array and an
  absent instrument are the two facts the charter separates.
- Close-on-exec on the descriptors the worker receives, **owed to
  `weaver-harness-Spec`** rather than run here, the flag being supplied at the
  harness's receive site per `weaver-admin-harness-contract` section 5. The test
  spawns a child and confirms it does not inherit the handle, watched to fail when
  the flag is removed. This crate runs no part of it, and the reason is
  the corpus's own rule rather than a dependency budget: `weaver-admin-harness-
  contract` section 2 makes close-on-exec **a set and not a check**, because a step
  that finds the flag clear and reports rather than repairs leaves the descriptor
  inheritable anyway. A recorder that validated on receive would be that step. An
  earlier draft promised the validation, which would also have required `fcntl`
  through a third dependency this crate's manifest does not carry, so the promise
  and the dependency set contradicted each other before the doctrine settled it.

**This crate's threat walk.** The adversary is the agent reaching its own account
through a tool it elected, and the mechanism is that this crate offers no way to
name the sink: no path-taking constructor, no path stored, nothing to disclose and
nothing to reopen. The perturbation test is the compile-fail set above, and the
attack it defeats is a tool that has read `/proc/self/fd` and wants a second handle,
which fails because there is no call that takes what it learned.

```graph
node: trace-no-async-runtime-no-socket-crate
kind: assertion
tag: manifest

edge: asserts
from: weaver-trace
to: trace-no-async-runtime-no-socket-crate

node: trace-path-shapes-pinned-by-doctest
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-trace
to: trace-path-shapes-pinned-by-doctest

node: trace-receive-shape-pinned-by-doctest
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-trace
to: trace-receive-shape-pinned-by-doctest
```

## 11. Open elections

- **The in-RAM read cost.** Section 4 elects rendered lines plus an envelope index,
  which costs one parse per message per prompt assembly. Reopened by a measurement
  showing that cost material against the decode it precedes, which is the charter's
  staged entry condition read forward.
- **The queue's high-water mark**, the depth at which pressure is reported rather
  than the depth of the queue itself. Settled by a measurement against a real sink
  at a real rate, which is the same measurement the back-pressure election of
  `weaver-admin-operator-contract` section 3 waits on, and the two settle together.
- **The satellite types.** `Sequence`, `MonotonicNs`, `Subsystem`'s Rust
  spelling, `FieldName`, `WriteError`, `StopReason`, `OwnedFd`'s wrapper if
  one is taken, and from the trace act, `TemplateId`, `ModelId`,
  `WeightsHash`, `TokenId`, `Bits`, `Finish`, `PromptBlock`, and
  `DecodeTimings`. The last two carry shape rather than only a name:
  a block names the span of the token sequence it covers, and the timings
  carry what the charter's row enumerates. Identifier and newtype choices with
  no cross-crate consequence,
  listed so what this Spec leaves to a builder is complete rather than implied.
  `Sequence` and `MonotonicNs` carry the decimal-string rendering of section 2,
  which is a constraint on their serde implementation rather than on their Rust
  shape.
- **The deferred payload shape**, singular since the trace act of 2026-08-02.
  The tool bracket's content waits on the tool workflow, which
  `weaver-traits-PRD` section 3.1 holds blocked, and until it lands its two
  kinds cannot be submitted with a shape this crate invented, which is the
  half-chartered discipline read forward. The fault and the three model
  payloads left this entry with that act, shaped in section 3 against charter
  section 3.2.
- **The weight this crate carries at all.** The charter stages the question of how
  much durability machinery survives its obligations. This Spec's answer is the
  five modules of section 1 and nothing further, and the entry condition is the
  Spec pass, which is now, so the item closes here rather than being carried:
  canonical rendering, an append-only structure, a bounded queue with an
  interrogable boundary, and a typed failure set are what the obligations demand,
  and anything beyond them would be weight this program did not ask for.
