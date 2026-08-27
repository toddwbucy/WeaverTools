//! The state seam's ask end, per `weaver-harness-Spec` section 6: a clone
//! of the standing state channel held on the run beside the tee the enter
//! attaches, the ask written and the answer awaited on the serving thread
//! inside a bound this crate elects. Serialization on the shared channel is
//! the serving thread itself - the tee's feed and this ask both run on it,
//! so no ask can interleave a distillate's octets - and the member's
//! answers are the only traffic that ever flows toward this crate, so
//! whatever the wait reads is the answer or is malformed, with no third
//! case to disambiguate.

use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::os::unix::net::UnixStream;

/// The bound on the answer wait, per the contract's missing-answer clause:
/// generous against a member whose answer is one pass over its own
/// holdings, and an expiry is the dead peer converted into the same absence
/// a missing leg serves.
const ANSWER_BOUND_MS: u16 = 2_000;

/// The bound on the answer's size, matched by the member's own cap on
/// what it renders: an answer still unframed past this many octets is not
/// the seam's traffic, and the seam retires rather than growing the turn
/// path's memory on a peer's behavior.
const ANSWER_BOUND_BYTES: usize = 1024 * 1024;

/// The session's shape as the member answered it, per
/// `weaver-harness-state-contract` section 2: the runs in the order custody
/// first saw them, each with its held event counts by kind. The counts are
/// organized envelope fact and carry no judgment - what a count means to a
/// turn is the asking loop's business.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionShape {
    pub runs: Vec<RunShape>,
}

/// One run's shape: its reference and its event counts by kind, spelled as
/// the envelope spelled them.
#[derive(Debug, Clone, PartialEq)]
pub struct RunShape {
    pub run: String,
    pub kinds: Vec<(String, u64)>,
}

/// One event off either serving answer, per the contract: the envelope's
/// facts the asking loop composes with, and the elected pairs as custody
/// kept them - the distillate's own shape served back.
///
/// **The run travels beside the turn**, per `diagnostic-replay-loop`
/// section 3, whose walk groups the answer's events by run and turn from
/// their envelopes: two events carrying one turn label in different runs
/// are different events, and a shape that dropped the run would pair them
/// into a generation that never ran. The session does not travel, being
/// constant across any answer by the contract's session-bounded clause and
/// already held by the asker that declared it.
#[derive(Debug, Clone, PartialEq)]
pub struct Recalled {
    pub kind: String,
    pub run: String,
    pub turn: Option<String>,
    pub sequence: String,
    pub pairs: Vec<(String, String)>,
}

/// The harness's end of the serve direction: the ask, the bounded wait, and
/// the parse. Held on the run, granted to the seat, mintable nowhere else.
pub struct StateSeam {
    channel: UnixStream,
    /// The serve direction retires on its first failure: a late answer
    /// after a timed-out ask would be read as the next ask's answer,
    /// mis-attributing its position in the stream, so a seam that failed
    /// one ask answers no later one. The ingest direction is the tee's
    /// and is unaffected.
    dead: bool,
}

impl StateSeam {
    /// Crate-private, the port discipline: the enter constructs it from the
    /// standing channel's clone and nothing outside loop 0 can.
    pub(crate) fn new(channel: UnixStream) -> StateSeam {
        StateSeam {
            channel,
            dead: false,
        }
    }

    /// The shape ask: write the frame, await the answer inside the bound,
    /// parse. `None` is the contract's dead peer at the seat, whether the
    /// leg is down, the write refused, the bound expired, or the answer
    /// malformed, converted into the same absence a missing leg serves,
    /// and the serve direction retires on it.
    pub(crate) fn ask_shape(&mut self) -> Option<SessionShape> {
        if self.dead {
            return None;
        }
        let answered = self.exchange();
        if answered.is_none() {
            self.dead = true;
        }
        answered
    }

