//! The tee, per `weaver-trace-PRD` section 11: the distillation surface over
//! the canonical event stream. The mechanism is this crate's because what is
//! tee'd is this crate's own rendering, and the harness applies it as the one
//! party that writes. The tee selects and never computes, per the three-way
//! division of `weaver-state-PRD` section 2: the envelope always rides, the
//! election ranges over payload key paths alone, and an event the election
//! does not match costs nothing.

use std::collections::BTreeMap;
use std::io::Write;
use std::os::unix::net::UnixStream;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// The election, as `weaver-harness-state-contract` defines the term: the
/// elected kinds and their payload key paths, fixed at load. The default is
/// the envelope of every kind and nothing more, per `weaver-trace-PRD`
/// section 11, so a deployment that elects nothing still holds the session's
/// shape and pays for no payload it never asked to keep.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Election {
    /// Every kind crosses with its envelope. When false, only the kinds
    /// named in `keys` cross at all.
    pub all_kinds: bool,
    /// Payload key paths per kind, each path dotted from the payload root,
    /// on top of the envelope. An entry with no paths is a meaningful
    /// election: presence itself is state.
    pub keys: Vec<ElectedKind>,
}

/// One kind's election: the kind as the canonical form spells it, and the
/// payload key paths elected for it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ElectedKind {
    pub kind: String,
    pub paths: Vec<String>,
}

impl Default for Election {
    fn default() -> Self {
        Election {
            all_kinds: true,
            keys: Vec::new(),
        }
    }
}

/// The seam's opener frame, sent whole at every standing of the channel and
/// never per event, per the contract's ingest clause: the custodian builds
/// its indexes from it before the first distillate.
pub fn opener(election: &Election) -> String {
    #[derive(Serialize)]
    struct Opener<'a> {
        election: &'a Election,
    }
    let mut frame = serde_json::to_string(&Opener { election }).expect("the election renders");
    frame.push('\n');
    frame
}

/// The canonical line's face as the tee reads it: the envelope's five by
/// name, and the payload untouched as raw text so what crosses is the value
/// the canonical JSON held, never a re-rendering of it.
#[derive(Deserialize)]
struct CanonicalLine<'a> {
    session: &'a str,
    run: &'a str,
    turn: Option<&'a str>,
    kind: &'a str,
    sequence: i64,
    #[serde(borrow)]
    payload: Option<&'a RawValue>,
}

/// The distillate frame, per the contract: the envelope whole, the elected
/// pairs beside it, each value spelled as the canonical JSON spelled it.
#[derive(Serialize)]
struct Frame<'a> {
    envelope: FrameEnvelope<'a>,
    pairs: BTreeMap<&'a str, &'a RawValue>,
}

#[derive(Serialize)]
struct FrameEnvelope<'a> {
    session: &'a str,
    run: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    turn: Option<&'a str>,
    kind: &'a str,
    sequence: i64,
}

/// Distill one canonical line under the election. Selection only: `None`
/// means the election did not match and the event costs nothing, the trace
/// remaining complete regardless because the tee reads the stream and never
/// thins it. A line this crate rendered always parses, so a parse failure
/// here is unreachable in custody and answered by not distilling.
pub fn distill(line: &str, election: &Election) -> Option<String> {
    let event: CanonicalLine = serde_json::from_str(line).ok()?;
    let elected = election.keys.iter().find(|entry| entry.kind == event.kind);
    if !election.all_kinds && elected.is_none() {
        return None;
    }
    let mut pairs = BTreeMap::new();
    if let (Some(entry), Some(payload)) = (elected, event.payload) {
        for path in &entry.paths {
            if let Some(value) = value_at(payload, path) {
                pairs.insert(path.as_str(), value);
            }
        }
    }
    let mut frame = serde_json::to_string(&Frame {
        envelope: FrameEnvelope {
            session: event.session,
            run: event.run,
            turn: event.turn,
            kind: event.kind,
            sequence: event.sequence,
        },
        pairs,
    })
    .expect("the distillate renders");
    frame.push('\n');
    Some(frame)
}

/// Walk a dotted key path through raw JSON, returning the raw text at the
/// leaf. Each step reparses one object's keys and nothing else, so the value
/// that crosses is byte-identical to the canonical form's spelling. A path
/// the payload does not hold is simply absent from the pairs: a miss costs
/// nothing, per the charter.
fn value_at<'a>(payload: &'a RawValue, path: &str) -> Option<&'a RawValue> {
    let mut cursor = payload;
    for segment in path.split('.') {
        let object: BTreeMap<&str, &RawValue> = serde_json::from_str(cursor.get()).ok()?;
        cursor = object.get(segment)?;
    }
    Some(cursor)
}

