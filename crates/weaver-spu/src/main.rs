//! conforms: spu-service-serial-one-loop
//! conforms: spu-out-of-order-refused-on-residency
//! conforms: spu-out-of-order-refused-on-decode
//! conforms: spu-fault-below-the-exchange-layer
//!
//! Entry, the hygiene sets, and the service loop, per `weaver-spu-Spec`
//! sections 2, 3, and 9.
//!
//! **The service is serial per channel and the two channels are one loop.** One
//! lifecycle directive at a time, one decode exchange at a time, per both
//! contracts' ordering. Nothing here is concurrent in this pass, and the shape
//! that would change it is the executor election deferred in Spec section 1.1.
//!
//! **The order is judged against the channel's recorded position before the
//! directive reaches residency,** per Spec section 9, which is what makes
//! not-queued mean anything at all: a refusal that had already run the work
//! would be a refusal about nothing. The residency seam has three positions and
//! the last is terminal: before-admit, admitted, released. A release before any
//! admit answers `OutOfOrder`, a second admit answers the same whatever the
//! first answered, and any directive after a release answers the same.
//! conforms: spu-session-parameters-carry-dispositions

use std::process::ExitCode;

use weaver_spu::channel::{
    self, ChannelFault, DecodeSocket, EntryFault, Inherited, LifecycleChannel,
};
use weaver_spu::decoder::backend::{DecodeFault, TokenId};
use weaver_spu::decoder::session::{CancelPoll, Session, StopCondition, Stopped};
use weaver_spu::family;
use weaver_spu::readout::ReadoutElection;
use weaver_spu::residency::{Headroom, Residency, Resident, StopSet};
use weaver_spu::sampling::{
    Disposition, EffectiveKnobs, EffectiveSessionParameters, KnobRefusal, Knobs, SessionParameters,
    TunableValues,
};
use weaver_traits::Message;
use weaver_types::{
    ExchangeId, Finish, Generation, LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Opener,
    OrganEnvelope, Payload, Position, TokenAnswer, TokenDirective, TokenRefusal,
};

/// The headroom the worker's composition root supplies. A deployment fact
/// rather than an operator election, per Spec section 3, and a number a builder
/// can supply before the measurement that replaces it exists.
const HEADROOM_BYTES: u64 = 512 * 1024 * 1024;

/// The residency seam's recorded position, per Spec section 9 and
/// `weaver-harness-spu-contract` section 3. The order is judged here, before a
/// directive reaches residency, and the released position is terminal.
///
/// A refused admit still spends the one admission: a second admit answers
/// `OutOfOrder` whatever the first answered, this crate admitting once and
/// dying rather than matching a prior residency against a later request.
#[derive(Debug, Clone, Copy, PartialEq)]
enum SeamPosition {
    /// No admit has arrived. Only an admit is in order.
    BeforeAdmit,
    /// An admit arrived and was refused. Nothing further is in order: the
    /// process holds no residency and will accept no second attempt.
    AdmitRefused,
    /// The residency stands. Only a release is in order.
    Admitted,
    /// Terminal. A directive of any kind arriving after a release answers
    /// `OutOfOrder`.
    Released,
}

/// The decode seam's recorded position, per Spec section 9 and
/// `weaver-harness-spu-decode-contract` section 3. The seam holds the richer
/// state and the same discipline: the order is judged here, before a directive
/// reaches the session.
///
/// The mid-generation cases the contract names, a second append or a flush
/// while a generation is in flight, are not positions of this machine: the
/// service is serial, so a generation runs inside the handling of its own
/// exchange and the only reader mid-flight is the cancel poll, which refuses
/// what it polls up that is not a cancel. What this machine holds is the
/// between-exchanges state.
#[derive(Debug, Clone, Copy, PartialEq)]
enum DecodePosition {
    /// No open has arrived. Only an open is in order, and only once the
    /// residency it serves is confirmed on the lifecycle seam.
    BeforeOpen,
    /// The session stands at rest. Append-and-generate, cancel answering at
    /// rest, and flush are in order. A second open is not: the contract has
    /// open first and once, and rewinding is what the flush is for.
    AtRest,
}

/// The ordering judgment for the decode seam, pure and total: a refusal, or
/// the directive is in order for the position. Judged before the session is
/// reached, which is what makes not-queued mean anything, per Spec section 9
/// on both seams alike.
///
/// **No sampling value reaches this seam to be judged.** The tunable map left
/// the directive when the values moved to the declaration, per
/// `weaver-spu-Spec` section 8, so what this judges is position and order
/// alone. The finiteness check that rode here went with the map: the floor
/// judges it at parse and this crate judges a value against its parameter at
/// resolve, both before any device work.
fn judge_decode(
    position: DecodePosition,
    residency_confirmed: bool,
    directive: &TokenDirective,
) -> Option<TokenRefusal> {
    match (position, directive) {
        (DecodePosition::BeforeOpen, TokenDirective::Open { .. }) => {
            if residency_confirmed {
                None
            } else {
                // Open is valid only after the residency it serves is
                // confirmed, one contract's ordering read against the other's.
                Some(TokenRefusal::OutOfOrder)
            }
        }
        // The session is not open for the ask.
        (
            DecodePosition::BeforeOpen,
            TokenDirective::AppendAndGenerate { .. }
            | TokenDirective::Cancel { .. }
            | TokenDirective::Flush { .. },
        ) => Some(TokenRefusal::NotOpen),
        // A second open refuses rather than rewinding.
        (DecodePosition::AtRest, TokenDirective::Open { .. }) => Some(TokenRefusal::OutOfOrder),
        (DecodePosition::AtRest, TokenDirective::AppendAndGenerate { .. }) => None,
        (DecodePosition::AtRest, TokenDirective::Cancel { .. } | TokenDirective::Flush { .. }) => {
            None
        }
    }
}

/// **This binary's elections, per charter section 13.8 and `weaver-spu-Spec`
/// section 8.** Each parameter is `Frozen` with its value compiled in, or
/// `OperatorTunable` and read from `decoder.tunable_values` at admit. The
/// election is the builder's and the composition root states it rather than
/// hiding it, so changing one is this line and a recompile.
///
/// This deployment freezes every one. A deployment iterating on an agent flips
/// the parameters it is moving to `OperatorTunable` and sets them in
/// `agent.yaml`, which is the loop the disposition mechanism exists for, and a
/// production build freezes them back so no declaration can move them.
const KNOBS: Knobs = Knobs {
    temperature: Disposition::Frozen(0.7),
    top_k: Disposition::Frozen(40),
    top_p: Disposition::Frozen(0.95),
    repetition_penalty: Disposition::Frozen(1.1),
    repetition_window: Disposition::Frozen(64),
    seed: Disposition::Frozen(11),
};