    fn exchange(&mut self) -> Option<SessionShape> {
        if !self.send(b"{\"ask\":{\"shape\":{}}}\n") {
            return None;
        }
        let line = self.await_line(u64::from(ANSWER_BOUND_MS))?;
        parse_shape_answer(&line)
    }

    /// The recall ask, per the contract: the conversation as custody holds
    /// it, bounded to the most recent turns where a bound is given. The
    /// same one-strike economics as the shape ask, for the same
    /// mis-attribution reason.
    pub(crate) fn ask_recall(&mut self, last_turns: Option<u64>) -> Option<Vec<Recalled>> {
        if self.dead {
            return None;
        }
        let answered = self.recall_exchange(last_turns);
        if answered.is_none() {
            self.dead = true;
        }
        answered
    }

    /// The replay ask, per the contract and `weaver-harness-Spec` section
    /// 6's 2026-08-24 clause: the session's elected events whole, in
    /// landing order, as the member answered them. **The bound is the
    /// caller's to pass**, because the replay ask is the one whose answer
    /// lawfully waits, parked at an open preload until the seal, and only
    /// the asking loop knows how long a preload is worth waiting on. The
    /// same one-strike economics as the other asks: a bound that expires
    /// retires the serve direction, a late answer otherwise reading as the
    /// next ask's.
    pub(crate) fn ask_replay(&mut self, bound_ms: u64) -> Option<Vec<Recalled>> {
        if self.dead {
            return None;
        }
        let answered = self.replay_exchange(bound_ms);
        if answered.is_none() {
            self.dead = true;
        }
        answered
    }

    fn replay_exchange(&mut self, bound_ms: u64) -> Option<Vec<Recalled>> {
        if !self.send(b"{\"ask\":{\"replay\":{}}}\n") {
            return None;
        }
        let line = self.await_line(bound_ms)?;
        parse_replay_answer(&line)
    }

    fn recall_exchange(&mut self, last_turns: Option<u64>) -> Option<Vec<Recalled>> {
        let ask = match last_turns {
            Some(bound) => format!("{{\"ask\":{{\"recall\":{{\"last-turns\":{bound}}}}}}}\n"),
            None => "{\"ask\":{\"recall\":{}}}\n".to_string(),
        };
        if !self.send(ask.as_bytes()) {
            return None;
        }
        let line = self.await_line(u64::from(ANSWER_BOUND_MS))?;
        parse_recall_answer(&line)
    }

    /// One frame whole or nothing, the tee's own economics: the channel is
    /// nonblocking - the flag rides the shared open file description the
    /// tee set - and a peer that cannot take a frame now is not waited on.
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

    /// Await one line inside the bound. The channel shares the tee's
    /// nonblocking flag, so the wait is a poll deadline rather than a read
    /// timeout.
    fn await_line(&mut self, bound_ms: u64) -> Option<String> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(bound_ms);
        let mut buffer: Vec<u8> = Vec::new();
        loop {
            if let Some(position) = buffer.iter().position(|&b| b == b'\n') {
                return Some(String::from_utf8_lossy(&buffer[..position]).into_owned());
            }
            if buffer.len() > ANSWER_BOUND_BYTES {
                return None;
            }
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                return None;
            }
            let wait = poll_slice(remaining);
            let mut fds = [nix::poll::PollFd::new(
                self.channel.as_fd(),
                nix::poll::PollFlags::POLLIN,
            )];
            match nix::poll::poll(&mut fds, wait) {
                Ok(0) => continue,
                Ok(_) => {}
                Err(nix::errno::Errno::EINTR) => continue,
                Err(_) => return None,
            }
            let mut chunk = [0u8; 65536];
            match self.channel.read(&mut chunk) {
                Ok(0) => return None,
                Ok(count) => buffer.extend_from_slice(&chunk[..count]),
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) if error.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => return None,
            }
        }
    }
}

