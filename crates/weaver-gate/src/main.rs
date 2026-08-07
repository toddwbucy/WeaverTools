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

use weaver_gate::channel::{self, Channel, ChannelFault, EntryFault};
use weaver_gate::hook::{Hook, RaiseRefusal};
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
    Raised(Hook),
    /// Terminal. A directive of any kind arriving here answers `OutOfOrder`.
    Lowered,
}

fn main() -> ExitCode {
    // Entry performs its two hygiene sets and one election before the first
    // read. A refusal here is a refusal to serve.
    let channel = match channel::adopt() {
        Ok(channel) => channel,
        Err(fault) => {
            eprintln!("{}", entry_refusal_line(&fault));
            return ExitCode::FAILURE;
        }
    };
    serve(channel)
}

fn entry_refusal_line(fault: &EntryFault) -> String {
    match fault {
        EntryFault::ChannelUnusable | EntryFault::AlreadyAdopted => {
            "{\"refusal\":\"descriptors_unusable\"}".to_string()
        }
        EntryFault::HygieneFailed => "{\"refusal\":\"boundary_unverified\"}".to_string(),
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
        let envelope = match channel.recv() {
            Ok(envelope) => envelope,
            Err(ChannelFault::Closed) => {
                // The interior is gone. Dropping the position closes the
                // listener if one stands, and this exits rather than answering.
                drop(state);
                return ExitCode::SUCCESS;
            }
            Err(fault) => {
                // Truncated or undecodable: faults below the exchange layer,
                // so neither is answered on an exchange, and a channel this
                // process cannot read faithfully is one it stops serving.
                eprintln!("{}", fault_line(&fault));
                drop(state);
                return ExitCode::FAILURE;
            }
        };

        let payload = dispatch(&mut state, &envelope);
        if channel
            .send(&OrganEnvelope {
                exchange: envelope.exchange,
                position: Position::Close,
                payload,
            })
            .is_err()
        {
            drop(state);
            return ExitCode::FAILURE;
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
                    *state = HookState::Raised(hook);
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
        (HookState::Raised(_), LifecycleDirective::Lower) => {
            // The close happens here, before the answer is formed, so nothing
            // new can arrive once the harness reads stopped.
            let previous = std::mem::replace(state, HookState::Lowered);
            if let HookState::Raised(hook) = previous {
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
        assert!(matches!(state, HookState::Raised(_)));
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