/// The context capacity and the per-turn generation ceiling, elected the same
/// way. The ceiling is the stop condition's backstop for a model that never
/// emits a stop token.
///
/// **The ceiling is operator-tunable as of 2026-08-19, per issue #218**:
/// the frozen 512 was the first outside consumer's first wall, a code
/// answer needing four to eight times that, so the election moves to the
/// declaration where the deployment sizes it. A declaration must now
/// supply `max-tokens-per-turn` in `tunable-values` or the load refuses by
/// name, which is the explicitness the tunable route carries.
///
/// The capacity unfroze on the operator's ruling of 2026-08-19, issue
/// #221: the frozen 4096 was the wall both of that issue's faults hit,
/// while the declaration's stated capacity had been silently ignored
/// under the frozen rule since it was written. The declaration now
/// governs, sized by the operator against the KV cache's device
/// footprint, 32768 being the ruled starting point on this deployment.
const SESSION_PARAMETERS: SessionParameters = SessionParameters {
    context_capacity: Disposition::OperatorTunable,
    max_tokens_per_turn: Disposition::OperatorTunable,
};

/// What this load runs with, resolved once from the declaration.
#[derive(Debug, Clone, Copy)]
struct Effective {
    knobs: EffectiveKnobs,
    session: EffectiveSessionParameters,
}

/// Resolve both sets against what the declaration supplied.
///
/// **Called before the admit takes a device.** A parameter this binary left
/// tunable with nothing supplied for it, or a count supplied as a fraction or
/// a negative or something past its type, refuses the load naming the
/// parameter rather than reaching an engine, and refusing before the admit
/// keeps a declaration's mistake from costing device work.
fn resolve_effective(supplied: &TunableValues) -> Result<Effective, KnobRefusal> {
    Ok(Effective {
        knobs: KNOBS.resolve(supplied)?,
        session: SESSION_PARAMETERS.resolve(supplied)?,
    })
}

/// Render a turn's delta through the family, text blocks only.
///
/// **The turns and nothing around them.** The identity prefix goes through
/// [`family::Family::render_identity`] instead, because a family whose prefix
/// opens with a preamble its turns do not repeat renders that there and only
/// there. A block or a role the family cannot render is the delta malformed for
/// the family, which is the contract's own case rather than one invented here:
/// the tool shapes are blocked with the tool workflow and the families render
/// text today.
///
/// **This function used to hold the role names and the text flattening for
/// every family at once.** It does not, because a role's wire name is a family
/// fact and this is the composition root: `gemma4` calls the assistant `model`,
/// and a map living here would have taught every other family the word or
/// grown an arm naming one, which Spec section 5's placement rule forbids.
fn render_delta(family: &dyn family::Family, delta: &[Message]) -> Result<String, TokenRefusal> {
    family::render_each(family, delta).map_err(TokenRefusal::from)
}

/// One frame out on the decode seam: the trio crosses as bare JSON, per
/// `weaver-types-Spec` section 4.4's provisional encoding.
fn send_answer(decode: &DecodeSocket, answer: &TokenAnswer) -> Result<(), ChannelFault> {
    let body = serde_json::to_vec(answer).map_err(|_| ChannelFault::Undecodable)?;
    decode.send_octets(&body)
}

fn send_refusal(decode: &DecodeSocket, refusal: &TokenRefusal) -> Result<(), ChannelFault> {
    let body = serde_json::to_vec(refusal).map_err(|_| ChannelFault::Undecodable)?;
    decode.send_octets(&body)
}

/// A decode fault the seam cannot answer dies loudly: the fault exchange's
/// payload shape is the open election the harness lane gathers, so until it
/// lands, the contract's own closure rule is the signal, a closed channel
/// with an exchange outstanding being that exchange's failure and never its
/// success. The line on standard error is the operator's account of why.
fn decode_fault_line(fault: &DecodeFault) -> String {
    serde_json::json!({"decode_fault": format!("{fault:?}")}).to_string()
}

/// The cancel poll over the decode socket, per Spec section 4.3: between
/// tokens the loop asks the channel rather than waiting on it. A cancel
/// stops the generation at the boundary. **Any other directive polled up
/// mid-flight is out of order for a seam with a generation outstanding and
/// is refused right here, not queued**, which is where the contract's
/// mid-generation cases live in a serial service. A channel fault mid-flight
/// marks the poll fatal: the generation stops and the phase dies with the
/// exchange open, closure with an exchange outstanding being that exchange's
/// failure per the contract's own rule, and an answer on a channel the poll
/// just faulted would be a promise the channel cannot keep.
struct SeamCancel<'a> {
    socket: &'a DecodeSocket,
    cancelled: bool,
    fatal: bool,
}

impl CancelPoll for SeamCancel<'_> {
    fn cancelled(&mut self) -> bool {
        loop {
            match self.socket.try_recv_octets() {
                Ok(None) => return false,
                Ok(Some(frame)) => match serde_json::from_slice::<TokenDirective>(&frame) {
                    Ok(TokenDirective::Cancel { .. }) => {
                        self.cancelled = true;
                        return true;
                    }
                    Ok(_) => {
                        if send_refusal(self.socket, &TokenRefusal::OutOfOrder).is_err() {
                            self.fatal = true;
                            return true;
                        }
                    }
                    Err(_) => {
                        self.fatal = true;
                        return true;
                    }
                },
                Err(_) => {
                    self.fatal = true;
                    return true;
                }
            }
        }
    }
}

/// What stands once the open exchange succeeds: the session and the two
/// family facts every later exchange reads.
struct OpenedSession<'r> {
    session: Session<'r>,
    renderer: &'static dyn family::Family,
    /// **Carried for the record and not for the render.** The measurement
    /// names the template that rendered the prompt, and this is the string it
    /// names. Nothing renders through it here: that is [`Self::renderer`]'s,
    /// and a template identity worth the name is issues #88 and #129's.
    template: &'static str,
    generation_opener: &'static str,
    /// **The family's stop conditions, promoted against this artifact.** Built
    /// once at open, because the vocabulary cannot change under a residency and
    /// a set rebuilt per turn would be the same answer at a per-turn cost.
    stop: StopSet,
}

