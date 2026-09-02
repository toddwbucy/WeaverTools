//! conforms: analysis-reading-drains-within-a-turn
//!
//! The streaming reading's watches, per `weaver-analysis-Spec` section 5.
//! The fixtures are shaped like the real records the live run drained.

use weaver_analysis::capture::{Key, Streaming};
use weaver_analysis::{Drained, Signals, Step, drain};

fn record(closed: Option<&str>, trailing: &str) -> String {
    let mut lines = vec![
        r#"{"session":"s","run":"r","sequence":"0","kind":"replay.opened","payload":{"reader_elected":false}}"#.to_string(),
        r#"{"session":"s","run":"r","turn":"t-1","sequence":"1","kind":"residual.column","payload":{"position":10,"layers":2,"width":2,"values":[[1.0,2.0],[3.0,4.0]]}}"#.to_string(),
        r#"{"session":"s","run":"r","turn":"t-1","sequence":"2","kind":"residual.column","payload":{"position":11,"layers":2,"width":2,"values":[[5.0,6.0],[7.0,8.0]]}}"#.to_string(),
        r#"{"session":"s","run":"r","turn":"t-1","sequence":"3","kind":"model.measurement","payload":{"output_tokens":[100,200],"entropies":[1.5,2.5],"surprisals":[0.5,9.5],"perplexity":2.0}}"#.to_string(),
    ];
    if let Some(outcome) = closed {
        lines.push(format!(
            r#"{{"session":"s","run":"r","sequence":"4","kind":"replay.closed","payload":{{"outcome":{{"kind":"{outcome}"}}}}}}"#
        ));
    }
    if !trailing.is_empty() {
        lines.push(trailing.to_string());
    }
    lines.join("\n") + "\n"
}

/// **The bracket's close ends the reading, not the stream's end.** A
/// pipe's writer is the agent, which holds it open for the run's whole
/// residency, so a reader waiting for end-of-stream waits for the unload.
/// The live run of 2026-09-02 found exactly that: the reading never
/// emitted while the worker stood.
///
/// Perturbation: answer `Continue` at `replay.closed` and this fails, the
/// drain running past the close into whatever follows. Watched under
/// exactly that change.
#[test]
fn the_close_ends_the_drain_and_the_stream_does_not() {
    let after = r#"{"session":"s","run":"r","turn":"t-2","sequence":"5","kind":"residual.column","payload":{"position":99,"layers":1,"width":1,"values":[[42.0]]}}"#;
    let text = record(Some("certified"), after);
    let mut seen: Vec<(Key, u32)> = Vec::new();
    let mut pair = |key: &Key, _column: &[f32], token: u32| seen.push((key.clone(), token));
    let mut reader = Streaming::new(vec![10], &mut pair);
    let ended = drain(std::io::Cursor::new(text.as_bytes()), &mut reader);
    assert_eq!(ended, Drained::Stopped, "the close stops the drain");
    assert!(reader.certified());
    assert!(
        !reader.kept.keys().any(|(_, p)| *p == 99),
        "nothing past the close was read: {:?}",
        reader.kept.keys().collect::<Vec<_>>()
    );
    assert_eq!(seen.len(), 2, "both positions of the turn paired");
}

/// **What a reading holds is bounded by the analyst's named positions.**
/// The turn's final layers pass through the pairing and are dropped; only
/// the named positions' full columns stay.
///
/// Perturbation: keep every column rather than the named ones and this
/// fails, the second position appearing in what was held. Watched under
/// exactly that change.
#[test]
fn the_reading_holds_only_what_was_named() {
    let text = record(Some("certified"), "");
    let mut pair = |_: &Key, _: &[f32], _: u32| {};
    let mut reader = Streaming::new(vec![11], &mut pair);
    drain(std::io::Cursor::new(text.as_bytes()), &mut reader);
    let held: Vec<u64> = reader.kept.keys().map(|(_, p)| *p).collect();
    assert_eq!(held, vec![11], "only the named position is held: {held:?}");
}

