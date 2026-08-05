//! conforms: trace-identity-newtypes-harness-converts
//! conforms: trace-kind-explicit-renames
//! conforms: trace-subsystem-case-set
//! conforms: trace-kind-enum-exhaustive
//! conforms: trace-turn-optional-never-inferred
//! conforms: trace-message-payloads-splice
//! conforms: trace-envelope-flattens
//! conforms: trace-no-envelope-field-named-payload
//! conforms: trace-payload-untagged-kind-discriminant
//! conforms: trace-bracket-kind-omits-payload
//! conforms: trace-turn-close-internally-tagged
//! conforms: trace-kind-payload-mapping-total
//! conforms: trace-splice-or-shape
//! conforms: trace-measurement-absent-not-zero
//! conforms: trace-payload-serialize-only
//!
//! The event: the envelope, the kind set, and the payload shapes, per
//! `weaver-trace-Spec` section 3. The envelope flattens into the event and the
//! payload does not, so a line renders flat with `kind` at the top level, which
//! is what every consumer keys on first.

use std::sync::Arc;

use serde::Serialize;
use serde_json::value::RawValue;

use crate::canonical::{MonotonicNs, Sequence};

/// One event: the envelope's members and at most one `payload` member.
///
/// A bracket kind carries no payload member at all rather than a null one,
/// which `skip_serializing_if` carries. No field added to [`Envelope`] may be
/// named `payload` - the flatten puts both layers' members in one object.
#[derive(Debug, Clone, Serialize)]
pub struct Event {
    #[serde(flatten)]
    pub envelope: Envelope,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Payload>,
}

/// The identity and ordering fields every event carries.
///
/// `turn` is optional and `causal_parent` is optional, and nothing else is. A
/// run-level event belongs to no turn, and the recorder never infers one: a
/// turn the recorder supplied would be a key that never travelled with the
/// work. The absent options emit no member, on the same argument the payload's
/// absence carries.
#[derive(Debug, Clone, Serialize)]
pub struct Envelope {
    pub session: SessionRef,
    pub run: RunOrdinal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn: Option<TurnRef>,
    pub sequence: Sequence,
    pub kind: Kind,
    pub subsystem: Subsystem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causal_parent: Option<Sequence>,
    pub wall_ms: u64,
    pub monotonic_ns: MonotonicNs,
}

/// A session's identity as the harness converts it at the submit call, per
/// `weaver-trace-Spec` section 1: this crate links no internal crate, so the
/// floor's `SessionId` does not appear here and the harness converts at one
/// call site.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SessionRef(pub String);

/// The run's ordinal within its session, carried without interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct RunOrdinal(pub u64);

/// The turn's identity as the harness converts it, the join key carried and
/// never invented.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TurnRef(pub String);

/// The producing party. The four crates that can produce a report, and `Tool`
/// as the fifth because a tool's result reaches the record through the harness
/// and a record attributing it to the harness would lose the one fact an
/// operator reading a tool result wants first. A sixth case arrives when a
/// crate that can produce a report is chartered, a floor edit in the same act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Subsystem {
    Admin,
    Harness,
    Spu,
    Gate,
    Tool,
}

/// The fourteen event kinds, exhaustive, matching the charter's section 3.1
/// exactly. Every kind carries an explicit rename because no scheme produces
/// the charter's dotted names, and the enum is exhaustive because the set is
/// closed by ruling: an attribute that let a consumer absorb a fifteenth kind
/// into a wildcard would defeat the closure the corpus keys on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Kind {
    #[serde(rename = "load")]
    Load,
    #[serde(rename = "unload")]
    Unload,
    #[serde(rename = "session.closed")]
    SessionClosed,
    #[serde(rename = "turn.started")]
    TurnStarted,
    #[serde(rename = "turn.closed")]
    TurnClosed,
    #[serde(rename = "message.user")]
    MessageUser,
    #[serde(rename = "message.assistant")]
    MessageAssistant,
    #[serde(rename = "message.tool_result")]
    MessageToolResult,
    #[serde(rename = "tool.call.started")]
    ToolCallStarted,
    #[serde(rename = "tool.call.completed")]
    ToolCallCompleted,
    #[serde(rename = "fault")]
    Fault,
    #[serde(rename = "model.request")]
    ModelRequest,
    #[serde(rename = "model.output")]
    ModelOutput,
    #[serde(rename = "model.measurement")]
    ModelMeasurement,
}

