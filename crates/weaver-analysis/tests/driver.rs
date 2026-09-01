//! conforms: analysis-parse-skips-the-unknown
//! conforms: analysis-derives-no-absent-member
//! conforms: analysis-election-declares-what-follows
//! conforms: analysis-projection-splices-verbatim
//! conforms: analysis-sequence-order-preserved
//! conforms: analysis-seal-ends-the-preload
//! conforms: analysis-declaration-derives-from-the-record
//!
//! The driver half's watches, per `weaver-analysis-Spec` section 6, each
//! naming its perturbation. The fixtures are real records: the smoke-loop1
//! cell-q8 serving trace the null replay of 2026-08-31 certified against,
//! carried verbatim.

use weaver_analysis::{AnalystInputs, DeriveRefusal, parse_record, project, render_opener};

const SOURCE: &str = include_str!("fixtures/serving-source.ndjson");

fn inputs() -> AnalystInputs {
    AnalystInputs {
        devices: vec![0],
        readout: false,
        field_depth: None,
        surprisal: false,
        sink_path: "/tmp/diag.ndjson".to_string(),
    }
}

/// **The parse skips a kind and a payload member it does not know, and
/// neither decides a grouping.** An invented kind and an invented member
/// land in the record and the projection is byte-identical to the
/// projection without them.
///
/// Perturbation: refuse unknown kinds in `parse_record` and the count
/// moves, the invented kind failing the record instead of passing by.
/// Watched under exactly that addition.
#[test]
fn an_invented_kind_and_member_move_no_grouping() {
    let clean: Vec<String> = project(&parse_record(SOURCE))
        .iter()
        .map(|d| d.frame().to_string())
        .collect();
    let salted = format!(
        "{}{}{}",
        r#"{"session":"s-karl-1","run":"r-x","sequence":"90","kind":"invented.kind","subsystem":"harness","payload":{"who":"knows"}}"#,
        "\n",
        SOURCE.replacen(
            r#""payload": {"rendered"#,
            r#""payload": {"invented_member": 7, "rendered"#,
            1
        ),
    );
    let salted: Vec<String> = project(&parse_record(&salted))
        .iter()
        .map(|d| d.frame().to_string())
        .collect();
    assert_eq!(clean, salted, "the unknown decides nothing");
}

