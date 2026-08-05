//! conforms: admin-operator-socket-stream
//! conforms: admin-surface-refuses-at-accept
//! conforms: admin-refusal-by-closure-unanswered
//! conforms: admin-one-answer-per-request-serially
//! conforms: admin-request-line-refused-unaccumulated
//! conforms: admin-surface-carries-no-envelope
//! conforms: admin-descriptors-owned-types
//!
//! The operator surface, per `weaver-admin-Spec` section 2: the socket of the
//! charter's section 8, governed by `weaver-admin-operator-contract`.
//!
//! The socket is `SOCK_STREAM`, because the contract fixes newline-delimited
//! JSON completely - the newline is the framing, so a boundary-preserving type
//! would carry a second framing under the first. This is the opposite election
//! from the organ channels and it is principled the same way: that channel has
//! no in-band framing and buys boundaries from the type, this surface has
//! in-band framing from its contract and buys nothing from packet boundaries.

use std::collections::BTreeSet;
use std::io::{BufReader, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

use weaver_types::{
    AccessRule, LifecycleAnswer, LifecycleDirective, LifecycleRefusal, MAX_ENVELOPE_BYTES,
    PeerIdentity, authorized,
};

/// The rule the operator surface judges a peer against: the allow set is the
/// `weaver-admin` group and the deny set is every agent uid the fleet's
/// allow-list names. Denial wins over permission, per `weaver-types-Spec`
/// section 3, so an agent uid inside a misconfigured group grant is still
/// refused.
#[derive(Debug, Clone)]
pub struct SurfacePolicy {
    pub rule: AccessRule,
}

impl SurfacePolicy {
    pub fn new(admin_gid: u32, agent_uids: impl IntoIterator<Item = u32>) -> Self {
        SurfacePolicy {
            rule: AccessRule {
                allowed_uids: BTreeSet::new(),
                allowed_gids: BTreeSet::from([admin_gid]),
                denied_uids: agent_uids.into_iter().collect(),
            },
        }
    }
}

/// What a served connection did, so a caller and a test can tell a refusal at
/// accept from a connection that was served.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Served {
    /// The peer failed the predicate: the connection closed, unanswered, with
    /// no byte read.
    RefusedAtAccept,
    /// The peer passed and its requests were served serially, in order.
    Served { requests: usize },
}

/// Binds the operator socket. The directory is the operator's to place, per
/// section 9, and this call takes the path it was given rather than searching
/// for one.
pub fn bind(path: &Path) -> std::io::Result<UnixListener> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    let listener = UnixListener::bind(path)?;
    // The socket file is 0660: a group member can dial and the agent uid
    // cannot. Reachability is not the authentication - it is one more fence in
    // front of the check that counts.
    set_mode(path, 0o660)?;
    Ok(listener)
}

/// Reads the connecting peer's credential from the kernel.
///
/// The credential on an accepted connection reports the connecting peer's own
/// uid, gid, and pid, which is what makes this check real where an inherited
/// pair's credential is not.
pub fn peer_identity(stream: &UnixStream) -> std::io::Result<PeerIdentity> {
    // SAFETY: getsockopt on a descriptor this process owns, into a struct the
    // kernel fills entirely.
    let fd = stream.as_raw_fd();
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
    Ok(PeerIdentity {
        uid: credential.uid,
        gid: credential.gid,
        pid: credential.pid,
    })
}

