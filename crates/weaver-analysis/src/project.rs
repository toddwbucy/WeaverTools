//! conforms: analysis-election-declares-what-follows
//! conforms: analysis-projection-splices-verbatim
//! conforms: analysis-sequence-order-preserved
//! conforms: analysis-preload-cuts-and-renames
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
    // **The four message kinds with `role` and `content`**, per Spec
    // section 3 as of 2026-09-06 and issue #432: a session standing from a
    // preloaded record rebuilds its prefix and its conversation from these
    // pairs through the `identity` and `recall` asks, and the replay loop
    // reads past them untouched, so one election serves both preloads.
    ElectedKind {
        kind: "message.system",
        paths: &["role", "content"],
    },
    ElectedKind {
        kind: "message.user",
        paths: &["role", "content"],
    },
    ElectedKind {
        kind: "message.assistant",
        paths: &["role", "content"],
    },
    ElectedKind {
        kind: "message.tool_result",
        paths: &["role", "content"],
    },
];

/// Why a cut refuses before anything is sent, per Spec section 4: the run
/// or the turn is not the record's, named.
#[derive(Debug, Clone, PartialEq)]
pub enum CutRefusal {
    RunNotHeld(String),
    TurnNotHeld { run: String, turn: u64 },
}

impl std::fmt::Display for CutRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CutRefusal::RunNotHeld(run) => write!(f, "the record holds no run {run:?}"),
            CutRefusal::TurnNotHeld { run, turn } => {
                write!(f, "run {run:?} holds no turn {turn}")
            }
        }
    }
}

/// **The cut**, per Spec section 4 as of 2026-09-04: every event of the
/// record through the named turn's last event in landing order and none
/// after it, which is the turn's close where the record is whole. A run the
/// record does not hold, or a turn that run does not hold, refuses before
/// anything is sent, and a turn is named beside its run because a turn's
/// number recurs across runs.
pub fn cut_through<'a>(
    events: &'a [Event],
    run: &str,
    turn: u64,
) -> Result<&'a [Event], CutRefusal> {
    if !events.iter().any(|event| event.envelope.run == run) {
        return Err(CutRefusal::RunNotHeld(run.to_string()));
    }
    let key = format!("t-{turn}");
    let last = events
        .iter()
        .rposition(|event| {
            event.envelope.run == run && event.envelope.turn.as_deref() == Some(key.as_str())
        })
        .ok_or_else(|| CutRefusal::TurnNotHeld {
            run: run.to_string(),
            turn,
        })?;
    Ok(&events[..=last])
}

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
    project_as(events, None)
}

/// The projection landed under another session name, per Spec section 4:
/// every distillate's session member rewritten as it crosses and the run
/// and sequence as recorded, which is what a branch needs because the
/// member bounds every answer to the session its opener declared. `None`
/// keeps the record's own name.
pub fn project_as(events: &[Event], session: Option<&str>) -> Vec<Distillate> {
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
            serde_json::json!(session.unwrap_or(event.envelope.session.as_str())),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::parse_record;

    fn record() -> String {
        concat!(
            r#"{"session":"s-1","run":"r-a","sequence":"0","kind":"load","payload":{"tee":{}}}"#, "
",
            r#"{"session":"s-1","run":"r-a","sequence":"1","kind":"message.system","payload":{"role":"system","content":[{"type":"text","text":"You are Karl."}]}}"#, "
",
            r#"{"session":"s-1","run":"r-a","turn":"t-1","sequence":"2","kind":"message.user","payload":{"role":"user","content":[{"type":"text","text":"hi"}]}}"#, "
",
            r#"{"session":"s-1","run":"r-a","turn":"t-1","sequence":"3","kind":"turn.closed","payload":{}}"#, "
",
            r#"{"session":"s-1","run":"r-a","turn":"t-2","sequence":"4","kind":"message.user","payload":{"role":"user","content":[{"type":"text","text":"more"}]}}"#, "
",
            r#"{"session":"s-1","run":"r-a","turn":"t-2","sequence":"5","kind":"turn.closed","payload":{}}"#, "
",
            r#"{"session":"s-1","run":"r-b","sequence":"6","kind":"load","payload":{"tee":{}}}"#, "
",
            r#"{"session":"s-1","run":"r-b","turn":"t-1","sequence":"7","kind":"turn.closed","payload":{}}"#, "
",
        )
        .to_string()
    }

    /// **The cut is the named turn's last event in landing order**, per
    /// Spec section 4: everything through it crosses and nothing after, a
    /// run or turn the record does not hold refuses naming it.
    ///
    /// Perturbation: return the whole record from `cut_through` and the
    /// count assertion fails, events past the turn crossing. Watched under
    /// exactly that change.
    #[test]
    fn the_cut_projects_through_the_turns_last_event_and_none_after() {
        let text = record();
        let events = parse_record(&text);
        let through = cut_through(&events, "r-a", 1).expect("the cut exists");
        assert_eq!(through.len(), 4, "the load, the prefix, and turn one whole");
        assert_eq!(
            through.last().unwrap().envelope.sequence,
            "3",
            "turn one's close"
        );
        assert_eq!(
            cut_through(&events, "r-zz", 1).err(),
            Some(CutRefusal::RunNotHeld("r-zz".into()))
        );
        assert_eq!(
            cut_through(&events, "r-a", 9).err(),
            Some(CutRefusal::TurnNotHeld {
                run: "r-a".into(),
                turn: 9
            })
        );
    }

    /// **The rename touches the envelope's session alone**, per Spec
    /// section 4: every distillate carries the new name with its run and
    /// sequence as recorded, and no name is given, the record's stands.
    ///
    /// Perturbation: drop the rewrite in `project_as` and the first
    /// assertion fails, a distillate keeping the record's name under an
    /// opener that declared another. Watched under exactly that change.
    #[test]
    fn the_rename_rewrites_the_session_and_nothing_else() {
        let text = record();
        let events = parse_record(&text);
        let renamed = project_as(&events, Some("s-2"));
        assert!(!renamed.is_empty());
        for distillate in &renamed {
            assert!(
                distillate.frame.contains("\"session\":\"s-2\""),
                "{}",
                distillate.frame
            );
            assert!(
                !distillate.frame.contains("s-1"),
                "the record's name is gone"
            );
        }
        assert!(
            renamed[0].frame.contains("\"run\":\"r-a\""),
            "the run is as recorded"
        );
        let kept = project(&events);
        assert!(
            kept[0].frame.contains("\"session\":\"s-1\""),
            "no name given, the record's stands"
        );
    }

    /// **The election names the four message kinds with role and content**,
    /// per Spec section 3 as of 2026-09-06, so a restoring open has the
    /// pairs it rebuilds from.
    ///
    /// Perturbation: drop a message kind from `ELECTION` and its distillate
    /// carries no pairs, the assertion on `content` failing. Watched under
    /// exactly that removal.
    #[test]
    fn the_election_carries_the_conversation_for_the_open() {
        let text = record();
        let events = parse_record(&text);
        let out = project(&events);
        let user = out
            .iter()
            .find(|d| d.frame.contains("\"kind\":\"message.user\""))
            .expect("the user message crosses");
        assert!(user.frame.contains("\"role\":\"user\""));
        assert!(
            user.frame.contains("\"content\":["),
            "the content crosses spliced"
        );
        let system = out
            .iter()
            .find(|d| d.frame.contains("\"kind\":\"message.system\""))
            .expect("the prefix crosses");
        assert!(system.frame.contains("You are Karl."));
    }
}
