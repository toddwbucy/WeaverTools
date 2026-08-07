//! conforms: gate-out-of-order-refused
//! conforms: gate-channel-state-three-positions
//! conforms: gate-closure-is-death
//!
//! Entry, the two hygiene sets, and wiring, and nothing else, per
//! `weaver-gate-Spec` sections 1 and 2.
//!
//! **The exchange service is a serial loop over the channel.** A directive out
//! of order for the channel's state answers `OutOfOrder`, per
//! `weaver-harness-gate-contract` section 3, and the state has three positions,
//! before-raise, raised, and lowered, the last terminal.

use std::process::ExitCode;

use nix::poll::{PollFd, PollFlags, PollTimeout, poll};

use weaver_gate::channel::{self, Channel, ChannelFault, EntryFault};
use weaver_gate::hook::{AcceptOutcome, Admitted, Hook, RaiseRefusal};
use weaver_types::{
    LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Opener, OrganEnvelope, Payload,
    Position,
};

/// The hook's state, as a type rather than a flag.
///
/// Named for the hook rather than the channel so it cannot be misread beside
/// `weaver_types::Position`, which is an envelope's place in its exchange and
/// a different thing entirely.
///
/// **Three positions, the last terminal.** A directive against a lowered hook
/// is refused by a match arm rather than by a flag check, which is what makes
/// the ordering the compiler's business: adding a position breaks every match
/// that judges one.
///
/// The hook lives inside `Raised` rather than beside it, so a raised state
/// without a listener and a lowered state still holding one are both
/// unrepresentable.
enum HookState {
    /// No raise has arrived. Only a raise is in order.
    BeforeRaise,
    /// The hook stands. Only a lower is in order.
    ///
    /// The admitted connections ride with it, because they exist only while it
    /// does: the lower drops the hook and them together, which is what makes
    /// `weaver-gate-world-contract` section 5's "a request while the hook is
    /// lowered finds no listener" true of a connection that was already open.
    Raised(Hook, Vec<Admitted>),
    /// Terminal. A directive of any kind arriving here answers `OutOfOrder`.
    Lowered,
}

fn main() -> ExitCode {
    // Entry performs its two hygiene sets and one election before the first
    // read.
    match channel::adopt() {
        Ok(channel) => serve(channel),
        Err(EntryFault::Unusable) => {
            // Mute by construction: with no usable channel the only refusal
            // available is the exit the harness observes.
            eprintln!(
                "{}",
                serde_json::json!({"refusal": "descriptors_unusable"})
            );
            ExitCode::FAILURE
        }
        Err(EntryFault::HygieneFailed(channel)) => {
            // **The channel is usable, so this refuses in words rather than by
            // dying.** Exiting here would reach the harness as a closed channel
            // and be aggregated as a no-residency, sending the operator to
            // debug the wrong subsystem for what the floor already names.
            eprintln!("{}", serde_json::json!({"refusal": "boundary_unverified"}));
            refuse_everything(channel)
        }
    }
}

/// A process that failed its hygiene sets serves nothing and answers every
/// directive with the floor's case for exactly this: the boundary could not be
/// established. It runs the same loop shape so the harness reads a typed reason
/// on the exchange it opened.
fn refuse_everything(channel: Channel) -> ExitCode {
    loop {
        let envelope = match channel.recv() {
            Ok(envelope) => envelope,
            Err(ChannelFault::Closed) => return ExitCode::FAILURE,
            Err(_) => return ExitCode::FAILURE,
        };
        let answered = channel.send(&OrganEnvelope {
            exchange: envelope.exchange,
            position: Position::Close,
            payload: Payload::Refusal(LifecycleRefusal::BoundaryUnverified),
        });
        if answered.is_err() {
            return ExitCode::FAILURE;
        }
    }
}

