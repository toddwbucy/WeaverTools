//! conforms: harness-loop-mints-no-port
//! conforms: harness-extension-seam-at-loaded-and-idle
//! conforms: harness-turn-authors-the-model-events
//! conforms: harness-stop-polled-during-the-stream
//!
//! Loop 1's seat and the decode surface it composes, per `weaver-harness-Spec`
//! sections 6 and 6.1. The loop itself is the builder's, written at the worker
//! composition root and compiled into the worker binary. What this crate holds
//! is the seat and the granted surface the loop composes against.
//!
//! **The extension seam is crossed at loaded-and-idle itself**: loop 0 hands a
//! standing interior to whatever loop 1 the binary carries, and takes it back
//! at the stop and at the leave, the bracket discipline being loop 0's for
//! every loop alike. The interior is handed as a borrow, so a loop holds the
//! surface for exactly as long as loop 0 lends it and cannot outlive the run.
//!
//! **The blade is structural.** A port is a type this crate owns with a
//! constructor no consumer can reach, so a loop composes against the granted
//! surface or does not compile. There is no call by which a loop mints a port:
//! [`Ports`] has private fields and a crate-private constructor, and a loop
//! that needs a port this surface does not offer is a capability change
//! entering through the front door as a charter and contract edit.
//!
//! **The turn is loop 0's machinery, granted.** Loop 1 supplies the delta and
//! receives the outcome, and loop 0 drives the append-and-generate exchange,
//! consumes the stream, reads the close, and authors the record. Loop 1
//! authors nothing, which is the sole-writer property held where the turn runs.

use weaver_trace::{Kind, ModelOutput, Payload, Recorder, Subsystem, TurnClose};
use weaver_traits::{ContentBlock, Message, Role};
use weaver_types::{TokenAnswer, TokenDirective, TokenRefusal, TurnKey};

use crate::assembly::Prompt;
use crate::authorship::Author;
use crate::channel::{CoordinationListener, DecodeChannel, OrganChannel};

/// The granted surface handed to loop 1 at loaded-and-idle, borrowing the
/// standing interior for the seat's lifetime. Its fields are private and its
/// only constructor is crate-private, which is the blade.
/// The caller's clock per tool invocation, in milliseconds: the
/// composition root's bound like `MAX_TOOL_ROUNDS`, stated by this caller
/// on every execution per the one-clock rule and adopted by the gate as
/// the kill clock. At or under the shell's declared maximum by
/// construction here, so the refusal arm is for callers less careful.
pub(crate) const TOOL_CALL_CLOCK_MS: u64 = 30_000;

/// How many envelopes one run's shelf may hold before the surplus is
/// dropped and the drop recorded: the bound on both the queue and the
/// serve loop's appetite, a refusal of hoarding rather than of the client.
pub(crate) const MAX_HELD_FRAMES: usize = 64;

/// The gate seam's grant to the turn: the channel the execution exchange
/// crosses, the ordinal source for harness-opened exchanges, and the shelf
/// for envelopes that arrive while an execution is awaited - a client's turn
/// frame crossing mid-execution is held for the serve loop, never dropped
/// and never answered out of order, and bounded at [`MAX_HELD_FRAMES`].
pub(crate) struct GatePort<'a> {
    pub(crate) channel: &'a crate::channel::OrganChannel,
    pub(crate) ordinal: &'a mut u64,
    pub(crate) held: &'a mut std::collections::VecDeque<weaver_types::OrganEnvelope>,
}

pub struct Ports<'a> {
    decode: &'a DecodeChannel,
    author: &'a Author,
    recorder: &'a mut Recorder,
    turn_ordinal: &'a mut u64,
    assembled: Option<Prompt>,
    /// The coordination listener, waited against beside the decode channel
    /// while a generation streams, per Spec 6.1: the stop is heard
    /// mid-stream by `poll`. Private, so the seat carries the ear without
    /// granting the loop a port.
    coordination: &'a CoordinationListener,
    /// The verb connection being served, shared with loop 0's own wait: a
    /// dial accepted mid-stream lands here and is handed back when the
    /// turn returns. `None` for a seat granted outside the serve loop,
    /// which then streams without the ear.
    pending: Option<&'a mut Option<OrganChannel>>,
    /// The gate seam, for the execution exchange. `None` for a seat granted
    /// where no gate stands, whose turns then carry their calls unexecuted:
    /// the assistant's record holds them and no tool-result turn follows,
    /// which is a fact the record shows rather than hides.
    gate: Option<GatePort<'a>>,
    /// The state port, per `weaver-harness-Spec` section 6: the seam's ask
    /// end where the leg stands, and `None` where it does not, which the
    /// port serves as the same absence a missing answer does.
    state: Option<&'a mut crate::state::StateSeam>,
    /// The classify port's seam, per `weaver-harness-Spec` section 6: the
    /// label seam's near end where the arm stands, and `None` where the
    /// declaration carried no binding.
    classify: Option<&'a crate::channel::ClassifyChannel>,
    /// The session's fullness as the last generation carried it, per the
    /// context ports of `weaver-harness-Spec` section 6: written by the
    /// turn from the decode answer, read by the loop before the wall.
    fullness: &'a mut Option<(u64, u64)>,
}

/// Why a turn did not complete. A refusal the seam typed is the session
/// declining the ask, and a fault below the exchange is the worker gone or the
/// octets unreadable, per the decode contract's closure rule.
#[derive(Debug)]
pub enum TurnError {
    /// The decode seam typed a refusal to the append. **Carries the turn**,
    /// which opened before the refusal arrived, so the close a client
    /// receives can name it, per `weaver-gate-world-contract` section 3.
    Refused {
        turn: TurnKey,
        refusal: TokenRefusal,
    },
    /// The channel faulted or answered something the turn cannot read. No
    /// turn key rides it because service ends here and no close is sent.
    ChannelLost,
    /// The SPU's fault emission arrived while this turn streamed. **The
    /// report is already in the record when the caller sees this**: the
    /// engine authors the `fault` event inside the turn's bracket, before
    /// the close its error path lands, because a turn-attributed event
    /// filed after `turn.closed` would sit outside the bracket that names
    /// it. The report rides the error for the close the caller renders,
    /// never for a second authoring. The exchange terminates at the
    /// emission rather than waiting on a frame the contract says will not
    /// come.
    Faulted {
        turn: TurnKey,
        report: weaver_types::FaultReport,
    },
    /// A message loop 1 supplied is not licensed for its role. The bracket
    /// opened before the delta was judged, so the turn exists and is named.
    Unlicensed { turn: TurnKey },
}

/// What a completed turn produced for loop 1: the emission verbatim and how it
/// ended. The record holds more, but this is what the reasoning loop reads to
/// decide its next move.
#[derive(Debug, Clone)]
pub struct TurnOutcome {
    /// The turn this outcome closed, so the close a client receives can name
    /// it, per `weaver-gate-world-contract` section 3. Carried from what the
    /// turn already holds rather than rebuilt: a second construction is a
    /// second chance to disagree with the record.
    pub turn: TurnKey,
    pub emission: String,
    pub stopped: bool,
    /// The turn was aborted by the operator's stop rather than running to
    /// its own close: the bracket closed with the directive's reason and
    /// the partial stands. A model-side stop is not this, per the close's
    /// own distinction.
    pub aborted: bool,
    /// The generation was cut at the turn's token limit, the `Length`
    /// finish of `weaver-types-Spec` section 4.4: its own fact rather than
    /// a case of `stopped`, because overloading the stop flag is the same
    /// conflation the finish vocabulary exists to end. The answered close
    /// surfaces it per the world contract.
    pub truncated: bool,
}

