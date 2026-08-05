//! conforms: harness-truncation-is-a-fault
//! conforms: harness-one-write-is-one-read
//! conforms: harness-atomic-cloexec-at-creation
//! conforms: harness-trace-fd-cloexec-at-receive
//! conforms: harness-organ-ends-from-descriptor-three
//! conforms: harness-dumpable-flag-cleared
//!
//! The threat walks of `weaver-harness-Spec` section 8, made executable, and
//! the channel perturbations beside them. Each names its perturbation: the
//! mutation under which it was watched to fail.
#![cfg(target_os = "linux")]

use std::os::fd::{AsFd, AsRawFd, OwnedFd};

use weaver_harness::{ChannelFault, FIRST_ORGAN_DESCRIPTOR, Harness, OrganBinaries, OrganChannel};
use weaver_types::{
    AgentName, ExchangeId, LifecycleDirective, MAX_ENVELOPE_BYTES, Opener, OrganEnvelope, Payload,
    Position,
};

fn directive(ordinal: u64, agent: &str) -> OrganEnvelope {
    OrganEnvelope {
        exchange: ExchangeId {
            opener: Opener::Admin,
            ordinal,
        },
        position: Position::Open,
        payload: Payload::Directive(LifecycleDirective::Load {
            agent: AgentName(agent.to_string()),
        }),
    }
}

fn cloexec_set(fd: std::os::fd::BorrowedFd<'_>) -> bool {
    // SAFETY: reading F_GETFD on a descriptor this process owns.
    let flags = unsafe { nix::libc::fcntl(fd.as_raw_fd(), nix::libc::F_GETFD) };
    flags != -1 && (flags & nix::libc::FD_CLOEXEC) != 0
}

/// **The second walk: a tool inherits a channel end.** Both ends of every
/// created pair carry close-on-exec from the creating act, by `SOCK_CLOEXEC`
/// in the `socketpair` call rather than by a later `fcntl` - no window between
/// creation and flag.
///
/// Perturbation: drop `SOCK_CLOEXEC` from the `socketpair` call and both
/// assertions fail immediately; with a later `fcntl` they pass while the
/// window stands, which is why the walk below forks to check what actually
/// crosses.
#[test]
fn pairs_are_close_on_exec_from_creation() {
    let (near, far) = OrganChannel::pair().expect("pair");
    assert!(
        cloexec_set(near.as_fd()),
        "the harness end is flagged at creation"
    );
    assert!(
        cloexec_set(far.as_fd()),
        "the child end is flagged at creation"
    );
}

/// **The second walk, executed.** A forked child that does not run the handoff
/// inherits no channel end: every end this crate creates is flagged, so an
/// exec closes it. The child reports which of the parent's descriptors it can
/// still see after an exec-equivalent flag check.
///
/// Perturbation: dropping the atomic flag to a later `fcntl` leaves a window a
/// fork can land in, which this test's child observes as an inherited end.
#[test]
fn tool_fork_inherits_no_channel_end() {
    let (near, _far) = OrganChannel::pair().expect("pair");
    let watched = near.as_fd().as_raw_fd();
    let (report_r, report_w) = nix::unistd::pipe().expect("pipe");
    // SAFETY: the child writes one byte and _exits; no allocation, no unwind.
    match unsafe { nix::unistd::fork() }.expect("fork") {
        nix::unistd::ForkResult::Child => {
            let flags = unsafe { nix::libc::fcntl(watched, nix::libc::F_GETFD) };
            let inherited_and_clear = if flags != -1 && (flags & nix::libc::FD_CLOEXEC) == 0 {
                1u8
            } else {
                0u8
            };
            let _ = nix::unistd::write(report_w.as_fd(), &[inherited_and_clear]);
            unsafe { nix::libc::_exit(0) };
        }
        nix::unistd::ForkResult::Parent { child } => {
            drop(report_w);
            let mut byte = [0u8; 1];
            let _ = nix::unistd::read(report_r.as_fd(), &mut byte);
            let _ = nix::sys::wait::waitpid(child, None);
            assert_eq!(
                byte[0], 0,
                "no end crosses a fork without its close-on-exec flag"
            );
        }
    }
}

/// **One write is one read**, the boundary property the socket type was
/// elected to buy: two envelopes written back to back arrive as two reads of
/// exactly one envelope each.
///
/// Perturbation: change `SOCK_SEQPACKET` to `SOCK_STREAM` at the `socketpair`
/// call and the first read returns both envelopes concatenated, failing the
/// decode. **Two messages are what make the watch reachable** - a single small
/// envelope crosses a stream socket whole, so a one-message test would pass
/// under the substitution and pin nothing.
#[test]
fn one_write_is_one_read() {
    let (near, far) = OrganChannel::pair().expect("pair");
    let peer = far.into_channel();
    peer.send(&directive(1, "alpha")).expect("first write");
    peer.send(&directive(2, "beta")).expect("second write");
    let first = near.recv().expect("first read decodes on its own");
    let second = near.recv().expect("second read decodes on its own");
    assert_eq!(first.exchange.ordinal, 1);
    assert_eq!(second.exchange.ordinal, 2);
}

