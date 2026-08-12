//! conforms: types-loop0-encoding-json
//! conforms: types-tagging-test
//! conforms: types-envelope-bound-64k
//! conforms: types-socket-seqpacket
//!
//! The loop 0 wire vocabulary, per `weaver-types-Spec` section 4: the envelope
//! every organ channel carries and loop 0's trio, named for the loop whose
//! traffic it carries. Loop 0's traffic is JSON, one envelope to one message,
//! low in volume and diagnostic in audience. The tagging follows the mechanical
//! test the two floor Specs share: a fieldless enum is a plain renamed string,
//! an enum whose every variant is struct-shaped or wraps a struct is internally
//! tagged, an enum with any variant wrapping a primitive, a sequence, or
//! another tagged enum is adjacently tagged, and the same arm takes a variant
//! wrapping a struct that carries a spliced member, internal tagging buffering
//! the read side into a shape that cannot represent pre-serialized JSON.
//!
//! The socket type election (`SOCK_SEQPACKET`) and the boundary obligations bind
//! the pair-creating crates, per the Spec; this crate opens no socket and the
//! one number a builder needs is [`MAX_ENVELOPE_BYTES`].

use serde::{Deserialize, Serialize};

use crate::config::{FieldName, GateInstruction, SpuInstruction};

/// The receiver's buffer bound: a message exceeding it sets the truncation flag
/// and is treated as a channel fault rather than a message, because a silently
/// shortened directive is the failure mode the boundary property was elected to
/// prevent. Generous against loop 0's traffic, whose largest case is an enter
/// payload of identifiers.
pub const MAX_ENVELOPE_BYTES: usize = 64 * 1024;

/// One message on an organ channel.
///
/// Three members of one object, nothing flattened, so no two layers can
/// contribute one key:
///
/// ```text
/// {"exchange":{"opener":"admin","ordinal":7},
///  "position":"open",
///  "payload":{"kind":"directive","body":{"kind":"load","agent":"alpha"}}}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganEnvelope {
    pub exchange: ExchangeId,
    pub position: Position,
    pub payload: Payload,
}

/// The opening party and an ordinal, per `weaver-organ-channel` section 1.
/// Naming the party rather than carrying a bare bit is what lets a capture read
/// as itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeId {
    pub opener: Opener,
    pub ordinal: u64,
}

/// The four parties that hold an organ channel. The enum grows when an organ
/// does, which is a floor edit in the act that charters the organ.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Opener {
    Admin,
    Harness,
    Spu,
    Gate,
}

/// A message's position in its exchange: open, continue, or close, per
/// `weaver-organ-channel` section 1. Three rather than two because the channel
/// layer permits an intermediate message between a directive and its answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Position {
    Open,
    Continue,
    Close,
}

/// What an envelope carries: a typed enum rather than octets, which is what
/// makes the encoding legible. Adjacently tagged, because its variants wrap
/// enums that carry a tag of their own.
///
/// A later loop's vocabulary enters this enum in the act that charters that
/// loop. `Fault` enters on different grounds: a fault report is what any organ
/// hands the harness across whatever channel it holds, and the gate has no
/// second socket to carry it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "body", rename_all = "snake_case")]
pub enum Payload {
    Directive(LifecycleDirective),
    Answer(LifecycleAnswer),
    Refusal(LifecycleRefusal),
    Frame(TurnFrame),
    Fault(FaultReport),
}

/// The organs that refuse inside a fan-out: only the SPU and the gate. Admin
/// does not refuse to itself and the harness returns the aggregate rather than
/// appearing inside it, so reusing the four-case `Opener` would let a
/// well-typed aggregate claim that admin refused as an organ.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefusingOrgan {
    Spu,
    Gate,
}

/// One directive type for loop 0, carrying every case that crosses any of its
/// four seams; each contract's vocabulary clause names the subset that crosses
/// its own. Internally tagged, which is why every case carrying a value takes a
/// struct variant. Exhaustive: a case added later breaks every consumer's match
/// at compile time, in the same act that edits the floor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
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

