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
//! tagged, and an enum with any variant wrapping a primitive, a sequence, or
//! another tagged enum is adjacently tagged.
//!
//! The socket type election (`SOCK_SEQPACKET`) and the boundary obligations bind
//! the pair-creating crates, per the Spec; this crate opens no socket and the
//! one number a builder needs is [`MAX_ENVELOPE_BYTES`].

use serde::{Deserialize, Serialize};

use crate::config::{FieldName, GateInstruction, ModelBinding};

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
    pub model_binding: ModelBinding,
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
