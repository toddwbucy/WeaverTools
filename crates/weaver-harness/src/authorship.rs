//! conforms: harness-timestamps-stamped-at-authoring
//! conforms: harness-licensed-combinations-refused-before-submit
//! conforms: harness-refused-submission-not-retried
//! conforms: harness-nothing-waits-on-the-sink
//! conforms: harness-faults-authored-as-events
//! conforms: harness-fault-payload-carried-unchanged
//!
//! Trace authorship, per `weaver-harness-Spec` section 4: the authoring half
//! of `weaver-harness-trace-contract`, one module with one submit path. The
//! harness is the sole author and the recorder never infers anything it was
//! not handed.

use std::time::{Instant, SystemTime, UNIX_EPOCH};

use weaver_trace::{
    Envelope, Event, Failure, Kind, MonotonicNs, Payload, Recorder, RunOrdinal, Sequence,
    SessionRef, Subsystem, TurnRef, raw_payload,
};
use weaver_traits::{ContentBlock, Message, Role};
use weaver_types::{SessionId, TurnKey};

use crate::failure::UnlicensedMessage;

/// The authoring path: holds the run's identity and the origin of its
/// monotonic clock, and stamps both timestamps at authoring from the standard
/// library's two clocks. The recorder's clock is never consulted, the contract
/// denying it the fields.
///
/// The monotonic reading is nanoseconds since the run's origin, an origin only
/// the author holds, so a component stamping its own report would carry a
/// reading placeable in no run.
pub struct Author {
    session: SessionRef,
    run: RunOrdinal,
    origin: Instant,
}

impl Author {
    /// The origin is captured when the `load` event is authored, which is the
    /// first act of the run, so this constructor runs at that moment.
    pub fn new(session: &SessionId, run_ordinal: u64) -> Self {
        Author {
            session: convert_session(session),
            run: RunOrdinal(run_ordinal),
            origin: Instant::now(),
        }
    }

    /// Authors one event and submits it. Both timestamps are stamped here.
    ///
    /// A refusal is not treated as recorded, not projected, and not retried
    /// under a new sequence: a refusal on the authoring path is a defect in
    /// the author, and it surfaces as a fault rather than a retry. Nothing on
    /// any turn path waits on the sink - the working structure's return is the
    /// acknowledgment the interior proceeds on.
    pub fn author(
        &self,
        recorder: &mut Recorder,
        kind: Kind,
        subsystem: Subsystem,
        turn: Option<&TurnKey>,
        payload: Option<Payload>,
    ) -> Result<Sequence, Failure> {
        let event = Event {
            envelope: Envelope {
                session: self.session.clone(),
                run: self.run,
                turn: turn.map(convert_turn),
                // The recorder assigns the run-scoped sequence; a caller
                // cannot author a gapless order.
                sequence: Sequence(0),
                kind,
                subsystem,
                causal_parent: None,
                wall_ms: wall_clock_millis(),
                monotonic_ns: MonotonicNs(self.origin.elapsed().as_nanos() as u64),
            },
            payload,
        };
        recorder.submit(event)
    }

    /// Authors a message event after judging it against the licensing rule.
    /// An unlicensed message is refused by this crate and never submitted,
    /// because the recorder judges the envelope and never the interior.
    pub fn author_message(
        &self,
        recorder: &mut Recorder,
        message: &Message,
        turn: &TurnKey,
    ) -> Result<Result<Sequence, Failure>, UnlicensedMessage> {
        licensed(message)?;
        let kind = match message.role {
            Role::User => Kind::MessageUser,
            Role::Assistant => Kind::MessageAssistant,
            Role::ToolResult => Kind::MessageToolResult,
            // The role set grows with the conversation model, and a role this
            // crate cannot map to an event kind is not authorable.
            _ => {
                return Err(UnlicensedMessage {
                    role: "unmapped",
                    block: "any",
                });
            }
        };
        let rendered = serde_json::to_string(message).map_err(|_| UnlicensedMessage {
            role: "any",
            block: "unrenderable",
        })?;
        let payload = raw_payload(&rendered).ok_or(UnlicensedMessage {
            role: "any",
            block: "unrenderable",
        })?;
        Ok(self.author(
            recorder,
            kind,
            Subsystem::Harness,
            Some(turn),
            Some(Payload::Message(payload)),
        ))
    }

    /// Authors the `fault` event a reported condition becomes. The payload is
    /// the floor's fault report, carried unchanged from the reporting organ
    /// and authored without translation: a fault an organ signalled for itself
    /// would be a record entry no turn key attributes to anything.
    pub fn author_fault(
        &self,
        recorder: &mut Recorder,
        subsystem: Subsystem,
        turn: Option<&TurnKey>,
        report_octets: &str,
    ) -> Result<Sequence, Failure> {
        let payload =
            raw_payload(report_octets)
                .map(Payload::Fault)
                .ok_or(Failure::RefusedOnSubmit {
                    reason: weaver_trace::SubmitRefusal::PayloadMalformed,
                })?;
        self.author(recorder, Kind::Fault, subsystem, turn, Some(payload))
    }
}

/// The licensing rule of `weaver-traits-Spec` section 3: a `User` message
/// carries `Text` blocks, an `Assistant` message carries `Text` and `ToolCall`
/// blocks, a `ToolResult` message carries `ToolResult` blocks, and every other
/// pairing is unlicensed.
pub fn licensed(message: &Message) -> Result<(), UnlicensedMessage> {
    for block in &message.content {
        let ok = matches!(
            (&message.role, block),
            (Role::User, ContentBlock::Text { .. })
                | (Role::Assistant, ContentBlock::Text { .. })
                | (Role::Assistant, ContentBlock::ToolCall(_))
                | (Role::ToolResult, ContentBlock::ToolResult(_))
        );
        if !ok {
            return Err(UnlicensedMessage {
                role: role_name(&message.role),
                block: block_name(block),
            });
        }
    }
    Ok(())
}

fn role_name(role: &Role) -> &'static str {
    match role {
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::ToolResult => "tool_result",
        _ => "unknown",
    }
}

fn block_name(block: &ContentBlock) -> &'static str {
    match block {
        ContentBlock::Text { .. } => "text",
        ContentBlock::ToolCall(_) => "tool_call",
        ContentBlock::ToolResult(_) => "tool_result",
        _ => "unknown",
    }
}

/// The conversion the no-dependency rule of `weaver-trace-Spec` section 1
/// forces, a total function at the one site that submits.
fn convert_session(session: &SessionId) -> SessionRef {
    SessionRef(session.0.clone())
}

fn convert_turn(turn: &TurnKey) -> TurnRef {
    TurnRef(turn.0.clone())
}

fn wall_clock_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