/// What an event carries beside its envelope. Untagged: the envelope's `kind`
/// is the discriminant, because a tag would wrap the spliced message bytes in
/// an object of this crate's making, the double encoding the `RawValue`
/// election exists to avoid. Admission enforces the kind-to-payload pairing,
/// since serde no longer can.
///
/// Derives `Serialize` and not `Deserialize`: this crate never reads an event
/// back, the working structure holding rendered lines, and the asymmetry is a
/// compile property pinned at the crate root.
///
/// The kind-to-payload mapping is total, fourteen kinds and eight
/// dispositions: `load`, `unload`, `session.closed`, and `turn.started` carry
/// no payload; the three message kinds carry `Message`; `turn.closed` carries
/// `TurnClosed`; `fault` carries `Fault`; the three model kinds carry their
/// three own shapes; and the tool bracket's two carry `Deferred`.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Payload {
    /// A message payload the harness pre-rendered, spliced as the bytes stand.
    Message(Box<RawValue>),
    /// The turn's close, the one payload the merged corpus fixes today.
    TurnClosed(TurnClose),
    /// The floor's fault-report shape, rendered by the harness and spliced.
    Fault(Box<RawValue>),
    ModelRequest(ModelRequest),
    ModelOutput(ModelOutput),
    ModelMeasurement(ModelMeasurement),
    /// The payloads whose shapes their own workflows settle, since the trace
    /// act of 2026-08-02 the tool bracket's two alone. Raw bytes in the
    /// interim rather than a placeholder struct, because a struct shaped
    /// against no chartered content would be a reserved slot.
    Deferred(Box<RawValue>),
}

/// How a turn closed. Internally tagged under the two floor Specs' shared
/// test: `Clean` is fieldless and `Stopped` is struct-shaped, rendering
/// `{"close":"clean"}` and `{"close":"stopped","reason":...}`, one shape for
/// both closes.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "close", rename_all = "snake_case")]
pub enum TurnClose {
    Clean,
    Stopped { reason: StopReason },
}

/// Why a stopped turn stopped. A satellite election per `weaver-trace-Spec`
/// section 11: the stop directive is today's one clean interrupt and a fault
/// is the other way a turn ends early.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    Directive,
    Fault,
}

/// The decode boundary, request side: the rendered prompt as the family
/// library produced it and the sampling values, both spliced because their
/// shapes are other crates' - what is shaped here is what no other crate
/// defines.
#[derive(Debug, Clone, Serialize)]
pub struct ModelRequest {
    pub rendered: Box<RawValue>,
    pub template: TemplateId,
    pub sampling: Box<RawValue>,
}

/// The decode boundary, response side: the emission verbatim, before any
/// parse, and how the generation ended.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModelOutput {
    pub emission: String,
    pub finish: Finish,
}

/// How the generation ended, per the charter: completed or stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Finish {
    Completed,
    Stopped,
}

/// The instrument readings. The optional members are absent rather than zero:
/// an unproduced reading emits no member at all, because a zero-length vector
/// rendered would say the reading was taken and found empty, and a zeroed one
/// would say the model was certain, and neither is what an absent instrument
/// means.
#[derive(Debug, Clone, Serialize)]
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
    pub reductions: Option<Box<RawValue>>,
}

/// The chat template's identity. Satellite newtype per section 11.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TemplateId(pub String);

/// The model's identity. Satellite newtype per section 11.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModelId(pub String);

/// The weights hash joining a measurement to the exact artifact.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeightsHash(pub String);

/// One token's identifier in the model's vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TokenId(pub u32);

/// A per-token signal in bits.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Bits(pub f64);

/// One block of the prompt-block partition: a name and the span of the token
/// sequence it covers, end exclusive. Satellite shape per section 11.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PromptBlock {
    pub label: String,
    pub start: u64,
    pub end: u64,
}

/// The decode's timing readings, in nanoseconds under the decimal-string rule.
/// Satellite shape per section 11: the two spans a decode divides into, with
/// further readings arriving when the token workflow measures them.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DecodeTimings {
    pub prefill_ns: MonotonicNs,
    pub decode_ns: MonotonicNs,
}

/// A pre-rendered payload arrives as validated octets: `RawValue` construction
/// validates, so admission's well-formedness check has a mechanism rather than
/// a promise.
pub fn raw_payload(octets: &str) -> Option<Box<RawValue>> {
    RawValue::from_string(octets.to_string()).ok()
}

/// The line type both sinks hold: one rendering, two holders.
pub type Line = Arc<str>;
