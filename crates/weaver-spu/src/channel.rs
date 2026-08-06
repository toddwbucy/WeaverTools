//! conforms: spu-descriptors-owned-types
//! conforms: spu-dumpable-flag-cleared
//! conforms: spu-channel-ends-close-on-exec
//! conforms: spu-descriptor-count-check
//! conforms: spu-envelope-on-lifecycle-only
//! conforms: spu-truncation-is-a-fault
//!
//! The two channel ends and the process facts, per `weaver-spu-Spec` section 2.
//!
//! Both ends arrive inherited rather than re-decided: **3 is the lifecycle
//! channel and 4 is the decode socket**, the numbering and its order elected by
//! `weaver-harness-Spec` section 2.2 and owed to this document, which takes it.
//! Lifecycle sits first because it is the channel every organ has and the one an
//! organ with a single end already places there, so the gate's placement is
//! unchanged and this crate's first end sits where the gate's does.
//!
//! The envelope crosses the lifecycle end only. Per charter section 13.2 the
//! decode socket is not an organ channel, so [`DecodeSocket`] carries octets and
//! offers no envelope method at all: the confinement is the absence of the
//! surface rather than a rule a caller is asked to keep.

use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};

use nix::dir::Dir;
use nix::fcntl::{FcntlArg, FdFlag, OFlag, fcntl};
use nix::sys::socket::{MsgFlags, recv, send};
use nix::sys::stat::Mode;
use weaver_types::{MAX_ENVELOPE_BYTES, OrganEnvelope};

/// The lifecycle channel's inherited number. Descriptor 3 is the channel every
/// organ has, which is why it is first.
pub const LIFECYCLE_FD: RawFd = 3;

/// The decode socket's inherited number, the second end the decoder cut gave
/// this crate.
pub const DECODE_FD: RawFd = 4;

/// What entry refuses on, before it serves anything.
#[derive(Debug, PartialEq)]
pub enum EntryFault {
    /// The count check failed: this process holds descriptors beyond the two it
    /// was given and the standard streams. A count above two means the harness's
    /// fork discipline failed upstream, and this process is not the one to
    /// continue past it. The count is reported rather than the identity, since
    /// this crate cannot know what a stray descriptor refers to.
    DescriptorCountWrong { found: usize },
    /// An inherited number is not an open descriptor, or the enumeration the
    /// count check needs could not be performed.
    DescriptorsUnusable,
    /// A hygiene set failed. Both sets are sets and never checks, so a failure
    /// here is a refusal rather than a report: a step that finds a flag wrong
    /// and continues leaves the process attachable, which is the condition the
    /// set exists to prevent.
    HygieneFailed,
}

/// The two ends, adopted and owned.
///
/// Descriptors are owned types end to end in this crate, so no raw number
/// outlives the thing it names and no close happens twice. The wrap is not a
/// formality: it is what `spu-descriptors-owned-types` pins.
#[derive(Debug)]
pub struct Inherited {
    pub lifecycle: LifecycleChannel,
    pub decode: DecodeSocket,
}

/// The first end: the organ channel with the harness, carrying the envelope.
#[derive(Debug)]
pub struct LifecycleChannel {
    end: OwnedFd,
}

/// The second end: the decode socket, which is not an organ channel.
///
/// It carries the token trio in whatever encoding the measurement of Spec
/// section 11 elects, so this type offers octets and no envelope. The election
/// is open and code filling it would be invention.
#[derive(Debug)]
pub struct DecodeSocket {
    end: OwnedFd,
}

/// Adopt both ends and perform the two sets, before the first read.
///
/// The order is the Spec's: the count check answers first, because a process
/// that failed the discipline upstream should refuse rather than adopt; then
/// the wrap; then close-on-exec on both ends; then the dumpable clear.
///
/// The close-on-exec set matters even though this crate forks nothing, because
/// `execve` clears the flag and the requirement is stated against the last exec.
pub fn adopt() -> Result<Inherited, EntryFault> {
    let held = descriptors_beyond_the_standard_streams()?;
    if held != 2 {
        return Err(EntryFault::DescriptorCountWrong { found: held });
    }

    // Both numbers must name open descriptors before either is wrapped: wrapping
    // first and failing second would drop a half-adopted pair through the
    // OwnedFd close, which is a close of a descriptor this process may not own.
    for fd in [LIFECYCLE_FD, DECODE_FD] {
        let borrowed = unsafe { BorrowedFd::borrow_raw(fd) };
        fcntl(borrowed, FcntlArg::F_GETFD).map_err(|_| EntryFault::DescriptorsUnusable)?;
    }

    // SAFETY: both numbers were confirmed open above, the count check confirmed
    // there are exactly two beyond the standard streams, and this is the one
    // place in the crate that constructs an owned handle from an inherited
    // number. Every later use flows from these two values.
    let lifecycle = unsafe { OwnedFd::from_raw_fd(LIFECYCLE_FD) };
    let decode = unsafe { OwnedFd::from_raw_fd(DECODE_FD) };

    set_close_on_exec(lifecycle.as_fd())?;
    set_close_on_exec(decode.as_fd())?;
    clear_dumpable()?;

    Ok(Inherited {
        lifecycle: LifecycleChannel { end: lifecycle },
        decode: DecodeSocket { end: decode },
    })
}

/// A set and never a check, per `weaver-organ-channel` section 2.
fn set_close_on_exec(end: BorrowedFd<'_>) -> Result<(), EntryFault> {
    fcntl(end, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC)).map_err(|_| EntryFault::HygieneFailed)?;
    Ok(())
}

