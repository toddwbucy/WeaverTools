# weaver-types - Spec

**Status:** DRAFT. Cut 2026-08-01 with `weaver-traits-Spec` as the floor pass of
phase one's Spec pass. No code is written against it until phase three is
ratified, per Working Process section 6.

**Date filed:** 2026-08-01
**Revised:** 2026-08-01, on the review seat's return. The directive enum drops
`#[non_exhaustive]`, which contradicted the loudness the same section claimed, the
derivation list corrects from five contracts to six, the access rule and the
config's items are shaped rather than implied, the answer and the refusal gain
their enumerations, the envelope's own encoding is elected and its payload stops
being octets, the satellite types are shaped with `Position` corrected against the
drawn material, the YAML election carries the maintenance fact, the truncation
obligation lands with the socket election, the test placements are named per
crate, and one pin reclassifies from the compiler to review.
**Revised:** 2026-08-01, again, per the human's G2 ruling: the format and encoding
elections cite the criteria `weaver-types-PRD` sections 2.1 and 2.3 now carry,
rather than developing those grounds here. Revised again the same day, on the
second return: the tag collision and the unrepresentable variants are fixed and
the fix verified against serde 1.x, the directive-to-answer mapping is stated,
`RefusingOrgan` replaces `Opener` in the aggregate, the two election fields take
their charter names, the sink's creation-flag asymmetry gains its argument, the
surviving working-list citation leaves, and two open elections split by whether a
contract constrains them. Revised once more the same day, on the third return:
the answering-case claim states one answer per request rather than one case per
directive, `Stop` having two cases and the earlier wording contradicting itself
two lines on.
**Revised:** 2026-08-02. `verbosity_ceiling_election` and the `VerbosityElection`
enum leave the config, per the human's ruling of this date that the trace carries
no recording level. Six fields rather than seven, and what an operator elects at
load governs production alone.
**Revised:** 2026-08-02, a second entry this date. Section 4's decode sentence
cites the closed cell rather than holding it open, decode taking its own socket
per the decoder-cut ruling of this date recorded at `weaver-spu-PRD` section 10.
**Revised:** 2026-08-02, a third entry this date. Section 2 states the feature
gate's scope, the `config` feature gating the parser and the `parse` surface
alone and never a type, `ModelBinding` and `GateInstruction` compiling with
the feature off, the owed placement of the harness Spec review landing here
after the harness branch rather than beside it.
**Revised:** 2026-08-02, a fourth entry this date, the token trio's landing at
the charter. Section 0's count reads fifteen, section 4 scopes itself to the
loop 0 subset with the trio's representation named as the token workflow's,
and the open election on the decode seam drops the channel half the
decoder-cut ruling closed, an unswept remnant of that act caught here.
**Revised:** 2026-08-02, a fifth entry this date, the token workflow's gate
act. `Payload` gains `Frame`, the enum's own later-loop rule taking its first
exercise, with the frame's shape left as an election against the octets-in-
JSON constraint this document already recorded. Section 0's count reads
sixteen and section 4 scopes to four of eight.
**Document ID:** `weaver-types-Spec`
**Parent:** `weaver-types-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

Build instructions for `weaver-types`: the module layout, the item signatures, the
file format the operator writes, the wire encoding the organ channels carry, and
the elections a builder would otherwise invent. It is derived from
`weaver-types-PRD` and from every contract that draws this crate, which is six:
the coordination, residency, gate, and trace agreements, and the two external
boundaries, `weaver-admin-operator-contract` drawing the identity pair with the
refusal and `weaver-gate-world-contract` drawing the identity pair with the gate
instruction.

Level discipline. The charter says what the crate holds and why. This document
says how it is represented, which for this crate means two representation
questions the corpus deliberately left here: what shape the agent config takes on
disk, and what octets the loop 0 vocabulary becomes on the wire. Where this
document and the charter disagree the charter yields nothing.

**This document declares no graph records,** per Document Format section 1. The
charter is the source of the crate node, the `agent-config` artifact, its six
`holds` edges, and the sixteen vocabulary definitions.

## 1. The crate

**Layout.** One module per charter subsection, re-exported at the root.

    src/lib.rs        re-exports, and nothing else
    src/config.rs     the agent config and its six fields, section 2
    src/identity.rs   peer identity and the authorization predicate, section 3
    src/wire.rs       the loop 0 vocabulary and the envelope, section 4

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
used, for the reason `weaver-traits-Spec` section 1 gives: a nightly requirement
on the floor is a nightly requirement everywhere.

**The dependency set is four crates, one of them optional, and each is argued.**
`weaver-traits`, the one internal dependency, declared as the `floor-link` the
charter carries and required because the config's permission mode and tool set
elect from that crate's vocabulary. `serde` with `derive`, for the config and the
wire types both. `serde_json` for the wire encoding, per section 4.3. A
maintained YAML implementation for the config file, per the election of section
2, **behind a non-default `config` cargo feature**. Nothing else, and specifically
no socket crate, no async runtime, and no logging: this crate defines what crosses
a boundary and crossing it is somebody else's crate, per charter section 3.

**The feature gate is thinness applied where the floor is widest.** Every crate in
the program links this one, and only admin and the harness parse the config file,
per charter section 2.1. Without the gate the gate crate and the SPU carry a YAML
parser they never call, into processes whose whole argument is that they hold
little. `weaver-traits-PRD` section 5 states the doctrine this follows: thin is
the point, and a floor that accumulates stops meaning anything. The wire types and
the identity pair are unconditional, because more than one crate draws each.

**Two format crates rather than one is a deliberate cost.** The config is written
by a human and the wire is written by a program, and the two audiences want
different things, per the elections below. A single format for both would be one
dependency lighter and would make one audience worse off, which is the wrong trade
on a file an operator hand-edits under load.

## 2. The agent config

The declarative document that defines an agent, per charter section 2.1: written
by the operator, validated by admin before a process exists, read by the harness
for the elections it carries.

**The format is YAML, elected against the charter's criterion, and the
maintenance fact is part of the argument rather than a discovery a builder
makes.** `weaver-types-PRD` section 2.1 states the ground: the file's reader is a
human writing it, so the format answers to a writer rather than a parser, carries
nesting without ceremony, and survives an operator's comments. YAML meets that
criterion, being what a system administrator already reads. TOML was the alternative
and is the Rust-native choice, and it loses on deep nesting for a reader who is
not a Rust programmer. JSON was never a candidate: no comments, and a
trailing-comma error at three in the morning on a file that gates a load is a bad
way to learn about JSON. **What this election must survive is that `serde_yaml`
was archived and deprecated by its author in 2024**, so the implementation is a
maintained one and the Spec pass names that as a requirement rather than a
preference. A builder confirms maintenance status at the moment of writing the
manifest, and if no maintained implementation exists then the YAML conclusion
falls with its premise and the TOML comparison re-runs on the writer-audience
grounds above.

**One file per agent, named for the agent, in a directory the operator owns.**
This Spec fixes neither the directory nor the naming convention, which are
operator provisioning and outside what this program governs, per
`weaver-admin-PRD` section 1. What it fixes is that admin resolves an agent name
to exactly one config file and refuses a load where it resolves to none or to
more than one.

```rust
pub struct AgentConfig {
    pub model_binding: ModelBinding,
    pub tool_set: Vec<ToolName>,
    pub permission_mode: weaver_traits::PermissionMode,
    pub residual_readout_election: bool,
    pub gate_instruction: GateInstruction,
    pub trace_sink: TraceSink,
}

