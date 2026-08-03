# weaver-trace - Spec

**Status:** DRAFT. Cut 2026-08-01, third of the Spec pass and the last of the
floor's build order. No code is written against it until phase three is ratified,
per Working Process section 6.

**Date filed:** 2026-08-01
**Revised:** 2026-08-02, on the review seat's return. Every kind takes an explicit
rename so the wire carries the charter's dotted names, `Payload` gains its
representation with `RawValue` splicing the message kinds and the deferred shapes
listed with their settlers, `Recorder` appears as the crate's principal type with
the receive site declared, the verbosity section states that this crate holds no
level and reports the contract's residual sentence, the close-on-exec test is owed
to `weaver-harness-Spec`, and the satellite types are listed as open.
**Revised:** 2026-08-02, on the second return: `Payload` is untagged with the
envelope's kind as discriminant and admission enforcing the pairing, a bracket
kind carries no payload member rather
than a null one, `TurnClose` is internally tagged under the shared test, the
kind-to-payload mapping is stated as total, `Subsystem`'s case set names its
grounds, and the flag-validating receive is dropped on the corpus's set-not-check
rule. Revised once more the same day: the envelope's flatten election gains its
argument, its no-collision ground, and the forward constraint on envelope fields.
**Revised:** 2026-08-02. Section 8 restates and retitles against the ruling that
retired the recording levels: no level exists to hold, and the contract residue
this Spec reported dissolves with its subject rather than needing the act section
11 named. On the review seat's return the same day, two earlier changelog entries
were restored to what their own acts said, this document's and
`weaver-harness-PRD`'s, and the entries of this date were moved to the end of
this list, filing order having been broken and history having been edited to
match a later ruling.
**Revised:** 2026-08-02, a fourth entry this date, the token workflow's trace
act. `Payload` gains four variants against charter section 3.2, the fault
splicing the floor's report and the three model kinds taking shapes this crate
defines, with the splice-or-shape line stated as the charter's own rule read
mechanically. The dispositions recount from four to eight, the absent-not-zero
obligation lands as a serde election, `Payload` is stated as serialize-only
with the kind-first consumer as its ground, and section 11's deferred-shapes
entry goes to the singular.
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

**This document declares no graph records,** per Document Format section 1. The
charter is the source of this crate's node, its parent edge, and its six vocabulary
definitions.

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

**The identity fields are opaque newtypes here, and the harness converts, which is
the election the no-dependency rule forces.** The envelope identifies a session, a
run, and a turn, and `weaver-types` owns `SessionId` and `TurnKey` for the wire.
Linking that crate to reuse them would give this crate an internal dependency the
charter forbids and would make the floor's wire vocabulary a dependency of the
recorder, which records rather than speaks. So this crate defines
`SessionRef`, `RunOrdinal`, and `TurnRef` as newtypes over owned strings and small
integers, carries them without interpreting them, and the harness, which links both
crates, converts at the submit call. The cost is that one concept has two type
names in two crates. The alternative costs the charter's central structural claim,
and the conversion is a total function at one call site rather than a judgment
spread across the crate.

## 2. Canonical form

One rule, used everywhere, per charter section 4.2.

**An event renders to exactly one line of UTF-8 JSON with no interior newline, and
the newline that terminates it is the record separator.** This is what makes the
stream NDJSON without a framing layer above it. Verified against serde_json 1.x:
`to_string` emits no raw newline for any value, escaping an embedded newline in a
string to `\n`, so a payload carrying prose cannot split one event into two lines.

**Every integer that can exceed the double-safe range serializes as a decimal
string.** That is the monotonic reading in nanoseconds and the sequence, and it is
elected because a consumer parsing JSON numbers as doubles gets a silently
different value with no error and no way back. Verified: `9007199254740993`
rendered bare and read as a double returns `9007199254740992`, and the same value
rendered as `"9007199254740993"` round-trips exactly. The wall-clock stamp in
milliseconds stays a bare number, being far below the range where the loss begins.

