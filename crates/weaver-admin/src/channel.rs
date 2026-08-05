//! conforms: admin-one-write-is-one-read
//! conforms: admin-bind-and-listen-precedes-unit-start
//! conforms: admin-coordination-directory-unsearchable
//! conforms: admin-coordination-peer-credential-checked
//! conforms: admin-listener-closed-after-one-accept
//! conforms: admin-truncation-is-a-channel-fault
//! conforms: admin-enter-carries-descriptor-in-one-message
//!
//! The coordination channel, per `weaver-admin-Spec` section 7: constructed
//! fresh per load, `SOCK_SEQPACKET`, carrying the election of
//! `weaver-types-Spec` section 4 rather than re-deciding it.
//!
//! **Four acts, and the acts straddle the unit's start.** The first two run
//! before it, because the worker connects at its start through the declared
//! open and a connect against a name not yet listening is the race the
//! ordering exists to prevent. The last two run after it, because there is no
//! peer to accept until the worker exists.

use std::os::fd::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::path::Path;

use nix::sys::socket::{
    AddressFamily, ControlMessage, MsgFlags, SockFlag, SockType, UnixAddr, accept4, bind, listen,
    sendmsg, socket,
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
    /// The peer's credential is not the agent's: the connection is refused.
    WrongPeer {
        uid: u32,
    },
}

/// The listener, standing between the first two acts and the last two.
#[derive(Debug)]
pub struct Listener {
    fd: OwnedFd,
}

/// The accepted connection to the worker.
#[derive(Debug)]
pub struct Coordination {
    fd: OwnedFd,
    ordinal: u64,
}

/// Acts one and two: create the socket with close-on-exec in the creating call
/// and bind it to a per-agent name inside an admin-owned directory of mode
/// 0700 - the unsearchable home - then listen, **before the unit is asked
/// for**.
pub fn bind_and_listen(
    directory: &Path,
    agent: &str,
) -> std::io::Result<(Listener, std::path::PathBuf)> {
    std::fs::create_dir_all(directory)?;
    // 0700: the agent uid cannot traverse to the name at all. This is a fence
    // in front of the checks that count, not the check itself.
    crate::surface::set_mode(directory, 0o700)?;
    let path = directory.join(format!("{agent}.sock"));
    if path.exists() {
        std::fs::remove_file(&path)?;
    }
    let fd = socket(
        AddressFamily::Unix,
        SockType::SeqPacket,
        SockFlag::SOCK_CLOEXEC,
        None,
    )
    .map_err(std::io::Error::from)?;
    let address = UnixAddr::new(&path).map_err(std::io::Error::from)?;
    bind(fd.as_raw_fd(), &address).map_err(std::io::Error::from)?;
    listen(&fd, nix::sys::socket::Backlog::new(1).unwrap()).map_err(std::io::Error::from)?;
    crate::surface::set_mode(&path, 0o600)?;
    Ok((Listener { fd }, path))
}

impl Listener {
    /// The listener's descriptor, for the walk that asserts it carries
    /// close-on-exec from its creating call.
    #[cfg(test)]
    pub fn as_fd(&self) -> BorrowedFd<'_> {
        use std::os::fd::AsFd;
        self.fd.as_fd()
    }

    /// Acts three and four: accept exactly once with close-on-exec in the
    /// accepting call, check the peer's credential against the agent's
    /// expected uid, and **close the listener after the one accept**.
    ///
    /// The closure is what leaves possession meaning something here: an
    /// elected tool running as the agent uid passes the credential check,
    /// being the uid the check expects, and what refuses it is that no
    /// listener remains to answer a second dial.
    pub fn accept_once(self, expected_uid: u32) -> Result<Coordination, ChannelFault> {
        let fd = accept4(self.fd.as_raw_fd(), SockFlag::SOCK_CLOEXEC)
            .map_err(|_| ChannelFault::Closed)?;
        // SAFETY: the kernel just created this descriptor and no other owner
        // exists.
        let connection = unsafe { OwnedFd::from_raw_fd(fd) };
        let credential = peer_uid(connection.as_raw_fd()).map_err(|_| ChannelFault::Closed)?;
        // The listener closes here whatever the credential says: a refused
        // peer must not leave a listener standing for a second attempt.
        drop(self.fd);
        if credential != expected_uid {
            return Err(ChannelFault::WrongPeer { uid: credential });
        }
        Ok(Coordination {
            fd: connection,
            ordinal: 0,
        })
    }
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

