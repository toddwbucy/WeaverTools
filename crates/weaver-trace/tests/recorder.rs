//! conforms: trace-one-line-per-event
//! conforms: trace-large-integers-as-decimal-strings
//! conforms: trace-admission-precedes-fan-out
//! conforms: trace-one-rendering-two-holders
//! conforms: trace-sequence-gapless
//! conforms: trace-measurement-absent-not-zero
//! conforms: trace-whole-events-only
//! conforms: trace-bracket-kind-omits-payload
//! conforms: trace-envelope-flattens
//! conforms: trace-turn-close-internally-tagged
//!
//! The perturbation-verified tests of `weaver-trace-Spec` section 10. Each
//! names its perturbation, the mutation under which it was watched to fail.

use std::fs::File;
use std::io::Read;
use std::os::fd::OwnedFd;

use weaver_trace::{
    Envelope, Event, Failure, Kind, MonotonicNs, Payload, Recorder, RunOrdinal, Sequence,
    SessionRef, StopReason, SubmitRefusal, Subsystem, TurnClose, TurnRef, raw_payload,
};

fn sink() -> (OwnedFd, std::path::PathBuf) {
    let path = std::env::temp_dir().join(format!(
        "weaver-trace-test-{}-{:?}",
        std::process::id(),
        std::thread::current().id()
    ));
    let file = File::create(&path).expect("temp sink");
    (OwnedFd::from(file), path)
}

fn recorder() -> (Recorder, std::path::PathBuf) {
    let (fd, path) = sink();
    let r = Recorder::receive(fd, RunOrdinal(0), SessionRef("s-1".into())).expect("receives");
    (r, path)
}

fn envelope(kind: Kind, turn: Option<&str>) -> Envelope {
    Envelope {
        session: SessionRef("s-1".into()),
        run: RunOrdinal(0),
        turn: turn.map(|t| TurnRef(t.into())),
        sequence: Sequence(0),
        kind,
        subsystem: Subsystem::Harness,
        causal_parent: None,
        wall_ms: 1_754_400_000_000,
        monotonic_ns: MonotonicNs(9_007_199_254_740_993),
    }
}

fn event(kind: Kind, turn: Option<&str>, payload: Option<Payload>) -> Event {
    Event {
        envelope: envelope(kind, turn),
        payload,
    }
}

fn user_message(turn: &str) -> Event {
    event(
        Kind::MessageUser,
        Some(turn),
        Some(Payload::Message(
            raw_payload("{\"role\":\"user\",\"content\":[]}").unwrap(),
        )),
    )
}

