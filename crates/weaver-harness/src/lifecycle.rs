//! conforms: harness-one-constructor
//! conforms: harness-failed-set-refuses-construction
//! conforms: harness-channel-state-three-positions
//! conforms: harness-out-of-order-refused
//! conforms: harness-run-state-options-checked-unwind
//! conforms: harness-spu-channels-one-field
//! conforms: harness-decode-end-own-type
//! conforms: harness-one-sink-descriptor
//! conforms: harness-decode-pair-created-before-the-fork
//! conforms: harness-gate-pair-waits-on-residency
//! conforms: harness-diagnostic-enter-forks-no-gate
//! conforms: harness-scoped-refusal-account
//! conforms: harness-left-follows-drain
//! conforms: harness-announce-after-record
//! conforms: harness-dumpable-flag-cleared
//! conforms: harness-binds-coordination-socket-first
//! conforms: harness-loop-zero-takes-no-abstraction
//! conforms: harness-spawns-no-thread
//! conforms: harness-organ-forks-on-worker-lifetime-thread
//! conforms: harness-internal-dependency-set
//! conforms: harness-session-opens-at-enter
//! conforms: harness-frame-grants-the-seat
//! conforms: harness-parse-refuses-not-faults
//!
//! The lifecycle interior, per `weaver-harness-Spec` section 3: the harness
//! type, the run state, and the fan-out of loop 0.
//!
//! **Loop 0 takes neither a type nor a trait**, and the cell closes here: the
//! loop is the interval between two directives, its state is the [`Run`]
//! struct, and its control flow is the serial service below, so an
//! abstraction would have no second implementor and no caller that varies.
//! This crate spawns no thread - the one auxiliary thread the merged set
//! names is the stream writer's, and it belongs to `weaver-trace` - so the
//! organ forks run on a thread whose lifetime is the worker's, which is the
//! timing guarantee the gate's parent-death backing relies on.

use std::os::fd::OwnedFd;
use std::path::PathBuf;

use weaver_trace::{Kind, Payload, Recorder, RunRef, SessionRef, Subsystem, TurnClose};
use weaver_types::{
    ExchangeId, LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Opener, OrganEnvelope,
    Position, SessionId, TokenAnswer, TokenDirective, TurnKey,
};

use crate::authorship::Author;
use crate::channel::{CoordinationListener, DecodeChannel, OrganChannel};
use crate::failure::{AdoptionFault, ChannelFault, Outcome};

/// What the entered-state wait woke on, one of the four descriptors section
/// 6.2 spans. The tag is carried beside its `PollFd` so the wake names its
/// origin without re-deriving it from a position.
#[derive(Clone, Copy)]
enum Wake {
    Listener,
    Connection,
    Gate,
    Decode,
}

/// The request's parse, per `weaver-gate-Spec` section 4: one JSON object,
/// one `text` member, a string, unknown members refused. Every failure is
/// the refused turn's reason, content the harness authors, never a channel
/// fault, which is the layer split the frame election bought.
fn parse_request(frame: &weaver_types::TurnFrame) -> Result<String, &'static str> {
    let Some(octets) = frame.octets() else {
        return Err("the frame's carriage is not the canonical encoding");
    };
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(&octets) else {
        return Err("the line does not parse as one JSON value");
    };
    let Some(object) = value.as_object() else {
        return Err("a request is one JSON object");
    };
    if object.len() != 1 {
        return Err("a request carries the text member and nothing else");
    }
    let Some(text) = object.get("text").and_then(|value| value.as_str()) else {
        return Err("a request carries the text member, a string");
    };
    Ok(text.to_string())
}

/// One response line, the kind-named close of `weaver-gate-Spec` section 4:
/// `answered` with `text`, `stopped` or `refused` with `reason`. The map
/// orders keys, so the kind leads whatever the member.
/// The close a client receives, per `weaver-gate-world-contract` section 3.
///
/// **`named` is the turn this close answers and the run it belongs to, and it
/// is absent where no turn opened.** A frame that does not parse is refused
/// before the seat is granted, and a prompt with a hole closes before it, so
/// there is no turn for those to identify and the close says only what it is.
/// A turn that opened carries its key whatever became of it. The members take
/// the trace envelope's own names so a consumer holding both joins on what it
/// already reads.
fn render_close(
    kind: &str,
    member: &str,
    value: &str,
    named: Option<(&TurnKey, &weaver_types::RunId)>,
) -> String {
    render_close_with_finish(kind, member, value, named, false)
}

/// The world contract's one optional member: an answered close whose
/// generation was cut at the turn's token limit carries `finish:
/// "length"`, and carries it only then, so a client renders a truncated
/// answer as truncated and pays nothing on the whole ones.
fn render_close_with_finish(
    kind: &str,
    member: &str,
    value: &str,
    named: Option<(&TurnKey, &weaver_types::RunId)>,
    truncated: bool,
) -> String {
    let mut map = serde_json::Map::new();
    map.insert(
        "kind".to_string(),
        serde_json::Value::String(kind.to_string()),
    );
    map.insert(
        member.to_string(),
        serde_json::Value::String(value.to_string()),
    );
    if truncated {
        map.insert(
            "finish".to_string(),
            serde_json::Value::String("length".to_string()),
        );
    }
    if let Some((turn, run)) = named {
        map.insert(
            "turn".to_string(),
            serde_json::Value::String(turn.0.clone()),
        );
        map.insert("run".to_string(), serde_json::Value::String(run.0.clone()));
    }
    serde_json::Value::Object(map).to_string()
}

/// Author the record's `refusal` for a lifecycle refusal that reached a
/// standing run, per `weaver-harness-Spec` section 6.
///
/// **The run stays entered and this is not a close.** A refused enter leaves
/// a run in place holding what the fan-out forked, and what ends the bracket
/// is the operator's `leave` arriving later, so this authors an event inside
/// a standing run rather than a last word before teardown.
///
/// **The ask is named and not reproduced.** `LifecycleAsk::Enter` says which
/// directive was refused, and the declaration it carried is not copied in:
/// the load event already holds the run's posture, and reproducing a
/// declaration here would be one fact in two places, per
/// `weaver-types-Spec` section 4's rule for the record.
///
/// Best-effort. A refusal that could not be recorded is a defect in the
/// author rather than in the ask, and the answer to admin is already
/// travelling.
fn author_lifecycle_refusal(run: &mut Run, refusal: &LifecycleRefusal) {
    let record = weaver_types::RefusalRecord::Lifecycle {
        asked: weaver_types::LifecycleAsk::Enter,
        refusal: refusal.clone(),
    };
    let Ok(rendered) = serde_json::to_string(&record) else {
        return;
    };
    let Some(payload) = weaver_trace::raw_payload(&rendered) else {
        return;
    };
    let _ = run.author.author(
        &mut run.recorder,
        Kind::Refusal,
        Subsystem::Harness,
        None,
        Some(weaver_trace::Payload::Refusal(payload)),
    );
}

/// A decode refusal rendered as the stopped close's reason, content for the
/// client rather than the wire case itself.
///
/// **Derived from the recorded refusal rather than matched in parallel**,
/// per `weaver-types-Spec` section 4's conversion rule: a conversion may
/// narrow a refusal for a reader and may not be the only record of it. The
/// same value that reaches the record reaches this function, so the client's
/// sentence cannot say something the record does not. It stays lossy, a
/// client being owed a sentence rather than four integers, and what it stops
/// being is independent.
fn refusal_reason(refusal: &weaver_types::TokenRefusal) -> &'static str {
    match refusal {
        weaver_types::TokenRefusal::NotOpen => "the session is not open",
        weaver_types::TokenRefusal::OutOfOrder => "the ask was out of order for the seam",
        weaver_types::TokenRefusal::Overflow { .. } => "the session cannot take the delta",
        weaver_types::TokenRefusal::MalformedDelta => "the delta was malformed for the family",
        // The span's own numbers are the loop's to read off the refusal, so
        // this rendering says the kind and leaves the arithmetic where a
        // reader can act on it rather than flattening it into prose.
        weaver_types::TokenRefusal::UnremovableSpan { .. } => {
            "the elision named no removable span"
        }
        weaver_types::TokenRefusal::RefeedPermissionAbsent => {
            "the re-feed permission was not granted"
        }
        weaver_types::TokenRefusal::RefeedPathEmpty => "the re-feed carried no token path",
        weaver_types::TokenRefusal::ColumnPermissionAbsent => {
            "the column permission was not granted"
        }
        weaver_types::TokenRefusal::ColumnReadoutUnelected => {
            "the column ask stood without the readout election"
        }
        weaver_types::TokenRefusal::ColumnUndeclared => {
            "the family's declaration holds no column"
        }
    }
}

/// Where the organ binaries live: a deployment fact supplied by the
/// composition root as a construction parameter, the way `weaver-trace-Spec`
/// section 6 takes its queue depth. This is the one exception to the
/// no-path-taken rule of section 2.3, and it is not an operator election and
/// not a discovery.
#[derive(Debug, Clone)]
pub struct OrganBinaries {
    pub spu: PathBuf,
    pub gate: PathBuf,
    /// The classify process's binary, `weaver-spu-classify`, per
    /// `weaver-spu-Spec` section 11: optional because the arm is, and a
    /// declaration carrying the binding with no binary provisioned refuses
    /// the load rather than standing half an arm.
    pub classify: Option<PathBuf>,
}

/// The construction parameters this worker hands its organs, per
/// `weaver-harness-Spec` section 2.2.
///
/// **A host's facts and not an agent's.** What belongs here is a number two
/// agents sharing this host cannot sensibly disagree about. A number that is a
/// property of the agent reaches its organ in the declaration instead, which is
/// why the context capacity and the per-turn ceiling are not here.
///
/// Carried as the string a deployment wrote rather than parsed on the way
/// through: this crate hands it on and the organ's composition root is what
/// judges it, so a bad value is refused by the crate that knows what the
/// parameter means rather than twice.
#[derive(Debug, Clone, Default)]
pub struct OrganParameters {
    /// The admission headroom, in bytes, if this deployment supplies one. An
    /// organ given none keeps its own compiled default.
    pub headroom_bytes: Option<String>,
}

impl OrganParameters {
    /// The SPU's vector. Named flags rather than positions, because a
    /// deployment supplying one parameter and not another must not have to
    /// know the order, and an organ meeting an unknown flag says which.
    pub fn spu_arguments(&self) -> Vec<String> {
        let mut arguments = Vec::new();
        if let Some(headroom) = &self.headroom_bytes {
            arguments.push("--headroom-bytes".to_string());
            arguments.push(headroom.clone());
        }
        arguments
    }
}

#[cfg(test)]
mod parameter_tests {
    use super::OrganParameters;

    /// **A deployment that states nothing hands nothing**, so an organ keeps
    /// the default it compiled and an installation that never thought about a
    /// parameter behaves as it always did.
    #[test]
    fn no_parameter_is_no_argument() {
        assert!(OrganParameters::default().spu_arguments().is_empty());
    }

    /// A stated parameter travels as a named flag and its value, so a
    /// deployment supplying one need not know the order of any other.
    #[test]
    fn a_stated_parameter_travels_named() {
        let parameters = OrganParameters {
            headroom_bytes: Some("268435456".to_string()),
        };
        assert_eq!(
            parameters.spu_arguments(),
            vec!["--headroom-bytes".to_string(), "268435456".to_string()]
        );
    }
}

/// The coordination channel's state: three positions, the last terminal, and
/// the middle one carrying the run. A directive out of order for the state
/// reaches a match arm rather than a flag check, which is what the type buys.
enum ChannelState {
    BeforeEnter,
    Entered(Box<Run>),
    Left,
}

/// The fan-out's progress held as data, which is what makes the unwind total.
/// Each `Option` is an arm of the enter fan-out that has or has not stood up,
/// so a leave arriving after a refused enter unwinds exactly what stands, and
/// the compiler's match on the options is what makes a forgotten arm
/// unrepresentable rather than unlikely.
struct Run {
    recorder: crate::record::Record,
    author: Author,
    session: SessionId,
    /// The run's own reference, retained so a close can name the run a turn
    /// belongs to, per `weaver-gate-world-contract` section 3. The author
    /// holds its own converted copy for the record, and this is the floor's,
    /// carried rather than rebuilt.
    run: weaver_types::RunId,
    spu: Option<SpuChannels>,
    gate: Option<GateChannel>,
    /// The state seam's ask end, per `weaver-harness-Spec` section 6: a
    /// clone of the standing state channel, granted to the seat as the
    /// state port. `None` where the leg is not standing, which the port
    /// serves as the same absence a missing answer does.
    state: Option<crate::state::StateSeam>,
    /// The classify arm, per `weaver-harness-Spec` section 6: `None` where
    /// the declaration carried no binding, which the port serves as the
    /// missing leg's absence.
    classify: Option<ClassifyArm>,
    /// The session's fullness as the last generation carried it, granted to
    /// the seat as the fullness read, per the context ports of the Spec's
    /// section 6.
    fullness: Option<(u64, u64)>,
    /// Whether the recorder's pressure has been reported since it last
    /// crossed the high-water mark, per `weaver-harness-Spec` section 4.
    ///
    /// **Held by the run because the condition is the run's.** A seat is
    /// granted per turn and a full queue outlives any one turn, so a flag
    /// scoped to the seat would report again on every turn while the depth
    /// stayed high, which is the repetition the once-per-crossing rule
    /// refuses.
    pressure_reported: bool,
    turn_in_flight: Option<TurnKey>,
    /// The diagnostic run's own facts, present exactly where the binding
    /// is: what the load declared for the replay's opening event, the
    /// artifact the identity step checks the record against, and whether
    /// the seat has been granted, it being granted once at the run's
    /// opening per `weaver-harness-Spec` section 6.2's second criterion.
    diagnostic: Option<DiagnosticSeat>,
    /// Each initiator numbers exchanges on its own channel, so the SPU's and
    /// the gate's counters are separate and neither is hardcoded.
    spu_ordinal: u64,
    gate_ordinal: u64,
    /// Envelopes that arrived on the gate channel while a tool execution was
    /// awaited: a client's frame crossing mid-execution is held here by the
    /// turn and served after it, per the one-turn discipline.
    held_frames: std::collections::VecDeque<weaver_types::OrganEnvelope>,
    /// The turn counter loop 0 mints turn keys from, per the gate contract's
    /// rule that the turn does not exist until the harness opens it.
    turn_ordinal: u64,
}

/// The Gateless seat's standing, per `weaver-harness-Spec` section 6.2: the
/// run itself is the work, so what is held is what the one grant needs and
/// a flag that it happened.
struct DiagnosticSeat {
    reader_elected: bool,
    artifact: String,
    replayed: bool,
}

/// The SPU's arm is a pair of channels rather than one, and they are one field
/// because they stand up and fall together: the two are created in one act and
/// cross one fork, so an option over the pair keeps the arm's all-or-nothing
/// shape where two options would admit a half-stood arm the unwind would have
/// to reason about.
#[derive(Debug)]
struct SpuChannels {
    lifecycle: OrganChannel,
    decode: DecodeChannel,
    /// Retained so the leave can reap the organ rather than leaving a zombie
    /// entry for the life of the worker.
    pid: nix::unistd::Pid,
}

/// The classify readiness bound: generous against an admission that loads a
/// sub-gigabyte artifact in seconds, because the expiry refuses the whole
/// load and a slow disk should not read as a dead child.
const CLASSIFY_READY_BOUND_MS: u64 = 60_000;

