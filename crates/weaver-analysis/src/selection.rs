//! conforms: analysis-reconstruction-follows-recorded-election
//! conforms: analysis-preload-validates-before-opener
//! conforms: analysis-diagnostic-requires-distinct-destination
//!
//! Independent reading of the recorded tee rule, per analysis Spec sections 3-4.
//! No writer type is linked here: the record's snake_case representation is
//! trace Spec section 3's, while the selection semantics are section 11's.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::record::{Event, value_at};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElectedKind {
    pub kind: String,
    pub paths: Vec<String>,
}

/// No deserialization defaults: absence is missing evidence, not an election.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Election {
    pub all_kinds: bool,
    pub keys: Vec<ElectedKind>,
}

impl Election {
    pub fn diagnostic() -> Self {
        Self {
            all_kinds: false,
            keys: crate::project::ELECTION
                .iter()
                .map(|entry| ElectedKind {
                    kind: entry.kind.to_string(),
                    paths: entry.paths.iter().map(|path| (*path).to_string()).collect(),
                })
                .collect(),
        }
    }

    /// Resolve first-match precedence BEFORE disregarding order and repeated
    /// paths. Sorting or merging duplicate kinds changes the recorded rule.
    fn effective(&self) -> (bool, BTreeMap<&str, BTreeSet<&str>>) {
        let mut keys = BTreeMap::new();
        for entry in &self.keys {
            keys.entry(entry.kind.as_str())
                .or_insert_with(|| entry.paths.iter().map(String::as_str).collect());
        }
        (self.all_kinds, keys)
    }
}

pub struct Prepared<'a> {
    pub events: &'a [Event],
    pub source: &'a str,
    pub destination: &'a str,
    pub election: Election,
}

/// Complete semantic preflight before the composition root can dial. The cut
/// bounds session and rule evidence; diagnostic mode does not manufacture a
/// missing source rule, but carries that absence onward for certification.
pub fn preflight<'a>(
    events: &'a [Event],
    diagnostic: bool,
    destination: Option<&'a str>,
    through: Option<(&str, u64)>,
) -> Result<Prepared<'a>, String> {
    let events = match through {
        Some((run, turn)) => {
            crate::project::cut_through(events, run, turn).map_err(|why| why.to_string())?
        }
        None => events,
    };
    let source = events
        .first()
        .ok_or("the record holds no event")?
        .envelope
        .session
        .as_str();
    if source.is_empty() || events.iter().any(|event| event.envelope.session != source) {
        return Err("the selected prefix must hold one nonempty source session".into());
    }
    if destination == Some("") {
        return Err("the destination must be nonempty".into());
    }
    if (diagnostic || through.is_some()) && (destination.is_none() || destination == Some(source)) {
        return Err(
            "diagnostic projection and a cut require --as distinct from the source session".into(),
        );
    }
    let election = if diagnostic {
        Election::diagnostic()
    } else {
        recorded(events)?
    };
    Ok(Prepared {
        events,
        source,
        destination: destination.unwrap_or(source),
        election,
    })
}

fn recorded(events: &[Event]) -> Result<Election, String> {
    let mut runs: BTreeMap<&str, Election> = BTreeMap::new();
    let mut held: Option<Election> = None;
    for event in events {
        let run = event.envelope.run.as_str();
        if event.envelope.kind == "load" {
            let rule = event
                .payload
                .as_deref()
                .and_then(|payload| value_at(payload, "tee"))
                .ok_or_else(|| format!("run {run:?} load lacks tee election evidence"))?;
            let rule: Election = serde_json::from_str(rule.get())
                .map_err(|why| format!("run {run:?} load.tee is malformed: {why}"))?;
            if let Some(previous) = runs.get(run)
                && previous.effective() != rule.effective()
            {
                return Err(format!(
                    "run {run:?} has conflicting duplicate load.tee evidence"
                ));
            }
            if let Some(previous) = &held
                && previous.effective() != rule.effective()
            {
                return Err(format!(
                    "run {run:?} has a different effective election in the selected prefix"
                ));
            }
            if held.is_none() {
                held = Some(rule.clone());
            }
            runs.insert(run, rule);
        } else if !runs.contains_key(run) {
            return Err(format!("run {run:?} event precedes its governing load.tee"));
        }
    }
    held.ok_or_else(|| "the selected prefix lacks load.tee election evidence".into())
}