/// The applied tee: the election and the seam's end, held by the recorder
/// once the harness attaches it at load. Opening writes the election as the
/// channel's first traffic, per the contract.
pub struct Tee {
    election: Election,
    channel: UnixStream,
}

impl Tee {
    /// Stand the tee on a channel: the opener goes first, and the stream is
    /// set nonblocking because the contract forbids backpressure onto the
    /// turn path in any form.
    pub fn open(channel: UnixStream, election: Election) -> std::io::Result<Tee> {
        channel.set_nonblocking(true)?;
        let mut tee = Tee { election, channel };
        if !tee.send(opener(&tee.election).as_bytes()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "the state seam refused the opener",
            ));
        }
        Ok(tee)
    }

    /// Feed one canonical line. `false` means the seam is broken and the
    /// tee is done: the cheapest honest answer is to stop distilling until
    /// the next load, per the contract's dead-peer clause. The distillate
    /// is lost, never the turn.
    pub fn feed(&mut self, line: &str) -> bool {
        match distill(line, &self.election) {
            Some(frame) => self.send(frame.as_bytes()),
            None => true,
        }
    }

    /// Write one frame whole or give the seam up. A peer whose buffer is
    /// full has stalled for longer than any healthy custodian ever is, and
    /// waiting on it would be backpressure, so `WouldBlock` is treated as
    /// the same breakage a closed peer is.
    fn send(&mut self, mut bytes: &[u8]) -> bool {
        while !bytes.is_empty() {
            match self.channel.write(bytes) {
                Ok(0) => return false,
                Ok(written) => bytes = &bytes[written..],
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINE: &str = concat!(
        r#"{"session":"alpha-1","run":"r-1","turn":"t-1","kind":"turn.closed","#,
        r#""sequence":7,"subsystem":"harness","wall_ms":1,"monotonic_ns":2,"#,
        r#""payload":{"close":"clean","request":{"sampling":{"temperature":0.7}}}}"#
    );

    /// The default election: the envelope of every kind and nothing more.
    /// The frame carries the five, spelled as the canonical form spelled
    /// them, and an empty pairs object.
    #[test]
    fn the_default_election_is_the_envelope_of_every_kind() {
        let frame = distill(LINE, &Election::default()).expect("distills");
        assert_eq!(
            frame,
            concat!(
                r#"{"envelope":{"session":"alpha-1","run":"r-1","turn":"t-1","#,
                r#""kind":"turn.closed","sequence":7},"pairs":{}}"#,
                "\n"
            )
        );
    }

    /// An elected payload key path crosses with the value the canonical
    /// JSON held at it, byte for byte, nesting included, and a path the
    /// payload does not hold is simply absent.
    #[test]
    fn elected_paths_cross_verbatim_and_misses_cost_nothing() {
        let election = Election {
            all_kinds: true,
            keys: vec![ElectedKind {
                kind: "turn.closed".into(),
                paths: vec![
                    "close".into(),
                    "request.sampling".into(),
                    "absent.path".into(),
                ],
            }],
        };
        let frame = distill(LINE, &election).expect("distills");
        assert!(frame.contains(r#""close":"clean""#), "{frame}");
        assert!(
            frame.contains(r#""request.sampling":{"temperature":0.7}"#),
            "{frame}"
        );
        assert!(!frame.contains("absent"), "{frame}");
    }

    /// With `all_kinds` down, a kind the election does not name does not
    /// cross at all, and costs nothing.
    #[test]
    fn an_unelected_kind_is_not_distilled() {
        let election = Election {
            all_kinds: false,
            keys: vec![ElectedKind {
                kind: "turn.started".into(),
                paths: Vec::new(),
            }],
        };
        assert!(distill(LINE, &election).is_none());
    }

    /// A turnless event distills without a turn member, mirroring the
    /// canonical form's own omission.
    #[test]
    fn a_turnless_event_stays_turnless() {
        let line = r#"{"session":"s","run":"r","kind":"run.load","sequence":0,"wall_ms":1}"#;
        let frame = distill(line, &Election::default()).expect("distills");
        assert!(!frame.contains("turn"), "{frame}");
    }
}