/// The classify arm, per `weaver-harness-Spec` section 6: the label seam's
/// near end and the process behind it, standing only where the declaration
/// carried the binding, falling whole under the unwind like every arm.
struct ClassifyArm {
    channel: crate::channel::ClassifyChannel,
    pid: nix::unistd::Pid,
}

/// How an enter refused, and whether the bracket stands. **A refusal before
/// the load event leaves the stream clean and the state at before-enter; a
/// refusal after it leaves the authored bracket standing and the run in place
/// for the leave that unwinds it**, which is the scoped account the Spec's
/// section 3 states.
enum EnterFailure {
    BeforeLoad(LifecycleRefusal),
    AfterLoad(Box<Run>, LifecycleRefusal),
}

/// The gate's arm: its channel and the pid the leave reaps.
struct GateChannel {
    channel: OrganChannel,
    pid: nix::unistd::Pid,
}

/// The hub. Adopts the coordination end the unit's declared open delivered,
/// performs the worker's hygiene as sets and not checks, and serves loop 0.
pub struct Harness {
    coordination: CoordinationListener,
    organs: OrganBinaries,
    parameters: OrganParameters,
    state: ChannelState,
}

impl Harness {
    /// The crate's one constructor. There is no second path: the fields are
    /// private and no other function returns a `Harness`.
    ///
    /// The hygiene is two sets, not two checks: a check that finds the flag
    /// wrong and reports leaves the descriptor inheritable and the process
    /// attachable. Clearing the dumpable flag reparents the proc entries to
    /// root and refuses a same-uid attach, which is what closes the one route
    /// apex 5.1's possession-as-authentication argument assumes shut.
    pub fn listen(
        coordination: CoordinationListener,
        organs: OrganBinaries,
        parameters: OrganParameters,
    ) -> Result<Self, AdoptionFault> {
        clear_dumpable()?;
        Ok(Harness {
            coordination,
            organs,
            parameters,
            state: ChannelState::BeforeEnter,
        })
    }

    /// Serves loop 0 until leave is answered or a fault below the exchange
    /// layer ends service, waiting where a landing can arrive, per
    /// `weaver-harness-Spec` section 6.2: the coordination listener, the
    /// connection a dialed verb is being served on, the gate channel, and
    /// the decode channel, one `poll` across all of them and one wake
    /// handled at a time.
    ///
    /// Dispatch is by payload kind, and only the frame grants the seat. A
    /// directive is the lifecycle interior's, judged against the channel's
    /// state, answered or refused, out-of-order refused and not queued. A
    /// report is clerked, no turn opening and no seat granted. A frame is
    /// the one arrival owed an answer: `entry` is the loop the binary
    /// carries across the dev boundary, called with the granted surface and
    /// the parsed request, and its return is the response the frame's
    /// exchange is answered with. `identity` and `tool_schemas` are the
    /// composition root's, the c-material of the assembled prompt.
    pub fn serve<F>(
        mut self,
        identity: &str,
        tool_schemas: &[String],
        mut entry: F,
    ) -> Result<Outcome, ChannelFault>
    where
        F: FnMut(
            &mut crate::engine::Ports<'_>,
            &str,
        ) -> Result<crate::engine::TurnOutcome, crate::engine::TurnError>,
    {
        // **One connection at a time**, held in the wait rather than blocked
        // on: the listener leaves the read set while a verb's connection is
        // being served, which is what holds the contract's
        // one-exchange-in-flight rule now that no fleet map does.
        let mut pending: Option<OrganChannel> = None;
        loop {
            match self.wait(pending.as_ref()) {
                Ok(Wake::Listener) => match self.coordination.accept_root() {
                    Ok(connection) => pending = Some(connection),
                    // A peer that is not root never reaches an exchange. The
                    // listener stands and the next dial is accepted, because
                    // refusing one caller is not a reason to stop serving
                    // the one party that may call.
                    Err(ChannelFault::WrongPeer { .. }) => continue,
                    // The listener itself failed, which ends service.
                    Err(fault) => {
                        self.unwind_if_entered(weaver_types::FaultCase::ListenerLost, &fault);
                        return Err(fault);
                    }
                },
                Ok(Wake::Connection) => {
                    let connection = pending.take().expect("the wake proved a connection");
                    let (envelope, sink, state_end) = match connection.recv_with_descriptor() {
                        Ok(received) => received,
                        // The verb answered and admin closed. Ordinary, and
                        // not an ending: the run is held across connections,
                        // per `weaver-admin-harness-contract` section 4.
                        Err(ChannelFault::Closed) => continue,
                        Err(fault) => {
                            self.unwind_if_entered(
                                weaver_types::FaultCase::OrganDeathObserved,
                                &fault,
                            );
                            return Err(fault);
                        }
                    };
                    let exchange = envelope.exchange.clone();
                    let directive = match envelope.payload {
                        weaver_types::Payload::Directive(directive) => directive,
                        // Anything else on this channel is not a directive
                        // this crate can attribute to an exchange for a
                        // refusal to answer.
                        _ => {
                            self.unwind_if_entered(
                                weaver_types::FaultCase::OrganDeathObserved,
                                &ChannelFault::Undecodable,
                            );
                            return Err(ChannelFault::Undecodable);
                        }
                    };
                    match self.dispatch_on(&connection, exchange, directive, sink, state_end) {
                        Ok(Some(outcome)) => return Ok(outcome),
                        // The verb answered and the run stands: the same
                        // connection may carry another directive, so it
                        // returns to the wait.
                        Ok(None) => pending = Some(connection),
                        Err(fault) => {
                            self.unwind_if_entered(
                                weaver_types::FaultCase::OrganDeathObserved,
                                &fault,
                            );
                            return Err(fault);
                        }
                    }
                    // **The Gateless seat, granted once at the run's
                    // opening**, per `weaver-harness-Spec` section 6.2's
                    // second criterion: a diagnostic enter that just
                    // answered ready is the run arriving as the work, so
                    // the replay runs here, after the answer crossed and
                    // before the wait resumes. A serving run, and a
                    // diagnostic run already replayed, pass through.
                    if let Err(fault) = self.serve_replay(&mut pending) {
                        self.unwind_if_entered(
                            weaver_types::FaultCase::OrganDeathObserved,
                            &fault,
                        );
                        return Err(fault);
                    }
                }
                Ok(Wake::Gate) => {
                    if let Err(fault) =
                        self.serve_gate_wake(identity, tool_schemas, &mut entry, &mut pending)
                    {
                        self.unwind_if_entered(
                            weaver_types::FaultCase::OrganDeathObserved,
                            &fault,
                        );
                        return Err(fault);
                    }
                }
                Ok(Wake::Decode) => {
                    // One at-rest arrival is legitimate: the cancel race's
                    // residue. A stop whose cancel lost to a natural
                    // completion reaches the SPU at rest and answers
                    // `AtRest`, which surfaces here and drains. Everything
                    // else is octets the seam's state cannot read, the
                    // report's wire case being issue 106's open election: a
                    // fault below the exchange layer, ending service the
                    // way section 3 ends it.
                    let verdict = match &self.state {
                        ChannelState::Entered(run) => {
                            run.spu.as_ref().map(|spu| spu.decode.recv_reply())
                        }
                        _ => None,
                    };
                    match verdict {
                        Some(Ok(crate::channel::DecodeReply::Answer(TokenAnswer::AtRest))) => {}
                        // A reply that decoded and is not the residue is the
                        // one place the synthetic fault is honest: the octets
                        // read, the seam's state cannot.
                        Some(Ok(_)) | None => {
                            self.unwind_if_entered(
                                weaver_types::FaultCase::OrganDeathObserved,
                                &ChannelFault::Undecodable,
                            );
                            return Err(ChannelFault::Undecodable);
                        }
                        // The channel's own fault is retained, so the fault
                        // event's account carries what the seam met rather
                        // than a synthetic stand-in.
                        Some(Err(fault)) => {
                            self.unwind_if_entered(
                                weaver_types::FaultCase::OrganDeathObserved,
                                &fault,
                            );
                            return Err(fault);
                        }
                    }
                }
                Err(fault) => {
                    self.unwind_if_entered(weaver_types::FaultCase::ListenerLost, &fault);
                    return Err(fault);
                }
            }
        }
    }

    /// One wake from the entered-state wait, per `weaver-harness-Spec`
    /// section 6.2. The read set is what can originate now: the listener
    /// when no verb is mid-service, the verb's connection while one is, and
    /// the gate and decode channels while a run stands. `poll` sleeps
    /// against all of them and wakes on the first ready, serial as ever,
    /// and no executor enters.
    fn wait(&self, pending: Option<&OrganChannel>) -> Result<Wake, ChannelFault> {
        use std::os::fd::AsFd;

        use nix::poll::{PollFd, PollFlags, PollTimeout, poll};
        loop {
            let mut fds: Vec<PollFd<'_>> = Vec::with_capacity(4);
            let mut wakes: Vec<Wake> = Vec::with_capacity(4);
            match pending {
                Some(connection) => {
                    fds.push(PollFd::new(connection.as_fd(), PollFlags::POLLIN));
                    wakes.push(Wake::Connection);
                }
                None => {
                    fds.push(PollFd::new(self.coordination.as_fd(), PollFlags::POLLIN));
                    wakes.push(Wake::Listener);
                }
            }
            if let ChannelState::Entered(run) = &self.state {
                if let Some(gate) = &run.gate {
                    fds.push(PollFd::new(gate.channel.as_fd(), PollFlags::POLLIN));
                    wakes.push(Wake::Gate);
                }
                if let Some(spu) = &run.spu {
                    fds.push(PollFd::new(spu.decode.as_fd(), PollFlags::POLLIN));
                    wakes.push(Wake::Decode);
                }
            }
            match poll(&mut fds, PollTimeout::NONE) {
                Ok(_) => {}
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return Err(ChannelFault::Closed),
            }
            let woken = PollFlags::POLLIN | PollFlags::POLLHUP | PollFlags::POLLERR;
            for (fd, wake) in fds.iter().zip(&wakes) {
                let revents = fd.revents().unwrap_or(PollFlags::empty());
                // An invalid descriptor never becomes readable, so treating
                // POLLNVAL as anything but an ending would spin this loop
                // forever against a wake that cannot arrive.
                if revents.contains(PollFlags::POLLNVAL) {
                    return Err(ChannelFault::Closed);
                }
                if revents.intersects(woken) {
                    return Ok(*wake);
                }
            }
        }
    }

    /// The diagnostic seat's one grant, per `weaver-harness-Spec` section
    /// 6.2: under a diagnostic binding the seat is granted once, at the
    /// run's opening, on the run itself as the work - the operator's
    /// sealed preload is what arrived owed an answer and the certification
    /// is the answer it is owed. No frame ever grants this seat, and a
    /// serving run never reaches the entry. Every outcome of the replay
    /// itself lands in the diagnostic-trace: only a channel-level loss
    /// surfaces here, ending service the way a serving turn's loss does.
    fn serve_replay(&mut self, pending: &mut Option<OrganChannel>) -> Result<(), ChannelFault> {
        let ChannelState::Entered(run) = &mut self.state else {
            return Ok(());
        };
        let Some(seat) = run.diagnostic.as_mut() else {
            return Ok(());
        };
        if seat.replayed {
            return Ok(());
        }
        seat.replayed = true;
        let reader_elected = seat.reader_elected;
        let artifact = seat.artifact.clone();
        let Some(spu) = run.spu.as_ref() else {
            return Ok(());
        };
        let mut ports = crate::engine::Ports::grant(
            &spu.decode,
            &run.author,
            &mut run.recorder,
            &mut run.turn_ordinal,
            // No frame, no prompt: the replay feeds the record's rendered
            // forms and assembles nothing.
            None,
            &self.coordination,
            Some(pending),
            // No Gate stands under this binding, by construction.
            None,
            run.state.as_mut(),
            run.classify.as_ref().map(|arm| &arm.channel),
            &mut run.fullness,
            &mut run.pressure_reported,
        );
        match crate::replay::drive(&mut ports, reader_elected, &artifact) {
            Ok(()) => Ok(()),
            Err(crate::engine::TurnError::ChannelLost) => Err(ChannelFault::Closed),
            // Refusals and faults are already the record's, authored
            // inside the pass, and the run stands for the operator's
            // leave.
            Err(_) => Ok(()),
        }
    }

    /// A gate-channel wake, dispatched by payload kind per section 6.2: a
    /// frame grants the seat, a report is clerked, and the gate gone is the
    /// survivor's fault to report, the run standing so admin can still
    /// leave.
    fn serve_gate_wake<F>(
        &mut self,
        identity: &str,
        tool_schemas: &[String],
        entry: &mut F,
        pending: &mut Option<OrganChannel>,
    ) -> Result<(), ChannelFault>
    where
        F: FnMut(
            &mut crate::engine::Ports<'_>,
            &str,
        ) -> Result<crate::engine::TurnOutcome, crate::engine::TurnError>,
    {
        let envelope = {
            let ChannelState::Entered(run) = &mut self.state else {
                return Ok(());
            };
            let Some(gate) = run.gate.as_ref() else {
                return Ok(());
            };
            match gate.channel.recv() {
                Ok(envelope) => envelope,
                // The gate died with the hook raised: the boundary is gone
                // and the interior is healthy, which is the survivor's fault
                // to report, per the fault-carrier ruling. The run stands so
                // the operator can still leave; the agent is unreachable
                // until then.
                Err(ChannelFault::Closed) => {
                    let gate = run.gate.take().expect("the arm stood");
                    drop(gate.channel);
                    reap(gate.pid);
                    let _ = run.author.author_fault(
                        &mut run.recorder,
                        Subsystem::Harness,
                        None,
                        &crate::authorship::harness_report(
                            weaver_types::FaultCase::OrganDeathObserved,
                            r#"{"organ":"gate","death":"channel closed with the hook raised"}"#,
                        ),
                    );
                    return Ok(());
                }
                Err(fault) => return Err(fault),
            }
        };
        match envelope.payload {
            weaver_types::Payload::Frame(frame) => self.serve_frame(
                envelope.exchange,
                frame,
                identity,
                tool_schemas,
                entry,
                pending,
            ),
            // The gate's fault report is clerked to the record per the
            // fault-carrier ruling: the report arrives whole, the gate having
            // named its own case per `weaver-types-Spec` section 4.2, and the
            // harness authors what it is handed. No sender exists in the gate
            // today, which the working list registers against the relay act,
            // but the receive side no longer fabricates an account when one
            // arrives.
            weaver_types::Payload::Fault(report) => {
                if let ChannelState::Entered(run) = &mut self.state {
                    let _ =
                        run.author
                            .author_fault(&mut run.recorder, Subsystem::Gate, None, &report);
                }
                Ok(())
            }
            // Nothing else is the gate's to open on this seam.
            _ => Err(ChannelFault::Undecodable),
        }
    }

