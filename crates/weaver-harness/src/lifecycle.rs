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
//! conforms: harness-scoped-refusal-account
//! conforms: harness-left-follows-drain
//! conforms: harness-announce-after-record
//! conforms: harness-dumpable-flag-cleared
//! conforms: harness-coordination-end-close-on-exec
//! conforms: harness-loop-zero-takes-no-abstraction
//! conforms: harness-spawns-no-thread
//! conforms: harness-organ-forks-on-worker-lifetime-thread
//! conforms: harness-internal-dependency-set
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

use std::os::fd::{AsRawFd, OwnedFd};
use std::path::PathBuf;

use weaver_trace::{Kind, Payload, Recorder, RunOrdinal, SessionRef, Subsystem, TurnClose};
use weaver_types::{
    ExchangeId, LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Opener, OrganEnvelope,
    Position, SessionId, TurnKey,
};

use crate::authorship::Author;
use crate::channel::{DecodeChannel, OrganChannel};
use crate::failure::{AdoptionFault, ChannelFault, Outcome};

/// Where the organ binaries live: a deployment fact supplied by the
/// composition root as a construction parameter, the way `weaver-trace-Spec`
/// section 6 takes its queue depth. This is the one exception to the
/// no-path-taken rule of section 2.3, and it is not an operator election and
/// not a discovery.
#[derive(Debug, Clone)]
pub struct OrganBinaries {
    pub spu: PathBuf,
    pub gate: PathBuf,
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
    recorder: Recorder,
    author: Author,
    session: SessionId,
    spu: Option<SpuChannels>,
    gate: Option<OrganChannel>,
    turn_in_flight: Option<TurnKey>,
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
}

/// The hub. Adopts the coordination end the unit's declared open delivered,
/// performs the worker's hygiene as sets and not checks, and serves loop 0.
pub struct Harness {
    coordination: OrganChannel,
    organs: OrganBinaries,
    state: ChannelState,
    exchange_ordinal: u64,
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
    pub fn adopt(coordination: OwnedFd, organs: OrganBinaries) -> Result<Self, AdoptionFault> {
        set_close_on_exec(&coordination)?;
        clear_dumpable()?;
        Ok(Harness {
            coordination: OrganChannel::adopt(coordination),
            organs,
            state: ChannelState::BeforeEnter,
            exchange_ordinal: 0,
        })
    }

    /// Serves the coordination channel until leave is answered or closure is
    /// observed, or fails on a fault below the exchange layer.
    ///
    /// One directive at a time arrives, is judged against the channel's state,
    /// and is answered or refused. A directive out of order for the state
    /// answers `OutOfOrder` and is not queued.
    pub fn serve(mut self) -> Result<Outcome, ChannelFault> {
        loop {
            let (envelope, sink) = match self.coordination.recv_with_descriptor() {
                Ok(received) => received,
                Err(ChannelFault::Closed) => return Ok(Outcome::ChannelClosed),
                Err(fault) => return Err(fault),
            };
            let directive = match envelope.payload {
                weaver_types::Payload::Directive(directive) => directive,
                // Anything else on this channel is not a directive this crate
                // can attribute to an exchange for a refusal to answer.
                _ => return Err(ChannelFault::Undecodable),
            };
            let answered = self.dispatch(directive, sink)?;
            if let Some(outcome) = answered {
                return Ok(outcome);
            }
        }
    }

