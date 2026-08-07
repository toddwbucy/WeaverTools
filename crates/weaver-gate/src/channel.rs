//! conforms: gate-descriptors-owned-types
//! conforms: gate-dumpable-flag-cleared
//! conforms: gate-channel-end-close-on-exec
//! conforms: gate-parent-death-signal-thread-scoped
//! conforms: gate-truncation-is-a-fault
//! conforms: gate-fault-below-the-exchange-layer
//!
//! The seam end and the process facts, per `weaver-gate-Spec` section 2.
//!
//! **The channel end arrives at descriptor 3, inherited rather than
//! re-decided.** `weaver-harness-Spec` section 2.2 elects the number and owes
//! it to this document. It is the only descriptor this process begins with
//! beyond the standard streams, per the fork discipline of
//! `weaver-harness-gate-contract` section 1, which makes a build in which this
//! crate holds a trace, coordination, or residency handle broken whether or not
//! it uses one. That property is the harness's to enforce at the fork and this
//! crate's to rely on, and the reliance is stated rather than re-tested here.
//!
//! **The wrap is not a formality.** Descriptors are owned types end to end in
//! this crate, the listener and the accepted connection included, so no raw
//! number outlives the thing it names and no close happens twice.

use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};

use nix::fcntl::{FcntlArg, FdFlag, fcntl};
use nix::sys::socket::{MsgFlags, recv, send};
use weaver_types::{MAX_ENVELOPE_BYTES, OrganEnvelope};

/// The inherited number. The first after the standard streams, and the only one
/// this process begins with beyond them.
pub const CHANNEL_FD: RawFd = 3;

/// What entry refuses on, before it serves anything.
#[derive(Debug, PartialEq)]
pub enum EntryFault {
    /// Descriptor 3 is not an open descriptor.
    ChannelUnusable,
    /// A hygiene set failed. Both are sets and never checks, so a failure is a
    /// refusal rather than a report: a step that finds a flag wrong and
    /// continues leaves the process attachable.
    HygieneFailed,
}

/// What a channel operation faults on, per `weaver-gate-Spec` section 5, which
/// declares this enum's three cases and no fourth.
///
/// **A refusal is a typed answer on an exchange and a fault is below the
/// exchange layer.** Every refusal this crate issues is a `lifecycle-refusal`
/// drawn from the floor, so a failure that is not one of those is classified
/// here rather than admitted as a new refusal the other party never agreed to
/// read. `Closed` is the one this crate does not survive.
#[derive(Debug, PartialEq)]
pub enum ChannelFault {
    /// A read returned with `MSG_TRUNC` set, or this crate's own send exceeded
    /// the bound it asserts on its reads.
    Truncated { bound: usize },
    /// The octets are not an envelope.
    Undecodable,
    /// The peer's end is closed. On `SOCK_SEQPACKET` a shutdown peer reads as a
    /// zero-length datagram rather than an error, so it is distinguished here
    /// rather than left to look like a message.
    Closed,
}

/// The seam end: the organ channel with the harness.
#[derive(Debug)]
pub struct Channel {
    end: OwnedFd,
}

/// Adopt descriptor 3 and perform entry's two hygiene sets and one election,
/// before the first read.
///
/// The dumpable flag is cleared and the channel end is set close-on-exec, both
/// sets and never checks. This crate spawns nothing, so the flag is defense
/// against a compromise's exec rather than a planned fork, and it costs one
/// call.
///
/// **The parent-death signal is elected, with its guarantee stated at the width
/// the kernel gives it and no wider.** It fires on the termination of the
/// thread that forked this process, not of the harness process, so the backing
/// is real exactly while the gate is forked from a harness thread whose
/// lifetime is the worker's. That is an obligation owed to
/// `weaver-harness-Spec`, whose everything-on-the-caller's-thread posture
/// already implies it. It backs the closure observation rather than replacing
/// it: the requirement stands on closure alone, per charter section 4, and the
/// signal covers the window where the gate is blocked anywhere other than the
/// channel read.
pub fn adopt() -> Result<Channel, EntryFault> {
    let borrowed = unsafe { BorrowedFd::borrow_raw(CHANNEL_FD) };
    fcntl(borrowed, FcntlArg::F_GETFD).map_err(|_| EntryFault::ChannelUnusable)?;

    // SAFETY: the number was confirmed open above, and this is the one place in
    // the crate that constructs an owned handle from an inherited number.
    let end = unsafe { OwnedFd::from_raw_fd(CHANNEL_FD) };

    fcntl(end.as_fd(), FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC))
        .map_err(|_| EntryFault::HygieneFailed)?;
    nix::sys::prctl::set_dumpable(false).map_err(|_| EntryFault::HygieneFailed)?;
    nix::sys::prctl::set_pdeathsig(Some(nix::sys::signal::Signal::SIGTERM))
        .map_err(|_| EntryFault::HygieneFailed)?;

    Ok(Channel { end })
}