    /// A frame grants the seat, per section 6.2 and the dev boundary: the
    /// parse at the threshold, the entry with the granted surface, and the
    /// response frame answered on the exchange. The parse refuses rather
    /// than faults, per the ruling: a line that is not the request answers
    /// `refused` as content and the channel stands.
    fn serve_one_frame<F>(
        &mut self,
        exchange: ExchangeId,
        frame: weaver_types::TurnFrame,
        identity: &str,
        tool_schemas: &[String],
        entry: &mut F,
        pending: &mut Option<OrganChannel>,
    ) -> Result<(), ChannelFault>
    where
        F: FnMut(
            &mut crate::engine::Ports<'_>,
            &str,
        ) -> Result<crate::engine::TurnOutcome, crate::engine::TurnError>,
    {
        let ChannelState::Entered(run) = &mut self.state else {
            return Ok(());
        };
        // Taken before the borrow the seat needs, so a close can name the run
        // without holding the run across the grant.
        let run_ref = run.run.clone();
        let response = match parse_request(&frame) {
            Err(reason) => render_close("refused", "reason", reason, None),
            Ok(text) => {
                // The seat, granted at loaded-and-idle: the assembly read,
                // the grant across the dev boundary, and the take-back at
                // the entry's return.
                let structure = run
                    .recorder
                    .structure()
                    .expect("the frame path runs under the serving record");
                let prompt = crate::assembly::assemble(structure, identity, tool_schemas);
                if prompt.undecodable > 0 {
                    // An incomplete prompt does not cross the seam: the hole
                    // becomes the fault event and the turn does not run.
                    let account = format!(
                        "{{\"organ\":\"harness\",\"undecodable-message-records\":{}}}",
                        prompt.undecodable
                    );
                    let _ = run.author.author_fault(
                        &mut run.recorder,
                        Subsystem::Harness,
                        None,
                        &crate::authorship::harness_report(
                            weaver_types::FaultCase::MessageRecordUndecodable,
                            &account,
                        ),
                    );
                    render_close(
                        "stopped",
                        "reason",
                        "the working structure holds a hole",
                        None,
                    )
                } else {
                    let Some(spu) = run.spu.as_ref() else {
                        // A run with no SPU is not loaded, and nothing can
                        // turn against it.
                        return Err(ChannelFault::Undecodable);
                    };
                    let gate_port = run.gate.as_ref().map(|gate| crate::engine::GatePort {
                        channel: &gate.channel,
                        ordinal: &mut run.gate_ordinal,
                        held: &mut run.held_frames,
                    });
                    let mut ports = crate::engine::Ports::grant(
                        &spu.decode,
                        &run.author,
                        &mut run.recorder,
                        &mut run.turn_ordinal,
                        Some(prompt),
                        &self.coordination,
                        Some(pending),
                        gate_port,
                        run.state.as_mut(),
                        run.classify.as_ref().map(|arm| &arm.channel),
                        &mut run.fullness,
                    &mut run.pressure_reported,
                    );
                    match entry(&mut ports, &text) {
                        // A turn the operator's stop aborted answers the
                        // stopped close, the partial standing in the record.
                        Ok(outcome) if outcome.aborted => render_close(
                            "stopped",
                            "reason",
                            "the operator stopped the turn",
                            Some((&outcome.turn, &run_ref)),
                        ),
                        // A model-side stop is a completed turn whose
                        // truncation the record holds, so the client is
                        // answered with what stands.
                        Ok(outcome) => render_close_with_finish(
                            "answered",
                            "text",
                            &outcome.emission,
                            Some((&outcome.turn, &run_ref)),
                            outcome.truncated,
                        ),
                        Err(crate::engine::TurnError::Refused { turn, refusal }) => render_close(
                            "stopped",
                            "reason",
                            refusal_reason(&refusal),
                            Some((&turn, &run_ref)),
                        ),
                        // The emission arrived mid-stream. **The engine
                        // already authored the report inside the turn's
                        // bracket**, before the close its error path lands,
                        // so this arm renders the client's close and authors
                        // nothing: a second authoring here would file one
                        // fact twice, and filing it here at all would put it
                        // after `turn.closed`. Service continues, the fault
                        // being one the worker survives by definition of the
                        // case set.
                        Err(crate::engine::TurnError::Faulted { turn, report: _ }) => render_close(
                            "stopped",
                            "reason",
                            "the model organ reported a fault",
                            Some((&turn, &run_ref)),
                        ),
                        Err(crate::engine::TurnError::Unlicensed { turn }) => render_close(
                            "stopped",
                            "reason",
                            "a message was not licensed for its role",
                            Some((&turn, &run_ref)),
                        ),
                        // The decode channel or the record is gone, and the
                        // record is untrustworthy either way: service ends.
                        Err(crate::engine::TurnError::ChannelLost) => {
                            return Err(ChannelFault::Closed);
                        }
                    }
                }
            }
        };
        let Some(gate) = run.gate.as_ref() else {
            return Ok(());
        };
        gate.channel.send(&OrganEnvelope {
            exchange,
            position: Position::Close,
            payload: weaver_types::Payload::Frame(weaver_types::TurnFrame::carry(
                response.as_bytes(),
            )),
        })?;
        Ok(())
    }

    /// One frame's turn, then the shelf, per [`Run::held_frames`]: the
    /// frames the turn held are served after its close, in arrival order. A
    /// client that spoke while an execution was awaited was neither dropped
    /// nor answered out of order: its envelope waited on the run's shelf
    /// and is served here as if just received, the one-turn discipline
    /// preserved across the executions inside a turn.
    ///
    /// **The drain is a flat loop, never a recursive serve per held
    /// frame.** A held frame's own turn can hold more frames, and a serve
    /// that recursed would let a client that speaks through every execution
    /// grow the stack with the shelf; the loop re-reads the queue after
    /// each turn instead, the depth constant however long the shelf runs.
    fn serve_frame<F>(
        &mut self,
        exchange: ExchangeId,
        frame: weaver_types::TurnFrame,
        identity: &str,
        tool_schemas: &[String],
        entry: &mut F,
        pending: &mut Option<OrganChannel>,
    ) -> Result<(), ChannelFault>
    where
        F: FnMut(
            &mut crate::engine::Ports<'_>,
            &str,
        ) -> Result<crate::engine::TurnOutcome, crate::engine::TurnError>,
    {
        self.serve_one_frame(exchange, frame, identity, tool_schemas, entry, pending)?;
        loop {
            let held = {
                let ChannelState::Entered(run) = &mut self.state else {
                    return Ok(());
                };
                let Some(held) = run.held_frames.pop_front() else {
                    return Ok(());
                };
                held
            };
            match held.payload {
                weaver_types::Payload::Frame(frame) => {
                    self.serve_one_frame(
                        held.exchange,
                        frame,
                        identity,
                        tool_schemas,
                        entry,
                        pending,
                    )?;
                }
                weaver_types::Payload::Fault(report) => {
                    let ChannelState::Entered(run) = &mut self.state else {
                        return Ok(());
                    };
                    let turn = run.turn_in_flight.clone();
                    let _ = run.author.author_fault(
                        &mut run.recorder,
                        Subsystem::Gate,
                        turn.as_ref(),
                        &report,
                    );
                }
                // Nothing else is the gate's to open, and an answer with no
                // exchange awaiting it correlates to nothing: dropped as the
                // protocol noise it is rather than faulting the run.
                _ => {}
            }
        }
    }

    /// Unwinds a standing run, for a path that ends service without a
    /// leave, and records why before it does. A worker that dies seconds
    /// after a fault with no fault event in the stream is the record gap
    /// issue #221 filed: the stream is the program's one fault carrier,
    /// and a silent death lies by omission. The case is the caller's,
    /// because the site that met the fault knows which party it lost, and
    /// the account carries the fault's own spelling.
    fn unwind_if_entered(&mut self, case: weaver_types::FaultCase, fault: &ChannelFault) {
        if let ChannelState::Entered(run) = std::mem::replace(&mut self.state, ChannelState::Left) {
            let mut run = *run;
            let turn = run.turn_in_flight.clone();
            let account = serde_json::json!({
                "organ": "harness",
                "service-ended": format!("{fault:?}"),
            })
            .to_string();
            let _ = run.author.author_fault(
                &mut run.recorder,
                Subsystem::Harness,
                turn.as_ref(),
                &crate::authorship::harness_report(case, &account),
            );
            let _ = leave(&mut run);
        }
    }

    /// The serial service's one step. Returns `Some(outcome)` when service
    /// ends. The exchange the directive opened is carried through, because an
    /// answer closes the exchange the directive named rather than one this
    /// crate invents.
    fn dispatch_on(
        &mut self,
        connection: &OrganChannel,
        exchange: ExchangeId,
        directive: LifecycleDirective,
        sink: Option<OwnedFd>,
        state_end: Option<OwnedFd>,
    ) -> Result<Option<Outcome>, ChannelFault> {
        match (&mut self.state, directive) {
            (ChannelState::BeforeEnter, LifecycleDirective::Enter { payload }) => {
                match self.enter(payload, sink, state_end) {
                    Ok(run) => {
                        self.state = ChannelState::Entered(Box::new(run));
                        self.answer(connection, &exchange, LifecycleAnswer::Ready)?;
                    }
                    Err(EnterFailure::BeforeLoad(refusal)) => {
                        // Nothing was authored and nothing stood up, so the
                        // stream is clean and the position holds.
                        self.refuse(connection, &exchange, refusal)?;
                    }
                    Err(EnterFailure::AfterLoad(mut run, refusal)) => {
                        // **The refusal is clerked before the answer goes
                        // out**, per `weaver-harness-Spec` section 6 and the
                        // admin contract's clause of 2026-08-22: a bracket
                        // that stands is a run whose record can say why its
                        // enter was refused, and the answer alone leaves
                        // nothing behind. This is the one site for all
                        // thirteen paths that raise a lifecycle refusal, a
                        // refusal being one kind of event whatever raised
                        // it.
                        author_lifecycle_refusal(&mut run, &refusal);
                        // The bracket stands, so the run stays in place for
                        // the leave that unwinds it. Dropping it here would
                        // orphan what forked and leave the bracket unclosed.
                        self.state = ChannelState::Entered(run);
                        self.refuse(connection, &exchange, refusal)?;
                    }
                }
                Ok(None)
            }
            (ChannelState::Entered(run), LifecycleDirective::Leave) => {
                if run.turn_in_flight.is_some() {
                    self.refuse(connection, &exchange, LifecycleRefusal::ActivityNotAtRest)?;
                    return Ok(None);
                }
                let mut run = match std::mem::replace(&mut self.state, ChannelState::Left) {
                    ChannelState::Entered(run) => *run,
                    // Unreachable: the match arm above proved the position.
                    other => {
                        self.state = other;
                        return Err(ChannelFault::Undecodable);
                    }
                };
                match leave(&mut run) {
                    Ok(()) => {
                        self.answer(connection, &exchange, LifecycleAnswer::Left)?;
                    }
                    // Everything admitted did not reach the stream, so the
                    // answer says so rather than claiming a clean close.
                    Err(refusal) => {
                        self.refuse(connection, &exchange, refusal)?;
                    }
                }
                Ok(Some(Outcome::Left))
            }
            (ChannelState::Entered(run), LifecycleDirective::Stop) => {
                // Announce after record: the turn's close event is placed with
                // the stop reason, and only then does the answer carry
                // TurnAborted.
                let answer = match run.turn_in_flight.take() {
                    Some(turn) => {
                        let _ = run.author.author(
                            &mut run.recorder,
                            Kind::TurnClosed,
                            Subsystem::Harness,
                            Some(&turn),
                            Some(Payload::TurnClosed(TurnClose::Stopped {
                                reason: weaver_trace::StopReason::Directive,
                            })),
                        );
                        LifecycleAnswer::TurnAborted {
                            turn: TurnKey(turn.0.clone()),
                        }
                    }
                    // A stop at rest is a clean close and not a refusal.
                    None => LifecycleAnswer::AtRest,
                };
                self.answer(connection, &exchange, answer)?;
                Ok(None)
            }
            // Every other pairing is out of order for the position. The left
            // position is terminal, so a directive of any kind arriving after
            // a leave answers the same refusal.
            _ => {
                self.refuse(connection, &exchange, LifecycleRefusal::OutOfOrder)?;
                Ok(None)
            }
        }
    }