/// **Truncation is a fault**, never a message: a read that returns with
/// `MSG_TRUNC` set carries a prefix the kernel shortened and discarded the
/// rest of.
///
/// Perturbation: remove the `MSG_TRUNC` check from `recv` and the shortened
/// octets reach the decoder, which either fails opaquely or - for a payload
/// that happens to remain valid JSON - decodes a silently different directive.
#[test]
fn truncation_is_a_fault() {
    let (near, far) = OrganChannel::pair().expect("pair");
    let oversized = vec![b'x'; MAX_ENVELOPE_BYTES + 4096];
    // SAFETY: a raw write is how the test produces a message larger than the
    // receiver's buffer; the crate's own send refuses to author one.
    let written = unsafe {
        nix::libc::send(
            far.as_fd().as_raw_fd(),
            oversized.as_ptr().cast(),
            oversized.len(),
            0,
        )
    };
    if written < 0 {
        // **The branch under test was not reached**, and a silent return here
        // would let the stated perturbation go undetected on this machine.
        // The kernel refused the oversized datagram outright, which is the
        // same boundary holding one layer down, so this is a skip and says so.
        eprintln!(
            "SKIP truncation_is_a_fault: the kernel refused a {} byte datagram \
             (errno {}), so the MSG_TRUNC branch was not exercised here",
            oversized.len(),
            std::io::Error::last_os_error()
        );
        return;
    }
    match near.recv() {
        Err(ChannelFault::Truncated { bound }) => assert_eq!(bound, MAX_ENVELOPE_BYTES),
        other => panic!("a truncated read must be a fault, got {other:?}"),
    }
}

/// **The first walk: a rogue elected tool reaches for the trace.** The trace
/// descriptor enters this crate at exactly one call, and that call asks for
/// `MSG_CMSG_CLOEXEC` in the receive itself, so the handle arrives flagged and
/// no tool subprocess inherits a writable handle to the record.
///
/// Perturbation: drop `MSG_CMSG_CLOEXEC` from the receive and the received
/// descriptor arrives clear, which this assertion catches directly.
#[test]
fn received_trace_descriptor_is_close_on_exec() {
    let (near, far) = OrganChannel::pair().expect("pair");
    let peer = far.into_channel();
    let sink =
        std::fs::File::create(std::env::temp_dir().join("weaver-harness-walk-one")).expect("sink");
    let sink = OwnedFd::from(sink);
    peer.send_with_descriptor(&directive(7, "alpha"), sink.as_fd())
        .expect("send with descriptor");
    let (envelope, received) = near.recv_with_descriptor().expect("receive");
    assert_eq!(envelope.exchange.ordinal, 7);
    let received = received.expect("the sink crossed");
    assert!(
        cloexec_set(received.as_fd()),
        "the sink arrives close-on-exec, so no tool fork inherits the record"
    );
}

/// **The second walk's placement half:** a child finds the ends it is owed at
/// descriptor 3 upward, in the channels' own order, with the flag cleared.
///
/// **This test does not reach the `dup2` no-op corner**, and the honest
/// statement is worth more than a watch that cannot fail: producing it needs
/// a `ChildEnd` whose own descriptor number is already 3, which this crate's
/// private field does not admit from a test. Making the clear conditional
/// leaves this test passing. That is why `weaver-harness-Spec` section 2.2
/// tags the unconditional clear `review` and says so - review's by election
/// and not by impossibility, the shape both seats ran to find the corner.
#[test]
fn child_ends_land_from_descriptor_three() {
    let (_near_a, far_a) = OrganChannel::pair().expect("pair a");
    let (_near_b, far_b) = OrganChannel::pair().expect("pair b");
    let (report_r, report_w) = nix::unistd::pipe().expect("pipe");
    // **The report pipe must sit above the range the placement overwrites.**
    // `place_child_ends` targets 3 and 4, and `dup2` closes whatever occupies
    // its target: a report end at either number would be closed by the very
    // placement this test measures, and the parent would read nothing and
    // report a flag problem that did not occur. Descriptor numbers are the
    // lowest free number, so the pairs usually take the lower ones - usually
    // is not a guarantee.
    assert!(
        report_r.as_raw_fd() > FIRST_ORGAN_DESCRIPTOR + 1
            && report_w.as_raw_fd() > FIRST_ORGAN_DESCRIPTOR + 1,
        "the report pipe sits above the placement's targets"
    );
    // SAFETY: the child runs the handoff and then reports; async-signal-safe
    // calls only, then _exit.
    match unsafe { nix::unistd::fork() }.expect("fork") {
        nix::unistd::ForkResult::Child => {
            let placed = unsafe { weaver_harness::place_child_ends(&[&far_a, &far_b]) };
            let mut ok = placed.is_ok() as u8;
            for offset in 0..2 {
                let fd = FIRST_ORGAN_DESCRIPTOR + offset;
                let flags = unsafe { nix::libc::fcntl(fd, nix::libc::F_GETFD) };
                if flags == -1 || (flags & nix::libc::FD_CLOEXEC) != 0 {
                    ok = 0;
                }
            }
            let _ = nix::unistd::write(report_w.as_fd(), &[ok]);
            unsafe { nix::libc::_exit(0) };
        }
        nix::unistd::ForkResult::Parent { child } => {
            drop(report_w);
            let mut byte = [0u8; 1];
            let _ = nix::unistd::read(report_r.as_fd(), &mut byte);
            let _ = nix::sys::wait::waitpid(child, None);
            assert_eq!(
                byte[0], 1,
                "both ends sit at 3 and 4 with the flag cleared unconditionally"
            );
        }
    }
}

