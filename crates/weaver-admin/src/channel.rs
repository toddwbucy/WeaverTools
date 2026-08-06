//! conforms: admin-one-write-is-one-read
//! conforms: admin-dial-retries-within-a-bound
//! conforms: admin-truncation-is-a-channel-fault
//! conforms: admin-enter-carries-descriptor-in-one-message
//!
//! The coordination channel, per `weaver-admin-Spec` section 7: reached in one
//! act, the dial, `SOCK_SEQPACKET`, carrying the election of
//! `weaver-types-Spec` section 4 rather than re-deciding it.
//!
//! **The channel is dialed and never bound.** Any socket connecting to the
//! harness is an internal connection, so the harness binds inside the agent's
//! sandbox and this crate connects, per `weaver-admin-harness-contract`
//! section 2. What refuses a stranger is the credential the harness reads at
//! its accept, not anything here.
//!
//! **The dial retries within a bound.** The load starts the unit and then
//! dials, and the bind is the worker's first act, so the race is structural.
//! A bound exceeded refuses rather than waiting without end.

use std::os::fd::{AsRawFd, BorrowedFd, OwnedFd};
use std::path::Path;
use std::time::{Duration, Instant};

use nix::sys::socket::{
    AddressFamily, ControlMessage, MsgFlags, SockFlag, SockType, UnixAddr, connect, sendmsg, socket,
};
use weaver_types::{MAX_ENVELOPE_BYTES, OrganEnvelope};

/// A fault below the exchange layer on the coordination channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelFault {
    Truncated {
        bound: usize,
    },
    Undecodable,
    Closed,
    /// The dial found no listener within the bound. The worker never bound its
    /// socket, and section 7's ceiling is what turns that into an answer
    /// rather than a wait.
    NotDialable,
}

/// The connected end this invocation holds for the life of one verb.
#[derive(Debug)]
pub struct Coordination {
    fd: OwnedFd,
    ordinal: u64,
}

/// The dial's ceiling and its interval, this Spec's election. One second of
/// attempts at ten milliseconds, per section 7: the bind is the worker's first
/// act and the load's dial may arrive first, so a bounded retry covers the
/// race, and the ceiling is what keeps a worker that never binds from hanging
/// the operator's terminal.
const DIAL_CEILING: Duration = Duration::from_secs(1);
const DIAL_INTERVAL: Duration = Duration::from_millis(10);

/// Connects to the socket the worker bound, retrying within the bound above.
///
/// Close-on-exec is asked for in the socket call itself, so no window exists
/// between creation and flag. The connection is this invocation's and closes
/// when the verb answers, there being no standing end to keep.
pub fn dial(socket_path: &Path) -> Result<Coordination, ChannelFault> {
    let deadline = Instant::now() + DIAL_CEILING;
    loop {
        match dial_once(socket_path) {
            Ok(connection) => return Ok(connection),
            Err(fault) => {
                if Instant::now() >= deadline {
                    return Err(fault);
                }
                std::thread::sleep(DIAL_INTERVAL);
            }
        }
    }
}

fn dial_once(socket_path: &Path) -> Result<Coordination, ChannelFault> {
    let fd = socket(
        AddressFamily::Unix,
        SockType::SeqPacket,
        SockFlag::SOCK_CLOEXEC,
        None,
    )
    .map_err(|_| ChannelFault::NotDialable)?;
    let address = UnixAddr::new(socket_path).map_err(|_| ChannelFault::NotDialable)?;
    connect(fd.as_raw_fd(), &address).map_err(|_| ChannelFault::NotDialable)?;
    Ok(Coordination { fd, ordinal: 0 })
}

impl Coordination {
    /// The next exchange identity: ordinals assigned serially by this crate,
    /// per `weaver-organ-channel` section 1.
    pub fn next_ordinal(&mut self) -> u64 {
        self.ordinal += 1;
        self.ordinal
    }

    /// Writes one envelope as one message, asserting the same bound the
    /// receiver holds, because a bound only the receiver holds is a bound the
    /// sender discovers in production.
    /// Sends one directive on this channel, opening its exchange.
    pub fn send_directive(
        &self,
        ordinal: u64,
        directive: weaver_types::LifecycleDirective,
    ) -> Result<(), ChannelFault> {
        self.send(&OrganEnvelope {
            exchange: weaver_types::ExchangeId {
                opener: weaver_types::Opener::Admin,
                ordinal,
            },
            position: weaver_types::Position::Open,
            payload: weaver_types::Payload::Directive(directive),
        })
    }