    /// Enter runs the charter's steps in order, and the answer is the
    /// aggregate. The fan-out's extent follows the match on the payload's
    /// binding, per `weaver-harness-Spec` section 3: the serving case runs
    /// the whole walk and the diagnostic case ends it at the SPU's
    /// confirmation, forking no gate.
    fn enter(
        &mut self,
        payload: weaver_types::EnterPayload,
        sink: Option<OwnedFd>,
        state_end: Option<OwnedFd>,
    ) -> Result<Run, EnterFailure> {
        // The sink descriptor arrives on the directive's own message, the
        // state channel's end beside it where the member stands, per the
        // ruling of 2026-08-26. The sink's absence refuses and the end's
        // absence is the leg not standing, never a refusal. Nothing is
        // authored yet, so a refusal here leaves the stream clean.
        let sink = sink.ok_or(EnterFailure::BeforeLoad(
            LifecycleRefusal::DescriptorsUnusable,
        ))?;
        let session = payload.session.clone();
        // **The kind selects the record's mechanism**, per apex section 6 as
        // ruled 2026-08-24 and `weaver-harness-Spec` section 9's settled
        // election: a serving run authors the trace and a diagnostic run
        // authors a diagnostic-trace, one shape over both arms, this crate
        // the sole writer under either. The diagnostic record opens with
        // `replay.opened`, the loop's to author, so the load bracket and
        // the seated prefix below are the serving arm's alone.
        let diagnostic = matches!(payload.binding, weaver_types::EnterBinding::Diagnostic);
        let mut recorder = if diagnostic {
            crate::record::Record::Diagnostic(
                weaver_diagnostic::Recorder::receive(
                    sink,
                    weaver_diagnostic::RunRef(payload.run.0.clone()),
                    weaver_diagnostic::SessionRef(session.0.clone()),
                )
                .map_err(|_| EnterFailure::BeforeLoad(LifecycleRefusal::DescriptorsUnusable))?,
            )
        } else {
            crate::record::Record::Serving(
                Recorder::receive(
                    sink,
                    RunRef(payload.run.0.clone()),
                    SessionRef(session.0.clone()),
                )
                .map_err(|_| EnterFailure::BeforeLoad(LifecycleRefusal::DescriptorsUnusable))?,
            )
        };

        // The state seam, per `weaver-harness-state-contract` as ruled
        // 2026-08-26: a nameless pair whose harness end arrived on this
        // enter when admin stood the member, and its absence is the leg not
        // standing, never a refused load. Attached before the load event is authored
        // so the run's opening distills like everything after it. The
        // election is the declaration's, resolved by admin and carried in
        // the enter per the contract's sections 3 and 5, converted here at
        // the one site the way the session reference is: the floor's
        // spelling in, the tee's own out. The ask end is a clone of the
        // same channel, per Spec section 6, taken before the tee owns the
        // original, and it stands only where the tee does: one seam, both
        // directions or neither.
        let election = weaver_trace::Election {
            all_kinds: payload.state_election.all_kinds,
            keys: payload
                .state_election
                .keys
                .iter()
                .map(|elected| weaver_trace::ElectedKind {
                    kind: elected.kind.clone(),
                    paths: elected.paths.clone(),
                })
                .collect(),
        };
        // The record carries the same rule the tee applies, cloned before
        // the seam consumes it and written whether or not the seam stands,
        // per `weaver-trace-Spec` section 3: the rule is the load's fact
        // and not the channel's, and absence on a record is reserved for
        // one that predates the member.
        let tee_election = election.clone();
        let mut state_seam = None;
        // The channel arrived in the enter directive as the harness half of
        // the pair admin created at the member's spawn, per the ruling of
        // 2026-08-26: dialed nowhere, derived from nothing, possession the
        // authentication. No end means no member stood, the leg down and
        // the load unrefused.
        if let Some(end) = state_end {
            let channel = std::os::unix::net::UnixStream::from(end);
            match recorder.serving_mut() {
                Some(serving) => {
                    let ask_end = channel.try_clone();
                    // The session rides the opener beside the election, both
                    // being facts this enter declared, per the contract's
                    // amended term: the custodian bounds its answers to it,
                    // so a store holding an earlier session answers within
                    // the running one.
                    if let Ok(tee) =
                        weaver_trace::Tee::open(channel, session.0.clone(), election)
                    {
                        serving.attach_tee(tee);
                        state_seam = ask_end.ok().map(crate::state::StateSeam::new);
                    }
                }
                // **No tee under the diagnostic record**: the tee distills a
                // serving trace into the store, and a replay reads the store
                // through this same channel, which is the ask end whole -
                // the third refusal, nothing writes back to what is under
                // examination, per `weaver-diagnostic-PRD` section 2. **The
                // opener still crosses**, because the contract makes it the
                // channel's first traffic on every standing, tee or no tee:
                // the custodian reads its session bound from it, and a
                // channel whose first line is an ask would have that ask
                // eaten as an opener naming no session. The harness sends
                // it here, the same rendering the tee would have sent.
                None => {
                    use std::io::Write;
                    let mut channel = channel;
                    let frame = weaver_trace::opener(&session.0, &election);
                    if channel.write_all(frame.as_bytes()).is_ok() {
                        state_seam = Some(crate::state::StateSeam::new(channel));
                    }
                }
            }
        }

        // The load event is the run's opening and the origin of its monotonic
        // clock, so the author is constructed at this moment.
        let author = Author::new(&session, &payload.run);
        // **The load declares the posture it was written in**, per
        // `weaver-trace-PRD` section 3.1: each diagnostic election of this
        // enter, named individually and none bundled. This crate is the
        // party that holds them already, and they arrive by two routes per
        // `weaver-harness-Spec` section 6.1: the readout, the field, and
        // the surprisal ride the SPU instruction, and the tee's rule is a
        // sibling of the instruction on the enter payload, converted above
        // for the seam and cloned for the record. The declaration still
        // costs one read of what is in hand. Without it a record holding
        // no field and a record whose election stood and produced nothing
        // are one absence on disk.
        let elections = weaver_trace::Elections {
            residual_readout: payload.spu_instruction.decoder.residual_readout_election,
            field: payload
                .spu_instruction
                .decoder
                .field_election
                .as_ref()
                .map(|election| election.depth),
            surprisal: payload.spu_instruction.decoder.surprisal_election,
            tee: Some(tee_election),
        };
        // **The serving arm's alone**: the diagnostic record carries no
        // `load` kind and opens with `replay.opened`, which is the loop's
        // to author at the replay, per `weaver-diagnostic-Spec` section 4.
        if !diagnostic {
            author
                .author(
                    &mut recorder,
                    Kind::Load,
                    Subsystem::Harness,
                    None,
                    Some(Payload::Elections(elections)),
                )
                .map_err(|_| EnterFailure::BeforeLoad(LifecycleRefusal::Malformed))?;
        }

        // **The seated prefix reaches the record beside the load**, per
        // `weaver-harness-Spec` section 6 and `weaver-trace-PRD` section 5.
        // The accumulation rule of the trace charter's section 3.2 bases the
        // effective context on the identity prefix, and before this the
        // prefix lived in the configuration alone, so a consumer holding the
        // record could not close the reconstruction. The write is one read of
        // what is in hand: the same instruction the elections came from.
        //
        // A refusal here is a defect in the declaration rather than a reason
        // to abandon the load, per `weaver-harness-PRD` section 5's fifth
        // case: the door refuses a role that is not system and a load that
        // has already bracketed does not fail on a record it could not
        // write, so what is owed is the account rather than the abort. **The
        // refusal is authored rather than dropped**, because the prefix is
        // seated at the session's open regardless and a dropped refusal
        // launders the declaration by omission - the reader cannot tell an
        // agent that seated no prefix from one that seated an unrecorded
        // one, which is the door's stated purpose defeated by its caller.
        // Where the rule that judges a declaration lands is `weaver-types`'
        // to say, and until it does this is the record's only account.
        //
        // conforms: harness-identity-refusal-authored-not-dropped
        if !diagnostic {
            seat_identity_prefix(
                &author,
                &mut recorder,
                &payload.spu_instruction.decoder.identity,
            );
        }

        // **Past this line the bracket stands**, so every refusal below
        // carries the partial run back rather than dropping it: the leave that
        // follows unwinds exactly what stood up, closes the bracket, and
        // drains. Dropping it here would orphan whatever forked and leave an
        // unclosed bracket on the stream.
        let mut run = Run {
            classify: None,
            recorder,
            author,
            session,
            run: payload.run.clone(),
            spu: None,
            gate: None,
            state: state_seam,
            fullness: None,
            pressure_reported: false,
            turn_in_flight: None,
            spu_ordinal: 0,
            gate_ordinal: 0,
            held_frames: std::collections::VecDeque::new(),
            turn_ordinal: 0,
            diagnostic: diagnostic.then(|| DiagnosticSeat {
                reader_elected: payload.spu_instruction.decoder.residual_readout_election,
                artifact: payload
                    .spu_instruction
                    .decoder
                    .model_binding
                    .artifact
                    .0
                    .clone(),
                replayed: false,
            }),
        };

        macro_rules! after_load {
            ($run:expr, $refusal:expr) => {
                return Err(EnterFailure::AfterLoad(Box::new($run), $refusal))
            };
        }

        // The residency and decode pairs are created in one act before the SPU
        // fork: a socket with no address cannot be reached later by resolving
        // one, so the only moment it can reach a child is before that child
        // exists.
        let (lifecycle, lifecycle_child) = match OrganChannel::pair() {
            Ok(pair) => pair,
            Err(_) => after_load!(run, LifecycleRefusal::DescriptorsUnusable),
        };
        let (decode, decode_child) = match DecodeChannel::pair() {
            Ok(pair) => pair,
            Err(_) => after_load!(run, LifecycleRefusal::DescriptorsUnusable),
        };
        // SAFETY: the fork runs on the serving thread, whose lifetime is the
        // worker's, and the child performs only the async-signal-safe calls of
        // `place_child_ends` before its exec.
        let spu_pid = match unsafe {
            crate::spawn::fork_organ(
                &self.organs.spu,
                &[&lifecycle_child, &decode_child],
                &self.parameters.spu_arguments(),
            )
        } {
            Ok(pid) => pid,
            Err(_) => after_load!(run, LifecycleRefusal::BindFailed),
        };
        drop(lifecycle_child);
        drop(decode_child);
        run.spu = Some(SpuChannels {
            lifecycle,
            decode,
            pid: spu_pid,
        });

        // Admit carries the SPU instruction uninterpreted.
        run.spu_ordinal += 1;
        let ordinal = run.spu_ordinal;
        let spu = run.spu.as_ref().expect("the arm just stood up");
        if let Err(refusal) = exchange(
            &spu.lifecycle,
            Opener::Harness,
            ordinal,
            weaver_types::RefusingOrgan::Spu,
            LifecycleDirective::Admit {
                instruction: payload.spu_instruction.clone(),
            },
        ) {
            // A closed channel means the organ is gone: its exit status says
            // whether the placement or the exec failed, which the exchange
            // itself could only report as an absent residency.
            let refusal = match refusal {
                LifecycleRefusal::NoResidency => {
                    let pid = run.spu.as_ref().expect("the arm stood up").pid;
                    classify_organ_death(pid).unwrap_or(LifecycleRefusal::NoResidency)
                }
                other => other,
            };
            after_load!(run, refusal);
        }

        // **The decode session opens once residency confirms**, per
        // `weaver-harness-Spec` section 6.1: the session opens at the enter
        // fan-out and not at the first turn, so the interior loop 0 grants is
        // a session at rest. The open carries the instruction's identity as
        // its messages and the run's session as its session, read from the
        // payload this crate already holds. A refused open is a refused enter,
        // returned after-load so the bracket stands for the leave.
        let spu = run.spu.as_ref().expect("residency stands");
        // **The column ask crosses where and only where the binding is
        // diagnostic and the readout is elected**, per `weaver-spu-PRD`
        // section 13.7's cadence election and the decode contract's
        // supplies clause: a serving harness never writes the ask, and a
        // diagnostic load without the readout election has no tap running
        // and no column to ask for - the ask would meet the registry's
        // second arm.
        let column_ask = run.diagnostic.is_some()
            && payload.spu_instruction.decoder.residual_readout_election;
        if let Err(refusal) = open_session(
            &spu.decode,
            SessionId(run.session.0.clone()),
            payload.spu_instruction.decoder.identity.clone(),
            column_ask,
        ) {
            after_load!(run, refusal);
        }

        // The classify arm, per `weaver-harness-Spec` section 6 and
        // `weaver-spu-PRD` section 15.3: where the declaration carries the
        // binding the fan-out grows this arm, the model having admitted
        // first so a device too small for both refuses deterministically at
        // this arm and names it. Absent binding, no process: the port then
        // serves the missing leg's absence.
        if let Some(instruction) = payload.spu_instruction.classify.as_ref() {
            let Some(binary) = self.organs.classify.as_ref() else {
                // A binding with no binary provisioned is half an arm, and
                // half an arm refuses rather than standing.
                after_load!(run, LifecycleRefusal::ConfigInvalid {
                    field: Some(weaver_types::FieldName("classify".into())),
                });
            };
            let (channel, child) = match crate::channel::ClassifyChannel::pair() {
                Ok(pair) => pair,
                Err(_) => after_load!(run, LifecycleRefusal::DescriptorsUnusable),
            };
            let Some(device) = instruction.model_binding.devices.first() else {
                after_load!(run, LifecycleRefusal::ConfigInvalid {
                    field: Some(weaver_types::FieldName("classify".into())),
                });
            };
            let arguments = vec![
                instruction.model_binding.artifact.0.clone(),
                device.0.to_string(),
            ];
            // SAFETY: as the SPU's fork above, on the serving thread, the
            // child performing only the async-signal-safe calls.
            let pid = match unsafe {
                crate::spawn::fork_organ(binary, &[&child], &arguments)
            } {
                Ok(pid) => pid,
                Err(_) => after_load!(run, LifecycleRefusal::BindFailed),
            };
            drop(child);
            let arm = ClassifyArm { channel, pid };
            // Readiness gates service, per the contract: the seam's first
            // message is the admission's outcome, and a typed refusal, a
            // closure, or the bound's expiry refuses the load whole. The
            // wait is bounded because a child hung mid-admission would
            // otherwise hold the load forever, and the expiry path kills
            // before it reaps: a process still loading weights does not
            // exit on a channel it has not read yet.
            match arm.channel.recv_reply_within(CLASSIFY_READY_BOUND_MS) {
                Ok(crate::channel::ClassifyReply::Answer(
                    weaver_types::LabelAnswer::Ready,
                )) => {
                    run.classify = Some(arm);
                }
                _ => {
                    drop(arm.channel);
                    let _ = nix::sys::signal::kill(arm.pid, nix::sys::signal::Signal::SIGKILL);
                    reap(arm.pid);
                    after_load!(run, LifecycleRefusal::NoResidency);
                }
            }
        }

        // The fan-out's extent follows the kind, per `weaver-harness-Spec`
        // section 3: the serving case runs the gate step and the diagnostic
        // case ends the fan-out at the SPU's confirmation - no pair created,
        // no binary forked, no raise opened. The run state's gate arm stays
        // empty for the whole residency, so the leave and the unwind lower
        // what stands, reading the same option they always read.
        let gate_instruction = match &payload.binding {
            weaver_types::EnterBinding::Serving { gate_instruction } => gate_instruction,
            weaver_types::EnterBinding::Diagnostic => return Ok(run),
        };

        // The gate pair is created only after the SPU's answer confirms
        // residency: one organ's readiness gates another organ's
        // construction, which neither organ can see from inside its domain.
        let (gate, gate_child) = match OrganChannel::pair() {
            Ok(pair) => pair,
            Err(_) => after_load!(run, LifecycleRefusal::DescriptorsUnusable),
        };
        // SAFETY: as above.
        let gate_pid =
            match unsafe { crate::spawn::fork_organ(&self.organs.gate, &[&gate_child], &[]) } {
                Ok(pid) => pid,
                Err(_) => after_load!(run, LifecycleRefusal::BindFailed),
            };
        drop(gate_child);
        run.gate_ordinal += 1;
        if let Err(refusal) = exchange(
            &gate,
            Opener::Harness,
            run.gate_ordinal,
            weaver_types::RefusingOrgan::Gate,
            LifecycleDirective::Raise {
                instruction: gate_instruction.clone(),
                // The socket is this crate's rather than the declaration's,
                // per `weaver-gate-PRD` section 2, named beside the
                // coordination socket so the manager's runtime directory
                // covers both and no pathname outlives its worker.
                socket: self.coordination.gate_socket(),
            },
        ) {
            let refusal = match refusal {
                // Only a classified death replaces the exchange's own
                // refusal: an unreadable status is not evidence of a failed
                // bind, and claiming one would tell the peer something this
                // crate does not know.
                LifecycleRefusal::NoResidency => {
                    classify_organ_death(gate_pid).unwrap_or(LifecycleRefusal::NoResidency)
                }
                other => other,
            };
            // The gate stood up far enough to reap even though it refused.
            run.gate = Some(GateChannel {
                channel: gate,
                pid: gate_pid,
            });
            after_load!(run, refusal);
        }
        run.gate = Some(GateChannel {
            channel: gate,
            pid: gate_pid,
        });

        Ok(run)
    }

    /// The extension seam, crossed at loaded-and-idle: loop 0 hands a standing
    /// interior to whatever loop 1 the binary carries, and takes it back at
    /// the stop and at the leave. A loop composes what this grants or does not
    /// compile - there is no call by which it mints a port.
    pub fn grant_seat(
        &mut self,
        identity: &str,
        tool_schemas: &[String],
    ) -> Option<crate::Ports<'_>> {
        match &mut self.state {
            // **The extension seam is crossed at loaded-and-idle itself.** A
            // turn in flight is the active state, not the idle one, and loop 0
            // has not taken the interior back yet, so there is no standing
            // interior to hand across.
            ChannelState::Entered(run) if run.turn_in_flight.is_some() => None,
            ChannelState::Entered(run) => {
                // **A diagnostic run grants no seat here.** Its record holds
                // no working structure and no frame ever arrives to assemble
                // for - the one grant is the run's opening, per Spec 6.2's
                // second criterion - so this seam answers the same
                // nothing-to-hand-across the other arms give rather than
                // panicking on a record shape the guard above never read.
                let structure = run.recorder.structure()?;
                let prompt = crate::assembly::assemble(structure, identity, tool_schemas);
                // **An incomplete prompt does not cross the seam.** A
                // message-kind record that did not decode is a hole where a
                // turn's content was, so the count becomes the `fault` event
                // this crate authors and the seat is not granted: handing a
                // loop a prompt with a hole would put the loss in the model's
                // context and nowhere else.
                if prompt.undecodable > 0 {
                    let account = format!(
                        "{{\"organ\":\"harness\",\"undecodable-message-records\":{}}}",
                        prompt.undecodable
                    );
                    let turn = run.turn_in_flight.clone();
                    let _ = run.author.author_fault(
                        &mut run.recorder,
                        Subsystem::Harness,
                        turn.as_ref(),
                        &crate::authorship::harness_report(
                            weaver_types::FaultCase::MessageRecordUndecodable,
                            &account,
                        ),
                    );
                    return None;
                }
                let spu = run.spu.as_ref()?;
                Some(crate::engine::Ports::grant(
                    &spu.decode,
                    &run.author,
                    &mut run.recorder,
                    &mut run.turn_ordinal,
                    Some(prompt),
                    &self.coordination,
                    // A seat granted outside the serve loop streams without
                    // the ear, the verb slot being the loop's own, and
                    // executes no tools: the dev boundary's seat reasons and
                    // the serve loop's seat reaches the world.
                    None,
                    None,
                    run.state.as_mut(),
                    run.classify.as_ref().map(|arm| &arm.channel),
                    &mut run.fullness,
                    &mut run.pressure_reported,
                ))
            }
            // Before enter and after leave there is no standing interior to
            // hand across, which is the bracket discipline being loop 0's.
            _ => None,
        }
    }

    /// Closes the exchange the directive opened. Admin opens every
    /// coordination exchange, so the answer carries the identity that arrived
    /// rather than a fresh one this crate numbered.
    /// The answer goes to the connection the directive arrived on, not to a
    /// channel this crate holds: the listener is what it holds, and each verb
    /// brings its own connection.
    fn answer(
        &mut self,
        connection: &OrganChannel,
        exchange: &ExchangeId,
        answer: LifecycleAnswer,
    ) -> Result<(), ChannelFault> {
        connection.send(&OrganEnvelope {
            exchange: exchange.clone(),
            position: Position::Close,
            payload: weaver_types::Payload::Answer(answer),
        })
    }

    fn refuse(
        &mut self,
        connection: &OrganChannel,
        exchange: &ExchangeId,
        refusal: LifecycleRefusal,
    ) -> Result<(), ChannelFault> {
        connection.send(&OrganEnvelope {
            exchange: exchange.clone(),
            position: Position::Close,
            payload: weaver_types::Payload::Refusal(refusal),
        })
    }
}

