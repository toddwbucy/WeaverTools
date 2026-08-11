# weaver-types - Spec

**Status:** MERGED. Cut 2026-08-01 with `weaver-traits-Spec` as the floor pass of phase
one's Spec pass. Code is written against it under the gates of Working Process section
6.

**Date filed:** 2026-08-01
**Revised:** 2026-08-11, the seam streams. `TokenAnswer` gains its `Token`
case, the intermediate the decode contract's streaming ruling of this date
enumerates, any number preceding the close and none closing, the identifier
a bare `u32` and the piece the family's rendering of that one token. The
decoder's section gains the `identity` field, the open exchange's canonical
messages as configuration rather than history, required with an empty list
legitimate, interior to the section with no vocabulary node moving.
**Revised:** 2026-08-10, second of that date. The shared tagging test of section
4.3 gains its fourth arm, identical in `weaver-traits-Spec` section 3 so the
floor cannot drift: an enum with a variant wrapping a struct that carries a
spliced member is adjacently tagged, the trio's code act having measured
internal tagging failing the round trip at deserialization, an invalid-type
error at the splice, where the rule's rationale named only write-side
failures. Applied at section 4.3: the token directive and refusal internally
tagged, `Finish` fieldless, and `TokenAnswer` adjacent under the new arm.
**Revised:** 2026-08-10. The SPU's configuration becomes a section:
`spu-instruction` holds a `decoder` subsection carrying the model binding and
the readout election together, and the section is what `EnterPayload` and
`Admit` now carry, which closes the route the charter's revision of this date
names, the election having been declared, held, and given no seam to cross. The
gate's instruction is the pattern followed rather than a precedent invented. In
the same act, `Generation` settles at section 4.4 and its bullet leaves section
6: the emission and the finish are shaped here because the harness consumes
them, and the measurement splices because nothing consumes it on the way to the
trace's model events, the satellites staying `weaver-trace`'s. Section 1's
dependency set names `serde_json`'s `raw_value` feature, the splice's gate, on
the review seat's finding that the set was argued at feature granularity and
the new member was absent. No graph record moves, the trio's nodes being the
charter's and the config vocabulary unchanged.
**Revised:** 2026-08-08, second of that date. The trio names what exists. The
messages it carries are `weaver-traits`' `Message`, drawn rather than restated,
where an earlier wording named a `CanonicalMessage` that exists nowhere.
`Generation` is moved to section 6 as an open election rather than left as a
name with no shape: the determination is the decode contract's and is not in
question, and what is open is where the payload lives, seven of the satellites
it implies being `weaver-trace`'s against a crate that links one internal
dependency. Restating them would put one fact in two places with no authority
and drawing them would widen a floor whose argument is thinness, so the deferral
follows the fault report's rather than inventing a shape ahead of its answer.
Section 6's encoding bullet narrows to the encoding, a review finding of this
act: the entry preceding this one argued that the shape and the encoding
separate and did not carry that through to the bullet still claiming both, so
the document represented the trio at section 4.4 and called it unrepresented at
section 6.
**Revised:** 2026-08-08. The token trio is held at the new section 4.4, its cases
being the decode contract's sections 2 and 5 and this crate holding rather than
creating them. An earlier wording of section 4 deferred the trio's shape and its
encoding together, and the two separate: what cases a type carries is determined
by the demand construction puts on it, and only the encoding waits on the
hot-path measurement. The deferral had become circular, that measurement being
taken against traffic that cannot exist until the seam carries the trio. The
section is retitled from the loop 0 wire vocabulary to the organ wire vocabulary,
the heading being the only place in the corpus that phrase appeared and every
citation to this section being by number. **The sorting test is rate of change
rather than loop number,** per the operator of this date: a type an organ
publishes is constitutive and lives here, loop 0 among them because a badly
changed loop 0 is a model that cannot load, and a loop conforms to an organ's
type where it uses one without being obliged to use it. A shelf for types a
particular loop 1 defines is not created here, nothing in this crate being one
today. No graph record moves: the nodes are the charter's and the ordering's
instrument is `weaver-spu-Spec` section 9's.
**Revised:** 2026-08-06, `lifecycle-refusal` gains `StateNotObservable`. The
`weaver-admin` code act found sections 2 and 3 of `weaver-admin-Spec` jointly
unsatisfiable for `show` and `list`: the answer must be one of the floor's two
enums, and the only fitting answer cases carry an `AgentState` the corpus has no
source for. The case is the honest third door, per section 4.2, and it retires
with the observation exchange that closes the gap.
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
disk, and what octets the organ wire vocabulary becomes on the wire. Where this
document and the charter disagree the charter yields nothing.