/// The serial loop.
///
/// **Closure is death.** A read that returns closure means the interior is
/// gone: this crate closes its listener if one stands and exits, never treating
/// closure as an answer. The listener's close is the drop of the position,
/// which happens on the way out of this function.
fn serve(channel: Channel) -> ExitCode {
    let mut state = HookState::BeforeRaise;

    loop {
        // **Wait on the channel and, while raised, the hook's listener.** A
        // loop that waited on the channel alone would leave a dialing peer in
        // the backlog unjudged for the whole raised window, which is the
        // boundary not existing rather than the boundary being quiet.
        let mut waiting = Vec::with_capacity(2);
        waiting.push(PollFd::new(channel.as_fd(), PollFlags::POLLIN));
        if let HookState::Raised(hook, _) = &state {
            waiting.push(PollFd::new(hook.listener(), PollFlags::POLLIN));
        }
        match poll(&mut waiting, PollTimeout::NONE) {
            Ok(_) => {}
            Err(nix::errno::Errno::EINTR) => continue,
            Err(_) => {
                drop(state);
                return ExitCode::FAILURE;
            }
        }

        let channel_ready = waiting[0]
            .revents()
            .is_some_and(|r| r.intersects(PollFlags::POLLIN | PollFlags::POLLHUP));
        let listener_ready = waiting
            .get(1)
            .and_then(|p| p.revents())
            .is_some_and(|r| r.contains(PollFlags::POLLIN));

        // The channel first: a lower waiting behind a queue of dials would let
        // a caller hold the hook open by dialing it.
        if !channel_ready {
            if listener_ready && let HookState::Raised(hook, admitted) = &mut state {
                judge_one(hook, admitted);
            }
            continue;
        }

        let envelope = match channel.recv() {
            Ok(envelope) => envelope,
            Err(ChannelFault::Closed) => {
                // The interior is gone. Dropping the state closes the listener
                // and every admitted connection with it, and this exits rather
                // than answering.
                drop(state);
                return ExitCode::SUCCESS;
            }
            Err(fault) => {
                eprintln!("{}", fault_line(&fault));
                drop(state);
                return ExitCode::FAILURE;
            }
        };

        let payload = dispatch(&mut state, &envelope);
        match channel.send(&OrganEnvelope {
            exchange: envelope.exchange,
            position: Position::Close,
            payload,
        }) {
            Ok(()) => {}
            // **The interior went away between the directive and its answer,
            // which is the ordinary teardown rather than a failure here.** The
            // harness's leave sends Lower, drops the channel, and reaps without
            // reading the answer, so this is the path every clean session
            // takes: treating it as a failure made the gate exit non-zero on
            // every orderly unload, and any supervisor reading exit status
            // recorded a fault per session.
            Err(ChannelFault::Closed) => {
                drop(state);
                return ExitCode::SUCCESS;
            }
            // The other two mean this crate built something it cannot send,
            // which is this crate's fault and is reported as one.
            Err(fault) => {
                eprintln!("{}", fault_line(&fault));
                drop(state);
                return ExitCode::FAILURE;
            }
        }
    }
}

/// How many admitted connections the raised window retains before it refuses
/// further ones by closure.
///
/// **A bound rather than a capacity.** In this pass no legitimate client
/// traffic exists, the relay being deferred, so this exists to keep a peer that
/// dials in a loop from consuming the gate's descriptor table rather than to
/// size a workload. Sizing it against real traffic is the relay act's, which is
/// also where retained connections stop being retained and start being served.
const RETAINED_LIMIT: usize = 64;

/// Accept one dialing peer and judge it.
///
/// **The refusal half is what this act implements**, and it is what
/// `weaver-gate-world-contract` section 5 specifies end to end: a peer that
/// fails the predicate is refused at accept, before any content is read, by
/// closure with nothing written back. `Hook::accept` returns no stream for a
/// refused peer, so the closure is the drop inside it.
///
/// **An admitted peer is held rather than served or closed, up to a bound.**
/// What it is owed is a conversation, which is the relay of Spec section 4 and
/// is deferred, so closing it would refuse a peer the predicate admitted and
/// answering it would be inventing the relay. Holding it is the truthful third
/// thing: its request is pending. The connections ride with the hook and go
/// when the lower does.
///
/// **Past the bound the peer is closed, and the accept still runs.** Retaining
/// without a limit lets a peer dial in a loop until the descriptor table is
/// full, after which every accept fails and, because the listener stays
/// readable, the loop spins: re-polling, re-failing, and flooding standard
/// error while directives wait behind it. Declining to accept at the bound
/// produces the same spin for the same reason, so the accept always happens and
/// what changes at the bound is whether the connection is kept. Draining the
/// backlog is what keeps the loop honest.
fn judge_one(hook: &Hook, admitted: &mut Vec<Admitted>) {
    match hook.accept() {
        Ok(peer) => {
            if admitted.len() < RETAINED_LIMIT {
                admitted.push(peer);
            }
            // Past the bound the connection drops here, closing it. The peer
            // passed the predicate and is refused by capacity rather than by
            // identity, which is a distinction the relay act will need to make
            // answerable and this pass can only make bounded.
        }
        // Refused, and already closed by the accept that judged it.
        Err(AcceptOutcome::Denied) => {}
        Err(AcceptOutcome::Unusable { detail }) => {
            eprintln!(
                "{}",
                serde_json::json!({"fault": "accept_failed", "detail": detail})
            );
        }
    }
}