/// **The outcome the record states is what a reading is gated on.**
#[test]
fn the_outcome_is_read_from_the_close() {
    for (closed, want) in [
        (Some("certified"), Some("certified")),
        (Some("diverged"), Some("diverged")),
        (Some("abandoned"), Some("abandoned")),
        (None, None),
    ] {
        let text = record(closed, "");
        let mut pair = |_: &Key, _: &[f32], _: u32| {};
        let mut reader = Streaming::new(vec![], &mut pair);
        drain(std::io::Cursor::new(text.as_bytes()), &mut reader);
        assert_eq!(reader.outcome.as_deref(), want, "closed {closed:?}");
        assert_eq!(reader.certified(), want == Some("certified"));
    }
}

/// **The signals reader rides the same drain and needs nothing else**: no
/// lens, no weights, no tap. The entropies ride every generation and the
/// surprisals ride their election, and an absent vector stays absent.
#[test]
fn the_signals_series_pairs_and_keeps_absence() {
    let text = record(Some("certified"), "");
    let mut reader = Signals::default();
    drain(std::io::Cursor::new(text.as_bytes()), &mut reader);
    let series = &reader.series;
    assert_eq!(series.points.len(), 2);
    assert_eq!(series.points[0].token, 100);
    assert_eq!(series.points[0].entropy, Some(1.5));
    assert_eq!(series.points[1].surprisal, Some(9.5));
    assert_eq!(series.perplexities, vec![(Some("t-1".to_string()), 2.0)]);

    // A record whose surprisal election did not stand carries entropies
    // and no surprisals, and the reading says so rather than inventing.
    let bare = text.replace(r#","surprisals":[0.5,9.5]"#, "");
    let mut reader = Signals::default();
    drain(std::io::Cursor::new(bare.as_bytes()), &mut reader);
    assert!(reader.series.points.iter().all(|p| p.surprisal.is_none()));
    assert!(reader.series.points.iter().all(|p| p.entropy.is_some()));
    assert!(reader.series.spikes(2.0).is_empty(), "no surprisal, no spike");
}

/// **A spike is a position that clears the caller's bar**, stated in
/// deviations because what counts as a spike depends on the series.
#[test]
fn the_spikes_are_the_positions_that_clear_the_bar() {
    let mut text = record(Some("certified"), "");
    text = text.replace(
        r#""output_tokens":[100,200],"entropies":[1.5,2.5],"surprisals":[0.5,9.5]"#,
        r#""output_tokens":[1,2,3,4],"entropies":[1.0,1.0,1.0,1.0],"surprisals":[1.0,1.0,1.0,20.0]"#,
    );
    let mut reader = Signals::default();
    drain(std::io::Cursor::new(text.as_bytes()), &mut reader);
    let spikes = reader.series.spikes(1.5);
    assert_eq!(spikes.len(), 1, "one position turned");
    assert_eq!(spikes[0].ordinal, 3);
    assert_eq!(spikes[0].token, 4);
}

/// **A malformed line is skipped rather than fatal**, per section 2's
/// reader rules carried into the drain.
///
/// **The signals reader exhausts where the capture reader stops**, and
/// the difference is the readers' own: a capture's reading ends at the
/// bracket the pass closed, while a signal series has no such marker and
/// serves a serving record too, where no `replay.closed` exists to wait
/// for. Read over a pipe, this reader therefore runs until the writer
/// closes - which is a property of what it reads rather than of the
/// drain, and is stated here so a caller streaming signals knows what it
/// is waiting on.
#[test]
fn a_malformed_line_does_not_end_the_stream() {
    let text = record(Some("certified"), "").replace(
        r#"{"session":"s","run":"r","turn":"t-1","sequence":"2","#,
        r#"{ this is not an event "#,
    );
    let mut reader = Signals::default();
    let ended = drain(std::io::Cursor::new(text.as_bytes()), &mut reader);
    assert_eq!(ended, Drained::Exhausted, "a signal series reads to the end");
    assert_eq!(reader.series.points.len(), 2, "the measurement still read");
}