**Field order is declaration order and the renderer is deterministic.** The same
event renders to the same bytes on every run of every build, because the working
structure and the stream hold that one rendering and a consumer comparing two
copies of one event is comparing bytes. Serde's derive emits struct fields in
declaration order, so this is a property of not reordering fields rather than of
sorting them at render time.

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
    pub run: RunOrdinal,
    pub turn: Option<TurnRef>,
    pub sequence: Sequence,
    pub kind: Kind,
    pub subsystem: Subsystem,
    pub causal_parent: Option<Sequence>,
    pub wall_ms: u64,
    pub monotonic_ns: MonotonicNs,
}

#[serde(rename_all = "snake_case")]
pub enum Subsystem { Admin, Harness, Spu, Gate, Tool }

pub enum Kind {
    #[serde(rename = "load")]                 Load,
    #[serde(rename = "unload")]               Unload,
    #[serde(rename = "session.closed")]       SessionClosed,
    #[serde(rename = "turn.started")]         TurnStarted,
    #[serde(rename = "turn.closed")]          TurnClosed,
    #[serde(rename = "message.user")]         MessageUser,
    #[serde(rename = "message.assistant")]    MessageAssistant,
    #[serde(rename = "message.tool_result")]  MessageToolResult,
    #[serde(rename = "tool.call.started")]    ToolCallStarted,
    #[serde(rename = "tool.call.completed")]  ToolCallCompleted,
    #[serde(rename = "fault")]                Fault,
    #[serde(rename = "model.request")]        ModelRequest,
    #[serde(rename = "model.output")]         ModelOutput,
    #[serde(rename = "model.measurement")]    ModelMeasurement,
}

#[serde(untagged)]
pub enum Payload {
    Message(Box<serde_json::value::RawValue>),
    TurnClosed(TurnClose),
    Fault(Box<serde_json::value::RawValue>),
    ModelRequest(ModelRequest),
    ModelOutput(ModelOutput),
    ModelMeasurement(ModelMeasurement),
    Deferred(Box<serde_json::value::RawValue>),
}

pub struct ModelRequest {
    pub rendered: Box<serde_json::value::RawValue>,
    pub template: TemplateId,
    pub sampling: Box<serde_json::value::RawValue>,
}

pub struct ModelOutput {
    pub emission: String,
    pub finish: Finish,
}