fn fault_line(fault: &ChannelFault) -> String {
    match fault {
        ChannelFault::Truncated { bound } => {
            serde_json::json!({"fault": "truncated", "bound": bound}).to_string()
        }
        ChannelFault::Undecodable => serde_json::json!({"fault": "undecodable"}).to_string(),
        ChannelFault::Closed => serde_json::json!({"fault": "closed"}).to_string(),
    }
}

/// **The two exchanges of `weaver-harness-gate-contract`, judged against the
/// channel's position first.**
///
/// The contract draws raise and lower. Everything else, including a well-formed
/// directive at the wrong position, a directive that does not open its
/// exchange, and an exchange claiming an opener that is not the harness,
/// answers `OutOfOrder` before the hook is touched.
///
/// The match carries no wildcard arm over the directive, so a case added to
/// loop 0 breaks this crate loudly in the act that edits the floor.
fn dispatch(state: &mut HookState, envelope: &OrganEnvelope) -> Payload {
    if envelope.position != Position::Open || envelope.exchange.opener != Opener::Harness {
        return Payload::Refusal(LifecycleRefusal::OutOfOrder);
    }
    let Payload::Directive(directive) = &envelope.payload else {
        return Payload::Refusal(LifecycleRefusal::OutOfOrder);
    };

    match (&*state, directive) {
        (HookState::BeforeRaise, LifecycleDirective::Raise { instruction }) => {
            match Hook::raise(instruction) {
                Ok(hook) => {
                    // Ready is answered only after the bind and listen have
                    // returned, which is what makes ready a fact about the
                    // listener rather than a statement of intent.
                    *state = HookState::Raised(hook, Vec::new());
                    Payload::Answer(LifecycleAnswer::GateReady)
                }
                Err(RaiseRefusal::BindFailed { detail }) => {
                    // The reason travels to standard error, because the floor's
                    // closed refusal set carries no detail field and the
                    // operator who must clear the path needs to know which one.
                    // A refusal leaves nothing held: the position is unchanged,
                    // so no listener and no half-bound socket stand, and the
                    // aggregate's rollback has nothing of this crate's to
                    // unwind.
                    eprintln!(
                        "{}",
                        serde_json::json!({"refusal": "bind_failed", "detail": detail})
                    );
                    Payload::Refusal(LifecycleRefusal::BindFailed)
                }
            }
        }
        (HookState::Raised(_, _), LifecycleDirective::Lower) => {
            // The close happens here, before the answer is formed, so nothing
            // new can arrive once the harness reads stopped.
            let previous = std::mem::replace(state, HookState::Lowered);
            if let HookState::Raised(hook, admitted) = previous {
                // The admitted connections close with the listener, which is
                // what makes a lowered hook find nothing standing.
                drop(admitted);
                hook.lower();
            }
            Payload::Answer(LifecycleAnswer::GateStopped)
        }

        // Every other pairing is out of order: a lower before any raise, a
        // second raise, anything after a lower, and every directive the
        // contract does not draw. Refused before the hook is touched, which is
        // what not-queued means.
        (
            _,
            LifecycleDirective::Raise { .. }
            | LifecycleDirective::Lower
            | LifecycleDirective::Enter { .. }
            | LifecycleDirective::Leave
            | LifecycleDirective::Stop
            | LifecycleDirective::Admit { .. }
            | LifecycleDirective::Release
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
    use std::collections::BTreeSet;
    use weaver_types::{AccessRule, AgentName, ExchangeId, GateInstruction};

    fn opened(directive: LifecycleDirective) -> OrganEnvelope {
        OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: 1,
            },
            position: Position::Open,
            payload: Payload::Directive(directive),
        }
    }

    fn instruction(path: std::path::PathBuf) -> GateInstruction {
        GateInstruction {
            socket_path: path,
            access_rule: AccessRule {
                allowed_uids: BTreeSet::new(),
                allowed_gids: BTreeSet::new(),
                denied_uids: BTreeSet::new(),
            },
        }
    }

    /// A scratch socket path, pre-cleaned, matching the boundary and entry
    /// helpers so a previous run's leftover cannot make a raise refuse.
    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("weaver-gate-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a scratch dir");
        let path = dir.join("gate.sock");
        std::fs::remove_file(&path).ok();
        path
    }

    /// **A lower before any raise answers `OutOfOrder`.** The order is judged
    /// against the position before the hook is touched.
    ///
    /// Perturbation: give `(BeforeRaise, Lower)` an arm that answers
    /// `GateStopped` and this test fails. Watched under exactly that addition.
    #[test]
    fn a_lower_before_any_raise_answers_out_of_order() {
        let mut state = HookState::BeforeRaise;
        assert_eq!(
            dispatch(&mut state, &opened(LifecycleDirective::Lower)),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );
        assert!(matches!(state, HookState::BeforeRaise), "and is not queued");
    }

    /// **The lowered position is terminal.** A directive of any kind arriving
    /// after a lower answers `OutOfOrder`.
    #[test]
    fn any_directive_after_a_lower_answers_out_of_order() {
        let mut state = HookState::Lowered;
        for case in [
            LifecycleDirective::Raise {
                instruction: instruction(scratch("terminal")),
            },
            LifecycleDirective::Lower,
            LifecycleDirective::List,
        ] {
            assert_eq!(
                dispatch(&mut state, &opened(case)),
                Payload::Refusal(LifecycleRefusal::OutOfOrder)
            );
            assert!(matches!(state, HookState::Lowered), "terminal stays terminal");
        }
    }

    /// **A raise answers ready, and a lower after it answers stopped**, each
    /// once, in order.
    #[test]
    fn raise_then_lower_walks_the_three_positions() {
        let path = scratch("walk");
        let mut state = HookState::BeforeRaise;

        let ready = dispatch(
            &mut state,
            &opened(LifecycleDirective::Raise {
                instruction: instruction(path.clone()),
            }),
        );
        assert_eq!(ready, Payload::Answer(LifecycleAnswer::GateReady));
        assert!(matches!(state, HookState::Raised(_, _)));
        assert!(path.exists(), "ready is a fact about a bound listener");

        // A second raise is out of order even though the first succeeded.
        assert_eq!(
            dispatch(
                &mut state,
                &opened(LifecycleDirective::Raise {
                    instruction: instruction(path.clone()),
                }),
            ),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );

        let stopped = dispatch(&mut state, &opened(LifecycleDirective::Lower));
        assert_eq!(stopped, Payload::Answer(LifecycleAnswer::GateStopped));
        assert!(matches!(state, HookState::Lowered));
        // **This crate unlinks nothing**: the path is the operator's artifact
        // and survives the lower.
        assert!(path.exists(), "the lower closes the listener and unlinks nothing");
        std::fs::remove_file(&path).ok();
    }

    /// **A refused raise leaves nothing held.** The position is unchanged, so
    /// the aggregate's rollback has nothing of this crate's to unwind, and the
    /// refusal is answered rather than exited on.
    ///
    /// Perturbation: move `*position = HookState::Raised(..)` before the bind's
    /// result is judged and this test fails. Watched under exactly that move.
    #[test]
    fn a_refused_raise_holds_nothing_and_is_answered() {
        // A path inside a directory that does not exist cannot be bound. Built
        // from this test's own scratch directory with a missing component in
        // the middle, so the fixture does not rest on `/nonexistent` being
        // absent on whatever machine runs it.
        let unbindable = scratch("refused")
            .parent()
            .expect("the scratch dir")
            .join("no-such-directory")
            .join("gate.sock");
        let mut state = HookState::BeforeRaise;
        assert_eq!(
            dispatch(
                &mut state,
                &opened(LifecycleDirective::Raise {
                    instruction: instruction(unbindable),
                }),
            ),
            Payload::Refusal(LifecycleRefusal::BindFailed),
            "the refusal is answered, never exited on"
        );
        assert!(
            matches!(state, HookState::BeforeRaise),
            "and nothing is held: no listener, no half-bound socket"
        );
    }

    /// Everything outside the drawn vocabulary refuses as out of order.
    #[test]
    fn a_directive_outside_the_drawn_vocabulary_refuses() {
        let mut state = HookState::BeforeRaise;
        for case in [
            LifecycleDirective::Leave,
            LifecycleDirective::Stop,
            LifecycleDirective::Release,
            LifecycleDirective::List,
            LifecycleDirective::Show {
                agent: AgentName("alpha".into()),
            },
        ] {
            assert_eq!(
                dispatch(&mut state, &opened(case)),
                Payload::Refusal(LifecycleRefusal::OutOfOrder)
            );
        }
    }

    /// A mis-shapen exchange is refused before the directive's case is read.
    #[test]
    fn a_mis_shapen_exchange_refuses_before_the_directive_runs() {
        let mut state = HookState::BeforeRaise;
        let mut closing = opened(LifecycleDirective::Raise {
            instruction: instruction(scratch("misshapen")),
        });
        closing.position = Position::Close;
        assert_eq!(
            dispatch(&mut state, &closing),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );
        assert!(matches!(state, HookState::BeforeRaise), "nothing was bound");
    }
}
