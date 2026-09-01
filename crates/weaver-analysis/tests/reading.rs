//! conforms: analysis-gates-on-the-stated-outcome
//! conforms: analysis-null-replay-gates-the-rest
//!
//! The reading half's watches, per `weaver-analysis-Spec` sections 5 and 6,
//! against the real diagnostic-traces of 2026-08-31: the certified null
//! replay and the abandoned first run, carried verbatim.

use weaver_analysis::{Gated, Outcome, RecordKind, parse_record};

const CERTIFIED: &str = include_str!("fixtures/diagnostic-certified.ndjson");
const ABANDONED: &str = include_str!("fixtures/diagnostic-abandoned.ndjson");
const SERVING: &str = include_str!("fixtures/serving-source.ndjson");

/// **Which record this crate holds is answered by the record**, never by
/// the invocation: a diagnostic-trace opens with `replay.opened`, a
/// serving record with `load`, and a record answering neither is neither.
#[test]
fn the_record_answers_which_record_it_is() {
    assert_eq!(
        weaver_analysis::record_kind(&parse_record(CERTIFIED)),
        RecordKind::Diagnostic
    );
    assert_eq!(
        weaver_analysis::record_kind(&parse_record(SERVING)),
        RecordKind::Serving
    );
    assert_eq!(
        weaver_analysis::record_kind(&parse_record("{\"x\":1}")),
        RecordKind::Neither
    );
}

/// **A certified null pass licenses the reading**, read from the real
/// record the null replay closed.
#[test]
fn a_certified_null_pass_produces() {
    let Gated::Produces { passes } = weaver_analysis::gate(&parse_record(CERTIFIED)) else {
        panic!("the certified record produces");
    };
    assert_eq!(passes.len(), 1);
    assert!(!passes[0].reader_elected, "the null pass elected no reader");
    assert!(matches!(passes[0].outcome, Some(Outcome::Certified)));
}

/// **An abandoned pass produces nothing**, its account standing in the
/// record: the real first run, whose replay ask went unanswered.
#[test]
fn an_abandoned_pass_produces_nothing() {
    let Gated::Nothing { why } = weaver_analysis::gate(&parse_record(ABANDONED)) else {
        panic!("an abandoned record licenses nothing");
    };
    assert!(why.contains("no certified null pass"), "{why}");
}

/// **The gate reads the stated outcome and never the end of available
/// bytes.** A record whose bracket never closed produces nothing, on the
/// same terms whichever way it came to be unclosed - a pass that died and
/// a pass still running leave one absence between them.
///
/// Perturbation: read the end of available bytes as an ending in
/// `brackets` - a fallback outcome for an unclosed bracket - and the
/// truncated certified record produces. Watched under exactly that change.
#[test]
fn an_unclosed_bracket_produces_nothing() {
    let truncated: String = CERTIFIED
        .lines()
        .filter(|l| !l.contains("replay.closed"))
        .collect::<Vec<_>>()
        .join("\n");
    let Gated::Nothing { why } = weaver_analysis::gate(&parse_record(&truncated)) else {
        panic!("an unclosed bracket licenses nothing");
    };
    assert!(why.contains("no certified null pass"), "{why}");
    let passes = weaver_analysis::brackets(&parse_record(&truncated));
    assert_eq!(passes.len(), 1);
    assert!(
        passes[0].outcome.is_none(),
        "the not-ended answer is one answer and not two"
    );
}

/// **The null replay gates the rest**: a certified reader pass beside no
/// certified null pass licenses nothing, a readout from an uncertified
/// replay being a picture of an unknown run.
///
/// Perturbation: drop the reader distinction from the gate and the reader
/// pass licenses itself. Watched under exactly that removal.
#[test]
fn a_reader_pass_alone_licenses_nothing() {
    let reader_only = CERTIFIED.replace(
        r#""payload":{"reader_elected":false}"#,
        r#""payload":{"reader_elected":true}"#,
    );
    assert_ne!(reader_only, CERTIFIED, "the fixture moved");
    let Gated::Nothing { why } = weaver_analysis::gate(&parse_record(&reader_only)) else {
        panic!("a reader pass alone licenses nothing");
    };
    assert!(why.contains("no certified null pass"), "{why}");
}