pub enum TraceSink {
    File { path: PathBuf, create: bool },
    Pipe { path: PathBuf },
    Socket { path: PathBuf },
}

pub fn parse(source: &str) -> Result<AgentConfig, ConfigError>

pub struct ConfigError {
    pub field: Option<FieldName>,
    pub kind: ConfigErrorKind,
}

pub enum ConfigErrorKind {
    Malformed,
    MissingField,
    UnknownField,
    BadValue,
}
```

**Field names are kebab-case on disk and snake_case in Rust, which takes an
explicit election rather than a convention.** `#[serde(rename_all = "kebab-case",
deny_unknown_fields)]` on the struct, so the operator writes `model-binding` and
`trace-sink` as the charter's vocabulary names them and the Rust field names stay
idiomatic. Without the rename the two spellings diverge, which is the collision
Document Format section 5 rules against for graph identifiers and which reads the
same way on a page.

**Every field is required and absence is a refusal, with the temptation named.**
Charter section 5 rules that absence is never read as a default unless the charter
says a field is optional and says what its absence means. No field here is
optional. The residual-readout election is what a builder will reach to default,
to off, and it is exactly the one that must not: an operator who stated no readout
has not thereby declined it, and admin refusing the load is how that operator
learns the file is incomplete rather than discovering it in a record with no
reductions in it. This is why `AgentConfig` derives no `Default` and `parse`
returns no partial value.

**`trace-sink` names a sink and not only a path.** A file, a pipe, or a socket are
all conforming sinks, per `weaver-admin-operator-contract` section 3, so the field
carries a discriminated shape and admin opens by the discriminant. A bare path
would force admin to guess from the filesystem what the operator meant, and the
guess is wrong exactly when the operator meant a named pipe that does not exist
yet, which is the discriminant's whole argument.

**`File` and `Pipe` carry a creation flag and `Socket` does not, and the
asymmetry has a reason rather than an oversight.** Admin can create either of the
first two, an empty file or a `mkfifo`, and the charter's validate step reads the
flag to decide whether a missing sink refuses the load or is made. A socket sink
is different in kind: admin connects to it and something on the operator's side
must already be listening, so a creation flag would promise an act admin cannot
perform. A missing socket sink therefore always refuses.

**`ModelBinding` and `GateInstruction` are defined here and are the same types the
wire carries, and the feature gate never reaches them.** The `config` feature
gates the parser and the `parse` surface alone, and never a type: the two are
wire types the loop 0 directives carry, so they compile with the feature off,
which the harness's featureless link constructs directives against, per
`weaver-harness-Spec` section 1. A build with the feature off holds every type
of this crate and no YAML dependency, which is section 1's unconditional-wire
sentence stated as the gate's mechanical scope. Both travel two paths, into the
config from the operator and
across a seam inside a directive, and one type for both is what keeps admin from
re-encoding what it validated. `ToolName` is a name today and gains its element
type with the tool workflow, because it elects from `tool-trait`, which
`weaver-traits-PRD` section 3.1 holds blocked.

**Validation is a total parse into a typed value, and admin performs it.** The
crate exposes `parse` and nothing partial: no builder, no field-by-field accessor
over a half-read document. A partially valid config is the shape that lets a load
proceed on half a declaration, and the type system is where that is prevented
rather than in admin remembering to check. What admin adds beyond the parse is
existence, that the model artifact resolves, the sink exists or its creation flag
is set, and the boundary is as the operator wrote it, per `weaver-admin-PRD`
section 4.3. The parse answers well-formed and the checks answer real, and this
crate owns the first only.

**Unknown fields refuse rather than being ignored**, which `deny_unknown_fields`
supplies. An ignored field is an operator's declaration silently discarded, and a
typo in `permission-mode` that parses as an unknown field and vanishes is the
failure this rejection exists to prevent.

## 3. Peer identity and the authorization predicate

The identity type, the rule it is judged against, and the one policy function, per
charter section 2.2, drawn by the two seams that admit an outside principal: the
gate's client socket, governed by `weaver-gate-world-contract`, and the operator
surface, governed by `weaver-admin-operator-contract`.

```rust
pub struct PeerIdentity {
    pub uid: u32,
    pub gid: u32,
    pub pid: i32,
}

