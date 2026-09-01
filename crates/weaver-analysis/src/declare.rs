//! conforms: analysis-declaration-derives-from-the-record
//!
//! The declaration derived from the record, per `weaver-analysis-Spec`
//! section 3's third projection and the charter's section 3 as amended on
//! issue #394: every fact of the source run comes from the record, so the
//! diagnostic run is correct to the run and never to the analyst's memory.
//! Three members are the analyst's inputs - device placement, the readers'
//! elections, and the diagnostic sink - and two take the fixed spellings
//! the Spec names for members the record does not carry and a run under
//! this binding never reads.

use crate::record::{Event, value_at};

/// The analyst's three inputs, the charter's exceptions each for its own
/// reason: the record deliberately names no silicon, the readers are the
/// analyst's question, and the sink is the new record's home.
#[derive(Debug, Clone)]
pub struct AnalystInputs {
    pub devices: Vec<u32>,
    pub readout: bool,
    pub field_depth: Option<u32>,
    pub surprisal: bool,
    pub sink_path: String,
}

/// Why a derivation refused, naming the member: disagreement is a question
/// for the operator and never a pick, and absence refuses rather than
/// defaulting - completeness is claim-relative and the claim is the whole
/// declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum DeriveRefusal {
    MemberAbsent { member: &'static str },
    MemberDisagrees { member: &'static str, held: String, met: String },
}

/// One value across the record or a refusal naming the member.
fn one_value(
    events: &[Event],
    kind: &str,
    path: &str,
    member: &'static str,
) -> Result<String, DeriveRefusal> {
    let mut held: Option<String> = None;
    for event in events.iter().filter(|e| e.envelope.kind == kind) {
        let Some(payload) = &event.payload else { continue };
        let Some(value) = value_at(payload, path) else { continue };
        let met = value.get().to_string();
        match &held {
            None => held = Some(met),
            Some(prior) if *prior == met => {}
            Some(prior) => {
                return Err(DeriveRefusal::MemberDisagrees {
                    member,
                    held: prior.clone(),
                    met,
                });
            }
        }
    }
    held.ok_or(DeriveRefusal::MemberAbsent { member })
}

/// The derived declaration, rendered as the YAML the operator loads. The
/// identity messages are embedded in JSON flow spelling, YAML carrying JSON
/// whole, so the seated prefix crosses verbatim from the record's own
/// `message.system` payloads rather than through a re-rendering.
pub fn derive(events: &[Event], inputs: &AnalystInputs) -> Result<String, DeriveRefusal> {
    let session = events
        .first()
        .map(|e| e.envelope.session.clone())
        .ok_or(DeriveRefusal::MemberAbsent { member: "session" })?;
    let artifact: String = serde_json::from_str(&one_value(
        events,
        "model.measurement",
        "model",
        "model-binding.artifact",
    )?)
    .map_err(|_| DeriveRefusal::MemberAbsent {
        member: "model-binding.artifact",
    })?;
    let seed = one_value(events, "model.request", "sampling.seed", "tunable-values.seed")?;
    let max_tokens = one_value(
        events,
        "model.request",
        "stop.max_tokens",
        "tunable-values.max-tokens-per-turn",
    )?;
    let capacity = one_value(
        events,
        "model.output",
        "capacity",
        "tunable-values.context-capacity",
    )?;
    // The seated prefix: the turnless message.system events at the run's
    // opening, in landing order, each payload carried verbatim.
    let prefix: Vec<&str> = events
        .iter()
        .filter(|e| e.envelope.kind == "message.system" && e.envelope.turn.is_none())
        .filter_map(|e| e.payload.as_deref().map(|p| p.get()))
        .collect();
    if prefix.is_empty() {
        return Err(DeriveRefusal::MemberAbsent { member: "identity" });
    }

    // Every interpolated string scalar is serialized as a JSON string,
    // which YAML carries whole: a session, artifact, or sink path holding a
    // colon, a quote, or any other YAML-significant character crosses as
    // the value it is rather than as markup.
    let mut declaration = String::new();
    declaration.push_str(&format!("session: {}\n", serde_json::json!(session)));
    declaration.push_str("binding-kind: diagnostic\n");
    declaration.push_str("spu-instruction:\n  decoder:\n");
    declaration.push_str("    model-binding:\n");
    declaration.push_str(&format!("      artifact: {}\n", serde_json::json!(artifact)));
    let devices: Vec<String> = inputs.devices.iter().map(|d| d.to_string()).collect();
    declaration.push_str(&format!("      devices: [{}]\n", devices.join(", ")));
    declaration.push_str(&format!(
        "    residual-readout-election: {}\n",
        inputs.readout
    ));
    if let Some(depth) = inputs.field_depth {
        declaration.push_str(&format!("    field-election:\n      depth: {depth}\n"));
    }
    declaration.push_str(&format!("    surprisal-election: {}\n", inputs.surprisal));
    declaration.push_str(&format!("    identity: [{}]\n", prefix.join(", ")));
    declaration.push_str("    tunable-values:\n");
    declaration.push_str(&format!("      seed: {seed}\n"));
    declaration.push_str(&format!("      context-capacity: {capacity}\n"));
    declaration.push_str(&format!("      max-tokens-per-turn: {max_tokens}\n"));
    // The fixed spellings, per the Spec: members the record does not carry
    // and a run under this binding never reads take a spelling rather than
    // a guess.
    declaration.push_str("tool-set: []\n");
    declaration.push_str("permission-mode: ask\n");
    declaration.push_str("trace-sink:\n  kind: file\n");
    declaration.push_str(&format!("  path: {}\n", serde_json::json!(inputs.sink_path)));
    declaration.push_str("  create: true\n");
    Ok(declaration)
}
