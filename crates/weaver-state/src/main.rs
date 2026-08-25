//! conforms: state-preload-door-stands-only-diagnostic
//! conforms: state-preload-door-refuses-the-agent
//!
//! The member's process: stand the seam's name, judge the peer by
//! credential, read the election, stand the store, and land distillates
//! until the channel closes. Per `weaver-harness-state-contract`, the seam
//! is a Unix socket with a name, authenticated by credential per the first
//! invariant, so the member binds and the worker dials. The arguments are
//! the territory, the socket path, the one uid the first door's peer may
//! hold, and, where the party that stands this member names one, the
//! preload socket.
//!
//! **The second door stands where that fourth argument does**, per
//! `weaver-state-Spec` section 4, and its judgment inverts the first
//! door's: it refuses the agent's uid and admits the operator principal.
//! Both doors are served from one loop by `poll`, so the store keeps one
//! owner and a distillate lands the same way whichever door carried it,
//! which is the mechanism of the contract's indistinguishability claim.

use std::io::Read;
use std::os::fd::AsFd;

use weaver_state::{
    Ask, Election, Store, parse_ask, parse_distillate, render_recall_answer,
    render_replay_answer, render_shape_answer,
};

/// How long the name waits for its one peer before concluding the load
/// never came, so an abandoned member is a bounded cost rather than a
/// resident one.
const ACCEPT_WAIT_MS: u16 = 60_000;

/// The bound on one answer frame, matched by the harness's own cap on
/// what it reads: an answer past this size is a fault answered with
/// silence, per the contract's clause that custody never invents an
/// answer shape for a fault.
const ANSWER_BOUND: usize = 1024 * 1024;

/// The bound on the answer's write: a peer that takes nothing for this
/// long has stopped reading, and a custodian wedged on its behalf would
/// cost the session its custody, so the seam retires instead, holdings
/// standing.
const RESPOND_WAIT_MS: u16 = 2_000;

fn main() -> std::process::ExitCode {
    // **The preload name is a fourth argument and its absence is a serving
    // load**, per `weaver-state-Spec` section 2's pattern, which leaves the
    // descriptor choreography to this act: the name reaches the member the
    // way the territory and the first socket do, on the vector, because no
    // exchange this member holds carries a path.
    let mut arguments = std::env::args().skip(1);
    let (Some(territory), Some(socket), Some(peer)) =
        (arguments.next(), arguments.next(), arguments.next())
    else {
        eprintln!(
            "{}",
            serde_json::json!({"state_fault": "usage: weaver-state <territory> <socket> <peer-uid> [preload-socket]"})
        );
        return std::process::ExitCode::FAILURE;
    };
    let preload_socket = arguments.next();
    let Ok(peer_uid) = peer.parse::<u32>() else {
        eprintln!("{}", serde_json::json!({"state_fault": "peer uid does not parse"}));
        return std::process::ExitCode::FAILURE;
    };

    let Some(mut channel) = stand_and_accept(&socket, peer_uid) else {
        return std::process::ExitCode::FAILURE;
    };

    let path = std::path::Path::new(&territory).join("state.sql");
    let mut store = match Store::open(&path) {
        Ok(store) => store,
        Err(fault) => {
            eprintln!("{}", serde_json::json!({"state_fault": format!("{fault:?}")}));
            return std::process::ExitCode::FAILURE;
        }
    };

    // The election opens the flow, per the contract: the first line is the
    // opener, and the indexes stand before the first distillate.
    let mut lines = LineReader::new(&mut channel);
    let Some(opener) = lines.next_line() else {
        // A channel closed before its opener is a load that did not finish
        // standing, and an empty stand is the honest outcome.
        return std::process::ExitCode::SUCCESS;
    };
    // **The session the opener declared bounds every answer.** Held for the
    // channel's life beside the election, per the contract's amended term.
    // An opener that names none leaves it empty, which matches no row, so a
    // custodian that could not learn its session answers nothing rather than
    // answering across every session the file holds - the defect this
    // repairs, where unbounded reads looked perfectly well formed.
    let session = parse_session(&opener).unwrap_or_default();
    let election = parse_election(&opener).unwrap_or_default();
    if let Err(fault) = store.index_election(&election) {
        eprintln!("{}", serde_json::json!({"state_fault": format!("{fault:?}")}));
        return std::process::ExitCode::FAILURE;
    }

    // Both doors are poll-driven from here, so the first door stops blocking.
    if lines.stream.set_nonblocking(true).is_err() {
        eprintln!("{}", serde_json::json!({"state_fault": "channel would not go non-blocking"}));
        return std::process::ExitCode::FAILURE;
    }

    // The preload name stands only where the party that stands this member
    // named one, and that party names it only under a diagnostic binding,
    // per `weaver-state-Spec` section 4. Absence is a serving load, not a
    // fault: the door does not exist and a driver finds nothing to dial.
    let preload = match preload_socket.as_deref().map(stand_preload_name) {
        None => None,
        Some(Some(listener)) => Some(listener),
        Some(None) => return std::process::ExitCode::FAILURE,
    };

    serve(lines, preload, &mut store, &session, peer_uid)
}