pub struct AccessRule {
    pub allowed_uids: BTreeSet<u32>,
    pub allowed_gids: BTreeSet<u32>,
    pub denied_uids: BTreeSet<u32>,
}

pub fn authorized(peer: &PeerIdentity, against: &AccessRule) -> bool
```

**The rule is a type here rather than a shape each consumer invents, because that
is the whole of the charter's carve-out argument.** `weaver-types-PRD` section 2.2
rests the exception on one shared definition being the only way separate processes
provably enforce the same rule, and a predicate that took a rule shaped by its
caller would deliver a shared signature over two different rules, which is the
disagreement the carve-out exists to prevent. The three sets are what the two
consumers need and no more: the gate admits front-end principals by uid or group,
the operator surface admits by membership in the `weaver-admin` group, and both
exclude the agent uid.

**Denial wins over permission, and the ordering is the security property.**
`authorized` returns false if the peer's uid is in `denied_uids`, whatever the
allow sets say. The gate's predicate must exclude the agent uid even where a
broad group grant would otherwise admit it, per `weaver-gate-PRD` section 2, and a
rule evaluated permission-first would let a group membership readmit the one
principal the boundary exists to keep out.

**The predicate is a free function over data and reaches nothing.** No file, no
environment, no clock, no global. Where the rule comes from, how a failure is
handled, and what happens on refusal live in the consuming crates, per charter
section 2.2, and a predicate that loaded its own access list would have taken the
first of those back.

**`pid` is carried and is never the basis of a decision.** `SO_PEERCRED` reports
it, so the type would be lying by omission to drop it, and it is unsound as an
authorization input because a pid is reused. It exists for the record and for a
diagnostic, and the predicate ignores it. A builder tempted to key on it is the
reason this sentence is here rather than in a comment.

**`PeerIdentity` derives `Serialize` and deliberately not `Deserialize`, and this
is a compile-fail pin.** The type's whole point is that the kernel supplies it and
the peer never asserts it. A derived `Deserialize` is the machinery for
constructing a peer identity from bytes that arrived over a socket, which is one
careless call away from the exact substitution `SO_PEERCRED` exists to prevent.
Serialization stays, because a refusal that names the peer it refused is worth
recording.

**The threat walk.** This Spec names the adversary each security mechanism
defeats and derives that mechanism's test from the attack, which is how a
perturbation test under apex section 11 becomes a scenario rather than an
assertion. The adversary is a
process on the host that is not a front-end principal and dials one of the two
named sockets: an elected tool running as the agent uid reaching for the agent's
own mouth, or any local account reaching the operator surface. The mechanism is
that both sockets are named and therefore dialable by anything that can resolve
the path, so admission cannot rest on reachability and must rest on identity. The
kernel supplies that identity rather than the peer asserting it, which is what
`SO_PEERCRED` is for and what the missing `Deserialize` keeps true, and the
predicate judges it by one shared rule so that two processes enforcing one
boundary cannot disagree.

**The walk yields tests at two levels, and each names the crate that owns it.** In
this crate, over values: `authorized` returns false for a peer whose uid is the
agent's against a rule whose allow sets include the agent's group, confirmed by
watching it return true when denial stops preceding permission. In the gate's
crate, over behavior: a connection from the agent uid is refused at accept, before
any content is read, confirmed by watching content reach the harness when the
predicate is weakened. The second is named here because the rule is defined here
and owed by `weaver-gate-Spec`, which is the G7 shape read forward rather than a
test this crate can pretend to run.

## 4. The loop 0 wire vocabulary

The loop 0 subset of charter section 2.3, four of its eight definitions: the
envelope every organ channel
carries, and loop 0's trio named for the loop whose traffic it carries rather than
for a sender, per the naming ruling of 2026-08-01. The token trio landed at the
charter on 2026-08-02 under that ruling's ratified extension, and nothing of it
is shaped here: its representation is the token workflow's, elected with the
hot-path measurement, per this section's own decode rule.

### 4.1 The envelope

```rust
pub struct OrganEnvelope {
    pub exchange: ExchangeId,
    pub position: Position,
    pub payload: Payload,
}

