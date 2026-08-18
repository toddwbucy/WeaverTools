//! conforms: types-loop0-encoding-json
//! conforms: types-tagging-test
//! conforms: types-envelope-bound-64k
//! conforms: types-socket-seqpacket
//! conforms: types-frame-survives-arbitrary-octets
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

use crate::config::{FieldName, GateInstruction, SpuInstruction, ToolName};

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
    /// The execution exchange's ask, harness-opened per recovered call, as of
    /// the tool workflow's opening act: the enum's own rule, a later loop's
    /// vocabulary entering in the act that charters that loop.
    Tool(ToolExecution),
    /// The execution exchange's answer, closing it, one of the four contents
    /// `weaver-harness-gate-contract` section 2 names.
    ToolAnswer(ToolOutcome),
}

/// One tool call as it crosses the gate seam: the name and arguments exactly
/// as the family parse recovered them from the emission, uninterpreted by
/// the harness that carries them, and the caller's clock, per
/// `weaver-harness-gate-contract` section 2 as amended by the tool boundary
/// ruling: one clock, the caller's, validated against the tool's declared
/// maximum at the refusal layer and adopted as the kill clock.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolExecution {
    pub name: ToolName,
    pub arguments: String,
    /// The caller's clock in milliseconds. The unit is this definition's
    /// election, the contract fixing the rule and not the representation.
    pub clock_ms: u64,
}

/// The four contents an execution's answer carries, told apart by tag
/// alone, every one of them content rather than a channel fault, because
/// each is a fact the model must learn. The rule beneath the four is who
/// speaks in the return: a result is the tool's own words, a refusal is the
/// gate's voice with nothing run, an error is the machinery's, and a kill
/// carries no tool voice by construction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ToolOutcome {
    /// The invocation ran and the tool answers in its own words. For the
    /// shell a nonzero exit is a result, accounted in content, not an
    /// error.
    Result { content: String },
    /// Nothing ran, and the gate speaks in its own voice: a name it does
    /// not hold - never a nearest match, the family registry's discipline
    /// one organ over - malformed arguments, or a clock beyond the
    /// declared maximum. No side effect exists.
    Refused { reason: String },
    /// The invocation machinery failed - the fork, a pipe, the supervisor -
    /// and the account's speaker is the infrastructure, never the tool.
    Errored { detail: String },
    /// The caller's clock expired and the gate killed the invocation's
    /// whole process group. No account from the tool exists by
    /// construction - the absence of the tool's words is the fact - and
    /// output drained before the kill rides as an attachment, never a
    /// result.
    Killed { partial: Option<String> },
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
    Enter {
        payload: EnterPayload,
    },
    Leave,
    Stop,
    Admit {
        instruction: SpuInstruction,
    },
    Release,
    /// **Two fields because two authors.** The instruction is the operator's
    /// election carried uninterpreted, and the socket is the program's
    /// deployment fact, supplied by the harness inside the unit's runtime
    /// directory so the manager's create-and-destroy makes a stale pathname
    /// unreachable, per `weaver-gate-PRD` section 2. A single field would put
    /// the operator's name on a value they do not choose.
    Raise {
        instruction: GateInstruction,
        socket: std::path::PathBuf,
    },
    Lower,
    Load {
        agent: AgentName,
    },
    Unload {
        agent: AgentName,
    },
    Validate {
        agent: AgentName,
    },
    List,
    Show {
        agent: AgentName,
    },
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
    /// A prior process for this unit exited non-zero and its name is still
    /// held by the manager, so a later start under that name refuses until it
    /// is reaped.
    ///
    /// **It claims that and no more.** `failed` is the one state the init
    /// boundary reports without ambiguity, per
    /// `weaver-admin-systemd-contract` section 3, and it does not say whether
    /// the worker bound or how long it served: a unit that bound its socket,
    /// served, and exited non-zero later reads the same. What it says is what
    /// refused the load, and that is a different fact from a socket that would
    /// not bind, which is `BindFailed`.
    PriorUnitUnreaped,
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
    pub run: RunId,
    pub spu_instruction: SpuInstruction,
    pub gate_instruction: GateInstruction,
}

/// A session's identifier. An identifier choice with no cross-crate
/// consequence, shaped in this crate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionId(pub String);