/// Serves one accepted connection.
///
/// **Every connection is authenticated before any byte is read.** A peer that
/// fails the predicate is refused by closure, unanswered: the refusal is
/// enacted at the connection rather than written to it, and no
/// `LifecycleRefusal` value crosses to a peer the predicate rejected, which is
/// the contract's two clauses holding as one behaviour.
///
/// **Requests are served serially within the connection**, which is what makes
/// one answer per request in request order a structural property rather than a
/// bookkeeping claim.
pub fn serve_connection<F>(
    stream: UnixStream,
    policy: &SurfacePolicy,
    mut dispatch: F,
) -> std::io::Result<Served>
where
    F: FnMut(LifecycleDirective) -> Result<LifecycleAnswer, LifecycleRefusal>,
{
    let peer = peer_identity(&stream)?;
    if !authorized(&peer, &policy.rule) {
        // Closure is the refusal. Nothing is read and nothing is written.
        drop(stream);
        return Ok(Served::RefusedAtAccept);
    }

    let reader_stream = stream.try_clone()?;
    let mut writer = stream;
    let mut reader = BufReader::new(reader_stream);
    let mut requests = 0usize;

    loop {
        let mut line = String::new();
        match read_bounded_line(&mut reader, &mut line)? {
            LineOutcome::Eof => break,
            LineOutcome::OverBound => {
                // **A line over the bound is refused without being
                // accumulated**, because an unbounded line is an unbounded
                // allocation handed to a peer whose content nothing has
                // authenticated.
                write_answer(&mut writer, &Err(LifecycleRefusal::Malformed))?;
                requests += 1;
                continue;
            }
            LineOutcome::NotUtf8 => {
                write_answer(&mut writer, &Err(LifecycleRefusal::Malformed))?;
                requests += 1;
                continue;
            }
            LineOutcome::Line => {}
        }
        let trimmed = line.trim_end_matches('\n');
        // **An empty frame is answered rather than skipped.** A skipped frame
        // shifts request-to-answer correlation for everything after it, which
        // is the one property this surface's serial discipline exists to give.
        if trimmed.is_empty() {
            write_answer(&mut writer, &Err(LifecycleRefusal::Malformed))?;
            requests += 1;
            continue;
        }
        // **The wire shapes are the floor's own, bare.** One request line is
        // one directive in the floor's internally tagged rendering, and no
        // organ envelope appears here, because this is not an organ channel
        // and the contract draws none.
        let outcome = match serde_json::from_str::<LifecycleDirective>(trimmed) {
            Ok(directive) => dispatch(directive),
            Err(_) => Err(LifecycleRefusal::Malformed),
        };
        write_answer(&mut writer, &outcome)?;
        requests += 1;
    }
    Ok(Served::Served { requests })
}

enum LineOutcome {
    Line,
    OverBound,
    /// The frame's octets are not UTF-8, so no JSON can be decoded from them.
    NotUtf8,
    Eof,
}

/// Reads one newline-terminated line, bounded at the program's one message
/// bound. The bound is the organ envelope's, reused rather than justified a
/// second time.
fn read_bounded_line(
    reader: &mut BufReader<UnixStream>,
    line: &mut String,
) -> std::io::Result<LineOutcome> {
    let mut raw: Vec<u8> = Vec::new();
    let mut taken = 0usize;
    loop {
        let mut byte = [0u8; 1];
        let read = std::io::Read::read(reader, &mut byte)?;
        if read == 0 {
            return Ok(if taken == 0 {
                LineOutcome::Eof
            } else {
                LineOutcome::Line
            });
        }
        if byte[0] == b'\n' {
            // The whole frame is decoded at once, and octets that are not
            // UTF-8 are refused rather than replaced.
            match std::str::from_utf8(&raw) {
                Ok(text) => {
                    line.push_str(text);
                    return Ok(LineOutcome::Line);
                }
                Err(_) => return Ok(LineOutcome::NotUtf8),
            }
        }
        taken += 1;
        if taken > MAX_ENVELOPE_BYTES {
            // Drain to the newline without keeping any of it: the refusal
            // costs one line's traversal and no allocation.
            drain_to_newline(reader)?;
            line.clear();
            return Ok(LineOutcome::OverBound);
        }
        // **Bytes are buffered as bytes.** Pushing `byte as char` would map
        // each octet to its own Unicode scalar, so any multi-byte sequence
        // decodes as different text than the peer sent - a UTF-8 agent name
        // could name a different agent.
        raw.push(byte[0]);
    }
}

