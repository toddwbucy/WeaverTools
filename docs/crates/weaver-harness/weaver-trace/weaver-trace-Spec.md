# weaver-trace - Spec

**Status:** DRAFT. Cut 2026-08-01, third of the Spec pass and the last of the
floor's build order. No code is written against it until phase three is ratified,
per Working Process section 6.

**Date filed:** 2026-08-01
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

**The dependency set is two crates and no internal one.** `serde` with `derive` and
`serde_json` for the canonical rendering of section 2. **No `weaver-*` dependency at
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
    pub envelope: Envelope,
    pub payload: Payload,
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

pub enum Kind {
    Load, Unload, SessionClosed,
    TurnStarted, TurnClosed,
    MessageUser, MessageAssistant, MessageToolResult,
    ToolCallStarted, ToolCallCompleted,
    Fault,
    ModelRequest, ModelOutput, ModelMeasurement,
}
```

**Fourteen kinds, exhaustive, matching charter section 3.1 exactly.** The enum is
exhaustive rather than `#[non_exhaustive]` because the set is closed by ruling and
adding one is an edit to the charter and to every contract naming the set: an
attribute that let a consumer absorb a fifteenth kind into a wildcard would defeat
the closure the corpus keys on.

**`turn` is optional and `causal_parent` is optional, and nothing else is.** A
run-level event belongs to no turn, which is what the option expresses, and the
recorder never infers one, per the contract's section 3. A malformed submission
carrying no turn on a turn-level kind is refused rather than defaulted.

**The payload is opaque for the three message kinds and typed for the rest.**
`Payload` holds pre-rendered canonical bytes the harness supplies for
`MessageUser`, `MessageAssistant`, and `MessageToolResult`, because this crate
neither defines the message model nor decodes it, per charter section 3. For every
other kind the payload is a shape this crate defines, since the kind's content is
trace vocabulary rather than conversation content.

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
pub fn submit(&mut self, event: Event) -> Result<Sequence, Failure>
```

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

**`submit` takes `&mut self` and there is one of it.** The recorder is not shared
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

**The receive site is one function taking no flag argument.** Close-on-exec is
supplied by the harness at its receive, per `weaver-admin-harness-contract` section
5, and append-only rides the open file description from admin's open. Neither flag
is a type property, so neither is pinned: what the pin reaches is the shape, one
receive site returning a handle the rest of the crate cannot construct another way.

**Drain empties the queue and returns.** At unload the harness calls `drain` before
answering left, so a left answer means everything admitted reached the sink, per the
coordination contract's ordering. At process death nothing drains and the tail is
forfeited, which this crate cannot report because the process reporting it is the
one that died.

## 8. Verbosity

**The recorder applies the level and never chooses it.** The harness filters at
authoring, per the contract's section 3, so an event above an unelected ceiling is
never submitted. This crate holds the run's level to answer questions about what
was recorded and applies no filter of its own, because a recorder that dropped an
event because it judged the level would have taken policy the harness holds.

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
- Close-on-exec on every descriptor this crate writes through, confirmed by
  watching a spawned child inherit the handle when the flag is removed, per apex
  section 11's requirement that the test be watched to fail.

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
- **The weight this crate carries at all.** The charter stages the question of how
  much durability machinery survives its obligations. This Spec's answer is the
  five modules of section 1 and nothing further, and the entry condition is the
  Spec pass, which is now, so the item closes here rather than being carried:
  canonical rendering, an append-only structure, a bounded queue with an
  interrogable boundary, and a typed failure set are what the obligations demand,
  and anything beyond them would be weight this program did not ask for.