pub struct ExchangeId {
    pub opener: Opener,
    pub ordinal: u64,
}

pub enum Opener {
    Admin,
    Harness,
    Spu,
    Gate,
}

pub enum Position {
    Open,
    Continue,
    Close,
}

pub enum Payload {
    Directive(LifecycleDirective),
    Answer(LifecycleAnswer),
    Refusal(LifecycleRefusal),
    Frame(TurnFrame),
}

pub enum RefusingOrgan {
    Spu,
    Gate,
}
```

**`Position` is three, per `weaver-organ-channel` section 1**, which defines a
message's position in its exchange as open or continue or close. A two-variant
reading would serve the minimal exchange, which opens and closes at once, and
would have nothing to say about a directive whose answer arrives after an
intermediate message, which the channel's own layer permits.

**`ExchangeId` is the opening party and an ordinal**, per the same section, and
`Opener` enumerates the four parties that hold an organ channel. Naming the party
rather than carrying a bare bit is what lets a capture read as itself, and the
enum grows when an organ does, which is a floor edit in the act that charters the
organ.

**The payload is a typed enum rather than octets, which is what makes the
encoding of 4.3 possible.** An earlier draft of this Spec carried
`payload: Vec<u8>` with a separate kind discriminant, on the reasoning that a
generic envelope would push each seam's vocabulary into the floor's signature.
The reasoning failed against the naming ruling: loop naming means the payload
types live on the floor already, so naming them here adds no dependency and
removes a layer of encoding. It also removed a defect, since octets inside a JSON
envelope render as an array of numbers by default, tripling the size and
destroying exactly the legibility the JSON election rests on.

**`Frame` is that rule's first exercise, from the token workflow's gate act of
2026-08-02.** The gate's turn exchanges cross the same organ channel the
lifecycle directives do, so the frame enters this enum rather than taking a
carrier of its own, and every consumer's match sees the addition, which is
what the rule below is for. **Its shape is an open election with a stated
constraint,** per section 6: a frame is opaque to the gate, so it is whatever
the client sent, and an earlier draft of this Spec already recorded what
happens to raw octets inside a JSON envelope, an array of numbers that
triples the size and destroys the legibility the encoding election rests on.
The frame is therefore not held as a byte vector, and which of the two honest
answers it takes, a splice of the line as it stands where the world contract's
own NDJSON shape makes that safe, or an encoding that survives arbitrary
octets, is elected against a measurement rather than assumed here.

**A later loop's vocabulary enters this enum in the act that charters that loop,**
which is the same loudness the trio's own case sets carry: one owner, contracts
drawing rather than growing, and a floor edit every consumer's match then sees.

### 4.2 The trio

```rust
pub enum LifecycleDirective {
    Enter { payload: EnterPayload },
    Leave,
    Stop,
    Admit { binding: ModelBinding },
    Release,
    Raise { instruction: GateInstruction },
    Lower,
    Load { agent: AgentName },
    Unload { agent: AgentName },
    Validate { agent: AgentName },
    List,
    Show { agent: AgentName },
}