impl<'a> Ports<'a> {
    /// Crate-private: no consumer can reach it, which makes the blade a
    /// compile property. Loop 0 calls it at loaded-and-idle, lending the
    /// interior across the extension seam.
    ///
    /// The arguments are the granted surface itself, one per port, and a
    /// bundling struct would be a second `Ports` wrapping this one, so the
    /// lint is answered rather than obeyed.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn grant(
        decode: &'a DecodeChannel,
        author: &'a Author,
        recorder: &'a mut Recorder,
        turn_ordinal: &'a mut u64,
        assembled: Option<Prompt>,
        coordination: &'a CoordinationListener,
        pending: Option<&'a mut Option<OrganChannel>>,
        gate: Option<GatePort<'a>>,
        state: Option<&'a mut crate::state::StateSeam>,
        classify: Option<&'a crate::channel::ClassifyChannel>,
        fullness: &'a mut Option<(u64, u64)>,
    ) -> Self {
        Ports {
            decode,
            author,
            recorder,
            turn_ordinal,
            assembled,
            coordination,
            pending,
            gate,
            state,
            classify,
            fullness,
        }
    }

    /// The fullness read, per `weaver-harness-Spec` section 6's context
    /// ports: the session's resident token count and its capacity as the
    /// last generation carried them, `None` before any generation. Plain
    /// counts whose meaning is the loop's - when a flush is worth its cost
    /// is the loop's business.
    pub fn fullness(&self) -> Option<(u64, u64)> {
        *self.fullness
    }

    /// The flush, per `weaver-harness-Spec` section 6's context ports:
    /// drives the decode seam's standing flush exchange, valid between
    /// turns, and on confirmation authors the record's `flush` event from
    /// the counts the confirmation carried, the SPU being the one
    /// authority on either number. Since the cut ruling of 2026-08-19 the
    /// call takes `keep`, the resident length the session returns to,
    /// forwarded to the directive unjudged: the seam bounds it below by
    /// the identity prefix and above by the resident count, and the
    /// answered pair says what held. Answers the pair, or `None` where the
    /// seam refused or broke - the next turn will meet the dead seam
    /// properly, so nothing here converts it.
    pub fn flush(&mut self, keep: u64) -> Option<(u64, u64)> {
        self.decode
            .send_directive(&TokenDirective::Flush { keep })
            .ok()?;
        let counts = loop {
            match self.decode.recv_reply().ok()? {
                crate::channel::DecodeReply::Answer(TokenAnswer::Flushed {
                    resident_before,
                    resident_after,
                }) => break (resident_before, resident_after),
                crate::channel::DecodeReply::Answer(TokenAnswer::AtRest) => continue,
                _ => return None,
            }
        };
        self.author
            .author(
                self.recorder,
                Kind::Flush,
                Subsystem::Harness,
                None,
                Some(Payload::Flush(weaver_trace::FlushCounts {
                    resident_before: counts.0,
                    resident_after: counts.1,
                })),
            )
            .ok()?;
        if let Some((_, capacity)) = *self.fullness {
            *self.fullness = Some((counts.1, capacity));
        }
        Some(counts)
    }

    /// The recall ask, per `weaver-harness-state-contract` section 2: the
    /// conversation as custody holds it, in landing order, bounded to the
    /// most recent turns where a bound is given. `None` is the dead peer at
    /// the seat, the same absence a missing leg serves.
    pub fn recall(&mut self, last_turns: Option<u64>) -> Option<Vec<crate::state::Recalled>> {
        self.state.as_mut()?.ask_recall(last_turns)
    }

    /// The state port, per `weaver-harness-Spec` section 6: the shape ask
    /// of `weaver-harness-state-contract` section 2, answered from the
    /// member's holdings, or `None` where the leg is down, the answer
    /// malformed, or the bound expired - each the contract's dead peer
    /// converted into the same absence a missing leg serves. What any
    /// count means to the turn is this caller's business, per the
    /// three-way division.
    pub fn session_shape(&mut self) -> Option<crate::state::SessionShape> {
        self.state.as_mut()?.ask_shape()
    }

    /// The classify port, per `weaver-harness-Spec` section 6: content in,
    /// the artifact's scored labels back in the head's own order, or `None`
    /// where the leg is down, was never declared, refused typed, or
    /// answered malformed - each converted at the seat into the same
    /// absence a missing leg serves. Both sides of the exchange are
    /// authored into the record per the charter's classify pair, the turn
    /// member absent because the seat lends between turns, which is when a
    /// loop holds it to ask.
    pub fn classify(&mut self, content: &str) -> Option<Vec<(String, f64)>> {
        let channel = self.classify?;
        self.author
            .author(
                self.recorder,
                Kind::ClassifyRequest,
                Subsystem::Harness,
                None,
                Some(Payload::ClassifyRequest(weaver_trace::ClassifyAsk {
                    content: content.to_string(),
                })),
            )
            .ok()?;
        let asked = channel.send_directive(&weaver_types::LabelDirective::Classify {
            turn: None,
            content: content.to_string(),
        });
        if asked.is_err() {
            self.author_classify_refused("unsent");
            return None;
        }
        loop {
            match channel.recv_reply() {
                Ok(crate::channel::ClassifyReply::Answer(
                    weaver_types::LabelAnswer::Scored { labels, .. },
                )) => {
                    let scored: Vec<(String, f64)> = labels
                        .into_iter()
                        .map(|scored| (scored.label, scored.score))
                        .collect();
                    self.author
                        .author(
                            self.recorder,
                            Kind::ClassifyOutput,
                            Subsystem::Harness,
                            None,
                            Some(Payload::ClassifyOutput(
                                weaver_trace::ClassifyOutcome::Scored {
                                    labels: scored.clone(),
                                },
                            )),
                        )
                        .ok()?;
                    return Some(scored);
                }
                // A late readiness is not this exchange's answer: skipped,
                // the way the flush skips an interleaved at-rest.
                Ok(crate::channel::ClassifyReply::Answer(weaver_types::LabelAnswer::Ready)) => {
                    continue;
                }
                // The in-flight fault is this exchange's typed answer, per
                // the contract, and the record's fault event carries it by
                // the fault custody rule.
                Ok(crate::channel::ClassifyReply::Answer(weaver_types::LabelAnswer::Fault(
                    report,
                ))) => {
                    let rendered = serde_json::to_string(&report).ok()?;
                    let payload = weaver_trace::raw_payload(&rendered)?;
                    let _ = self.author.author(
                        self.recorder,
                        Kind::Fault,
                        Subsystem::Spu,
                        None,
                        Some(Payload::Fault(payload)),
                    );
                    return None;
                }
                Ok(crate::channel::ClassifyReply::Refusal(refusal)) => {
                    let name = match refusal {
                        weaver_types::LabelRefusal::NotAdmitted { .. } => "not_admitted",
                        weaver_types::LabelRefusal::NotReady => "not_ready",
                        weaver_types::LabelRefusal::Oversized { .. } => "oversized",
                        weaver_types::LabelRefusal::MalformedContent => "malformed_content",
                    };
                    self.author_classify_refused(name);
                    return None;
                }
                Err(_) => return None,
            }
        }
    }

    /// The refused outcome is the record's own fact, per the charter's
    /// eighteenth kind: authored where a typed refusal closed the exchange,
    /// never fabricated into an answer.
    fn author_classify_refused(&mut self, refusal: &str) {
        let _ = self.author.author(
            self.recorder,
            Kind::ClassifyOutput,
            Subsystem::Harness,
            None,
            Some(Payload::ClassifyOutput(
                weaver_trace::ClassifyOutcome::Refused {
                    refusal: refusal.to_string(),
                },
            )),
        );
    }

    /// The prompt loop 0 assembled from the working structure, a read loop 1
    /// has over the granted surface.
    pub fn assembled(&self) -> Option<&Prompt> {
        self.assembled.as_ref()
    }

    /// **Run one turn**, per `weaver-harness-Spec` section 6.1. Loop 1 supplies
    /// the delta as canonical messages, and loop 0 drives the exchange and
    /// authors the record: the turn bracket, the delta as message events, the
    /// three model events across the boundary, and the assistant's turn as a
    /// message event. Loop 1 authors nothing.
    ///
    /// The seam is append-only, so only the delta crosses, the SPU holding the
    /// resident session from the open at enter. The model events splice what
    /// the SPU rendered, the request and the measurement carried opaque and the
    /// output shaped from the emission and finish this crate consumes.
    pub fn turn(&mut self, delta: Vec<Message>) -> Result<TurnOutcome, TurnError> {
        *self.turn_ordinal += 1;
        let turn = TurnKey(format!("t-{}", self.turn_ordinal));

        // The bracket opens. A failure to open it leaves no bracket to close,
        // so it returns before there is anything to unwind.
        self.author
            .author(
                self.recorder,
                Kind::TurnStarted,
                Subsystem::Harness,
                Some(&turn),
                None,
            )
            .map_err(|_| TurnError::ChannelLost)?;

        // **Every exit past the open closes the bracket.** A turn that opened
        // and then lost the channel or met a refusal must not leave a
        // `turn.started` without its `turn.closed`, which a consumer pairing
        // the bracket would read as a turn that never ended. The body runs to
        // its own close on success, and a failure closes with the fault reason
        // before the error returns. The stop slot rides out here so a cancel
        // that crossed before the failure still gets its answer: an exchange
        // the dialer opened is owed a close on every path, and a turn that
        // died of a refusal keeps serving, so a dropped exchange would hold
        // that dialer forever.
        let mut stop: Option<weaver_types::ExchangeId> = None;
        match self.run_turn(&turn, delta, &mut stop) {
            Ok(outcome) => Ok(outcome),
            Err(error) => {
                // **The SPU's report is authored first, inside the bracket.**
                // A `fault` event carrying a turn key must land before that
                // turn's close, or the record shows a closed turn followed by
                // an event attributed to it. The engine holds the author and
                // the recorder, so the report is filed here rather than
                // carried out to a caller who could only file it after the
                // close below has already landed.
                let fault_landed = match &error {
                    TurnError::Faulted {
                        turn: faulted,
                        report,
                    } => self
                        .author
                        .author_fault(self.recorder, Subsystem::Spu, Some(faulted), report)
                        .is_ok(),
                    _ => true,
                };
                // **A failed close is itself a failure the caller must learn.**
                // The recorder can be broken when the turn fails, so authoring
                // the fault close can fail too, and swallowing that would leave
                // a `turn.started` with no `turn.closed` and no error naming it.
                // The original error is returned only when the close lands, and
                // a close that cannot author returns `ChannelLost` because the
                // record is now untrustworthy whatever the first cause was. A
                // report that cannot author is the same untrustworthiness one
                // event earlier, so it takes the same answer, after the close
                // is still attempted for the bracket's sake.
                match self.author.author(
                    self.recorder,
                    Kind::TurnClosed,
                    Subsystem::Harness,
                    Some(&turn),
                    Some(Payload::TurnClosed(TurnClose::Stopped {
                        reason: weaver_trace::StopReason::Fault,
                    })),
                ) {
                    Ok(_) => {
                        // **Announce after record**, on the failure path as on
                        // the clean one: the turn the stop asked to end has
                        // ended, its close in the record naming the fault, and
                        // the interior stands at rest for the dialer's
                        // purpose. Best-effort, because the fault may already
                        // be taking the service down, and closure then signals
                        // what the answer could not.
                        if let Some(exchange) = stop.take()
                            && let Some(slot) = self.pending.as_deref_mut()
                            && let Some(connection) = slot.as_ref()
                        {
                            let _ = connection.send(&weaver_types::OrganEnvelope {
                                exchange,
                                position: weaver_types::Position::Close,
                                payload: weaver_types::Payload::Answer(
                                    weaver_types::LifecycleAnswer::AtRest,
                                ),
                            });
                        }
                        if fault_landed {
                            Err(error)
                        } else {
                            Err(TurnError::ChannelLost)
                        }
                    }
                    Err(_) => Err(TurnError::ChannelLost),
                }
            }
        }
    }

    /// The turn's body, run inside the bracket [`turn`] opens and closes. It
    /// authors the delta, drives the exchange, and authors the answer, and
    /// every error it returns is closed by the caller.
    fn run_turn(
        &mut self,
        turn: &TurnKey,
        delta: Vec<Message>,
        stop: &mut Option<weaver_types::ExchangeId>,
    ) -> Result<TurnOutcome, TurnError> {
        // **A tool-result message in loop 1's delta refuses before anything
        // is authored**, per `weaver-harness-Spec` section 6: the role's one
        // door is the grant's, inside this turn's own execution loop, so a
        // supplied result is a fabrication whatever it carries.
        if delta
            .iter()
            .any(|message| matches!(message.role, Role::ToolResult))
        {
            return Err(TurnError::Unlicensed { turn: turn.clone() });
        }

        // The delta is authored as the turn's user messages, before the
        // exchange so the record reads the ask before the answer.
        for message in &delta {
            self.author
                .author_message(self.recorder, message, turn)
                .map_err(|_| TurnError::Unlicensed { turn: turn.clone() })?
                .map_err(|_| TurnError::ChannelLost)?;
        }

        // **The execution loop**, per the tool workflow's opening act: each
        // round appends a delta and generates, and a generation whose parse
        // recovered calls executes them through the gate, the granted
        // results becoming the next round's delta. The bound refuses further
        // rounds and never the turn: the final emission stands whatever the
        // model still wanted.
        let mut delta = delta;
        let mut rounds = 0usize;
        let generation = loop {
            let generation = self.generate_once(turn, delta, stop)?;
            let calls: Vec<weaver_traits::ToolCall> = generation
                .content
                .iter()
                .filter_map(|block| match block {
                    ContentBlock::ToolCall(call) => Some(call.clone()),
                    _ => None,
                })
                .collect();
            if calls.is_empty() || self.gate.is_none() || rounds >= crate::tools::MAX_TOOL_ROUNDS {
                break generation;
            }
            rounds += 1;
            let mut next_delta = Vec::with_capacity(calls.len());
            for call in calls {
                let grant = self.execute_call(turn, &call)?;
                // The record and the seam read one construction: the grant
                // authors the tool-result event at its one door, and the
                // same content crosses to the SPU as the next delta.
                let message = Message {
                    role: Role::ToolResult,
                    content: vec![ContentBlock::ToolResult(grant.block())],
                };
                self.author
                    .author_tool_result(self.recorder, &grant, turn)
                    .map_err(|_| TurnError::ChannelLost)?;
                next_delta.push(message);
            }
            delta = next_delta;
        };

        let aborted = self.close_turn(turn, &generation, stop)?;
        *self.fullness = Some((generation.resident, generation.capacity));
        Ok(TurnOutcome {
            turn: turn.clone(),
            emission: generation.emission,
            stopped: matches!(generation.finish, weaver_types::Finish::Stopped),
            aborted,
            truncated: matches!(generation.finish, weaver_types::Finish::Length),
        })
    }

    /// **One execution through the gate**, per `weaver-harness-gate-contract`
    /// section 2: the exchange opens with the call as the parse recovered it,
    /// and completes with one of the three contents, the grant constructed
    /// from the completion whichever arrived - the one construction site.
    /// The tool bracket's events ride the exchange: started as the harness's
    /// dispatch, completed as the gate's answer, the deferred payloads
    /// carrying the call and the outcome as this crate renders them.
    fn execute_call(
        &mut self,
        turn: &TurnKey,
        call: &weaver_traits::ToolCall,
    ) -> Result<crate::tools::ToolResult, TurnError> {
        let started = serde_json::json!({
            "name": call.name,
            "arguments": call.arguments,
        })
        .to_string();
        let started = weaver_trace::raw_payload(&started).ok_or(TurnError::ChannelLost)?;
        self.author
            .author(
                self.recorder,
                Kind::ToolCallStarted,
                Subsystem::Harness,
                Some(turn),
                Some(Payload::Deferred(started)),
            )
            .map_err(|_| TurnError::ChannelLost)?;

        let gate = self
            .gate
            .as_mut()
            .expect("execute_call runs only with a gate");
        *gate.ordinal += 1;
        let exchange = weaver_types::ExchangeId {
            opener: weaver_types::Opener::Harness,
            ordinal: *gate.ordinal,
        };
        gate.channel
            .send(&weaver_types::OrganEnvelope {
                exchange: exchange.clone(),
                position: weaver_types::Position::Open,
                payload: weaver_types::Payload::Tool(weaver_types::ToolExecution {
                    name: weaver_types::ToolName(call.name.clone()),
                    arguments: call.arguments.clone(),
                    clock_ms: TOOL_CALL_CLOCK_MS,
                }),
            })
            .map_err(|_| TurnError::ChannelLost)?;

        // Await the completion. A turn frame arriving mid-execution is a
        // client speaking while the turn runs, held for the serve loop per
        // the one-turn discipline; a fault report is held the same way. The
        // exchange identity is the correlation, per the contract: nothing
        // else closes this ordinal.
        let outcome = loop {
            let envelope = gate.channel.recv().map_err(|_| TurnError::ChannelLost)?;
            if envelope.exchange == exchange && envelope.position == weaver_types::Position::Close {
                match envelope.payload {
                    weaver_types::Payload::ToolAnswer(outcome) => break outcome,
                    _ => return Err(TurnError::ChannelLost),
                }
            } else if gate.held.len() >= MAX_HELD_FRAMES {
                // **The shelf is bounded.** A client that keeps speaking
                // through a long execution would otherwise grow it - and the
                // drain's stack with it - without limit. The surplus envelope
                // is dropped and the drop is recorded: the connection that
                // spoke loses its exchange, which is the case the fault
                // names.
                let account =
                    format!("{{\"organ\":\"harness\",\"held-frames-bound\":{MAX_HELD_FRAMES}}}");
                let _ = self.author.author_fault(
                    self.recorder,
                    Subsystem::Harness,
                    Some(turn),
                    &crate::authorship::harness_report(
                        weaver_types::FaultCase::ClientConnectionFailedMidTurn,
                        &account,
                    ),
                );
            } else {
                gate.held.push_back(envelope);
            }
        };

        let completed = serde_json::to_string(&outcome).map_err(|_| TurnError::ChannelLost)?;
        let completed = weaver_trace::raw_payload(&completed).ok_or(TurnError::ChannelLost)?;
        self.author
            .author(
                self.recorder,
                Kind::ToolCallCompleted,
                Subsystem::Gate,
                Some(turn),
                Some(Payload::Deferred(completed)),
            )
            .map_err(|_| TurnError::ChannelLost)?;

        Ok(crate::tools::ToolResult::granted(&outcome))
    }

    /// One append-and-generate round: the delta crosses, the stream is
    /// consumed to the close, and the three model events author at engine
    /// grain.
    fn generate_once(
        &mut self,
        turn: &TurnKey,
        delta: Vec<Message>,
        stop: &mut Option<weaver_types::ExchangeId>,
    ) -> Result<weaver_types::Generation, TurnError> {
        // Append and generate. The delta crosses, the SPU appends it at the
        // resident end, and the stream returns each token before the close.
        self.decode
            .send_directive(&TokenDirective::AppendAndGenerate {
                turn: turn.clone(),
                delta,
            })
            .map_err(|_| TurnError::ChannelLost)?;

        // Consume the stream to the close, hearing the stop while it runs,
        // per Spec 6.1: `poll` sleeps against the decode channel, the
        // coordination listener, and the verb connection if one stands, and
        // wakes on the first ready. A stop dialed mid-stream cancels the
        // turn at the seam, the outstanding generation answering with its
        // partial marked stopped and the tokens already streamed standing
        // in the close. A dialer that connects and then sends nothing holds
        // only its own connection, never the streaming turn.
        let generation = loop {
            match self.stream_wake()? {
                StreamWake::Decode => match self
                    .decode
                    .recv_reply()
                    .map_err(|_| TurnError::ChannelLost)?
                {
                    crate::channel::DecodeReply::Answer(TokenAnswer::Token { .. }) => continue,
                    crate::channel::DecodeReply::Answer(TokenAnswer::Generated(generation)) => {
                        break generation;
                    }
                    crate::channel::DecodeReply::Answer(TokenAnswer::AtRest) => continue,
                    // The emission, matched by name rather than left to a
                    // wildcard that would misfile it as channel loss and
                    // discard the report. It closes no exchange, so this
                    // turn ends without waiting on a frame the contract says
                    // will not come.
                    crate::channel::DecodeReply::Answer(TokenAnswer::Fault(report)) => {
                        return Err(TurnError::Faulted {
                            turn: turn.clone(),
                            report,
                        });
                    }
                    crate::channel::DecodeReply::Refusal(refusal) => {
                        return Err(TurnError::Refused {
                            turn: turn.clone(),
                            refusal,
                        });
                    }
                    crate::channel::DecodeReply::Answer(_) => return Err(TurnError::ChannelLost),
                },
                StreamWake::Dial => {
                    let Some(slot) = self.pending.as_deref_mut() else {
                        continue;
                    };
                    match self.coordination.accept_root() {
                        Ok(connection) => *slot = Some(connection),
                        // A refused peer never reaches an exchange, and the
                        // stream continues.
                        Err(crate::failure::ChannelFault::WrongPeer { .. }) => {}
                        Err(_) => return Err(TurnError::ChannelLost),
                    }
                }
                StreamWake::Directive => {
                    let Some(slot) = self.pending.as_deref_mut() else {
                        continue;
                    };
                    let Some(connection) = slot.as_ref() else {
                        continue;
                    };
                    match connection.recv() {
                        Ok(envelope) => {
                            let exchange = envelope.exchange.clone();
                            match envelope.payload {
                                weaver_types::Payload::Directive(
                                    weaver_types::LifecycleDirective::Stop,
                                ) if stop.is_none() => {
                                    // The cancel crosses at once, and the
                                    // stream is consumed to the close it
                                    // will produce, the answer owed after
                                    // the record, not here.
                                    self.decode
                                        .send_directive(&TokenDirective::Cancel {
                                            turn: turn.clone(),
                                        })
                                        .map_err(|_| TurnError::ChannelLost)?;
                                    *stop = Some(exchange);
                                }
                                // A second stop, a leave, and everything
                                // else are out of order for a turn in
                                // flight, refused and not queued.
                                weaver_types::Payload::Directive(
                                    weaver_types::LifecycleDirective::Leave,
                                ) => {
                                    let _ = connection.send(&weaver_types::OrganEnvelope {
                                        exchange,
                                        position: weaver_types::Position::Close,
                                        payload: weaver_types::Payload::Refusal(
                                            weaver_types::LifecycleRefusal::ActivityNotAtRest,
                                        ),
                                    });
                                }
                                weaver_types::Payload::Directive(_) => {
                                    let _ = connection.send(&weaver_types::OrganEnvelope {
                                        exchange,
                                        position: weaver_types::Position::Close,
                                        payload: weaver_types::Payload::Refusal(
                                            weaver_types::LifecycleRefusal::OutOfOrder,
                                        ),
                                    });
                                }
                                _ => return Err(TurnError::ChannelLost),
                            }
                        }
                        Err(crate::failure::ChannelFault::Closed) => {
                            // The dialer left, and the stream continues.
                            *slot = None;
                        }
                        Err(_) => return Err(TurnError::ChannelLost),
                    }
                }
            }
        };

        // The three model events author across the boundary at engine grain,
        // `SpuDecoder` rather than `Spu`, per issue #103's ruling: the organ
        // will hold more than a decoder and a reader of a model event wants
        // the engine first. Each is spliced or shaped by the custody model:
        // the request and the measurement carried opaque, the output shaped
        // from the emission and finish.
        self.author
            .author(
                self.recorder,
                Kind::ModelRequest,
                Subsystem::SpuDecoder,
                Some(turn),
                Some(Payload::ModelRequest(generation.request.clone())),
            )
            .map_err(|_| TurnError::ChannelLost)?;
        self.author
            .author(
                self.recorder,
                Kind::ModelOutput,
                Subsystem::SpuDecoder,
                Some(turn),
                Some(Payload::ModelOutput(ModelOutput {
                    emission: generation.emission.clone(),
                    // The one conversion site, all three cases carried: an
                    // if on the stopped flag flattened Length into
                    // Completed, which is issue #218's lie at the record.
                    finish: match generation.finish {
                        weaver_types::Finish::Completed => weaver_trace::Finish::Completed,
                        weaver_types::Finish::Stopped => weaver_trace::Finish::Stopped,
                        weaver_types::Finish::Length => weaver_trace::Finish::Length,
                    },
                })),
            )
            .map_err(|_| TurnError::ChannelLost)?;
        self.author
            .author(
                self.recorder,
                Kind::ModelMeasurement,
                Subsystem::SpuDecoder,
                Some(turn),
                Some(Payload::ModelMeasurement(generation.measurement.clone())),
            )
            .map_err(|_| TurnError::ChannelLost)?;

        // The assistant's turn enters the record as a message event, the
        // canonical parse beside the verbatim the output holds: the family
        // module's own bridge crossed the seam in the generation, text as
        // text and every recovered call as a `ToolCall` block, per the tool
        // workflow's opening act.
        let assistant = Message {
            role: Role::Assistant,
            content: generation.content.clone(),
        };
        self.author
            .author_message(self.recorder, &assistant, turn)
            .map_err(|_| TurnError::Unlicensed { turn: turn.clone() })?
            .map_err(|_| TurnError::ChannelLost)?;

        Ok(generation)
    }

    /// **The close names what ended the turn**, and the announce follows the
    /// record. Split from the rounds so one bracket closes however many
    /// generations ran inside it: the turn is the conversation's unit and the
    /// executions are its interior.
    fn close_turn(
        &mut self,
        turn: &TurnKey,
        generation: &weaver_types::Generation,
        stop: &mut Option<weaver_types::ExchangeId>,
    ) -> Result<bool, TurnError> {
        let stopped = matches!(generation.finish, weaver_types::Finish::Stopped);
        // **The close names what ended the turn.** A model-side stop, the
        // generation reaching capacity or its own stop token, is a completed
        // turn whose truncation is recorded in the output's finish and the
        // bracket closes clean. A turn the operator's stop cancelled closes
        // with the directive's reason, the partial standing. The cancel
        // losing the race to a natural completion is the first case: the
        // turn completed, and the stop is answered at rest.
        let aborted = stop.is_some() && stopped;
        self.author
            .author(
                self.recorder,
                Kind::TurnClosed,
                Subsystem::Harness,
                Some(turn),
                Some(Payload::TurnClosed(if aborted {
                    TurnClose::Stopped {
                        reason: weaver_trace::StopReason::Directive,
                    }
                } else {
                    TurnClose::Clean
                })),
            )
            .map_err(|_| TurnError::ChannelLost)?;

        // **Announce after record**: the stop's answer follows the close it
        // reports, carrying the turn's fate, aborted or completed-at-rest,
        // both truthful at the moment of answering.
        if let Some(exchange) = stop.take()
            && let Some(slot) = self.pending.as_deref_mut()
            && let Some(connection) = slot.as_ref()
        {
            let answer = if aborted {
                weaver_types::LifecycleAnswer::TurnAborted { turn: turn.clone() }
            } else {
                weaver_types::LifecycleAnswer::AtRest
            };
            let _ = connection.send(&weaver_types::OrganEnvelope {
                exchange,
                position: weaver_types::Position::Close,
                payload: weaver_types::Payload::Answer(answer),
            });
        }

        Ok(aborted)
    }

    /// One wake from the streaming wait, per Spec 6.1: the decode channel
    /// first, then the verb connection if one stands, then the listener
    /// while none does, the same serial discipline as the idle wait's.
    fn stream_wake(&self) -> Result<StreamWake, TurnError> {
        use std::os::fd::AsFd;

        use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
        loop {
            let mut fds: Vec<PollFd<'_>> = Vec::with_capacity(3);
            let mut wakes: Vec<StreamWake> = Vec::with_capacity(3);
            fds.push(PollFd::new(self.decode.as_fd(), PollFlags::POLLIN));
            wakes.push(StreamWake::Decode);
            match self.pending.as_ref().map(|slot| slot.as_ref()) {
                Some(Some(connection)) => {
                    fds.push(PollFd::new(connection.as_fd(), PollFlags::POLLIN));
                    wakes.push(StreamWake::Directive);
                }
                Some(None) => {
                    fds.push(PollFd::new(self.coordination.as_fd(), PollFlags::POLLIN));
                    wakes.push(StreamWake::Dial);
                }
                // A seat granted outside the serve loop streams without the
                // ear, the slot being the loop's own.
                None => {}
            }
            match poll(&mut fds, PollTimeout::NONE) {
                Ok(_) => {}
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return Err(TurnError::ChannelLost),
            }
            let woken = PollFlags::POLLIN | PollFlags::POLLHUP | PollFlags::POLLERR;
            for (fd, wake) in fds.iter().zip(&wakes) {
                let revents = fd.revents().unwrap_or(PollFlags::empty());
                if revents.contains(PollFlags::POLLNVAL) {
                    return Err(TurnError::ChannelLost);
                }
                if revents.intersects(woken) {
                    return Ok(*wake);
                }
            }
        }
    }
}

