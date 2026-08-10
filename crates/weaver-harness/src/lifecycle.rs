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
//! conforms: harness-binds-coordination-socket-first
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

use std::os::fd::OwnedFd;
use std::path::PathBuf;

use weaver_trace::{Kind, Payload, Recorder, RunOrdinal, SessionRef, Subsystem, TurnClose};
use weaver_types::{
    ExchangeId, LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Opener, OrganEnvelope,
    Position, SessionId, TurnKey,
};

use crate::authorship::Author;
use crate::channel::{CoordinationListener, DecodeChannel, OrganChannel};
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
    gate: Option<GateChannel>,
    turn_in_flight: Option<TurnKey>,
    /// Each initiator numbers exchanges on its own channel, so the SPU's and
    /// the gate's counters are separate and neither is hardcoded.
    spu_ordinal: u64,
    gate_ordinal: u64,
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
    ) -> Result<Self, AdoptionFault> {
        clear_dumpable()?;
        Ok(Harness {
            coordination,
            organs,
            state: ChannelState::BeforeEnter,
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
            // **One connection at a time**, which is what holds the contract's
            // one-exchange-in-flight rule now that no fleet map does.
            let connection = match self.coordination.accept_root() {
                Ok(connection) => connection,
                // A peer that is not root never reaches an exchange. The
                // listener stands and the next dial is accepted, because
                // refusing one caller is not a reason to stop serving the one
                // party that may call.
                Err(ChannelFault::WrongPeer { .. }) => continue,
                // The listener itself failed, which is the one thing that ends
                // service without a leave.
                Err(fault) => {
                    self.unwind_if_entered();
                    return Err(fault);
                }
            };
            match self.serve_connection(connection) {
                Ok(Some(outcome)) => return Ok(outcome),
                // **The connection closed and the run did not.** Admin is
                // per-invocation, so each verb's connection closes when the
                // verb answers, and the state of section 3 is held across it,
                // per `weaver-admin-harness-contract` section 4. Unwinding
                // here would tear down the run the next verb expects to find.
                Ok(None) => continue,
                Err(fault) => {
                    self.unwind_if_entered();
                    return Err(fault);
                }
            }
        }
    }

    /// Serves the directives arriving on one connection. Returns `Some` when
    /// service ends, and `None` when the connection closed with the run still
    /// standing.
    fn serve_connection(
        &mut self,
        connection: OrganChannel,
    ) -> Result<Option<Outcome>, ChannelFault> {
        loop {
            let (envelope, sink) = match connection.recv_with_descriptor() {
                Ok(received) => received,
                // The verb answered and admin closed. Ordinary, and not an
                // ending.
                Err(ChannelFault::Closed) => return Ok(None),
                Err(fault) => return Err(fault),
            };
            let exchange = envelope.exchange.clone();
            let directive = match envelope.payload {
                weaver_types::Payload::Directive(directive) => directive,
                // Anything else on this channel is not a directive this crate
                // can attribute to an exchange for a refusal to answer.
                _ => return Err(ChannelFault::Undecodable),
            };
            if let Some(outcome) = self.dispatch_on(&connection, exchange, directive, sink)? {
                return Ok(Some(outcome));
            }
        }
    }

    /// Unwinds a standing run, for a path that ends service without a leave.
    fn unwind_if_entered(&mut self) {
        if let ChannelState::Entered(run) = std::mem::replace(&mut self.state, ChannelState::Left) {
            let mut run = *run;
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
    ) -> Result<Option<Outcome>, ChannelFault> {
        match (&mut self.state, directive) {
            (ChannelState::BeforeEnter, LifecycleDirective::Enter { payload }) => {
                match self.enter(payload, sink) {
                    Ok(run) => {
                        self.state = ChannelState::Entered(Box::new(run));
                        self.answer(connection, &exchange, LifecycleAnswer::Ready)?;
                    }
                    Err(EnterFailure::BeforeLoad(refusal)) => {
                        // Nothing was authored and nothing stood up, so the
                        // stream is clean and the position holds.
                        self.refuse(connection, &exchange, refusal)?;
                    }
                    Err(EnterFailure::AfterLoad(run, refusal)) => {
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

    /// Enter runs four steps in the charter's order, and the answer is the
    /// aggregate.
    fn enter(
        &mut self,
        payload: weaver_types::EnterPayload,
        sink: Option<OwnedFd>,
    ) -> Result<Run, EnterFailure> {
        // One sink descriptor arrives on the directive's own message. Nothing
        // is authored yet, so a refusal here leaves the stream clean.
        let sink = sink.ok_or(EnterFailure::BeforeLoad(
            LifecycleRefusal::DescriptorsUnusable,
        ))?;
        let session = payload.session.clone();
        let mut recorder = Recorder::receive(
            sink,
            RunOrdinal(payload.run_ordinal),
            SessionRef(session.0.clone()),
        )
        .map_err(|_| EnterFailure::BeforeLoad(LifecycleRefusal::DescriptorsUnusable))?;

        // The load event is the run's opening and the origin of its monotonic
        // clock, so the author is constructed at this moment.
        let author = Author::new(&session, payload.run_ordinal);
        author
            .author(&mut recorder, Kind::Load, Subsystem::Harness, None, None)
            .map_err(|_| EnterFailure::BeforeLoad(LifecycleRefusal::Malformed))?;

        // **Past this line the bracket stands**, so every refusal below
        // carries the partial run back rather than dropping it: the leave that
        // follows unwinds exactly what stood up, closes the bracket, and
        // drains. Dropping it here would orphan whatever forked and leave an
        // unclosed bracket on the stream.
        let mut run = Run {
            recorder,
            author,
            session,
            spu: None,
            gate: None,
            turn_in_flight: None,
            spu_ordinal: 0,
            gate_ordinal: 0,
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
            crate::spawn::fork_organ(&self.organs.spu, &[&lifecycle_child, &decode_child])
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

        // The gate pair is created only after the SPU's answer confirms
        // residency: one organ's readiness gates another organ's
        // construction, which neither organ can see from inside its domain.
        let (gate, gate_child) = match OrganChannel::pair() {
            Ok(pair) => pair,
            Err(_) => after_load!(run, LifecycleRefusal::DescriptorsUnusable),
        };
        // SAFETY: as above.
        let gate_pid = match unsafe { crate::spawn::fork_organ(&self.organs.gate, &[&gate_child]) }
        {
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
                instruction: payload.gate_instruction.clone(),
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
    pub fn grant_seat(&mut self, identity: &str, tool_schemas: &[String]) -> Option<crate::Ports> {
        match &mut self.state {
            // **The extension seam is crossed at loaded-and-idle itself.** A
            // turn in flight is the active state, not the idle one, and loop 0
            // has not taken the interior back yet, so there is no standing
            // interior to hand across.
            ChannelState::Entered(run) if run.turn_in_flight.is_some() => None,
            ChannelState::Entered(run) => {
                let prompt =
                    crate::assembly::assemble(run.recorder.structure(), identity, tool_schemas);
                // **An incomplete prompt does not cross the seam.** A
                // message-kind record that did not decode is a hole where a
                // turn's content was, so the count becomes the `fault` event
                // this crate authors and the seat is not granted: handing a
                // loop a prompt with a hole would put the loss in the model's
                // context and nowhere else.
                if prompt.undecodable > 0 {
                    let report = format!(
                        "{{\"organ\":\"harness\",\"undecodable-message-records\":{}}}",
                        prompt.undecodable
                    );
                    let turn = run.turn_in_flight.clone();
                    let _ = run.author.author_fault(
                        &mut run.recorder,
                        Subsystem::Harness,
                        turn.as_ref(),
                        &report,
                    );
                    return None;
                }
                Some(crate::engine::Ports::grant(Some(prompt)))
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
    if let Some(gate) = run.gate.take() {
        run.gate_ordinal += 1;
        let _ = gate.channel.send(&OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: run.gate_ordinal,
            },
            position: Position::Open,
            payload: weaver_types::Payload::Directive(LifecycleDirective::Lower),
        });
        drop(gate.channel);
        reap(gate.pid);
    }
    let _ = run.author.author(
        &mut run.recorder,
        Kind::Unload,
        Subsystem::Harness,
        None,
        None,
    );

    // **The drain's outcome is carried, not discarded.** `Left` means
    // everything admitted reached the stream, so a failed drain must not be
    // answered as one: the peer could not otherwise tell a complete stream
    // from a truncated one.
    let drained = run.recorder.drain();

    if let Some(spu) = run.spu.take() {
        run.spu_ordinal += 1;
        let _ = spu.lifecycle.send(&OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: run.spu_ordinal,
            },
            position: Position::Open,
            payload: weaver_types::Payload::Directive(LifecycleDirective::Release),
        });
        drop(spu.lifecycle);
        drop(spu.decode);
        reap(spu.pid);
    }
    let _ = &run.session;

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
                spu_ordinal: 0,
                gate_ordinal: 0,
            },
            near,
            path,
        )
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
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            state: ChannelState::Entered(Box::new(run)),
        };
        harness
            .dispatch_on(
                &harness_end,
                test_exchange(),
                LifecycleDirective::Stop,
                None,
            )
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
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            state: ChannelState::Entered(Box::new(run)),
        };
        harness.unwind_if_entered();
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
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
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
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            state: ChannelState::Entered(Box::new(idle)),
        };
        assert!(
            harness.grant_seat("identity", &[]).is_some(),
            "a loaded-and-idle run grants the seat"
        );
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
                spu: "/nonexistent/spu".into(),
                gate: "/nonexistent/gate".into(),
            },
            state: ChannelState::Entered(Box::new(run)),
        };
        harness
            .dispatch_on(
                &harness_end,
                test_exchange(),
                LifecycleDirective::Stop,
                None,
            )
            .expect("stop");
        let peer = peer_end.into_channel();
        assert!(matches!(
            peer.recv().expect("answer").payload,
            weaver_types::Payload::Answer(LifecycleAnswer::AtRest)
        ));
    }
}