/// One line per event: a payload carrying an embedded newline renders to one
/// line, serde escaping it to `\n`.
///
/// Perturbation: bypass the escaping by splicing octets that carry a raw
/// newline - `raw_payload` refuses the construction, which is the mechanism,
/// and hand-building the line instead was watched to split the stream.
#[test]
fn one_line_per_event() {
    let (mut r, path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    let prose = "line one\nline two";
    let rendered =
        serde_json::to_string(&serde_json::json!({"role":"user","text":prose})).expect("renders");
    r.submit(event(
        Kind::MessageUser,
        Some("t-1"),
        Some(Payload::Message(raw_payload(&rendered).unwrap())),
    ))
    .unwrap();
    r.drain().unwrap();
    let mut out = String::new();
    File::open(&path).unwrap().read_to_string(&mut out).unwrap();
    assert_eq!(
        out.lines().count(),
        2,
        "two events, two lines, embedded newline escaped"
    );
}

/// A raw newline in submitted octets cannot enter, in either position: JSON
/// forbids one inside a string, and the separator check refuses one between
/// tokens, where pretty-printing legally puts it.
#[test]
fn raw_newline_octets_refuse_construction() {
    assert!(raw_payload("{\"text\":\"a\nb\"}").is_none());
    assert!(raw_payload("{\n  \"role\": \"user\"\n}").is_none());
    assert!(raw_payload("{\r\n\"role\":\"user\"}").is_none());
    assert!(raw_payload("{\"text\":\"a\\nb\"}").is_some(), "an escaped newline is two octets");
}

/// The monotonic reading beyond the double-safe range serializes as a decimal
/// string and survives exactly.
///
/// Perturbation: render the reading as a bare number and a consumer parsing
/// doubles reads 9007199254740992 - one off, silently. Watched by asserting
/// on the string form, which the bare rendering fails.
#[test]
fn large_integers_render_as_decimal_strings() {
    let (mut r, path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    r.drain().unwrap();
    let mut out = String::new();
    File::open(&path).unwrap().read_to_string(&mut out).unwrap();
    assert!(
        out.contains("\"monotonic_ns\":\"9007199254740993\""),
        "the reading is a decimal string: {out}"
    );
    assert!(
        out.contains("\"sequence\":\"0\""),
        "the sequence is a decimal string: {out}"
    );
    assert!(
        out.contains("\"wall_ms\":1754400000000"),
        "the wall clock stays bare: {out}"
    );
}

/// Admission precedes the fan-out: a refused submission leaves no record in
/// the structure and no line on the stream.
///
/// Perturbation: move the refusal after the append and a row appears for the
/// refused event. Watched under exactly that reordering.
#[test]
fn refused_submission_touches_neither_sink() {
    let (mut r, path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    let err = r
        .submit(event(Kind::MessageUser, Some("t-1"), None))
        .unwrap_err();
    assert!(matches!(
        err,
        Failure::RefusedOnSubmit {
            reason: SubmitRefusal::RequiredFieldAbsent { .. }
        }
    ));
    assert_eq!(r.structure().len(), 1, "the refused event landed nowhere");
    r.drain().unwrap();
    let mut out = String::new();
    File::open(&path).unwrap().read_to_string(&mut out).unwrap();
    assert_eq!(
        out.lines().count(),
        1,
        "the stream holds only the admitted event"
    );
}

/// One rendering, two holders: the bytes in the structure are the bytes on
/// the stream.
///
/// Perturbation: re-render for the writer from the event and the comparison
/// fails the moment the two paths diverge. Watched by comparing structure
/// lines against the drained file byte for byte.
#[test]
fn structure_bytes_are_stream_bytes() {
    let (mut r, path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    r.submit(user_message("t-1")).unwrap();
    r.submit(event(
        Kind::TurnClosed,
        Some("t-1"),
        Some(Payload::TurnClosed(TurnClose::Stopped {
            reason: StopReason::Directive,
        })),
    ))
    .unwrap();
    r.drain().unwrap();
    let held: String = r.structure().iter().map(|rec| rec.line.as_ref()).collect();
    let mut streamed = String::new();
    File::open(&path)
        .unwrap()
        .read_to_string(&mut streamed)
        .unwrap();
    assert_eq!(held, streamed, "one rendering reaches both holders");
}

/// The sequence is gapless over admitted events: a refused submission
/// consumes no sequence.
///
/// Perturbation: assign the sequence before admission and a refusal leaves a
/// gap. Watched under exactly that reordering.
#[test]
fn sequence_is_gapless_over_admitted_events() {
    let (mut r, _path) = recorder();
    let a = r.submit(event(Kind::Load, None, None)).unwrap();
    let _ = r
        .submit(event(Kind::MessageUser, Some("t"), None))
        .unwrap_err();
    let b = r
        .submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap();
    let c = r.submit(user_message("t-1")).unwrap();
    assert_eq!(
        (a, b, c),
        (Sequence(0), Sequence(1), Sequence(2)),
        "no gap for the refusal"
    );
}

/// A bracket kind emits no payload member at all, and the envelope flattens:
/// the line is one flat object keyed on kind at the top level.
#[test]
fn bracket_kind_omits_payload_and_line_is_flat() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    let line = r.structure().iter().next().unwrap().line.clone();
    assert!(
        !line.contains("\"payload\""),
        "no payload member on a bracket kind: {line}"
    );
    assert!(
        !line.contains("\"envelope\""),
        "the envelope flattens: {line}"
    );
    assert!(
        line.starts_with("{\"session\":"),
        "declaration order from the top: {line}"
    );
    assert!(
        line.contains("\"kind\":\"load\""),
        "the dotted-name scheme's kind member: {line}"
    );
}

/// The turn close is internally tagged: one shape for both closes.
#[test]
fn turn_close_is_internally_tagged() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    r.submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap();
    r.submit(event(
        Kind::TurnClosed,
        Some("t-1"),
        Some(Payload::TurnClosed(TurnClose::Clean)),
    ))
    .unwrap();
    let line = r
        .structure()
        .by_kind(Kind::TurnClosed)
        .next()
        .unwrap()
        .line
        .clone();
    assert!(
        line.contains("\"payload\":{\"close\":\"clean\"}"),
        "internally tagged: {line}"
    );
}

/// A kind-to-payload mismatch refuses as malformed rather than rendering.
#[test]
fn mismatched_payload_refuses() {
    let (mut r, _path) = recorder();
    let err = r
        .submit(event(
            Kind::Load,
            None,
            Some(Payload::TurnClosed(TurnClose::Clean)),
        ))
        .unwrap_err();
    assert!(matches!(
        err,
        Failure::RefusedOnSubmit {
            reason: SubmitRefusal::PayloadMalformed
        }
    ));
}

/// The measurement's optional members are absent rather than zero.
///
/// Perturbation: remove the skip election and an empty array appears, saying
/// the reading was taken and found empty. Watched under exactly that removal.
#[test]
fn absent_measurement_members_emit_nothing() {
    use weaver_trace::{
        DecodeTimings, ModelId, ModelMeasurement, PromptBlock, TokenId, WeightsHash,
    };
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    r.submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap();
    r.submit(event(
        Kind::ModelMeasurement,
        Some("t-1"),
        Some(Payload::ModelMeasurement(ModelMeasurement {
            model: ModelId("qwen3-4b-instruct".into()),
            weights_hash: WeightsHash("sha256:abc".into()),
            input_tokens: vec![TokenId(1), TokenId(2)],
            output_tokens: vec![TokenId(3)],
            blocks: vec![PromptBlock {
                label: "system".into(),
                start: 0,
                end: 2,
            }],
            entropies: vec![],
            surprisals: vec![],
            timings: DecodeTimings {
                prefill_ns: MonotonicNs(1_000),
                decode_ns: MonotonicNs(2_000),
            },
            reductions: None,
        })),
    ))
    .unwrap();
    let line = r
        .structure()
        .by_kind(Kind::ModelMeasurement)
        .next()
        .unwrap()
        .line
        .clone();
    assert!(
        !line.contains("entropies"),
        "an unproduced reading emits no member: {line}"
    );
    assert!(
        !line.contains("surprisals"),
        "an unproduced reading emits no member: {line}"
    );
    assert!(
        !line.contains("reductions"),
        "an unproduced reading emits no member: {line}"
    );
}

/// The boundary derives its states: committed meets admitted after a drain,
/// and queued returns to zero.
#[test]
fn boundary_derives_after_drain() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    r.submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap();
    r.drain().unwrap();
    let b = r.boundary();
    assert_eq!(b.admitted, Some(Sequence(1)));
    assert_eq!(b.committed, Some(Sequence(1)));
    assert_eq!(b.queued, 0);
    assert_eq!(b.last_error, None);
}

/// An event bound to another session refuses: the recorder records one run.
#[test]
fn foreign_session_refuses() {
    let (mut r, _path) = recorder();
    let mut e = event(Kind::Load, None, None);
    e.envelope.session = SessionRef("s-2".into());
    let err = r.submit(e).unwrap_err();
    assert!(matches!(err, Failure::RefusedOnSubmit { .. }));
}

/// A pretty-printed payload that bypasses `raw_payload` still refuses at
/// render: the two layers hold the same line, construction first and the
/// render choke point as the backstop for a `RawValue` built any other way.
///
/// Perturbation: remove the interior-newline check from `render` and the
/// stream gains extra lines from one event. Watched under exactly that
/// removal.
#[test]
fn pretty_printed_payload_refuses_at_render() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, None)).unwrap();
    let pretty = "{\n  \"role\": \"user\",\n  \"content\": []\n}";
    let bypassed = serde_json::value::RawValue::from_string(pretty.to_string())
        .expect("valid JSON, construction alone admits it");
    let err = r
        .submit(event(Kind::MessageUser, Some("t-1"), Some(Payload::Message(bypassed))))
        .unwrap_err();
    assert!(matches!(
        err,
        Failure::RefusedOnSubmit {
            reason: SubmitRefusal::PayloadMalformed
        }
    ));
    assert_eq!(r.structure().len(), 1, "the refused event landed nowhere");
}