pub struct ModelMeasurement {
    pub model: ModelId,
    pub weights_hash: WeightsHash,
    pub input_tokens: Vec<TokenId>,
    pub output_tokens: Vec<TokenId>,
    pub blocks: Vec<PromptBlock>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entropies: Vec<Bits>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub surprisals: Vec<Bits>,
    pub timings: DecodeTimings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reductions: Option<Box<serde_json::value::RawValue>>,
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
fourteen variants, fourteen renames, and the wire spelling is the charter's.

**`Subsystem` names the producing party and takes the plain snake-case scheme**,
its values being single words with no dotted spelling to match. **The case set is
this Spec's election and the charter fixes only that the field exists**, so the
grounds are stated here rather than assumed: the four crates that can produce a
report are `weaver-admin`, `weaver-harness`, `weaver-spu`, and `weaver-gate`, and
`Tool` is the fifth because a tool's result reaches the record through the harness
and a record that attributed it to the harness would lose the one fact an operator
reading a tool result wants first. A sixth case arrives when a crate that can
produce a report is chartered, which is a floor edit in the same act.

**Fourteen kinds, exhaustive, matching charter section 3.1 exactly.** The enum is
exhaustive rather than `#[non_exhaustive]` because the set is closed by ruling and
adding one is an edit to the charter and to every contract naming the set: an
attribute that let a consumer absorb a fifteenth kind into a wildcard would defeat
the closure the corpus keys on.

**`turn` is optional and `causal_parent` is optional, and nothing else is.** A
run-level event belongs to no turn, which is what the option expresses, and the
recorder never infers one, per the contract's section 3. A malformed submission
carrying no turn on a turn-level kind is refused rather than defaulted.

**The message payloads splice rather than nest, and the mechanism is elected
rather than assumed.** This crate neither defines the message model nor decodes
it, per charter section 3, so the harness supplies those three payloads
pre-rendered. Verified against serde_json 1.x: holding them as a `String` and
rendering the enclosing event escapes them into a JSON string,
`"payload":"{\"role\":\"user\"...}"`, which every consumer must then unescape
and which is a second representation arriving by the back door.
`serde_json::value::RawValue` splices the bytes as they stand,
`"payload":{"role":"user",...}`, and it is what `Payload::Message` holds.

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

**Nothing collides because no `Envelope` field is named `payload`, and that becomes
a constraint on every later envelope edit.** Flattening puts the envelope's members
and the payload member in one object, so a field added to `Envelope` later and
named `payload` would produce two members of one name. The constraint lives here
rather than nowhere: **no field added to `Envelope` may be named `payload`.**

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

**A bracket kind carries no payload member at all, rather than a null one.** The
run and session brackets and the turn's opening are identified entirely by their
envelope, so `payload` is `Option<Payload>` and those kinds carry `None` with
`skip_serializing_if`, emitting `{"kind":"load"}`. Verified against serde_json
1.x: a unit variant inside an untagged enum renders `"payload":null` instead,
which is a member whose only content is the statement that there is no content,
and a consumer keying on member presence would see two stream shapes for one
absence.

**`TurnClosed` carries the close kind, internally tagged under the shared test.**
`Clean` is fieldless and `Stopped` is struct-shaped, so the enum falls in the
test's second case and renders `{"close":"clean"}` and `{"close":"stopped",
"reason":...}`, one shape for both closes. Verified that the default emits a bare
string for the first and an object for the second, which is two shapes for one
field. This is the one payload the merged corpus fixes today, per
`weaver-harness-trace-contract` section 3 and charter section 3.1.

**The kind-to-payload mapping is total, fourteen kinds and eight
dispositions.** `load`, `unload`, `session.closed`, and `turn.started` carry
`None`. The three message kinds carry `Message`. `turn.closed` carries
`TurnClosed`. `fault` carries `Fault`. The three model kinds carry their three
own shapes, one each. The tool bracket's two carry `Deferred`. Four plus three
plus one plus one plus three plus two is fourteen, which is the whole of
charter section 3.1's set. The count is stated because an earlier draft of this
paragraph assigned thirteen and left `turn.started` homeless, and it is
recounted here because the token workflow's trace act of 2026-08-02 moved four
kinds out of `Deferred` and the dispositions went from four to eight.

**What is spliced and what is shaped, and the line between them is the
charter's own.** `Fault` splices, because `fault-report` is `weaver-types`'
definition and this crate links no internal crate, so the harness renders the
floor's shape and hands the bytes exactly as it does for a message. The
`sampling` member splices for the same reason, the knobs being the floor's and
the SPU's. The `rendered` member splices because it is the model's prompt as
the family library produced it, which is that library's shape and not this
crate's. What is shaped here is what no other crate defines: the measurement's
readings, the finish condition, and the identities that join them. The rule is
the charter's section 2.5 read mechanically, this crate defines the record's
own schema and carries everything else opaque.

**The measurement's optional members are absent rather than zero, which is a
serde election with a stated reason.** `skip_serializing_if` on the two signal
vectors and on the reductions means an unproduced reading emits no member at
all, per the charter's producing obligation. A zero-length vector rendered
would say the reading was taken and found empty, and a zeroed vector would say
the model was certain, and neither is what an absent instrument means.

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
pairing on the writing side.

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

```rust
pub struct WorkingStructure { /* private */ }