/// Custody until closure, across the doors this standing carries.
///
/// **One store, one path, one thread.** A distillate arriving on the preload
/// channel lands exactly as one arriving from the tee, which is the mechanism
/// of the contract's indistinguishability claim, and the serve loop reaches
/// both doors by `poll` rather than by a second thread so the store keeps one
/// owner and the landing order is the arrival order.
fn serve(
    mut harness: LineReader<'_>,
    preload_listener: Option<std::os::unix::net::UnixListener>,
    store: &mut Store,
    session: &str,
    agent_uid: u32,
) -> std::process::ExitCode {
    use std::os::fd::AsFd;

    // **The seal is a per-standing fact held apart from the transport**, per
    // the Spec: false before any dial, false mid-stream, false after a
    // sealless close, and true from the seal frame on. Nothing sets it back.
    let mut sealed = false;
    // **One parked replay slot per channel**, per the contract's retry
    // mechanism: a second replay ask replaces the first, the replaced ask
    // cleared unanswered because its asker's bound already converted it.
    let mut replay_parked = false;
    // The door stands until a peer is admitted, then the channel stands in
    // its place. Both are never live at once, the name being unlinked at the
    // accept the way the first door's is.
    let mut listener = preload_listener;
    let mut preload: Option<std::os::unix::net::UnixStream> = None;
    let mut preload_frames: Vec<u8> = Vec::new();
    let mut preload_opened = false;
    // Where no preload door stands, a replay ask answers immediately like
    // its two siblings, per the contract's closing sentence.
    let door_stands = listener.is_some();

    loop {
        let mut fds = Vec::with_capacity(2);
        fds.push(nix::poll::PollFd::new(
            harness.stream.as_fd(),
            nix::poll::PollFlags::POLLIN,
        ));
        if let Some(channel) = &preload {
            fds.push(nix::poll::PollFd::new(
                channel.as_fd(),
                nix::poll::PollFlags::POLLIN,
            ));
        } else if let Some(door) = &listener {
            fds.push(nix::poll::PollFd::new(
                door.as_fd(),
                nix::poll::PollFlags::POLLIN,
            ));
        }
        match nix::poll::poll(&mut fds, nix::poll::PollTimeout::NONE) {
            Ok(_) => {}
            Err(nix::errno::Errno::EINTR) => continue,
            Err(_) => return std::process::ExitCode::SUCCESS,
        }
        let harness_ready = fds[0].revents().is_some_and(|r| !r.is_empty());
        let second_ready = fds.len() > 1 && fds[1].revents().is_some_and(|r| !r.is_empty());
        drop(fds);

        // The preload door first, so a seal landing in the same wakeup as a
        // parked ask answers in that wakeup rather than the next.
        if second_ready {
            if let Some(channel) = preload.as_mut() {
                let live = fill_buffer(channel, &mut preload_frames);
                while let Some(line) = take_frame(&mut preload_frames) {
                    if !preload_opened {
                        // **The opener's retirement is the one act this path
                        // adds**, per the Spec: the declared session's rows go
                        // in the same transaction that records the opener,
                        // before any distillate lands, so re-running a preload
                        // replaces the holdings rather than appending to them
                        // and a dead driver's prefix needs no cleanup act.
                        let election = parse_election(&line).unwrap_or_default();
                        if store.retire_and_index(session, &election).is_err() {
                            return std::process::ExitCode::FAILURE;
                        }
                        preload_opened = true;
                        continue;
                    }
                    if is_seal(&line) {
                        sealed = true;
                        continue;
                    }
                    if let Some(distillate) = parse_distillate(&line) {
                        let _ = store.land(&distillate);
                    }
                }
                if !live {
                    // A close without a seal leaves the fact false, which is
                    // the clause's point: the prefix looks like holdings at
                    // rest and must not answer a parked ask.
                    preload = None;
                }
            } else if let Some(door) = &listener {
                if let Some(channel) = admit_operator(door, agent_uid) {
                    preload = Some(channel);
                    listener = None;
                }
            }
        }

        if harness_ready {
            let live = harness.fill();
            while let Some(line) = harness.take_line() {
                if let Some(distillate) = parse_distillate(&line) {
                    let _ = store.land(&distillate);
                    continue;
                }
                let Some(ask) = parse_ask(&line) else { continue };
                // **The parked ask steps out of the arrival order**, per the
                // contract's stated exception: a shape or recall ask arriving
                // while a replay parks is answered in its own arrival order,
                // against the holdings the stream carried before it.
                if matches!(ask, Ask::Replay) && door_stands && !sealed {
                    replay_parked = true;
                    continue;
                }
                // A store that cannot answer, like an answer past the bound,
                // is silence the harness's bound converts, per the contract:
                // custody never invents an answer shape for a fault.
                let frame = match ask {
                    Ask::Shape => store.shape(session).map(|shape| render_shape_answer(&shape)),
                    Ask::Recall { last_turns } => store
                        .recall(session, last_turns)
                        .map(|events| render_recall_answer(&events)),
                    Ask::Replay => store
                        .replay(session)
                        .map(|events| render_replay_answer(&events)),
                };
                if let Ok(frame) = frame
                    && frame.len() <= ANSWER_BOUND
                    && !harness.respond(frame.as_bytes())
                {
                    return std::process::ExitCode::SUCCESS;
                }
            }
            if !live {
                // Closure is retirement, the holdings standing for the next
                // run. A replay still parked is cleared unanswered with the
                // channel, its answer never owed.
                return std::process::ExitCode::SUCCESS;
            }
        }

        // **The seal is the only fact that answers**, and its view is the
        // seal's position: every distillate received through the seal is in
        // the answer, which holds by construction here because the preload
        // frames of this wakeup landed above before the answer is built.
        if replay_parked && sealed {
            replay_parked = false;
            if let Ok(events) = store.replay(session) {
                let frame = render_replay_answer(&events);
                if frame.len() <= ANSWER_BOUND && !harness.respond(frame.as_bytes()) {
                    return std::process::ExitCode::SUCCESS;
                }
            }
        }
    }
}