/// A run-level kind carrying a turn refuses: a join key the work never held
/// would be a false attribution. `fault` stays exempt, its option being the
/// caller's fact.
#[test]
fn turn_on_run_level_kind_refuses() {
    let (mut r, _path) = recorder();
    for kind in [Kind::Load, Kind::Unload, Kind::SessionClosed] {
        let err = r.submit(event(kind, Some("t-1"), None)).unwrap_err();
        assert!(
            matches!(
                err,
                Failure::RefusedOnSubmit {
                    reason: SubmitRefusal::PayloadMalformed
                }
            ),
            "{kind:?} with a turn must refuse"
        );
    }
    assert_eq!(r.structure().len(), 0);
    r.submit(event(Kind::Load, None, None)).unwrap();
    r.submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap();
    let fault = raw_payload("{\"kind\":\"stub\"}").unwrap();
    r.submit(event(Kind::Fault, Some("t-1"), Some(Payload::Fault(fault))))
        .unwrap();
    let fault2 = raw_payload("{\"kind\":\"stub\"}").unwrap();
    r.submit(event(Kind::Fault, None, Some(Payload::Fault(fault2))))
        .unwrap();
}

/// A failed write is terminal and named: committed never advances past the
/// first failed sequence, later queued records are discarded with the
/// accounting kept consistent, and drain returns `CommitFailed` identifying
/// the record.
#[test]
#[cfg(target_os = "linux")]
fn failed_write_is_terminal_and_named() {
    let file = File::create("/dev/full").expect("dev full");
    let mut r = Recorder::receive(OwnedFd::from(file), RunOrdinal(0), SessionRef("s-1".into()))
        .expect("receives");
    r.submit(event(Kind::Load, None, None)).unwrap();
    let err = r.drain().unwrap_err();
    match err {
        Failure::CommitFailed { sequence, .. } => assert_eq!(sequence, Sequence(0)),
        other => panic!("drain names the failed record, got {other:?}"),
    }
    let b = r.boundary();
    assert_eq!(
        b.committed, None,
        "committed never advances past the failure"
    );
    assert_eq!(b.queued, 0, "accounting stays consistent");
    let next = r
        .submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap_err();
    assert!(matches!(
        next,
        Failure::CommitFailed {
            sequence: Sequence(0),
            ..
        }
    ));
}