/// What the streaming wait woke on.
#[derive(Clone, Copy)]
enum StreamWake {
    Decode,
    Dial,
    Directive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::fd::{AsRawFd, OwnedFd};

    use nix::sys::socket::{AddressFamily, MsgFlags, SockFlag, SockType, recv, send, socketpair};
    use weaver_trace::{Recorder, RunRef, SessionRef};
    use weaver_types::{Finish, Generation, SessionId};

    /// **A turn runs, and loop 0 authors the whole bracket.** Loop 1 supplies
    /// the delta and receives the outcome, and the record carries the turn's
    /// user message, the three model events, the assistant's turn, and the
    /// close, in that order. The decode peer is scripted here rather than a
    /// real SPU: it streams two tokens and then the generation whole, which is
    /// what the turn method consumes and authors.
    #[test]
    fn a_turn_authors_the_whole_bracket() {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        let decode = crate::channel::decode_from_owned(near);

        // The scripted decode peer: read the append, stream two tokens, then
        // answer with the generation whole, the request and measurement the
        // SPU-rendered blobs the turn splices.
        let peer = std::thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv append");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("the append parses");
            assert!(
                matches!(
                    directive,
                    weaver_types::TokenDirective::AppendAndGenerate { .. }
                ),
                "the turn sends append-and-generate"
            );
            let send_answer = |answer: &weaver_types::TokenAnswer| {
                let bytes = serde_json::to_vec(answer).expect("answer renders");
                send(far.as_raw_fd(), &bytes, MsgFlags::empty()).expect("send answer");
            };
            send_answer(&weaver_types::TokenAnswer::Token {
                token: 9707,
                piece: "one".into(),
            });
            send_answer(&weaver_types::TokenAnswer::Token {
                token: 1879,
                piece: " word".into(),
            });
            let request = serde_json::value::RawValue::from_string(
                r#"{"rendered":"<|im_start|>user\nsay a word<|im_end|>\n","template":"qwen2","sampling":{"temperature":0.7}}"#
                    .to_string(),
            )
            .unwrap();
            let measurement = serde_json::value::RawValue::from_string(
                r#"{"model":"qwen2.5","weights_hash":"sha256:abc","input_tokens":[1,2],"output_tokens":[9707,1879],"blocks":[{"label":"turn-delta","start":0,"end":10}],"timings":{"prefill_ns":"10","decode_ns":"20"}}"#
                    .to_string(),
            )
            .unwrap();
            send_answer(&weaver_types::TokenAnswer::Generated(Generation {
                content: vec![weaver_traits::ContentBlock::Text {
                    text: "one word".into(),
                }],
                emission: "one word".into(),
                finish: Finish::Completed,
                resident: 64,
                capacity: 4096,
                request,
                measurement,
            }));
        });

        let session = SessionId("s-1".into());
        let sink = tempfile();
        let mut recorder =
            Recorder::receive(sink, RunRef("r-1".into()), SessionRef(session.0.clone()))
                .expect("recorder");
        let author = Author::new(&session, &weaver_types::RunId("r-1".into()));
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .expect("load");
        let mut turn_ordinal = 0u64;

        let listener = test_listener();
        let outcome = {
            let mut fullness = None;
            let mut ports = Ports::grant(
                &decode,
                &author,
                &mut recorder,
                &mut turn_ordinal,
                None,
                &listener,
                None,
                None,
                None,
                None,
                &mut fullness,
            );
            let delta = vec![Message {
                role: Role::User,
                content: vec![ContentBlock::Text {
                    text: "say a word".into(),
                }],
            }];
            ports.turn(delta).expect("the turn completes")
        };
        peer.join().expect("the decode peer finishes");

        assert_eq!(outcome.emission, "one word", "loop 1 receives the emission");
        assert!(!outcome.stopped, "a clean generation is not stopped");

        // The bracket, in order: the turn's user message, the three model
        // events, the assistant's turn, and the close.
        let kinds: Vec<Kind> = recorder
            .structure()
            .iter()
            .filter(|r| r.turn.is_some())
            .map(|r| r.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                Kind::TurnStarted,
                Kind::MessageUser,
                Kind::ModelRequest,
                Kind::ModelOutput,
                Kind::ModelMeasurement,
                Kind::MessageAssistant,
                Kind::TurnClosed,
            ],
            "the whole bracket authored in order"
        );

        // The model events splice what the peer rendered: the request carries
        // the template, the measurement the weights hash, both opaque.
        let line_of = |kind: Kind| {
            recorder
                .structure()
                .by_kind(kind)
                .next()
                .expect("the event authored")
                .line
                .to_string()
        };
        assert!(
            line_of(Kind::ModelRequest).contains("\"template\":\"qwen2\""),
            "the request splice carries what the SPU rendered"
        );
        assert!(
            line_of(Kind::ModelMeasurement).contains("sha256:abc"),
            "the measurement splice carries what the SPU rendered"
        );

        // The attribution, at engine grain: all three model events carry the
        // decoder's case on the wire, per the #103 ruling. Read from the
        // rendered line rather than the enum so a regression to `Spu` at the
        // author call fails here naming the string, which is the emitter-side
        // half of the pin whose spelling half lives in the recorder's tests.
        for kind in [
            Kind::ModelRequest,
            Kind::ModelOutput,
            Kind::ModelMeasurement,
        ] {
            assert!(
                line_of(kind).contains("\"subsystem\":\"spu_decoder\""),
                "{kind:?} is attributed to the decode engine, got {}",
                line_of(kind)
            );
        }
    }

    /// The emission mid-stream: the report is authored inside the bracket,
    /// before the close. The pin is the order: a turn-attributed `fault`
    /// filed after `turn.closed` would sit outside the bracket that names
    /// it, which is the defect the wrapper's author-then-close sequence
    /// exists to prevent.
    #[test]
    fn a_fault_emission_lands_inside_the_bracket_before_the_close() {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        let decode = crate::channel::decode_from_owned(near);

        // The scripted peer: read the append, stream one token, then emit
        // the fault instead of the generation.
        let peer = std::thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let _ = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv append");
            let send_answer = |answer: &weaver_types::TokenAnswer| {
                let bytes = serde_json::to_vec(answer).expect("answer renders");
                send(far.as_raw_fd(), &bytes, MsgFlags::empty()).expect("send answer");
            };
            send_answer(&weaver_types::TokenAnswer::Token {
                token: 9707,
                piece: "one".into(),
            });
            send_answer(&weaver_types::TokenAnswer::Fault(
                weaver_types::FaultReport {
                    case: weaver_types::FaultCase::DeviceFaultDuringGeneration,
                    account: serde_json::value::RawValue::from_string(
                        r#"{"device":"cuda:0","errored":"mid-forward"}"#.to_string(),
                    )
                    .unwrap(),
                },
            ));
        });

        let session = SessionId("s-1".into());
        let sink = tempfile();
        let mut recorder =
            Recorder::receive(sink, RunRef("r-1".into()), SessionRef(session.0.clone()))
                .expect("recorder");
        let author = Author::new(&session, &weaver_types::RunId("r-1".into()));
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .expect("load");
        let mut turn_ordinal = 0u64;

        let listener = test_listener();
        let error = {
            let mut fullness = None;
            let mut ports = Ports::grant(
                &decode,
                &author,
                &mut recorder,
                &mut turn_ordinal,
                None,
                &listener,
                None,
                None,
                None,
                None,
                &mut fullness,
            );
            let delta = vec![Message {
                role: Role::User,
                content: vec![ContentBlock::Text {
                    text: "say a word".into(),
                }],
            }];
            ports.turn(delta).expect_err("the emission fails the turn")
        };
        peer.join().expect("the decode peer finishes");

        let TurnError::Faulted { turn, report } = error else {
            panic!("the emission is matched by name, got another error");
        };
        assert_eq!(turn.0, "t-1", "the streaming turn is the one named");
        assert_eq!(
            report.case,
            weaver_types::FaultCase::DeviceFaultDuringGeneration,
            "the report rides the error for the caller's close"
        );

        // The pin: the turn-attributed sequence holds the fault before the
        // close, both inside the bracket.
        let kinds: Vec<Kind> = recorder
            .structure()
            .iter()
            .filter(|r| r.turn.is_some())
            .map(|r| r.kind)
            .collect();
        let fault_at = kinds
            .iter()
            .position(|k| *k == Kind::Fault)
            .expect("the report was authored");
        let close_at = kinds
            .iter()
            .position(|k| *k == Kind::TurnClosed)
            .expect("the bracket closed");
        assert!(
            fault_at < close_at,
            "the fault lands inside the bracket, before the close: {kinds:?}"
        );
    }

    /// **A turn executes its calls, and the grant authors the result.** The
    /// whole mechanism in one bracket: the scripted decode peer's first
    /// generation carries a `ToolCall` block, the scripted gate peer answers
    /// the execution exchange with the result, the grant authors the
    /// tool-result turn at its one door, the same content crosses back as
    /// the second delta, and the second generation closes the turn as text.
    ///
    /// What the bracket must read, in order: the user message, the first
    /// model triplet, the assistant turn carrying the call, the tool
    /// bracket's two events, the tool-result message, the second model
    /// triplet, the closing assistant turn, and the close - two generations
    /// inside one turn, which is the loop the tool workflow exists for.
    ///
    /// Perturbation: drop the `execute_call` loop and route calls as plain
    /// text, and this fails at the kinds assertion missing the tool bracket.
    #[test]
    fn a_turn_executes_its_calls_and_the_grant_authors_the_result() {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        let decode = crate::channel::decode_from_owned(near);

        // The decode peer: two rounds. Round one answers a generation whose
        // canonical content carries the call; round two reads the tool-result
        // delta back and answers plain text.
        let peer = std::thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let raw =
                |text: &str| serde_json::value::RawValue::from_string(text.to_string()).unwrap();
            let send_answer = |answer: &weaver_types::TokenAnswer| {
                let bytes = serde_json::to_vec(answer).expect("answer renders");
                send(far.as_raw_fd(), &bytes, MsgFlags::empty()).expect("send answer");
            };

            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv append");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("the append parses");
            assert!(matches!(
                directive,
                weaver_types::TokenDirective::AppendAndGenerate { .. }
            ));
            send_answer(&weaver_types::TokenAnswer::Generated(Generation {
                content: vec![weaver_traits::ContentBlock::ToolCall(
                    weaver_traits::ToolCall {
                        name: "calculator".into(),
                        arguments: r#"{"expression":"37 * 43"}"#.into(),
                    },
                )],
                emission: r#"<tool_call>{"name":"calculator"}</tool_call>"#.into(),
                finish: Finish::Completed,
                resident: 64,
                capacity: 4096,
                request: raw(r#"{"rendered":"r1","template":"qwen2","sampling":{}}"#),
                measurement: raw(
                    r#"{"model":"m","weights_hash":"h","input_tokens":[1],"output_tokens":[2],"blocks":[{"label":"turn-delta","start":0,"end":1}],"timings":{"prefill_ns":"1","decode_ns":"2"}}"#,
                ),
            }));

            // Round two: the tool-result delta arrives, carrying the granted
            // content and nothing the loop invented.
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv round 2");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("round 2 parses");
            let weaver_types::TokenDirective::AppendAndGenerate { delta, .. } = directive else {
                panic!("round two is an append");
            };
            assert_eq!(delta.len(), 1, "one tool-result message crosses");
            assert!(matches!(delta[0].role, Role::ToolResult));
            assert!(
                matches!(
                    &delta[0].content[0],
                    ContentBlock::ToolResult(block) if block.content == "1591"
                ),
                "the granted content is what crosses: {:?}",
                delta[0].content
            );
            send_answer(&weaver_types::TokenAnswer::Generated(Generation {
                content: vec![weaver_traits::ContentBlock::Text {
                    text: "37 * 43 is 1591.".into(),
                }],
                emission: "37 * 43 is 1591.".into(),
                finish: Finish::Completed,
                resident: 64,
                capacity: 4096,
                request: raw(r#"{"rendered":"r2","template":"qwen2","sampling":{}}"#),
                measurement: raw(
                    r#"{"model":"m","weights_hash":"h","input_tokens":[3],"output_tokens":[4],"blocks":[{"label":"turn-delta","start":0,"end":1}],"timings":{"prefill_ns":"1","decode_ns":"2"}}"#,
                ),
            }));
        });

        // The gate peer: one execution exchange, answered with the result,
        // exactly as the gate's dispatch would.
        let (gate_near, gate_child) = crate::channel::OrganChannel::pair().expect("gate pair");
        let gate_peer = std::thread::spawn(move || {
            let gate_far = gate_child.into_channel();
            let envelope = gate_far.recv().expect("the execution opens");
            assert_eq!(envelope.position, weaver_types::Position::Open);
            let weaver_types::Payload::Tool(execution) = &envelope.payload else {
                panic!("the exchange carries the call, got {:?}", envelope.payload);
            };
            assert_eq!(execution.name.0, "calculator");
            let outcome = weaver_gate_execute_stub(execution);
            gate_far
                .send(&weaver_types::OrganEnvelope {
                    exchange: envelope.exchange.clone(),
                    position: weaver_types::Position::Close,
                    payload: weaver_types::Payload::ToolAnswer(outcome),
                })
                .expect("the answer closes");
        });

        let session = SessionId("s-t".into());
        let sink = tempfile();
        let mut recorder =
            Recorder::receive(sink, RunRef("r-1".into()), SessionRef(session.0.clone()))
                .expect("recorder");
        let author = Author::new(&session, &weaver_types::RunId("r-1".into()));
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .expect("load");
        let mut turn_ordinal = 0u64;
        let mut gate_ordinal = 0u64;
        let mut held = std::collections::VecDeque::new();

        let listener = test_listener();
        let outcome = {
            let mut fullness = None;
            let mut ports = Ports::grant(
                &decode,
                &author,
                &mut recorder,
                &mut turn_ordinal,
                None,
                &listener,
                None,
                Some(GatePort {
                    channel: &gate_near,
                    ordinal: &mut gate_ordinal,
                    held: &mut held,
                }),
                None,
                None,
                &mut fullness,
            );
            let delta = vec![Message {
                role: Role::User,
                content: vec![ContentBlock::Text {
                    text: "what is 37 * 43?".into(),
                }],
            }];
            ports.turn(delta).expect("the turn completes")
        };
        peer.join().expect("the decode peer finishes");
        gate_peer.join().expect("the gate peer finishes");

        assert_eq!(outcome.emission, "37 * 43 is 1591.");
        assert!(held.is_empty(), "nothing crossed mid-execution to hold");

        let kinds: Vec<Kind> = recorder
            .structure()
            .iter()
            .filter(|r| r.turn.is_some())
            .map(|r| r.kind)
            .collect();
        assert_eq!(
            kinds,
            vec![
                Kind::TurnStarted,
                Kind::MessageUser,
                Kind::ModelRequest,
                Kind::ModelOutput,
                Kind::ModelMeasurement,
                Kind::MessageAssistant,
                Kind::ToolCallStarted,
                Kind::ToolCallCompleted,
                Kind::MessageToolResult,
                Kind::ModelRequest,
                Kind::ModelOutput,
                Kind::ModelMeasurement,
                Kind::MessageAssistant,
                Kind::TurnClosed,
            ],
            "two generations inside one bracket, the tool loop between them"
        );

        // The attribution: started is the harness's dispatch, completed is
        // the gate's answer.
        let line_of = |kind: Kind| {
            recorder
                .structure()
                .by_kind(kind)
                .next()
                .expect("authored")
                .line
                .to_string()
        };
        assert!(line_of(Kind::ToolCallStarted).contains("\"subsystem\":\"harness\""));
        assert!(line_of(Kind::ToolCallCompleted).contains("\"subsystem\":\"gate\""));
        assert!(
            line_of(Kind::MessageToolResult).contains("1591"),
            "the granted content is the record's"
        );
    }

    /// **A fabricated tool result in loop 1's delta refuses before anything
    /// authors**, per `weaver-harness-Spec` section 6: the role's one door is
    /// the grant's, and a supplied result is a fabrication whatever it
    /// carries. What this reads is the refusal arriving before the record
    /// gains a single event of the turn's interior.
    ///
    /// Perturbation: the door is doubled on purpose - `run_turn` refuses the
    /// role and `author_message` refuses it again - so the watch drops both:
    /// remove the role check at the top of `run_turn` and the refusal arm in
    /// `author_message`, and this fails with `ChannelLost`, the fabricated
    /// delta then authoring and the turn reaching a decode seam this test
    /// deliberately closed. Watched under exactly that pair.
    #[test]
    fn a_supplied_tool_result_refuses_at_the_door() {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        let decode = crate::channel::decode_from_owned(near);
        // Closed on purpose: a perturbed turn that authored the fabrication
        // would reach the decode seam, and a closed seam fails it fast
        // instead of hanging the suite on a peer that never answers.
        drop(far);
        let session = SessionId("s-f".into());
        let sink = tempfile();
        let mut recorder =
            Recorder::receive(sink, RunRef("r-1".into()), SessionRef(session.0.clone()))
                .expect("recorder");
        let author = Author::new(&session, &weaver_types::RunId("r-1".into()));
        let mut turn_ordinal = 0u64;
        let listener = test_listener();
        let mut fullness = None;
        let mut ports = Ports::grant(
            &decode,
            &author,
            &mut recorder,
            &mut turn_ordinal,
            None,
            &listener,
            None,
            None,
            None,
            None,
            &mut fullness,
        );
        let delta = vec![Message {
            role: Role::ToolResult,
            content: vec![ContentBlock::ToolResult(weaver_traits::ToolResultBlock {
                content: "fabricated".into(),
            })],
        }];
        assert!(
            matches!(ports.turn(delta), Err(TurnError::Unlicensed { .. })),
            "the supplied result refuses at the door"
        );
    }

    /// A calculator-shaped stub for the scripted gate peer, so this test does
    /// not depend on the gate crate: the answer is what the real gate's
    /// dispatch would produce for this call.
    fn weaver_gate_execute_stub(
        execution: &weaver_types::ToolExecution,
    ) -> weaver_types::ToolOutcome {
        assert_eq!(execution.arguments, r#"{"expression":"37 * 43"}"#);
        weaver_types::ToolOutcome::Result {
            content: "1591".into(),
        }
    }

    /// **The stop is heard mid-stream, per Spec 6.1.** The streaming wait
    /// spans the decode channel and the verb connection at once, the stop
    /// cancels the turn at the seam, the partial stands in the close with
    /// the directive's reason, and the stop is answered with the turn's
    /// fate after the record. Perturbation: collapse the streaming wait to
    /// the decode channel alone and the cancel this scripted peer waits
    /// for never arrives.
    #[test]
    fn a_stop_dialed_mid_stream_cancels_the_turn() {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        let decode = crate::channel::decode_from_owned(near);
        let listener = test_listener();
        let (verb_end, admin_end) = crate::channel::OrganChannel::pair().expect("pair");
        let admin = admin_end.into_channel();

        // The operator's stop is already on the connection when the turn
        // begins, the way the poll's readiness would deliver it mid-stream.
        admin
            .send(&weaver_types::OrganEnvelope {
                exchange: weaver_types::ExchangeId {
                    opener: weaver_types::Opener::Admin,
                    ordinal: 9,
                },
                position: weaver_types::Position::Open,
                payload: weaver_types::Payload::Directive(weaver_types::LifecycleDirective::Stop),
            })
            .expect("the stop sends");

        let peer = std::thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv append");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("the append parses");
            assert!(matches!(
                directive,
                weaver_types::TokenDirective::AppendAndGenerate { .. }
            ));
            // The cancel arrives because the wait heard the stop.
            let n = recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv cancel");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("the cancel parses");
            assert!(
                matches!(directive, weaver_types::TokenDirective::Cancel { .. }),
                "the stop cancels at the seam"
            );
            let send_answer = |answer: &weaver_types::TokenAnswer| {
                let bytes = serde_json::to_vec(answer).expect("answer renders");
                send(far.as_raw_fd(), &bytes, MsgFlags::empty()).expect("send answer");
            };
            send_answer(&weaver_types::TokenAnswer::Token {
                token: 7,
                piece: "par".into(),
            });
            let request = serde_json::value::RawValue::from_string(
                r#"{"rendered":"r","template":"t","sampling":{}}"#.to_string(),
            )
            .unwrap();
            let measurement = serde_json::value::RawValue::from_string(
                r#"{"model":"m","weights_hash":"h","input_tokens":[1],"output_tokens":[7],"blocks":[],"timings":{"prefill_ns":"1","decode_ns":"2"}}"#
                    .to_string(),
            )
            .unwrap();
            send_answer(&weaver_types::TokenAnswer::Generated(Generation {
                content: vec![],
                emission: "par".into(),
                finish: Finish::Stopped,
                resident: 64,
                capacity: 4096,
                request,
                measurement,
            }));
        });

        let session = SessionId("s-2".into());
        let sink = tempfile();
        let mut recorder =
            Recorder::receive(sink, RunRef("r-1".into()), SessionRef(session.0.clone()))
                .expect("recorder");
        let author = Author::new(&session, &weaver_types::RunId("r-1".into()));
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .expect("load");
        let mut turn_ordinal = 0u64;
        let mut slot = Some(verb_end);

        let outcome = {
            let mut fullness = None;
            let mut ports = Ports::grant(
                &decode,
                &author,
                &mut recorder,
                &mut turn_ordinal,
                None,
                &listener,
                Some(&mut slot),
                None,
                None,
                None,
                &mut fullness,
            );
            let delta = vec![Message {
                role: Role::User,
                content: vec![ContentBlock::Text {
                    text: "stop me".into(),
                }],
            }];
            ports.turn(delta).expect("the aborted turn still returns")
        };
        peer.join().expect("the decode peer finishes");

        assert!(outcome.aborted, "the directive aborted the turn");
        assert_eq!(outcome.emission, "par", "the partial stands");

        // Announce after record: the answer carries the turn's fate.
        match admin.recv().expect("the stop's answer").payload {
            weaver_types::Payload::Answer(weaver_types::LifecycleAnswer::TurnAborted { turn }) => {
                assert_eq!(turn.0, "t-1");
            }
            other => panic!("the stop answers the turn's fate, got {other:?}"),
        }

        // The close names the directive, the partial standing before it.
        let close = recorder
            .structure()
            .by_kind(Kind::TurnClosed)
            .next()
            .expect("the close authored")
            .line
            .to_string();
        assert!(
            close.contains(r#""close":"stopped""#) && close.contains(r#""reason":"directive""#),
            "the close names the directive, not the fault: {close}"
        );
    }

    fn test_listener() -> crate::channel::CoordinationListener {
        let dir = std::env::temp_dir().join(format!(
            "weaver-engine-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let path = dir.join("c.sock");
        std::fs::remove_file(&path).ok();
        crate::channel::bind_coordination(&path).expect("bind")
    }

    fn tempfile() -> OwnedFd {
        let path = std::env::temp_dir().join(format!(
            "weaver-harness-turn-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let file = std::fs::File::create(&path).expect("sink");
        std::fs::remove_file(&path).ok();
        OwnedFd::from(file)
    }
}