/// Pop one buffered frame, or nothing where no whole frame is held. Never
/// touches a socket, so a caller draining frames cannot block on a peer that
/// stopped mid frame.
fn take_frame(buffer: &mut Vec<u8>) -> Option<String> {
    let position = buffer.iter().position(|&b| b == b'\n')?;
    let line: Vec<u8> = buffer.drain(..=position).collect();
    Some(String::from_utf8_lossy(&line[..line.len() - 1]).into_owned())
}

/// Read whatever the socket has ready into the buffer. `false` says the
/// channel is done, by close or by fault or by a frame past the bound, which
/// the serve loop reads as the end of that door. The stream is non-blocking
/// by the time this runs, so `WouldBlock` is the ordinary answer and says
/// only that this wakeup is spent.
fn fill_buffer(stream: &mut std::os::unix::net::UnixStream, buffer: &mut Vec<u8>) -> bool {
    use std::io::Read;
    loop {
        if buffer.len() > LineReader::FRAME_BOUND {
            return false;
        }
        let mut chunk = [0u8; 65536];
        match stream.read(&mut chunk) {
            Ok(0) => return false,
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => return true,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
            Err(_) => return false,
        }
    }
}

/// Bind the preload name and listen, without waiting for a peer. The door
/// stands from the load so a driver dialing early finds it, and the accept
/// happens in the serve loop beside the first door's traffic.
fn stand_preload_name(path: &str) -> Option<std::os::unix::net::UnixListener> {
    let _ = std::fs::remove_file(path);
    match std::os::unix::net::UnixListener::bind(path) {
        Ok(listener) => {
            if listener.set_nonblocking(true).is_err() {
                let _ = std::fs::remove_file(path);
                return None;
            }
            Some(listener)
        }
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"state_fault": format!("preload name unavailable: {error}")})
            );
            None
        }
    }
}