/// A run's identifier, minted by admin at the load and distinct within its
/// session, per `weaver-admin-PRD` section 10.
///
/// **Not an ordinal.** Nothing program-side counts runs, because nothing
/// program-side is alive between two of them, and the requirement the record
/// carries is distinctness rather than position. What makes one distinct is
/// argued where it is minted: this crate fixes only that it is an identifier
/// rather than a number, which is what lets the mint answer without anything
/// being remembered between invocations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunId(pub String);

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

/// A turn frame, opaque to the gate: whatever the client sent, and the
/// answer going back, one definition for both directions per the charter.
///
/// The member is the line's octets encoded base64, per the election of
/// `weaver-types-Spec` section 4.1 and the ruling of 2026-08-12: RFC 4648
/// section 4's standard alphabet, padded, no line breaks and no interior
/// whitespace, so one octet sequence has exactly one carried form. The
/// encoding rides here with the type, one implementation holding the
/// canonical form for every party, and [`TurnFrame::octets`] refuses what
/// [`TurnFrame::carry`] would not produce.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TurnFrame {
    pub octets: String,
}

impl TurnFrame {
    /// Carries octets as a frame, encoded to the one canonical form.
    pub fn carry(octets: &[u8]) -> TurnFrame {
        TurnFrame {
            octets: encode_base64(octets),
        }
    }

    /// The carried octets, decoded, or `None` where the member is not the
    /// form the encode produces: a length off the four-boundary, a byte
    /// outside the alphabet, padding anywhere but the tail, or trailing
    /// bits the encode would have zeroed.
    pub fn octets(&self) -> Option<Vec<u8>> {
        decode_base64(&self.octets)
    }
}

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_base64(octets: &[u8]) -> String {
    let mut out = String::with_capacity(octets.len().div_ceil(3) * 4);
    for chunk in octets.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = u32::from(chunk.get(1).copied().unwrap_or(0));
        let b2 = u32::from(chunk.get(2).copied().unwrap_or(0));
        let word = (b0 << 16) | (b1 << 8) | b2;
        out.push(char::from(BASE64_ALPHABET[(word >> 18) as usize & 0x3f]));
        out.push(char::from(BASE64_ALPHABET[(word >> 12) as usize & 0x3f]));
        out.push(if chunk.len() > 1 {
            char::from(BASE64_ALPHABET[(word >> 6) as usize & 0x3f])
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            char::from(BASE64_ALPHABET[word as usize & 0x3f])
        } else {
            '='
        });
    }
    out
}

fn base64_value(byte: u8) -> Option<u32> {
    match byte {
        b'A'..=b'Z' => Some(u32::from(byte - b'A')),
        b'a'..=b'z' => Some(u32::from(byte - b'a') + 26),
        b'0'..=b'9' => Some(u32::from(byte - b'0') + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn decode_base64(text: &str) -> Option<Vec<u8>> {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return Some(Vec::new());
    }
    if !bytes.len().is_multiple_of(4) {
        return None;
    }
    let groups = bytes.len() / 4;
    let mut out = Vec::with_capacity(groups * 3);
    for g in 0..groups {
        let group = &bytes[g * 4..g * 4 + 4];
        let is_last = g + 1 == groups;
        let pad = group.iter().filter(|b| **b == b'=').count();
        let pad_at_tail = match pad {
            0 => true,
            1 => group[3] == b'=',
            2 => group[2] == b'=' && group[3] == b'=',
            _ => false,
        };
        if !pad_at_tail || (pad > 0 && !is_last) {
            return None;
        }
        let v0 = base64_value(group[0])?;
        let v1 = base64_value(group[1])?;
        let v2 = if pad >= 2 { 0 } else { base64_value(group[2])? };
        let v3 = if pad >= 1 { 0 } else { base64_value(group[3])? };
        // The encode zeroes the bits no octet fills, so a set bit there is
        // a second spelling of the same octets and is refused.
        if pad == 2 && v1 & 0x0f != 0 {
            return None;
        }
        if pad == 1 && v2 & 0x03 != 0 {
            return None;
        }
        let word = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3;
        out.push((word >> 16) as u8);
        if pad < 2 {
            out.push((word >> 8) as u8);
        }
        if pad < 1 {
            out.push(word as u8);
        }
    }
    Some(out)
}

/// The decode seam's ask, per `weaver-types-Spec` section 4.4: the cases are
/// `weaver-harness-spu-decode-contract` section 2's four exchanges read from
/// the harness's side, the seam's fifth message being the SPU's emitted
/// report, owed nothing back and taking its wire case with `FaultReport`'s
/// shape, and this crate holds rather than creates them. The
/// messages are `weaver-traits`' [`weaver_traits::Message`], drawn rather than
/// restated.
///
/// **No sampling value crosses this seam inbound.** The tunable map left this
/// directive when the values moved to the declaration, per `weaver-spu-Spec`
/// section 8: the engine builds its sampler once at session open, so a value
/// arriving with a turn had no engine to reach. The value discipline that
/// guarded the map moved with it to [`crate::config::DecoderInstruction`].
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
    },
    Cancel {
        turn: TurnKey,
    },
    Flush,
}