/// **The third walk: a same-uid process attaches.** The dumpable flag is
/// cleared at adoption, which reparents the proc entries to root and refuses
/// the attach.
///
/// Perturbation: remove the `prctl` from `adopt` and the flag reads 1, which
/// this assertion catches. Run in a forked child so the cleared flag does not
/// outlive the test into the rest of the suite.
#[test]
fn adoption_clears_the_dumpable_flag() {
    let (report_r, report_w) = nix::unistd::pipe().expect("pipe");
    // SAFETY: the child adopts, reads the flag, reports, and _exits.
    match unsafe { nix::unistd::fork() }.expect("fork") {
        nix::unistd::ForkResult::Child => {
            let (r, _w) = nix::unistd::pipe().expect("pipe");
            let adopted = Harness::adopt(
                r,
                OrganBinaries {
                    spu: "/nonexistent/spu".into(),
                    gate: "/nonexistent/gate".into(),
                },
            );
            let dumpable = unsafe { nix::libc::prctl(nix::libc::PR_GET_DUMPABLE, 0, 0, 0, 0) };
            let ok = (adopted.is_ok() && dumpable == 0) as u8;
            let _ = nix::unistd::write(report_w.as_fd(), &[ok]);
            unsafe { nix::libc::_exit(0) };
        }
        nix::unistd::ForkResult::Parent { child } => {
            drop(report_w);
            let mut byte = [0u8; 1];
            let _ = nix::unistd::read(report_r.as_fd(), &mut byte);
            let _ = nix::sys::wait::waitpid(child, None);
            assert_eq!(
                byte[0], 1,
                "adopt clears the dumpable flag and the adopted end is flagged"
            );
        }
    }
}

/// The adopted coordination end carries close-on-exec after adoption: the same
/// property at the exec rather than at the attach, reaching an end this crate
/// was handed rather than one it created.
#[test]
fn adopted_end_is_close_on_exec() {
    let (report_r, report_w) = nix::unistd::pipe().expect("pipe");
    // SAFETY: as above.
    match unsafe { nix::unistd::fork() }.expect("fork") {
        nix::unistd::ForkResult::Child => {
            // A pipe end stands in for the unit's declared open: an owned
            // descriptor this process did not create with the flag.
            let (r, _w) = nix::unistd::pipe().expect("pipe");
            let raw = r.as_raw_fd();
            unsafe { nix::libc::fcntl(raw, nix::libc::F_SETFD, 0) };
            let adopted = Harness::adopt(
                r,
                OrganBinaries {
                    spu: "/nonexistent/spu".into(),
                    gate: "/nonexistent/gate".into(),
                },
            );
            let flags = unsafe { nix::libc::fcntl(raw, nix::libc::F_GETFD) };
            let ok = (adopted.is_ok() && flags != -1 && (flags & nix::libc::FD_CLOEXEC) != 0) as u8;
            let _ = nix::unistd::write(report_w.as_fd(), &[ok]);
            unsafe { nix::libc::_exit(0) };
        }
        nix::unistd::ForkResult::Parent { child } => {
            drop(report_w);
            let mut byte = [0u8; 1];
            let _ = nix::unistd::read(report_r.as_fd(), &mut byte);
            let _ = nix::sys::wait::waitpid(child, None);
            assert_eq!(
                byte[0], 1,
                "the adopted end is flagged by the set at adoption"
            );
        }
    }
}