/// **The credential judgment inverts the first door's**, per
/// `weaver-state-Spec` section 4: this accept refuses a peer bearing the
/// agent's uid before any byte is read, and admits the operator principal.
/// The first door admits exactly one uid and this one refuses exactly one,
/// which is what keeps the agent from reaching a door that exists to carry
/// the operator's own record into custody.
fn admit_operator(
    listener: &std::os::unix::net::UnixListener,
    agent_uid: u32,
) -> Option<std::os::unix::net::UnixStream> {
    let (channel, _address) = listener.accept().ok()?;
    match nix::sys::socket::getsockopt(&channel, nix::sys::socket::sockopt::PeerCredentials) {
        Ok(credentials) if credentials.uid() == agent_uid => {
            eprintln!(
                "{}",
                serde_json::json!({
                    "state_fault": "preload dial refused: the agent's own uid may not preload"
                })
            );
            None
        }
        Ok(_) => {
            if channel.set_nonblocking(true).is_err() {
                return None;
            }
            Some(channel)
        }
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"state_fault": format!("credential read failed: {error}")})
            );
            None
        }
    }
}

/// Whether a preload frame is the seal: one empty frame after the last
/// distillate, per `weaver-analysis-state-contract` section 2. Empty means
/// an empty JSON object, which is what distinguishes a seal from a blank
/// line a sender's framing left behind.
fn is_seal(line: &str) -> bool {
    match serde_json::from_str::<serde_json::Value>(line) {
        Ok(serde_json::Value::Object(members)) => members.is_empty(),
        _ => false,
    }
}

/// Stand the name, wait for the one peer, and judge it by credential. A
/// peer holding the wrong uid is closed unread and the name keeps
/// listening until the deadline, because the socket's mode is open on
/// purpose - the filesystem is not the gate here, the credential check is,
/// per the contract's authentication clause - and a stranger dialing first
/// must not cost the worker its leg. The name is unlinked once the right
/// peer is accepted, so the standing seam has exactly two ends.
fn stand_and_accept(socket: &str, peer_uid: u32) -> Option<std::os::unix::net::UnixStream> {
    let path = std::path::Path::new(socket);
    let _ = std::fs::remove_file(path);
    let listener = match std::os::unix::net::UnixListener::bind(path) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"state_fault": format!("bind failed: {error}")})
            );
            return None;
        }
    };
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o666));

    let deadline = std::time::Instant::now()
        + std::time::Duration::from_millis(u64::from(ACCEPT_WAIT_MS));
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            // The load never came. An abandoned name is removed and the
            // empty stand is the honest outcome.
            let _ = std::fs::remove_file(path);
            return None;
        }
        let wait = remaining.as_millis().min(u128::from(u16::MAX)) as u16;
        let mut fds = [nix::poll::PollFd::new(
            listener.as_fd(),
            nix::poll::PollFlags::POLLIN,
        )];
        match nix::poll::poll(&mut fds, wait) {
            Ok(0) => continue,
            Ok(_) => {}
            Err(nix::errno::Errno::EINTR) => continue,
            Err(_) => {
                let _ = std::fs::remove_file(path);
                return None;
            }
        }
        let Ok((channel, _address)) = listener.accept() else {
            continue;
        };
        // The credential judgment, per the first invariant's rule for a
        // channel with a name.
        match nix::sys::socket::getsockopt(&channel, nix::sys::socket::sockopt::PeerCredentials) {
            Ok(credentials) if credentials.uid() == peer_uid => {
                let _ = std::fs::remove_file(path);
                return Some(channel);
            }
            Ok(credentials) => {
                eprintln!(
                    "{}",
                    serde_json::json!({
                        "state_fault": format!(
                            "peer uid {} is not the expected {peer_uid}",
                            credentials.uid()
                        )
                    })
                );
            }
            Err(error) => {
                eprintln!(
                    "{}",
                    serde_json::json!({"state_fault": format!("credential read failed: {error}")})
                );
            }
        }
    }
}

/// The session the opener names, per the contract's `election` term as
/// amended 2026-08-20. Absent where the frame does not parse or carries no
/// session, which the caller reads as the empty session.
fn parse_session(line: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    Some(value.get("session")?.as_str()?.to_string())
}

/// The opener's shape: `{"election":{"all_kinds":true,"keys":[...]}}`. A
/// malformed opener falls back to the default election, the envelope of
/// every kind, which is the contract's default and never a guess.
fn parse_election(line: &str) -> Option<Election> {
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    let election = value.get("election")?;
    let all_kinds = election.get("all_kinds")?.as_bool()?;
    let keys = election
        .get("keys")
        .and_then(|k| k.as_array())
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let kind = entry.get("kind")?.as_str()?.to_string();
                    let paths = entry
                        .get("paths")?
                        .as_array()?
                        .iter()
                        .filter_map(|p| p.as_str().map(str::to_string))
                        .collect();
                    Some((kind, paths))
                })
                .collect()
        })
        .unwrap_or_default();
    Some(Election { all_kinds, keys })
}

