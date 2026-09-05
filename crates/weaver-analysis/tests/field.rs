//! The field read's watches, per `weaver-analysis-Spec` section 5 as of
//! 2026-09-04: one position's alternatives and their mass, read from the
//! record and never re-rendered, with nothing else held.

use std::process::Command;

use weaver_analysis::{Address, Answer, Drained, FieldReader, drain};

/// A serving record of two runs. The first run holds two turns: `t-1`
/// draws two tokens at positions 10 and 11, and `t-2` draws three at 30,
/// 31, and 32, the draw at 31 landing past the reported depth so its
/// token is the measurement's alone. The second run repeats `t-1` at
/// position 10 with another field, the way turn keys repeat across the
/// runs of a serving record. Probabilities are spelled the way a
/// re-render would not spell them.
const RANKED_31: &str = r#"[{"token":8,"probability":0.10},{"token":80,"probability":1e-3},{"token":800,"probability":0.5}]"#;

fn record() -> String {
    let lines = [
        r#"{"session":"s","run":"r-1","sequence":"0","kind":"load","payload":{"residual_readout":false,"field":3,"surprisal":false}}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-1","sequence":"1","kind":"turn.started"}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-1","sequence":"2","kind":"model.field","payload":{"position":10,"ranked":[{"token":100,"probability":0.9},{"token":101,"probability":0.05},{"token":102,"probability":0.01}],"realized":0}}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-1","sequence":"3","kind":"model.field","payload":{"position":11,"ranked":[{"token":201,"probability":0.6},{"token":200,"probability":0.3},{"token":202,"probability":0.02}],"realized":1}}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-1","sequence":"4","kind":"model.measurement","payload":{"output_tokens":[100,200],"entropies":[0.5,1.2]}}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-1","sequence":"5","kind":"turn.closed","payload":{}}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-2","sequence":"6","kind":"turn.started"}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-2","sequence":"7","kind":"model.field","payload":{"position":30,"ranked":[{"token":7,"probability":0.7},{"token":70,"probability":0.2},{"token":700,"probability":0.05}],"realized":0}}"#.to_string(),
        format!(
            r#"{{"session":"s","run":"r-1","turn":"t-2","sequence":"8","kind":"model.field","payload":{{"position":31,"ranked":{RANKED_31},"realized":3}}}}"#
        ),
        r#"{"session":"s","run":"r-1","turn":"t-2","sequence":"9","kind":"model.field","payload":{"position":32,"ranked":[{"token":9,"probability":0.8},{"token":90,"probability":0.1},{"token":900,"probability":0.05}],"realized":0}}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-2","sequence":"10","kind":"model.measurement","payload":{"output_tokens":[7,8000,9],"entropies":[0.1,3.0,0.2]}}"#.to_string(),
        r#"{"session":"s","run":"r-1","turn":"t-2","sequence":"11","kind":"turn.closed","payload":{}}"#.to_string(),
        r#"{"session":"s","run":"r-1","sequence":"12","kind":"unload"}"#.to_string(),
        r#"{"session":"s","run":"r-2","sequence":"0","kind":"load","payload":{"residual_readout":false,"field":3,"surprisal":false}}"#.to_string(),
        r#"{"session":"s","run":"r-2","turn":"t-1","sequence":"1","kind":"turn.started"}"#.to_string(),
        r#"{"session":"s","run":"r-2","turn":"t-1","sequence":"2","kind":"model.field","payload":{"position":10,"ranked":[{"token":55,"probability":0.5},{"token":56,"probability":0.4},{"token":57,"probability":0.1}],"realized":1}}"#.to_string(),
        r#"{"session":"s","run":"r-2","turn":"t-1","sequence":"3","kind":"model.measurement","payload":{"output_tokens":[56],"entropies":[1.0]}}"#.to_string(),
        r#"{"session":"s","run":"r-2","turn":"t-1","sequence":"4","kind":"turn.closed","payload":{}}"#.to_string(),
    ];
    lines.join("\n") + "\n"
}

fn read(text: &str, address: &str, run: Option<&str>) -> (Vec<Answer>, Drained, bool, usize) {
    let mut answers: Vec<Answer> = Vec::new();
    let mut emit = |answer: &Answer| answers.push(answer.clone());
    let mut reader = FieldReader::new(
        Address::parse(address).expect("the address parses"),
        run.map(str::to_string),
        &mut emit,
    );
    let drained = drain(std::io::Cursor::new(text.as_bytes()), &mut reader);
    if drained == Drained::Exhausted {
        reader.finish();
    }
    let seen = reader.seen_field;
    let answered = reader.answered;
    (answers, drained, seen, answered)
}