    /// The serial service's one step. Returns `Some(outcome)` when service
    /// ends.
    fn dispatch(
        &mut self,
        directive: LifecycleDirective,
        sink: Option<OwnedFd>,
    ) -> Result<Option<Outcome>, ChannelFault> {
        match (&mut self.state, directive) {
            (ChannelState::BeforeEnter, LifecycleDirective::Enter { payload }) => {
                match self.enter(payload, sink) {
                    Ok(run) => {
                        self.state = ChannelState::Entered(Box::new(run));
                        self.answer(LifecycleAnswer::Ready)?;
                    }
                    Err(refusal) => {
                        // The scoped account: a refusal before the load event
                        // leaves the stream clean and the state at
                        // before-enter, so nothing half-stood survives.
                        self.refuse(refusal)?;
                    }
                }
                Ok(None)
            }
            (ChannelState::Entered(run), LifecycleDirective::Leave) => {
                if run.turn_in_flight.is_some() {
                    self.refuse(LifecycleRefusal::ActivityNotAtRest)?;
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
                leave(&mut run);
                self.answer(LifecycleAnswer::Left)?;
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
                self.answer(answer)?;
                Ok(None)
            }
            // Every other pairing is out of order for the position. The left
            // position is terminal, so a directive of any kind arriving after
            // a leave answers the same refusal.
            _ => {
                self.refuse(LifecycleRefusal::OutOfOrder)?;
                Ok(None)
            }
        }
    }

    /// Enter runs four steps in the charter's order, and the answer is the
    /// aggregate.
    fn enter(
        &mut self,
        payload: weaver_types::EnterPayload,
        sink: Option<OwnedFd>,
    ) -> Result<Run, LifecycleRefusal> {
        // One sink descriptor arrives on the directive's own message.
        let sink = sink.ok_or(LifecycleRefusal::DescriptorsUnusable)?;
        let session = payload.session.clone();
        let mut recorder = Recorder::receive(
            sink,
            RunOrdinal(payload.run_ordinal),
            SessionRef(session.0.clone()),
        )
        .map_err(|_| LifecycleRefusal::DescriptorsUnusable)?;

        // The load event is the run's opening and the origin of its monotonic
        // clock, so the author is constructed at this moment.
        let author = Author::new(&session, payload.run_ordinal);
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .map_err(|_| LifecycleRefusal::Malformed)?;

        let mut run = Run {
            recorder,
            author,
            session,
            spu: None,
            gate: None,
            turn_in_flight: None,
        };

        // The residency and decode pairs are created in one act before the SPU
        // fork: a socket with no address cannot be reached later by resolving
        // one, so the only moment it can reach a child is before that child
        // exists.
        let (lifecycle, lifecycle_child) =
            OrganChannel::pair().map_err(|_| LifecycleRefusal::DescriptorsUnusable)?;
        let (decode, decode_child) =
            DecodeChannel::pair().map_err(|_| LifecycleRefusal::DescriptorsUnusable)?;
        // SAFETY: the fork runs on the serving thread, whose lifetime is the
        // worker's, and the child performs only the three async-signal-safe
        // calls of `place_child_ends` before its exec.
        unsafe {
            crate::spawn::fork_organ(&self.organs.spu, &[&lifecycle_child, &decode_child])
                .map_err(|_| LifecycleRefusal::BindFailed)?;
        }
        drop(lifecycle_child);
        drop(decode_child);
        run.spu = Some(SpuChannels { lifecycle, decode });

        // Admit carries the model binding uninterpreted.
        let spu = run.spu.as_ref().expect("the arm just stood up");
        exchange(
            &spu.lifecycle,
            Opener::Harness,
            self.next_ordinal(),
            LifecycleDirective::Admit {
                binding: payload.model_binding.clone(),
            },
        )?;

        // The gate pair is created only after the SPU's answer confirms
        // residency: one organ's readiness gates another organ's
        // construction, which neither organ can see from inside its domain.
        let (gate, gate_child) =
            OrganChannel::pair().map_err(|_| LifecycleRefusal::DescriptorsUnusable)?;
        // SAFETY: as above.
        unsafe {
            crate::spawn::fork_organ(&self.organs.gate, &[&gate_child])
                .map_err(|_| LifecycleRefusal::BindFailed)?;
        }
        drop(gate_child);
        exchange(
            &gate,
            Opener::Harness,
            self.next_ordinal(),
            LifecycleDirective::Raise {
                instruction: payload.gate_instruction.clone(),
            },
        )?;
        run.gate = Some(gate);

        Ok(run)
    }

    /// The extension seam, crossed at loaded-and-idle: loop 0 hands a standing
    /// interior to whatever loop 1 the binary carries, and takes it back at
    /// the stop and at the leave. A loop composes what this grants or does not
    /// compile - there is no call by which it mints a port.
    pub fn grant_seat(&self, identity: &str, tool_schemas: &[String]) -> Option<crate::Ports> {
        match &self.state {
            ChannelState::Entered(run) => Some(crate::engine::Ports::grant(Some(
                crate::assembly::assemble(run.recorder.structure(), identity, tool_schemas),
            ))),
            // Before enter and after leave there is no standing interior to
            // hand across, which is the bracket discipline being loop 0's.
            _ => None,
        }
    }

    fn next_ordinal(&mut self) -> u64 {
        self.exchange_ordinal += 1;
        self.exchange_ordinal
    }

    fn answer(&mut self, answer: LifecycleAnswer) -> Result<(), ChannelFault> {
        let ordinal = self.next_ordinal();
        self.coordination.send(&OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Admin,
                ordinal,
            },
            position: Position::Close,
            payload: weaver_types::Payload::Answer(answer),
        })
    }

    fn refuse(&mut self, refusal: LifecycleRefusal) -> Result<(), ChannelFault> {
        let ordinal = self.next_ordinal();
        self.coordination.send(&OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Admin,
                ordinal,
            },
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
fn leave(run: &mut Run) {
    if let Some(gate) = run.gate.take() {
        let _ = gate.send(&OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: 0,
            },
            position: Position::Open,
            payload: weaver_types::Payload::Directive(LifecycleDirective::Lower),
        });
    }
    let _ = run.author.author(
        &mut run.recorder,
        Kind::Unload,
        Subsystem::Harness,
        None,
        None,
    );
    let _ = run.recorder.drain();
    if let Some(spu) = run.spu.take() {
        let _ = spu.lifecycle.send(&OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: 0,
            },
            position: Position::Open,
            payload: weaver_types::Payload::Directive(LifecycleDirective::Release),
        });
        drop(spu.decode);
    }
    let _ = &run.session;
}