impl WorkingStructure {
    pub fn new(run: RunOrdinal) -> Self
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
than a discipline.

**The index is the envelope fields and never the payload.** `sequence`, `kind`, and
`turn` are lifted out of the line so a reader selects without parsing, which is what
keeps the harness's per-turn reads from scanning the whole run. Nothing else is
lifted, because every further field would be a second copy of data the line already
holds.

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

## 5. The emit path

The four steps of charter section 4.2, in order, always, with admission ahead of
the fan-out.

```rust
pub struct Recorder { /* private */ }

impl Recorder {
    /// The crate's one constructor and its one receive site.
    pub fn receive(sink: OwnedFd, run: RunOrdinal, session: SessionRef)
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

**The structure lands first and the return is the acknowledgment.** The turn
proceeds on the return, with the stream write still in flight, which is the trade
charter section 4.2 makes deliberately. A submission that cannot append is a failure
before anything is queued, so the two sinks cannot disagree about what was admitted.

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

**The three boundary states are derived rather than stored.** Committed is the
highest sequence handed to the sink, admitted is the highest assigned, queued is
the difference, and a failed write is the last error. A reader interrogates the
boundary while the process lives, and nothing about it survives the process, per
charter section 6.

**Whole events only.** The writer hands the sink one complete line plus its
terminator per write and never a partial line, so a consumer's account truncates at
an event boundary. Where the sink is a file opened append-only, the kernel's append
semantics carry that, and where it is a pipe or a socket the writer's own framing
does, a short write being retried to completion rather than reported as success.

**Pressure is reported to the harness and never authored by this crate.** When the
queue exceeds its high-water mark while the sink remains writable, the recorder
surfaces `CommitPressure` to its caller. The harness authors a `fault` event in
response, per the fault-carrier ruling of 2026-08-01, which is the only way a
pressure condition reaches the record: this crate authors no event and holds no
event kind, per charter section 2, and a recorder that emitted its own pressure
event would have ended the sole-writer property in the act of reporting on it.

**A write that fails against a live process is named and surfaced, never
swallowed**, per charter section 4.2. A tail lost to process death is the different
failure the charter separates, and this crate reports the first and cannot report
the second.

## 7. Open and drain

**Open takes descriptors and offers no path-taking constructor.** There is no
`open<P: AsRef<Path>>` in this crate, not behind a feature, not for tests. Charter
section 4.1 makes the absence the custody model's API consequence, and the previous
tree's trace-root resolver with zero production callers is the failure it names.
This is a compile-fail pin, per section 10.

**The receive site is `Recorder::receive`, declared in section 5, and it takes no
flag argument.** It accepts an `OwnedFd` and nothing that could name a path.
Close-on-exec is supplied by the harness at its own receive, per
`weaver-admin-harness-contract` section 5, and append-only rides the open file
description from admin's open. Neither flag is a type property of this crate, so
neither is pinned here: what the pin reaches is the shape, one constructor
returning a `Recorder` the rest of the crate cannot build another way.

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
    CommitPressure { queued: usize },
    AppendFailed { sequence: Sequence },
    WriteTargetUnusable { source: WriteError },
}

pub enum SubmitRefusal {
    UnknownKind,
    PayloadMalformed,
    RequiredFieldAbsent { field: FieldName },
}
```

Five cases, matching `weaver-harness-trace-contract` section 5 one to one. Nothing
returns a partial result with a success status, and every refusal names its case
rather than carrying a string, so a caller branches on a value.

**`AppendFailed` fails loudly and does not attempt recovery.** The contract gives
the recorder the resolution and there is no record to rebuild from since the cut of
2026-08-01, so the only honest response is to fail rather than continue against a
structure that is missing what the stream holds.

## 10. What is enforced, and by which instrument

**Enforced by the compiler.**

- The kind enum is exhaustive, so a fifteenth kind breaks every consumer's match.
- `WorkingStructure` exposes no mutation surface: no method returns `&mut Record`
  and the append is crate-private, so alteration after landing is unrepresentable
  rather than merely forbidden.
- The failure enum is exhaustive, so a new case reaches every caller.

**Enforced by compile-fail tests, because the property is an absence.**

- No path-taking write surface: doctests attempting to construct a writer from a
  `&str`, a `String`, and a `PathBuf` each fail to compile. Three named shapes
  rather than a claim about all possible shapes, with the general prohibition
  staying review's, per the split `weaver-traits-Spec` section 7 makes.
- No `remove`, `truncate`, or `get_mut` on the working structure.

**Enforced by the manifest.** No `weaver-*` dependency, read against the graph
under gate H2. No async runtime and no socket crate in the resolved tree, by the
build-time `cargo tree` assertion the floor Specs share.

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