/// Every directive receives exactly one answer: `Enter` answers `Ready`,
/// `Leave` answers `Left`, `Stop` answers `TurnAborted` or `AtRest` by what it
/// interrupted, `Admit` answers `Admitted`, `Release` answers `Released`,
/// `Raise` answers `GateReady`, `Lower` answers `GateStopped`, `Validate`
/// answers `Validated`, `Load`, `Unload`, and `Show` answer `State`, and `List`
/// answers `Agents`. Any directive may answer a [`LifecycleRefusal`] instead,
/// which is the second half of what one answer per request means.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
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

/// The refusal cases, drawn from the merged contracts, the set closed at this
/// crate. A receiving party matches its own cases and refuses the rest as
/// `OutOfOrder`, which is a real obligation rather than a formality.
///
/// `OrganRefused` boxes an inner refusal because the aggregate carries it
/// unchanged: a refusing organ's reason reaches admin without translation, so
/// the harness wraps rather than re-encodes, and the box keeps the enum's size
/// from being set by its deepest case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LifecycleRefusal {
    Unauthorized,
    Malformed,
    NoSuchAgent,
    CarriedWork,
    OutOfOrder,
    DescriptorsUnusable,
    BoundaryUnverified,
    ConfigInvalid {
        field: Option<FieldName>,
    },
    ArtifactUnresolvable,
    ArtifactUnreadable,
    DeviceCannotAdmit,
    NoResidency,
    BindFailed,
    OrganRefused {
        organ: RefusingOrgan,
        reason: Box<LifecycleRefusal>,
    },
    ActivityNotAtRest,
    /// The answer cannot be formed, where every case above says an act could
    /// not be performed. The party that knows an agent's lifecycle state is the
    /// harness, and no chartered exchange asks it, so `show` and `list` refuse
    /// with this rather than construct an `AgentState` from residency, which is
    /// a different fact. Retired by the observation exchange that closes the
    /// gap, per `weaver-types-Spec` section 4.2.
    StateNotObservable,
}

/// What admin supplies in the enter directive, per
/// `weaver-admin-harness-contract` sections 3 and 5.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct EnterPayload {
    pub session: SessionId,
    pub run_ordinal: u64,
    pub spu_instruction: SpuInstruction,
    pub gate_instruction: GateInstruction,
}

/// A session's identifier. An identifier choice with no cross-crate
/// consequence, shaped in this crate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionId(pub String);

/// The join key that names a turn. An identifier choice with no cross-crate
/// consequence, shaped in this crate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnKey(pub String);

/// An agent's name, as the operator provisioned it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentName(pub String);

/// What an operator can be told about an agent's place in the lifecycle. The
/// four states are the apex section 6 diagram's, the floor of the set; whether
/// it carries more is settled with the operator surface's own design.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentState {
    /// No provisioned identity exists.
    Absent,
    /// Provisioned and unloaded: identity exists, nothing resident.
    Unloaded,
    /// Loaded and idle: resident and interruptible, ready for the next run.
    Idle,
    /// Work in flight.
    Active,
}

/// One row of the `List` answer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentSummary {
    pub name: AgentName,
    pub state: AgentState,
}

/// A turn frame, opaque to the gate: whatever the client sent.
///
/// **The shape is an open election with a stated constraint**, per
/// `weaver-types-Spec` sections 4.1 and 6: the frame is not held as a byte
/// vector, and which encoding it takes is elected against a measurement over
/// real client traffic. This declaration is placement and carriage only - the
/// deferral is the corpus's and code filling it would be invention.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnFrame {}

/// The decode seam's ask, per `weaver-types-Spec` section 4.4: the cases are
/// `weaver-harness-spu-decode-contract` section 2's five exchanges read from
/// the harness's side, and this crate holds rather than creates them. The
/// messages are `weaver-traits`' [`weaver_traits::Message`], drawn rather than
/// restated. The tunable map's values must be finite, and the check is the
/// receiver's at the point the map is read, per the Spec: a `NaN` or an
/// infinity refuses as `MalformedDelta` rather than reaching a sampler.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TokenDirective {
    Open {
        session: SessionId,
        messages: Vec<weaver_traits::Message>,
    },
    AppendAndGenerate {
        turn: TurnKey,
        delta: Vec<weaver_traits::Message>,
        tunable: std::collections::BTreeMap<String, f64>,
    },
    Cancel {
        turn: TurnKey,
    },
    Flush,
}

