//! The member's process: stand the seam's name, judge the peer by
//! credential, read the election, stand the store, and land distillates
//! until the channel closes. Per `weaver-harness-state-contract`, the seam
//! is a Unix socket with a name, authenticated by credential per the first
//! invariant, so the member binds and the worker dials. The arguments are
//! the territory, the socket path, and the one uid the peer may hold.

use std::io::Read;
use std::os::fd::AsFd;

use weaver_state::{Election, Store, parse_distillate};

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
    // per the contract's malformed-row clause. Closure is retirement, the
    // holdings standing for the next run.
    while let Some(line) = lines.next_line() {
        if let Some(distillate) = parse_distillate(&line) {
            let _ = store.land(&distillate);
        }
    }
    std::process::ExitCode::SUCCESS
}

/// Stand the name, wait for the one peer, and judge it by credential. The
/// name is unlinked once the peer is accepted, so the seam has exactly two
/// ends for its whole life. The socket file's mode is opened wide on
/// purpose: the filesystem is not the gate here, the credential check is,
/// per the contract's authentication clause.
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

    let mut fds = [nix::poll::PollFd::new(
        listener.as_fd(),
        nix::poll::PollFlags::POLLIN,
    )];
    match nix::poll::poll(&mut fds, ACCEPT_WAIT_MS) {
        Ok(0) => {
            // The load never came. An abandoned name is removed and the
            // empty stand is the honest outcome.
            let _ = std::fs::remove_file(path);
            return None;
        }
        Ok(_) => {}
        Err(_) => {
            let _ = std::fs::remove_file(path);
            return None;
        }
    }
    let (channel, _address) = match listener.accept() {
        Ok(accepted) => accepted,
        Err(error) => {
            eprintln!(
                "{}",
                serde_json::json!({"state_fault": format!("accept failed: {error}")})
            );
            let _ = std::fs::remove_file(path);
            return None;
        }
    };
    let _ = std::fs::remove_file(path);

    // The credential judgment, per the first invariant's rule for a channel
    // with a name: the peer holds the one expected uid or the channel
    // closes unread.
    let credentials =
        nix::sys::socket::getsockopt(&channel, nix::sys::socket::sockopt::PeerCredentials);
    match credentials {
        Ok(credentials) if credentials.uid() == peer_uid => Some(channel),
        Ok(credentials) => {
            eprintln!(
                "{}",
                serde_json::json!({
                    "state_fault":
                        format!("peer uid {} is not the expected {peer_uid}", credentials.uid())
                })
            );
            None
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

    fn next_line(&mut self) -> Option<String> {
        loop {
            if let Some(position) = self.buffer.iter().position(|&b| b == b'\n') {
                let line: Vec<u8> = self.buffer.drain(..=position).collect();
                let text = String::from_utf8_lossy(&line[..line.len() - 1]).into_owned();
                return Some(text);
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

    /// A distillate as the tee renders it parses whole on this end: the
    /// envelope's five attributable, the elected pair carried verbatim.
    #[test]
    fn a_distillate_crosses_from_tee_to_row() {
        let line = concat!(
            r#"{"session":"alpha-1","run":"r-1","turn":"t-1","kind":"turn.closed","#,
            r#""sequence":7,"subsystem":"harness","wall_ms":1,"monotonic_ns":2,"#,
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
