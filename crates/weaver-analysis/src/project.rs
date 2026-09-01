//! conforms: analysis-election-declares-what-follows
//! conforms: analysis-projection-splices-verbatim
//! conforms: analysis-sequence-order-preserved
//!
//! The election and the projection, per `weaver-analysis-Spec` section 3:
//! the election this crate declares is composed from what the replay reads,
//! not declared by an operator, and this document names the kinds those
//! steps read and the payload key paths they read out of them, and nothing
//! further. A distillate is the envelope whole and the elected pairs beside
//! it, each value as the record spelled it, reached by splicing raw payload
//! text rather than re-encoding a parsed value - which is what makes the
//! preload's indistinguishability claim true rather than approximate.

use crate::record::{Event, value_at};

/// One elected kind and the payload key paths read out of it.
#[derive(Debug, Clone)]
pub struct ElectedKind {
    pub kind: &'static str,
    pub paths: &'static [&'static str],
}

/// The election, composed from what `diagnostic-replay-loop` sections 2 and
/// 3 read: `load` for the tee's election the state claim rests on,
/// `model.request` for each generation's rendered contribution and its
/// identity, and `model.measurement` for the recorded token path the
/// certification compares against. A step added to that document widens
/// this election in the act that adds it.
pub const ELECTION: &[ElectedKind] = &[
    ElectedKind {
        kind: "load",
        paths: &["tee"],
    },
    ElectedKind {
        kind: "model.request",
        paths: &["rendered", "template", "sampling"],
    },
    ElectedKind {
        kind: "model.measurement",
        paths: &["input_tokens", "output_tokens", "model", "weights_hash"],
    },
];

/// The opener's frame: the election whole, with the session it declares
/// being the replayed session's own name, per the contract's section 2.
pub fn render_opener(session: &str) -> String {
    let keys: Vec<serde_json::Value> = ELECTION
        .iter()
        .map(|entry| {
            serde_json::json!({
                "kind": entry.kind,
                "paths": entry.paths,
            })
        })
        .collect();
    let mut frame = serde_json::json!({
        "session": session,
        "election": { "all_kinds": false, "keys": keys },
    })
    .to_string();
    frame.push('\n');
    frame
}

/// One distillate: the envelope's five members by name and the elected
/// pairs beside them, each value spliced as raw text. Carried as the
/// rendered frame because the value must not pass through a re-encoding on
/// its way to the wire.
#[derive(Debug, Clone)]
pub struct Distillate {
    frame: String,
}

impl Distillate {
    /// The rendered frame, one line, newline-terminated.
    pub fn frame(&self) -> &str {
        &self.frame
    }
}

/// The projection: every elected event of the record, in the record's
/// landing order, each distilled to its envelope and its elected pairs. No
/// sort and no grouping happens here - the loop's pairing runs in landing
/// order, so a stream that reordered would pair a request with another
/// generation's measurement.
pub fn project(events: &[Event]) -> Vec<Distillate> {
    let mut out = Vec::new();
    for event in events {
        let Some(entry) = ELECTION.iter().find(|e| e.kind == event.envelope.kind) else {
            continue;
        };
        let mut pairs = String::new();
        if let Some(payload) = &event.payload {
            for path in entry.paths {
                if let Some(value) = value_at(payload, path) {
                    if !pairs.is_empty() {
                        pairs.push(',');
                    }
                    // The key is this crate's rendering and the value is the
                    // record's bytes, spliced.
                    pairs.push_str(&serde_json::json!(path).to_string());
                    pairs.push(':');
                    pairs.push_str(value.get());
                }
            }
        }
        let mut envelope = format!(
            "{{\"session\":{},\"run\":{},",
            serde_json::json!(event.envelope.session),
            serde_json::json!(event.envelope.run),
        );
        if let Some(turn) = &event.envelope.turn {
            envelope.push_str(&format!("\"turn\":{},", serde_json::json!(turn)));
        }
        envelope.push_str(&format!(
            "\"kind\":{},\"sequence\":{}",
            serde_json::json!(event.envelope.kind),
            serde_json::json!(event.envelope.sequence),
        ));
        let frame = format!("{{\"envelope\":{envelope}}},\"pairs\":{{{pairs}}}}}\n");
        out.push(Distillate { frame });
    }
    out
}