/// Newline-delimited reading over the stream, per the seam's provisional
/// JSON encoding.
struct LineReader<'a> {
    stream: &'a mut std::os::unix::net::UnixStream,
    buffer: Vec<u8>,
}

impl<'a> LineReader<'a> {
    fn new(stream: &'a mut std::os::unix::net::UnixStream) -> Self {
        LineReader {
            stream,
            buffer: Vec::new(),
        }
    }

    /// One frame larger than the bound is not the seam's traffic, and the
    /// custodian answers it as closure rather than growing without bound:
    /// the peer holds a credential, not a license to exhaust this process.
    const FRAME_BOUND: usize = 8 * 1024 * 1024;

    /// Write one answer frame back on the channel, whole inside the write
    /// bound or reporting the seam broken: the serve direction's one write
    /// site, used only when asked, per the contract. The wait rides a poll
    /// deadline because a blocking write against a peer that stopped
    /// reading would wedge custody for the session's life.
    fn respond(&mut self, bytes: &[u8]) -> bool {
        use std::io::Write;
        use std::os::fd::AsFd;
        let deadline = std::time::Instant::now()
            + std::time::Duration::from_millis(u64::from(RESPOND_WAIT_MS));
        let mut sent = 0;
        while sent < bytes.len() {
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                return false;
            }
            let wait = remaining.as_millis().min(u128::from(u16::MAX)) as u16;
            let mut fds = [nix::poll::PollFd::new(
                self.stream.as_fd(),
                nix::poll::PollFlags::POLLOUT,
            )];
            match nix::poll::poll(&mut fds, wait) {
                Ok(0) => return false,
                Ok(_) => {}
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return false,
            }
            match self.stream.write(&bytes[sent..]) {
                Ok(0) => return false,
                Ok(count) => sent += count,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => return false,
            }
        }
        true
    }

    fn take_line(&mut self) -> Option<String> {
        take_frame(&mut self.buffer)
    }

    fn fill(&mut self) -> bool {
        fill_buffer(self.stream, &mut self.buffer)
    }

    fn next_line(&mut self) -> Option<String> {
        loop {
            if let Some(position) = self.buffer.iter().position(|&b| b == b'\n') {
                let line: Vec<u8> = self.buffer.drain(..=position).collect();
                let text = String::from_utf8_lossy(&line[..line.len() - 1]).into_owned();
                return Some(text);
            }
            if self.buffer.len() > Self::FRAME_BOUND {
                return None;
            }
            let mut chunk = [0u8; 65536];
            match self.stream.read(&mut chunk) {
                Ok(0) | Err(_) => return None,
                Ok(n) => self.buffer.extend_from_slice(&chunk[..n]),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The preload door refuses the agent's own uid**, per
    /// `weaver-state-Spec` section 4: the credential judgment inverts the
    /// first door's, so a peer bearing the uid the first door admits is the
    /// one peer this door turns away, before any byte is read. The dial in
    /// this test carries the running uid, so naming that uid the agent's
    /// makes it the refused peer and naming any other makes it admitted.
    ///
    /// Perturbation: drop the `credentials.uid() == agent_uid` arm from
    /// `admit_operator` and the refusal half fails, the agent admitted to a
    /// door that exists to keep the operator's record out of its reach.
    #[test]
    fn the_preload_door_refuses_the_agent_and_admits_the_operator() {
        let running = nix::unistd::getuid().as_raw();
        for (agent_uid, admitted) in [(running, false), (running.wrapping_add(1), true)] {
            let path = std::env::temp_dir().join(format!(
                "weaver-state-preload-{}-{agent_uid}",
                std::process::id()
            ));
            let path = path.to_string_lossy().into_owned();
            let listener = stand_preload_name(&path).expect("the name stands");
            let _dialer = std::os::unix::net::UnixStream::connect(&path).expect("dials");
            // **Wait for the door to be readable before judging it.** The
            // listener is non-blocking and `admit_operator` answers `None`
            // both for a refused peer and for an accept that would block, so
            // a bare call could report a refusal the credential never made.
            // Polling first makes the accept certain and the `None` mean the
            // one thing this test is about.
            use std::os::fd::AsFd;
            let mut fds = [nix::poll::PollFd::new(
                listener.as_fd(),
                nix::poll::PollFlags::POLLIN,
            )];
            let ready = nix::poll::poll(&mut fds, 5_000u16).expect("the door answers poll");
            assert_eq!(ready, 1, "the dial reaches the door");
            drop(fds);
            let outcome = admit_operator(&listener, agent_uid);
            assert_eq!(
                outcome.is_some(),
                admitted,
                "agent uid {agent_uid} against a running uid of {running}"
            );
            let _ = std::fs::remove_file(&path);
        }
    }

    /// **The door stands only where the party that stands the member names
    /// it**, per `weaver-state-Spec` section 4, and this crate's half of that
    /// is the fourth argument: no name, no door. The kind is the caller's
    /// fact and reaches here as the name's presence alone, which is what
    /// keeps this crate from holding an opinion about the binding.
    ///
    /// Perturbation: give `stand_preload_name` a default path when the
    /// argument is absent and a serving load stands a door nothing should
    /// dial.
    #[test]
    fn no_preload_name_means_no_preload_door() {
        let absent: Option<String> = None;
        assert!(
            absent.as_deref().map(stand_preload_name).is_none(),
            "a serving load names no preload socket and stands no door"
        );
        let path = std::env::temp_dir()
            .join(format!("weaver-state-named-{}", std::process::id()))
            .to_string_lossy()
            .into_owned();
        assert!(
            stand_preload_name(&path).is_some(),
            "and a named one stands"
        );
        let _ = std::fs::remove_file(&path);
    }

    /// The contract's election round trip, section 8: the opener as the
    /// tee renders it parses to the same election on this end, so a
    /// restarted member rebuilds the identical index set.
    #[test]
    fn the_opener_round_trips_across_the_seam() {
        let sent = weaver_trace::Election {
            all_kinds: false,
            keys: vec![weaver_trace::ElectedKind {
                kind: "turn.closed".into(),
                paths: vec!["close".into(), "request.sampling".into()],
            }],
        };
        let opener = weaver_trace::opener("s-1", &sent);
        let received = parse_election(opener.trim_end()).expect("the opener parses");
        assert!(!received.all_kinds);
        assert_eq!(
            received.keys,
            vec![(
                "turn.closed".to_string(),
                vec!["close".to_string(), "request.sampling".to_string()]
            )]
        );
    }

    /// A malformed opener falls to the default election, the envelope of
    /// every kind, which is the contract's default and never a guess.
    #[test]
    fn a_malformed_opener_falls_back_to_the_default() {
        for bad in ["not json", "{}", r#"{"election":{"keys":[]}}"#] {
            let election = parse_election(bad).unwrap_or_default();
            assert!(election.all_kinds, "{bad}");
            assert!(election.keys.is_empty(), "{bad}");
        }
    }

    /// An entry missing its paths member is dropped whole, while an entry
    /// with an empty paths list stands as the meaningful envelope-only
    /// election. The tee always renders paths, so the dropped shape is a
    /// hand-built opener's defect, documented here as the current behavior.
    #[test]
    fn an_entry_without_paths_is_dropped_whole() {
        let opener = concat!(
            r#"{"election":{"all_kinds":false,"keys":["#,
            r#"{"kind":"load"},{"kind":"turn.closed","paths":[]}]}}"#
        );
        let election = parse_election(opener).expect("parses");
        assert_eq!(election.keys, vec![("turn.closed".to_string(), vec![])]);
    }

    /// A distillate as the tee renders it parses whole on this end: the
    /// envelope's five attributable, the elected pair carried verbatim.
    #[test]
    fn a_distillate_crosses_from_tee_to_row() {
        let line = concat!(
            r#"{"session":"alpha-1","run":"r-1","turn":"t-1","kind":"turn.closed","#,
            r#""sequence":"7","subsystem":"harness","wall_ms":1,"monotonic_ns":"2","#,
            r#""payload":{"close":"clean"}}"#
        );
        let election = weaver_trace::Election {
            all_kinds: true,
            keys: vec![weaver_trace::ElectedKind {
                kind: "turn.closed".into(),
                paths: vec!["close".into()],
            }],
        };
        let frame = weaver_trace::distill(line, &election).expect("distills");
        let distillate = parse_distillate(frame.trim_end()).expect("parses");
        assert_eq!(distillate.session, "alpha-1");
        assert_eq!(distillate.run, "r-1");
        assert_eq!(distillate.turn.as_deref(), Some("t-1"));
        assert_eq!(distillate.kind, "turn.closed");
        assert_eq!(distillate.sequence, 7);
        assert_eq!(
            distillate.pairs,
            vec![("close".to_string(), "\"clean\"".to_string())]
        );
    }
}
