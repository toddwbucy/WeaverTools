//! conforms: analysis-gates-on-the-stated-outcome
//! conforms: analysis-null-replay-gates-the-rest
//!
//! The diagnostic-trace's parse and the gate, per `weaver-analysis-Spec`
//! section 5. The line is the same line and the envelope the same envelope,
//! so section 2's parse binds here unchanged: skip the unknown, derive
//! nothing absent, hold payloads raw. What differs is the kind set, and
//! `weaver-diagnostic-Spec` section 3.2 is authoritative for it.
//!
//! **The gate is the outcome the record states.** A reading is produced
//! where a bracket closed certified, the divergence where one closed
//! diverged, neither where one closed abandoned, and nothing for any
//! unclosed bracket, on the same terms whichever way it came to be
//! unclosed: a pass that died and a pass still running leave one absence
//! between them, and treating the end of available bytes as the end of a
//! run is exactly what the marker exists to stop.

use crate::record::{Event, value_at};

/// Which record this crate holds, answered by the record: a bracket opening
/// with `replay.opened` is a diagnostic-trace and one opening with `load`
/// is a serving record, per `weaver-diagnostic-Spec` section 4, and a
/// record that answers neither is refused as neither.
#[derive(Debug, Clone, PartialEq)]
pub enum RecordKind {
    Diagnostic,
    Serving,
    Neither,
}

pub fn record_kind(events: &[Event]) -> RecordKind {
    match events.first().map(|e| e.envelope.kind.as_str()) {
        Some("replay.opened") => RecordKind::Diagnostic,
        Some("load") => RecordKind::Serving,
        _ => RecordKind::Neither,
    }
}

/// One pass's bracket as the record states it: the run it rode, whether its
/// opening declared a reader, and the outcome its close carried - `None`
/// where no `replay.closed` stands, the not-ended answer that is one answer
/// and not two.
#[derive(Debug, Clone)]
pub struct Bracket {
    pub run: String,
    pub reader_elected: bool,
    pub outcome: Option<Outcome>,
}

/// The three outcomes a pass can state, each carrying the close's raw
/// payload so a reader of the reading holds what the record held.
#[derive(Debug, Clone)]
pub enum Outcome {
    Certified,
    Diverged { detail: String },
    Abandoned { detail: String },
}

/// The record's brackets, one per `replay.opened`, in landing order.
pub fn brackets(events: &[Event]) -> Vec<Bracket> {
    let mut out: Vec<Bracket> = Vec::new();
    for event in events {
        match event.envelope.kind.as_str() {
            "replay.opened" => {
                let reader_elected = event
                    .payload
                    .as_deref()
                    .and_then(|p| value_at(p, "reader_elected"))
                    .is_some_and(|v| v.get() == "true");
                out.push(Bracket {
                    run: event.envelope.run.clone(),
                    reader_elected,
                    outcome: None,
                });
            }
            "replay.closed" => {
                let Some(open) = out
                    .iter_mut()
                    .rev()
                    .find(|b| b.run == event.envelope.run && b.outcome.is_none())
                else {
                    continue;
                };
                let Some(payload) = event.payload.as_deref() else {
                    continue;
                };
                let Some(kind) = value_at(payload, "outcome.kind") else {
                    continue;
                };
                let detail = || {
                    value_at(payload, "outcome")
                        .map(|v| v.get().to_string())
                        .unwrap_or_default()
                };
                open.outcome = match kind.get() {
                    "\"certified\"" => Some(Outcome::Certified),
                    "\"diverged\"" => Some(Outcome::Diverged { detail: detail() }),
                    "\"abandoned\"" => Some(Outcome::Abandoned { detail: detail() }),
                    _ => None,
                };
            }
            _ => {}
        }
    }
    out
}

/// What the gate answers for a record in hand.
#[derive(Debug, Clone)]
pub enum Gated {
    /// A certified null pass stands and every **stated** outcome is
    /// listed: readings downstream are licensed. An unclosed bracket
    /// contributes nothing here - the gate produces nothing for it, per
    /// the Spec's own sentence, so the licensed list carries closed
    /// brackets alone and a not-ended pass is read from [`brackets`]
    /// where an account wants it.
    Produces { passes: Vec<Bracket> },
    /// Nothing is produced, and the account says why: no certified null
    /// pass stands in the record, whatever a reader pass beside it
    /// reported, or the record is not a diagnostic-trace at all.
    Nothing { why: String },
}

/// The gate: this crate reads a null pass's outcome first and gates every
/// reading downstream on it. Where no certified null pass stands, nothing
/// is produced regardless of what a reader pass reported - a readout from
/// an uncertified replay is a picture of an unknown run.
pub fn gate(events: &[Event]) -> Gated {
    if record_kind(events) != RecordKind::Diagnostic {
        return Gated::Nothing {
            why: "the record is not a diagnostic-trace".to_string(),
        };
    }
    let passes = brackets(events);
    let null_certified = passes
        .iter()
        .any(|b| !b.reader_elected && matches!(b.outcome, Some(Outcome::Certified)));
    if !null_certified {
        return Gated::Nothing {
            why: "no certified null pass stands in the record".to_string(),
        };
    }
    // Nothing is produced for an unclosed bracket, on the same terms
    // whichever way it came to be unclosed.
    let passes = passes.into_iter().filter(|b| b.outcome.is_some()).collect();
    Gated::Produces { passes }
}