fn peer_uid(fd: std::os::fd::RawFd) -> std::io::Result<u32> {
    // SAFETY: getsockopt on a descriptor this process owns, into a struct the
    // kernel fills entirely.
    let mut credential: nix::libc::ucred = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<nix::libc::ucred>() as nix::libc::socklen_t;
    let rc = unsafe {
        nix::libc::getsockopt(
            fd,
            nix::libc::SOL_SOCKET,
            nix::libc::SO_PEERCRED,
            (&mut credential as *mut nix::libc::ucred).cast(),
            &mut len,
        )
    };
    if rc == -1 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(credential.uid)
}

#[cfg(test)]
mod tests {
    //! The first walk's coordination half and the fourth walk, plus the
    //! framing pair `weaver-types-Spec` section 5 owes the pair-creating
    //! crates. Run from inside the crate because this crate publishes no
    //! library surface.

    use super::*;
    use weaver_types::{AgentName, ExchangeId, LifecycleDirective, Opener, Payload, Position};

    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "weaver-admin-chan-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

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

    fn dial(path: &std::path::Path) -> std::io::Result<OwnedFd> {
        let fd = socket(
            AddressFamily::Unix,
            SockType::SeqPacket,
            SockFlag::SOCK_CLOEXEC,
            None,
        )
        .map_err(std::io::Error::from)?;
        let address = UnixAddr::new(path).map_err(std::io::Error::from)?;
        nix::sys::socket::connect(fd.as_raw_fd(), &address).map_err(std::io::Error::from)?;
        Ok(fd)
    }