/// The measurement, rendered by this organ to the shape the trace's model
/// events accept, per the custody rule: the trace owns the boxes, the organ
/// owns the contents, and conformance is the harness's to check at the
/// submit call. The blocks carry the declared label vocabulary of Spec
/// section 6, the turn's delta being the one span this measurement covers.
fn render_measurement(
    resident: &Resident,
    input_tokens: &[TokenId],
    generated: &weaver_spu::decoder::session::Generated,
    delta_text_len: usize,
) -> String {
    let mut measurement = serde_json::json!({
        "model": resident.artifact.display().to_string(),
        "weights_hash": resident.weights_hash.0,
        "input_tokens": input_tokens.iter().map(|t| t.0).collect::<Vec<u32>>(),
        "output_tokens": generated.tokens.iter().map(|t| t.0).collect::<Vec<u32>>(),
        "blocks": [{"label": "turn-delta", "start": 0, "end": delta_text_len as u64}],
        "timings": {
            "prefill_ns": generated.prefill_ns.to_string(),
            "decode_ns": generated.decode_ns.to_string(),
        },
    });
    // Absent rather than empty, per Spec section 6: an unproduced reading
    // emits no member at all, the absence itself being the report, which is
    // the same discipline the record's own serialization carries.
    if let Some(bits) = &generated.signals.entropy_bits {
        measurement["entropies"] =
            serde_json::json!(bits.iter().map(|b| f64::from(*b)).collect::<Vec<f64>>());
    }
    if let Some(bits) = &generated.signals.surprisal_bits {
        measurement["surprisals"] =
            serde_json::json!(bits.iter().map(|b| f64::from(*b)).collect::<Vec<f64>>());
    }
    // The residual reductions where the residency was admitted with readout
    // elected, per Spec sections 6 and 7: one norm per layer per forward
    // this generation ran, and absent rather than empty like every reading.
    if let Some(norms) = &generated.residual_norms {
        measurement["residual_norms"] =
            serde_json::json!(norms.iter().map(|n| f64::from(*n)).collect::<Vec<f64>>());
    }
    measurement.to_string()
}