/// Parse the shape answer, per the contract:
/// `{"answer":{"shape":{"runs":[{"run":...,"kinds":{...}}]}}}`. A frame
/// that does not carry the whole shape is malformed and answers nothing.
fn parse_shape_answer(line: &str) -> Option<SessionShape> {
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    let runs = value.get("answer")?.get("shape")?.get("runs")?.as_array()?;
    let mut shaped = Vec::with_capacity(runs.len());
    for entry in runs {
        let run = entry.get("run")?.as_str()?.to_string();
        let kinds = entry
            .get("kinds")?
            .as_object()?
            .iter()
            .map(|(kind, count)| Some((kind.clone(), count.as_u64()?)))
            .collect::<Option<Vec<_>>>()?;
        shaped.push(RunShape { run, kinds });
    }
    Some(SessionShape { runs: shaped })
}

/// Parse the recall answer, per the contract:
/// `{"answer":{"recall":{"events":[{"envelope":{...},"pairs":{...}}]}}}`.
/// A frame that does not carry the whole shape answers nothing.
/// One poll's timeout out of what remains of the wait. **The clamp bounds
/// one poll and never the wait**: `poll` takes milliseconds in a `u16`, so
/// a bound past that ceiling is served by re-arming against what remains
/// rather than by waiting once and giving up, the deadline check being the
/// only thing that ends the wait. A caller passing the replay ask's
/// generous bound would otherwise be answered `None` at sixty-five seconds
/// by an arithmetic it never asked for.
fn poll_slice(remaining: std::time::Duration) -> u16 {
    remaining.as_millis().min(u128::from(u16::MAX)) as u16
}

/// Parse the replay answer, per the contract:
/// `{"answer":{"replay":{"events":[{"envelope":{...},"pairs":{...}}]}}}`.
/// The recall's shape under the replay's name, and a frame that does not
/// carry the whole shape answers nothing.
fn parse_replay_answer(line: &str) -> Option<Vec<Recalled>> {
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    let events = value
        .get("answer")?
        .get("replay")?
        .get("events")?
        .as_array()?;
    parse_recalled_events(events)
}

/// Parse the recall answer, per the contract:
/// `{"answer":{"recall":{"events":[{"envelope":{...},"pairs":{...}}]}}}`.
/// A frame that does not carry the whole shape answers nothing.
fn parse_recall_answer(line: &str) -> Option<Vec<Recalled>> {
    let value: serde_json::Value = serde_json::from_str(line).ok()?;
    let events = value
        .get("answer")?
        .get("recall")?
        .get("events")?
        .as_array()?;
    parse_recalled_events(events)
}