pub enum LifecycleAnswer {
    Ready,
    Left,
    TurnAborted { turn: TurnKey },
    AtRest,
    Admitted,
    Released,
    GateReady,
    GateStopped,
    Validated,
    State { state: AgentState },
    Agents { agents: Vec<AgentSummary> },
}

pub enum LifecycleRefusal {
    Unauthorized,
    Malformed,
    NoSuchAgent,
    CarriedWork,
    OutOfOrder,
    DescriptorsUnusable,
    BoundaryUnverified,
    ConfigInvalid { field: Option<FieldName> },
    ArtifactUnresolvable,
    ArtifactUnreadable,
    DeviceCannotAdmit,
    NoResidency,
    BindFailed,
    OrganRefused { organ: RefusingOrgan, reason: Box<LifecycleRefusal> },
    ActivityNotAtRest,
}

pub struct EnterPayload {
    pub session: SessionId,
    pub run_ordinal: u64,
    pub model_binding: ModelBinding,
    pub gate_instruction: GateInstruction,
}
```

**All three enums are exhaustive, and the absence of `#[non_exhaustive]` is the
loudness this section claims.** The attribute would force every out-of-crate
consumer to carry a wildcard arm, and a case added later would land in that
wildcard silently, which is the opposite of the property the naming ruling bought.
Exhaustive, a new case breaks every consumer's match at compile time, in the same
act that edits the floor, and every contract's drawn subset is re-read by a human
because the compiler made them look. An earlier draft of this Spec carried the
attribute beside this same argument, which the review seat caught as the
contradiction it was.

