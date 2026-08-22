//! conforms: trace-one-line-per-event
//! conforms: trace-large-integers-as-decimal-strings
//! conforms: trace-admission-precedes-fan-out
//! conforms: trace-one-rendering-two-holders
//! conforms: trace-sequence-gapless
//! conforms: trace-whole-events-only
//! conforms: trace-bracket-kind-omits-payload
//! conforms: trace-envelope-flattens
//! conforms: trace-turn-close-internally-tagged
//! conforms: trace-output-carries-the-counts
//!
//! The perturbation-verified tests of `weaver-trace-Spec` section 10. Each
//! names its perturbation, the mutation under which it was watched to fail.

use std::fs::File;
use std::io::Read;
use std::os::fd::OwnedFd;

use weaver_trace::{
    Envelope, Event, Failure, Kind, MonotonicNs, Payload, Recorder, RunRef, Sequence, SessionRef,
    StopReason, SubmitRefusal, Subsystem, TurnClose, TurnRef, raw_payload,
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
    let r =
        Recorder::receive(fd, RunRef("r-1".into()), SessionRef("s-1".into())).expect("receives");
    (r, path)
}

fn envelope(kind: Kind, turn: Option<&str>) -> Envelope {
    Envelope {
        session: SessionRef("s-1".into()),
        run: RunRef("r-1".into()),
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

/// The elections a load declares. Every `load` event carries them as of
/// 2026-08-21, so a record says what posture it was written in.
fn elections() -> Payload {
    Payload::Elections(weaver_trace::Elections {
        residual_readout: false,
        field: None,
        surprisal: false,
    })
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
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
    assert!(
        raw_payload("{\"text\":\"a\\nb\"}").is_some(),
        "an escaped newline is two octets"
    );
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
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
    let a = r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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

/// A payload-free kind emits no payload member at all, and the envelope
/// flattens: the line is one flat object keyed on kind at the top level.
///
/// **Read on `unload` rather than `load` as of 2026-08-21**, `load` having
/// stopped being payload-free when it began carrying the diagnostic
/// elections of its load. The property under test is the rendering's and
/// not that kind's, so it moves to a kind that still holds it and the run
/// bracket's other half is the nearest one.
#[test]
fn bracket_kind_omits_payload_and_line_is_flat() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    r.submit(event(Kind::Unload, None, None)).unwrap();
    let line = r
        .structure()
        .by_kind(Kind::Unload)
        .next()
        .unwrap()
        .line
        .clone();
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
        line.contains("\"kind\":\"unload\""),
        "the dotted-name scheme's kind member: {line}"
    );
}

/// The turn close is internally tagged: one shape for both closes.
#[test]
fn turn_close_is_internally_tagged() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
    // The measurement is a spliced payload as of the custody act, the SPU
    // producing the absence and the trace carrying it verbatim, so a blob
    // rendered without the unproduced members emits none of them: the record
    // carries exactly what the organ rendered, no serde election of this
    // crate's between them.
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    r.submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap();
    let measurement = weaver_trace::raw_payload(
        r#"{"model":"qwen3-4b-instruct","weights_hash":"sha256:abc","input_tokens":[1,2],"output_tokens":[3],"blocks":[{"label":"turn-delta","start":0,"end":2}],"timings":{"prefill_ns":"1000","decode_ns":"2000"}}"#,
    )
    .expect("the measurement blob splices");
    r.submit(event(
        Kind::ModelMeasurement,
        Some("t-1"),
        Some(Payload::ModelMeasurement(measurement)),
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

/// **The output carries the session's position**, per `weaver-trace-Spec`
/// section 3: an analysis placing a turn inside the context has the record
/// and nothing else once the run is over, and a member that serializes is
/// lost silently - the line still renders and every consumer still parses.
///
/// Perturbation: drop either count from `ModelOutput` and this fails.
#[test]
fn the_output_carries_the_counts() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    r.submit(event(Kind::TurnStarted, Some("t-1"), None))
        .unwrap();
    r.submit(event(
        Kind::ModelOutput,
        Some("t-1"),
        Some(Payload::ModelOutput(weaver_trace::ModelOutput {
            emission: "the answer".into(),
            finish: weaver_trace::Finish::Completed,
            resident: 26_214,
            capacity: 32_768,
        })),
    ))
    .unwrap();
    let line = r
        .structure()
        .by_kind(Kind::ModelOutput)
        .next()
        .unwrap()
        .line
        .clone();
    let rendered: serde_json::Value = serde_json::from_str(&line).expect("the line is one value");
    let payload = rendered
        .get("payload")
        .expect("the output carries a payload");
    assert_eq!(
        payload.get("resident").and_then(|v| v.as_u64()),
        Some(26_214),
        "the resident count reaches the record: {line}"
    );
    assert_eq!(
        payload.get("capacity").and_then(|v| v.as_u64()),
        Some(32_768),
        "the capacity reaches the record: {line}"
    );
}

/// The boundary derives its states: committed meets admitted after a drain,
/// and queued returns to zero.
#[test]
fn boundary_derives_after_drain() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
    let mut e = event(Kind::Load, None, Some(elections()));
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
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    let pretty = "{\n  \"role\": \"user\",\n  \"content\": []\n}";
    let bypassed = serde_json::value::RawValue::from_string(pretty.to_string())
        .expect("valid JSON, construction alone admits it");
    let err = r
        .submit(event(
            Kind::MessageUser,
            Some("t-1"),
            Some(Payload::Message(bypassed)),
        ))
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
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
    let mut r = Recorder::receive(
        OwnedFd::from(file),
        RunRef("r-1".into()),
        SessionRef("s-1".into()),
    )
    .expect("receives");
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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

/// Pressure is a reading on a recorded event and never a failure of one, per
/// `weaver-trace-Spec` section 9 as of 2026-08-22. The test drives a real
/// pipe past the mark, confirms every submission landed in the structure,
/// reads the depth from the recorder at the crossing, then kills the sink and
/// confirms the terminal-failure discard keeps the accounting consistent. The
/// absolute-full case blocks by design and is not driven here, the block
/// being backpressure a test cannot observe ending.
///
/// **This test asserted the property before the shape carried it.** It read
/// the depth off `Err(Failure::CommitPressure)` while asserting in the same
/// breath that "the report is not a refusal", which is the contradiction the
/// shape now resolves: a submission that landed answers `Ok`.
#[test]
fn high_water_reports_on_recorded_events() {
    use weaver_trace::HIGH_WATER_MARK;
    let (reader, pipe_writer) = std::io::pipe().expect("pipe");
    let mut r = Recorder::receive(
        OwnedFd::from(pipe_writer),
        RunRef("r-1".into()),
        SessionRef("s-1".into()),
    )
    .expect("receives");
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
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
            Err(other) => panic!("a submission that lands answers Ok: {other:?}"),
        }
        // The reading is taken after the submission, which is where the
        // harness takes it: the depth is the recorder's own and not the
        // submission's answer.
        let pressure = r.pressure();
        if pressure.over_mark {
            reported += 1;
            depth_at_first_report = Some(pressure.queued);
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
    assert_eq!(
        r.boundary().queued,
        0,
        "discard keeps the accounting consistent"
    );
}

/// **The subsystem's wire spellings are pinned, every case, and the organ and
/// its engine are distinct values.** The field is the record's attribution and
/// a consumer keys on the strings, so a variant rename or a serde-scheme
/// change that moved one silently would re-attribute history. The
/// organ-against-engine pair carries the claim of the #103 ruling: `spu` and
/// `spu_decoder` are two producing parties, the organ's residency facts and
/// the decode engine's model events, and a set that collapsed them would lose
/// the fact a reader of a model event wants first.
///
/// Perturbation: rename `SpuDecoder` to `Decoder`, or drop it, and this fails
/// naming the spelling. Watched by the pair below going through the same
/// serializer the recorder uses.
#[test]
fn the_subsystem_spellings_are_pinned_and_the_engine_is_not_the_organ() {
    let spelled = |subsystem: Subsystem| {
        let mut e = envelope(Kind::Load, None);
        e.subsystem = subsystem;
        serde_json::to_string(&e).expect("the envelope renders")
    };
    for (case, wire) in [
        (Subsystem::Admin, "\"subsystem\":\"admin\""),
        (Subsystem::Harness, "\"subsystem\":\"harness\""),
        (Subsystem::Spu, "\"subsystem\":\"spu\""),
        (Subsystem::SpuDecoder, "\"subsystem\":\"spu_decoder\""),
        (Subsystem::Gate, "\"subsystem\":\"gate\""),
        (Subsystem::Tool, "\"subsystem\":\"tool\""),
    ] {
        let line = spelled(case);
        assert!(line.contains(wire), "expected {wire} in {line}");
    }
    assert_ne!(
        spelled(Subsystem::Spu),
        spelled(Subsystem::SpuDecoder),
        "the organ and its engine are two attributions, which is the split's point"
    );
}

/// The classify pair admits with a turn and without one, per the charter's
/// adding text - a classify between turns belongs to no turn - and the
/// pairing is enforced: each kind takes exactly its own payload, and a
/// scored outcome under the request kind refuses.
#[test]
fn the_classify_pair_is_turn_optional_and_pairing_enforced() {
    let (mut r, _path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    let ask = || {
        Some(Payload::ClassifyRequest(weaver_trace::ClassifyAsk {
            content: "the recalled passage".into(),
        }))
    };
    let outcome = || {
        Some(Payload::ClassifyOutput(weaver_trace::ClassifyScored {
            labels: vec![("entailment".into(), 0.9), ("not_entailment".into(), 0.1)],
        }))
    };
    r.submit(event(Kind::ClassifyRequest, None, ask()))
        .expect("between turns, no turn");
    r.submit(event(Kind::ClassifyOutput, None, outcome()))
        .expect("the outcome follows");
    r.submit(event(Kind::ClassifyRequest, Some("t-1"), ask()))
        .expect("within a turn, the key rides");
    // **A refused classify authors no output at all**, per the charter's
    // clause of 2026-08-22: the refusal reaches the record under its own
    // kind and `classify.output` carries the scored labels alone.
    r.submit(event(
        Kind::Refusal,
        Some("t-1"),
        Some(Payload::Refusal(
            raw_payload(
                "{\"seam\":\"classify\",\"refusal\":{\"refusal\":\"oversized\"}}",
            )
            .unwrap(),
        )),
    ))
    .expect("a refusal is the record's own fact, under the class's kind");
    assert!(
        r.submit(event(Kind::ClassifyRequest, None, outcome())).is_err(),
        "the pairing is total"
    );
    assert!(
        r.submit(event(Kind::ClassifyOutput, None, None)).is_err(),
        "the outcome carries its account"
    );
}

/// The system kind is real since its act, and turn-optional since the
/// prefix act: it admits under the message payload inside a turn, admits
/// with no turn because the seated identity prefix belongs to none, and the
/// wire spelling is the charter's dotted name.
///
/// **The asymmetry is the point and is asserted here rather than assumed.**
/// This test read `turn-required like its siblings` until the prefix act,
/// which moved this one kind and left the other three where they were, so
/// what it watches now is that the move was to one kind and not to the
/// message kinds as a class.
#[test]
fn the_system_kind_is_turn_optional_and_its_siblings_are_not() {
    let (mut r, path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    let seq = r
        .submit(event(
            Kind::MessageSystem,
            Some("t-1"),
            Some(Payload::Message(
                raw_payload("{\"role\":\"system\",\"content\":[]}").unwrap(),
            )),
        ))
        .expect("admits inside a turn");
    assert!(seq.0 > 0);
    r.submit(event(
        Kind::MessageSystem,
        None,
        Some(Payload::Message(
            raw_payload("{\"role\":\"system\",\"content\":[]}").unwrap(),
        )),
    ))
    .expect("and admits with no turn, the seated prefix belonging to none");
    for sibling in [
        Kind::MessageUser,
        Kind::MessageAssistant,
        Kind::MessageToolResult,
    ] {
        assert!(
            r.submit(event(
                sibling,
                None,
                Some(Payload::Message(raw_payload("{}").unwrap())),
            ))
            .is_err(),
            "the other message kinds are turn-required as they were"
        );
    }
    r.drain().unwrap();
    let mut out = String::new();
    File::open(&path).unwrap().read_to_string(&mut out).unwrap();
    assert!(out.contains("\"message.system\""), "the dotted spelling");
}

/// **The surprisal's election is written even when declined.** Charter
/// section 3.2 names each election individually so a record's posture is
/// recoverable from the record, and this one is the first that can be
/// absent while the reading it governs is present: every record written
/// before 2026-08-21 carries the surprisal vector and no flag beside it.
/// Absent, false, and true are therefore three states, and only an
/// explicit `false` separates a declined election from a record older than
/// the election.
///
/// Perturbation: give `surprisal` a `skip_serializing_if` that drops the
/// false, and the declined case becomes indistinguishable from the old
/// record. Watched under exactly that.
#[test]
fn a_declined_surprisal_election_is_written_down() {
    let (mut r, path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    r.submit(event(
        Kind::Load,
        None,
        Some(Payload::Elections(weaver_trace::Elections {
            residual_readout: false,
            field: None,
            surprisal: true,
        })),
    ))
    .unwrap();
    r.drain().unwrap();

    let mut out = String::new();
    File::open(&path).unwrap().read_to_string(&mut out).unwrap();
    let lines: Vec<&str> = out.lines().collect();
    assert!(
        lines[0].contains("\"surprisal\":false"),
        "the declined election is on the wire: {}",
        lines[0]
    );
    assert!(
        lines[1].contains("\"surprisal\":true"),
        "and so is the standing one: {}",
        lines[1]
    );
    assert!(
        !lines[0].contains("\"field\""),
        "while the field's absence stays an absence, the two elections \
         shaped differently on purpose"
    );
}


/// **An elision refuses a turn rather than merely not needing one.** It is
/// asked between turns on the flush's ground, so a turn on one is a
/// malformed submission and not a posture, which is the distinction
/// `turn_forbidden` draws and `turn_required` cannot: a kind that is merely
/// turn-optional admits both.
///
/// Perturbation: move `Kind::Elision` out of `turn_forbidden` and leave it
/// turn-optional, and the turn-bearing submission below is admitted.
/// Watched under exactly that move.
#[test]
fn an_elision_refuses_a_turn() {
    let (mut r, path) = recorder();
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();
    let span = || {
        Some(Payload::Elision(weaver_trace::ElisionSpan {
            from: 41,
            to: 57,
            resident_before: 1237,
            resident_after: 1221,
        }))
    };
    r.submit(event(Kind::Elision, None, span()))
        .expect("a turnless elision is the ordinary case");
    assert!(
        r.submit(event(Kind::Elision, Some("t-1"), span())).is_err(),
        "an elision carrying a turn is refused rather than admitted"
    );
    r.drain().unwrap();

    let mut out = String::new();
    File::open(&path).unwrap().read_to_string(&mut out).unwrap();
    let elisions: Vec<&str> = out.lines().filter(|l| l.contains("\"elision\"")).collect();
    assert_eq!(elisions.len(), 1, "only the turnless one reached the sink");
    assert!(
        elisions[0].contains("\"from\":41") && elisions[0].contains("\"to\":57"),
        "and it carries the span it removed: {}",
        elisions[0]
    );
}


/// **A submission that lands answers `Ok` whatever the queue holds**, per
/// `weaver-trace-Spec` section 9 as of 2026-08-22, and the depth is a
/// reading taken from the recorder rather than an answer to a submission.
///
/// The property that makes this the right shape: past the mark, the event is
/// in the working structure and the answer is `Ok`. Before this act the same
/// event produced `Err`, so a caller could not act on pressure without also
/// treating a recorded event as a lost one.
///
/// Perturbation: return `Err` past the mark again and the first assertion
/// fires on a submission whose event is in the structure beside it.
#[test]
fn a_submission_past_the_mark_still_answers_ok() {
    use weaver_trace::HIGH_WATER_MARK;
    let (reader, pipe_writer) = std::io::pipe().expect("pipe");
    let mut r = Recorder::receive(
        OwnedFd::from(pipe_writer),
        RunRef("r-1".into()),
        SessionRef("s-1".into()),
    )
    .expect("receives");
    r.submit(event(Kind::Load, None, Some(elections()))).unwrap();

    let mut submitted = 1usize;
    while !r.pressure().over_mark && submitted < 4 * HIGH_WATER_MARK {
        r.submit(event(
            Kind::Fault,
            None,
            Some(Payload::Fault(raw_payload("{\"kind\":\"stub\"}").unwrap())),
        ))
        .expect("a submission that lands answers Ok");
        submitted += 1;
    }
    assert!(r.pressure().over_mark, "the mark was crossed");

    // The one that matters: past the mark, still Ok, and in the structure.
    let before = r.structure().len();
    r.submit(event(
        Kind::Fault,
        None,
        Some(Payload::Fault(raw_payload("{\"kind\":\"stub\"}").unwrap())),
    ))
    .expect("a submission past the mark answers Ok because its event landed");
    assert_eq!(
        r.structure().len(),
        before + 1,
        "and the event it answered for is in the structure"
    );
    assert!(
        r.pressure().queued > HIGH_WATER_MARK,
        "the depth is readable and says what it holds"
    );

    drop(reader);
    let _ = r.drain();
}