/// Leave runs the reverse order and drains before it answers: lower the gate
/// first, author the unload event, drain the writer's queue, and release the
/// SPU. Left is answered only after the drain returns, which is what makes the
/// answer mean that everything admitted reached the stream.
///
/// The match on the options is the checked unwind: a forgotten arm is a
/// compile error rather than a leaked residency.
fn leave(run: &mut Run) -> Result<(), LifecycleRefusal> {
    // The unwind runs whole whatever refuses along it, because stopping at
    // the first refusal leaks everything after it: a refused lower must not
    // leave a device held. The first refusal in sequence order is what the
    // answer names, per the contract's a-refusal-names-where-it-stopped.
    let mut first_refusal: Option<LifecycleRefusal> = None;

    if let Some(gate) = run.gate.take() {
        run.gate_ordinal += 1;
        // The lower is an exchange, not a shot: its confirmation is what
        // admin's leave aggregate rests on, so the answer is read before the
        // channel drops rather than raced against it.
        let lowered = exchange(
            &gate.channel,
            Opener::Harness,
            run.gate_ordinal,
            weaver_types::RefusingOrgan::Gate,
            LifecycleDirective::Lower,
        );
        drop(gate.channel);
        reap(gate.pid);
        if let Err(refusal) = lowered {
            first_refusal.get_or_insert(refusal);
        }
    }
    if let Some(classify) = run.classify.take() {
        // Closure is the release, per the classify contract's failure
        // section: the process exits on its seam's close and the reap reads
        // it out, the arm falling second because it stood second to last.
        drop(classify.channel);
        reap(classify.pid);
    }
    // The serving arm's bracket close: the diagnostic record carries no
    // `unload`, its own close being `replay.closed`, the loop's to author,
    // per `weaver-diagnostic-Spec` section 4.
    if run.recorder.serving().is_some() {
        let _ = run.author.author(
            &mut run.recorder,
            Kind::Unload,
            Subsystem::Harness,
            None,
            None,
        );
    }

    // **The drain's outcome is carried, not discarded.** `Left` means
    // everything admitted reached the stream, so a failed drain must not be
    // answered as one: the peer could not otherwise tell a complete stream
    // from a truncated one.
    let drained = run.recorder.drain();

    if let Some(spu) = run.spu.take() {
        run.spu_ordinal += 1;
        // **The decode end drops first**, ending the worker's decode phase so
        // the release lands on a seam that is listening, per the serve loop's
        // one-loop rule: from admit until its decode end closes, the worker
        // reads nothing else. Issue 113's repair, and the ordering the seam
        // tests always had.
        drop(spu.decode);
        // The release is the exchange the contract says it is: confirmed
        // after the device is free and never before, and admin's leave
        // aggregate rests one arm on that confirmation, so the answer is
        // read before the channel drops.
        let released = exchange(
            &spu.lifecycle,
            Opener::Harness,
            run.spu_ordinal,
            weaver_types::RefusingOrgan::Spu,
            LifecycleDirective::Release,
        );
        drop(spu.lifecycle);
        reap(spu.pid);
        if let Err(refusal) = released {
            first_refusal.get_or_insert(match drained.is_err() {
                // The drain failed earlier in the sequence than the
                // release did, so it is the failure the answer names.
                true => LifecycleRefusal::DescriptorsUnusable,
                false => refusal,
            });
        }
    }

    if let Some(refusal) = first_refusal {
        return Err(refusal);
    }
    match drained {
        Ok(()) => Ok(()),
        // The stream did not close cleanly, which is a fault the operator
        // learns from the refusal rather than from a silence.
        Err(_) => Err(LifecycleRefusal::DescriptorsUnusable),
    }
}

/// Reads a dead organ's exit status and names what killed it, so a placement
/// fault is not reported to the peer as a residency problem.
///
/// **The child writes no status**, and that is deliberate rather than an
/// omission: a setup-status pipe would put a fourth call between fork and
/// exec, and `weaver-harness-Spec` section 2.2 enumerates exactly three "and
/// nothing else". The exit status carries the same fact without adding one,
/// and it is read here - after the exchange observed closure - where the child
/// is already dead and there is no race to lose.
/// Open the decode session once residency confirms, per
/// `weaver-harness-Spec` section 6.1. The identity messages are the open's
/// messages and the run's session its session, and a refusal or a fault below
/// the exchange maps onto the load-refusal set the enter aggregate carries.
fn open_session(
    decode: &DecodeChannel,
    session: SessionId,
    identity: Vec<weaver_traits::Message>,
    column_ask: bool,
) -> Result<(), LifecycleRefusal> {
    decode
        .send_directive(&TokenDirective::Open {
            session,
            messages: identity,
            column_ask,
        })
        .map_err(|_| LifecycleRefusal::NoResidency)?;
    match decode.recv_reply() {
        Ok(crate::channel::DecodeReply::Answer(TokenAnswer::Opened)) => Ok(()),
        // The emission, matched by name: a fault at open is the residency
        // unfit to serve, not the load's shape refused, and the wildcard
        // below would misname it. No recorder stands this early, so the
        // refusal carries the fact to the aggregate rather than the record.
        Ok(crate::channel::DecodeReply::Answer(TokenAnswer::Fault(_))) => {
            Err(LifecycleRefusal::NoResidency)
        }
        // A typed refusal on the open is the session declining to stand, which
        // the harness carries into the aggregate as the SPU unable to admit
        // what the load asked, the decode seam's refusals having no floor
        // lifecycle case of their own.
        Ok(_) => Err(LifecycleRefusal::DeviceCannotAdmit),
        // A fault below the exchange, the worker gone or the octets
        // undecodable, is the residency lost the moment it was confirmed.
        Err(_) => Err(LifecycleRefusal::NoResidency),
    }
}

fn classify_organ_death(pid: nix::unistd::Pid) -> Option<LifecycleRefusal> {
    // **The wait does not block.** An organ that closed its end and kept
    // running would otherwise hold the serving thread here forever, which is
    // a worse failure than the misreport this classification exists to
    // prevent. A child still running, or a status this call cannot read,
    // yields nothing and the caller keeps the refusal the exchange produced.
    match nix::sys::wait::waitpid(pid, Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
        Ok(nix::sys::wait::WaitStatus::Exited(_, crate::spawn::PLACEMENT_FAILED)) => {
            // The ends this organ was owed never reached descriptor 3, which
            // is a descriptor fault and not a residency one.
            Some(LifecycleRefusal::DescriptorsUnusable)
        }
        Ok(nix::sys::wait::WaitStatus::Exited(_, crate::spawn::EXEC_FAILED)) => {
            // The organ binary never ran, so nothing it was asked to stand up
            // could have stood up.
            Some(LifecycleRefusal::BindFailed)
        }
        _ => None,
    }
}

/// Reaps a forked organ so it does not stay a zombie entry for the life of the
/// worker. The wait is bounded by the organ's own exit: both organs are sent
/// their closing directive and have their channel dropped first, so the read
/// that follows returns closure and the process exits.
fn reap(pid: nix::unistd::Pid) {
    let _ = nix::sys::wait::waitpid(pid, None);
}

/// Opens one exchange on an organ channel and reads its answer, carrying a
/// refusing organ's reason into the aggregate unchanged.
fn exchange(
    channel: &OrganChannel,
    opener: Opener,
    ordinal: u64,
    organ: weaver_types::RefusingOrgan,
    directive: LifecycleDirective,
) -> Result<LifecycleAnswer, LifecycleRefusal> {
    channel
        .send(&OrganEnvelope {
            exchange: ExchangeId { opener, ordinal },
            position: Position::Open,
            payload: weaver_types::Payload::Directive(directive),
        })
        .map_err(|_| LifecycleRefusal::NoResidency)?;
    match channel.recv() {
        Ok(envelope) => match envelope.payload {
            weaver_types::Payload::Answer(answer) => Ok(answer),
            // The refusing organ is the one the caller spoke to: a gate
            // refusal reported as the SPU's would have the peer acting on the
            // wrong organ.
            weaver_types::Payload::Refusal(reason) => Err(LifecycleRefusal::OrganRefused {
                organ,
                reason: Box::new(reason),
            }),
            _ => Err(LifecycleRefusal::Malformed),
        },
        Err(_) => Err(LifecycleRefusal::NoResidency),
    }
}

fn clear_dumpable() -> Result<(), AdoptionFault> {
    // SAFETY: prctl with PR_SET_DUMPABLE affects only this process.
    let rc = unsafe { nix::libc::prctl(nix::libc::PR_SET_DUMPABLE, 0, 0, 0, 0) };
    if rc == -1 {
        return Err(AdoptionFault::DumpableNotCleared {
            errno: std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        });
    }
    Ok(())
}