**One directive type for loop 0, carrying every case that crosses any of its four
seams, and each contract's vocabulary clause names the subset that crosses its
own:** enter, leave, and stop at coordination, admit and release at residency,
raise and lower at the gate, and the verbs with the observations at the operator
surface. The answer and the refusal follow the same rule.

**Every directive receives exactly one answer, and the mapping from directive to
answering case is stated because the operator contract requires that and a
builder would otherwise invent it.** The case is determined per directive, and
for `Stop` by what it interrupted. `Enter` answers
`Ready`, `Leave` answers `Left`, `Stop` answers `TurnAborted` or `AtRest`,
`Admit` answers `Admitted`, `Release` answers `Released`, `Raise` answers
`GateReady`, `Lower` answers `GateStopped`, `Validate` answers `Validated`,
`Load`, `Unload`, and `Show` answer `State`, and `List` answers `Agents`. Eleven
of the twelve have a single answering case. `Stop` has two, `TurnAborted` or
`AtRest`, selected by whether a turn was in flight, and both are clean closes
rather than a refusal, per `weaver-admin-harness-contract` section 3. Any
directive may answer a `LifecycleRefusal` instead, which is the second half of
what one answer per request means. `Validated` exists because validation reports an
outcome without transitioning anything, per `weaver-admin-PRD` section 4.3, and
answering it with a state would report a transition that did not happen.

**A receiving party matches its own cases and refuses the rest as `OutOfOrder`,
which is a real obligation rather than a formality.** The gate receiving an
`Admit` is not a case the gate implements, and the contracts already rule that a
directive out of order for the channel's state is refused and not queued. The enum
being wide is what makes that refusal expressible in the type the seam already
carries, and the per-seam test of section 5 is what confirms each party wrote the
refusing arms rather than a wildcard.

**`OrganRefused` boxes an inner refusal because the aggregate carries it
unchanged.** `weaver-admin-harness-contract` section 6 requires a refusing organ's
reason to reach admin without translation, so the harness wraps rather than
re-encodes, and the box is what keeps the enum's size from being set by its
deepest case. **It names a `RefusingOrgan` rather than an `Opener`**, because
only the SPU and the gate refuse inside a fan-out: admin does not refuse to
itself and the harness returns the aggregate rather than appearing inside it, so
reusing the four-case `Opener` would let a well-typed aggregate claim that admin
refused as an organ.

**The refusal cases are drawn from the merged contracts and the set is closed at
this crate.** Whether the SPU's admit cases needed a type of their own was the
cell `weaver-spu-PRD` section 10 held, and the naming ruling settled it as
extension: they are loop 0 refusals because they refuse loop 0's directives.

### 4.3 The encoding

**Loop 0's traffic is JSON, one envelope to one message, elected against the
charter's criterion.** `weaver-types-PRD` section 2.3 states the ground: this
traffic is low in volume, so compactness buys nothing measurable, and diagnostic
in audience, read from a capture when a load refuses unexpectedly.