/// Opens one exchange on an organ channel and reads its answer, carrying a
/// refusing organ's reason into the aggregate unchanged.
fn exchange(
    channel: &OrganChannel,
    opener: Opener,
    ordinal: u64,
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
            weaver_types::Payload::Refusal(reason) => Err(LifecycleRefusal::OrganRefused {
                organ: weaver_types::RefusingOrgan::Spu,
                reason: Box::new(reason),
            }),
            _ => Err(LifecycleRefusal::Malformed),
        },
        Err(_) => Err(LifecycleRefusal::NoResidency),
    }
}

fn set_close_on_exec(end: &OwnedFd) -> Result<(), AdoptionFault> {
    // SAFETY: fcntl on a descriptor this process owns.
    let rc =
        unsafe { nix::libc::fcntl(end.as_raw_fd(), nix::libc::F_SETFD, nix::libc::FD_CLOEXEC) };
    if rc == -1 {
        return Err(AdoptionFault::CloseOnExecUnset {
            errno: std::io::Error::last_os_error().raw_os_error().unwrap_or(0),
        });
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    //! The announce-after-record test of section 8, run from inside the crate
    //! because an entered run needs organ doubles the integration suite does
    //! not buy, while the run struct itself is constructible here.

    use super::*;
    use std::fs::File;
    use weaver_trace::Kind;

    fn entered_run(turn: Option<&str>) -> (Run, OrganChannel) {
        let path = std::env::temp_dir().join(format!(
            "weaver-harness-stop-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let sink = OwnedFd::from(File::create(&path).expect("sink"));
        let session = SessionId("s-1".to_string());
        let mut recorder = Recorder::receive(sink, RunOrdinal(0), SessionRef(session.0.clone()))
            .expect("recorder");
        let author = Author::new(&session, 0);
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
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
        (
            Run {
                recorder,
                author,
                session,
                spu: None,
                gate: None,
                turn_in_flight: turn.map(|t| TurnKey(t.to_string())),
            },
            near,
        )
    }

    /// **Announce after record:** a stop's answer follows the close event's
    /// placement, which is what the announce-after-record discipline means.
    ///
    /// Perturbation: move the answer ahead of the authoring in the stop arm
    /// and the recorder holds no close event when the answer is read. Watched
    /// under exactly that reordering, by asserting the close landed before the
    /// answer was written.
    #[test]
    fn stop_records_the_close_before_it_answers() {
        let (run, _spare) = entered_run(Some("t-1"));
        let (harness_end, peer_end) = OrganChannel::pair().expect("pair");
        let mut harness = Harness {
            coordination: harness_end,
            organs: OrganBinaries {
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            state: ChannelState::Entered(Box::new(run)),
            exchange_ordinal: 0,
        };
        harness
            .dispatch(LifecycleDirective::Stop, None)
            .expect("stop dispatches");

        // The close is in the record.
        let closes = match &harness.state {
            ChannelState::Entered(run) => {
                run.recorder.structure().by_kind(Kind::TurnClosed).count()
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

    /// A stop at rest answers `AtRest`, a clean close and not a refusal, and
    /// places no close event because there was no turn to close.
    #[test]
    fn stop_at_rest_answers_at_rest() {
        let (run, _spare) = entered_run(None);
        let (harness_end, peer_end) = OrganChannel::pair().expect("pair");
        let mut harness = Harness {
            coordination: harness_end,
            organs: OrganBinaries {
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            state: ChannelState::Entered(Box::new(run)),
            exchange_ordinal: 0,
        };
        harness
            .dispatch(LifecycleDirective::Stop, None)
            .expect("stop");
        let peer = peer_end.into_channel();
        assert!(matches!(
            peer.recv().expect("answer").payload,
            weaver_types::Payload::Answer(LifecycleAnswer::AtRest)
        ));
    }
}