/// `CommitPressure` has one meaning: the high-water report on a recorded
/// event. The test drives a real pipe past the mark, confirms every reported
/// submission landed in the structure, reads the depth at the report, then
/// kills the sink and confirms the terminal-failure discard keeps the
/// accounting consistent. The absolute-full case blocks by design and is not
/// driven here, the block being backpressure a test cannot observe ending.
#[test]
fn high_water_reports_on_recorded_events() {
    use weaver_trace::HIGH_WATER_MARK;
    let (reader, pipe_writer) = std::io::pipe().expect("pipe");
    let mut r = Recorder::receive(
        OwnedFd::from(pipe_writer),
        RunOrdinal(0),
        SessionRef("s-1".into()),
    )
    .expect("receives");
    r.submit(event(Kind::Load, None, None)).unwrap();
    let mut submitted = 1usize;
    let mut reported = 0usize;
    let mut depth_at_first_report = None;
    while reported == 0 && submitted < 4 * HIGH_WATER_MARK {
        match r.submit(event(
            Kind::Fault,
            None,
            Some(Payload::Fault(raw_payload("{\"kind\":\"stub\"}").unwrap())),
        )) {
            Ok(_) => submitted += 1,
            Err(Failure::CommitPressure { queued }) => {
                submitted += 1;
                reported += 1;
                depth_at_first_report = Some(queued);
            }
            Err(other) => panic!("unexpected: {other:?}"),
        }
    }
    assert!(reported > 0, "the mark was crossed and reported");
    assert!(
        depth_at_first_report.unwrap() > HIGH_WATER_MARK,
        "the report carries the depth that crossed the mark"
    );
    assert_eq!(
        r.structure().len(),
        submitted,
        "every reported submission landed: the report is not a refusal"
    );
    drop(reader);
    let _ = r.drain();
    assert_eq!(r.boundary().queued, 0, "discard keeps the accounting consistent");
}