fn drain_to_newline(reader: &mut BufReader<UnixStream>) -> std::io::Result<()> {
    let mut byte = [0u8; 1];
    loop {
        let read = std::io::Read::read(reader, &mut byte)?;
        if read == 0 || byte[0] == b'\n' {
            return Ok(());
        }
    }
}

/// One answer line: one `LifecycleAnswer` or one `LifecycleRefusal`, and the
/// discriminant between the two is the tag itself, the case sets being
/// disjoint at the floor.
fn write_answer(
    writer: &mut UnixStream,
    outcome: &Result<LifecycleAnswer, LifecycleRefusal>,
) -> std::io::Result<()> {
    let rendered = match outcome {
        Ok(answer) => serde_json::to_string(answer),
        Err(refusal) => serde_json::to_string(refusal),
    }
    .map_err(std::io::Error::other)?;
    writer.write_all(rendered.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.flush()
}

/// Sets a path's mode. Used for the socket file and the directories this crate
/// owns, all of which the operator placed and this crate does not search for.
pub fn set_mode(path: &Path, mode: u32) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
}

/// Accepts one connection with close-on-exec set in the accepting call, so no
/// subprocess this crate spawns can inherit it.
pub fn accept_cloexec(listener: &UnixListener) -> std::io::Result<UnixStream> {
    // SAFETY: accept4 on a listener this process owns; the returned descriptor
    // is adopted immediately into an owned type.
    let fd = unsafe {
        nix::libc::accept4(
            listener.as_raw_fd(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            nix::libc::SOCK_CLOEXEC,
        )
    };
    if fd == -1 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: the kernel just created this descriptor and no other owner
    // exists.
    let owned = unsafe { OwnedFd::from_raw_fd(fd) };
    Ok(UnixStream::from(owned))
}

/// Reads a descriptor's close-on-exec flag, for the walks that assert custody.
#[cfg(test)]
pub fn is_close_on_exec(fd: std::os::fd::BorrowedFd<'_>) -> bool {
    // SAFETY: F_GETFD on a descriptor this process owns.
    let flags = unsafe { nix::libc::fcntl(fd.as_raw_fd(), nix::libc::F_GETFD) };
    flags != -1 && (flags & nix::libc::FD_CLOEXEC) != 0
}

#[cfg(test)]
mod tests {
    //! The first walk's operator-surface half, run from inside the crate
    //! because this crate publishes no library surface and an integration test
    //! could not reach these modules without one, which
    //! `admin-no-library-surface` forbids.

    use super::*;
    use std::io::{BufRead, BufReader};
    use weaver_types::AgentName;

    fn policy_denying_us() -> SurfacePolicy {
        // The connecting peer is this process, so denying our own uid is how
        // the test presents the agent uid to the predicate - and our group is
        // in the allow set, which is the misconfigured-group-grant case the
        // denial ordering exists to defeat.
        SurfacePolicy {
            rule: AccessRule {
                allowed_uids: BTreeSet::new(),
                allowed_gids: BTreeSet::from([nix::unistd::getgid().as_raw()]),
                denied_uids: BTreeSet::from([nix::unistd::getuid().as_raw()]),
            },
        }
    }

    fn policy_admitting_us() -> SurfacePolicy {
        SurfacePolicy {
            rule: AccessRule {
                allowed_uids: BTreeSet::new(),
                allowed_gids: BTreeSet::from([nix::unistd::getgid().as_raw()]),
                denied_uids: BTreeSet::new(),
            },
        }
    }

    /// **The first walk: the agent dials its own supervisor.** The predicate
    /// refuses the agent uid at accept, denial winning over the group grant,
    /// and the refusal is a closure with nothing written.
    ///
    /// Perturbation: drop the deny set from the rule and the connection is
    /// served; replace the closure with a written refusal and a value crosses
    /// to a peer the predicate rejected. Watched under both.
    #[test]
    fn a_denied_peer_is_refused_by_closure_unanswered() {
        let (client, server) = UnixStream::pair().expect("pair");
        // The write half closes up front so this watch fails fast rather than
        // hanging: with the predicate weakened the server would otherwise
        // block on a read no one answers, and a watch that hangs cannot tell
        // detection from an infrastructure problem.
        client
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown write");
        let served = serve_connection(server, &policy_denying_us(), |_| {
            panic!("a denied peer's content must never be read")
        })
        .expect("served");
        assert_eq!(served, Served::RefusedAtAccept);

        let mut reader = BufReader::new(client);
        let mut line = String::new();
        let read = reader.read_line(&mut line).expect("read");
        assert_eq!(read, 0, "no answer crosses to a refused peer, got {line:?}");
    }

    /// An admitted peer is served one answer per request, in request order, on
    /// the one connection.
    ///
    /// Perturbation: serve a connection's requests concurrently and the
    /// answers interleave, which the alternation below catches.
    #[test]
    fn an_admitted_peer_is_served_serially_in_order() {
        let (client, server) = UnixStream::pair().expect("pair");
        let mut writer = client.try_clone().expect("clone");
        let handle = std::thread::spawn(move || {
            serve_connection(
                server,
                &policy_admitting_us(),
                |directive| match directive {
                    LifecycleDirective::Show { .. } => Ok(LifecycleAnswer::State {
                        state: weaver_types::AgentState::Idle,
                    }),
                    LifecycleDirective::List => Ok(LifecycleAnswer::Agents { agents: vec![] }),
                    _ => Err(LifecycleRefusal::OutOfOrder),
                },
            )
        });

        for _ in 0..3 {
            let show = serde_json::to_string(&LifecycleDirective::Show {
                agent: AgentName("alpha".into()),
            })
            .expect("renders");
            writeln!(writer, "{show}").expect("write");
            let list = serde_json::to_string(&LifecycleDirective::List).expect("renders");
            writeln!(writer, "{list}").expect("write");
        }
        // The write half closes so the server sees end-of-file; the read half
        // stays open to collect the answers. Dropping the clone alone would
        // leave the server blocked on a read that never returns.
        writer
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown write");
        drop(writer);

        let reader = BufReader::new(client);
        let answers: Vec<String> = reader.lines().map(|l| l.expect("line")).collect();
        let served = handle.join().expect("thread").expect("served");
        assert_eq!(served, Served::Served { requests: 6 });
        assert_eq!(answers.len(), 6, "one answer per request");
        for (index, answer) in answers.iter().enumerate() {
            if index % 2 == 0 {
                assert!(answer.contains("\"state\""), "answer {index}: {answer}");
            } else {
                assert!(answer.contains("\"agents\""), "answer {index}: {answer}");
            }
        }
    }

    /// A line over the program's one message bound is refused without being
    /// accumulated: an unbounded line is an unbounded allocation handed to a
    /// peer whose content nothing has authenticated.
    #[test]
    fn an_over_bound_line_is_refused_not_accumulated() {
        let (client, server) = UnixStream::pair().expect("pair");
        let mut writer = client.try_clone().expect("clone");
        let handle = std::thread::spawn(move || {
            serve_connection(server, &policy_admitting_us(), |_| {
                panic!("an over-bound line never reaches dispatch")
            })
        });
        let oversized = "x".repeat(MAX_ENVELOPE_BYTES + 1024);
        writeln!(writer, "{oversized}").expect("write");
        writer
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown write");
        drop(writer);
        let mut reader = BufReader::new(client);
        let mut line = String::new();
        reader.read_line(&mut line).expect("read");
        assert!(line.contains("malformed"), "refused as malformed: {line}");
        let _ = handle.join().expect("thread");
    }

    /// A frame whose octets are not UTF-8 is refused rather than mangled, and
    /// a multi-byte name survives the read intact.
    ///
    /// Perturbation: buffer with `byte as char` and the multi-byte assertion
    /// fails, because each octet becomes its own scalar. Watched under exactly
    /// that change.
    #[test]
    fn frames_preserve_utf8_and_refuse_what_is_not() {
        use weaver_types::AgentName;
        let (client, server) = UnixStream::pair().expect("pair");
        let mut writer = client.try_clone().expect("clone");
        let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let recorder = std::sync::Arc::clone(&seen);
        let handle = std::thread::spawn(move || {
            serve_connection(server, &policy_admitting_us(), move |d| {
                if let LifecycleDirective::Show { agent } = &d {
                    recorder.lock().unwrap().push(agent.0.clone());
                }
                Ok(LifecycleAnswer::Validated)
            })
        });
        let name = "agent-\u{e9}\u{4e2d}";
        let show = serde_json::to_string(&LifecycleDirective::Show {
            agent: AgentName(name.into()),
        })
        .expect("renders");
        writeln!(writer, "{show}").expect("write");
        writer.write_all(&[0xff, 0xfe, b'\n']).expect("write raw");
        writer
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown");
        drop(writer);
        let reader = BufReader::new(client);
        let answers: Vec<String> = reader.lines().map(|l| l.expect("line")).collect();
        let _ = handle.join().expect("thread");
        assert_eq!(
            seen.lock().unwrap().as_slice(),
            &[name.to_string()],
            "the multi-byte name survives the read"
        );
        assert_eq!(answers.len(), 2, "both frames are answered");
        assert!(
            answers[1].contains("malformed"),
            "non-UTF-8 refuses: {}",
            answers[1]
        );
    }

    /// An empty frame is answered rather than skipped, so request-to-answer
    /// correlation cannot shift under a client that sends one.
    #[test]
    fn an_empty_frame_is_answered() {
        let (client, server) = UnixStream::pair().expect("pair");
        let mut writer = client.try_clone().expect("clone");
        let handle = std::thread::spawn(move || {
            serve_connection(server, &policy_admitting_us(), |_| {
                Ok(LifecycleAnswer::Validated)
            })
        });
        writeln!(writer).expect("empty frame");
        writeln!(
            writer,
            "{}",
            serde_json::to_string(&LifecycleDirective::List).expect("renders")
        )
        .expect("write");
        writer
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown");
        drop(writer);
        let reader = BufReader::new(client);
        let answers: Vec<String> = reader.lines().map(|l| l.expect("line")).collect();
        let _ = handle.join().expect("thread");
        assert_eq!(answers.len(), 2, "one answer per frame, empty included");
        assert!(
            answers[0].contains("malformed"),
            "the empty frame is refused"
        );
    }

    /// The answer line carries the floor's shape bare: no organ envelope
    /// appears on this surface, because this is not an organ channel and the
    /// contract draws none.
    #[test]
    fn the_surface_carries_no_envelope() {
        let (client, server) = UnixStream::pair().expect("pair");
        let mut writer = client.try_clone().expect("clone");
        let handle = std::thread::spawn(move || {
            serve_connection(server, &policy_admitting_us(), |_| {
                Ok(LifecycleAnswer::Validated)
            })
        });
        writeln!(
            writer,
            "{}",
            serde_json::to_string(&LifecycleDirective::List).expect("renders")
        )
        .expect("write");
        writer
            .shutdown(std::net::Shutdown::Write)
            .expect("shutdown write");
        drop(writer);
        let mut reader = BufReader::new(client);
        let mut line = String::new();
        reader.read_line(&mut line).expect("read");
        assert!(
            line.contains("\"kind\":\"validated\""),
            "bare answer: {line}"
        );
        assert!(!line.contains("\"exchange\""), "no envelope: {line}");
        assert!(!line.contains("\"position\""), "no envelope: {line}");
        let _ = handle.join().expect("thread");
    }
}
