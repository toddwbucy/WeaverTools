//! conforms: spu-service-serial-one-loop
//! conforms: spu-out-of-order-refused-on-residency
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
    let mut position = SeamPosition::BeforeAdmit;

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

        let payload = dispatch(&mut position, &mut residency, &envelope);
        if answer(&lifecycle, envelope.exchange, payload).is_err() {
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
        (SeamPosition::BeforeAdmit, LifecycleDirective::Admit { binding }) => {
            match residency.admit(binding, Headroom(HEADROOM_BYTES)) {
                Ok(_) => {
                    *position = SeamPosition::Admitted;
                    Payload::Answer(LifecycleAnswer::Admitted)
                }
                Err(refusal) => {
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
    use weaver_types::{AgentName, ArtifactRef, DeviceOrdinal, ModelBinding};

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
            dispatch(&mut position, &mut residency, &directive(LifecycleDirective::Release)),
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
            LifecycleDirective::Admit { binding: binding() },
            LifecycleDirective::Release,
            LifecycleDirective::List,
        ] {
            assert_eq!(
                dispatch(&mut position, &mut residency, &directive(case.clone())),
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
            &directive(LifecycleDirective::Admit { binding: binding() }),
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
            &directive(LifecycleDirective::Admit { binding: binding() }),
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
                dispatch(&mut position, &mut residency, &directive(case.clone())),
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

        let mut closing = directive(LifecycleDirective::Admit { binding: binding() });
        closing.position = Position::Close;
        assert_eq!(
            dispatch(&mut position, &mut residency, &closing),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );

        let mut wrong_opener = directive(LifecycleDirective::Admit { binding: binding() });
        wrong_opener.exchange.opener = Opener::Spu;
        assert_eq!(
            dispatch(&mut position, &mut residency, &wrong_opener),
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
            dispatch(&mut position, &mut residency, &envelope),
            Payload::Refusal(LifecycleRefusal::OutOfOrder)
        );
    }
}
