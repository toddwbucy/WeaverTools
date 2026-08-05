//! conforms: harness-out-of-order-refused
//! conforms: harness-scoped-refusal-account
//!
//! The service tests of `weaver-harness-Spec` section 8: the out-of-order
//! refusal with its two watches and the scoped refusal account. The
//! announce-after-record discipline needs an entered run, which needs organ
//! doubles, so it is tested from inside the crate in `lifecycle.rs` where the
//! run can be built directly.
#![cfg(target_os = "linux")]

use std::os::fd::{AsFd, AsRawFd};

use weaver_harness::{Harness, OrganBinaries, OrganChannel, Outcome};
use weaver_types::{
    ExchangeId, LifecycleAnswer, LifecycleDirective, LifecycleRefusal, Opener, OrganEnvelope,
    Payload, Position,
};

/// Stands a coordination peer up against a harness serving on a thread, so a
/// test can drive directives and read answers. The harness's own fork sites
/// are not reached by these directives, which is what keeps the service
/// testable without organ binaries.
///
/// **`adopt` clears the dumpable flag for this whole test process**, since
/// `prctl(PR_SET_DUMPABLE, 0)` is process-wide and this stands the harness up
/// on a thread rather than in a forked child. The first test to run therefore
/// disables core dumps and same-uid `ptrace` for this binary and every test
/// after it. A maintainer who cannot attach a debugger to these tests is owed
/// the reason, and the containment a forked child would buy is not taken
/// because the peer must outlive the harness to drive it.
struct Peer {
    channel: OrganChannel,
}

impl Peer {
    fn stand_up() -> (
        Peer,
        std::thread::JoinHandle<Result<Outcome, weaver_harness::ChannelFault>>,
    ) {
        let (harness_end, peer_end) = OrganChannel::pair().expect("pair");
        let peer = Peer {
            channel: peer_end.into_channel(),
        };
        // SAFETY: the harness owns its end for the thread's lifetime.
        let fd = harness_end.into_fd();
        let handle = std::thread::spawn(move || {
            let harness = Harness::adopt(
                fd,
                OrganBinaries {
                    spu: "/nonexistent/spu".into(),
                    gate: "/nonexistent/gate".into(),
                },
            )
            .expect("adopt");
            harness.serve()
        });
        (peer, handle)
    }

    fn send(&self, ordinal: u64, directive: LifecycleDirective) {
        self.channel
            .send(&OrganEnvelope {
                exchange: ExchangeId {
                    opener: Opener::Admin,
                    ordinal,
                },
                position: Position::Open,
                payload: Payload::Directive(directive),
            })
            .expect("directive sent");
    }

    fn read(&self) -> Payload {
        self.channel.recv().expect("answer received").payload
    }
}

/// **A directive out of order is refused and not queued.** A leave arriving
/// before any enter answers `OutOfOrder` and reaches no unwind.
///
/// Perturbation one: make the before-enter arm fall through to the unwind and
/// the leave reaches it, ending service where a refusal was owed. Watched
/// under exactly that change.
#[test]
fn leave_before_enter_is_refused() {
    let (peer, handle) = Peer::stand_up();
    peer.send(1, LifecycleDirective::Leave);
    match peer.read() {
        Payload::Refusal(LifecycleRefusal::OutOfOrder) => {}
        other => panic!("an early leave must answer OutOfOrder, got {other:?}"),
    }
    // Service continues: the refusal was not a fault and the channel stands.
    peer.send(2, LifecycleDirective::Stop);
    assert!(matches!(
        peer.read(),
        Payload::Refusal(LifecycleRefusal::OutOfOrder)
    ));
    drop(peer);
    let _ = handle.join().expect("thread");
}

/// A stop arriving before any enter is out of order for the position, which
/// is the before-enter arm rather than the at-rest answer.
///
/// **The at-rest close and the terminal position are not this test's**, and
/// saying so beats a name that overstates: `AtRest` needs an entered run and
/// is tested in `lifecycle.rs`, and the terminal arm needs a completed leave,
/// which needs organ doubles this suite does not buy.
#[test]
fn stop_before_enter_is_out_of_order() {
    let (peer, handle) = Peer::stand_up();
    // A stop before any enter is out of order for the position, which is the
    // before-enter arm rather than the at-rest answer.
    peer.send(1, LifecycleDirective::Stop);
    assert!(
        matches!(peer.read(), Payload::Refusal(LifecycleRefusal::OutOfOrder)),
        "a stop before enter is out of order, not an at-rest close"
    );
    drop(peer);
    let _ = handle.join().expect("thread");
}