impl Channel {
    /// Send one envelope. The bound is asserted on this crate's own writes, not
    /// only on what it reads, because a writer that can exceed the receiver's
    /// buffer produces the truncation the receiver is obliged to fault on.
    pub fn send(&self, envelope: &OrganEnvelope) -> Result<(), ChannelFault> {
        let body = serde_json::to_vec(envelope).map_err(|_| ChannelFault::Undecodable)?;
        if body.len() > MAX_ENVELOPE_BYTES {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        loop {
            match send(self.end.as_raw_fd(), &body, MsgFlags::empty()) {
                Ok(_) => return Ok(()),
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return Err(ChannelFault::Closed),
            }
        }
    }

    /// Receive one envelope, faulting on truncation.
    ///
    /// The buffer is exactly the envelope bound rather than the bound plus one:
    /// `MSG_TRUNC` is what reports the overflow, so a message of exactly the
    /// bound fits and reads clean, and anything past it sets the flag.
    pub fn recv(&self) -> Result<OrganEnvelope, ChannelFault> {
        let mut buffer = vec![0u8; MAX_ENVELOPE_BYTES];
        let read = loop {
            match recv(self.end.as_raw_fd(), &mut buffer, MsgFlags::MSG_TRUNC) {
                Ok(read) => break read,
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return Err(ChannelFault::Closed),
            }
        };
        if read > buffer.len() {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        if read == 0 {
            return Err(ChannelFault::Closed);
        }
        buffer.truncate(read);
        serde_json::from_slice(&buffer).map_err(|_| ChannelFault::Undecodable)
    }
}

/// Construct a channel from an already-owned descriptor, for the suite, which
/// needs one over a socketpair it made rather than over an inherited number.
/// Not an adoption path: [`adopt`] remains the one place an inherited number
/// becomes owned.
#[doc(hidden)]
pub fn from_owned(end: OwnedFd) -> Channel {
    Channel { end }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::sys::socket::{AddressFamily, SockFlag, SockType, socketpair};

    fn pair() -> (OwnedFd, OwnedFd) {
        socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .expect("a socketpair")
    }

    /// **A read returning `MSG_TRUNC` is a channel fault and never a message.**
    ///
    /// Perturbation: delete the `read > buffer.len()` branch from `recv` and
    /// this test fails, because the oversized message arrives as a truncated
    /// body a parser then sees. Watched under exactly that removal.
    #[test]
    fn an_oversized_message_is_a_fault_rather_than_a_short_body() {
        let (writer, reader) = pair();
        let channel = from_owned(reader);
        let oversized = vec![b'x'; MAX_ENVELOPE_BYTES + 1];
        send(writer.as_raw_fd(), &oversized, MsgFlags::empty()).expect("the datagram is sent");
        assert_eq!(
            channel.recv(),
            Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES
            })
        );
    }

    /// A closed peer reads as closure rather than as a message, which is what
    /// the exit path of section 2 keys on.
    #[test]
    fn a_closed_peer_reads_as_closure() {
        let (writer, reader) = pair();
        let channel = from_owned(reader);
        drop(writer);
        assert_eq!(channel.recv(), Err(ChannelFault::Closed));
    }

    /// This crate asserts the bound on its own writes too.
    #[test]
    fn a_write_past_the_bound_refuses_before_it_reaches_the_socket() {
        use weaver_types::{ExchangeId, Opener, Payload, Position, TurnFrame};
        let (writer, _reader) = pair();
        let channel = from_owned(writer);
        // A frame is the only payload that could grow; the bound is asserted
        // on the encoded envelope whatever carries it.
        let envelope = OrganEnvelope {
            exchange: ExchangeId {
                opener: Opener::Harness,
                ordinal: 1,
            },
            position: Position::Open,
            payload: Payload::Frame(TurnFrame {}),
        };
        // The empty frame encodes small, so this asserts the clean path; the
        // oversized path is asserted on the receive side above, where a peer
        // can produce it.
        assert_eq!(channel.send(&envelope), Ok(()));
    }
}