**The tagging follows the same mechanical test `weaver-traits-Spec` section 3
states, and the two floor Specs share it so they cannot drift.** A fieldless enum
is a plain renamed string. An enum whose every variant is struct-shaped or wraps
a struct is internally tagged. An enum with any variant wrapping a primitive, a
sequence, or another tagged enum is adjacently tagged, because internal tagging
cannot represent those shapes and fails at serialization rather than at compile
time.

Applied here: `Position`, `Opener`, and `RefusingOrgan` are fieldless and
serialize as plain renamed strings. **The trio is internally tagged**, which is
why every case carrying a value in 4.2 takes a struct variant, `Load { agent }`
rather than `Load(AgentName)`. **`Payload` is adjacently tagged**, `#[serde(tag =
"kind", content = "body")]`, because its variants wrap enums that carry a tag of
their own.

**The envelope's layout is stated rather than left to a reader.** Nothing is
flattened. `exchange`, `position`, and `payload` are three members of one object,
and the adjacent tagging nests the payload's own tagged object under `body`, so
no two layers can contribute one key:

    {"exchange":{"opener":"admin","ordinal":7},
     "position":"open",
     "payload":{"kind":"directive","body":{"kind":"load","agent":"alpha"}}}

**Both halves of this election were verified against serde 1.x rather than
reasoned from memory, because an earlier draft got both wrong.** That draft
tagged `Payload` and the trio with the same `kind` name, which emits two `kind`
members into one object and fails to deserialize with a duplicate-field error,
and it used newtype variants, which fail at serialization for any case wrapping a
name or a sequence. The wire shape section 0 claims this Spec settles has to
round-trip, and the shape above does, including the sequence-carrying cases.

**This election does not reach the decode seam and must not be read as reaching
it.** Decode traffic is the hot path, its volume is per token rather than per run,
and its encoding is the token workflow's election with a measurement behind it.
Decode does not share this channel, per the decoder-cut ruling of 2026-08-02
recorded at `weaver-spu-PRD` section 10: it takes its own socket, owned by that
crate, so the head-of-line question this sentence once held open is closed.

**The envelope is length-free, because the socket type carries boundaries.**
`weaver-organ-channel` section 2 requires one write to be one read and rules that
the property comes from the socket type rather than from framing above it, leaving
the type to the Spec. The election is `SOCK_SEQPACKET`: it preserves message
boundaries and connection semantics together, where `SOCK_DGRAM` on a Unix socket
would preserve boundaries but invite the datagram reading that document explicitly
disclaims, and `SOCK_STREAM` would push a length prefix into every contract that
draws the channel, which is the layer violation that document exists to prevent.

**The election binds the crates that create the pairs, and it is owed to their
Specs.** This crate opens no socket. `weaver-admin-Spec` creates the coordination
pair and `weaver-harness-Spec` creates the residency and gate pairs, per their
charters, and each carries this election rather than re-deciding it. Naming the
landing sites here is what keeps a decision made in one Spec from reading as
settled everywhere while binding nothing.

**One write is one read only while the reader's buffer covers the message, so the
election arrives with an obligation.** A short read on `SOCK_SEQPACKET` truncates
silently and discards the remainder unless the receiver checks `MSG_TRUNC`. The
receiving crates therefore size their buffer against a **maximum envelope of 64
kibibytes** and treat a truncation flag as a channel fault rather than a message,
because a silently shortened directive is the failure mode the boundary property
was elected to prevent. The bound is generous against loop 0's traffic, whose
largest case is an enter payload of identifiers, and it is stated as a number so
that a builder sizing a buffer has one to use.

## 5. What is enforced, and by which instrument

**Enforced by the compiler.**

- The crate's one internal dependency is `weaver-traits`, checked against the
  graph's single `floor-link` under gate H2.