/// **The scoped refusal account:** a refusal before the `load` event leaves the
/// stream clean and the state at before-enter, so the channel still serves and
/// a later directive is judged against that position rather than a half-stood
/// one.
///
/// Perturbation: author the load event before the sink is received and a
/// refused enter leaves a bracket on a stream nothing will close. Watched by
/// moving the authoring point ahead of the receive.
#[test]
fn refused_enter_leaves_the_state_at_before_enter() {
    let (peer, handle) = Peer::stand_up();
    // An enter with no ancillary sink descriptor cannot construct a recorder,
    // so it refuses before anything is authored.
    peer.send(
        1,
        LifecycleDirective::Enter {
            payload: weaver_types::EnterPayload {
                session: weaver_types::SessionId("s-1".into()),
                run_ordinal: 0,
                model_binding: weaver_types::ModelBinding {
                    artifact: weaver_types::ArtifactRef("qwen".into()),
                    devices: vec![weaver_types::DeviceOrdinal(0)],
                },
                gate_instruction: weaver_types::GateInstruction {
                    socket_path: "/run/weaver/gate.sock".into(),
                    access_rule: weaver_types::AccessRule {
                        allowed_uids: Default::default(),
                        allowed_gids: Default::default(),
                        denied_uids: Default::default(),
                    },
                },
            },
        },
    );
    match peer.read() {
        Payload::Refusal(LifecycleRefusal::DescriptorsUnusable) => {}
        other => panic!("an enter with no sink refuses, got {other:?}"),
    }
    // The position held: a leave is still out of order, which it would not be
    // had the refused enter left the state entered.
    peer.send(2, LifecycleDirective::Leave);
    assert!(
        matches!(peer.read(), Payload::Refusal(LifecycleRefusal::OutOfOrder)),
        "the state stayed at before-enter"
    );
    drop(peer);
    let _ = handle.join().expect("thread");
}

/// Closure is observed as death and never synthesized into an answer: when the
/// peer drops its end, service ends with `ChannelClosed` rather than a
/// refusal.
#[test]
fn closure_ends_service_as_an_outcome() {
    let (peer, handle) = Peer::stand_up();
    drop(peer);
    let outcome = handle.join().expect("thread").expect("no fault");
    assert_eq!(outcome, Outcome::ChannelClosed);
}

/// A message that is not a directive cannot be attributed to an exchange for a
/// refusal to answer, so it is a fault below the exchange layer.
#[test]
fn non_directive_payload_is_a_fault() {
    let (peer, handle) = Peer::stand_up();
    peer.channel
        .send(&OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Admin,
                ordinal: 1,
            },
            position: Position::Open,
            payload: Payload::Answer(LifecycleAnswer::Ready),
        })
        .expect("sent");
    let result = handle.join().expect("thread");
    assert!(
        matches!(result, Err(weaver_harness::ChannelFault::Undecodable)),
        "an answer on the coordination channel is below the exchange layer, got {result:?}"
    );
    drop(peer);
}

/// The adopted end is owned: dropping the `Harness` closes it, which is the
/// close being a type property rather than an integer left behind.
///
/// The previous version of this test asserted that a descriptor number was
/// above 2, which the kernel guarantees for any process with the standard
/// streams open - it never constructed a `Harness` and could not fail. This
/// one adopts, drops, and observes the descriptor is gone. Run in a forked
/// child because `adopt` clears the dumpable flag process-wide.
#[test]
fn dropping_the_harness_closes_the_adopted_end() {
    let (report_r, report_w) = nix::unistd::pipe().expect("pipe");
    // SAFETY: the child adopts, drops, probes, reports, and _exits.
    match unsafe { nix::unistd::fork() }.expect("fork") {
        nix::unistd::ForkResult::Child => {
            let (r, _w) = nix::unistd::pipe().expect("pipe");
            let raw = r.as_raw_fd();
            let harness = Harness::adopt(
                r,
                OrganBinaries {
                    spu: "/nonexistent/spu".into(),
                    gate: "/nonexistent/gate".into(),
                },
            )
            .expect("adopt");
            drop(harness);
            let flags = unsafe { nix::libc::fcntl(raw, nix::libc::F_GETFD) };
            let closed = (flags == -1) as u8;
            let _ = nix::unistd::write(report_w.as_fd(), &[closed]);
            unsafe { nix::libc::_exit(0) };
        }
        nix::unistd::ForkResult::Parent { child } => {
            drop(report_w);
            let mut byte = [0u8; 1];
            let _ = nix::unistd::read(report_r.as_fd(), &mut byte);
            let _ = nix::sys::wait::waitpid(child, None);
            assert_eq!(byte[0], 1, "the adopted end is closed when its owner drops");
        }
    }
}