/// The decode seam's answer, per `weaver-types-Spec` section 4.4. A cancel
/// with nothing in flight answers `AtRest` rather than refusing, and
/// `Received` closes the SPU-opened fault report, both being answers because
/// neither is a failure of the ask.
///
/// Adjacently tagged under the spliced-member arm of section 4.3's test:
/// `Generated` wraps the one struct in the vocabulary carrying a `RawValue`,
/// and internal tagging was measured failing the round trip at
/// deserialization, an invalid-type error at the splice, because the tagged
/// content is buffered on the read side and the buffer cannot represent
/// pre-serialized JSON. A wire type one party can write and the other cannot
/// read is not a wire type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "body", rename_all = "snake_case")]
pub enum TokenAnswer {
    Opened,
    /// The stream's intermediate, per the contract's section 2 as of the
    /// streaming ruling: any number cross before the close and none closes
    /// the exchange, which stays `Generated`'s alone. The identifier is a
    /// bare `u32` because the wire's numbers are, and the piece is the
    /// family's rendering that became emittable at this token, which is the
    /// same text a consumer accumulating pieces holds as the emission grows.
    Token {
        token: u32,
        piece: String,
    },
    Generated(Generation),
    AtRest,
    Flushed,
    Received,
}

/// The decode seam's refusal, per `weaver-types-Spec` section 4.4: the four
/// cases are the decode contract's section 5 and this type adds none.
/// `OutOfOrder` is the same word loop 0's refusal carries and is not the same
/// case, the states it is judged against being the decode seam's, which is why
/// this trio carries its own rather than drawing loop 0's.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TokenRefusal {
    NotOpen,
    OutOfOrder,
    Overflow {
        resident: u64,
        requested: u64,
        capacity: u64,
    },
    MalformedDelta,
}

/// What a generation answers with: shaped where the harness consumes and
/// spliced where it forwards, per `weaver-types-Spec` section 4.4. The
/// emission enters the working structure and the finish closes the turn, so
/// both are shaped here. The measurement is pre-serialized JSON written in
/// place, consumed by nothing on the way to the trace's model events, and its
/// conformance to what those events accept is the harness's to enforce at the
/// submit call, the splice being opaque to the compiler.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    pub emission: String,
    pub finish: Finish,
    /// The model.request content the SPU rendered whole, the rendered prompt
    /// with its template and effective sampling, spliced into the request
    /// event's box, per `weaver-types-Spec` section 4.4 as of the custody act.
    /// Named `request` and not `rendered` because it carries the whole request
    /// and not the prompt alone.
    pub request: Box<serde_json::value::RawValue>,
    pub measurement: Box<serde_json::value::RawValue>,
}

impl PartialEq for Generation {
    fn eq(&self, other: &Self) -> bool {
        self.emission == other.emission
            && self.finish == other.finish
            && self.request.get() == other.request.get()
            && self.measurement.get() == other.measurement.get()
    }
}

/// How the generation ended, per `weaver-types-Spec` section 4.4.
/// `weaver-trace` shapes its own `Finish` and the harness converts at one call
/// site, the arrangement `SessionId` and `SessionRef` already take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Finish {
    Completed,
    Stopped,
}

/// A fault report: what any organ hands the harness across whatever channel it
/// holds.
///
/// **The shape is an open election**, per `weaver-types-Spec` section 6,
/// elected by the token workflow's trace act against the closed case sets the
/// three charters carry, since the same shape serves the wire and the `fault`
/// event's payload. This declaration is placement and carriage only - the
/// deferral is the corpus's and code filling it would be invention.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaultReport {}
