//! conforms: harness-truncation-is-a-fault
//! conforms: harness-one-write-is-one-read
//! conforms: harness-atomic-cloexec-at-creation
//! conforms: harness-binds-coordination-socket-first
//! conforms: harness-coordination-accept-refuses-non-root
//! conforms: harness-bind-never-unlinks
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
//! **One path is taken, and it is not the trace's.** The coordination socket
//! is bound to a name the composition root supplies, per section 2.3 and the
//! inversion ruling of 2026-08-05, which is a deployment fact of the same kind
//! as the organ binaries of section 3. The prohibition `weaver-harness-PRD`
//! section 5 states is about the trace, and the trace still reaches this crate
//! as a descriptor and never as a name.

use std::io::IoSlice;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};

use nix::sys::socket::{
    AddressFamily, ControlMessage, MsgFlags, SockFlag, SockType, UnixAddr, accept4, bind,
    getsockopt, listen, recvmsg, sendmsg, socket, socketpair, sockopt,
};
use weaver_types::{MAX_ENVELOPE_BYTES, OrganEnvelope};

use crate::failure::ChannelFault;

/// The control buffer's size, generous against the one descriptor loop 0
/// legitimately carries so an honest message never truncates. A message that
/// truncates it anyway is a fault, and every descriptor it did deliver is
/// adopted and closed before that fault returns.
const CONTROL_BYTES: usize = 256;

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

/// The coordination listener, bound inside the agent's own runtime directory
/// and held for the worker's whole life.
#[derive(Debug)]
pub struct CoordinationListener {
    fd: OwnedFd,
}

/// Binds the coordination socket and listens, which is the worker's first act.
///
/// **Any socket connecting to the harness is an internal connection**, per
/// `weaver-admin-harness-contract` section 2, so this crate creates and binds
/// it rather than adopting a handed end. The socket carries close-on-exec from
/// its creating call, so no window exists between creation and flag.
///
/// **The bind never unlinks.** A Unix socket's pathname outlives the process
/// that bound it, and the directory this socket lives in is created by the init
/// system at the unit's start and removed with the unit, per
/// `weaver-admin-systemd-contract` sections 2 and 5, so the name cannot be
/// inherited from a previous run and there is nothing to clear. **A bind that
/// finds its name occupied is a fault and never a thing to remove**, because
/// the only ways a name is occupied are that a live worker holds it, where
/// unlinking would strand the running agent's supervisor, or that the manager
/// did not honor the directory, where this crate's assumption is wrong and it
/// should say so rather than repair.
pub fn bind_coordination(
    socket_path: &std::path::Path,
) -> Result<CoordinationListener, ChannelFault> {
    let fd = socket(
        AddressFamily::Unix,
        SockType::SeqPacket,
        SockFlag::SOCK_CLOEXEC,
        None,
    )
    .map_err(|_| ChannelFault::Closed)?;
    let address = UnixAddr::new(socket_path).map_err(|_| ChannelFault::Undecodable)?;
    bind(fd.as_raw_fd(), &address).map_err(|_| ChannelFault::Closed)?;
    listen(&fd, nix::sys::socket::Backlog::new(1).unwrap()).map_err(|_| ChannelFault::Closed)?;
    Ok(CoordinationListener { fd })
}

impl AsFd for CoordinationListener {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl CoordinationListener {
    /// Accepts one connection, reading the peer credential before any byte.
    ///
    /// **Every accept refuses what is not root.** The socket is reachable from
    /// inside the sandbox by construction, so an elected tool at the agent uid
    /// can dial it, and what refuses that tool is that it is not root. The
    /// earlier design expected the agent's own uid here, which every tool of
    /// the agent's satisfies, and leaned on a listener closed after one accept
    /// to do the refusing.
    ///
    /// **The listener is not closed after an accept**, because admin is
    /// per-invocation and every later verb dials again.
    pub fn accept_root(&self) -> Result<OrganChannel, ChannelFault> {
        let fd = accept4(self.fd.as_raw_fd(), SockFlag::SOCK_CLOEXEC)
            .map_err(|_| ChannelFault::Closed)?;
        // SAFETY: the kernel just created this descriptor and no other owner
        // exists.
        let connection = unsafe { OwnedFd::from_raw_fd(fd) };
        let peer =
            getsockopt(&connection, sockopt::PeerCredentials).map_err(|_| ChannelFault::Closed)?;
        if peer.uid() != 0 {
            // Refused at the connection rather than written to: no answer
            // crosses to a peer the check rejected.
            drop(connection);
            return Err(ChannelFault::WrongPeer { uid: peer.uid() });
        }
        Ok(OrganChannel { end: connection })
    }
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
        let mut control = [0u8; CONTROL_BYTES];

        // **The receive is issued through `libc` rather than `nix` here, and
        // the reason is custody.** `nix`'s `RecvMsg::cmsgs` refuses to iterate
        // when `MSG_CTRUNC` is set, returning `ENOBUFS`, which is exactly the
        // case where descriptors were installed and must be closed: an
        // iterator that will not run on the truncated path leaves every
        // delivered descriptor unowned. Owning the `msghdr` is what lets the
        // walk below happen on every path, truncated or not.
        let mut iov = nix::libc::iovec {
            iov_base: buffer.as_mut_ptr().cast(),
            iov_len: buffer.len(),
        };
        // SAFETY: a zeroed msghdr is a valid empty header; every field the
        // call reads is set below.
        let mut mhdr: nix::libc::msghdr = unsafe { std::mem::zeroed() };
        mhdr.msg_iov = &mut iov;
        mhdr.msg_iovlen = 1;
        mhdr.msg_control = control.as_mut_ptr().cast();
        mhdr.msg_controllen = control.len() as _;

