//! The member's process: adopt the seam end, judge the peer, read the
//! election, stand the store, and land distillates until the channel
//! closes. Per `weaver-state-Spec` section 2, the seam end arrives as an
//! inherited descriptor the way every stood channel's does, and the
//! territory path arrives as the one argument.

use std::io::Read;
use std::os::fd::{FromRawFd, OwnedFd};

use weaver_state::{Election, Store, parse_distillate};

/// The seam end's descriptor number, the standing pattern of the worker's
/// organ channels: the spawner places the end at the first descriptor
/// beyond the standard three.
const SEAM_DESCRIPTOR: i32 = 3;

fn main() -> std::process::ExitCode {
    let mut arguments = std::env::args().skip(1);
    let Some(territory) = arguments.next() else {
        eprintln!("{}", serde_json::json!({"state_fault": "no territory argument"}));
        return std::process::ExitCode::FAILURE;
    };

    // The inherited end, adopted as owned per the descriptor discipline.
    // SAFETY: the spawner placed the seam end at this descriptor and
    // nothing else in this process claims it.
    let seam = unsafe { OwnedFd::from_raw_fd(SEAM_DESCRIPTOR) };
    let mut channel = std::os::unix::net::UnixStream::from(seam);

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
