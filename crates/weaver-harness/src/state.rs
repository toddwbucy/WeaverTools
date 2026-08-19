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
        let line = self.await_line()?;
        parse_shape_answer(&line)
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
    fn await_line(&mut self) -> Option<String> {
        let deadline = std::time::Instant::now()
            + std::time::Duration::from_millis(u64::from(ANSWER_BOUND_MS));
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
            let wait = remaining.as_millis().min(u128::from(u16::MAX)) as u16;
            let mut fds = [nix::poll::PollFd::new(
                self.channel.as_fd(),
                nix::poll::PollFlags::POLLIN,
            )];
            match nix::poll::poll(&mut fds, wait) {
                Ok(0) => return None,
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
}