/// **The asked position answers, and nothing else is held or emitted.**
/// The address is the turn beside the position, so a position the
/// record holds at another turn is not this one.
///
/// Perturbation: match on the position alone, dropping the turn from the
/// comparison, and a record whose two turns both hold position 10 would
/// answer twice. Watched under exactly that change, the address `t-1:10`
/// against a record salted with a `t-2` field at position 10.
#[test]
fn the_asked_position_answers_and_nothing_else() {
    let (answers, drained, seen, answered) = read(&record(), "t-2:30", None);
    assert!(seen);
    assert_eq!(answered, 1);
    assert_eq!(
        drained,
        Drained::Exhausted,
        "a serving record is read to its end"
    );
    assert_eq!(answers.len(), 1, "one position, one answer");
    let answer = &answers[0];
    assert_eq!(answer.run, "r-1");
    assert_eq!(answer.turn, "t-2");
    assert_eq!(answer.position, 30);
    assert_eq!(answer.realized, 0);
    assert_eq!(answer.drawn, Some(7), "the candidate at the realized rank");
    assert_eq!(
        answer.ranked.get(),
        r#"[{"token":7,"probability":0.7},{"token":70,"probability":0.2},{"token":700,"probability":0.05}]"#
    );

    // The same position at another turn is another position.
    let salted = record().replace(
        r#""turn":"t-2","sequence":"7","kind":"model.field","payload":{"position":30"#,
        r#""turn":"t-2","sequence":"7","kind":"model.field","payload":{"position":10"#,
    );
    let (answers, _, _, _) = read(&salted, "t-1:10", None);
    let runs: Vec<&str> = answers.iter().map(|a| a.run.as_str()).collect();
    assert_eq!(
        runs,
        vec!["r-1", "r-2"],
        "t-1:10 stands once per run and never at t-2"
    );
    assert!(answers.iter().all(|a| a.turn == "t-1"));
}

/// **The ranked list crosses as the record spelled it.** `0.10` and `1e-3`
/// are spellings a parse-and-render would change to `0.1` and `0.001`,
/// and the answer carries the record's bytes.
///
/// Perturbation: parse the ranked list to a value and render it back, and
/// the two spellings normalise. Watched under exactly that change.
#[test]
fn the_ranked_list_crosses_as_the_record_spelled_it() {
    let (answers, _, _, _) = read(&record(), "t-2:31", None);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].ranked.get(), RANKED_31);
    let rendered = serde_json::to_string(&answers[0]).expect("renders");
    assert!(
        rendered.contains(r#""probability":0.10}"#) && rendered.contains(r#""probability":1e-3}"#),
        "the rendered line keeps the record's spelling: {rendered}"
    );
}

