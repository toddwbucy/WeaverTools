//! The member's process: stand the seam's name, judge the peer by
//! credential, read the election, stand the store, and land distillates
//! until the channel closes. Per `weaver-harness-state-contract`, the seam
//! is a Unix socket with a name, authenticated by credential per the first
//! invariant, so the member binds and the worker dials. The arguments are
//! the territory, the socket path, and the one uid the peer may hold.

use std::io::Read;
use std::os::fd::AsFd;

use weaver_state::{Election, Store, is_shape_ask, parse_distillate, render_shape_answer};

/// How long the name waits for its one peer before concluding the load
/// never came, so an abandoned member is a bounded cost rather than a
/// resident one.
const ACCEPT_WAIT_MS: u16 = 60_000;

fn main() -> std::process::ExitCode {
    let mut arguments = std::env::args().skip(1);
    let (Some(territory), Some(socket), Some(peer)) =
        (arguments.next(), arguments.next(), arguments.next())
    else {
        eprintln!(
            "{}",
            serde_json::json!({"state_fault": "usage: weaver-state <territory> <socket> <peer-uid>"})
        );
        return std::process::ExitCode::FAILURE;
    };
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
    let election = parse_election(&opener).unwrap_or_default();
    if let Err(fault) = store.index_election(&election) {
        eprintln!("{}", serde_json::json!({"state_fault": format!("{fault:?}")}));
        return std::process::ExitCode::FAILURE;
    }

    // Custody until closure: parse, land whole, drop what does not parse,
    // per the contract's malformed-row clause, and answer the shape ask in
    // stream order, which is what delivers the contract's answered-against
    // clause - nothing lands between reading an ask and answering it.
    // Closure is retirement, the holdings standing for the next run.
    while let Some(line) = lines.next_line() {
        if let Some(distillate) = parse_distillate(&line) {
            let _ = store.land(&distillate);
        } else if is_shape_ask(&line) {
            // A store that cannot answer is silence the harness's bound
            // converts, per the contract: custody never invents an answer
            // shape for a fault.
            if let Ok(shape) = store.shape()
                && !lines.respond(render_shape_answer(&shape).as_bytes())
            {
                break;
            }
        }
    }
    std::process::ExitCode::SUCCESS
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

    /// Write one answer frame back on the channel, whole or reporting the
    /// seam broken: the serve direction's one write site, used only when
    /// asked, per the contract.
    fn respond(&mut self, bytes: &[u8]) -> bool {
        use std::io::Write;
        self.stream.write_all(bytes).is_ok()
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
        let opener = weaver_trace::opener(&sent);
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