/// Authors the seated identity prefix beside the load, one `message.system`
/// per declared message, and authors the door's refusal as a `fault` where a
/// message carries a role the door does not write.
///
/// **The miss is authored and not dropped**, per `weaver-harness-PRD`
/// section 5's fifth case and `weaver-harness-Spec` section 6. The prefix is
/// seated at the session's open whether this door wrote it or not, so a miss
/// that goes nowhere leaves the record reading as an agent that seated no
/// prefix when it seated one no reader can see - laundering by omission
/// rather than by a wrong record.
///
/// **Both arms of the door's answer are read, because both leave the same
/// hole.** The outer arm is the door's refusal, itself three conditions: a
/// role the door does not write, a system message carrying a block the
/// licensing rule does not admit, and a message that will not render. The
/// inner arm is the recorder declining the write. All four seat a prefix the
/// record cannot show, so all four are accounted for, and which one happened
/// travels in the account rather than in the case, per the custody rule of
/// `weaver-types-Spec` section 4.2.
///
/// **The load is not refused on any of them.** The seated prefix is the
/// operator's declaration, and a run that has already bracketed does not die
/// on a record it could not write, so what is owed is the account rather than
/// the abort. The fault is best-effort for the same reason it is owed: where
/// the recorder is the thing that failed, the account may fail with it, and a
/// miss nobody could write down is still not a miss worth aborting a run
/// over. Where the rule that judges a declaration lands is `weaver-types`' to
/// say.
///
/// conforms: harness-identity-refusal-authored-not-dropped
fn seat_identity_prefix(
    author: &crate::authorship::Author,
    recorder: &mut crate::record::Record,
    identity: &[weaver_traits::Message],
) {
    for message in identity {
        let account = match author.author_identity(recorder, message) {
            // The prefix reached the record, which is the whole of what this
            // loop is for.
            Ok(Ok(_)) => continue,
            // The door refused. `role` and `block` together name which of the
            // three conditions it was: the role where the block reads
            // `identity-door-system-only`, the licensing rule where it names
            // a block, and the rendering where it reads `unrenderable`.
            Err(unlicensed) => serde_json::json!({
                "organ": "harness",
                "miss": "identity-door-refused",
                "role": unlicensed.role,
                "block": unlicensed.block,
            }),
            // The door passed it and the recorder would not take it. Named
            // apart because the declaration is not at fault here, and a
            // reader sent after a bad declaration would be chasing the wrong
            // thing.
            Ok(Err(failure)) => serde_json::json!({
                "organ": "harness",
                "miss": "recorder-declined",
                "failure": format!("{failure:?}"),
            }),
        };
        let _ = author.author_fault(
            recorder,
            Subsystem::Harness,
            None,
            &crate::authorship::harness_report(
                weaver_types::FaultCase::IdentityPrefixUnrecorded,
                &account.to_string(),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    //! The announce-after-record test of section 8, run from inside the crate
    //! because an entered run needs organ doubles the integration suite does
    //! not buy, while the run struct itself is constructible here.

    use super::*;
    use std::fs::File;
    use weaver_trace::Kind;

    /// A bound listener for tests that exercise `dispatch_on` directly. The
    /// listener is never accepted on: these tests supply the connection, and
    /// the field exists because the type does.
    fn test_listener() -> CoordinationListener {
        static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("weaver-harness-unit-{}-{n}", std::process::id()));
        // Removed first: a directory left by an earlier run would hold a
        // stale socket, and the bind would refuse a name nothing is using.
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::create_dir_all(&dir);
        crate::channel::bind_coordination(&dir.join("c.sock")).expect("bind")
    }

    /// **The identity door's refusal reaches the record as a `fault`**, per
    /// `weaver-harness-PRD` section 5's fifth case. A declaration carrying a
    /// non-system identity role is seated at the session's open regardless,
    /// so a dropped refusal would leave the record reading as an agent that
    /// seated no prefix - the shape the 2026-08-25 cross-precision deposit
    /// carries, and the one a certification reads as a divergence it then
    /// blames on the model.
    ///
    /// The system-role message is authored and the user-role message is not,
    /// so the record carries exactly one `message.system` and one `fault`,
    /// which is what distinguishes authoring the refusal from authoring
    /// everything.
    ///
    /// Perturbation: restore `let _ = author.author_identity(..)` in
    /// `seat_identity_prefix` and the `fault` is absent, the record carrying
    /// the prefix that was written and no account of the one that was not.
    /// Watched under exactly that restoration.
    ///
    /// conforms: harness-identity-refusal-authored-not-dropped
    #[test]
    fn the_identity_refusal_is_authored_not_dropped() {
        let path = std::env::temp_dir().join(format!(
            "weaver-harness-identity-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let sink = OwnedFd::from(File::create(&path).expect("sink"));
        let session = SessionId("s-1".to_string());
        let mut recorder = crate::record::Record::Serving(
            Recorder::receive(sink, RunRef("r-1".into()), SessionRef(session.0.clone()))
                .expect("recorder"),
        );
        let author = Author::new(&session, &weaver_types::RunId("r-1".into()));

        let said = |role| weaver_traits::Message {
            role,
            content: vec![weaver_traits::ContentBlock::Text {
                text: "You are Karl, a small local agent.".into(),
            }],
        };
        // A system message carrying a block the licensing rule does not admit
        // is the door's second refusal condition, and it is not a role fault.
        let bad_block = weaver_traits::Message {
            role: weaver_traits::Role::System,
            content: vec![weaver_traits::ContentBlock::ToolCall(
                weaver_traits::ToolCall {
                    name: "calculator".into(),
                    arguments: "{}".into(),
                },
            )],
        };
        seat_identity_prefix(
            &author,
            &mut recorder,
            &[
                said(weaver_traits::Role::System),
                said(weaver_traits::Role::User),
                bad_block,
            ],
        );

        let kinds: Vec<Kind> = recorder.structure().expect("the serving record").iter().map(|r| r.kind).collect();
        assert_eq!(
            kinds,
            vec![Kind::MessageSystem, Kind::Fault, Kind::Fault],
            "the licensed prefix records and both misses are accounted for"
        );

        let faults: Vec<&str> = recorder
            .structure().expect("the serving record")
            .iter()
            .filter(|r| r.kind == Kind::Fault)
            // The line is read rather than a typed payload, the record
            // holding the canonical rendering the stream carries and nothing
            // beside it.
            .map(|r| &*r.line)
            .collect();
        for line in &faults {
            assert!(
                line.contains(r#""case":"identity_prefix_unrecorded""#),
                "every miss files under the one case, got {line}"
            );
        }
        assert!(
            faults[0].contains(r#""role":"user""#)
                && faults[0].contains(r#""block":"identity-door-system-only""#),
            "the role refusal names the role and the role block, got {}",
            faults[0]
        );
        // **The account separates the causes the case deliberately does
        // not.** A system message refused on its content is not a role
        // fault, and a reader sent after a bad role here would be chasing
        // the wrong thing.
        assert!(
            faults[1].contains(r#""role":"system""#)
                && faults[1].contains(r#""block":"tool_call""#),
            "the content refusal names the block rather than a bad role, got {}",
            faults[1]
        );
        let _ = std::fs::remove_file(&path);
    }

    fn test_exchange() -> ExchangeId {
        ExchangeId {
            opener: Opener::Admin,
            ordinal: 1,
        }
    }

    fn entered_run(turn: Option<&str>) -> (Run, OrganChannel, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!(
            "weaver-harness-stop-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let sink = OwnedFd::from(File::create(&path).expect("sink"));
        let session = SessionId("s-1".to_string());
        let mut recorder = crate::record::Record::Serving(
            Recorder::receive(sink, RunRef("r-1".into()), SessionRef(session.0.clone()))
                .expect("recorder"),
        );
        let author = Author::new(&session, &weaver_types::RunId("r-1".into()));
        author
            .author(
                &mut recorder,
                Kind::Load,
                Subsystem::Harness,
                None,
                Some(Payload::Elections(weaver_trace::Elections {
                    residual_readout: false,
                    field: None,
                    surprisal: false,
                    tee: Some(weaver_trace::Election::default()),
                })),
            )
            .expect("load");
        if let Some(turn) = turn {
            let key = TurnKey(turn.to_string());
            author
                .author(
                    &mut recorder,
                    Kind::TurnStarted,
                    Subsystem::Harness,
                    Some(&key),
                    None,
                )
                .expect("turn started");
        }
        let (near, _far) = OrganChannel::pair().expect("pair");
        // A loaded-and-idle run holds a resident SPU, so the helper builds one
        // over a socketpair: the decode end is what the granted seat drives,
        // and a run with no SPU is not loaded.
        let (lifecycle, _spu_lifecycle) = OrganChannel::pair().expect("pair");
        let (decode, _spu_decode) = DecodeChannel::pair().expect("decode pair");
        (
            Run {
                classify: None,
                recorder,
                author,
                session,
                run: weaver_types::RunId("r-1".into()),
                spu: Some(SpuChannels {
                    lifecycle,
                    decode,
                    pid: nix::unistd::Pid::from_raw(1),
                }),
                gate: None,
                diagnostic: None,
                state: None,
                fullness: None,
                pressure_reported: false,
                turn_in_flight: turn.map(|t| TurnKey(t.to_string())),
                spu_ordinal: 0,
                gate_ordinal: 0,
                held_frames: std::collections::VecDeque::new(),
                turn_ordinal: 0,
            },
            near,
            path,
        )
    }

    /// The declared election reaches the tee, through the real enter path:
    /// a fake member listens at the seam's name, the enter runs with a
    /// non-default election and organ binaries that cannot exec, and by
    /// the time the fan-out fails after-load the member has already
    /// received the opener carrying exactly the declared kinds and paths,
    /// followed by the load event's distillate. The pairs-for-elected-keys
    /// behavior itself is the tee suite's to pin - what this buys is the
    /// plumbing from `EnterPayload` to `Tee::open` that nothing else
    /// watches. Needs no device, no fixture, and no built organs.
    #[test]
    fn the_declared_election_reaches_the_tee() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-election-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        let listener =
            crate::channel::bind_coordination(&dir.join("coordination.sock")).expect("bind");

        // The fake member: the far end of the pair the enter would carry,
        // per the ruling of 2026-08-26 - no name, no dial, the end handed
        // in. Hand back every line that arrives until the peer closes.
        let (state_end, member_end) =
            std::os::unix::net::UnixStream::pair().expect("the member's pair");
        let (send, receive) = std::sync::mpsc::channel::<String>();
        let reader = std::thread::spawn(move || {
            use std::io::Read;
            let mut channel = member_end;
            let mut held = String::new();
            let _ = channel.read_to_string(&mut held);
            for line in held.lines() {
                let _ = send.send(line.to_string());
            }
        });

        let sink_path = dir.join("trace.ndjson");
        let sink = OwnedFd::from(File::create(&sink_path).expect("sink"));
        let mut harness = Harness {
            coordination: listener,
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/weaver-spu".into(),
                gate: "/nonexistent/weaver-gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::BeforeEnter,
        };
        let payload = weaver_types::EnterPayload {
            session: SessionId("s-election".into()),
            run: weaver_types::RunId("r-1".into()),
            spu_instruction: weaver_types::SpuInstruction {
                classify: None,
                decoder: weaver_types::DecoderInstruction {
                    model_binding: weaver_types::ModelBinding {
                        artifact: weaver_types::ArtifactRef("unreachable".into()),
                        devices: vec![weaver_types::DeviceOrdinal(0)],
                    },
                    residual_readout_election: false,
                    field_election: None,
                    surprisal_election: false,
                    refeed_permission: false,
                    column_permission: false,
                    identity: Vec::new(),
                    tunable_values: Default::default(),
                },
            },
            binding: weaver_types::EnterBinding::Serving {
                gate_instruction: weaver_types::GateInstruction {
                    access_rule: weaver_types::AccessRule {
                        allowed_uids: Default::default(),
                        allowed_gids: Default::default(),
                        denied_uids: Default::default(),
                    },
                },
            },
            state_election: weaver_types::StateElection {
                all_kinds: false,
                keys: vec![weaver_types::ElectedKindConfig {
                    kind: "load".into(),
                    paths: vec!["origin".into()],
                }],
            },
        };

        // The fan-out fails after-load at the SPU exec, which is the point:
        // the tee attach and the load event precede the forks. The bracket
        // stands on that failure, so the run is retained and left the way
        // the serving loop leaves it - dropping it unleft would orphan the
        // forked child unreaped and leave the bracket unclosed.
        let mut run = match harness.enter(
            payload,
            Some(sink),
            Some(std::os::fd::OwnedFd::from(state_end)),
        ) {
            Err(EnterFailure::AfterLoad(run, _)) => run,
            Ok(_) => panic!("the bogus fan-out cannot succeed"),
            Err(EnterFailure::BeforeLoad(refusal)) => {
                panic!("failed before the load: {refusal:?}")
            }
        };
        let _ = leave(&mut run);
        drop(run);
        // The leave closed the bracket and the drop closed the tee's
        // channel, so the reader drains to end-of-stream and finishes.
        reader.join().expect("the member read to closure");

        let opener = receive.recv().expect("the opener arrived first");
        let parsed: serde_json::Value = serde_json::from_str(&opener).expect("opener parses");
        let election = &parsed["election"];
        assert_eq!(election["all_kinds"], serde_json::Value::Bool(false));
        assert_eq!(election["keys"][0]["kind"], "load");
        assert_eq!(election["keys"][0]["paths"][0], "origin");

        let distilled = receive.recv().expect("the load event distilled");
        let frame: serde_json::Value = serde_json::from_str(&distilled).expect("frame parses");
        assert_eq!(frame["envelope"]["kind"], "load");
        assert_eq!(frame["envelope"]["session"], "s-election");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **A refused enter leaves its reason in the run's record.** Before
    /// 2026-08-22 a lifecycle refusal reached admin's answer and nothing
    /// else, so a trace showed a run that loaded and then stopped, with the
    /// reason living only in an answer no reader of the record holds.
    ///
    /// The run here is the one a real after-load refusal produces, taken
    /// from the enter path rather than assembled, so the record it carries
    /// is the record that would have been written.
    ///
    /// **What this pins and what it does not.** It pins the shape: that a
    /// lifecycle refusal reaching a standing run becomes a `refusal` naming
    /// the seam and the ask, carrying the seam's own case, belonging to no
    /// turn, and closing nothing. **It does not pin the call site.** The
    /// clerking happens in the serve loop's after-load arm, which needs a
    /// coordination connection to reach, and this test calls the authoring
    /// directly, so removing the call from that arm leaves this test
    /// passing. Watched under exactly that removal, which is how the gap is
    /// known rather than assumed.
    ///
    /// The call site is one line in one arm and the arm is read in review.
    /// A test that drove it would drive the whole serve loop, which is the
    /// shape `the_turn_rehearses_on_the_device` already carries for the
    /// turn, and it is the honest place to add this if the arm ever grows a
    /// second path.
    #[test]
    fn a_refused_enter_leaves_its_reason_in_the_record() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-refused-enter-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        let listener =
            crate::channel::bind_coordination(&dir.join("coordination.sock")).expect("bind");
        let sink = OwnedFd::from(File::create(dir.join("trace.ndjson")).expect("sink"));
        let mut harness = Harness {
            coordination: listener,
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/weaver-spu".into(),
                gate: "/nonexistent/weaver-gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::BeforeEnter,
        };
        let payload = weaver_types::EnterPayload {
            session: SessionId("s-refused".into()),
            run: weaver_types::RunId("r-1".into()),
            spu_instruction: weaver_types::SpuInstruction {
                classify: None,
                decoder: weaver_types::DecoderInstruction {
                    model_binding: weaver_types::ModelBinding {
                        artifact: weaver_types::ArtifactRef("unreachable".into()),
                        devices: vec![weaver_types::DeviceOrdinal(0)],
                    },
                    residual_readout_election: false,
                    field_election: None,
                    surprisal_election: false,
                    refeed_permission: false,
                    column_permission: false,
                    identity: Vec::new(),
                    tunable_values: Default::default(),
                },
            },
            binding: weaver_types::EnterBinding::Serving {
                gate_instruction: weaver_types::GateInstruction {
                    access_rule: weaver_types::AccessRule {
                        allowed_uids: Default::default(),
                        allowed_gids: Default::default(),
                        denied_uids: Default::default(),
                    },
                },
            },
            state_election: weaver_types::StateElection {
                all_kinds: false,
                keys: Vec::new(),
            },
        };
        // The fan-out fails after-load at the SPU exec, which is the arm
        // this test exists for: the bracket stands and the refusal is the
        // enter's own.
        let (mut run, refusal) = match harness.enter(payload, Some(sink), None) {
            Err(EnterFailure::AfterLoad(run, refusal)) => (*run, refusal),
            Ok(_) => panic!("the bogus fan-out cannot succeed"),
            Err(EnterFailure::BeforeLoad(refusal)) => {
                panic!("failed before the load: {refusal:?}")
            }
        };

        author_lifecycle_refusal(&mut run, &refusal);

        let refusals: Vec<&weaver_trace::Record> = run
            .recorder
            .structure().expect("the serving record")
            .iter()
            .filter(|r| r.kind == Kind::Refusal)
            .collect();
        assert_eq!(refusals.len(), 1, "the refusal reached the run's record");
        let event: serde_json::Value =
            serde_json::from_str(refusals[0].line.as_ref()).expect("the line parses");
        assert_eq!(event["payload"]["seam"], "lifecycle");
        assert_eq!(
            event["payload"]["asked"]["ask"], "enter",
            "the ask is named"
        );
        // **Compared against the refusal itself, not against its shape.** A
        // type check passes for any case the seam could have sent, so it
        // would have held while the record carried the wrong refusal, which
        // is the failure this whole class was written to end.
        assert_eq!(
            event["payload"]["refusal"],
            serde_json::to_value(&refusal).expect("the refusal renders"),
            "the seam's own case travels whole: {event}"
        );
        assert!(
            event.get("turn").is_none(),
            "an enter belongs to no turn"
        );

        // **The bracket is still open**, which is the property the shape
        // rests on: a refused enter authors an event inside a standing run
        // and the close belongs to the leave.
        assert_eq!(
            run.recorder
                .structure().expect("the serving record")
                .iter()
                .filter(|r| r.kind == Kind::TurnClosed)
                .count(),
            0,
            "a refused enter closes no turn"
        );

        let _ = leave(&mut run);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **The seated prefix reaches the record through the real enter path.**
    /// The door itself is watched in the authorship suite. What this buys is
    /// the call site: that `enter` reads the identity out of the SPU
    /// instruction it was handed and authors it, in order, after the load.
    /// Without it the door could be correct and never called, which is the
    /// shape the defect took before this act, the prefix having been seated
    /// at open and written down nowhere.
    ///
    /// The fan-out fails after-load at the SPU exec as its sibling above
    /// does, which is what makes the test cheap: the load event and the
    /// prefix precede the forks, so no organ has to run for the record to
    /// be complete at the point this reads it.
    #[test]
    fn the_entered_identity_reaches_the_record() {
        let dir = std::env::temp_dir().join(format!(
            "weaver-identity-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        let listener =
            crate::channel::bind_coordination(&dir.join("coordination.sock")).expect("bind");
        let sink_path = dir.join("trace.ndjson");
        let sink = OwnedFd::from(File::create(&sink_path).expect("sink"));
        let mut harness = Harness {
            coordination: listener,
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/weaver-spu".into(),
                gate: "/nonexistent/weaver-gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::BeforeEnter,
        };
        let payload = weaver_types::EnterPayload {
            session: SessionId("s-identity".into()),
            run: weaver_types::RunId("r-1".into()),
            spu_instruction: weaver_types::SpuInstruction {
                classify: None,
                decoder: weaver_types::DecoderInstruction {
                    model_binding: weaver_types::ModelBinding {
                        artifact: weaver_types::ArtifactRef("unreachable".into()),
                        devices: vec![weaver_types::DeviceOrdinal(0)],
                    },
                    residual_readout_election: false,
                    field_election: None,
                    surprisal_election: false,
                    refeed_permission: false,
                    column_permission: false,
                    identity: vec![weaver_traits::Message {
                        role: weaver_traits::Role::System,
                        content: vec![weaver_traits::ContentBlock::Text {
                            text: "You are a careful assistant.".into(),
                        }],
                    }],
                    tunable_values: Default::default(),
                },
            },
            binding: weaver_types::EnterBinding::Serving {
                gate_instruction: weaver_types::GateInstruction {
                    access_rule: weaver_types::AccessRule {
                        allowed_uids: Default::default(),
                        allowed_gids: Default::default(),
                        denied_uids: Default::default(),
                    },
                },
            },
            state_election: weaver_types::StateElection {
                all_kinds: false,
                keys: Vec::new(),
            },
        };
        let mut run = match harness.enter(payload, Some(sink), None) {
            Err(EnterFailure::AfterLoad(run, _)) => run,
            Ok(_) => panic!("the bogus fan-out cannot succeed"),
            Err(EnterFailure::BeforeLoad(refusal)) => {
                panic!("failed before the load: {refusal:?}")
            }
        };
        let _ = leave(&mut run);
        drop(run);

        let held = std::fs::read_to_string(&sink_path).expect("the sink reads back");
        let events: Vec<serde_json::Value> = held
            .lines()
            .map(|line| serde_json::from_str(line).expect("each line parses"))
            .collect();
        assert_eq!(events[0]["kind"], "load", "the load opens the run");
        assert_eq!(
            events[1]["kind"], "message.system",
            "and the seated prefix follows it"
        );
        assert!(
            events[1].get("turn").is_none(),
            "carrying no turn, a prefix preceding every turn there is"
        );
        let payload_text = events[1]["payload"]["content"][0]["text"]
            .as_str()
            .expect("the prefix carries its text");
        assert_eq!(
            payload_text, "You are a careful assistant.",
            "and the text is the one the instruction declared"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **The declared elections reach the load record, all three of them.**
    /// The authorship suite watches the door and this watches the mapping:
    /// that `enter` reads each election out of the SPU instruction it was
    /// handed and writes it into the record's `load`, rather than writing a
    /// default that happens to match the fixture.
    ///
    /// **Both states are run because a boolean that is always false passes
    /// a test that never sets it true.** The surprisal's election is the
    /// one this matters most for: its `false` is a written member rather
    /// than an omission, so a mapping that dropped the value entirely and a
    /// mapping that carried it would look alike on a declined load.
    #[test]
    fn the_declared_elections_reach_the_load_record() {
        for elected in [false, true] {
            let dir = std::env::temp_dir().join(format!(
                "weaver-elections-{}-{elected}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("scratch");
            let listener =
                crate::channel::bind_coordination(&dir.join("coordination.sock")).expect("bind");
            let sink_path = dir.join("trace.ndjson");
            let sink = OwnedFd::from(File::create(&sink_path).expect("sink"));
            let mut harness = Harness {
                coordination: listener,
                organs: OrganBinaries {
                    classify: None,
                    spu: "/nonexistent/weaver-spu".into(),
                    gate: "/nonexistent/weaver-gate".into(),
                },
                parameters: OrganParameters::default(),
                state: ChannelState::BeforeEnter,
            };
            let payload = weaver_types::EnterPayload {
                session: SessionId("s-elections".into()),
                run: weaver_types::RunId("r-1".into()),
                spu_instruction: weaver_types::SpuInstruction {
                    classify: None,
                    decoder: weaver_types::DecoderInstruction {
                        model_binding: weaver_types::ModelBinding {
                            artifact: weaver_types::ArtifactRef("unreachable".into()),
                            devices: vec![weaver_types::DeviceOrdinal(0)],
                        },
                        residual_readout_election: elected,
                        field_election: elected
                            .then(|| weaver_types::FieldElection { depth: 50 }),
                        surprisal_election: elected,
                        refeed_permission: false,
                        column_permission: false,
                        identity: Vec::new(),
                        tunable_values: Default::default(),
                    },
                },
                binding: weaver_types::EnterBinding::Serving {
                    gate_instruction: weaver_types::GateInstruction {
                        access_rule: weaver_types::AccessRule {
                            allowed_uids: Default::default(),
                            allowed_gids: Default::default(),
                            denied_uids: Default::default(),
                        },
                    },
                },
                state_election: weaver_types::StateElection {
                    all_kinds: false,
                    keys: vec![weaver_types::ElectedKindConfig {
                        kind: "turn.closed".to_string(),
                        paths: vec!["close".to_string()],
                    }],
                },
            };
            let mut run = match harness.enter(payload, Some(sink), None) {
                Err(EnterFailure::AfterLoad(run, _)) => run,
                Ok(_) => panic!("the bogus fan-out cannot succeed"),
                Err(EnterFailure::BeforeLoad(refusal)) => {
                    panic!("failed before the load: {refusal:?}")
                }
            };
            let _ = leave(&mut run);
            drop(run);

            let held = std::fs::read_to_string(&sink_path).expect("the sink reads back");
            let load: serde_json::Value = serde_json::from_str(
                held.lines().next().expect("the load opens the run"),
            )
            .expect("the load parses");
            assert_eq!(load["kind"], "load");
            assert_eq!(
                load["payload"]["residual_readout"],
                serde_json::json!(elected),
                "the readout's election is the one that was declared"
            );
            assert_eq!(
                load["payload"]["surprisal"],
                serde_json::json!(elected),
                "and so is the surprisal's, written down either way"
            );
            assert_eq!(
                load["payload"]["tee"],
                serde_json::json!({
                    "all_kinds": false,
                    "keys": [{"kind": "turn.closed", "paths": ["close"]}]
                }),
                "the tee's rule is the enter's declared election, keys and \
                 all, not a default"
            );
            if elected {
                assert_eq!(
                    load["payload"]["field"],
                    serde_json::json!(50),
                    "the field carries the declared depth"
                );
            } else {
                assert!(
                    load["payload"].get("field").is_none(),
                    "and an unelected field is an absence rather than a null"
                );
            }
            let _ = std::fs::remove_dir_all(&dir);
        }
    }

    /// **A diagnostic enter forks no gate.** The real SPU admits the real
    /// fixture and the fan-out ends at that confirmation: ready, with the
    /// gate arm never stood. The gate binary handed in is deliberately
    /// nonexistent, which is the discriminator - perturb `enter` to run the
    /// gate step whatever the kind and this fork fails `BindFailed`, so a
    /// passing run is one the gate step never touched rather than one it
    /// survived. Watched to fail under exactly that perturbation, run on the
    /// device 2026-08-24. Same gating as the rehearsal below: the
    /// device-featured SPU built and the fixture present, under
    /// `WEAVER_REHEARSAL=1`.
    #[test]
    fn the_diagnostic_enter_forks_no_gate() {
        let fixture = std::path::Path::new("/opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf");
        let target = std::env::current_exe()
            .expect("the test binary knows itself")
            .parent()
            .and_then(std::path::Path::parent)
            .expect("the target profile dir")
            .to_path_buf();
        let spu = std::env::var("WEAVER_SPU_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| target.join("weaver-spu"));
        if std::env::var("WEAVER_REHEARSAL").is_err() || !fixture.exists() || !spu.exists() {
            eprintln!(
                "SKIP the_diagnostic_enter_forks_no_gate: set WEAVER_REHEARSAL=1 with the \
                 device-featured SPU built, needs {} and {}",
                fixture.display(),
                spu.display()
            );
            return;
        }

        let scratch =
            std::env::temp_dir().join(format!("weaver-diagnostic-{}", std::process::id()));
        std::fs::create_dir_all(&scratch).expect("scratch");
        let sink_path = scratch.join("diagnostic.ndjson");
        let sink = OwnedFd::from(File::create(&sink_path).expect("sink"));

        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: spu.clone(),
                // The discriminator: a gate step that runs cannot fork this.
                gate: "/nonexistent/weaver-gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::BeforeEnter,
        };
        let payload = weaver_types::EnterPayload {
            session: SessionId("diagnostic".into()),
            run: weaver_types::RunId("r-1".into()),
            spu_instruction: weaver_types::SpuInstruction {
                classify: None,
                decoder: weaver_types::DecoderInstruction {
                    model_binding: weaver_types::ModelBinding {
                        artifact: weaver_types::ArtifactRef(fixture.to_string_lossy().into_owned()),
                        devices: vec![weaver_types::DeviceOrdinal(0)],
                    },
                    residual_readout_election: false,
                    field_election: None,
                    surprisal_election: false,
                    refeed_permission: false,
                    column_permission: false,
                    identity: Vec::new(),
                    tunable_values: [
                        ("max-tokens-per-turn".to_string(), 4096.0),
                        ("context-capacity".to_string(), 4096.0),
                        ("seed".to_string(), 11.0),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            binding: weaver_types::EnterBinding::Diagnostic,
            state_election: weaver_types::StateElection::default(),
        };

        let mut run = match harness.enter(payload, Some(sink), None) {
            Ok(run) => run,
            Err(EnterFailure::BeforeLoad(refusal)) => panic!("enter refused early: {refusal:?}"),
            Err(EnterFailure::AfterLoad(_, refusal)) => panic!("enter refused: {refusal:?}"),
        };

        // Ready with the arm never stood: the fan-out ended at the SPU's
        // confirmation, per `weaver-harness-Spec` section 3.
        assert!(run.gate.is_none(), "a diagnostic run stands no gate arm");

        // The leave lowers what stands, and the gate arm is not among it.
        leave(&mut run).expect("the leave unwinds the diagnostic run");
        drop(run);
        let _ = std::fs::remove_dir_all(&scratch);
    }

    /// **The turn rehearses on the device**, the live run's shape inside
    /// the suite: the real enter fan-out forks the real organ binaries,
    /// the SPU admits real weights on the device, the session opens with
    /// the instruction's identity, a frame grants the seat, the decode
    /// runs, the response frame returns, the leave unwinds, and the
    /// artifact is read back whole from a real sink. The real client dial
    /// stays the live run's, where uids differ: the gate denies its own
    /// uid by construction, so after the real raise proves the gate arm,
    /// the turn is driven on a scripted gate end.
    ///
    /// Skips loudly without the fixture and both built binaries. Build the
    /// SPU with the device features first:
    /// `cargo build -p weaver-spu --features cuda,gguf`.
    #[test]
    fn the_turn_rehearses_on_the_device() {
        let fixture = std::path::Path::new("/opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf");
        let target = std::env::current_exe()
            .expect("the test binary knows itself")
            .parent()
            .and_then(std::path::Path::parent)
            .expect("the target profile dir")
            .to_path_buf();
        let spu = std::env::var("WEAVER_SPU_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| target.join("weaver-spu"));
        let gate = std::env::var("WEAVER_GATE_BIN")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| target.join("weaver-gate"));
        if std::env::var("WEAVER_REHEARSAL").is_err()
            || !fixture.exists()
            || !spu.exists()
            || !gate.exists()
        {
            eprintln!(
                "SKIP the_turn_rehearses_on_the_device: set WEAVER_REHEARSAL=1 with the \
                 device-featured SPU built, needs {} and {} and {}",
                fixture.display(),
                spu.display(),
                gate.display()
            );
            return;
        }

        let scratch = std::env::temp_dir().join(format!("weaver-rehearsal-{}", std::process::id()));
        std::fs::create_dir_all(&scratch).expect("scratch");
        let sink_path = scratch.join("rehearsal.ndjson");
        let sink = OwnedFd::from(File::create(&sink_path).expect("sink"));
        let gate_socket = scratch.join("gate.sock");
        std::fs::remove_file(&gate_socket).ok();

        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: spu.clone(),
                gate: gate.clone(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::BeforeEnter,
        };
        let payload = weaver_types::EnterPayload {
            session: SessionId("rehearsal".into()),
            run: weaver_types::RunId("r-1".into()),
            spu_instruction: weaver_types::SpuInstruction {
                classify: None,
                decoder: weaver_types::DecoderInstruction {
                    model_binding: weaver_types::ModelBinding {
                        artifact: weaver_types::ArtifactRef(fixture.to_string_lossy().into_owned()),
                        devices: vec![weaver_types::DeviceOrdinal(0)],
                    },
                    residual_readout_election: false,
                    field_election: None,
                    surprisal_election: false,
                    refeed_permission: false,
                    column_permission: false,
                    identity: Vec::new(),
                    // The deployed root's three operator-tunables, required
                    // at admit since the seed act of 2026-08-19. The empty
                    // map this carried predates that election and refused
                    // every rehearsal since, invisibly behind the env gate.
                    tunable_values: [
                        ("max-tokens-per-turn".to_string(), 4096.0),
                        ("context-capacity".to_string(), 4096.0),
                        ("seed".to_string(), 11.0),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            binding: weaver_types::EnterBinding::Serving {
                gate_instruction: weaver_types::GateInstruction {
                    access_rule: weaver_types::AccessRule {
                        allowed_uids: std::collections::BTreeSet::new(),
                        allowed_gids: std::collections::BTreeSet::new(),
                        denied_uids: std::collections::BTreeSet::new(),
                    },
                },
            },
            state_election: weaver_types::StateElection::default(),
        };

        // The real fan-out: real forks, real admit against real weights,
        // the real session open, the real raise.
        let mut run = match harness.enter(payload, Some(sink), None) {
            Ok(run) => run,
            Err(EnterFailure::BeforeLoad(refusal)) => panic!("enter refused early: {refusal:?}"),
            Err(EnterFailure::AfterLoad(_, refusal)) => panic!("enter refused: {refusal:?}"),
        };

        // The raise proved the gate arm on the real binary. The turn is
        // driven on a scripted end, the real dial being act four's.
        if let Some(gate_arm) = run.gate.take() {
            drop(gate_arm.channel);
            reap(gate_arm.pid);
        }
        let (gate_end, gate_peer) = OrganChannel::pair().expect("gate pair");
        let gate_peer = gate_peer.into_channel();
        run.gate = Some(GateChannel {
            channel: gate_end,
            pid: nix::unistd::Pid::from_raw(1),
        });
        harness.state = ChannelState::Entered(Box::new(run));

        gate_peer
            .send(&OrganEnvelope {
                exchange: ExchangeId {
                    opener: Opener::Gate,
                    ordinal: 1,
                },
                position: Position::Open,
                payload: weaver_types::Payload::Frame(weaver_types::TurnFrame::carry(
                    b"{\"text\":\"Reply with exactly one word: hello\"}",
                )),
            })
            .expect("the frame sends");

        let mut verb_slot = None;
        harness
            .serve_gate_wake(
                "",
                &[],
                &mut |ports: &mut crate::engine::Ports<'_>, text: &str| {
                    let delta = vec![weaver_traits::Message {
                        role: weaver_traits::Role::User,
                        content: vec![weaver_traits::ContentBlock::Text {
                            text: text.to_string(),
                        }],
                    }];
                    ports.turn(delta)
                },
                &mut verb_slot,
            )
            .expect("the turn serves on the device");

        let answer = gate_peer.recv().expect("the response frame returns");
        let weaver_types::Payload::Frame(frame) = answer.payload else {
            panic!("a frame answers a frame");
        };
        let line = String::from_utf8(frame.octets().expect("canonical")).expect("utf8");
        assert!(
            line.starts_with(r#"{"kind":"answered""#),
            "a real decode answers: {line}"
        );

        // The leave unwinds the real SPU and drains the sink.
        let ChannelState::Entered(run) = std::mem::replace(&mut harness.state, ChannelState::Left)
        else {
            panic!("entered");
        };
        let mut run = *run;
        // The scripted gate arm answers no lower: the real lower is the real
        // binary's, proven with the raise, and act four's live run walks it
        // whole. The arm drops here so the leave's unwind meets no exchange
        // nothing will answer.
        if let Some(scripted) = run.gate.take() {
            drop(scripted.channel);
        }
        leave(&mut run).expect("the leave unwinds");

        // The artifact, read back whole: the first record any human
        // inspects rides exactly this shape in act four.
        let artifact = std::fs::read_to_string(&sink_path).expect("the artifact reads back");
        let kinds: Vec<String> = artifact
            .lines()
            .filter_map(|line| {
                serde_json::from_str::<serde_json::Value>(line)
                    .ok()
                    .and_then(|v| v.get("kind").and_then(|k| k.as_str()).map(String::from))
            })
            .collect();
        assert_eq!(
            kinds,
            vec![
                "load",
                "turn.started",
                "message.user",
                "model.request",
                "model.output",
                "model.measurement",
                "message.assistant",
                "turn.closed",
                "unload",
            ],
            "the whole bracket in the artifact, in order"
        );
        assert!(
            artifact.contains(r#""template""#),
            "the request splice carries the SPU's rendering"
        );
        assert!(
            artifact.contains("weights_hash"),
            "the measurement names the model and its weights"
        );
        eprintln!(
            "REHEARSAL: {} events at {}",
            kinds.len(),
            sink_path.display()
        );
    }

    /// A stop places the turn's close event and answers `TurnAborted`.
    ///
    /// **This test proves both effects happened, not that the record preceded
    /// the answer**, and the distinction is worth stating rather than papering
    /// over: `dispatch` returns after doing both, so every assertion here
    /// observes the finished state and reversing the two operations inside the
    /// stop arm leaves this test passing. An ordering watch needs an observer
    /// that reads the answer while `dispatch` is still running, which needs
    /// shared access to the recorder this crate deliberately keeps behind one
    /// owner. So the ordering itself is held by statement order and read at
    /// review, and `weaver-harness-Spec` section 8 tags
    /// `harness-announce-after-record` `perturbation` - a tension named here
    /// for the operator rather than hidden behind a green test.
    #[test]
    fn stop_records_the_close_and_answers_turn_aborted() {
        let (run, _spare, _sink_path) = entered_run(Some("t-1"));
        let (harness_end, peer_end) = OrganChannel::pair().expect("pair");
        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(run)),
        };
        harness
            .dispatch_on(
                &harness_end,
                test_exchange(),
                LifecycleDirective::Stop,
                None,
                None,
            )
            .expect("stop dispatches");

        // The close is in the record.
        let closes = match &harness.state {
            ChannelState::Entered(run) => {
                run.recorder.structure().expect("the serving record").by_kind(Kind::TurnClosed).count()
            }
            _ => panic!("the position stays entered after a stop"),
        };
        assert_eq!(closes, 1, "the turn's close event was placed");

        // ...and only then does the answer carry TurnAborted.
        let peer = peer_end.into_channel();
        match peer.recv().expect("answer").payload {
            weaver_types::Payload::Answer(LifecycleAnswer::TurnAborted { turn }) => {
                assert_eq!(turn.0, "t-1")
            }
            other => panic!("a stop on a turn answers TurnAborted, got {other:?}"),
        }
    }

    /// **A closure observed from the entered position unwinds before it
    /// returns.** The recorder is drained and the bracket closed with an
    /// `unload`, rather than the run being dropped with events still queued
    /// and the organs left as orphans of a living parent.
    ///
    /// Perturbation: empty `unwind_if_entered` and the position stays
    /// entered. Watched under exactly that removal.
    #[test]
    fn closure_from_entered_unwinds_the_run() {
        let (run, _spare, sink_path) = entered_run(None);
        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(run)),
        };
        harness.unwind_if_entered(
            weaver_types::FaultCase::OrganDeathObserved,
            &ChannelFault::Undecodable,
        );
        match &harness.state {
            ChannelState::Left => {}
            _ => panic!("the position is terminal after an unwind"),
        }
        // The transition alone would pass with an unwind that did nothing, so
        // the effects are read off the sink: the bracket is closed and the
        // drain that closes it ran.
        let stream = std::fs::read_to_string(&sink_path).expect("the sink");
        assert!(
            stream.contains("\"kind\":\"unload\""),
            "the unwind closes the bracket with an unload: {stream}"
        );
        assert!(
            stream.contains("\"kind\":\"load\""),
            "and the run it closes is the one that opened: {stream}"
        );
        // Issue #221's gap: the death recorded its why before the bracket
        // closed, inside the record whose reason for existing is that it
        // never lies.
        assert!(
            stream.contains("\"kind\":\"fault\""),
            "the ending authored a fault event: {stream}"
        );
        assert!(
            stream.contains("organ_death_observed") && stream.contains("Undecodable"),
            "the fault names its case and carries the fault's spelling: {stream}"
        );
    }

    /// **The seat is granted at loaded-and-idle and not mid-turn.** Loop 0
    /// has not taken the interior back while a turn is in flight, so there is
    /// no standing interior to hand across.
    ///
    /// Perturbation: drop the turn-in-flight guard from `grant_seat` and a
    /// seat is handed over mid-turn. Watched under exactly that removal.
    #[test]
    fn no_seat_is_granted_while_a_turn_is_in_flight() {
        let (run, _spare, _path) = entered_run(Some("t-1"));
        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(run)),
        };
        assert!(
            harness.grant_seat("identity", &[]).is_none(),
            "an active run hands nothing across the seam"
        );

        // ...and an idle one does, so the guard is the turn and not the
        // position.
        let (idle, _spare, _path) = entered_run(None);
        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(idle)),
        };
        assert!(
            harness.grant_seat("identity", &[]).is_some(),
            "a loaded-and-idle run grants the seat"
        );
    }

    /// **A diagnostic run grants no frame seat, and answers rather than
    /// panics.** The extension seam is the frame path's: a diagnostic
    /// record holds no working structure and no frame ever arrives, so the
    /// grant answers the same absence the mid-turn arm answers, per
    /// `weaver-harness-Spec` section 6.2's partition of the two criteria.
    ///
    /// Perturbation: restore the `expect` on the structure read in
    /// `grant_seat` and this panics instead of answering. Watched under
    /// exactly that restoration. Found by the review seat on PR 393.
    #[test]
    fn a_diagnostic_run_grants_no_frame_seat() {
        let (mut run, _spare, _path) = entered_run(None);
        let sink_path = std::env::temp_dir().join(format!(
            "weaver-harness-diag-seat-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let sink = OwnedFd::from(File::create(&sink_path).expect("sink"));
        run.recorder = crate::record::Record::Diagnostic(
            weaver_diagnostic::Recorder::receive(
                sink,
                weaver_diagnostic::RunRef("r-1".into()),
                weaver_diagnostic::SessionRef("s-1".into()),
            )
            .expect("the recorder receives"),
        );
        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(run)),
        };
        assert!(
            harness.grant_seat("identity", &[]).is_none(),
            "a diagnostic run hands nothing across the frame seam"
        );
        let _ = std::fs::remove_file(&sink_path);
    }

    /// A stop at rest answers `AtRest`, a clean close and not a refusal, and
    /// places no close event because there was no turn to close.
    #[test]
    fn stop_at_rest_answers_at_rest() {
        let (run, _spare, _sink_path) = entered_run(None);
        let (harness_end, peer_end) = OrganChannel::pair().expect("pair");
        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(run)),
        };
        harness
            .dispatch_on(
                &harness_end,
                test_exchange(),
                LifecycleDirective::Stop,
                None,
                None,
            )
            .expect("stop");
        let peer = peer_end.into_channel();
        assert!(matches!(
            peer.recv().expect("answer").payload,
            weaver_types::Payload::Answer(LifecycleAnswer::AtRest)
        ));
    }

    /// **A close that opens no turn names none**, per
    /// `weaver-gate-world-contract` section 3. A line that does not parse is
    /// refused before the seat is granted, so there is no turn for the close
    /// to identify and it says only what it is.
    ///
    /// The distinction is the point rather than an omission: a named close
    /// reports what became of a turn, and an unnamed one reports that a line
    /// never became a turn at all. Perturbation: render the refusal with a
    /// reconstructed key and this fails, which is what it is here to prevent,
    /// a key rebuilt from the ordinal being a second chance to disagree with
    /// the record.
    #[test]
    fn a_close_that_opens_no_turn_names_none() {
        let refused = render_close("refused", "reason", "the line is not a request", None);
        assert_eq!(
            refused,
            r#"{"kind":"refused","reason":"the line is not a request"}"#
        );
        assert!(!refused.contains("turn"), "no turn opened: {refused}");
        assert!(!refused.contains("run"), "no turn opened: {refused}");
    }

    /// **A frame grants the seat and the whole bracket authors**, per Spec
    /// section 6.2 and the dev boundary: the parse at the threshold, the
    /// entry across the crossing, the turn against a scripted decode peer,
    /// and the response frame answered on the exchange, kind-named.
    #[test]
    fn a_frame_grants_the_seat_and_is_answered() {
        use std::os::fd::AsRawFd;

        use nix::sys::socket::{
            AddressFamily, MsgFlags, SockFlag, SockType, recv as sock_recv, send as sock_send,
            socketpair,
        };

        let (mut run, _spare, _sink) = entered_run(None);
        // The decode arm is re-pointed at a scripted peer, engine-test style.
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("socketpair");
        run.spu.as_mut().expect("the arm stands").decode = crate::channel::decode_from_owned(near);
        let (gate_end, gate_peer) = OrganChannel::pair().expect("gate pair");
        run.gate = Some(GateChannel {
            channel: gate_end,
            pid: nix::unistd::Pid::from_raw(1),
        });
        let gate_peer = gate_peer.into_channel();

        let decode_peer = std::thread::spawn(move || {
            let mut buf = vec![0u8; 65536];
            let n = sock_recv(far.as_raw_fd(), &mut buf, MsgFlags::empty()).expect("recv append");
            let directive: weaver_types::TokenDirective =
                serde_json::from_slice(&buf[..n]).expect("append parses");
            assert!(matches!(
                directive,
                weaver_types::TokenDirective::AppendAndGenerate { .. }
            ));
            let request = serde_json::value::RawValue::from_string(
                r#"{"rendered":"user: say a word","template":"qwen2","sampling":{}}"#.to_string(),
            )
            .unwrap();
            let measurement = serde_json::value::RawValue::from_string(
                r#"{"model":"m","weights_hash":"h","input_tokens":[1],"output_tokens":[2],"blocks":[],"timings":{"prefill_ns":"1","decode_ns":"2"}}"#
                    .to_string(),
            )
            .unwrap();
            let answer = weaver_types::TokenAnswer::Generated(weaver_types::Generation {
                content: vec![],
                emission: "one word".into(),
                finish: weaver_types::Finish::Completed,
                request,
                measurement,
                resident: 64,
                capacity: 4096,
            });
            let bytes = serde_json::to_vec(&answer).expect("answer renders");
            sock_send(far.as_raw_fd(), &bytes, MsgFlags::empty()).expect("send answer");
        });

        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent".into(),
                gate: "/nonexistent".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(run)),
        };

        // The frame stands on the channel before the wake is served, the way
        // the poll's readiness would have found it.
        gate_peer
            .send(&OrganEnvelope {
                exchange: ExchangeId {
                    opener: Opener::Gate,
                    ordinal: 1,
                },
                position: Position::Open,
                payload: weaver_types::Payload::Frame(weaver_types::TurnFrame::carry(
                    b"{\"text\":\"say a word\"}",
                )),
            })
            .expect("frame sends");

        harness
            .serve_gate_wake(
                "identity",
                &[],
                &mut |ports: &mut crate::engine::Ports<'_>, text: &str| {
                    let delta = vec![weaver_traits::Message {
                        role: weaver_traits::Role::User,
                        content: vec![weaver_traits::ContentBlock::Text {
                            text: text.to_string(),
                        }],
                    }];
                    ports.turn(delta)
                },
                &mut None,
            )
            .expect("the wake serves");
        decode_peer.join().expect("the decode peer finishes");

        // The response frame, kind-named, on the exchange the frame opened.
        let answer = gate_peer.recv().expect("the response frame returns");
        assert_eq!(answer.exchange.ordinal, 1);
        let weaver_types::Payload::Frame(frame) = answer.payload else {
            panic!("a frame answers a frame");
        };
        let line = String::from_utf8(frame.octets().expect("canonical")).expect("utf8");
        // **The close names the turn it answers and the run it belongs to**,
        // per `weaver-gate-world-contract` section 3. Pinned as the whole line
        // rather than by membership, so a member added here has to be argued
        // rather than slipping past a contains check.
        assert_eq!(
            line,
            r#"{"kind":"answered","run":"r-1","text":"one word","turn":"t-1"}"#
        );

        // The bracket authored whole, in order.
        let ChannelState::Entered(run) = &harness.state else {
            panic!("the position stays entered");
        };
        let kinds: Vec<Kind> = run
            .recorder
            .structure().expect("the serving record")
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
            ]
        );
    }

    /// **The parse refuses rather than faults**, per Spec 6.2's threshold
    /// clause: a line that is not the request answers `refused` as content,
    /// no seat is granted, and the channel stands for the next frame, which
    /// is the layer split the frame election bought.
    #[test]
    fn an_unparseable_line_refuses_and_the_channel_stands() {
        let (mut run, _spare, _sink) = entered_run(None);
        let (gate_end, gate_peer) = OrganChannel::pair().expect("gate pair");
        run.gate = Some(GateChannel {
            channel: gate_end,
            pid: nix::unistd::Pid::from_raw(1),
        });
        let gate_peer = gate_peer.into_channel();
        let mut harness = Harness {
            coordination: test_listener(),
            organs: OrganBinaries {
                classify: None,
                spu: "/nonexistent".into(),
                gate: "/nonexistent".into(),
            },
            parameters: OrganParameters::default(),
            state: ChannelState::Entered(Box::new(run)),
        };
        let mut entered = false;
        let mut entry = |_: &mut crate::engine::Ports<'_>, _: &str| {
            entered = true;
            Err(crate::engine::TurnError::Unlicensed {
                turn: TurnKey("t-0".into()),
            })
        };

        // Not the canonical carriage at all.
        gate_peer
            .send(&OrganEnvelope {
                exchange: ExchangeId {
                    opener: Opener::Gate,
                    ordinal: 1,
                },
                position: Position::Open,
                payload: weaver_types::Payload::Frame(weaver_types::TurnFrame {
                    octets: "not base64!".into(),
                }),
            })
            .expect("frame sends");
        let mut verb_slot = None;
        harness
            .serve_gate_wake("", &[], &mut entry, &mut verb_slot)
            .expect("the wake serves");
        let answer = gate_peer.recv().expect("the refusal returns");
        let weaver_types::Payload::Frame(frame) = answer.payload else {
            panic!("a frame answers a frame");
        };
        let line = String::from_utf8(frame.octets().expect("canonical")).expect("utf8");
        assert!(line.contains(r#""kind":"refused""#), "{line}");

        // Canonical carriage, wrong shape: an unknown member refuses too,
        // and the channel is alive to say so, which is the claim.
        gate_peer
            .send(&OrganEnvelope {
                exchange: ExchangeId {
                    opener: Opener::Gate,
                    ordinal: 2,
                },
                position: Position::Open,
                payload: weaver_types::Payload::Frame(weaver_types::TurnFrame::carry(
                    b"{\"text\":\"hi\",\"role\":\"system\"}",
                )),
            })
            .expect("frame sends");
        harness
            .serve_gate_wake("", &[], &mut entry, &mut verb_slot)
            .expect("the channel stands");
        let answer = gate_peer.recv().expect("the second refusal returns");
        let weaver_types::Payload::Frame(frame) = answer.payload else {
            panic!("a frame answers a frame");
        };
        let line = String::from_utf8(frame.octets().expect("canonical")).expect("utf8");
        assert!(line.contains(r#""kind":"refused""#), "{line}");

        assert!(
            !entered,
            "no seat was granted for a line that did not parse"
        );
    }
}