/// **A draw past the reported depth takes its token from the measurement**,
/// at this field's ordinal within its generation: the fields pair with the
/// drawn tokens one for one in landing order, the stop token being neither
/// retained nor ranked.
///
/// Perturbation: take `output_tokens[0]` rather than the field's ordinal
/// and the drawn token reads 7, the first draw of the turn, rather than
/// 8000. Watched under exactly that change.
#[test]
fn a_draw_past_the_depth_takes_its_token_from_the_measurement() {
    let (answers, _, _, _) = read(&record(), "t-2:31", None);
    assert_eq!(answers.len(), 1);
    assert_eq!(
        answers[0].realized, 3,
        "the depth itself: past what was reported"
    );
    assert_eq!(
        answers[0].drawn,
        Some(8000),
        "the measurement's second draw"
    );

    // Where the measurement never lands, the token is absent rather than
    // invented.
    let truncated: String = record()
        .lines()
        .take_while(|l| !l.contains(r#""sequence":"10""#))
        .map(|l| format!("{l}\n"))
        .collect();
    let (answers, _, _, _) = read(&truncated, "t-2:31", None);
    assert_eq!(answers.len(), 1, "the field itself was in the record");
    assert_eq!(answers[0].drawn, None);
}

/// **A record without the field refuses as a missing election, and a
/// position the record does not hold refuses as a missing position**, told
/// apart by whether any field landed at all.
///
/// Perturbation: set `seen_field` on any event rather than on a
/// `model.field`, and a record without the field reads as a record
/// without the position. Watched under exactly that change.
#[test]
fn a_record_without_the_field_refuses_and_so_does_a_missing_position() {
    let bare: String = record()
        .lines()
        .filter(|l| !l.contains(r#""kind":"model.field""#))
        .map(|l| format!("{l}\n"))
        .collect();
    let (answers, _, seen, answered) = read(&bare, "t-1:10", None);
    assert!(!seen, "no field landed");
    assert_eq!(answered, 0);
    assert!(answers.is_empty());

    let (answers, _, seen, answered) = read(&record(), "t-1:99", None);
    assert!(seen, "the field stands in the record");
    assert_eq!(answered, 0, "and not at this position");
    assert!(answers.is_empty());
}

/// **A named run ends the read when it answers, and an unnamed one lets
/// every run holding the position answer**, each naming its run, since
/// turn keys repeat across the runs of a serving record.
///
/// Perturbation: answer `Continue` after a named run's answer and the
/// drain runs to the record's end, `Exhausted` rather than `Stopped`.
/// Watched under exactly that change.
#[test]
fn a_named_run_ends_the_read_and_an_unnamed_one_answers_per_run() {
    let (answers, drained, _, _) = read(&record(), "t-1:10", Some("r-2"));
    assert_eq!(
        drained,
        Drained::Stopped,
        "the named run answered and the read ended"
    );
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0].run, "r-2");
    assert_eq!(answers[0].drawn, Some(56));

    let (answers, drained, _, _) = read(&record(), "t-1:10", None);
    assert_eq!(drained, Drained::Exhausted);
    let seen: Vec<(&str, Option<u32>)> =
        answers.iter().map(|a| (a.run.as_str(), a.drawn)).collect();
    assert_eq!(seen, vec![("r-1", Some(100)), ("r-2", Some(56))]);
}

/// **A diagnostic record answers on the same terms, gated on no close's
/// outcome**: the field is the record's own fact about a position. The
/// bracket's close ends the read as it ends every reader's.
///
/// Perturbation: answer `Continue` at `replay.closed` and the `t-9` field
/// past the close answers, the drain running to `Exhausted`. Watched under
/// exactly that change. The verb-level watch below runs the abandoned
/// bracket through the binary, where a gate on the outcome would sit.
#[test]
fn a_diagnostic_record_answers_whatever_its_outcome() {
    for outcome in ["certified", "diverged", "abandoned"] {
        let text = format!(
            "{}\n{}\n{}\n{}\n{}\n",
            r#"{"session":"s","run":"d","sequence":"0","kind":"replay.opened","payload":{"reader_elected":true}}"#,
            r#"{"session":"s","run":"d","turn":"t-1","sequence":"1","kind":"model.field","payload":{"position":60,"ranked":[{"token":1,"probability":0.5},{"token":2,"probability":0.4}],"realized":1}}"#,
            r#"{"session":"s","run":"d","turn":"t-1","sequence":"2","kind":"model.measurement","payload":{"output_tokens":[2]}}"#,
            format!(
                r#"{{"session":"s","run":"d","sequence":"3","kind":"replay.closed","payload":{{"outcome":{{"kind":"{outcome}"}}}}}}"#
            ),
            r#"{"session":"s","run":"d","turn":"t-9","sequence":"4","kind":"model.field","payload":{"position":60,"ranked":[{"token":3,"probability":0.5}],"realized":0}}"#,
        );
        let (answers, drained, _, _) = read(&text, "t-1:60", None);
        assert_eq!(
            drained,
            Drained::Stopped,
            "the close ends the read: {outcome}"
        );
        assert_eq!(answers.len(), 1, "{outcome}");
        assert_eq!(answers[0].drawn, Some(2));
        let (answers, _, _, _) = read(&text, "t-9:60", None);
        assert!(
            answers.is_empty(),
            "nothing past the close is read: {outcome}"
        );
    }
}

/// **The address is `<turn>:<position>` and anything else refuses by
/// name.**
#[test]
fn the_address_is_turn_colon_position() {
    assert_eq!(
        Address::parse("t-3:117"),
        Ok(Address {
            turn: "t-3".to_string(),
            position: 117
        })
    );
    for bad in ["117", "t-3:", ":117", "t-3:abc", "t-3:-1", ""] {
        assert!(Address::parse(bad).is_err(), "{bad:?} is not an address");
    }
}

/// **The verb itself**, run as the front end would run it: one JSON line
/// per answer on stdout, and each refusal typed on stderr with a non-zero
/// exit.
#[test]
fn the_verb_answers_one_line_and_refuses_typed() {
    let dir = std::env::temp_dir().join(format!("weaver-analysis-field-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("record.ndjson");
    std::fs::write(&path, record()).expect("the record writes");
    let binary = env!("CARGO_BIN_EXE_weaver-analysis");

    let out = Command::new(binary)
        .args(["field", path.to_str().unwrap(), "--position", "t-2:31"])
        .output()
        .expect("runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf8");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "one line: {stdout}");
    let line: serde_json::Value = serde_json::from_str(lines[0]).expect("json");
    assert_eq!(line["run"], "r-1");
    assert_eq!(line["turn"], "t-2");
    assert_eq!(line["position"], 31);
    assert_eq!(line["realized"], 3);
    assert_eq!(line["drawn"], 8000);
    assert_eq!(line["ranked"].as_array().map(Vec::len), Some(3));
    assert!(
        lines[0].contains(RANKED_31),
        "spliced as spelled: {}",
        lines[0]
    );

    let refusal = |args: &[&str]| -> String {
        let out = Command::new(binary).args(args).output().expect("runs");
        assert!(!out.status.success(), "{args:?} must refuse");
        assert!(out.stdout.is_empty(), "nothing on stdout under a refusal");
        let stderr = String::from_utf8(out.stderr).expect("utf8");
        let line: serde_json::Value = serde_json::from_str(stderr.trim()).expect("typed");
        line["analysis_refusal"]
            .as_str()
            .expect("named")
            .to_string()
    };
    let record_path = path.to_str().unwrap();
    assert!(
        refusal(&["field", record_path, "--position", "t-2:99"]).contains("no field at t-2:99")
    );
    assert!(refusal(&["field", record_path, "--position", "t-2"]).contains("<turn>:<position>"));
    assert!(refusal(&["field", record_path]).contains("--position"));

    let bare: String = record()
        .lines()
        .filter(|l| !l.contains(r#""kind":"model.field""#))
        .map(|l| format!("{l}\n"))
        .collect();
    let bare_path = dir.join("bare.ndjson");
    std::fs::write(&bare_path, bare).expect("writes");
    let why = refusal(&["field", bare_path.to_str().unwrap(), "--position", "t-1:10"]);
    assert!(
        why.contains("elected the field at depth 3") && why.contains("no model.field event"),
        "{why}"
    );

    // A record whose load elected no field names that rather than a depth.
    let unelected = std::fs::read_to_string(&bare_path)
        .expect("reads")
        .replace(r#""field":3,"#, "");
    std::fs::write(&bare_path, unelected).expect("writes");
    let why = refusal(&["field", bare_path.to_str().unwrap(), "--position", "t-1:10"]);
    assert!(why.contains("not elected"), "{why}");

    // An abandoned diagnostic bracket answers: the verb gates on no
    // outcome, the field being the record's own fact about a position.
    // Perturbation: gate the verb on a certified close and this exits
    // non-zero.
    let abandoned = format!(
        "{}\n{}\n{}\n{}\n",
        r#"{"session":"s","run":"d","sequence":"0","kind":"replay.opened","payload":{"reader_elected":true}}"#,
        r#"{"session":"s","run":"d","turn":"t-1","sequence":"1","kind":"model.field","payload":{"position":60,"ranked":[{"token":1,"probability":0.5},{"token":2,"probability":0.4}],"realized":1}}"#,
        r#"{"session":"s","run":"d","turn":"t-1","sequence":"2","kind":"model.measurement","payload":{"output_tokens":[2]}}"#,
        r#"{"session":"s","run":"d","sequence":"3","kind":"replay.closed","payload":{"outcome":{"kind":"abandoned"}}}"#,
    );
    let abandoned_path = dir.join("abandoned.ndjson");
    std::fs::write(&abandoned_path, abandoned).expect("writes");
    let out = Command::new(binary)
        .args([
            "field",
            abandoned_path.to_str().unwrap(),
            "--position",
            "t-1:60",
        ])
        .output()
        .expect("runs");
    assert!(
        out.status.success(),
        "no gate on the outcome: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let line: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&out.stdout).trim()).expect("json");
    assert_eq!(line["drawn"], 2);

    // A position the bracket does not hold refuses by its address, on the
    // same terms as a serving record's.
    let why = refusal(&[
        "field",
        abandoned_path.to_str().unwrap(),
        "--position",
        "t-1:61",
    ]);
    assert!(why.contains("no field at t-1:61"), "{why}");

    std::fs::remove_dir_all(&dir).ok();
}