    /// The bound socket carries close-on-exec from its creating call, and the
    /// directory is 0700 - the unsearchable home the agent cannot traverse to.
    ///
    /// Perturbation: drop `SOCK_CLOEXEC` from the socket call and the flag
    /// assertion fails; loosen the directory mode and the mode assertion does.
    #[test]
    fn the_listener_is_flagged_and_its_directory_unsearchable() {
        use std::os::unix::fs::PermissionsExt;
        let dir = scratch("listen");
        let (listener, _path) = bind_and_listen(&dir, "alpha").expect("bind");
        assert!(
            crate::surface::is_close_on_exec(listener.as_fd()),
            "the listener is flagged at its creating call"
        );
        let mode = std::fs::metadata(&dir).expect("dir").permissions().mode() & 0o777;
        assert_eq!(mode, 0o700, "the coordination directory is unsearchable");
        drop(listener);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **The first walk's coordination half: the listener is closed after its
    /// one accept**, so a post-accept dial is kernel-refused even for a
    /// process that somehow resolved the name.
    ///
    /// Perturbation: keep the listener open past the accept and the second
    /// dial succeeds. Watched under exactly that change.
    #[test]
    fn a_second_dial_is_refused_after_the_one_accept() {
        let dir = scratch("second-dial");
        let (listener, path) = bind_and_listen(&dir, "alpha").expect("bind");
        let dial_path = path.clone();
        let peer = std::thread::spawn(move || dial(&dial_path).expect("first dial"));
        let us = nix::unistd::getuid().as_raw();
        let coordination = listener.accept_once(us).expect("accept");
        let held = peer.join().expect("thread");

        let second = dial(&path);
        assert!(
            second.is_err(),
            "no listener remains to answer a second dial"
        );
        // The accepted connection lives on: it is held here rather than
        // read, a read having no sender to answer it.
        drop(coordination);
        drop(held);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **The fourth walk: a stranger answers on the coordination channel.**
    /// The credential check at accept refuses a peer whose uid is not the
    /// agent's.
    ///
    /// Perturbation: remove the credential comparison and the stranger's
    /// connection is accepted. Watched under exactly that removal.
    #[test]
    fn a_wrong_uid_is_refused_at_accept() {
        let dir = scratch("credential");
        let (listener, path) = bind_and_listen(&dir, "alpha").expect("bind");
        let dial_path = path.clone();
        let peer = std::thread::spawn(move || dial(&dial_path).expect("dial"));
        // The connecting peer is this process, so expecting any other uid is
        // how the test presents a stranger to the check.
        let us = nix::unistd::getuid().as_raw();
        let refused = listener.accept_once(us.wrapping_add(1));
        let held = peer.join().expect("thread");
        match refused {
            Err(ChannelFault::WrongPeer { uid }) => assert_eq!(uid, us),
            other => panic!("a stranger is refused at accept, got {other:?}"),
        }
        drop(held);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// **One write is one read**, the boundary property the socket type was
    /// elected to buy: two envelopes written back to back arrive as two reads
    /// of exactly one envelope each.
    ///
    /// Perturbation: create the socket as `SOCK_STREAM` and the first read
    /// returns both envelopes concatenated, failing the decode. **Two messages
    /// are what make the watch reachable** - a single small envelope crosses a
    /// stream socket whole, so a one-message test would pass under the
    /// substitution and pin nothing.
    #[test]
    fn one_write_is_one_read() {
        let dir = scratch("framing");
        let (listener, path) = bind_and_listen(&dir, "alpha").expect("bind");
        let dial_path = path.clone();
        let peer = std::thread::spawn(move || {
            let fd = dial(&dial_path).expect("dial");
            let worker = Coordination::adopt(fd);
            worker.send(&directive(1, "alpha")).expect("first write");
            worker.send(&directive(2, "beta")).expect("second write");
            worker
        });
        let us = nix::unistd::getuid().as_raw();
        let admin = listener.accept_once(us).expect("accept");
        let held = peer.join().expect("thread");
        let first = admin.recv().expect("first read decodes on its own");
        let second = admin.recv().expect("second read decodes on its own");
        assert_eq!(first.exchange.ordinal, 1);
        assert_eq!(second.exchange.ordinal, 2);
        drop(held);
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
        let (listener, path) = bind_and_listen(&dir, "alpha").expect("bind");
        let dial_path = path.clone();
        let peer = std::thread::spawn(move || {
            let fd = dial(&dial_path).expect("dial");
            let oversized = vec![b'x'; MAX_ENVELOPE_BYTES + 4096];
            // SAFETY: a raw send is how the test produces a message larger
            // than the receiver's buffer; this crate's own send refuses to
            // author one.
            let written = unsafe {
                nix::libc::send(
                    fd.as_raw_fd(),
                    oversized.as_ptr().cast(),
                    oversized.len(),
                    0,
                )
            };
            (fd, written)
        });
        let us = nix::unistd::getuid().as_raw();
        let admin = listener.accept_once(us).expect("accept");
        let (held, written) = peer.join().expect("thread");
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
        drop(held);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The enter directive and its ancillary payload are one message: the
    /// descriptor crosses once, in the enter exchange, with no separate
    /// delivery to order against anything.
    #[test]
    fn the_enter_carries_its_descriptor_in_one_message() {
        use std::os::fd::AsFd;
        let dir = scratch("ancillary");
        let (listener, path) = bind_and_listen(&dir, "alpha").expect("bind");
        let dial_path = path.clone();
        let peer = std::thread::spawn(move || dial(&dial_path).expect("dial"));
        let us = nix::unistd::getuid().as_raw();
        let admin = listener.accept_once(us).expect("accept");
        let worker_fd = peer.join().expect("thread");

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
        .expect("receive");
        let read = message.bytes;
        let mut descriptors = 0;
        for cmsg in message.cmsgs().expect("cmsgs") {
            if let nix::sys::socket::ControlMessageOwned::ScmRights(fds) = cmsg {
                for fd in fds {
                    descriptors += 1;
                    // SAFETY: the kernel installed it for this process.
                    drop(unsafe { OwnedFd::from_raw_fd(fd) });
                }
            }
        }
        assert_eq!(
            descriptors, 1,
            "the sink crossed on the enter's own message"
        );
        let envelope: OrganEnvelope =
            serde_json::from_slice(&buffer[..read]).expect("one envelope");
        assert_eq!(envelope.exchange.ordinal, 7);
        drop(worker_fd);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