- The three wire enums are exhaustive, so a case added to loop 0 breaks every
  consumer's match rather than landing in a wildcard.
- The config parse yields a fully typed value or a typed error and exposes no
  partial value, so a half-valid config is unrepresentable rather than merely
  refused.
- `deny_unknown_fields` makes an unrecognized config key a parse error rather
  than a silent discard.

**Enforced by a compile-fail test, because the property is an absence.**
`PeerIdentity` implements no `Deserialize`: a doctest attempting to deserialize
one fails to compile, and it starts passing the day someone adds the derive.

**Enforced by review rather than by a mechanism, and said so.** The carve-out
bound of charter section 2.2, that this crate holds one policy function and gains
no second, is not a compile property: a signature cannot prevent a sibling
function being added beside it. It is charter enforcement read at review, and an
earlier draft of this Spec listed it as a compiler pin, which is the overclaim the
apex's enforcement section exists to prevent.

**Enforced by the manifest.** No socket crate, no runtime, no I/O in the
dependency tree, checked as `weaver-traits-Spec` section 7 checks its own, by a
build-time assertion over the resolved external tree rather than by H2.

**Requiring a perturbation-verified test, with the owning crate named.**

- In this crate: the denial-precedence test of section 3, confirmed by watching
  the agent uid pass when denial stops preceding permission.
- In this crate: a missing required field refuses the parse, run separately for
  the residual-readout election, since it is the one a builder is most likely to
  make optional.
- In this crate: a misspelled key refuses, confirmed by watching a mistyped
  `permission-mode` vanish when `deny_unknown_fields` is removed.
- In each organ's crate: a directive case belonging to another seam is refused as
  `OutOfOrder` rather than acted on, confirmed per seam by watching a wildcard arm
  swallow it.
- In the pair-creating crates, with a socket dev-dependency that the
  no-socket-crate rule does not reach because a dev-dependency ships in no build:
  one envelope write is one envelope read at the elected socket type, and a
  message exceeding the buffer sets the truncation flag rather than arriving
  short, both confirmed by watching a message split or silently shorten when the
  type is changed to `SOCK_STREAM`.
- In the gate's crate: the accept-time refusal of section 3.

## 6. Open elections

- **The config file's directory and naming convention.** Operator provisioning,
  outside what this program governs, per section 2. What this Spec fixes is that
  admin resolves one file or refuses.
- **The YAML implementation.** A maintained one, confirmed at the moment the
  manifest is written, per section 2. If none exists the format election re-runs
  against TOML on the writer-audience grounds.
- **The decode seam's encoding, and the token trio's representation.** The
  token workflow's, with the hot-path measurement, per section 4.3, the
  channel question having closed with the decoder-cut ruling, decode on its
  own socket.
- **`TurnFrame`'s shape.** Elected against the constraint section 4.1 states,
  a frame being opaque and octets inside a JSON envelope being the defect this
  crate already refused once. Settled with a measurement over real client
  traffic, which the gate's own Spec produces.
- **The `tool-set` field's element shape.** It elects from `tool-trait`, which
  `weaver-traits-PRD` section 3.1 holds blocked, so the field is a list of names
  today and gains its element type with the tool workflow.
- **`SessionId`, `TurnKey`, `AgentName`, and `FieldName`.** Named in the
  signatures above and shaped in this crate, their representations being
  identifier choices with no cross-crate consequence.
- **`AgentState` and `AgentSummary`, whose case sets are not free.** Their Rust
  representations are this crate's, but they ride a `lifecycle-answer` to the
  operator surface, so what an operator can be told about an agent is exactly
  what these enumerate. The lifecycle's four states, per apex section 6, are the
  floor of that set, and whether it carries more is settled with the operator
  surface's own design rather than by a builder.
- **`EnterPayload`'s field list**, which follows what admin supplies in the enter
  directive, per `weaver-admin-harness-contract` sections 3 and 5, and moves when
  that contract does.