**This document declares its crate's assertion records, and one edge that is
another crate's,** per Document Format sections 3 and 4 as of the notation of
2026-08-03. The exception is the shared tagging test of section 4.3: one claim
is one node with an `asserts` edge per crate bound by it, the node lives at the
statement both floor Specs share, and `weaver-traits`' edge is therefore
declared here beside it. **`weaver-traits-Spec` declares its own assertion
records except that edge,** and says so, which is the other half of the same
rule: without both halves the traits act either redeclares an edge already
declared here, which is the duplicate the format forbids, or drops part of its
crate's assertion set with nothing recording where it went. The charter
stays the source of the crate node, the `agent-config` artifact, its six
`holds` edges, and the seventeen vocabulary definitions. What this document
sources is the claims code must conform to, declared at the clauses that argue
them rather than gathered in one place, per that format's section 6, and
`asserts` runs from the crate rather than from this document, which is why the
document needs no node of its own.

## 1. The crate

**Layout.** One module per charter subsection, re-exported at the root.

    src/lib.rs        re-exports, and nothing else
    src/config.rs     the agent config and its six fields, section 2
    src/identity.rs   peer identity and the authorization predicate, section 3
    src/wire.rs       the organ wire vocabulary and the envelope, section 4

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
used, for the reason `weaver-traits-Spec` section 1 gives: a nightly requirement
on the floor is a nightly requirement everywhere.

**The dependency set is four crates, one of them optional, and each is argued.**
`weaver-traits`, the one internal dependency, declared as the `floor-link` the
charter carries and required because the config's permission mode and tool set
elect from that crate's vocabulary. `serde` with `derive`, for the config and the
wire types both. `serde_json` for the wire encoding, per section 4.3, with its
`raw_value` feature on, because section 4.4's measurement is spliced JSON and
the splicing type is what that feature gates. A
maintained YAML implementation for the config file, per the election of section
2, **behind a non-default `config` cargo feature**. Nothing else, and specifically
no socket crate, no async runtime, and no logging: this crate defines what crosses
a boundary and crossing it is somebody else's crate, per charter section 3.

```graph
node: types-one-floor-link
kind: assertion
tag: manifest

edge: asserts
from: weaver-types
to: types-one-floor-link

edge: grounds
from: types-one-floor-link
to: axiom-floor-is-vocabulary-behavior-is-socket

node: types-no-socket-no-runtime-no-io
kind: assertion
tag: manifest

edge: asserts
from: weaver-types
to: types-no-socket-no-runtime-no-io

edge: grounds
from: types-no-socket-no-runtime-no-io
to: axiom-floor-is-vocabulary-behavior-is-socket
```

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
way to learn about JSON.

```graph
node: types-config-format-yaml
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-config-format-yaml
```

**What this election must survive is that `serde_yaml`
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
    pub spu_instruction: SpuInstruction,
    pub tool_set: Vec<ToolName>,
    pub permission_mode: weaver_traits::PermissionMode,
    pub gate_instruction: GateInstruction,
    pub trace_sink: TraceSink,
}

pub struct SpuInstruction {
    pub decoder: DecoderInstruction,
}

