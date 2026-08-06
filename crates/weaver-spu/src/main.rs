//! conforms: spu-service-serial-one-loop
//!
//! Entry, the hygiene sets, and the service loop, per `weaver-spu-Spec`
//! sections 2 and 3.
//!
//! **The service is serial per channel and the two channels are one loop.** One
//! lifecycle directive at a time, one decode exchange at a time, per both
//! contracts' ordering. Nothing here is concurrent in this pass, and the shape
//! that would change it is the executor election deferred in Spec section 1.1.

use std::process::ExitCode;

use weaver_spu::channel::{self, ChannelFault, EntryFault, Inherited, LifecycleChannel};
use weaver_spu::residency::{Headroom, Residency};
use weaver_types::{
    ExchangeId, LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Opener, OrganEnvelope,
    Payload, Position,
};

/// The headroom the worker's composition root supplies. A deployment fact
/// rather than an operator election, per Spec section 3, and a number a builder
/// can supply before the measurement that replaces it exists.
const HEADROOM_BYTES: u64 = 512 * 1024 * 1024;

fn main() -> ExitCode {
    // Entry adopts both ends and performs its two sets before the first read.
    // A refusal here is a refusal to serve: the count check failing means the
    // harness's fork discipline failed upstream, and this process is not the
    // one to continue past it.
    let inherited = match channel::adopt() {
        Ok(inherited) => inherited,
        Err(fault) => {
            eprintln!("{}", entry_refusal_line(&fault));
            return ExitCode::FAILURE;
        }
    };
    serve(inherited)
}

/// A refusal before any channel is trusted goes to standard error, because the
/// lifecycle channel is exactly what this path could not establish.
fn entry_refusal_line(fault: &EntryFault) -> String {
    match fault {
        EntryFault::DescriptorCountWrong { found } => format!(
            "{{\"refusal\":\"descriptors_unusable\",\"held_beyond_standard_streams\":{found}}}"
        ),
        EntryFault::DescriptorsUnusable => {
            "{\"refusal\":\"descriptors_unusable\"}".to_string()
        }
        EntryFault::HygieneFailed => "{\"refusal\":\"boundary_unverified\"}".to_string(),
    }
}

/// The one loop. One directive at a time against one resident session.
fn serve(inherited: Inherited) -> ExitCode {
    let Inherited { lifecycle, decode } = inherited;
    // The decode socket is held for the decode submodule's exchanges. It is
    // bound to this scope so that its close is this process's exit rather than
    // an earlier drop.
    let _decode = decode;

    let mut residency = Residency::new();

    loop {
        let envelope = match lifecycle.recv() {
            Ok(envelope) => envelope,
            Err(ChannelFault::Truncated) => {
                // A read returning MSG_TRUNC is a channel fault and never a
                // message. Continuing would mean acting on a silently shortened
                // directive, which is the failure the boundary property was
                // elected to prevent.
                eprintln!("{{\"fault\":\"truncated\"}}");
                return ExitCode::FAILURE;
            }
            Err(ChannelFault::Malformed) => {
                // Malformed is answerable, since the exchange layer is intact.
                if answer(&lifecycle, malformed_exchange(), Payload::Refusal(
                    LifecycleRefusal::Malformed,
                ))
                .is_err()
                {
                    return ExitCode::FAILURE;
                }
                continue;
            }
            Err(ChannelFault::PeerGone) => {
                // The harness owns this process's lifetime, so a closed channel
                // is an orderly exit rather than a retry or a failure.
                return ExitCode::SUCCESS;
            }
            Err(_) => {
                // The socket refused the read. Nothing can be answered over a
                // channel that cannot be read, so this exits rather than loops.
                return ExitCode::FAILURE;
            }
        };

        let payload = dispatch(&mut residency, &envelope);
        if answer(&lifecycle, envelope.exchange, payload).is_err() {
            return ExitCode::FAILURE;
        }
    }
}

/// The exchange named on a refusal to a message that did not parse. The opener
/// is the harness, since only the harness opens on this channel.
fn malformed_exchange() -> ExchangeId {
    ExchangeId {
        opener: Opener::Harness,
        ordinal: 0,
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

/// **The two exchanges of `weaver-harness-spu-contract`, and nothing else.**
///
/// The contract's vocabulary clause draws admit and release on the directive
/// and their confirmations on the answer. Every other case of the floor's
/// closed set is refused as `OutOfOrder`, which is a real obligation rather
/// than a formality: a receiving party matches its own cases and refuses the
/// rest.
///
/// The match carries no wildcard arm, so a case added to loop 0 breaks this
/// crate loudly in the act that edits the floor.
fn dispatch(residency: &mut Residency, envelope: &OrganEnvelope) -> Payload {
    let Payload::Directive(directive) = &envelope.payload else {
        // An answer, a refusal, a frame, or a fault arriving here is the peer
        // speaking out of turn on a channel this crate does not open.
        return Payload::Refusal(LifecycleRefusal::OutOfOrder);
    };

    match directive {
        LifecycleDirective::Admit { binding } => {
            match residency.admit(binding, Headroom(HEADROOM_BYTES)) {
                Ok(_) => Payload::Answer(LifecycleAnswer::Admitted),
                Err(refusal) => Payload::Refusal(refusal.into()),
            }
        }
        LifecycleDirective::Release => match residency.release() {
            Ok(()) => Payload::Answer(LifecycleAnswer::Released),
            Err(refusal) => Payload::Refusal(refusal),
        },

        // Everything below is outside this seam's drawn vocabulary.
        LifecycleDirective::Enter { .. }
        | LifecycleDirective::Leave
        | LifecycleDirective::Stop
        | LifecycleDirective::Raise { .. }
        | LifecycleDirective::Lower
        | LifecycleDirective::Load { .. }
        | LifecycleDirective::Unload { .. }
        | LifecycleDirective::Validate { .. }
        | LifecycleDirective::List
        | LifecycleDirective::Show { .. } => {
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use weaver_types::AgentName;

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

    /// **Only admit and release cross this seam.** Every other case of the
    /// floor's closed set refuses as out of order.
    ///
    /// Perturbation: answer `Stop` with `AtRest` in `dispatch` and this test
    /// fails on the `Stop` case. Watched under exactly that addition.
    #[test]
    fn a_directive_outside_the_drawn_vocabulary_refuses_out_of_order() {
        let mut residency = Residency::new();
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
                dispatch(&mut residency, &directive(case.clone())),
                Payload::Refusal(LifecycleRefusal::OutOfOrder),
                "{case:?} is outside this seam's vocabulary"
            );
        }
    }

    /// A payload that is not a directive is the peer speaking out of turn.
    #[test]
    fn a_non_directive_payload_refuses_out_of_order() {
        let mut residency = Residency::new();
        let envelope = OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: 1,
            },
            position: Position::Open,
            payload: Payload::Answer(LifecycleAnswer::Ready),
        };
        assert_eq!(
            dispatch(&mut residency, &envelope),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );
    }
}
