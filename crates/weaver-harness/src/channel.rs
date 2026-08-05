//! conforms: harness-truncation-is-a-fault
//! conforms: harness-one-write-is-one-read
//! conforms: harness-atomic-cloexec-at-creation
//! conforms: harness-organ-ends-from-descriptor-three
//! conforms: harness-child-flag-clear-unconditional
//! conforms: harness-fork-to-exec-three-calls
//! conforms: harness-trace-fd-cloexec-at-receive
//! conforms: harness-no-path-taken
//! conforms: harness-os-surface-nix
//! conforms: harness-descriptors-owned-types
//!
//! Organ-channel I/O and descriptor custody, per `weaver-harness-Spec`
//! section 2. The pairs this crate creates are `SOCK_SEQPACKET`, carrying the
//! election of `weaver-types-Spec` section 4 rather than re-deciding it, and
//! the election arrives with its obligation: the receive buffer is sized to
//! the maximum envelope and a read returning `MSG_TRUNC` is a channel fault
//! and never a message.
//!
//! No path is taken anywhere in this module. Every entry point takes owned
//! descriptors, and the organ binaries of section 3 are the one exception,
//! supplied by the composition root as a construction parameter.

use std::io::IoSlice;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};

use nix::sys::socket::{
    AddressFamily, ControlMessage, ControlMessageOwned, MsgFlags, SockFlag, SockType, recvmsg,
    sendmsg, socketpair,
};
use weaver_types::{MAX_ENVELOPE_BYTES, OrganEnvelope};

use crate::failure::ChannelFault;

/// The first descriptor after the standard streams: where an organ finds its
/// lifecycle channel from its first instruction, per section 2.2. An organ's
/// further channel takes the next number, so the gate's single end sits where
/// the SPU's first does.
pub const FIRST_ORGAN_DESCRIPTOR: RawFd = 3;

/// One end of an organ channel: a `SOCK_SEQPACKET` socket carrying one
/// `OrganEnvelope` per message, framing coming from the socket type rather
/// than from a length prefix this crate would otherwise have to write.
#[derive(Debug)]
pub struct OrganChannel {
    end: OwnedFd,
}

/// The decode end takes its own type rather than `OrganChannel`, because
/// `weaver-spu-PRD` section 13.2 rules that socket not an organ channel and a
/// shared name would carry the envelope's assumptions onto a seam that does
/// not take them. Its traffic is the token workflow's to shape.
#[derive(Debug)]
pub struct DecodeChannel {
    end: OwnedFd,
}

/// The far end of a created pair, held until the fork that hands it to a
/// child. It is an owned descriptor and nothing else: there is no call that
/// turns it into a path, and dropping it closes it.
#[derive(Debug)]
pub struct ChildEnd {
    end: OwnedFd,
}

impl OrganChannel {
    /// Creates a pair with close-on-exec set atomically at creation by
    /// `SOCK_CLOEXEC` in the `socketpair` call rather than by a later `fcntl`.
    ///
    /// The atomic form is elected because the alternative has a window: this
    /// process forks a subprocess per tool call, and a fork between creation
    /// and a separate `fcntl` would inherit an unflagged end, which for the
    /// residency seam is a release directive handed to the tool surface.
    pub fn pair() -> Result<(OrganChannel, ChildEnd), ChannelFault> {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .map_err(|_| ChannelFault::Closed)?;
        Ok((OrganChannel { end: near }, ChildEnd { end: far }))
    }