pub struct DecoderInstruction {
    pub model_binding: ModelBinding,
    pub residual_readout_election: bool,
    pub identity: Vec<weaver_traits::Message>,
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

**The SPU's fields arrive as one section, and the gate's already did.** Charter
section 2.1 rules that an organ's fields are named together and cross together,
and `spu-instruction` is that rule's first application: one declaration admin
validates, the harness carries uninterpreted, and the SPU consumes.
**`decoder` names a role rather than a slot.** The organ's chartered domain is
every semantic operation in the text modality, per `weaver-spu-PRD` section 8,
and the decode role is the one whose seam stands,
`weaver-harness-spu-decode-contract`, so the key names something built, with a
reader today. An embedder key arrives in the act that builds an embedder, named
here as absent rather than carried empty, which is the near side of apex
section 9. The no-defaulting argument below survives the nesting untouched:
every field of the section is required, absence refuses the load, and the depth
at which a field sits changes nothing about what its absence means.

**The identity material rides the decoder's section**, resolving the open
exchange's source per the harness handoff's first question: the canonical
messages the identity prefix is rendered from are configuration rather than
history, the working structure being empty at enter by construction, so the
operator writes them, admin validates them as part of the whole parse, the
harness carries them uninterpreted, and the SPU consumes them at the
session's open. The field is required like every field and an empty list is
a declaration the operator made, an agent with no identity prefix being a
legitimate agent, where an absent field is a file unfinished. The messages
are `weaver-traits`' `Message`, drawn rather than restated, and the field is
representation interior to the section the way the gate instruction's access
rule is, no vocabulary node moving.

**Field names are kebab-case on disk and snake_case in Rust, which takes an
explicit election rather than a convention.** `#[serde(rename_all = "kebab-case",
deny_unknown_fields)]` on the struct, so the operator writes `model-binding` and
`trace-sink` as the charter's vocabulary names them and the Rust field names stay
idiomatic. The `deny_unknown_fields` half is the fixed-surface mechanism of section 2
and stands until the spec round
`weaver-types-PRD` section 2.1 names. Without the rename the two spellings diverge,
which is the collision Document Format section 5 rules against for graph identifiers and
which reads the same way on a page.

```graph
node: types-config-names-kebab
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-config-names-kebab
```

**Every field of the declared surface is required and absence is a refusal, with the
temptation named.** The surface is the union of what the organs register, per
`weaver-types-PRD` section 2.1, so what is required is a per-binary fact rather than
one struct's field list, and the refusal is against that surface rather than against a
fixed type. The property below is unchanged by that and is why the rule exists.
Charter section 5 rules that absence is never read as a default unless the charter
says a field is optional and says what its absence means. No field here is
optional. The residual-readout election is what a builder will reach to default,
to off, and it is exactly the one that must not: an operator who stated no readout
has not thereby declined it, and admin refusing the load is how that operator
learns the file is incomplete rather than discovering it in a record with no
reductions in it. This is why `AgentConfig` derives no `Default` and `parse`
returns no partial value.

```graph
node: types-required-field-refuses
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-required-field-refuses

node: types-no-default-derive
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-no-default-derive
```

**`trace-sink` names a sink and not only a path.** A file, a pipe, or a socket are
all conforming sinks, per `weaver-admin-operator-contract` section 3, so the field
carries a discriminated shape and admin opens by the discriminant. A bare path
would force admin to guess from the filesystem what the operator meant, and the
guess is wrong exactly when the operator meant a named pipe that does not exist
yet, which is the discriminant's whole argument.

```graph
node: types-trace-sink-discriminated
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-trace-sink-discriminated

edge: grounds
from: types-trace-sink-discriminated
to: axiom-contract-is-a-complete-interface
```

**`File` and `Pipe` carry a creation flag and `Socket` does not, and the
asymmetry has a reason rather than an oversight.** Admin can create either of the
first two, an empty file or a `mkfifo`, and the charter's validate step reads the
flag to decide whether a missing sink refuses the load or is made. A socket sink
is different in kind: admin connects to it and something on the operator's side
must already be listening, so a creation flag would promise an act admin cannot
perform. A missing socket sink therefore always refuses.

**`ModelBinding` carries the artifact and the devices it is assigned to, and
the devices are a set.**

```rust
pub struct ModelBinding {
    pub artifact: ArtifactRef,
    pub devices: Vec<DeviceOrdinal>,
}
```

Per charter section 2.1 as the ruling of 2026-08-03 states it. The vector is
ordered and the order is the shard order, so a two-device assignment says which
device holds which half rather than leaving a builder to pick, and a set of one
is the ordinary case with no special shape. **An empty set is a parse error
rather than a default**, because a binding assigning no device is a
declaration the operator did not finish, and defaulting it to device zero is
the placement decision this crate exists not to make. Whether the devices
exist, whether they can reach each other, and whether the backend can shard
across that many are all admission's, per `weaver-spu-Spec` section 3, and
this crate answers well-formed and nothing more.

**`SpuInstruction` and `GateInstruction` are defined here and are the same types the
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
rather than in admin remembering to check.

```graph
node: types-config-parse-total
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-types
to: types-config-parse-total
```

What admin adds beyond the parse is
existence, that the model artifact resolves, the sink exists or its creation flag
is set, and the boundary is as the operator wrote it, per `weaver-admin-PRD`
section 4.3. The parse answers well-formed and the checks answer real, and this
crate owns the first only.

**A field no organ registered refuses rather than being ignored.** Unknown is measured
against the declared surface, per `weaver-types-PRD` section 2.1. **The derive this
crate carries denies unknown fields against one fixed type, which is the mechanism for a
fixed surface, and it is what holds the property today.** It stands until the spec round
that charter section names, and what survives that round is the property rather than the
derive. The three sites below that name the derive say the same and are qualified rather
than rewritten, because the representation is that round's to change and not this act's.
An ignored field is an operator's declaration silently discarded, and a typo in
`permission-mode` that parses as an unknown field and vanishes is the failure this
rejection exists to prevent.

```graph
node: types-unknown-key-refuses
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-unknown-key-refuses
```

## 3. Peer identity and the authorization predicate

The identity type, the rule it is judged against, and the one policy function, per
charter section 2.2, drawn by the two seams that judge a peer by credential: the
gate's client socket, governed by `weaver-gate-world-contract`, and the
coordination socket the harness binds, governed by
`weaver-admin-harness-contract`. The second was the operator surface until the
recut of 2026-08-05 retired it, and the coordination seam took its place when the
inversion gave that seam a name and a credential to check.

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
consumers need and no more: the gate admits front-end principals by uid or group
and excludes the agent uid, and the coordination seam admits root alone, per the
inversion of 2026-08-05, which excludes the agent uid by the same reading. The
operator surface was the second consumer until that date and retired with the
service account it admitted to.

**Denial wins over permission, and the ordering is the security property.**
`authorized` returns false if the peer's uid is in `denied_uids`, whatever the
allow sets say. The gate's predicate must exclude the agent uid even where a
broad group grant would otherwise admit it, per `weaver-gate-PRD` section 2, and a
rule evaluated permission-first would let a group membership readmit the one
principal the boundary exists to keep out.

```graph
node: types-denial-precedes-permission
kind: assertion
tag: perturbation

edge: asserts
from: weaver-types
to: types-denial-precedes-permission
```

**The predicate is a free function over data and reaches nothing.** No file, no
environment, no clock, no global. Where the rule comes from, how a failure is
handled, and what happens on refusal live in the consuming crates, per charter
section 2.2, and a predicate that loaded its own access list would have taken the
first of those back.

```graph
node: types-one-policy-function
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-one-policy-function
```

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

```graph
node: types-peer-identity-no-deserialize
kind: assertion
tag: compile-fail

edge: asserts
from: weaver-types
to: types-peer-identity-no-deserialize

edge: grounds
from: types-peer-identity-no-deserialize
to: axiom-floor-is-vocabulary-behavior-is-socket
```

**The threat walk.** This Spec names the adversary each security mechanism
defeats and derives that mechanism's test from the attack, which is how a
perturbation test under apex section 11 becomes a scenario rather than an
assertion. The adversary is a
process on the host that is not a front-end principal and dials one of the two
named sockets: an elected tool running as the agent uid reaching for the agent's
own mouth, or the same tool reaching the coordination socket its harness binds.
The mechanism is
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

## 4. The organ wire vocabulary

**What this section owns is loop 0's definitions in full, plus the carriage of
anything the envelope carries,** which is a boundary the act of 2026-08-02
made visible and worth stating rather than leaving to be inferred. Loop 0's
four definitions are shaped here entire. `turn-frame` and `fault-report` are
shaped here only as far as the envelope carries them, their payload variants
and, for the frame, the election section 6 holds, because
`organ-envelope`'s representation is this document's. **The token trio is held
at section 4.4 and its encoding is not this document's**, a division the act of
2026-08-08 made after an earlier wording deferred both together. It rides the
decode socket, which is not an organ channel per `weaver-spu-PRD` section 13.2,
so the hot-path measurement elects what it looks like on the wire. What cases it
carries is the decode contract's determination and this crate holds it, because
this crate holds types rather than creating them. A later workflow reads that
boundary and knows which side its vocabulary lands on before it asks.

Four of the charter's nine definitions, then: the envelope every organ channel
carries, and loop 0's trio named for the loop whose traffic it carries rather than
for a sender, per the naming ruling of 2026-08-01.

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
    Fault(FaultReport),
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

**`Fault` enters on different grounds and the difference is worth stating.**
It is not a loop's vocabulary, so the rule below does not reach it: a fault
report is what any organ hands the harness across whatever channel it holds,
and it enters this enum because the gate's channel is an organ channel and
the gate has no second socket to carry it. The decode socket carries the same
floor definition inside its own trio rather than inside this envelope, which
is what makes `fault-report` one definition with two carriages instead of two
definitions that would drift.

**A later loop's vocabulary enters this enum in the act that charters that loop,**
which is the same loudness the trio's own case sets carry: one owner, contracts
drawing rather than growing, and a floor edit every consumer's match then sees.

### 4.2 The trio

```rust
pub enum LifecycleDirective {
    Enter { payload: EnterPayload },
    Leave,
    Stop,
    Admit { instruction: SpuInstruction },
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
    StateNotObservable,
}

pub struct EnterPayload {
    pub session: SessionId,
    pub run_ordinal: u64,
    pub spu_instruction: SpuInstruction,
    pub gate_instruction: GateInstruction,
}
```

**All three enums are exhaustive, and the absence of `#[non_exhaustive]` is the
loudness this section claims.** The attribute would force every out-of-crate
consumer to carry a wildcard arm, and a case added later would land in that
wildcard silently, which is the opposite of the property the naming ruling bought.
Exhaustive, a new case breaks every consumer's match at compile time, in the same
act that edits the floor, and every contract's drawn subset is re-read by a human
because the compiler made them look.

```graph
node: types-wire-enums-exhaustive
kind: assertion
tag: compile-pin

edge: asserts
from: weaver-types
to: types-wire-enums-exhaustive

edge: grounds
from: types-wire-enums-exhaustive
to: axiom-contract-is-a-complete-interface
```

An earlier draft of this Spec carried the
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

**`StateNotObservable` joins the set on 2026-08-06, and it refuses a question
rather than an act.** Every other case here says an act could not be performed.
This one says an answer cannot be formed: the party that knows an agent's
lifecycle state is the harness, which holds the run, and
`weaver-admin-harness-contract` section 3 charters enter, leave, and stop with
no observation, so nothing asks it. Admin can read residency from the init
system and residency is not lifecycle state, per `weaver-admin-Spec` section 3,
so `show` and `list` refuse with this case rather than construct an `AgentState`
the corpus has no source for.

**It is a marker with a scheduled death, which is why it is a refusal and not a
new answer.** Growing `AgentState` to carry a service manager's vocabulary would
settle this crate's vocabulary from another party's representation, which gate
G2 forbids, and adding an answer case for residency would make permanent a shape
the observation exchange is expected to replace. A refusal states the absence
plainly, reaches an operator as a typed value rather than as an empty result,
and leaves exactly one thing to delete when the exchange lands. **Whoever
charters that exchange retires this case in the same act**, and a corpus still
carrying it afterwards has left a marker for a gap that closed.

### 4.3 The encoding

**Loop 0's traffic is JSON, one envelope to one message, elected against the
charter's criterion.** `weaver-types-PRD` section 2.3 states the ground: this
traffic is low in volume, so compactness buys nothing measurable, and diagnostic
in audience, read from a capture when a load refuses unexpectedly.

```graph
node: types-loop0-encoding-json
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-loop0-encoding-json
```

**The tagging follows the same mechanical test `weaver-traits-Spec` section 3
states, and the two floor Specs share it so they cannot drift.** A fieldless enum
is a plain renamed string. An enum whose every variant is struct-shaped or wraps
a struct is internally tagged. An enum with any variant wrapping a primitive, a
sequence, or another tagged enum is adjacently tagged, because internal tagging
cannot represent those shapes and fails at serialization rather than at compile
time. **The same arm takes an enum with a variant wrapping a struct that
carries a spliced member**, and this clause is the trio's code act finding a
failure the rule's rationale did not name: internal tagging buffers the
content on the read side and the buffer cannot represent pre-serialized JSON,
so the round trip fails at deserialization rather than at serialization,
measured in this workspace as an invalid-type error at the splice. A wire type
one party can write and the other cannot read is not a wire type, so the arm
extends to the read side's failures rather than the write side's alone.

```graph
node: types-tagging-test
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-tagging-test

edge: asserts
from: weaver-traits
to: types-tagging-test
```

Applied here: `Position`, `Opener`, and `RefusingOrgan` are fieldless and
serialize as plain renamed strings. **The trio is internally tagged**, which is
why every case carrying a value in 4.2 takes a struct variant, `Load { agent }`
rather than `Load(AgentName)`. **`Payload` is adjacently tagged**, `#[serde(tag =
"kind", content = "body")]`, because its variants wrap enums that carry a tag of
their own. **The token trio of section 4.4 divides under the same test**:
`TokenDirective` and `TokenRefusal` are internally tagged, their value-carrying
cases struct-shaped, `Finish` is fieldless and a plain renamed string, and
`TokenAnswer` is adjacently tagged under the spliced-member arm, `Generated`
wrapping the one struct in the vocabulary that carries a `RawValue`.

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

```graph
node: types-socket-seqpacket
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-socket-seqpacket

edge: grounds
from: types-socket-seqpacket
to: axiom-floor-is-vocabulary-behavior-is-socket

node: types-envelope-bound-64k
kind: assertion
tag: review

edge: asserts
from: weaver-types
to: types-envelope-bound-64k

edge: grounds
from: types-envelope-bound-64k
to: axiom-floor-is-vocabulary-behavior-is-socket
```


### 4.4 The token trio

**This crate holds types rather than creating them.** What shape a type takes is
determined by the demand that construction puts on it, and the party under that
demand is the one that determines it. The decode contract's sections 2 and 5 are
that determination for the token trio, and this subsection is where the floor
holds the result. The charter said as much when it declared the trio: the cases
are the decode contract's enumeration, one owner and one drawer today.

**The trio is held here now because the demand exists now.** Section 9 of
`weaver-spu-Spec` requires the decode seam to answer a `token-refusal` and the
crate cannot write one, and apex section 3 steps 5 and 6 put the seam inside the
turn the program is built to complete, so no loop 1 avoids it however simple. An
earlier reading of this section deferred the shape along with the encoding. The
two come apart, and the deferral belongs to the encoding alone.

```rust
pub enum TokenDirective {
    Open { session: SessionId, messages: Vec<Message> },
    AppendAndGenerate {
        turn: TurnKey,
        delta: Vec<Message>,
        tunable: BTreeMap<String, f64>,
    },
    Cancel { turn: TurnKey },
    Flush,
}

pub enum TokenAnswer {
    Opened,
    Token { token: u32, piece: String },
    Generated(Generation),
    AtRest,
    Flushed,
    Received,
}

pub enum TokenRefusal {
    NotOpen,
    OutOfOrder,
    Overflow { resident: u64, requested: u64, capacity: u64 },
    MalformedDelta,
}
```

**Every tunable value is finite, and a value that is not is refused at the
seam.** The map carries `f64` because the wire's numbers are, and the knobs it
feeds are the operator-tunable remainder of Spec section 8's dispositions. A
`NaN` reaching a sampler is a temperature that compares false against every
bound, and an infinity is one no filter clamps, so neither may travel. Both are
refused as `MalformedDelta`, which is the case the contract already carries for
an ask this seam cannot serve, rather than as a case this document adds. **The
encoding makes the refusal reachable rather than theoretical:** JSON has no
literal for either, so a peer that computed one either emits something no
decoder accepts or emits `null`, and a `null` where a number belongs fails the
decode. The check is therefore stated here as the receiver's, at the point the
map is read, so a value that arrives by some later encoding that does carry them
meets the same refusal.

**`Message` is `weaver-traits`' and is drawn rather than restated.** The decode
contract's canonical messages are that type, shaped at `weaver-traits-Spec`
section 3 as a role and its content blocks, and this crate already links that
one as its floor-link. An earlier wording of this subsection named a
`CanonicalMessage` that exists nowhere, which is the naming slip a first reading
of the contract's prose invites: the contract describes the messages as
canonical and does not rename the type.

**`Generation` shapes what the harness reads and splices what it forwards.** The
decode contract's section 2 determines the answer as the generation and its
measurement, the measurement being the token identifiers, the per-token signals,
the timings, the template identity, the block partition, and the residual
reductions where the residency was admitted with readout elected.

```rust
pub struct Generation {
    pub emission: String,
    pub finish: Finish,
    pub measurement: Box<serde_json::value::RawValue>,
}

pub enum Finish {
    Completed,
    Stopped,
}
```

**The split is between what the harness consumes and what it carries through, and
`weaver-trace` states the rule it follows:** what is shaped in a crate is what no
other crate defines. The emission and the finish are consumed here, the first
entering the working structure as the assistant's message and the second closing
the turn, so both are shaped. The measurement is consumed by nothing on the way
past: it is forwarded whole into the trace's model events. **Seven of its
satellites are `weaver-trace`'s** and one is `weaver-spu`'s, against a crate that
links one internal dependency, so shaping it here would restate seven types that
already exist with no named authority over either copy, which is the duplication
G5 files as a defect rather than resolves by picking.

**Spliced JSON and not octets, which is a distinction this crate enforced once
already.** `RawValue` carries pre-serialized JSON written in place, so the
measurement is a member of the answer's object rather than a string of bytes
inside it. What section 4.1 refused was double encoding, and splicing is what
avoids it, so that refusal is satisfied here rather than worked around.
`weaver-trace` took the same election for the same reason, its message and fault
payloads being spliced because a tag would wrap the bytes in an object of that
crate's making.

**The SPU renders and the harness authors, which is a ruling this restates rather
than a position this invents.** The fault-carrier ruling already has the SPU
handing over a `fault-report` and the harness authoring it as the `fault` event,
per the decode contract's section 2. The measurement travels that path to the
model events, so no crate holds a second copy of a shape `weaver-trace` owns and
the sole-writer rule is untouched: what the SPU produces is data, and the event
is still the harness's to author.

**`Finish` is shaped here and `weaver-trace` shapes its own, with the harness
converting.** That is the arrangement `SessionId` and `SessionRef` already take,
per `weaver-trace-Spec` section 1, that crate linking no internal crate on purpose
and the harness converting at one call site. Two names for one fact with a named
conversion point is the corpus's existing answer for a floor type the trace needs,
and a different answer here would be a second one.

**What the harness must check, because the type cannot.** A spliced member is
opaque to the compiler, so the measurement's conformance to what the trace's model
events accept is the harness's to enforce at the submit call, in the same place the
kind-to-payload pairing is already enforced by admission rather than by serde. The
election buys thinness in the floor and pays for it there, and naming where it is
paid is what keeps the payment from going missing.

**The four refusal cases are the contract's section 5 and this document adds
none.** The session is not open or residency is not confirmed for the ask. The
directive is out of order for the seam's state, a second generation or a
mid-flight flush above all. The session cannot take the next delta, and the
overflow carries the session's own account of itself because the harness decides
what a full context means and cannot decide it without the numbers. The delta is
malformed for the family. **`OutOfOrder` is the same word loop 0's refusal
carries and is not the same case**, because the states it is judged against are
the decode seam's, which is why the trio carries its own rather than drawing
loop 0's.

**A cancel with nothing in flight answers `AtRest` rather than refusing**, per
the contract's section 2, and `Received` closes the SPU-opened fault report. Both
are answers because neither is a failure of the ask.

**The stream is a case of the answer and never a fourth type**, per the
contract's section 2 as of the streaming ruling of 2026-08-11. `Token` is the
intermediate: any number cross before the close, each carrying one drawn
identifier and its rendered piece, and none closes the exchange, which stays
`Generated`'s alone. The identifier is a bare `u32` because the wire's numbers
are, the backend's own token type staying on the SPU's side of the seam, and
the piece is the family's rendering of that one token, so a consumer
accumulating pieces holds the emission as it grows and a consumer ignoring
every intermediate reads the seam the batch way.

**The encoding stays deferred and the deferral is the encoding's alone.** The
hot-path measurement elects it, per section 4.3's boundary rule and
`weaver-spu-Spec` section 11, and the measurement is taken against real decode
traffic which the first demonstration produces. Until that traffic exists the
seam carries this trio as JSON, the same encoding loop 0 carries, and **that is a
provisional election with a stated trigger rather than an answer**: it is
reconsidered when the first demonstration produces traffic to measure, and a
build that has produced that traffic and not reconsidered has an open election
reading as a settled one. The decode socket carries octets at its boundary
already, so the election changes one function on each side and nothing above it.

**No record is added here.** The trio's nodes are the charter's, declared at
`weaver-types-PRD` section 2.3 with their `defines` edges, and the instrument
that buys the ordering is `weaver-spu-Spec` section 9's, which names the
assertion and the seam it is watched on. A node restated here would give the
mapper two sources for one record.

## 5. What is enforced, and by which instrument

**Enforced by the compiler.**

- The crate's one internal dependency is `weaver-traits`, checked against the
  graph's single `floor-link` under gate H2.
- The three wire enums are exhaustive, so a case added to loop 0 breaks every
  consumer's match rather than landing in a wildcard.
- The config parse yields a fully typed value or a typed error and exposes no
  partial value, so a half-valid config is unrepresentable rather than merely
  refused.
- `deny_unknown_fields` makes an unrecognized config key a parse error rather than a
  silent discard, against the surface this crate declares as one type today.

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

**Which invariant each claim serves, and why most serve none.** Seven of the
seventeen carry a `grounds` edge, five to
`axiom-floor-is-vocabulary-behavior-is-socket` and two to
`axiom-contract-is-a-complete-interface`. The other three axioms take nothing from this
crate. The floor states no claim about a turn key, it is not an organ, and it is not
integrated: the fifth invariant binds what crosses between domains and the floor crosses
nothing, being linked rather than reached. **A crate can be drawn by every domain and
still be the subject of none of the invariants about domains**, which is the floor's
whole character stated from the graph's side. **The test applied is whether the axiom is
the reason the claim exists.** Remove the socket invariant and this crate has no reason
to elect a socket type, no reason to bound an envelope, and no reason to withhold a
deserializer from a credential, so those three ground in it. Remove it and the config
format is still YAML, the names are still kebab-case, and the parse is still total, so
those three ground in nothing. **Ten claims grounding in no invariant is the expected
result and not a gap**, per Document Format section 4: a floor crate is mostly
representation, and representation is what the invariants are not about.

The two edges to the contract invariant are the ones worth stating rather than
leaving to be read. The wire enums are exhaustive so that every case a contract
can return reaches a match loudly, which is that invariant's completeness enforced
at the type level rather than asserted in prose. The sink is a discriminated shape
rather than a bare path for the same reason at the vocabulary layer: a bare path
is vocabulary without meaning, and the consumer would have to guess what the
operator meant, which is the guess the discriminant exists to refuse.

**Where the assertion records sit, and which of these this crate declares.**
The records are at the clauses that argue the claims, across sections 1 through
4, rather than gathered here, per Document Format section 6: this section sorts
by instrument and the arguments are elsewhere, so a block here would sit apart
from the prose that earns it. **A claim this section owes to another crate is
declared by that crate,** not here, because the assertion belongs where its test
lives and a node declared twice is the one-name-two-nodes defect the format
forbids for identifiers. Three of the bullets below are such owings and carry no
record in this document: the out-of-order refusal owed to each organ, the
boundary and truncation tests owed to the pair-creating crates, and the
accept-time refusal owed to the gate, which `weaver-gate-Spec` section 6 has
already discharged.

**Requiring a perturbation-verified test, with the owning crate named.**

- In this crate: the denial-precedence test of section 3, confirmed by watching
  the agent uid pass when denial stops preceding permission.
- In this crate: a missing required field refuses the parse, run separately for
  the residual-readout election, since it is the one a builder is most likely to
  make optional.
- In this crate: a misspelled key refuses, confirmed by watching a mistyped
  `permission-mode` vanish when `deny_unknown_fields` is removed. The watch is the
  derive's removal while the surface is one type's, and it moves with the surface.
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

- **`Generation`'s shape settled at section 4.4 and this bullet retires with it.**
  The emission and the finish are shaped in the floor because the harness consumes
  them, and the measurement splices because nothing consumes it on the way to the
  trace's model events. The seven satellites stay `weaver-trace`'s, neither
  restated nor drawn, and the floor's dependency set is unchanged. **What does not
  retire is the block partition's own question**, which party labels a block's
  spans. Splicing decides where that answer is owed rather than what it is: the
  renderer of a shape the trace accepts is the SPU, so the label is written there,
  and the question is answered before the first measurement crosses rather than
  carried as an open election. It is filed against `weaver-spu` rather than here,
  this crate no longer having a stake in it.
- **The config file's directory and naming convention.** Operator provisioning,
  outside what this program governs, per section 2. What this Spec fixes is that
  admin resolves one file or refuses.
- **The YAML implementation.** A maintained one, confirmed at the moment the
  manifest is written, per section 2. If none exists the format election re-runs
  against TOML on the writer-audience grounds.
- **The decode seam's encoding.** The token workflow's, with the hot-path
  measurement, per section 4.3, the channel question having closed with the
  decoder-cut ruling, decode on its own socket. **The trio's representation
  left this bullet when section 4.4 landed it,** the directive and the refusal
  being shaped there in full, and the one open part of the answer,
  `Generation`'s shape, carrying the bullet above. This bullet reads as the
  encoding alone, which is the separation section 4.4 argues and which an
  unnarrowed wording here contradicted.
- **`DeviceOrdinal` and `ArtifactRef`.** Satellites of section 2 with no
  cross-crate consequence beyond being well-formed: the first is an unsigned
  device number, so a negative one is a parse error rather than a check, and
  the second is what an operator writes to name an artifact, whose resolution
  is admin's and whose readability is the SPU's.
- **`FaultReport`'s shape.** Elected by the token workflow's trace act
  against the closed case set of `weaver-spu-PRD` section 13.10,
  `weaver-gate-PRD` section 13.4, and `weaver-harness-PRD` section 5, since
  the same shape serves the wire and the `fault` event's payload and electing
  it twice would be two shapes for one fact.
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
  representations are this crate's, but they ride a `lifecycle-answer` out of
  admin's invocation, so what an operator can be told about an agent is exactly
  what these enumerate. The lifecycle's four states, per apex section 6, are the
  floor of that set, and whether it carries more is settled with the operator
  interface's own design rather than by a builder.
- **`EnterPayload`'s field list**, which follows what admin supplies in the enter
  directive, per `weaver-admin-harness-contract` sections 3 and 5, and moves when
  that contract does.