        // SAFETY: the header points at buffers this frame owns for the call's
        // duration, and the descriptor is this channel's own.
        let read = unsafe {
            nix::libc::recvmsg(self.end.as_raw_fd(), &mut mhdr, nix::libc::MSG_CMSG_CLOEXEC)
        };
        if read < 0 {
            return Err(ChannelFault::Closed);
        }

        // **Custody is established before the fault checks, not after them.**
        // The kernel can install ancillary descriptors on the same call that
        // returns a truncated body or a truncated control buffer, so a fault
        // check that returns first leaks descriptors this process now owns
        // and nothing closes. A peer sending oversized descriptor-bearing
        // messages would exhaust the table and the harness could then create
        // no organ channels. Adopting first makes every path drop what it
        // owns.
        let mut sink = None;
        let mut surplus = Vec::new();
        // SAFETY: the walk reads only headers the kernel wrote into this
        // frame's control buffer, bounded by the length it reported, and each
        // descriptor is adopted exactly once.
        unsafe {
            let mut cmsg = nix::libc::CMSG_FIRSTHDR(&mhdr);
            while !cmsg.is_null() {
                let header_len = (*cmsg).cmsg_len as usize;
                let payload_offset = nix::libc::CMSG_LEN(0) as usize;
                if header_len < payload_offset || header_len > mhdr.msg_controllen as usize {
                    // A header the kernel could not complete: nothing after it
                    // can be trusted, and nothing in it names a descriptor.
                    break;
                }
                if (*cmsg).cmsg_level == nix::libc::SOL_SOCKET
                    && (*cmsg).cmsg_type == nix::libc::SCM_RIGHTS
                {
                    let data = nix::libc::CMSG_DATA(cmsg);
                    let width = std::mem::size_of::<RawFd>();
                    let count = (header_len - payload_offset) / width;
                    for index in 0..count {
                        let mut fd: RawFd = -1;
                        std::ptr::copy_nonoverlapping(
                            data.add(index * width),
                            (&mut fd as *mut RawFd).cast::<u8>(),
                            width,
                        );
                        if fd < 0 {
                            continue;
                        }
                        // The kernel just installed this descriptor for this
                        // process and no other owner exists, so adopting it
                        // into an OwnedFd is what makes its close a type
                        // property.
                        let owned = OwnedFd::from_raw_fd(fd);
                        if sink.is_none() {
                            sink = Some(owned);
                        } else {
                            surplus.push(owned);
                        }
                    }
                }
                cmsg = nix::libc::CMSG_NXTHDR(&mhdr, cmsg);
            }
        }
        drop(surplus);

        // A truncated control buffer means the kernel dropped descriptors the
        // sender authored, so the message is not the one that was sent and
        // cannot be repaired by reading what survived. What did arrive is
        // already owned and closed by the drop above.
        if mhdr.msg_flags & nix::libc::MSG_CTRUNC != 0 || mhdr.msg_flags & nix::libc::MSG_TRUNC != 0
        {
            return Err(ChannelFault::Truncated {
                bound: MAX_ENVELOPE_BYTES,
            });
        }
        let read = read as usize;
        if read == 0 {
            return Err(ChannelFault::Closed);
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
    if ends.len() > MAX_PLACED_ENDS {
        return Err(nix::Error::EINVAL);
    }
    let count = ends.len() as RawFd;
    let target_end = FIRST_ORGAN_DESCRIPTOR + count;

    // **A source sitting inside the target range is moved out of it first,
    // because `dup2` onto a target closes whatever already occupies that
    // number.** With ends at 4 and 3, placing the first at 3 would close the
    // second before its own turn, and placing the second at 4 would then
    // duplicate the first: both numbers would name one channel and the other
    // would be gone. Descriptor numbers are the lowest free number, so a
    // descending layout is reachable whenever an earlier close left a hole.
    let mut sources = [0 as RawFd; MAX_PLACED_ENDS];
    for (index, end) in ends.iter().enumerate() {
        let mut source = end.end.as_raw_fd();
        if source >= FIRST_ORGAN_DESCRIPTOR && source < target_end {
            // SAFETY: fcntl is async-signal-safe. F_DUPFD_CLOEXEC returns the
            // lowest free descriptor at or above the floor, which is outside
            // the range every dup2 below will overwrite.
            let moved = unsafe { nix::libc::fcntl(source, nix::libc::F_DUPFD_CLOEXEC, target_end) };
            if moved == -1 {
                return Err(nix::Error::last());
            }
            source = moved;
        }
        sources[index] = source;
    }

    for (index, source) in sources.iter().take(ends.len()).enumerate() {
        let target = FIRST_ORGAN_DESCRIPTOR + index as RawFd;
        if *source != target {
            // SAFETY: dup2 is async-signal-safe and both numbers are this
            // child's own. The duplicate is born with the flag clear.
            if unsafe { nix::libc::dup2(*source, target) } == -1 {
                // **A failed placement must not reach the exec.** The organ
                // would start with no channel at 3, or with whatever
                // unrelated descriptor sat there, which is the silent
                // no-channel start this whole design exists to prevent.
                return Err(nix::Error::last());
            }
        }
        // The unconditional clear: the no-op case above leaves the flag set,
        // so this runs whether or not a duplication moved anything.
        // SAFETY: fcntl is async-signal-safe.
        if unsafe { nix::libc::fcntl(target, nix::libc::F_SETFD, 0) } == -1 {
            return Err(nix::Error::last());
        }
    }
    Ok(())
}

/// The most ends any organ is handed: the SPU's two. A fixed array keeps the
/// pre-move bookkeeping allocation-free, which the async-signal-safe bound
/// between fork and exec requires.
const MAX_PLACED_ENDS: usize = 8;