    /// Writes one envelope as one message. The bound the receiver holds is
    /// asserted at the write site too, because a bound only the receiver holds
    /// is a bound the sender discovers in production.
    pub fn send(&self, envelope: &OrganEnvelope) -> Result<(), ChannelFault> {
        let body = serde_json::to_vec(envelope).map_err(|_| ChannelFault::Undecodable)?;
        if body.len() > MAX_ENVELOPE_BYTES {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        send_octets(self.end.as_fd(), &body)
    }

    /// Reads one envelope. The buffer is sized to the maximum envelope, and a
    /// read returning `MSG_TRUNC` is a channel fault: the kernel returns the
    /// truncated prefix with the flag set and discards the remainder, so an
    /// unchecked flag turns a long directive into a silently shortened one.
    pub fn recv(&self) -> Result<OrganEnvelope, ChannelFault> {
        let octets = recv_octets(self.end.as_fd())?;
        serde_json::from_slice(&octets).map_err(|_| ChannelFault::Undecodable)
    }

    /// The one receive site that takes a descriptor: the trace sink crosses
    /// once, as ancillary data on the enter directive's own message, and this
    /// call asks for `MSG_CMSG_CLOEXEC` in the receive itself. A descriptor
    /// received without the flag arrives clear, which is the window
    /// `weaver-organ-channel` section 2 describes and the reason the
    /// obligation is the receiver's.
    pub fn recv_with_descriptor(&self) -> Result<(OrganEnvelope, Option<OwnedFd>), ChannelFault> {
        let mut buffer = vec![0u8; MAX_ENVELOPE_BYTES];
        let mut control = nix::cmsg_space!([RawFd; 4]);
        let mut slices = [std::io::IoSliceMut::new(&mut buffer)];
        let message = recvmsg::<()>(
            self.end.as_raw_fd(),
            &mut slices,
            Some(&mut control),
            MsgFlags::MSG_CMSG_CLOEXEC,
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
        let mut sink = None;
        for cmsg in message.cmsgs().map_err(|_| ChannelFault::Undecodable)? {
            if let ControlMessageOwned::ScmRights(fds) = cmsg {
                for fd in fds {
                    // SAFETY: the kernel just created this descriptor for this
                    // process and no other owner exists. Adopting it into an
                    // OwnedFd is what makes its close a type property.
                    let owned = unsafe { OwnedFd::from_raw_fd(fd) };
                    if sink.is_none() {
                        sink = Some(owned);
                    }
                }
            }
        }
        let envelope =
            serde_json::from_slice(&buffer[..read]).map_err(|_| ChannelFault::Undecodable)?;
        Ok((envelope, sink))
    }

    /// Sends one envelope carrying one descriptor as ancillary data, the
    /// sending side of the receive above. Used by the tests that stand a
    /// coordination peer up, and by any composition root that speaks this
    /// seam.
    pub fn send_with_descriptor(
        &self,
        envelope: &OrganEnvelope,
        descriptor: BorrowedFd<'_>,
    ) -> Result<(), ChannelFault> {
        let body = serde_json::to_vec(envelope).map_err(|_| ChannelFault::Undecodable)?;
        if body.len() > MAX_ENVELOPE_BYTES {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        let fds = [descriptor.as_raw_fd()];
        let control = [ControlMessage::ScmRights(&fds)];
        let slices = [IoSlice::new(&body)];
        sendmsg::<()>(
            self.end.as_raw_fd(),
            &slices,
            &control,
            MsgFlags::empty(),
            None,
        )
        .map_err(|_| ChannelFault::Closed)?;
        Ok(())
    }

    /// Adopts an end this process was handed rather than one it created.
    pub(crate) fn adopt(end: OwnedFd) -> Self {
        OrganChannel { end }
    }

    /// Yields the owned descriptor, for a composition root that created a pair
    /// here and hands one end to `Harness::adopt`. Ownership moves with it.
    pub fn into_fd(self) -> OwnedFd {
        self.end
    }
}

impl AsFd for OrganChannel {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.end.as_fd()
    }
}

impl DecodeChannel {
    /// Creates the decode pair, `SOCK_SEQPACKET` with the same atomic flag.
    /// The seam's traffic shape is the token workflow's, so this type carries
    /// custody today and no vocabulary.
    pub fn pair() -> Result<(DecodeChannel, ChildEnd), ChannelFault> {
        let (near, far) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::SOCK_CLOEXEC,
        )
        .map_err(|_| ChannelFault::Closed)?;
        Ok((DecodeChannel { end: near }, ChildEnd { end: far }))
    }
}

impl AsFd for DecodeChannel {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.end.as_fd()
    }
}

impl ChildEnd {
    /// The descriptor a child will find this end at once the handoff runs.
    pub fn as_fd(&self) -> BorrowedFd<'_> {
        self.end.as_fd()
    }

    /// What an organ does with the end it was handed: adopt it as its channel.
    /// The organ crates reach this after finding their end at descriptor 3.
    pub fn into_channel(self) -> OrganChannel {
        OrganChannel { end: self.end }
    }
}

fn send_octets(end: BorrowedFd<'_>, body: &[u8]) -> Result<(), ChannelFault> {
    let slices = [IoSlice::new(body)];
    sendmsg::<()>(end.as_raw_fd(), &slices, &[], MsgFlags::empty(), None)
        .map_err(|_| ChannelFault::Closed)?;
    Ok(())
}

fn recv_octets(end: BorrowedFd<'_>) -> Result<Vec<u8>, ChannelFault> {
    let mut buffer = vec![0u8; MAX_ENVELOPE_BYTES];
    let mut slices = [std::io::IoSliceMut::new(&mut buffer)];
    let message = recvmsg::<()>(end.as_raw_fd(), &mut slices, None, MsgFlags::empty())
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
    buffer.truncate(read);
    Ok(buffer)
}

/// The child's side of the handoff, between fork and exec: `dup2` each end it
/// is given to a descriptor from 3 upward, `fcntl` the flag clear on each
/// unconditionally, and `execve`. Nothing else - all three are
/// async-signal-safe, and the bound is the safety argument rather than a
/// style, because the worker holds the writer's thread at every fork.
///
/// The clear is unconditional because the duplicate law has a corner: a
/// duplicate made by `dup2` is born with the flag clear, but only when the two
/// descriptors differ, and `dup2` onto the same number is a no-op that returns
/// the descriptor with its flag untouched. A child whose end already sat at
/// descriptor 3 would keep the flag, lose the end at `execve`, and start the
/// organ with no channel, silently.
///
/// # Safety
///
/// Must be called only in a forked child, before its exec, and only with ends
/// this process created.
pub unsafe fn place_child_ends(ends: &[&ChildEnd]) -> Result<(), nix::Error> {
    for (index, end) in ends.iter().enumerate() {
        let target = FIRST_ORGAN_DESCRIPTOR + index as RawFd;
        let source = end.end.as_raw_fd();
        if source != target {
            // SAFETY: dup2 is async-signal-safe and both numbers are this
            // child's own. The duplicate is born with the flag clear.
            unsafe { nix::libc::dup2(source, target) };
        }
        // The unconditional clear: the no-op case above leaves the flag set,
        // so this runs whether or not a duplication moved anything.
        // SAFETY: fcntl is async-signal-safe.
        unsafe { nix::libc::fcntl(target, nix::libc::F_SETFD, 0) };
    }
    Ok(())
}
