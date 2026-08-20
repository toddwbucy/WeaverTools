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
    pub run: RunRef,
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

/// The run's identity within its session, carried without interpretation.
///
/// **Not an ordinal.** It is minted by the party that performs the load and
/// is distinct within its session rather than positioned in a sequence, per
/// `weaver-admin-PRD` section 10. This crate neither mints it nor reads it,
/// which is why the change is a rename here and an election elsewhere. It
/// stops being `Copy` with the shape: a string is owned, and a run reference
/// crosses this crate once per submit rather than in a loop, so nothing here
/// wanted the copy.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RunRef(pub String);

/// The turn's identity as the harness converts it, the join key carried and
/// never invented.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TurnRef(pub String);

/// The producing party, at the granularity a reader needs.
///
/// Four cases are the crates that can produce a report. `Tool` is a producing
/// party and not a crate, because a tool's result reaches the record through
/// the harness and a record attributing it to the harness would lose the one
/// fact an operator reading a tool result wants first. **`SpuDecoder` is that
/// argument one level down**, per `weaver-trace-Spec` section 3: the SPU's
/// domain is every semantic operation in the text modality, so an event
/// stamped `Spu` loses which engine produced it once more than one lives
/// behind the organ. The three model events carry it; residency, admit,
/// release, and fault attribution stay `Spu`, being the organ's rather than
/// any engine's.
///
/// **A case arrives per producing party at the granularity a reader needs,
/// with its first emitter, a floor edit in the same act.** The encoder's
/// case, `spu_encoder`, is deliberately absent until the act that builds an
/// encoder gives it an emitter: a case with no producer is a reserved slot in
/// enum form, per apex section 9.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Subsystem {
    Admin,
    Harness,
    Spu,
    SpuDecoder,
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
    #[serde(rename = "message.system")]
    MessageSystem,
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
    #[serde(rename = "flush")]
    Flush,
    #[serde(rename = "model.request")]
    ModelRequest,
    #[serde(rename = "model.output")]
    ModelOutput,
    #[serde(rename = "model.measurement")]
    ModelMeasurement,
    #[serde(rename = "classify.request")]
    ClassifyRequest,
    #[serde(rename = "classify.output")]
    ClassifyOutput,
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
    /// The floor's fault-report shape: the reporting organ renders its
    /// account and names its case, the harness serializes the report whole
    /// and splices it, per apex section 5.2's custody rule. Formerly
    /// "rendered by the harness", which was true of the code until the act
    /// that shaped the report, and corrected in that act rather than ahead
    /// of it.
    Fault(Box<RawValue>),
    /// The request the model received, the SPU-rendered content the harness
    /// splices, per the custody act of 2026-08-11: the rendered prompt with
    /// its template and effective sampling, the organ's shape the record
    /// carries opaque.
    ModelRequest(Box<RawValue>),
    ModelOutput(ModelOutput),
    /// The flush's account: the resident token counts before and after the
    /// decode context returned to its prefix, per charter section 3.1's
    /// sixteenth kind. Both from the SPU's confirmation, the one authority
    /// on either number.
    Flush(FlushCounts),
    /// The instrument readings, the SPU-rendered measurement the harness
    /// splices, its unproduced members produced absent by the SPU rather than
    /// omitted by a serde election of this crate's.
    ModelMeasurement(Box<RawValue>),
    /// The label seam's request side, shaped on the flush's precedent:
    /// plain small data the harness authors from typed wire answers, per
    /// charter section 3.1's seventeenth kind.
    ClassifyRequest(ClassifyAsk),
    /// The label seam's response side: scored or refused, so a typed
    /// refusal the exchange met is the record's own fact and never a
    /// fabricated answer, per charter section 3.1's eighteenth kind.
    ClassifyOutput(ClassifyOutcome),
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
/// The decode boundary, response side: the emission verbatim, before any
/// parse, and how the generation ended.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FlushCounts {
    pub resident_before: u64,
    pub resident_after: u64,
}

/// The classify ask's account: the content the loop sent, per
/// `weaver-trace-Spec` section 3 as of the classifier act.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ClassifyAsk {
    pub content: String,
}

/// The classify answer's account: every label of the artifact's head
/// scored, or the typed refusal the exchange met, named. Tagged the way
/// `TurnClose` is, one shape for both outcomes.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ClassifyOutcome {
    Scored { labels: Vec<(String, f64)> },
    Refused { refusal: String },
}

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
    /// The turn's token limit reached, and that limit alone: this crate's
    /// mirror of the floor's case, converted by the harness at the one
    /// site, per `weaver-trace-Spec` section 3.
    Length,
}

/// The measurement and the request retired their typed shapes with the custody
/// act of 2026-08-11: both are the SPU's rendering the record carries opaque,
/// so their satellites, the model and template identities, the weights hash,
/// the token identifier, the per-token bits, the prompt block, and the decode
/// timings, are the SPU's construction shape and no longer this crate's, per
/// `weaver-trace-Spec` section 3. `ModelOutput` alone stays shaped, its
/// emission a string the harness consumes and its finish the turn's close.
///
/// A pre-rendered payload arrives as validated octets: `RawValue` construction
/// validates JSON, and the separator check ahead of it closes the hole
/// validation leaves - JSON holds no raw newline inside a string, but holds
/// them freely between tokens, so pretty-printed octets are valid JSON that
/// would split the frame. A legitimately escaped newline inside a string is
/// the two octets `\` `n`, so scanning for the raw byte has zero false
/// positives on well-formed payloads. The render path holds the same check as
/// the backstop for a `RawValue` built any other way.
pub fn raw_payload(octets: &str) -> Option<Box<RawValue>> {
    if octets.contains('\n') || octets.contains('\r') {
        return None;
    }
    RawValue::from_string(octets.to_string()).ok()
}

/// The line type both sinks hold: one rendering, two holders.
pub type Line = Arc<str>;
