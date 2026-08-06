//! conforms: spu-out-of-order-refused-on-residency
//!
//! The lifecycle seam exercised against `weaver-harness-spu-contract`'s failure
//! cases, per gate H3: a contract that names a refusal and a build that cannot
//! produce it has not implemented the contract.
//!
//! These drive the real binary over a real socket pair, so what is exercised is
//! the envelope crossing the seam rather than a function called in-process. The
//! contract's ordering clauses are what they read:
//!
//! - admit is first and happens exactly once on a channel
//! - release is last, happens at most once, and is terminal
//! - a release with no completed admit before it is refused and is not queued
//! - a directive that arrives out of that order is refused and is not queued
//!
//! **What is not exercised here is the success path**, because no build of this
//! crate can admit yet: the backends of Spec section 4 are not written. That is
//! a named gap rather than an oversight, and it closes when they land.

use std::os::fd::{AsRawFd, OwnedFd, RawFd};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Stdio};

use nix::libc;
use nix::sys::socket::{AddressFamily, MsgFlags, SockFlag, SockType, recv, send, socketpair};
use weaver_types::{
    ArtifactRef, DeviceOrdinal, ExchangeId, LifecycleDirective, LifecycleRefusal,
    ModelBinding, Opener, OrganEnvelope, Payload, Position,
};

fn seqpacket_pair() -> (OwnedFd, OwnedFd) {
    socketpair(
        AddressFamily::Unix,
        SockType::SeqPacket,
        None,
        SockFlag::SOCK_CLOEXEC,
    )
    .expect("a socketpair")
}

/// Start the binary with the two ends at 3 and 4, in two passes so a source
/// already sitting on a target number is not clobbered before it is placed.
fn spawn(child_ends: Vec<RawFd>) -> Child {
    let mut command = Command::new(env!("CARGO_BIN_EXE_weaver-spu"));
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    unsafe {
        command.pre_exec(move || {
            let mut lifted = Vec::with_capacity(child_ends.len());
            for source in &child_ends {
                let high = libc::fcntl(*source, libc::F_DUPFD, 32);
                if high < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                lifted.push(high);
            }
            for (offset, high) in lifted.iter().enumerate() {
                if libc::dup2(*high, 3 + offset as RawFd) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            // F_DUPFD clears close-on-exec, so these would survive and inflate
            // the count the child checks. Closing them is required.
            for high in lifted {
                libc::close(high);
            }
            Ok(())
        });
    }
    command.spawn().expect("the binary starts")
}

/// A harness end: send one directive, read one answer.
struct Harness {
    end: OwnedFd,
    ordinal: u64,
}

impl Harness {
    fn ask(&mut self, directive: LifecycleDirective) -> OrganEnvelope {
        self.ordinal += 1;
        let envelope = OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: self.ordinal,
            },
            position: Position::Open,
            payload: Payload::Directive(directive),
        };
        let body = serde_json::to_vec(&envelope).expect("an envelope encodes");
        send(self.end.as_raw_fd(), &body, MsgFlags::empty()).expect("the directive is sent");

        let mut buffer = vec![0u8; 64 * 1024];
        let read = recv(self.end.as_raw_fd(), &mut buffer, MsgFlags::empty())
            .expect("an answer arrives");
        buffer.truncate(read);
        serde_json::from_slice(&buffer).expect("the answer is an envelope")
    }
}

fn started() -> (Harness, Child) {
    let (parent, child_lifecycle) = seqpacket_pair();
    let (_decode, child_decode) = seqpacket_pair();
    let child = spawn(vec![child_lifecycle.as_raw_fd(), child_decode.as_raw_fd()]);
    (
        Harness {
            end: parent,
            ordinal: 0,
        },
        child,
    )
}

fn binding() -> ModelBinding {
    ModelBinding {
        artifact: ArtifactRef("/nonexistent/artifact".into()),
        devices: vec![DeviceOrdinal(0)],
    }
}

/// **A release before any admit answers `OutOfOrder` and is not queued,** per
/// Spec sections 9 and 10: the order is judged against the seam's recorded
/// position before the directive reaches residency, and the refusal returns on
/// the exchange that asked rather than the directive being held.
///
/// Perturbation: route the before-admit release through `Residency::release`
/// and this test fails, because the answer becomes `NoResidency`, which is
/// residency's account rather than the position's. Watched under exactly that
/// change.
#[test]
fn a_release_before_any_admit_answers_out_of_order_on_the_seam() {
    let (mut harness, mut child) = started();
    let answer = harness.ask(LifecycleDirective::Release);
    assert_eq!(
        answer.payload,
        Payload::Refusal(LifecycleRefusal::OutOfOrder),
        "a release before any admit is out of order for the seam's position"
    );
    assert_eq!(answer.exchange.ordinal, 1, "on the exchange that asked");
    drop(harness);
    let _ = child.wait();
}

/// **A directive outside this seam's drawn vocabulary is refused and is not
/// queued.** The contract draws admit and release, so the floor's other cases
/// refuse as out of order.
#[test]
fn a_directive_outside_the_vocabulary_is_refused_on_the_seam() {
    let (mut harness, mut child) = started();
    for directive in [
        LifecycleDirective::Leave,
        LifecycleDirective::Stop,
        LifecycleDirective::List,
    ] {
        let answer = harness.ask(directive.clone());
        assert_eq!(
            answer.payload,
            Payload::Refusal(LifecycleRefusal::OutOfOrder),
            "{directive:?} is outside this seam's vocabulary"
        );
    }
    drop(harness);
    let _ = child.wait();
}

/// **Admit happens exactly once on a channel.** A second admit is refused on
/// the ordering rather than re-running the steps, because this crate begins
/// empty, admits once, and dies.
///
/// The first admit refuses here too, on the artifact, since no build can yet
/// take a device. What this reads is that the two refusals differ: the first
/// names the artifact and the second names the order, which is what shows the
/// second one never re-ran step one.
///
/// Perturbation: remove the `admit_attempted` guard from `Residency::admit` and
/// this test fails, because the second admit returns the artifact refusal
/// again. Watched under exactly that removal.
#[test]
fn a_second_admit_is_refused_on_the_ordering_across_the_seam() {
    let (mut harness, mut child) = started();

    let first = harness.ask(LifecycleDirective::Admit { binding: binding() });
    assert_eq!(
        first.payload,
        Payload::Refusal(LifecycleRefusal::ArtifactUnresolvable),
        "the first admit refuses at step one, on the artifact"
    );

    let second = harness.ask(LifecycleDirective::Admit { binding: binding() });
    assert_eq!(
        second.payload,
        Payload::Refusal(LifecycleRefusal::OutOfOrder),
        "the second admit refuses on the ordering, never re-running step one"
    );

    drop(harness);
    let _ = child.wait();
}

/// One directive receives exactly one answer, on the exchange that asked it.
/// The serial property of Spec section 2 read from the outside.
#[test]
fn one_directive_receives_one_answer_on_its_own_exchange() {
    let (mut harness, mut child) = started();
    for expected in 1..=3u64 {
        let answer = harness.ask(LifecycleDirective::List);
        assert_eq!(answer.exchange.ordinal, expected);
        assert_eq!(answer.exchange.opener, Opener::Harness);
        assert_eq!(answer.position, Position::Close);
        assert!(matches!(answer.payload, Payload::Refusal(_)));
    }
    drop(harness);
    let _ = child.wait();
}
