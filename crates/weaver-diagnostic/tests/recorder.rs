//! conforms: diagnostic-canonical-form-follows-trace
//! conforms: diagnostic-session-is-the-replays-own
//! conforms: diagnostic-admission-precedes-the-write
//!
//! The writer's perturbation watches, per `weaver-diagnostic-Spec` section 7.
//! Each names the removal it was watched failing under, per the corpus's
//! perturbation doctrine: a test never seen red proves nothing about the
//! property it claims.

use std::io::Read;
use std::os::fd::OwnedFd;

use serde_json::value::RawValue;
use weaver_diagnostic::{
    Envelope, Event, Kind, MonotonicNs, Payload, Recorder, RunRef, Sequence, SessionRef,
    SubmitRefusal, Subsystem, TurnRef,
};

/// A recorder over a fresh temp file, with the file readable back beside it.
fn stand(session: &str, run: &str) -> (Recorder, std::fs::File) {
    let file = tempfile();
    let read_back = file.try_clone().expect("clone");
    let recorder = Recorder::receive(
        OwnedFd::from(file),
        RunRef(run.into()),
        SessionRef(session.into()),
    )
    .expect("receive");
    (recorder, read_back)
}

fn tempfile() -> std::fs::File {
    let dir = std::env::temp_dir();
    let path = dir.join(format!(
        "weaver-diagnostic-test-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&path)
        .expect("temp file");
    std::fs::remove_file(&path).expect("unlink");
    file
}

fn contents(read_back: &mut std::fs::File) -> String {
    use std::io::Seek;
    read_back.rewind().expect("rewind");
    let mut s = String::new();
    read_back.read_to_string(&mut s).expect("read");
    s
}

fn envelope(kind: Kind, turn: Option<&str>, wall_ms: u64, mono: u64) -> Envelope {
    Envelope {
        // Bound by the recorder at submit; these values are overwritten.
        session: SessionRef("caller-supplied".into()),
        run: RunRef("caller-supplied".into()),
        turn: turn.map(|t| TurnRef(t.into())),
        sequence: Sequence(0),
        kind,
        subsystem: Subsystem::Harness,
        causal_parent: None,
        wall_ms,
        monotonic_ns: MonotonicNs(mono),
    }
}

/// Two lines lifted verbatim from a serving record this box holds, the raw
/// sink at `~/.weaveragents/karl/trace.ndjson`, run
/// `2026-08-29T21:58:12.937Z-karl-1366ba970737be0b` of the loop-1 smoke of
/// 2026-08-29 - the harness's own canonical bytes, not a driver's
/// re-serialization, which pads separators with spaces and would fail this
/// comparison for the wrong reason.
const SERVING_TURN_STARTED: &str = "{\"session\":\"s-karl-1\",\"run\":\"2026-08-29T21:58:12.937Z-karl-1366ba970737be0b\",\"turn\":\"t-1\",\"sequence\":\"2\",\"kind\":\"turn.started\",\"subsystem\":\"harness\",\"wall_ms\":1788040694104,\"monotonic_ns\":\"1166347120\"}";
const SERVING_MESSAGE_USER: &str = "{\"session\":\"s-karl-1\",\"run\":\"2026-08-29T21:58:12.937Z-karl-1366ba970737be0b\",\"turn\":\"t-1\",\"sequence\":\"3\",\"kind\":\"message.user\",\"subsystem\":\"harness\",\"wall_ms\":1788040694104,\"monotonic_ns\":\"1166362689\",\"payload\":{\"role\":\"user\",\"content\":[{\"type\":\"text\",\"text\":\"In one short sentence, name a colour and nothing else.\"}]}}";

/// Canonical form follows `weaver-trace-Spec` section 2: a shared kind's
/// line compared byte for byte against the same event's serving line.
///
/// Perturbation: reorder any two [`Envelope`] fields, or render `sequence`
/// or `monotonic_ns` as a bare number, and this fails. Watched failing under
/// exactly that: `monotonic_ns` rendered bare diverges at the first byte of
/// its value.
#[test]
fn canonical_form_follows_the_serving_line() {
    let (mut recorder, mut read_back) =
        stand("s-karl-1", "2026-08-29T21:58:12.937Z-karl-1366ba970737be0b");
    // Two padding events walk the gapless sequence to the fixtures' own
    // ordinals, so the comparison is byte-whole rather than sequence-edited.
    for _ in 0..2 {
        recorder
            .submit(Event {
                envelope: envelope(Kind::TurnStarted, Some("t-0"), 1, 1),
                payload: None,
            })
            .expect("padding");
    }
    recorder
        .submit(Event {
            envelope: envelope(Kind::TurnStarted, Some("t-1"), 1788040694104, 1166347120),
            payload: None,
        })
        .expect("turn.started");
    let spliced = RawValue::from_string(
        "{\"role\":\"user\",\"content\":[{\"type\":\"text\",\"text\":\"In one short sentence, name a colour and nothing else.\"}]}".into(),
    )
    .expect("raw");
    recorder
        .submit(Event {
            envelope: envelope(Kind::MessageUser, Some("t-1"), 1788040694104, 1166362689),
            payload: Some(Payload::Spliced(spliced)),
        })
        .expect("message.user");
    let written = contents(&mut read_back);
    let lines: Vec<&str> = written.lines().collect();
    assert_eq!(lines[2], SERVING_TURN_STARTED, "turn.started diverges");
    assert_eq!(lines[3], SERVING_MESSAGE_USER, "message.user diverges");
}

/// The session is the replay's own: the envelope renders under the
/// diagnostic run's session whatever the caller supplied, and the replayed
/// name appears in the identity payload alone.
///
/// Perturbation: remove the binding in `submit` and the caller's
/// `caller-supplied` label reaches the record. Watched failing under
/// exactly that removal.
#[test]
fn the_session_is_the_replays_own() {
    let (mut recorder, mut read_back) = stand("s-diag-1", "r-diag-1");
    recorder
        .submit(Event {
            envelope: envelope(Kind::TurnStarted, None, 1, 1),
            payload: None,
        })
        .expect("submit");
    let written = contents(&mut read_back);
    assert!(written.contains("\"session\":\"s-diag-1\""), "not bound");
    assert!(written.contains("\"run\":\"r-diag-1\""), "run not bound");
    assert!(!written.contains("caller-supplied"), "caller label leaked");
}

/// Admission precedes the write: a refused submission leaves the sink
/// untouched and consumes no sequence.
///
/// Perturbation: move the `admit` call after the write and the refused
/// line lands on the sink. Watched failing under exactly that move.
#[test]
fn admission_precedes_the_write() {
    let (mut recorder, mut read_back) = stand("s-diag-1", "r-diag-1");
    let refused = recorder.submit(Event {
        envelope: envelope(Kind::ModelOutput, Some("t-1"), 1, 1),
        payload: None,
    });
    match refused {
        Err(weaver_diagnostic::Failure::SubmitRefused {
            refusal: SubmitRefusal::RequiredFieldAbsent { .. },
        }) => {}
        other => panic!("expected a required-field refusal, got {other:?}"),
    }
    assert_eq!(contents(&mut read_back), "", "the refusal touched the sink");
    let sequence = recorder
        .submit(Event {
            envelope: envelope(Kind::TurnStarted, None, 1, 1),
            payload: None,
        })
        .expect("the record survives a refusal");
    assert_eq!(sequence, Sequence(0), "the refusal consumed a sequence");
}

/// The kind-to-payload pairing refuses a mismatch before the sink, the
/// other half of admission: a typed payload under a spliced kind.
#[test]
fn a_mismatched_pairing_refuses() {
    let (mut recorder, mut read_back) = stand("s-diag-1", "r-diag-1");
    let refused = recorder.submit(Event {
        envelope: envelope(Kind::ModelOutput, Some("t-1"), 1, 1),
        payload: Some(Payload::ReplayOpened(weaver_diagnostic::ReplayOpened {
            reader_elected: false,
        })),
    });
    assert!(
        matches!(
            refused,
            Err(weaver_diagnostic::Failure::SubmitRefused {
                refusal: SubmitRefusal::PayloadKindMismatch
            })
        ),
        "expected a pairing refusal, got {refused:?}"
    );
    assert_eq!(contents(&mut read_back), "", "the refusal touched the sink");
}