/// The decode phase, entered once residency confirms and served until the
/// harness closes its decode end, which is the session ending with the run
/// rather than a fault. The lifecycle seam is silent between admit and
/// release by both contracts' ordering, so this phase owns the process until
/// the channel closes clean and the release path resumes on the other seam.
fn serve_decode(
    decode: &DecodeSocket,
    resident: &Resident,
    effective: &Effective,
) -> Result<(), ()> {
    let mut position = DecodePosition::BeforeOpen;
    let mut opened: Option<OpenedSession<'_>> = None;

    loop {
        let frame = match decode.recv_octets() {
            Ok(frame) => frame,
            Err(ChannelFault::Closed) => return Ok(()),
            Err(fault) => {
                eprintln!("{}", fault_line(&fault));
                return Err(());
            }
        };
        let directive: TokenDirective = match serde_json::from_slice(&frame) {
            Ok(directive) => directive,
            Err(_) => {
                // Below the exchange layer, per Spec section 9: octets that
                // are not a directive are not a message, and a channel this
                // process cannot read faithfully is one it stops serving.
                eprintln!("{}", fault_line(&ChannelFault::Undecodable));
                return Err(());
            }
        };

        // The order is judged before the session is reached. The residency is
        // confirmed by construction here: this phase is entered on admit.
        if let Some(refusal) = judge_decode(position, true, &directive) {
            if send_refusal(decode, &refusal).is_err() {
                return Err(());
            }
            continue;
        }

        match directive {
            TokenDirective::Open { messages, .. } => {
                // **The entry selected at admit, held rather than looked up
                // again.** This was a second lookup by architecture with a
                // branch for losing the family between admit and decode. The
                // resident now carries what admit selected, so there is no
                // second derivation to disagree with the first and no miss to
                // report: the incoherence that branch described is no longer
                // expressible. A contested architecture is also not
                // answerable by architecture alone, so the lookup this
                // replaces could not have served one.
                let declaration = resident.declaration;
                // The declaration cites its family's renderer, so the prefix is
                // that family's own rendering rather than a template string
                // this root walks. The preamble a family opens a prefix with
                // lands here and in no delta.
                let renderer = (declaration.renderer)();
                let prefix_text = match renderer
                    .render_identity(&messages)
                    .map_err(TokenRefusal::from)
                {
                    Ok(text) => text,
                    Err(refusal) => {
                        if send_refusal(decode, &refusal).is_err() {
                            return Err(());
                        }
                        continue;
                    }
                };
                let outcome = resident
                    .open_session(&effective.knobs, effective.session.context_capacity)
                    .and_then(|mut session| {
                        let prefix = resident.tokenize(&prefix_text)?;
                        session.open(&prefix)?;
                        // The family declares its stop conditions and the
                        // artifact's vocabulary is where they are checked, so
                        // the set is built once here rather than assumed from
                        // the artifact's end-of-sequence.
                        let stop = resident.stop_set(renderer)?;
                        Ok(OpenedSession {
                            session,
                            renderer,
                            template: declaration.template,
                            generation_opener: declaration.generation_opener,
                            stop,
                        })
                    });
                match outcome {
                    Ok(standing) => {
                        // A declared stop the artifact cannot promote is named
                        // on the operator's channel rather than dropped: it is
                        // a condition this residency will never recognise, and
                        // a run that carried it silently would report a turn
                        // that ran to the cap as a turn that ended.
                        if !standing.stop.unpromoted.is_empty() {
                            eprintln!(
                                "{}",
                                serde_json::json!({
                                    "stop_conditions_unpromoted": standing.stop.unpromoted,
                                })
                            );
                        }
                        opened = Some(standing);
                        position = DecodePosition::AtRest;
                        if send_answer(decode, &TokenAnswer::Opened).is_err() {
                            return Err(());
                        }
                    }
                    Err(fault) => {
                        eprintln!("{}", decode_fault_line(&fault));
                        return Err(());
                    }
                }
            }

            TokenDirective::AppendAndGenerate { delta, .. } => {
                // The directive carries no sampling value to refuse: the
                // values reached this crate in the declaration and the engine
                // took them when the session opened.
                let standing = opened.as_mut().expect("at rest implies an open session");
                // The delta's rendering closes with the assistant turn opened
                // and unfinished, so the generation completes that turn and
                // the terminator the engine makes resident closes it. The
                // identity prefix at open takes no opener: its turns are all
                // complete.
                let delta_text = match render_delta(standing.renderer, &delta) {
                    Ok(mut text) => {
                        text.push_str(standing.generation_opener);
                        text
                    }
                    Err(refusal) => {
                        if send_refusal(decode, &refusal).is_err() {
                            return Err(());
                        }
                        continue;
                    }
                };
                let delta_tokens = match resident.tokenize(&delta_text) {
                    Ok(tokens) => tokens,
                    Err(fault) => {
                        eprintln!("{}", decode_fault_line(&fault));
                        return Err(());
                    }
                };
                let stop = StopCondition {
                    stop_tokens: standing.stop.tokens.clone(),
                    terminator: standing.stop.terminator,
                    max_tokens: effective.session.max_tokens_per_turn,
                };
                let mut cancel = SeamCancel {
                    socket: decode,
                    cancelled: false,
                    fatal: false,
                };
                // **The stream, per the contract's section 2**: each retained
                // token crosses as it is drawn. The pieces ride a pending
                // buffer because a byte-pair vocabulary splits characters
                // across tokens: a batch that does not yet decode to text
                // waits for the token that completes it, and the piece that
                // then crosses is the rendering that became emittable at this
                // token, per `weaver-types-Spec` section 4.4. A send the
                // socket refuses marks the stream fatal and the phase dies
                // after the exchange, closure being the contract's signal.
                let stream_fatal = std::cell::Cell::new(false);
                let mut pending: Vec<TokenId> = Vec::new();
                let mut on_token = |token: TokenId| {
                    if stream_fatal.get() {
                        return;
                    }
                    pending.push(token);
                    if let Ok(piece) = resident.detokenize(&pending) {
                        pending.clear();
                        let frame = TokenAnswer::Token {
                            token: token.0,
                            piece,
                        };
                        if send_answer(decode, &frame).is_err() {
                            stream_fatal.set(true);
                        }
                    }
                };
                let generated = match standing.session.append_and_generate(
                    &delta_tokens,
                    &stop,
                    &mut cancel,
                    &mut on_token,
                ) {
                    Ok(generated) => generated,
                    Err(DecodeFault::Overflow {
                        resident,
                        requested,
                        capacity,
                    }) => {
                        let refusal = TokenRefusal::Overflow {
                            resident: resident as u64,
                            requested: requested as u64,
                            capacity: capacity as u64,
                        };
                        if send_refusal(decode, &refusal).is_err() {
                            return Err(());
                        }
                        continue;
                    }
                    Err(fault) => {
                        eprintln!("{}", decode_fault_line(&fault));
                        return Err(());
                    }
                };
                if cancel.fatal || stream_fatal.get() {
                    eprintln!("{}", fault_line(&ChannelFault::Undecodable));
                    return Err(());
                }
                let emission = match resident.detokenize(&generated.tokens) {
                    Ok(text) => text,
                    Err(fault) => {
                        eprintln!("{}", decode_fault_line(&fault));
                        return Err(());
                    }
                };
                let finish = match generated.stopped {
                    Stopped::Complete => Finish::Completed,
                    Stopped::Cancelled | Stopped::CapacityReached => Finish::Stopped,
                    Stopped::LimitReached => Finish::Length,
                };
                let measurement =
                    render_measurement(resident, &delta_tokens, &generated, delta_text.len());
                let measurement = match serde_json::value::RawValue::from_string(measurement) {
                    Ok(raw) => raw,
                    Err(_) => {
                        eprintln!(
                            "{}",
                            serde_json::json!({"decode_fault": "measurement did not render"})
                        );
                        return Err(());
                    }
                };
                // **The request is the model.request content whole**, per the
                // custody act: the rendered prompt with its template and the
                // turn's effective sampling, this crate rendering it because
                // the template and the knobs are this crate's, and the harness
                // splices it into the request box without reading it. The
                // values are the effective ones, whichever side set them, per
                // charter section 13.8: a frozen knob is as visible in the
                // record as a supplied one.
                let knobs = effective.knobs;
                let request = match serde_json::value::RawValue::from_string(
                    serde_json::json!({
                        "rendered": delta_text,
                        "template": standing.template,
                        "sampling": {
                            "temperature": knobs.temperature,
                            "top_k": knobs.top_k,
                            "top_p": knobs.top_p,
                            "repetition_penalty": knobs.repetition_penalty,
                            "repetition_window": knobs.repetition_window,
                            "seed": knobs.seed,
                        },
                    })
                    .to_string(),
                ) {
                    Ok(raw) => raw,
                    Err(_) => {
                        eprintln!(
                            "{}",
                            serde_json::json!({"decode_fault": "request did not render"})
                        );
                        return Err(());
                    }
                };
                // The canonical parse crosses beside the verbatim, per the
                // tool workflow's opening act: the family's own parser is the
                // bridge, text as text and every recovered call as a
                // `ToolCall` block. An unrecovered fragment is reported on
                // the operator channel rather than crossing as prose, the
                // parse discipline's own rule.
                let parsed = standing.renderer.parse(&emission);
                if parsed.has_unrecovered_call() {
                    eprintln!(
                        "{}",
                        serde_json::json!({
                            "unrecovered_calls": parsed.unrecovered.len(),
                        })
                    );
                }
                let content: Vec<weaver_traits::ContentBlock> = parsed
                    .content
                    .into_iter()
                    .map(|piece| match piece {
                        family::Content::Text(text) => weaver_traits::ContentBlock::Text { text },
                        family::Content::Call { name, arguments } => {
                            weaver_traits::ContentBlock::ToolCall(weaver_traits::ToolCall {
                                name: name.0,
                                arguments,
                            })
                        }
                    })
                    .collect();
                let answer = TokenAnswer::Generated(Generation {
                    emission,
                    finish,
                    content,
                    request,
                    measurement,
                    resident: standing.session.resident_len() as u64,
                    capacity: standing.session.capacity() as u64,
                });
                if send_answer(decode, &answer).is_err() {
                    return Err(());
                }
                // A cancel that stopped this generation is its own exchange
                // and closes second: the outstanding append answers with the
                // partial output marked stopped, then the cancel answers at
                // rest, per the contract's section 2.
                if cancel.cancelled && send_answer(decode, &TokenAnswer::AtRest).is_err() {
                    return Err(());
                }
            }

            TokenDirective::Cancel { .. } => {
                // At rest, a cancel answers at rest: a clean close rather
                // than a refusal, the operator's intent satisfied by the
                // state either way.
                if send_answer(decode, &TokenAnswer::AtRest).is_err() {
                    return Err(());
                }
            }

            TokenDirective::Flush { keep } => {
                let standing = opened.as_mut().expect("at rest implies an open session");
                let resident_before = standing.session.resident_len() as u64;
                // Saturating, not truncating: a keep past the platform's
                // range must stay large so the session's upper clamp cuts
                // nothing, where a truncation could cut more than asked.
                let keep = usize::try_from(keep).unwrap_or(usize::MAX);
                match standing.session.flush(keep) {
                    Ok(()) => {
                        // Both counts from the one authority, per the decode
                        // contract: the harness authors the record's flush
                        // event from exactly them.
                        let flushed = TokenAnswer::Flushed {
                            resident_before,
                            resident_after: standing.session.resident_len() as u64,
                        };
                        if send_answer(decode, &flushed).is_err() {
                            return Err(());
                        }
                    }
                    Err(fault) => {
                        // A failed flush closes the session, and a session
                        // this process cannot restore to its fixed outcome is
                        // a service it stops providing.
                        eprintln!("{}", decode_fault_line(&fault));
                        return Err(());
                    }
                }
            }
        }
    }
}