/// The second walk's mechanism: a same-uid process attaching by `ptrace` or
/// through `/proc/[pid]/fd` reaches a process holding the weights and both
/// channels. Clearing the flag is what closes it, and it is a set rather than a
/// check for the same reason the neighbour above is.
fn clear_dumpable() -> Result<(), EntryFault> {
    nix::sys::prctl::set_dumpable(false).map_err(|_| EntryFault::HygieneFailed)
}

/// Count the open descriptors this process holds beyond the standard streams.
///
/// The enumeration itself opens a descriptor, so the directory handle is
/// excluded by its own number. Counting it would make the check answer three
/// where the truth is two, which is a count check that can never pass.
fn descriptors_beyond_the_standard_streams() -> Result<usize, EntryFault> {
    let mut dir = Dir::open(
        "/proc/self/fd",
        OFlag::O_RDONLY | OFlag::O_DIRECTORY | OFlag::O_CLOEXEC,
        Mode::empty(),
    )
    .map_err(|_| EntryFault::DescriptorsUnusable)?;
    let own = dir.as_raw_fd();

    let mut held = 0usize;
    for entry in dir.iter() {
        let entry = entry.map_err(|_| EntryFault::DescriptorsUnusable)?;
        // `.` and `..` fail the parse and are skipped by it rather than by name.
        let Ok(number) = entry.file_name().to_string_lossy().parse::<RawFd>() else {
            continue;
        };
        if number <= 2 || number == own {
            continue;
        }
        held += 1;
    }
    Ok(held)
}

/// What a channel read or write refuses on.
#[derive(Debug, PartialEq)]
pub enum ChannelFault {
    /// The kernel reported a short read. Per the `SOCK_SEQPACKET` election of
    /// `weaver-types-Spec` section 4.3, a read returning `MSG_TRUNC` is a
    /// channel fault and never a message: a silently shortened directive is the
    /// failure mode the boundary property was elected to prevent.
    Truncated,
    /// This crate's own write exceeds the bound it asserts on its reads.
    TooLarge { bytes: usize },
    /// The peer is gone or the socket refused the operation.
    Unusable,
    /// The octets read are not an envelope.
    Malformed,
}

impl LifecycleChannel {
    /// Send one envelope. The bound is asserted on this crate's own writes, not
    /// only on what it reads, because a writer that can exceed the receiver's
    /// buffer produces the truncation the receiver is obliged to fault on.
    pub fn send(&self, envelope: &OrganEnvelope) -> Result<(), ChannelFault> {
        let body = serde_json::to_vec(envelope).map_err(|_| ChannelFault::Malformed)?;
        if body.len() > MAX_ENVELOPE_BYTES {
            return Err(ChannelFault::TooLarge { bytes: body.len() });
        }
        send_octets(self.end.as_fd(), &body)
    }

    /// Receive one envelope, faulting on truncation.
    pub fn recv(&self) -> Result<OrganEnvelope, ChannelFault> {
        let body = recv_octets(self.end.as_fd())?;
        serde_json::from_slice(&body).map_err(|_| ChannelFault::Malformed)
    }

    pub fn as_fd(&self) -> BorrowedFd<'_> {
        self.end.as_fd()
    }
}

impl DecodeSocket {
    /// Send one message. No envelope method exists on this type: the decode
    /// socket is not an organ channel and the confinement is the absent surface.
    pub fn send_octets(&self, body: &[u8]) -> Result<(), ChannelFault> {
        if body.len() > MAX_ENVELOPE_BYTES {
            return Err(ChannelFault::TooLarge { bytes: body.len() });
        }
        send_octets(self.end.as_fd(), body)
    }

    /// Receive one message, faulting on truncation on the same grounds the
    /// lifecycle end does: the obligation attaches to the socket type, and both
    /// ends carry it.
    pub fn recv_octets(&self) -> Result<Vec<u8>, ChannelFault> {
        recv_octets(self.end.as_fd())
    }

    pub fn as_fd(&self) -> BorrowedFd<'_> {
        self.end.as_fd()
    }
}

fn send_octets(end: BorrowedFd<'_>, body: &[u8]) -> Result<(), ChannelFault> {
    send(end.as_raw_fd(), body, MsgFlags::empty()).map_err(|_| ChannelFault::Unusable)?;
    Ok(())
}

/// The receive obligation the `SOCK_SEQPACKET` election attaches: a buffer sized
/// to the envelope bound, and a read returning `MSG_TRUNC` treated as a channel
/// fault.
///
/// The buffer is deliberately exactly the bound rather than the bound plus one.
/// `MSG_TRUNC` is what reports the overflow, so a message of exactly the bound
/// fits and reads clean, and anything past it sets the flag.
fn recv_octets(end: BorrowedFd<'_>) -> Result<Vec<u8>, ChannelFault> {
    let mut buffer = vec![0u8; MAX_ENVELOPE_BYTES];
    let read = recv(end.as_raw_fd(), &mut buffer, MsgFlags::MSG_TRUNC)
        .map_err(|_| ChannelFault::Unusable)?;
    if read > buffer.len() {
        // With MSG_TRUNC on a SEQPACKET socket the kernel returns the real
        // datagram length rather than the copied length, so a return above the
        // buffer is the truncation report.
        return Err(ChannelFault::Truncated);
    }
    buffer.truncate(read);
    Ok(buffer)
}

/// Construct a lifecycle end from an already-owned descriptor.
///
/// This exists for the suite, which needs a channel over a socketpair it made
/// rather than over an inherited number. It is not an adoption path: [`adopt`]
/// remains the one place an inherited number becomes owned.
#[doc(hidden)]
pub fn lifecycle_from_owned(end: OwnedFd) -> LifecycleChannel {
    LifecycleChannel { end }
}

/// The decode-end counterpart of [`lifecycle_from_owned`], on the same grounds.
#[doc(hidden)]
pub fn decode_from_owned(end: OwnedFd) -> DecodeSocket {
    DecodeSocket { end }
}