/// **No absent member is derived.** A record whose `model.output` carries
/// no capacity refuses the derivation naming the member, rather than
/// computing one from the members beside it or defaulting.
///
/// Perturbation: default the absent capacity in `derive` and the refusal
/// disappears. Watched under exactly that change.
#[test]
fn an_absent_member_refuses_rather_than_derives() {
    let without: String = SOURCE
        .lines()
        .filter(|l| !l.contains(r#""kind": "model.output""#))
        .collect::<Vec<_>>()
        .join("\n");
    let refused = weaver_analysis::derive(&parse_record(&without), &inputs());
    assert_eq!(
        refused,
        Err(DeriveRefusal::MemberAbsent {
            member: "tunable-values.context-capacity"
        }),
        "absence refuses naming the member"
    );
}

/// **A member the record spells two ways refuses naming the member**,
/// disagreement being the operator's question and never a pick.
#[test]
fn a_disagreeing_member_refuses_rather_than_picks() {
    let salted = format!(
        "{}\n{}",
        SOURCE.trim_end(),
        r#"{"session":"s-karl-1","run":"r-2","sequence":"90","kind":"model.measurement","subsystem":"spu.decoder","payload":{"model":"/other/artifact.gguf","input_tokens":[],"output_tokens":[1],"weights_hash":"x"}}"#,
    );
    let refused = weaver_analysis::derive(&parse_record(&salted), &inputs());
    assert!(
        matches!(
            refused,
            Err(DeriveRefusal::MemberDisagrees {
                member: "model-binding.artifact",
                ..
            })
        ),
        "disagreement refuses naming the member: {refused:?}"
    );
}

/// **The election declares what follows**: the stream carries no kind the
/// opener did not name, read against a record that holds plenty of others.
///
/// Perturbation: project every kind and the set widens past the opener.
/// Watched under exactly that change.
#[test]
fn no_kind_crosses_past_the_election() {
    let opener: serde_json::Value = serde_json::from_str(&render_opener("s-karl-1")).unwrap();
    let elected: Vec<&str> = opener["election"]["keys"]
        .as_array()
        .unwrap()
        .iter()
        .map(|k| k["kind"].as_str().unwrap())
        .collect();
    for distillate in project(&parse_record(SOURCE)) {
        let frame: serde_json::Value = serde_json::from_str(distillate.frame()).unwrap();
        let kind = frame["envelope"]["kind"].as_str().unwrap();
        assert!(
            elected.contains(&kind),
            "{kind} crossed past the election"
        );
    }
    assert_eq!(
        project(&parse_record(SOURCE)).len(),
        5,
        "one load, two requests, two measurements"
    );
}

/// **The projection splices verbatim.** Values a re-encoding would change -
/// an exponent spelling, a float at full precision, an integer past 2^53 -
/// cross byte-identical into the distillate's frame.
///
/// Perturbation: re-encode a parsed value in `project` and the exponent
/// spelling collapses. Watched under exactly that change.
#[test]
fn the_projection_splices_the_records_own_bytes() {
    let record = concat!(
        r#"{"session":"s-1","run":"r-1","turn":"t-1","sequence":"4","kind":"model.request","subsystem":"spu.decoder","#,
        r#""payload":{"rendered":"x","template":"y","sampling":{"seed":14458752852352082704,"temperature":0.699999988079071,"odd":1e3}}}"#,
    );
    let distillates = project(&parse_record(record));
    let frame = distillates[0].frame();
    assert!(
        frame.contains(r#""sampling":{"seed":14458752852352082704,"temperature":0.699999988079071,"odd":1e3}"#),
        "the record's own spellings cross: {frame}"
    );
}

/// **Sequence order is preserved**: distillates leave in the record's
/// landing order, never sorted and never grouped before sending.
///
/// Perturbation: sort the projection by kind and the interleaving of
/// requests and measurements collapses. Watched under exactly that change.
#[test]
fn distillates_leave_in_landing_order() {
    let kinds: Vec<String> = project(&parse_record(SOURCE))
        .iter()
        .map(|d| {
            let frame: serde_json::Value = serde_json::from_str(d.frame()).unwrap();
            frame["envelope"]["kind"].as_str().unwrap().to_string()
        })
        .collect();
    assert_eq!(
        kinds,
        vec![
            "load",
            "model.request",
            "model.measurement",
            "model.request",
            "model.measurement"
        ],
        "the record's own interleaving"
    );
}

/// **The seal ends the preload**: one empty JSON object on its own line
/// after the last distillate, `{}` canonically and never a blank line.
///
/// Perturbation: substitute a blank line for the seal and this fails, a
/// blank line being framing residue the custodian does not read as a seal.
/// Watched under exactly that change.
#[test]
fn the_seal_is_an_empty_object_on_its_own_line() {
    let events = parse_record(SOURCE);
    let mut wire: Vec<u8> = Vec::new();
    {
        let mut sender = weaver_analysis::preload::open(&mut wire, "s-karl-1").unwrap();
        for distillate in &project(&events) {
            sender.send(distillate).unwrap();
        }
        sender.seal().unwrap();
    }
    let text = String::from_utf8(wire).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 7, "opener, five distillates, the seal");
    assert_eq!(*lines.last().unwrap(), "{}", "the seal's one spelling");
    let opener: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(opener["session"], "s-karl-1", "the election opens the flow");
}

/// **The declaration derives from the record, correct to the run and never
/// to the analyst's memory.** Every source-run fact of the real record
/// lands in the derived declaration: the artifact, the seed the run
/// declared, the bounds, and the seated prefix verbatim.
///
/// Perturbation: fill the seed from a constant instead of the record and
/// the assertion fails on the recorded value. Watched under exactly that
/// change.
#[test]
fn the_declaration_derives_every_source_run_fact() {
    let declaration =
        weaver_analysis::derive(&parse_record(SOURCE), &inputs()).expect("the record is whole");
    assert!(declaration.contains(
        "artifact: /bulk-store/weaver-testing/cross-precision-repro/qwen2.5-0.5b-instruct-q8_0.gguf"
    ));
    assert!(declaration.contains("seed: 451234785645"));
    assert!(declaration.contains("context-capacity: 16384"));
    assert!(declaration.contains("max-tokens-per-turn: 1024"));
    assert!(declaration.contains("binding-kind: diagnostic"));
    assert!(
        declaration.contains("You are Karl"),
        "the seated prefix crosses verbatim"
    );
    assert!(
        !declaration.contains("gate-instruction"),
        "a diagnostic declaration carries no gate"
    );
    assert!(declaration.contains("session: s-karl-1"));
}