/// The admission headroom this process runs with.
///
/// **A host's fact, arriving on argv**, per `weaver-harness-Spec` section 2.2:
/// the room to leave spare on a device is a property of the host and two agents
/// sharing one cannot sensibly disagree about it, so it travels the worker's
/// vector rather than a declaration. A deployment supplying none leaves the
/// compiled default standing, which is what every deployment had before the
/// vector existed.
///
/// **A supplied value that is not a byte count refuses to serve.** The
/// alternative is a process running on a number nobody wrote, and an admission
/// judged against a headroom the deployment did not ask for is worse than a
/// refused start because nothing downstream could tell.
fn headroom_from_arguments() -> Result<u64, String> {
    headroom_from(std::env::args().skip(1))
}

/// **The whole vector is read before anything is returned.** An earlier
/// wording answered on the first parameter it understood, so a vector carrying
/// a misspelled one after it was served the value and never saw the mistake,
/// which is the failure the worker's own parser refuses by name. A parameter
/// stated twice is refused for the same reason: taking the first silently
/// picks for a deployment that plainly meant one of them.
fn headroom_from(arguments: impl Iterator<Item = String>) -> Result<u64, String> {
    let mut arguments = arguments.peekable();
    let mut headroom: Option<u64> = None;
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--headroom-bytes" => {
                let value = arguments
                    .next()
                    .ok_or_else(|| "--headroom-bytes takes a value".to_string())?;
                let parsed = value
                    .parse::<u64>()
                    .map_err(|_| format!("--headroom-bytes wants a byte count, got {value}"))?;
                if headroom.replace(parsed).is_some() {
                    return Err("--headroom-bytes is stated twice".to_string());
                }
            }
            other => return Err(format!("unknown parameter {other}")),
        }
    }
    Ok(headroom.unwrap_or(HEADROOM_BYTES))
}

fn main() -> ExitCode {
    // Entry adopts both ends and performs its two sets before the first read.
    // A refusal here is a refusal to serve: the count check failing means the
    // harness's fork discipline failed upstream, and this process is not the
    // one to continue past it.
    let headroom = match headroom_from_arguments() {
        Ok(headroom) => headroom,
        Err(complaint) => {
            eprintln!(
                "{}",
                serde_json::json!({"refusal": "bad_parameter", "detail": complaint})
            );
            return ExitCode::FAILURE;
        }
    };
    let inherited = match channel::adopt() {
        Ok(inherited) => inherited,
        Err(fault) => {
            eprintln!("{}", entry_refusal_line(&fault));
            return ExitCode::FAILURE;
        }
    };
    serve(inherited, headroom)
}

/// A refusal before any channel is trusted goes to standard error, because the
/// lifecycle channel is exactly what this path could not establish.
fn entry_refusal_line(fault: &EntryFault) -> String {
    match fault {
        EntryFault::DescriptorCountWrong { found } => format!(
            "{{\"refusal\":\"descriptors_unusable\",\"held_beyond_standard_streams\":{found}}}"
        ),
        EntryFault::DescriptorsUnusable => "{\"refusal\":\"descriptors_unusable\"}".to_string(),
        EntryFault::HygieneFailed => "{\"refusal\":\"boundary_unverified\"}".to_string(),
    }
}

/// The one loop. One directive at a time against one resident session.
fn serve(inherited: Inherited, headroom: u64) -> ExitCode {
    let Inherited { lifecycle, decode } = inherited;

    let mut residency = Residency::new();
    let mut position = SeamPosition::BeforeAdmit;
    let mut effective: Option<Effective> = None;

    loop {
        let envelope = match lifecycle.recv() {
            Ok(envelope) => envelope,
            Err(ChannelFault::Closed) => {
                // The harness owns this process's lifetime, so a closed channel
                // is an orderly exit rather than a retry or a failure.
                return ExitCode::SUCCESS;
            }
            Err(fault) => {
                // Truncated or undecodable. Faults are below the exchange
                // layer, per Spec section 9: neither is a message, so neither
                // is answered on an exchange, and a channel this process cannot
                // read faithfully is a channel it stops serving.
                eprintln!("{}", fault_line(&fault));
                return ExitCode::FAILURE;
            }
        };

        let payload = dispatch(
            &mut position,
            &mut residency,
            &envelope,
            &mut effective,
            headroom,
        );
        let admitted_now = matches!(payload, Payload::Answer(LifecycleAnswer::Admitted));
        if answer(&lifecycle, envelope.exchange, payload).is_err() {
            return ExitCode::FAILURE;
        }
        if admitted_now {
            // The residency stands, so the decode phase owns the process
            // until the harness closes its decode end, the session ending
            // with the run. The lifecycle seam is silent between admit and
            // release by both contracts' ordering, and the release arrives
            // here once the phase returns clean.
            let resident = residency.resident().expect("the admit just confirmed");
            let effective = effective.as_ref().expect("the admit resolved them");
            if serve_decode(&decode, resident, effective).is_err() {
                return ExitCode::FAILURE;
            }
        }
    }
}