/// One event list off either serving answer, the distillate's shape read
/// back: envelope fields by name, pairs as raw text.
fn parse_recalled_events(events: &[serde_json::Value]) -> Option<Vec<Recalled>> {
    let mut recalled = Vec::with_capacity(events.len());
    for event in events {
        let envelope = event.get("envelope")?;
        let pairs = event
            .get("pairs")
            .and_then(|p| p.as_object())
            .map(|object| {
                object
                    .iter()
                    .map(|(key, val)| (key.clone(), val.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        recalled.push(Recalled {
            kind: envelope.get("kind")?.as_str()?.to_string(),
            run: envelope.get("run")?.as_str()?.to_string(),
            turn: envelope
                .get("turn")
                .and_then(|t| t.as_str())
                .map(str::to_string),
            sequence: envelope.get("sequence")?.as_str()?.to_string(),
            pairs,
        });
    }
    Some(recalled)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The answer's wire spelling parses to the shape, and a frame missing
    /// any member of it answers nothing.
    #[test]
    fn the_shape_answer_parses_whole_or_not_at_all() {
        let line = concat!(
            r#"{"answer":{"shape":{"runs":[{"run":"r-1","kinds":{"load":1,"#,
            r#""turn.closed":3}},{"run":"r-2","kinds":{"load":1}}]}}}"#
        );
        let shape = parse_shape_answer(line).expect("parses");
        assert_eq!(shape.runs.len(), 2);
        assert_eq!(shape.runs[0].run, "r-1");
        assert!(
            shape.runs[0]
                .kinds
                .contains(&("turn.closed".to_string(), 3))
        );
        for malformed in [
            r#"{"answer":{"shape":{}}}"#,
            r#"{"answer":{}}"#,
            r#"{"answer":{"shape":{"runs":[{"kinds":{}}]}}}"#,
            "not json",
        ] {
            assert!(parse_shape_answer(malformed).is_none(), "{malformed}");
        }
    }

    /// The bounded wait converts a silent peer into a missing answer: a
    /// member that never speaks costs the answer inside the bound, never a
    /// hang. The bound is the constant's, so the test waits it out once.
    #[test]
    fn a_silent_peer_costs_the_answer_inside_the_bound() {
        let (ours, _theirs) = UnixStream::pair().expect("pair");
        ours.set_nonblocking(true).expect("nonblocking");
        let mut seam = StateSeam::new(ours);
        let started = std::time::Instant::now();
        assert!(seam.ask_shape().is_none());
        assert!(started.elapsed() < std::time::Duration::from_secs(10));
    }

    /// The serve direction retires on its first failure: a malformed
    /// answer costs its ask, and a well-formed answer already waiting
    /// behind it is never read, because attributing it to a later ask
    /// would misplace its position in the stream.
    #[test]
    fn the_seam_retires_on_its_first_failure() {
        let (ours, theirs) = UnixStream::pair().expect("pair");
        ours.set_nonblocking(true).expect("nonblocking");
        let mut seam = StateSeam::new(ours);
        let mut peer = theirs;
        peer.write_all(b"garbage\n{\"answer\":{\"shape\":{\"runs\":[]}}}\n")
            .expect("answers in advance");
        assert!(seam.ask_shape().is_none(), "a malformed answer misses");
        assert!(
            seam.ask_shape().is_none(),
            "the retired seam never reads the late well-formed answer"
        );
    }

    /// An answer still unframed past the byte bound retires the seam
    /// inside the bound rather than growing the turn path's memory.
    #[test]
    fn an_unframed_flood_retires_the_seam_inside_the_bound() {
        let (ours, theirs) = UnixStream::pair().expect("pair");
        ours.set_nonblocking(true).expect("nonblocking");
        let mut seam = StateSeam::new(ours);
        let responder = std::thread::spawn(move || {
            let mut peer = theirs;
            let mut taken = [0u8; 256];
            let _ = peer.read(&mut taken).expect("reads the ask");
            let flood = vec![b'x'; 2 * ANSWER_BOUND_BYTES];
            let _ = peer.write_all(&flood);
        });
        let started = std::time::Instant::now();
        assert!(seam.ask_shape().is_none());
        assert!(started.elapsed() < std::time::Duration::from_secs(10));
        drop(seam);
        responder.join().expect("responder");
    }

    /// A peer that answers is read whole across the seam.
    #[test]
    fn an_answering_peer_is_read() {
        let (ours, theirs) = UnixStream::pair().expect("pair");
        ours.set_nonblocking(true).expect("nonblocking");
        let mut seam = StateSeam::new(ours);
        let responder = std::thread::spawn(move || {
            let mut peer = theirs;
            let mut taken = [0u8; 256];
            let count = peer.read(&mut taken).expect("reads the ask");
            assert_eq!(&taken[..count], b"{\"ask\":{\"shape\":{}}}\n");
            peer.write_all(
                b"{\"answer\":{\"shape\":{\"runs\":[{\"run\":\"r-1\",\"kinds\":{\"load\":1}}]}}}\n",
            )
            .expect("answers");
        });
        let shape = seam.ask_shape().expect("answered");
        assert_eq!(shape.runs[0].run, "r-1");
        responder.join().expect("responder");
    }

    /// **A bound past the poll's ceiling is a slice and not the whole
    /// wait**, per the replay port's clause: `poll` takes a `u16` of
    /// milliseconds, and a wait that ended at that ceiling would answer
    /// `None` at sixty-five seconds to a caller who asked for longer -
    /// on precisely the ask whose clause exists so the loop may wait as
    /// long as a preload is worth.
    ///
    /// This pins the arithmetic half. The behavioural half, that a
    /// clamped poll's expiry re-arms rather than ending the wait, costs
    /// sixty-five seconds of wall clock to watch and is not bought here;
    /// it was measured live against this code at the review seat, an
    /// `ask_replay(120_000)` answering `None` at 65.551 seconds before
    /// the re-arm landed.
    #[test]
    fn a_bound_past_the_ceiling_is_a_slice() {
        use std::time::Duration;
        assert_eq!(poll_slice(Duration::from_millis(120_000)), u16::MAX);
        assert_eq!(poll_slice(Duration::from_millis(500)), 500);
        assert_eq!(poll_slice(Duration::from_millis(0)), 0);
    }

    /// **The recall answer parses under its own name**, the sibling of the
    /// replay parse and the other caller of the shared event read: the
    /// envelope's facts including the run, the pairs as custody kept them,
    /// and a frame under the replay's name is not a recall answer. Nothing
    /// watched this path before the audit of 2026-08-26 found the shared
    /// refactor unwatched on the recall side.
    #[test]
    fn the_recall_answer_parses_under_its_own_name() {
        let events = parse_recall_answer(concat!(
            r#"{"answer":{"recall":{"events":[{"envelope":{"session":"s","run":"r-2","#,
            r#""turn":"t-9","kind":"message.user","sequence":"11"},"pairs":{}}]}}}"#
        ))
        .expect("parses");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].run, "r-2");
        assert_eq!(events[0].turn.as_deref(), Some("t-9"));
        assert_eq!(events[0].kind, "message.user");
        assert!(
            parse_recall_answer(r#"{"answer":{"replay":{"events":[]}}}"#).is_none(),
            "the two answers stay apart by name"
        );
        assert!(
            parse_recall_answer(concat!(
                r#"{"answer":{"recall":{"events":[{"envelope":{"session":"s",""#,
                r#"turn":"t-9","kind":"message.user","sequence":"11"},"pairs":{}}]}}}"#
            ))
            .is_none(),
            "an envelope missing the run fails the parse whole"
        );
    }

    /// **The replay ask crosses under its own name and reads the answer's
    /// shape back**, per `weaver-harness-Spec` section 6's 2026-08-24
    /// clause: the wire ask is the contract's, the caller's bound is the
    /// wait, and the answer parses as the distillate's shape served back,
    /// envelope and pairs, whole or not at all. A frame under the recall's
    /// name is not a replay answer, which is what pins the two parses
    /// apart.
    #[test]
    fn the_replay_ask_crosses_and_its_answer_parses() {
        let (ours, theirs) = UnixStream::pair().expect("pair");
        ours.set_nonblocking(true).expect("nonblocking");
        let mut seam = StateSeam::new(ours);
        let responder = std::thread::spawn(move || {
            let mut peer = theirs;
            let mut taken = [0u8; 256];
            let count = peer.read(&mut taken).expect("reads the ask");
            assert_eq!(&taken[..count], b"{\"ask\":{\"replay\":{}}}\n");
            peer.write_all(
                b"{\"answer\":{\"replay\":{\"events\":[{\"envelope\":{\"session\":\"s\",\
                \"run\":\"r-1\",\"kind\":\"model.output\",\"sequence\":\"4\"},\
                \"pairs\":{}}]}}}\n",
            )
            .expect("answers");
        });
        let events = seam.ask_replay(2_000).expect("answered");
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].kind, "model.output");
        assert_eq!(events[0].run, "r-1");
        assert_eq!(events[0].sequence, "4");
        responder.join().expect("responder");

        // The recall's name does not parse as a replay answer.
        assert!(
            parse_replay_answer("{\"answer\":{\"recall\":{\"events\":[]}}}").is_none(),
            "the two answers stay apart by name"
        );
    }
}