    pub fn send(&self, envelope: &OrganEnvelope) -> Result<(), ChannelFault> {
        let body = serde_json::to_vec(envelope).map_err(|_| ChannelFault::Undecodable)?;
        if body.len() > MAX_ENVELOPE_BYTES {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        let slices = [std::io::IoSlice::new(&body)];
        sendmsg::<()>(self.fd.as_raw_fd(), &slices, &[], MsgFlags::empty(), None)
            .map_err(|_| ChannelFault::Closed)?;
        Ok(())
    }

    /// **The enter directive and its ancillary payload are one message.** The
    /// envelope is rendered and sent with the sink's descriptor as
    /// `SCM_RIGHTS` control data on the same `sendmsg`, which is what makes
    /// the descriptor cross once, in the enter exchange, with no separate
    /// delivery to order against anything.
    pub fn send_with_sink(
        &self,
        envelope: &OrganEnvelope,
        sink: BorrowedFd<'_>,
    ) -> Result<(), ChannelFault> {
        let body = serde_json::to_vec(envelope).map_err(|_| ChannelFault::Undecodable)?;
        if body.len() > MAX_ENVELOPE_BYTES {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        let fds = [sink.as_raw_fd()];
        let control = [ControlMessage::ScmRights(&fds)];
        let slices = [std::io::IoSlice::new(&body)];
        sendmsg::<()>(
            self.fd.as_raw_fd(),
            &slices,
            &control,
            MsgFlags::empty(),
            None,
        )
        .map_err(|_| ChannelFault::Closed)?;
        Ok(())
    }

    /// Reads one envelope. The buffer is sized to the envelope bound, and a
    /// read returning `MSG_TRUNC` **is a channel fault and never a message**:
    /// the kernel returns the truncated prefix with the flag set and discards
    /// the remainder, so an unchecked flag turns a long answer into a silently
    /// shortened one.
    pub fn recv(&self) -> Result<OrganEnvelope, ChannelFault> {
        let mut buffer = vec![0u8; MAX_ENVELOPE_BYTES];
        let mut slices = [std::io::IoSliceMut::new(&mut buffer)];
        let message = nix::sys::socket::recvmsg::<()>(
            self.fd.as_raw_fd(),
            &mut slices,
            None,
            MsgFlags::empty(),
        )
        .map_err(|_| ChannelFault::Closed)?;
        if message.flags.contains(MsgFlags::MSG_TRUNC) {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        let read = message.bytes;
        if read == 0 {
            return Err(ChannelFault::Closed);
        }
        serde_json::from_slice(&buffer[..read]).map_err(|_| ChannelFault::Undecodable)
    }

    /// Adopts a connected end, for a test peer standing in for the worker.
    #[cfg(test)]
    pub fn adopt(fd: OwnedFd) -> Self {
        Coordination { fd, ordinal: 0 }
    }
}

#[cfg(test)]
mod tests {
    //! The dial's bound and the channel's framing, run from inside the crate
    //! because this crate publishes no library surface.
    //!
    //! **The harness is stood in for here.** Under the inversion the worker
    //! binds and this crate dials, so every test below plays the harness's
    //! part with a listener of its own and exercises `dial` as the code under
    //! test.

    use super::*;
    use nix::sys::socket::{bind, listen};
    use weaver_types::{ExchangeId, LifecycleDirective, Opener, Payload, Position};

    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "weaver-admin-chan-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    /// The harness's part: bind the per-agent name and listen. The real one
    /// does this inside the unit's runtime directory as its first act.
    fn harness_listener(dir: &std::path::Path) -> (OwnedFd, std::path::PathBuf) {
        let path = dir.join("alpha.sock");
        let fd = socket(
            AddressFamily::Unix,
            SockType::SeqPacket,
            SockFlag::SOCK_CLOEXEC,
            None,
        )
        .expect("socket");
        let address = UnixAddr::new(&path).expect("address");
        bind(fd.as_raw_fd(), &address).expect("bind");
        listen(&fd, nix::sys::socket::Backlog::new(1).unwrap()).expect("listen");
        (fd, path)
    }

    fn harness_accepts(listener: &OwnedFd) -> OwnedFd {
        let fd = nix::sys::socket::accept4(listener.as_raw_fd(), SockFlag::SOCK_CLOEXEC)
            .expect("accept");
        // SAFETY: the kernel just created this descriptor and no other owner
        // exists.
        unsafe { <OwnedFd as std::os::fd::FromRawFd>::from_raw_fd(fd) }
    }

    fn directive(ordinal: u64, agent: &str) -> OrganEnvelope {
        OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Admin,
                ordinal,
            },
            position: Position::Open,
            payload: Payload::Directive(LifecycleDirective::Load {
                agent: weaver_types::AgentName(agent.to_string()),
            }),
        }
    }

    /// **The dial refuses within its bound rather than waiting.** A name
    /// nothing bound is the worker that never came up, and section 7's ceiling
    /// is what turns that into an answer.
    ///
    /// Perturbation: remove the deadline test from `dial`'s loop and this test
    /// never returns. Watched under exactly that removal.
    #[test]
    fn the_dial_refuses_within_its_bound() {
        let dir = scratch("bound");
        let path = dir.join("nothing-listens-here.sock");
        let started = std::time::Instant::now();
        let refused = dial(&path);
        let elapsed = started.elapsed();
        assert!(
            matches!(refused, Err(ChannelFault::NotDialable)),
            "a dial against an unbound name refuses, got {refused:?}"
        );
        assert!(
            elapsed >= DIAL_CEILING,
            "the dial retried for the whole bound, took {elapsed:?}"
        );
        assert!(
            elapsed < DIAL_CEILING * 4,
            "the dial stopped at its ceiling rather than waiting on, took {elapsed:?}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **The dial succeeds once the harness binds, which is the race the
    /// bound covers.** The listener appears after the dial has begun.
    #[test]
    fn the_dial_wins_the_race_when_the_bind_is_late() {
        let dir = scratch("race");
        let path = dir.join("alpha.sock");
        let bind_dir = dir.clone();
        let late = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(120));
            harness_listener(&bind_dir)
        });
        let connected = dial(&path);
        let (listener, _) = late.join().expect("thread");
        assert!(
            connected.is_ok(),
            "a bind inside the bound is reached, got {connected:?}"
        );
        drop(listener);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **One write is one read**, the boundary the socket type buys, tested
    /// where the connection is made.
    ///
    /// Perturbation: create the socket as `SOCK_STREAM` in `dial_once` and the
    /// first read returns both envelopes. **Two messages are what make the
    /// watch reachable** - a single small envelope crosses a stream socket
    /// whole, so a one-message test would pass under the substitution.
    #[test]
    fn one_write_is_one_read() {
        let dir = scratch("framing");
        let (listener, path) = harness_listener(&dir);
        let admin = dial(&path).expect("dial");
        let worker = harness_accepts(&listener);
        let worker = Coordination::adopt(worker);
        worker.send(&directive(1, "alpha")).expect("first write");
        worker.send(&directive(2, "beta")).expect("second write");
        let first = admin.recv().expect("first read decodes on its own");
        let second = admin.recv().expect("second read decodes on its own");
        assert_eq!(first.exchange.ordinal, 1);
        assert_eq!(second.exchange.ordinal, 2);
        drop(worker);
        drop(listener);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **Truncation is a fault**, never a message: a read returning with
    /// `MSG_TRUNC` set carries a prefix the kernel shortened.
    ///
    /// Perturbation: remove the `MSG_TRUNC` check from `recv` and the
    /// shortened octets reach the decoder.
    #[test]
    fn truncation_is_a_channel_fault() {
        let dir = scratch("truncate");
        let (listener, path) = harness_listener(&dir);
        let admin = dial(&path).expect("dial");
        let worker = harness_accepts(&listener);
        let oversized = vec![b'x'; MAX_ENVELOPE_BYTES + 4096];
        // SAFETY: a raw send is how the test produces a message larger than
        // the receiver's buffer; this crate's own send refuses to author one.
        let written = unsafe {
            nix::libc::send(
                worker.as_raw_fd(),
                oversized.as_ptr().cast(),
                oversized.len(),
                0,
            )
        };
        if written < 0 {
            eprintln!(
                "SKIP truncation_is_a_channel_fault: the kernel refused the oversized \
                 datagram (errno {}), so the MSG_TRUNC branch was not exercised here",
                std::io::Error::last_os_error()
            );
        } else {
            match admin.recv() {
                Err(ChannelFault::Truncated { bound }) => assert_eq!(bound, MAX_ENVELOPE_BYTES),
                other => panic!("a truncated read must be a fault, got {other:?}"),
            }
        }
        drop(worker);
        drop(listener);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The enter directive and its ancillary payload are one message: the
    /// descriptor crosses once, in the enter exchange, with no separate
    /// delivery to order against anything.
    #[test]
    fn the_enter_carries_its_descriptor_in_one_message() {
        use std::os::fd::AsFd;
        let dir = scratch("ancillary");
        let (listener, path) = harness_listener(&dir);
        let admin = dial(&path).expect("dial");
        let worker_fd = harness_accepts(&listener);

        let sink = std::fs::File::create(dir.join("sink")).expect("sink");
        let sink = OwnedFd::from(sink);
        admin
            .send_with_sink(&directive(7, "alpha"), sink.as_fd())
            .expect("one message");

        // The worker reads one message and finds the descriptor on it.
        let mut buffer = vec![0u8; MAX_ENVELOPE_BYTES];
        let mut control = nix::cmsg_space!([std::os::fd::RawFd; 4]);
        let mut slices = [std::io::IoSliceMut::new(&mut buffer)];
        let message = nix::sys::socket::recvmsg::<()>(
            worker_fd.as_raw_fd(),
            &mut slices,
            Some(&mut control),
            MsgFlags::MSG_CMSG_CLOEXEC,
        )
        .expect("one message read");
        let mut received = 0;
        for cmsg in message.cmsgs().expect("control data") {
            if let nix::sys::socket::ControlMessageOwned::ScmRights(fds) = cmsg {
                received += fds.len();
                for fd in fds {
                    // SAFETY: adopting the descriptors the kernel installed so
                    // the test closes them.
                    drop(unsafe { <OwnedFd as std::os::fd::FromRawFd>::from_raw_fd(fd) });
                }
            }
        }
        assert_eq!(
            received, 1,
            "the sink crosses on the directive's own message"
        );
        assert!(message.bytes > 0, "the envelope crossed with it");
        drop(worker_fd);
        drop(listener);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