fn fault_line(fault: &ChannelFault) -> String {
    match fault {
        ChannelFault::Truncated { bound } => {
            format!("{{\"fault\":\"truncated\",\"bound\":{bound}}}")
        }
        ChannelFault::Undecodable => "{\"fault\":\"undecodable\"}".to_string(),
        ChannelFault::Closed => "{\"fault\":\"closed\"}".to_string(),
    }
}

fn answer(
    lifecycle: &LifecycleChannel,
    exchange: ExchangeId,
    payload: Payload,
) -> Result<(), ChannelFault> {
    lifecycle.send(&OrganEnvelope {
        exchange,
        position: Position::Close,
        payload,
    })
}

/// **The two exchanges of `weaver-harness-spu-contract`, judged against the
/// seam's recorded position first.**
///
/// The contract's vocabulary clause draws admit and release on the directive,
/// and section 9's state machine says which is in order when. Everything else,
/// including a well-formed directive at the wrong position, a directive that
/// does not open its exchange, and an exchange claiming an opener that is not
/// the harness, refuses as `OutOfOrder` before residency is reached.
///
/// The match carries no wildcard arm, so a case added to loop 0 breaks this
/// crate loudly in the act that edits the floor.
fn dispatch(
    position: &mut SeamPosition,
    residency: &mut Residency,
    envelope: &OrganEnvelope,
    effective: &mut Option<Effective>,
    headroom: u64,
) -> Payload {
    // A directive opens its exchange, and only the harness opens on this
    // channel. Anything else is the peer speaking out of turn.
    if envelope.position != Position::Open || envelope.exchange.opener != Opener::Harness {
        return Payload::Refusal(LifecycleRefusal::OutOfOrder);
    }
    let Payload::Directive(directive) = &envelope.payload else {
        return Payload::Refusal(LifecycleRefusal::OutOfOrder);
    };

    match (*position, directive) {
        (SeamPosition::BeforeAdmit, LifecycleDirective::Admit { instruction }) => {
            // The election arrives beside the binding inside the instruction,
            // per `weaver-harness-spu-contract` section 2, so the judgment
            // receives what the operator declared rather than the placeholder
            // the routeless seam once forced. The judgment itself runs inside
            // admit, at charter step 3, before any device is taken.
            let decoder = &instruction.decoder;
            // **The elections resolve before a device is taken.** A parameter
            // this binary left tunable with nothing supplied, or a count
            // supplied as a fraction or a negative, refuses the load naming the
            // parameter. Refusing here rather than at open keeps a
            // declaration's mistake from costing the admit's device work.
            let resolved = match resolve_effective(&decoder.tunable_values) {
                Ok(resolved) => resolved,
                Err(refusal) => {
                    let knob = match refusal {
                        KnobRefusal::Unsupplied { knob } => knob,
                        KnobRefusal::NotACount { knob, .. } => knob,
                    };
                    // The one admission is spent whatever it answered, this
                    // route included: a declaration refused here has had its
                    // admission and a second attempt is out of order, the same
                    // invariant the artifact refusal below keeps.
                    *position = SeamPosition::AdmitRefused;
                    return Payload::Refusal(LifecycleRefusal::ConfigInvalid {
                        field: Some(weaver_types::FieldName(format!("tunable-values.{knob}"))),
                    });
                }
            };
            match residency.admit(
                &decoder.model_binding,
                Headroom(headroom),
                ReadoutElection(decoder.residual_readout_election),
            ) {
                Ok(_) => {
                    *position = SeamPosition::Admitted;
                    *effective = Some(resolved);
                    Payload::Answer(LifecycleAnswer::Admitted)
                }
                Err(refusal) => {
                    // The engine's account of a failed load dies at the floor
                    // conversion, whose closed set carries no detail field, so
                    // it lands on standard error first: an operator meeting
                    // DeviceCannotAdmit for a corrupt artifact should not be
                    // sent to look at a healthy card with nothing else to read.
                    if let weaver_spu::residency::AdmitRefusal::LoadFailed { detail } = &refusal {
                        eprintln!(
                            "{}",
                            serde_json::json!({"refusal": "load_failed", "detail": detail})
                        );
                    }
                    // The one admission is spent whatever it answered.
                    *position = SeamPosition::AdmitRefused;
                    Payload::Refusal(refusal.into())
                }
            }
        }
        (SeamPosition::Admitted, LifecycleDirective::Release) => match residency.release() {
            Ok(()) => {
                *position = SeamPosition::Released;
                Payload::Answer(LifecycleAnswer::Released)
            }
            Err(refusal) => Payload::Refusal(refusal),
        },

        // Every other pairing is out of order for the seam's recorded
        // position: a release before any admit, a second admit whatever the
        // first answered, anything after a release, and every directive the
        // contract does not draw. The directive is refused before it reaches
        // residency, which is what not-queued means.
        (
            _,
            LifecycleDirective::Admit { .. }
            | LifecycleDirective::Release
            | LifecycleDirective::Enter { .. }
            | LifecycleDirective::Leave
            | LifecycleDirective::Stop
            | LifecycleDirective::Raise { .. }
            | LifecycleDirective::Lower
            | LifecycleDirective::Load { .. }
            | LifecycleDirective::Unload { .. }
            | LifecycleDirective::Validate { .. }
            | LifecycleDirective::List
            | LifecycleDirective::Show { .. },
        ) => Payload::Refusal(LifecycleRefusal::OutOfOrder),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaver_traits::{ContentBlock, Role};
    use weaver_types::{
        AgentName, ArtifactRef, DecoderInstruction, DeviceOrdinal, ModelBinding, SpuInstruction,
    };

    /// **Every argument is read before the answer is given.**
    ///
    /// Perturbation: return on the first `--headroom-bytes` rather than
    /// recording it and continuing, and the trailing-unknown case passes with
    /// the value served and the mistake unseen. Watched under exactly that.
    #[test]
    fn the_whole_vector_is_judged() {
        let of = |v: &[&str]| headroom_from(v.iter().map(|s| s.to_string()));
        assert_eq!(
            of(&[]),
            Ok(HEADROOM_BYTES),
            "nothing stated keeps the compiled default"
        );
        assert_eq!(of(&["--headroom-bytes", "1024"]), Ok(1024));
        assert!(
            of(&["--headroom-bytes", "1024", "--bogus"]).is_err(),
            "a misspelled parameter after a good one is still a mistake"
        );
        assert!(
            of(&["--headroom-bytes", "1024", "--headroom-bytes", "2048"]).is_err(),
            "stated twice picks for a deployment that meant one of them"
        );
        assert!(
            of(&["--headroom-bytes"]).is_err(),
            "the value is not optional"
        );
        assert!(
            of(&["--headroom-bytes", "many"]).is_err(),
            "and it is a byte count"
        );
    }

    fn directive(directive: LifecycleDirective) -> OrganEnvelope {
        OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: 1,
            },
            position: Position::Open,
            payload: Payload::Directive(directive),
        }
    }

    fn binding() -> ModelBinding {
        ModelBinding {
            artifact: ArtifactRef("/nonexistent/artifact".into()),
            devices: vec![DeviceOrdinal(0)],
        }
    }

    fn instruction() -> SpuInstruction {
        SpuInstruction {
            decoder: DecoderInstruction {
                model_binding: binding(),
                residual_readout_election: false,
                identity: vec![],
                tunable_values: [
                            ("max-tokens-per-turn".to_string(), 4096.0),
                            ("context-capacity".to_string(), 4096.0),
                        ]
                    .into_iter()
                    .collect(),
            },
        }
    }

    /// **A release before any admit answers `OutOfOrder`,** per Spec sections 9
    /// and 10: the order is judged against the seam's recorded position before
    /// the directive reaches residency, so the refusal is the position's and
    /// not residency's `NoResidency`.
    ///
    /// Perturbation: route `(BeforeAdmit, Release)` through
    /// `residency.release()` and this test fails, because the answer becomes
    /// `NoResidency`. Watched under exactly that change.
    #[test]
    fn a_release_before_any_admit_answers_out_of_order() {
        let mut residency = Residency::new();
        let mut position = SeamPosition::BeforeAdmit;
        assert_eq!(
            dispatch(
                &mut position,
                &mut residency,
                &directive(LifecycleDirective::Release),
                &mut None,
                HEADROOM_BYTES,
            ),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );
        assert_eq!(position, SeamPosition::BeforeAdmit, "and is not queued");
    }

    /// **The released position is terminal.** A directive of any kind arriving
    /// after a release answers `OutOfOrder`. The seam cannot reach this
    /// position today, no build being able to admit, so the machine is driven
    /// to it directly: the state is this crate's own and the test reads the
    /// judgment, not the journey.
    #[test]
    fn any_directive_after_a_release_answers_out_of_order() {
        let mut residency = Residency::new();
        let mut position = SeamPosition::Released;
        for case in [
            LifecycleDirective::Admit {
                instruction: instruction(),
            },
            LifecycleDirective::Release,
            LifecycleDirective::List,
        ] {
            assert_eq!(
                dispatch(
                    &mut position,
                    &mut residency,
                    &directive(case.clone()),
                    &mut None,
                    HEADROOM_BYTES
                ),
                Payload::Refusal(LifecycleRefusal::OutOfOrder),
                "{case:?} after a release is out of order"
            );
            assert_eq!(position, SeamPosition::Released, "terminal stays terminal");
        }
    }

    /// **A second admit answers `OutOfOrder` whatever the first answered.** The
    /// first spends the one admission even when it refuses, and the second is
    /// refused at the position rather than reaching residency's resolution
    /// step.
    #[test]
    fn a_second_admit_is_refused_at_the_position() {
        let mut residency = Residency::new();
        let mut position = SeamPosition::BeforeAdmit;

        let first = dispatch(
            &mut position,
            &mut residency,
            &directive(LifecycleDirective::Admit {
                instruction: instruction(),
            }),
            &mut None,
            HEADROOM_BYTES,
        );
        assert_eq!(
            first,
            Payload::Refusal(LifecycleRefusal::ArtifactUnresolvable),
            "the first admit runs and refuses on the artifact"
        );
        assert_eq!(position, SeamPosition::AdmitRefused);

        let second = dispatch(
            &mut position,
            &mut residency,
            &directive(LifecycleDirective::Admit {
                instruction: instruction(),
            }),
            &mut None,
            HEADROOM_BYTES,
        );
        assert_eq!(second, Payload::Refusal(LifecycleRefusal::OutOfOrder));
    }

    /// **Only admit and release cross this seam.** Every other case of the
    /// floor's closed set refuses as out of order.
    ///
    /// Perturbation: answer `Stop` with `AtRest` in `dispatch` and this test
    /// fails on the `Stop` case. Watched under exactly that addition.
    #[test]
    fn a_directive_outside_the_drawn_vocabulary_refuses_out_of_order() {
        let mut residency = Residency::new();
        let mut position = SeamPosition::BeforeAdmit;
        let outside = [
            LifecycleDirective::Leave,
            LifecycleDirective::Stop,
            LifecycleDirective::Lower,
            LifecycleDirective::List,
            LifecycleDirective::Load {
                agent: AgentName("alpha".into()),
            },
            LifecycleDirective::Unload {
                agent: AgentName("alpha".into()),
            },
            LifecycleDirective::Validate {
                agent: AgentName("alpha".into()),
            },
            LifecycleDirective::Show {
                agent: AgentName("alpha".into()),
            },
        ];
        for case in outside {
            assert_eq!(
                dispatch(
                    &mut position,
                    &mut residency,
                    &directive(case.clone()),
                    &mut None,
                    HEADROOM_BYTES
                ),
                Payload::Refusal(LifecycleRefusal::OutOfOrder),
                "{case:?} is outside this seam's vocabulary"
            );
        }
    }

    /// A directive that does not open its exchange, or an exchange claiming an
    /// opener that is not the harness, is the peer speaking out of turn, judged
    /// before the directive's own case is looked at.
    ///
    /// Perturbation: remove the position-and-opener check at the top of
    /// `dispatch` and this test fails, because the well-formed admit inside the
    /// mis-shapen envelope executes and spends the one admission. Watched under
    /// exactly that removal.
    #[test]
    fn a_mis_shapen_exchange_refuses_before_the_directive_runs() {
        let mut residency = Residency::new();
        let mut position = SeamPosition::BeforeAdmit;

        let mut closing = directive(LifecycleDirective::Admit {
            instruction: instruction(),
        });
        closing.position = Position::Close;
        assert_eq!(
            dispatch(
                &mut position,
                &mut residency,
                &closing,
                &mut None,
                HEADROOM_BYTES
            ),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );

        let mut wrong_opener = directive(LifecycleDirective::Admit {
            instruction: instruction(),
        });
        wrong_opener.exchange.opener = Opener::Spu;
        assert_eq!(
            dispatch(
                &mut position,
                &mut residency,
                &wrong_opener,
                &mut None,
                HEADROOM_BYTES
            ),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );

        assert_eq!(
            position,
            SeamPosition::BeforeAdmit,
            "neither mis-shapen envelope spent the admission"
        );
    }

    /// A payload that is not a directive is the peer speaking out of turn.
    #[test]
    fn a_non_directive_payload_refuses_out_of_order() {
        let mut residency = Residency::new();
        let mut position = SeamPosition::BeforeAdmit;
        let envelope = OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: 1,
            },
            position: Position::Open,
            payload: Payload::Answer(LifecycleAnswer::Ready),
        };
        assert_eq!(
            dispatch(
                &mut position,
                &mut residency,
                &envelope,
                &mut None,
                HEADROOM_BYTES
            ),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );
    }

    fn open() -> TokenDirective {
        TokenDirective::Open {
            session: weaver_types::SessionId("s-1".into()),
            messages: vec![],
        }
    }

    fn append() -> TokenDirective {
        TokenDirective::AppendAndGenerate {
            turn: weaver_types::TurnKey("t-1".into()),
            delta: vec![],
        }
    }

    /// **An open before the residency it serves is confirmed refuses and is
    /// not queued,** per Spec section 9, one contract's ordering read against
    /// the other's. The judgment is pure and precedes the session, so the
    /// refusal is about the order and never about work already run.
    ///
    /// Perturbation: make `judge_decode` hold an early open for later and
    /// this test fails, there being no refusal to read. Watched under exactly
    /// that change.
    #[test]
    fn an_open_before_residency_refuses_and_after_it_is_in_order() {
        assert_eq!(
            judge_decode(DecodePosition::BeforeOpen, false, &open()),
            Some(TokenRefusal::OutOfOrder),
            "before residency, out of order"
        );
        assert_eq!(
            judge_decode(DecodePosition::BeforeOpen, true, &open()),
            None,
            "after residency, in order"
        );
    }

    /// **A directive before any open answers `NotOpen`,** the refusal case
    /// `weaver-types-Spec` section 4.4 gives the not-open session, distinct
    /// from the ordering refusal because the harness reads the two
    /// differently: one says open first, the other says never.
    #[test]
    fn a_directive_before_any_open_answers_not_open() {
        for directive in [
            append(),
            TokenDirective::Cancel {
                turn: weaver_types::TurnKey("t-1".into()),
            },
            TokenDirective::Flush { keep: 0 },
        ] {
            assert_eq!(
                judge_decode(DecodePosition::BeforeOpen, true, &directive),
                Some(TokenRefusal::NotOpen),
                "{directive:?} before any open"
            );
        }
    }

    /// **A second open refuses rather than rewinding,** per the contract's
    /// open-first-and-once, the flush being what a rewind has instead.
    #[test]
    fn a_second_open_refuses_rather_than_rewinding_on_the_seam() {
        assert_eq!(
            judge_decode(DecodePosition::AtRest, true, &open()),
            Some(TokenRefusal::OutOfOrder)
        );
    }

    /// **A block the family cannot render refuses as the delta malformed for
    /// the family,** the contract's own case. The tool workflow lifted the
    /// blanket block, so the subject here is a shape no party authors: a
    /// tool call riding a user turn, which qwen2 renders for the assistant
    /// alone.
    ///
    /// What this reads is the refusal arriving at the seam's own vocabulary,
    /// so it exercises the family's render and the [`From`] that carries its
    /// answer across. The role arm's wildcard is the future's alone and no
    /// test can construct its subject, `Role` being non-exhaustive with every
    /// current case rendered by the families that call the shared name kernel.
    #[test]
    fn a_non_text_block_refuses_as_malformed_delta() {
        let message = Message {
            role: Role::User,
            content: vec![ContentBlock::ToolCall(weaver_traits::ToolCall {
                name: "calculator".into(),
                arguments: "{}".into(),
            })],
        };
        assert_eq!(
            render_delta(family::qwen2::renderer(), &[message]),
            Err(TokenRefusal::MalformedDelta),
        );
    }

    /// **A tool result is not renderable under gemma4, and the refusal is the
    /// family's rather than the root's.**
    ///
    /// The family has no tool turn: its template carries tool results as
    /// standalone blocks outside the turn structure. Rendering one as
    /// `<|turn>tool` would be a silent substitution, so it refuses until the
    /// tool workflow names the block shape.
    ///
    /// Perturbation: give `Gemma4::role_name` a `Role::ToolResult => "tool"`
    /// arm and this fails, the render then succeeding on a turn shape the
    /// artifact's template never produces.
    #[test]
    fn gemma4_refuses_a_tool_result_rather_than_inventing_a_turn() {
        let message = Message {
            role: Role::ToolResult,
            content: vec![ContentBlock::Text {
                text: "42".to_string(),
            }],
        };
        assert_eq!(
            render_delta(family::gemma4::renderer(), &[message]),
            Err(TokenRefusal::MalformedDelta),
        );

        // The contrast, so this reads as a distinction rather than as a family
        // that refuses everything: the authored shape - the granted door's
        // result block in the tool-result role - renders under qwen2.
        let message = Message {
            role: Role::ToolResult,
            content: vec![ContentBlock::ToolResult(weaver_traits::ToolResultBlock {
                content: "42".to_string(),
            })],
        };
        assert!(render_delta(family::qwen2::renderer(), &[message]).is_ok());
    }

    /// At rest, a cancel and a flush are both in order for the seam: the
    /// cancel answers at rest rather than refusing, which is the executor's
    /// answer and not this judgment's refusal.
    #[test]
    fn cancel_and_flush_are_in_order_at_rest() {
        assert_eq!(
            judge_decode(
                DecodePosition::AtRest,
                true,
                &TokenDirective::Cancel {
                    turn: weaver_types::TurnKey("t-1".into()),
                },
            ),
            None
        );
        assert_eq!(
            judge_decode(
                DecodePosition::AtRest,
                true,
                &TokenDirective::Flush { keep: 0 }
            ),
            None
        );
    }
}