/// The decode seam's answer, per `weaver-types-Spec` section 4.4. A cancel
/// with nothing in flight answers `AtRest` rather than refusing, an answer
/// because it is not a failure of the ask. The SPU's fault report takes no
/// case here: it is the seam's one emission, owed nothing back per the
/// ruling of 2026-08-12, and its wire case arrives with `FaultReport`'s
/// shape.
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
    /// The seam's one SPU-originated emission, per the decode contract's
    /// second ruling of 2026-08-12: a case of the answer because the
    /// SPU-to-harness traffic is one enum, and the prose fact the type cannot
    /// carry is the Spec's - **it closes no exchange and answers nothing**,
    /// the trace entry being the acknowledgment. A fault arising inside a
    /// generation is that exchange's typed answer instead, so one fact never
    /// travels twice.
    Fault(FaultReport),
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
    /// The canonical parse of the emission, the family module's bridge from
    /// the verbatim to the conversation's blocks, as of the tool workflow's
    /// opening act: text as text and every recovered call as a `ToolCall`
    /// block, in emission order. Family knowledge stays in the family module
    /// and the canonical form is what crosses, so the harness dispatches
    /// calls it never parsed for. A fragment that opened a call and could
    /// not be recovered is reported by the SPU on its operator channel and
    /// is deliberately not here: a failed call must not arrive as prose.
    pub content: Vec<weaver_traits::ContentBlock>,
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
            && self.content == other.content
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
/// **The shape is the closed election of `weaver-types-Spec` section 6**,
/// decided by apex section 5.2's custody rule: the `case` is what the harness
/// itself consumes, typed and closed, because the judgment of what a fault
/// means for the turn and the residency is the harness's to make. The
/// `account` is the reporting organ's own rendering of what happened, spliced
/// verbatim, so the floor carries no organ's descriptive vocabulary and a
/// later organ's richer account grows no shared type.
///
/// **The reporting organ names its own case and the harness classifies
/// nothing.** A report arrives whole, per the contracts, so a case exists
/// before any `fault` event is authored and no raw account reaches the
/// record unclassified.
///
/// No member names the raiser and no member names the turn: the envelope's
/// field and the seam's context respectively, per the Spec's section 4.2.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultReport {
    pub case: FaultCase,
    pub account: Box<serde_json::value::RawValue>,
}

/// The spliced member compares by its octets, the pattern `Generation` set:
/// `RawValue` carries no `PartialEq` of its own and the derive would be lost
/// with it, so the comparison every test wants is written once here.
impl PartialEq for FaultReport {
    fn eq(&self, other: &Self) -> bool {
        self.case == other.case && self.account.get() == other.account.get()
    }
}

/// The three charters' closed ten, per `weaver-types-Spec` section 4.2: three
/// of `weaver-spu-PRD` section 13.10, three of `weaver-gate-PRD` section 13.4,
/// four of `weaver-harness-PRD` section 5. **An eleventh case is a charter
/// act before it is a code change**, and the tenth was exactly that: the act
/// typing these found the harness authoring an assembly fault no case
/// covered, and the charter widened before this enum did. The harness's four
/// ride the same shape although they cross no socket, one shape serving the
/// wire and the `fault` event's payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultCase {
    DeviceFaultDuringGeneration,
    ResidencyDegraded,
    ReadoutFaultWhileElected,
    ListenerLost,
    ClientConnectionFailedMidTurn,
    AdmissionFailingSystematically,
    RecorderCommitPressure,
    StreamWriteFailed,
    OrganDeathObserved,
    MessageRecordUndecodable,
}
